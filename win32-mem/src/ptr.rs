use crate::conv::FromIntoMemory;
use crate::ctx::{DefaultMemoryCtx, MemoryCtx};
use std::marker::PhantomData;

pub type PtrRepr = u32;
pub type PtrDiffRepr = i32;

/// Untyped fat target pointer
///
/// Stores memory context inside, along with the pointer value
/// Needs wrapping to provide any meaningful
#[derive(Copy, Clone)]
pub struct RawPtr<MCtx: MemoryCtx = DefaultMemoryCtx> {
    pub context: MCtx,
    pub value: PtrRepr,
}

impl<MCtx: MemoryCtx> RawPtr<MCtx> {
    fn new(ctx: MCtx, ptr: PtrRepr) -> Self {
        Self {
            context: ctx,
            value: ptr,
        }
    }
}

impl<MCtx: MemoryCtx> RawPtr<MCtx> {
    pub fn read<N: FromIntoMemory>(&self) -> N {
        self.context.read(self.value)
    }
    pub fn write<N: FromIntoMemory>(&self, value: N) {
        self.context.write::<N>(value, self.value)
    }

    pub fn offset(&self, offset: PtrDiffRepr) -> Self {
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
pub struct MutPtr<T: FromIntoMemory, MCtx: MemoryCtx = DefaultMemoryCtx>(
    RawPtr<MCtx>,
    PhantomData<T>,
);

impl<MCtx: MemoryCtx, T: FromIntoMemory> MutPtr<T, MCtx> {
    pub fn new(ctx: MCtx, val: PtrRepr) -> Self {
        Self(RawPtr::new(ctx, val), Default::default())
    }

    pub fn read(&self) -> T {
        self.0.read::<T>()
    }

    pub fn write(&self, value: T) {
        self.0.write::<T>(value)
    }
}

#[derive(Copy, Clone)]
pub struct ConstPtr<T: FromIntoMemory, MCtx: MemoryCtx = DefaultMemoryCtx>(
    RawPtr<MCtx>,
    PhantomData<T>,
);

impl<MCtx: MemoryCtx, T: FromIntoMemory> ConstPtr<T, MCtx> {
    pub fn new(ctx: MCtx, val: PtrRepr) -> Self {
        Self(RawPtr::new(ctx, val), Default::default())
    }

    pub fn read(&self) -> T {
        self.0.read::<T>()
    }
}
