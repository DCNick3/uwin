use std::ptr::copy_nonoverlapping;

type TargetPtrRepr = u32;
type TargetPtrDiffRepr = i32;

// many ideas borrowed from wonderful crate "scroll", but API tailored for uwin use-case

macro_rules! from_mem_impl {
    ($typ:tt, $size:expr) => {
        impl<'a> FromMemory<()> for $typ {
            type Error = ();

            #[inline]
            fn try_from_ctx(src: &[u8], _: ()) -> Result<Self, Self::Error> {
                Ok($typ::from_le_bytes(src.try_into().unwrap()))
            }

            #[inline]
            fn size_with(_: &()) -> usize {
                $size
            }
        }
    };
}

from_mem_impl!(u8, 1);
from_mem_impl!(i8, 1);
from_mem_impl!(u16, 2);
from_mem_impl!(i16, 2);
from_mem_impl!(u32, 4);
from_mem_impl!(i32, 4);
from_mem_impl!(u64, 8);
from_mem_impl!(i64, 8);
from_mem_impl!(u128, 16);
from_mem_impl!(i128, 16);

macro_rules! into_mem_impl {
    ($typ:tt, $size:expr) => {
        impl<'a> IntoMemory<()> for $typ {
            type Error = ();

            #[inline]
            fn try_into_ctx(self, dst: &mut [u8], _: ()) -> Result<(), Self::Error> {
                assert!(dst.len() >= $size);
                unsafe {
                    let bytes = self.to_le_bytes();
                    copy_nonoverlapping((&bytes).as_ptr(), dst.as_mut_ptr(), $size);
                }
                Ok(())
            }

            #[inline]
            fn size_with(_: &()) -> usize {
                $size
            }
        }
    };
}

into_mem_impl!(u8, 1);
into_mem_impl!(i8, 1);
into_mem_impl!(u16, 2);
into_mem_impl!(i16, 2);
into_mem_impl!(u32, 4);
into_mem_impl!(i32, 4);
into_mem_impl!(u64, 8);
into_mem_impl!(i64, 8);
into_mem_impl!(u128, 16);
into_mem_impl!(i128, 16);

pub trait FromMemory<Ctx: Copy>: Sized {
    type Error;
    fn try_from_ctx(from: &[u8], ctx: Ctx) -> Result<Self, Self::Error>;
    fn size_with(ctx: &Ctx) -> usize;
}

pub trait IntoMemory<Ctx: Copy>: Sized {
    type Error;
    fn try_into_ctx(self, into: &mut [u8], ctx: Ctx) -> Result<(), Self::Error>;
    fn size_with(ctx: &Ctx) -> usize;
}

/// Read from and write to a native pointer (u32 basically)
///
/// This context gets embedded into every converted pointer, so ideally it should be really lightweight
pub trait MemoryCtx: Copy {
    // TODO: should those be safe?
    // On one hand, they are unsafe, because they allow non-memory safe stuff to happen,
    //   like segfaulting and writing to random regions of the memory
    // On the other hand, this is really contained to the guest address space
    //   and we are not prohibited from doing any kind of despotism with it
    // segfaults in this case are __kind of__ deterministic, so no UB there
    // otherwise every glue operation with the target address space would have
    // to be unsafe which is noisy
    fn read_with<N: FromMemory<Ctx>, Ctx: Copy>(
        &self,
        ctx: Ctx,
        ptr: TargetPtrRepr,
    ) -> Result<N, N::Error>;
    fn write_with<N: IntoMemory<Ctx>, Ctx: Copy>(
        &self,
        ctx: Ctx,
        value: N,
        ptr: TargetPtrRepr,
    ) -> Result<(), N::Error>;
}

/// Untyped fat target pointer
///
/// Stores memory context inside, along with the pointer value
/// Needs wrapping to provide any meaningful
#[derive(Copy, Clone)]
struct RawPtr<MCtx: MemoryCtx> {
    pub context: MCtx,
    pub value: TargetPtrRepr,
}

impl<MCtx: MemoryCtx> RawPtr<MCtx> {
    pub fn read_with<N: FromMemory<Ctx>, Ctx: Copy>(&self, ctx: Ctx) -> Result<N, N::Error> {
        self.context.read_with(ctx, self.value)
    }
    pub fn write_with<N: IntoMemory<Ctx>, Ctx: Copy>(
        &self,
        ctx: Ctx,
        value: N,
    ) -> Result<(), N::Error> {
        self.context.write_with::<N, Ctx>(ctx, value, self.value)
    }

    pub fn offset(&self, offset: TargetPtrDiffRepr) -> Self {
        let res = if offset < 0 {
            self.value - ((-offset) as u32)
        } else {
            self.value + (offset as u32)
        };

        Self {
            context: self.context,
            value: res,
        }
    }
}

#[derive(Copy, Clone)]
struct MutPtr<MCtx: MemoryCtx>(RawPtr<MCtx>);

impl<MCtx: MemoryCtx> MutPtr<MCtx> {}

#[cfg(test)]
mod tests {
    use crate::{FromMemory, IntoMemory, MemoryCtx, RawPtr, TargetPtrRepr};

    #[derive(Clone, Copy)]
    struct DummyCtx();

    impl MemoryCtx for DummyCtx {
        fn read_with<N: FromMemory<Ctx>, Ctx: Copy>(
            &self,
            ctx: Ctx,
            _ptr: TargetPtrRepr,
        ) -> Result<N, N::Error> {
            let size = N::size_with(&ctx);
            let data = vec![0u8; size];
            let data = data.leak(); // really dummy impl...
            N::try_from_ctx(data, ctx)
        }

        fn write_with<N: IntoMemory<Ctx>, Ctx: Copy>(
            &self,
            ctx: Ctx,
            value: N,
            _ptr: TargetPtrRepr,
        ) -> Result<(), N::Error> {
            let size = N::size_with(&ctx);
            let mut data = vec![0u8; size];

            value.try_into_ctx(data.as_mut_slice(), ctx)
        }
    }

    #[test]
    fn read_write() {
        let ctx = DummyCtx();
        let ptr = RawPtr {
            value: 0,
            context: ctx,
        };

        assert_eq!(ptr.read_with::<u32, ()>(()).unwrap(), 0);
        ptr.write_with((), 12u32).unwrap();

        let ptr1 = ptr.offset(12);

        assert_eq!(ptr1.read_with::<u32, ()>(()).unwrap(), 0);
        ptr1.write_with((), 12u32).unwrap();
    }
}
