use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    if def.is_api_contract() {
        return quote! {};
    }

    if !gen.sys {
        if let Some(replacement) = replacements::gen(def) {
            return replacement;
        }
    }

    if def.is_handle() {
        return handles::gen(def, gen);
    }

    gen_struct_with_name(def, def.name(), &Cfg::new(), gen)
}

fn gen_struct_with_name(def: &TypeDef, struct_name: &str, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let name = gen_ident(struct_name);

    // println!("  {:50}", struct_name);

    if def.fields().next().is_none() {
        if let Some(guid) = GUID::from_attributes(def.attributes()) {
            let value = gen_guid(&guid, gen);
            let guid = gen_element_name(&Type::GUID, gen);
            return quote! { pub const #name: #guid = #value; };
        } else if name.as_str().ends_with("Vtbl") {
            // This just omits some useless struct declarations like `IDDVideoPortContainerVtbl`
            return quote! {};
        } else {
            return quote! {
                pub struct #name(pub u8);
            };
        }
    }

    let is_union = def.is_union();
    let cfg = cfg.union(&def.cfg());

    let fields = def.fields().map(|f| {
        let name = gen_ident(f.name());
        let ty = f.get_type(Some(def));
        let ty = gen_default_type(&ty, gen);

        #[allow(clippy::if_same_then_else)]
        if f.is_literal() {
            quote! {}
        } else if !gen.sys && is_union && !f.is_blittable(Some(def)) {
            // TODO: unions are funky
            quote! {}
        } else {
            quote! { pub #name: #ty, }
        }
    });

    let body = if is_union {
        let size = format!("{}", def.layout().size);
        let size = TokenStream::from(size);
        quote! { data: [u8; #size] }
    } else {
        quote! { #(#fields)* }
    };

    let doc = gen.doc(&cfg);
    let features = gen.cfg(&cfg);

    let mut tokens = quote! {
        #doc
        #features
        pub struct #name { #body }
    };

    tokens.combine(&gen_struct_constants(def, &name, &cfg, gen));
    tokens.combine(&gen_default(def, &name, &cfg, gen));
    tokens.combine(&gen_copy_clone(def, &name, &cfg, gen));
    tokens.combine(&gen_debug(def, &name, &cfg, gen));
    tokens.combine(&gen_compare_traits(def, &name, &cfg, gen));
    tokens.combine(&gen_from_into_mem(def, &name, &cfg, gen));

    // if !gen.sys {
    //     tokens.combine(&quote! {
    //         #features
    //         impl ::core::default::Default for #name {
    //             fn default() -> Self {
    //                 unsafe { ::core::mem::zeroed() }
    //             }
    //         }
    //     });
    // }

    if let Some(nested_types) = def.nested_types() {
        for (index, (_, nested_type)) in nested_types.iter().enumerate() {
            let nested_name = format!("{}_{}", struct_name, index);
            tokens.combine(&gen_struct_with_name(nested_type, &nested_name, &cfg, gen));
        }
    }

    tokens
}

fn gen_from_into_mem(def: &TypeDef, name: &TokenStream, cfg: &Cfg, gen: &Gen) -> TokenStream {
    // TODO: generate actual implementation
    let features = gen.cfg(cfg);

    let struct_size = def.layout().size;
    let struct_size = TokenStream::from(struct_size.to_string());

    if def.is_union() {
        return quote! {
            #features
            impl FromIntoMemory for #name {
                fn from_bytes(from: &[u8]) -> Self {
                    let mut data = [0u8; #struct_size];
                    <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
                    Self { data }
                }
                fn into_bytes(self, into: &mut [u8]) {
                    into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
                }
                fn size() -> usize {
                    #struct_size
                }
            }
        };
    }

    let field_sizes = def
        .fields()
        .map(|f| {
            let ty = f.get_type(Some(def));
            TokenStream::from(ty.layout().size.to_string())
        })
        .collect::<Vec<_>>();

    let field_offsets = def.field_offsets();

    let field_names = def.fields().map(|f| gen_ident(f.name()));

    // damn it, some field names collide with constant names and it blows up
    // so we prefix local variable names with some crud...
    let field_value_names = def
        .fields()
        .map(|f| gen_ident(&format!("f_{}", f.name())))
        .collect::<Vec<_>>();

    let field_into_bytes = def
        .fields()
        .zip(field_sizes.iter())
        .zip(&field_offsets)
        .map(|((f, size), &offset)| {
            let name = gen_ident(f.name());
            let offset = TokenStream::from(offset.to_string());
            quote! {
                FromIntoMemory::into_bytes(self.#name, &mut into[#offset..#offset+#size]);
            }
        });

    let field_from_bytes = def
        .fields()
        .zip(field_sizes.iter())
        .zip(field_value_names.iter())
        .zip(&field_offsets)
        .map(|(((f, size), var_name), &offset)| {
            let ty = f.get_type(Some(def));
            let ty = gen_default_type(&ty, gen);
            let offset = TokenStream::from(offset.to_string());
            quote! {
                let #var_name = <#ty as FromIntoMemory>::from_bytes(&from[#offset..#offset+#size]);
            }
        });

    // TODO: this is all and good, but how about alignment?

    quote! {
        #features
        impl FromIntoMemory for #name {
            fn from_bytes(from: &[u8]) -> Self {
                assert_eq!(from.len(), #struct_size);

                #(#field_from_bytes)*
                Self {
                    #(#field_names: #field_value_names),*
                }
            }
            fn into_bytes(self, into: &mut [u8]) {
                assert_eq!(into.len(), #struct_size);

                #(#field_into_bytes)*
            }
            fn size() -> usize {
                #struct_size
            }
        }
    }
}

#[allow(clippy::if_same_then_else)]
fn gen_compare_traits(def: &TypeDef, name: &TokenStream, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let features = gen.cfg(cfg);

    if def.is_union() {
        quote! {
            #features
            impl ::core::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.data == other.data
                }
            }
            #features
            impl ::core::cmp::Eq for #name {}
        }
    } else {
        let fields = def.fields().map(|f| {
            let name = gen_ident(f.name());
            if f.is_literal() {
                quote! {}
            } else {
                quote! { self.#name == other.#name }
            }
        });

        quote! {
            #features
            impl ::core::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    #(#fields)&&*
                }
            }
            #features
            impl ::core::cmp::Eq for #name {}
        }
    }
}

fn gen_debug(def: &TypeDef, ident: &TokenStream, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let name = ident.as_str();
    let features = gen.cfg(cfg);
    if gen.sys {
        quote! {}
    } else if def.is_union() {
        quote! {
            #features
            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct(#name)
                        .field("data", &self.data)
                        .finish()
                }
            }
        }
    } else {
        let fields = def.fields().map(|f| {
            if f.is_literal() {
                quote! {}
            } else {
                let name = f.name();
                let ident = gen_ident(name);
                let ty = f.get_type(Some(def));
                if ty.is_callback_array() {
                    quote! {}
                } else {
                    quote! { .field(#name, &self.#ident) }
                }
            }
        });

        quote! {
            #features
            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct(#name) #(#fields)* .finish()
                }
            }
        }
    }
}

fn gen_default(def: &TypeDef, ident: &TokenStream, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let features = gen.cfg(cfg);

    if def.is_union() {
        let size = def.layout().size;
        let size = TokenStream::from(size.to_string());
        quote! {
            #features
            impl ::core::default::Default for #ident {
                fn default() -> Self {
                    Self {
                        data: [0u8; #size]
                    }
                }
            }
        }
    } else {
        quote! {}
    }
}

fn gen_copy_clone(def: &TypeDef, name: &TokenStream, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let features = gen.cfg(cfg);

    if gen.sys || def.is_blittable() {
        quote! {
            #features
            impl ::core::marker::Copy for #name {}
            #features
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    *self
                }
            }
        }
    } else if def.is_union() {
        quote! {
            #features
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    unsafe { ::core::mem::transmute_copy(self) }
                }
            }
        }
    } else if def.class_layout().is_some() {
        // Don't support copy/clone of packed structs: https://github.com/rust-lang/rust/issues/82523
        quote! {}
    } else {
        let fields = def.fields().map(|f| {
            let name = gen_ident(f.name());
            if f.is_literal() {
                quote! {}
            } else if f.is_blittable(Some(def)) {
                quote! { #name: self.#name }
            } else {
                quote! { #name: self.#name.clone() }
            }
        });

        quote! {
            #features
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self { #(#fields),* }
                }
            }
        }
    }
}

fn gen_struct_constants(
    def: &TypeDef,
    struct_name: &TokenStream,
    cfg: &Cfg,
    gen: &Gen,
) -> TokenStream {
    let features = gen.cfg(cfg);

    let constants = def.fields().filter_map(|f| {
        if f.is_literal() {
            if let Some(constant) = f.constant() {
                let name = gen_ident(f.name());
                let value = gen_constant_type_value(&constant.value());

                return Some(quote! {
                    pub const #name: #value;
                });
            }
        }

        None
    });

    let mut tokens = quote! { #(#constants)* };

    if !tokens.is_empty() {
        tokens = quote! {
            #features
            impl #struct_name {
                #tokens
            }
        };
    }

    tokens
}
