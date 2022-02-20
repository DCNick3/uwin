
type TargetPtrRepr = u32;
type TargetPtrDiffRepr = i32;

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
    fn read(&self, ptr: TargetPtrRepr, into: &mut [u8]);
    fn write(&self, from: &[u8], ptr: TargetPtrRepr);
}

/// Untyped fat target pointer
///
/// Stores memory context inside, along with the pointer value
/// Needs wrapping to provide any meaningful
#[derive(Copy, Clone)]
struct RawPtr<Ctx: MemoryCtx> {
    pub context: Ctx,
    pub value: TargetPtrRepr,
}

impl<Ctx: MemoryCtx> RawPtr<Ctx> {
    pub fn read(&self, into: &mut [u8]) {
        self.context.read(self.value, into)
    }
    pub fn write(&self, from: &[u8]) {
        self.context.write(from, self.value)
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

#[cfg(test)]
mod tests {
    use crate::{MemoryCtx, RawPtr, TargetPtrRepr};

    #[derive(Clone, Copy)]
    struct DummyCtx();
    impl MemoryCtx for DummyCtx {
        fn read(&self, _ptr: TargetPtrRepr, into: &mut [u8]) {
            into.fill(0)
        }

        fn write(&self, _from: &[u8], _ptr: TargetPtrRepr) {}
    }

    #[test]
    fn read_write() {
        let ctx = DummyCtx();
        let ptr = RawPtr {
            value: 0,
            context: ctx
        };

        let mut data = [12u8; 12];

        ptr.read(&mut data);
        assert_eq!(data, [0u8; 12]);

        ptr.write(&data);

        let ptr1 = ptr.offset(12);

        let mut data = [12u8; 12];

        ptr1.read(&mut data);
        assert_eq!(data, [0u8; 12]);

        ptr1.write(&data);
    }
}