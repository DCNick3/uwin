use super::*;

pub fn gen() -> TokenStream {
    let underlying = quote!(PtrRepr);

    quote! {
        pub struct HANDLE(pub #underlying);
        impl ::core::default::Default for HANDLE {
            fn default() -> Self {
                Self(0)
            }
        }
        impl ::core::clone::Clone for HANDLE {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl ::core::marker::Copy for HANDLE {}
        impl ::core::cmp::PartialEq for HANDLE {
            fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
            }
        }
        impl ::core::cmp::Eq for HANDLE {}
        impl ::core::fmt::Debug for HANDLE {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple("HANDLE").field(&self.0).finish()
            }
        }

        impl FromIntoMemory for HANDLE {
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
