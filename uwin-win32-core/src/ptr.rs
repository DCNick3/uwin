use crate::conv::FromIntoMemory;
use crate::ctx::MemoryCtx;
use std::marker::PhantomData;

pub type TargetPtrRepr = u32;
type TargetPtrDiffRepr = i32;

/// Untyped fat target pointer
///
/// Stores memory context inside, along with the pointer value
/// Needs wrapping to provide any meaningful
#[derive(Copy, Clone)]
pub struct RawPtr<'a, MCtx: MemoryCtx<'a>> {
    pub context: MCtx,
    pub value: TargetPtrRepr,
    pub _phantom: PhantomData<&'a ()>,
}

impl<'a, MCtx: MemoryCtx<'a>> RawPtr<'a, MCtx> {
    pub fn read<N: FromIntoMemory>(&self) -> Result<N, N::Error> {
        self.context.read(self.value)
    }
    pub fn write<N: FromIntoMemory>(&self, value: N) -> Result<(), N::Error> {
        self.context.write::<N>(value, self.value)
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
            _phantom: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
struct MutPtr<'a, MCtx: MemoryCtx<'a>, T: FromIntoMemory>(RawPtr<'a, MCtx>, PhantomData<T>);

impl<'a, MCtx: MemoryCtx<'a>, T: FromIntoMemory> MutPtr<'a, MCtx, T> {
    pub fn read(&self) -> Result<T, T::Error> {
        self.0.read::<T>()
    }

    pub fn write(&self, value: T) -> Result<(), T::Error> {
        self.0.write::<T>(value)
    }
}

#[derive(Copy, Clone)]
struct ConstPtr<'a, MCtx: MemoryCtx<'a>, T: FromIntoMemory>(RawPtr<'a, MCtx>, PhantomData<T>);

impl<'a, MCtx: MemoryCtx<'a>, T: FromIntoMemory> ConstPtr<'a, MCtx, T> {
    pub fn read(&self) -> Result<T, T::Error> {
        self.0.read::<T>()
    }
}
