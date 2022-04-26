use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        gen_sys_handle(def, gen)
    } else {
        gen_win_handle(def, gen)
    }
}

pub fn gen_sys_handle(def: &TypeDef, gen: &Gen) -> TokenStream {
    let ident = gen_ident(def.name());
    let signature = gen_signature(def, gen);

    quote! {
        pub type #ident = #signature;
    }
}

pub fn gen_win_handle(def: &TypeDef, gen: &Gen) -> TokenStream {
    // TODO: what is it used for?

    let name = def.name();
    let ident = gen_ident(def.name());
    let signature = gen_signature(def, gen);

    let tokens = quote! {
        // Unfortunately, Rust requires these to be derived to allow constant patterns.
        #[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
        pub struct #ident(pub #signature);
        impl #ident {
            pub fn is_invalid(&self) -> bool {
                *self == unsafe { ::core::mem::zeroed() }
            }
        }
        impl ::core::default::Default for #ident {
            fn default() -> Self {
                unsafe { ::core::mem::zeroed() }
            }
        }
        impl ::core::clone::Clone for #ident {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl ::core::marker::Copy for #ident {}
        impl ::core::hash::Hash for #ident {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
        impl ::core::fmt::Debug for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple(#name).field(&self.0).finish()
            }
        }
        impl FromIntoMemory for #ident {
            fn from_bytes(from: &[u8]) -> Self {
                Self(<#signature as FromIntoMemory>::from_bytes(from))
            }
            fn into_bytes(self, into: &mut [u8]) {
                FromIntoMemory::into_bytes(self.0, into)
            }
            fn size() -> usize {
                std::mem::size_of::<#signature>()
            }
        }
    };

    tokens
}

fn gen_signature(def: &TypeDef, gen: &Gen) -> TokenStream {
    let def = def
        .fields()
        .next()
        .map(|field| field.get_type(Some(def)))
        .unwrap();
    gen_default_type(&def, gen)
}
