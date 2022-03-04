pub mod conv;
pub mod ctx;
pub mod ptr;

#[cfg(test)]
mod tests {
    use crate::conv::FromIntoMemory;
    use crate::ctx::MemoryCtx;
    use crate::ptr::{PtrRepr, RawPtr};

    #[derive(Clone, Copy)]
    struct DummyCtx();

    impl MemoryCtx for DummyCtx {
        fn read<N: FromIntoMemory>(&self, _ptr: PtrRepr) -> N {
            let size = N::size();
            let data = vec![0u8; size];
            let data = data.leak(); // really dummy impl...
            N::try_from_bytes(data)
        }

        fn write<N: FromIntoMemory>(&self, value: N, _ptr: PtrRepr) {
            let size = N::size();
            let mut data = vec![0u8; size];

            value.try_into_bytes(data.as_mut_slice())
        }
    }

    #[test]
    fn read_write() {
        let ctx = DummyCtx();
        let ptr = RawPtr {
            value: 0,
            context: ctx,
        };

        assert_eq!(ptr.read::<u32>(), 0);
        ptr.write(12u32);

        let ptr1 = ptr.offset(12);

        assert_eq!(ptr1.read::<u32>(), 0);
        ptr1.write(12u32);
    }
}
