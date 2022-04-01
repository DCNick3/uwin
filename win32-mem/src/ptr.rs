use crate::conv::FromIntoMemory;
use crate::ctx::{DefaultMemoryCtx, MemoryCtx};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

pub type PtrRepr = u32;
pub type PtrDiffRepr = i32;

/// Untyped fat target pointer
///
/// Stores memory context inside, along with the pointer value
/// Needs wrapping to provide any meaningful
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RawPtr<MCtx: MemoryCtx = DefaultMemoryCtx> {
    pub context: MCtx,
    pub value: PtrRepr,
}

impl<MCtx: MemoryCtx> Debug for RawPtr<MCtx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ptr {:#010x}", self.value)
    }
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

#[derive(Eq)]
pub struct MutPtr<T, MCtx: MemoryCtx = DefaultMemoryCtx>(RawPtr<MCtx>, PhantomData<T>);

impl<T, MCtx: MemoryCtx> MutPtr<T, MCtx> {
    pub fn new(ctx: MCtx, val: PtrRepr) -> Self {
        Self(RawPtr::new(ctx, val), Default::default())
    }
}
impl<T: FromIntoMemory, MCtx: MemoryCtx> MutPtr<T, MCtx> {
    pub fn read(&self) -> T {
        self.0.read::<T>()
    }

    pub fn write(&self, value: T) {
        self.0.write::<T>(value)
    }
}

impl<T, MCtx: MemoryCtx> Copy for MutPtr<T, MCtx> {}
impl<T, MCtx: MemoryCtx> Clone for MutPtr<T, MCtx> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T, MCtx: MemoryCtx> PartialEq for MutPtr<T, MCtx> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T, MCtx: MemoryCtx> Debug for MutPtr<T, MCtx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Eq)]
pub struct ConstPtr<T, MCtx: MemoryCtx = DefaultMemoryCtx>(RawPtr<MCtx>, PhantomData<T>);

impl<T, MCtx: MemoryCtx> ConstPtr<T, MCtx> {
    pub fn new(ctx: MCtx, val: PtrRepr) -> Self {
        Self(RawPtr::new(ctx, val), Default::default())
    }
}
impl<T: FromIntoMemory, MCtx: MemoryCtx> ConstPtr<T, MCtx> {
    pub fn read(&self) -> T {
        self.0.read::<T>()
    }
}

impl<T, MCtx: MemoryCtx> Copy for ConstPtr<T, MCtx> {}
impl<T, MCtx: MemoryCtx> Clone for ConstPtr<T, MCtx> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T, MCtx: MemoryCtx> PartialEq for ConstPtr<T, MCtx> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T, MCtx: MemoryCtx> Debug for ConstPtr<T, MCtx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
