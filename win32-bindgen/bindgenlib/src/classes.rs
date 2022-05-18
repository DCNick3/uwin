use super::*;

pub fn gen_classes(gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    let type_reader = TypeReader::get();

    for class in gen.com_classes {
        tokens.combine(&gen_class(class, gen, type_reader))
    }

    tokens
}

pub fn gen_class_thunks(gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    let type_reader = TypeReader::get();

    for class in gen.com_classes {
        let class_name = &class.name;

        for iface in class.interfaces.iter() {
            let (namespace, name) = iface.rsplit_once('.').unwrap();
            let type_ = type_reader
                .get_type((namespace, name))
                .expect("Find the interface referenced by com_interfaces")
                .clone();
            let type_ = match type_ {
                Type::TypeDef(def) => def,
                _ => unimplemented!(),
            };

            fn emit_interface(
                class_name: &str,
                iface_name: &str,
                tokens: &mut TokenStream,
                interface: BaseInterface,
            ) {
                match interface {
                    BaseInterface::IUnknown => {
                        let query_interface = gen_ident(&format!(
                            "thunk_com_{}_as_{}_QueryInterface",
                            class_name, iface_name
                        ));
                        let add_ref = gen_ident(&format!(
                            "thunk_com_{}_as_{}_AddRef",
                            class_name, iface_name
                        ));
                        let release = gen_ident(&format!(
                            "thunk_com_{}_as_{}_Release",
                            class_name, iface_name
                        ));

                        tokens.combine(&quote! {
                                #[no_mangle]
                                extern "C" fn #query_interface(context: &mut ExtendedContext, memory: FlatMemoryCtx) -> PtrRepr {
                                    std::process::abort();
                                }
                                #[no_mangle]
                                extern "C" fn #add_ref(context: &mut ExtendedContext, memory: FlatMemoryCtx) -> PtrRepr {
                                    std::process::abort();
                                }
                                #[no_mangle]
                                extern "C" fn #release(context: &mut ExtendedContext, memory: FlatMemoryCtx) -> PtrRepr {
                                    std::process::abort();
                                }
                        });
                    }
                    BaseInterface::TypeDef(def) => {
                        emit_interface(class_name, iface_name, tokens, def.base_interface());

                        for method in def.methods() {
                            let method = method.name();

                            let thunk_name = gen_ident(&format!(
                                "thunk_com_{}_as_{}_{}",
                                class_name, iface_name, method
                            ));

                            tokens.combine(&quote! {
                                #[no_mangle]
                                extern "C" fn #thunk_name(context: &mut ExtendedContext, memory: FlatMemoryCtx) -> PtrRepr {
                                    std::process::abort();
                                }
                            });
                        }
                    }
                }
            }

            emit_interface(class_name, name, &mut tokens, BaseInterface::TypeDef(type_));
        }
    }

    tokens
}

pub fn gen_com_stub_params(gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    let type_reader = TypeReader::get();

    for class in gen.com_classes {
        let name = &class.name;

        let vtables = class.interfaces.iter().map(|iface| {
            let (namespace, name) = iface.rsplit_once('.').unwrap();
            let type_ = type_reader
                .get_type((namespace, name))
                .expect("Find the interface referenced by com_interfaces")
                .clone();
            let type_ = match type_ {
                Type::TypeDef(def) => def,
                _ => unimplemented!(),
            };

            let mut tokens = TokenStream::new();

            fn emit_interface(name: &str, tokens: &mut TokenStream, interface: BaseInterface) {
                match interface {
                    BaseInterface::IUnknown => {
                        tokens.combine(&quote! {
                            (#name.to_string(), "QueryInterface".to_string()),
                            (#name.to_string(), "AddRef".to_string()),
                            (#name.to_string(), "Release".to_string()),
                        });
                    }
                    BaseInterface::TypeDef(def) => {
                        emit_interface(name, tokens, def.base_interface());

                        for method in def.methods() {
                            let method = method.name();
                            tokens.combine(&quote! {
                                (#name.to_string(), #method.to_string()),
                            });
                        }
                    }
                }
            }

            emit_interface(name, &mut tokens, BaseInterface::TypeDef(type_));

            tokens
        });

        tokens.combine(&quote! {
            ComStubClassParams {
                name: #name.to_string(),
                vtables: vec![#(ComStubVtableParams {
                    function_names: vec![
                        #vtables
                    ]
                })*],
            }
        });
    }

    tokens
}

fn gen_class(class: &ComClass, gen: &Gen, type_reader: &'static TypeReader) -> TokenStream {
    // Classes would be really different in uwin (if they would be there at all)
    // don't expect to get them working for now

    let name = &class.name;
    let name = gen_ident(&format!("{}_Repr", name));

    let interfaces = class.interfaces.iter().map(|iface| {
        let (namespace, name) = iface.rsplit_once('.').unwrap();
        let type_ = type_reader
            .get_type((namespace, name))
            .expect("Find the interface referenced by com_interfaces");
        let type_ = match type_ {
            Type::TypeDef(def) => def,
            _ => unimplemented!(),
        };

        let namespace = gen.namespace(namespace);
        let name = gen_trait_ident(type_, gen);

        quote!(#namespace #name)
    });

    let dyn_name = quote!(dyn #(#interfaces)+*);

    let vtable_names = class
        .interfaces
        .iter()
        .map(|name| gen_ident(&format!("vtable_{}", name.rsplit_once('.').unwrap().1)))
        .collect::<Vec<_>>();

    let vtable_count = vtable_names.len();

    let ptr_size = Type::USize.layout().size as usize;

    let vtable_offsets = (0..vtable_count)
        .into_iter()
        .map(|i| {
            let offset = i * ptr_size;
            quote! { #offset }
        })
        .collect::<Vec<_>>();

    let ptr_size = quote! { #ptr_size };

    quote! {
        pub struct #name {
            #(pub #vtable_names: PtrRepr,)*
            pub implementation: *const #dyn_name,
        }

        impl FromIntoMemory for #name {
            fn from_bytes(from: &[u8]) -> Self {
                const IMPL_SIZE: usize = std::mem::size_of::<*const #dyn_name>();

                assert_eq!(from.len(), Self::size());
                #(
                    let #vtable_names = <PtrRepr as FromIntoMemory>::from_bytes(&from[#vtable_offsets..#vtable_offsets + #ptr_size]);
                )*

                let implementation = <[u8; IMPL_SIZE]>::from_bytes(&from[(#vtable_count * #ptr_size)..]);
                let implementation = unsafe { std::mem::transmute(implementation) };

                Self {
                    vtable_IDirectDraw,
                    implementation,
                }
            }

            fn into_bytes(self, into: &mut [u8]) {
                const IMPL_SIZE: usize = std::mem::size_of::<*const #dyn_name>();

                assert_eq!(into.len(), Self::size());
                #(
                    FromIntoMemory::into_bytes(self.#vtable_names, &mut into[#vtable_offsets..#vtable_offsets + #ptr_size]);
                )*

                let implementation: [u8; IMPL_SIZE] = unsafe { std::mem::transmute(self.implementation) };
                into[(#vtable_count * #ptr_size)..].copy_from_slice(&implementation);
            }

            fn size() -> usize {
                #vtable_count * #ptr_size + std::mem::size_of::<*const #dyn_name>()
            }
        }
    }
}
