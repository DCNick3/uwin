use super::*;
use itertools::Itertools;

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
                gen: &Gen,
                class: &ComClass,
                iface_name: &str,
                tokens: &mut TokenStream,
                interface: BaseInterface,
            ) {
                match interface {
                    BaseInterface::IUnknown => {
                        let query_interface = gen_ident(&format!(
                            "thunk_com_{}_as_{}_QueryInterface",
                            class.name, iface_name
                        ));
                        let add_ref = gen_ident(&format!(
                            "thunk_com_{}_as_{}_AddRef",
                            class.name, iface_name
                        ));
                        let release = gen_ident(&format!(
                            "thunk_com_{}_as_{}_Release",
                            class.name, iface_name
                        ));

                        // TODO: generate __at least something__

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
                        emit_interface(gen, class, iface_name, tokens, def.base_interface());

                        // can't use .namespace method of gen because the code lives in another crate
                        // so do it manually, lol
                        let namespace = gen.thunk_namespace();

                        for def in def.methods() {
                            let method = def.name();
                            let ident = gen_ident(method);

                            let thunk_name = gen_ident(&format!(
                                "thunk_com_{}_as_{}_{}",
                                class.name, iface_name, method
                            ));

                            let name = format!("{}::{}", class.name, method);

                            let signature = def.signature(&[]);

                            let base_args = signature
                                .params
                                .iter()
                                .map(|param| {
                                    let name = gen_param_name(&param.def);
                                    assert!(!name.is_empty());
                                    name
                                })
                                .collect::<Vec<_>>();

                            // these are needed for handing unwinding and callbacking functions (in the future)
                            let mut args = Vec::new();
                            args.extend(base_args.clone().into_iter());

                            let arguments_fmt = [TokenStream::from("self")]
                                .iter()
                                .chain(base_args.iter())
                                .map(|arg| format!("{} = {{:?}}", arg))
                                .join(", ");
                            let arguments_fmt = format!("    args = {{{{{}}}}}", arguments_fmt);

                            let repr_type = gen_ident(&format!("{}_Repr", class.name));
                            let repr_type = quote! { #namespace #repr_type };

                            let body = quote! {
                                static SPAN_CALLSITE: crate::MyCallsite = crate::MyCallsite::new_span(tracing::callsite::Identifier(&SPAN_CALLSITE), #name);

                                crate::thunk_helper(
                                    context,
                                    memory,
                                    &SPAN_CALLSITE,
                                    |mut call, trace_event_enabled, callsite| {
                                        // TODO: adjust pointer if multiple vtables are used
                                        let self_: core_mem::ptr::ConstPtr<#repr_type> = call.get_arg();

                                        #(let #base_args = call.get_arg();)*

                                        if trace_event_enabled {
                                            let fields = callsite.metadata().fields();
                                            tracing::event::Event::dispatch(callsite.metadata(),
                                                &fields.value_set(&[(
                                                    &unsafe { fields.iter().next().unwrap_unchecked() },
                                                    Some(&format_args!(#arguments_fmt, self_, #(#base_args),*)
                                                        as &dyn tracing::Value),
                                                )]));
                                        }

                                        let self_ = self_.read_with(*call.memory_context());
                                        // SAFETY: The implementation is constructed from Arc, so it should safe though
                                        // target program can mess with this part of memory though
                                        let self_ = unsafe { &*self_.implementation };
                                        let res = self_.#ident(#(#args),*);

                                        if trace_event_enabled {
                                            let fields = callsite.metadata().fields();
                                            tracing::event::Event::dispatch(callsite.metadata(),
                                                &fields.value_set(&[(
                                                    &unsafe { fields.iter().next().unwrap_unchecked() },
                                                    Some(&format_args!("  result = {:?}", res)
                                                        as &dyn tracing::Value),
                                                )]));
                                        }

                                        call.finish(res)
                                    }
                                )
                            };

                            tokens.combine(&quote! {
                                #[no_mangle]
                                extern "C" fn #thunk_name(context: &mut ExtendedContext, memory: FlatMemoryCtx) -> PtrRepr {
                                    #body
                                }
                            });
                        }
                    }
                }
            }

            emit_interface(gen, class, name, &mut tokens, BaseInterface::TypeDef(type_));
        }
    }

    tokens
}

pub fn gen_com_stub_params(gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    let type_reader = TypeReader::get();

    for class in gen.com_classes {
        let class_name = &class.name;
        let namespace = gen.namespace;

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
                namespace: #namespace.to_string(),
                name: #class_name.to_string(),
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
