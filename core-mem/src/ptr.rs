use crate::conv::FromIntoMemory;
use crate::ctx::MemoryCtx;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub type PtrRepr = u32;
pub type PtrDiffRepr = i32;

/// Untyped fat target pointer
///
/// Stores memory context inside, along with the pointer value
/// Needs wrapping to provide any meaningful
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct RawPtr {
    pub value: PtrRepr,
}

impl Debug for RawPtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ptr {:#010x}", self.value)
    }
}

impl RawPtr {
    const fn new(ptr: PtrRepr) -> Self {
        Self { value: ptr }
    }
}

impl RawPtr {
    pub fn read_with<N: FromIntoMemory, MCtx: MemoryCtx>(&self, ctx: MCtx) -> N {
        ctx.read(self.value)
    }
    pub fn write_with<N: FromIntoMemory, MCtx: MemoryCtx>(&self, ctx: MCtx, value: N) {
        ctx.write::<N>(value, self.value)
    }

    pub fn read_bytes(&self, ctx: impl MemoryCtx, count: PtrRepr) -> Vec<u8> {
        let mut res = Vec::new();
        res.reserve(count as usize);
        for i in 0..count {
            res.push(self.offset(i as PtrDiffRepr).read_with(ctx));
        }
        res
    }

    pub fn write_bytes(&self, ctx: impl MemoryCtx, bytes: &[u8]) {
        for (i, &val) in bytes.iter().enumerate() {
            self.offset(i as PtrDiffRepr).write_with(ctx, val)
        }
    }

    pub fn offset(&self, offset: PtrDiffRepr) -> Self {
        let res = if offset < 0 {
            self.value - ((-offset) as u32)
        } else {
            self.value + (offset as u32)
        };

        Self { value: res }
    }
}

pub struct MutPtr<T>(RawPtr, PhantomData<T>);

impl<T> MutPtr<T> {
    pub const fn new(val: PtrRepr) -> Self {
        Self(RawPtr::new(val), PhantomData {})
    }
    pub fn repr(&self) -> PtrRepr {
        self.0.value
    }
    pub const fn null() -> Self {
        Self::new(0)
    }
    pub fn pun<T1>(&self) -> MutPtr<T1> {
        MutPtr(self.0, Default::default())
    }
}
impl<T: FromIntoMemory> MutPtr<T> {
    pub fn read_with<N: FromIntoMemory, MCtx: MemoryCtx>(&self, ctx: MCtx) -> N {
        self.0.read_with(ctx)
    }
    pub fn write_with<N: FromIntoMemory, MCtx: MemoryCtx>(&self, ctx: MCtx, value: N) {
        self.0.write_with(ctx, value)
    }

    pub fn offset(&self, offset: PtrDiffRepr) -> Self {
        let size: PtrDiffRepr = T::size().try_into().unwrap();
        Self(self.0.offset(offset * size), Default::default())
    }
}
impl MutPtr<u8> {
    pub fn read_bytes(&self, ctx: impl MemoryCtx, count: PtrRepr) -> Vec<u8> {
        self.0.read_bytes(ctx, count)
    }
    pub fn write_bytes(&self, ctx: impl MemoryCtx, bytes: &[u8]) {
        self.0.write_bytes(ctx, bytes)
    }
}

impl<T> Copy for MutPtr<T> {}
impl<T> Clone for MutPtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Hash for MutPtr<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.value.hash(state)
    }
}
impl<T> PartialEq for MutPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for MutPtr<T> {}
impl<T> Debug for MutPtr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> FromIntoMemory for MutPtr<T> {
    fn from_bytes(from: &[u8]) -> Self {
        MutPtr::new(<PtrRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0.value, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrRepr>()
    }
}

pub struct ConstPtr<T>(RawPtr, PhantomData<T>);

impl<T> ConstPtr<T> {
    pub const fn new(val: PtrRepr) -> Self {
        Self(RawPtr::new(val), PhantomData {})
    }
    pub fn repr(&self) -> PtrRepr {
        self.0.value
    }
    pub const fn null() -> Self {
        Self::new(0)
    }
    pub fn pun<T1>(&self) -> ConstPtr<T1> {
        ConstPtr(self.0, Default::default())
    }
}
impl ConstPtr<u8> {
    pub fn read_bytes(&self, ctx: impl MemoryCtx, count: PtrRepr) -> Vec<u8> {
        self.0.read_bytes(ctx, count)
    }
}
impl<T: FromIntoMemory> ConstPtr<T> {
    pub fn read_with<N: FromIntoMemory, MCtx: MemoryCtx>(&self, ctx: MCtx) -> N {
        self.0.read_with(ctx)
    }

    pub fn offset(&self, offset: PtrDiffRepr) -> Self {
        let size: PtrDiffRepr = T::size().try_into().unwrap();
        Self(self.0.offset(offset * size), Default::default())
    }
}

impl<T> Eq for ConstPtr<T> {}
impl<T> Copy for ConstPtr<T> {}
impl<T> Clone for ConstPtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Hash for ConstPtr<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.value.hash(state)
    }
}
impl<T> PartialEq for ConstPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Debug for ConstPtr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> FromIntoMemory for ConstPtr<T> {
    fn from_bytes(from: &[u8]) -> Self {
        ConstPtr::new(<PtrRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0.value, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrRepr>()
    }
}
