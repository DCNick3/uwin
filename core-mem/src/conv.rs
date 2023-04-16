use std::mem::MaybeUninit;
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
                assert!(dst.len() == $size);
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
from_into_mem_impl!(f32, 4);
from_into_mem_impl!(f64, 8);

impl FromIntoMemory for () {
    fn from_bytes(_: &[u8]) -> Self {}

    fn into_bytes(self, _: &mut [u8]) {}

    fn size() -> usize {
        0
    }
}

#[macro_export]
macro_rules! from_into_mem_impl_for_wrapper {
    ($typ:tt, $underlying:ty) => {
        impl core_mem::conv::FromIntoMemory for $typ {
            #[inline]
            fn from_bytes(from: &[u8]) -> Self {
                Self(<$underlying>::from_bytes(from))
            }

            #[inline]
            fn into_bytes(self, into: &mut [u8]) {
                self.0.into_bytes(into)
            }

            #[inline]
            fn size() -> usize {
                <$underlying>::size()
            }
        }
    };
}

/// Option is used (currently) for function pointers, so I use this to stub them out
impl<T> FromIntoMemory for Option<T> {
    fn from_bytes(_: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, _: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}

impl<T: FromIntoMemory, const N: usize> FromIntoMemory for [T; N] {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), Self::size());

        let mut res: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        let sz = T::size();
        for (value, chunk) in res.iter_mut().zip(from.chunks_exact(sz)) {
            value.write(T::from_bytes(chunk));
        }

        unsafe { std::mem::transmute_copy::<_, [T; N]>(&res) }
    }

    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), Self::size());
        let sz = T::size();
        for (value, chunk) in self.into_iter().zip(into.chunks_exact_mut(sz)) {
            T::into_bytes(value, chunk)
        }
    }

    fn size() -> usize {
        T::size() * N
    }
}
