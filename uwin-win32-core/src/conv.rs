use std::ptr::copy_nonoverlapping;

// many ideas borrowed from wonderful crate "scroll", but API tailored for uwin use-case

pub trait FromIntoMemory: Sized {
    type Error;
    fn try_from_bytes(from: &[u8]) -> Result<Self, Self::Error>;
    fn try_into_bytes(self, into: &mut [u8]) -> Result<(), Self::Error>;
    fn size() -> usize;
}

macro_rules! from_into_mem_impl {
    ($typ:tt, $size:expr) => {
        impl<'a> FromIntoMemory for $typ {
            type Error = ();

            #[inline]
            fn try_into_bytes(self, dst: &mut [u8]) -> Result<(), Self::Error> {
                assert!(dst.len() >= $size);
                unsafe {
                    let bytes = self.to_le_bytes();
                    copy_nonoverlapping((&bytes).as_ptr(), dst.as_mut_ptr(), $size);
                }
                Ok(())
            }

            #[inline]
            fn try_from_bytes(src: &[u8]) -> Result<Self, Self::Error> {
                Ok($typ::from_le_bytes(src.try_into().unwrap()))
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
