use super::*;

pub fn gen() -> TokenStream {
    let underlying = quote!(i32);

    quote! {
        pub struct BOOL(pub #underlying);

        impl BOOL {
            #[inline]
            pub fn as_bool(self) -> bool {
                self.0 != 0
            }
        }

        impl ::core::default::Default for BOOL {
            fn default() -> Self {
                Self(0)
            }
        }

        impl ::core::clone::Clone for BOOL {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl ::core::marker::Copy for BOOL {}

        impl ::core::cmp::PartialEq for BOOL {
            fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
            }
        }

        impl ::core::cmp::Eq for BOOL {}

        impl ::core::fmt::Debug for BOOL {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple("BOOL").field(&self.0).finish()
            }
        }

        impl ::core::convert::From<BOOL> for bool {
            fn from(value: BOOL) -> Self {
                value.as_bool()
            }
        }

        impl ::core::convert::From<&BOOL> for bool {
            fn from(value: &BOOL) -> Self {
                value.as_bool()
            }
        }

        impl ::core::convert::From<bool> for BOOL {
            fn from(value: bool) -> Self {
                if value {
                    BOOL(1)
                } else {
                    BOOL(0)
                }
            }
        }

        impl ::core::convert::From<&bool> for BOOL {
            fn from(value: &bool) -> Self {
                (*value).into()
            }
        }

        impl ::core::cmp::PartialEq<bool> for BOOL {
            fn eq(&self, other: &bool) -> bool {
                self.as_bool() == *other
            }
        }

        impl ::core::cmp::PartialEq<BOOL> for bool {
            fn eq(&self, other: &BOOL) -> bool {
                *self == other.as_bool()
            }
        }

        impl ::core::ops::Not for BOOL {
            type Output = Self;
            fn not(self) -> Self::Output {
                if self.as_bool() {
                    BOOL(0)
                } else {
                    BOOL(1)
                }
            }
        }

        impl FromIntoMemory for BOOL {
            fn from_bytes(from: &[u8]) -> Self {
                Self(<#underlying as FromIntoMemory>::from_bytes(from))
            }
            fn into_bytes(self, into: &mut [u8]) {
                FromIntoMemory::into_bytes(self.0, into)
            }
            fn size() -> usize {
                std::mem::size_of::<#underlying>()
            }
        }
    }
}
