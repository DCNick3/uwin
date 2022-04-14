use std::ptr::copy_nonoverlapping;

// many ideas borrowed from wonderful crate "scroll", but API tailored for uwin use-case

pub trait FromIntoMemory: Sized {
    fn from_bytes(from: &[u8]) -> Self;
    fn into_bytes(self, into: &mut [u8]);
    fn size() -> usize;
}

macro_rules! from_into_mem_impl {
    ($typ:tt, $size:expr) => {
        impl<'a> FromIntoMemory for $typ {
            #[inline]
            fn into_bytes(self, dst: &mut [u8]) {
                assert!(dst.len() >= $size);
                unsafe {
                    let bytes = self.to_le_bytes();
                    copy_nonoverlapping((&bytes).as_ptr(), dst.as_mut_ptr(), $size);
                }
            }

            #[inline]
            fn from_bytes(src: &[u8]) -> Self {
                $typ::from_le_bytes(src.try_into().unwrap())
            }

            #[inline]
            fn size() -> usize {
                $size
            }
        }
    };
}

from_into_mem_impl!(u8, 1);
from_into_mem_impl!(i8, 1);
from_into_mem_impl!(u16, 2);
from_into_mem_impl!(i16, 2);
from_into_mem_impl!(u32, 4);
from_into_mem_impl!(i32, 4);
from_into_mem_impl!(u64, 8);
from_into_mem_impl!(i64, 8);
from_into_mem_impl!(u128, 16);
from_into_mem_impl!(i128, 16);

impl FromIntoMemory for () {
    fn from_bytes(_: &[u8]) -> Self {}

    fn into_bytes(self, _: &mut [u8]) {}

    fn size() -> usize {
        0
    }
}
