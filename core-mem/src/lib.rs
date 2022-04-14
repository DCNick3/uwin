pub mod conv;
pub mod ctx;
pub mod ptr;
pub mod thread_ctx;

#[cfg(test)]
mod tests {
    use crate::conv::FromIntoMemory;
    use crate::ctx::MemoryCtx;
    use crate::ptr::{PtrRepr, RawPtr};

    #[derive(Clone, Copy, PartialEq)]
    struct DummyCtx();

    impl MemoryCtx for DummyCtx {
        fn read<N: FromIntoMemory>(&self, _ptr: PtrRepr) -> N {
            let size = N::size();
            let data = vec![0u8; size];
            let data = data.leak(); // really dummy impl...
            N::from_bytes(data)
        }

        fn write<N: FromIntoMemory>(&self, value: N, _ptr: PtrRepr) {
            let size = N::size();
            let mut data = vec![0u8; size];

            value.into_bytes(data.as_mut_slice())
        }
    }

    #[test]
    fn read_write() {
        let ctx = DummyCtx();
        let ptr = RawPtr { value: 0 };

        assert_eq!(ptr.read_with::<u32, _>(ctx), 0u32);
        ptr.write_with(ctx, 12u32);

        let ptr1 = ptr.offset(12);

        assert_eq!(ptr1.read_with::<u32, _>(ctx), 0u32);
        ptr1.write_with(ctx, 12u32);
    }
}
