use super::*;

pub fn gen() -> TokenStream {
    quote! {
        pub struct HANDLE(pub PtrRepr);
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
    }
}
