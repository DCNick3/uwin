use crate::conv::FromIntoMemory;
use crate::ctx::{DefaultMemoryCtx, MemoryCtx};
use std::marker::PhantomData;

pub type TargetPtrRepr = u32;
type TargetPtrDiffRepr = i32;

/// Untyped fat target pointer
///
/// Stores memory context inside, along with the pointer value
/// Needs wrapping to provide any meaningful
#[derive(Copy, Clone)]
pub struct RawPtr<'a, MCtx: MemoryCtx<'a> = DefaultMemoryCtx<'a>> {
    pub context: MCtx,
    pub value: TargetPtrRepr,
    pub _phantom: PhantomData<&'a ()>,
}

impl<'a, MCtx: MemoryCtx<'a>> RawPtr<'a, MCtx> {
    fn new(ctx: MCtx, ptr: TargetPtrRepr) -> Self {
        Self {
            context: ctx,
            value: ptr,
            _phantom: Default::default(),
        }
    }
}

impl<'a, MCtx: MemoryCtx<'a>> RawPtr<'a, MCtx> {
    pub fn read<N: FromIntoMemory>(&self) -> N {
        self.context.read(self.value)
    }
    pub fn write<N: FromIntoMemory>(&self, value: N) {
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
pub struct MutPtr<'a, T: FromIntoMemory, MCtx: MemoryCtx<'a> = DefaultMemoryCtx<'a>>(
    RawPtr<'a, MCtx>,
    PhantomData<T>,
);

impl<'a, MCtx: MemoryCtx<'a>, T: FromIntoMemory> MutPtr<'a, T, MCtx> {
    pub fn new(ctx: MCtx, val: TargetPtrRepr) -> Self {
        Self {
            0: RawPtr::new(ctx, val),
            1: Default::default(),
        }
    }

    pub fn read(&self) -> T {
        self.0.read::<T>()
    }

    pub fn write(&self, value: T) {
        self.0.write::<T>(value)
    }
}

#[derive(Copy, Clone)]
pub struct ConstPtr<'a, T: FromIntoMemory, MCtx: MemoryCtx<'a> = DefaultMemoryCtx<'a>>(
    RawPtr<'a, MCtx>,
    PhantomData<T>,
);

impl<'a, MCtx: MemoryCtx<'a>, T: FromIntoMemory> ConstPtr<'a, T, MCtx> {
    pub fn new(ctx: MCtx, val: TargetPtrRepr) -> Self {
        Self {
            0: RawPtr::new(ctx, val),
            1: Default::default(),
        }
    }

    pub fn read(&self) -> T {
        self.0.read::<T>()
    }
}
