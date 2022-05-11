use super::*;
use itertools::Itertools;

// pub fn gen_sys_functions(tree: &TypeTree, gen: &Gen) -> TokenStream {
//     if gen.sys {
//         let mut tokens = quote! {};
//
//         for entry in tree.types.values() {
//             tokens.combine(&gen_function_if(entry, gen));
//         }
//
//         if !tokens.is_empty() {
//             quote! {
//                 #[link(name = "windows")]
//                 extern "system" {
//                     #tokens
//                 }
//             }
//         } else {
//             quote! {}
//         }
//     } else {
//         quote! {}
//     }
// }

pub fn gen_function_declaration(def: &MethodDef, gen: &Gen) -> TokenStream {
    let name = def.name();
    let ident = gen_ident(name);

    let signature = def.signature(&[]);
    let unwindable = gen.function_unwindable(def.name());
    let callbacking = gen.function_callbacking(def.name());

    let params = if unwindable {
        vec![quote!(unwind_token: &mut UnwindToken)]
    } else {
        vec![]
    }
    .into_iter()
    .chain(
        if callbacking {
            vec![quote!(callback_token: &mut dyn StdcallCallbackTokenTrait)]
        } else {
            vec![]
        }
        .into_iter(),
    )
    .chain(signature.params.iter().map(|p| {
        let name = gen_param_name(&p.def);
        let tokens = gen_element_name(&p.ty, gen);
        quote! { #name: #tokens }
    }));

    let return_type = gen_return_sig(&signature, gen);

    println!("  {:50}: {:?}", def.name(), signature.kind());

    // TODO: this should actually be really different
    // if gen.sys {
    //     let function = gen_sys_function(def, gen);
    //
    //     quote! {
    //         #[link(name = "windows")]
    //         extern "system" {
    //             #function
    //         }
    //     }
    // } else {
    //     gen_win_function(def, gen)
    // }

    let cfg = def.cfg();
    let features = gen.cfg(&cfg);

    let res = quote! {
        #features
        fn #ident(&self, #(#params),*) #return_type {
            todo!(#name)
        }
    };

    res
}

pub fn gen_thunk_function(def: &MethodDef, gen: &Gen, namespace: &TokenStream) -> TokenStream {
    let cfg = def.cfg();

    if !gen.is_cfg_enabled(&cfg) {
        return quote! {};
    }

    let name = def.name();
    let ident = gen_ident(name);
    let signature = def.signature(&[]);

    let unwindable = gen.function_unwindable(name);
    let callbacking = gen.function_callbacking(name);

    let base_args = signature
        .params
        .iter()
        .map(|param| {
            let name = gen_param_name(&param.def);
            assert!(!name.is_empty());
            name
        })
        .collect::<Vec<_>>();

    let mut precall = TokenStream::new();

    let mut args = Vec::new();
    if unwindable {
        precall.combine(&quote! {
            let mut unwind_token = call.unwind_token();
        });

        args.push(quote!(&mut unwind_token));
    }

    if callbacking {
        precall.combine(&quote! {
            let mut callback_token = call.callback_token();
        });

        args.push(quote!(&mut callback_token));
    }

    args.extend(base_args.clone().into_iter());

    let call = quote! {
        {
            let api = #namespace get_api(&call.context().win32);
            #precall
            api.#ident(#(#args),*)
        }
    };

    let arguments_fmt = base_args
        .iter()
        .map(|arg| format!("{} = {{:?}}", arg))
        .join(", ");
    let arguments_fmt = format!("    args = {{{{{}}}}}", arguments_fmt);

    let body = quote! {
        static SPAN_CALLSITE: crate::MyCallsite = crate::MyCallsite::new_span(tracing::callsite::Identifier(&SPAN_CALLSITE), #name);

        crate::thunk_helper(context, memory, &SPAN_CALLSITE, |mut call, trace_event_enabled, callsite| {
            #(let #base_args = call.get_arg();)*

            if trace_event_enabled {
                let fields = callsite.metadata().fields();
                tracing::event::Event::dispatch(callsite.metadata(),
                    &fields.value_set(&[(
                        &unsafe { fields.iter().next().unwrap_unchecked() },
                        Some(&format_args!(#arguments_fmt, #(#base_args),*)
                            as &dyn tracing::Value),
                    )]));
            }

            let res = #call;

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
        })
    };

    let thunk_name = gen_ident(&format!("thunk_{}", def.name()));

    quote! {
        #[no_mangle]
        extern "C" fn #thunk_name(context: &mut ExtendedContext, memory: FlatMemoryCtx) -> PtrRepr {
            #body
        }
    }
}

pub fn gen_functions(tree: &TypeTree, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for entry in tree.types.values() {
        for def in entry {
            if let Type::MethodDef(def) = def {
                if !gen.excluded_items.contains(def.name()) && gen.dll_enabled(def.dll_import()) {
                    tokens.combine(&gen_function_declaration(def, gen));
                }
            }
        }
    }

    // do not emit an empty Api trait
    if !tokens.is_empty() {
        quote! {
            pub trait Api {
                #tokens
            }

            pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
                ctx.get::<dyn Api>()
            }
        }
    } else {
        quote! {}
    }
}

pub fn gen_rusty_x86_thunk_functions(tree: &TypeTree, gen: &Gen) -> TokenStream {
    let mut thunk_functions = TokenStream::new();

    // can't use .namespace method of gen because the code lives in another crate
    // so do it manually, lol

    let namespace = {
        let mut tokens = quote!(win32::);
        let namespace = tree.namespace.split('.').skip(1); // strip the "Windows" part

        for namespace in namespace {
            tokens.push_str(namespace);
            tokens.push_str("::");
        }

        tokens
    };

    for entry in tree.types.values() {
        for def in entry {
            if let Type::MethodDef(def) = def {
                if !gen.excluded_items.contains(def.name()) && gen.dll_enabled(def.dll_import()) {
                    thunk_functions.combine(&gen_thunk_function(def, gen, &namespace));
                }
            }
        }
    }

    thunk_functions
}

// fn gen_function_if(entry: &[Type], gen: &Gen) -> TokenStream {
//     let mut tokens = TokenStream::new();
//
//     for def in entry {
//         if let Type::MethodDef(def) = def {
//             tokens.combine(&gen_sys_function(def, gen));
//         }
//     }
//
//     tokens
// }

// #[allow(unused)]
// fn gen_win_function(def: &MethodDef, gen: &Gen) -> TokenStream {
//     let name = gen_ident(def.name());
//     let signature = def.signature(&[]);
//     let constraints = gen_param_constraints(&signature.params, gen);
//
//     let abi_params = signature.params.iter().map(|p| {
//         let name = gen_param_name(&p.def);
//         let tokens = gen_abi_element_name(&p.ty, gen);
//         quote! { #name: #tokens }
//     });
//
//     let abi_return_type = gen_return_sig(&signature, gen);
//
//     let link_attr = match def.static_lib() {
//         Some(link) => quote! { #[link(name = #link, kind = "static")] },
//         None => {
//             if gen.namespace.starts_with("Windows.") {
//                 quote! { #[link(name = "windows")] }
//             } else {
//                 let link = def
//                     .impl_map()
//                     .expect("Function")
//                     .scope()
//                     .name()
//                     .to_lowercase();
//
//                 quote! { #[link(name = #link)] }
//             }
//         }
//     };
//
//     let cfg = def.cfg();
//     let doc = gen.doc(&cfg);
//     let features = gen.cfg(&cfg);
//
//     match signature.kind() {
//         SignatureKind::Query => {
//             let leading_params = &signature.params[..signature.params.len() - 2];
//             let args = leading_params.iter().map(gen_win32_abi_arg);
//             let params = gen_win32_params(leading_params, gen);
//
//             quote! {
//                 #doc
//                 #features
//                 #[inline]
//                 pub unsafe fn #name<#constraints T: ::windows::core::Interface>(#params) -> ::windows::core::Result<T> {
//                     todo!();
//                 }
//             }
//         }
//         SignatureKind::QueryOptional => {
//             let leading_params = &signature.params[..signature.params.len() - 2];
//             let args = leading_params.iter().map(gen_win32_abi_arg);
//             let params = gen_win32_params(leading_params, gen);
//
//             quote! {
//                 #doc
//                 #features
//                 #[inline]
//                 pub unsafe fn #name<#constraints T: ::windows::core::Interface>(#params result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
//                     todo!();
//                 }
//             }
//         }
//         // SignatureKind::ResultValue => {
//         //     let leading_params = &signature.params[..signature.params.len() - 1];
//         //     let args = leading_params.iter().map(gen_win32_abi_arg);
//         //     let params = gen_win32_params(leading_params, gen);
//         //     let return_type = signature.params[signature.params.len() - 1].ty.deref();
//         //     let return_type_tokens = gen_element_name(&return_type, gen);
//         //     let abi_return_type_tokens = gen_abi_element_name(&return_type, gen);
//         //
//         //     quote! {
//         //         #doc
//         //         #features
//         //         #[inline]
//         //         pub unsafe fn #name<#constraints>(#params) -> ::windows::core::Result<#return_type_tokens> {
//         //             todo!();
//         //         }
//         //     }
//         // }
//         SignatureKind::ResultVoid => {
//             let params = gen_win32_params(&signature.params, gen);
//             let args = signature.params.iter().map(gen_win32_abi_arg);
//
//             quote! {
//                 #doc
//                 #features
//                 #[inline]
//                 pub unsafe fn #name<#constraints>(#params) -> ::windows::core::Result<()> {
//                     todo!();
//                 }
//             }
//         }
//         // SignatureKind::ReturnStruct | SignatureKind::PreserveSig => {
//         //     let params = gen_win32_params(&signature.params, gen);
//         //     let args = signature.params.iter().map(gen_win32_abi_arg);
//         //
//         //     quote! {
//         //         #doc
//         //         #features
//         //         #[inline]
//         //         pub unsafe fn #name<#constraints>(#params) #abi_return_type {
//         //             todo!();
//         //         }
//         //     }
//         // }
//         SignatureKind::ReturnVoid => {
//             let params = gen_win32_params(&signature.params, gen);
//             let args = signature.params.iter().map(gen_win32_abi_arg);
//             let does_not_return = does_not_return(def);
//
//             quote! {
//                 #doc
//                 #features
//                 #[inline]
//                 pub unsafe fn #name<#constraints>(#params) #does_not_return {
//                     todo!()
//                 }
//             }
//         }
//         _ => todo!(),
//     }
// }

#[allow(unused)]
fn does_not_return(def: &MethodDef) -> TokenStream {
    if def.does_not_return() {
        quote! { -> ! }
    } else {
        quote! {}
    }
}
