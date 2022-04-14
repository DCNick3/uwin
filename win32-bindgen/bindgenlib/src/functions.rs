use super::*;

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
    let name = gen_ident(def.name());

    let signature = def.signature(&[]);

    let params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.def);
        let tokens = gen_element_name(&p.ty, gen);
        quote! { #name: #tokens }
    });

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
        fn #name(&self, #(#params),*) #return_type {
            todo!()
        }
    };

    res
}

pub fn gen_magic_function(def: &MethodDef, gen: &Gen, namespace: &TokenStream) -> TokenStream {
    let cfg = def.cfg();

    if !gen.is_cfg_enabled(&cfg) {
        return quote! {};
    }

    let name = gen_ident(def.name());

    let signature = def.signature(&[]);

    let arg_names = signature
        .params
        .iter()
        .map(|param| {
            let name = gen_param_name(&param.def);
            assert!(!name.is_empty());
            name
        })
        .collect::<Vec<_>>();

    let body = quote! {
        let api = #namespace get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);

        #(let #arg_names = call.get_arg();)*

        let res = api.#name(#(#arg_names),*);

        call.finish(res);
    };

    let magic_name = gen_ident(&format!("magic_{}", def.name()));

    quote! {
        #[no_mangle]
        extern "C" fn #magic_name(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                #body
            }));

            if result.is_err() {
                eprintln!("Caught a panic in native code. Whoops, aborting..");

                std::process::abort();
            }
        }
    }
}

pub fn gen_functions(tree: &TypeTree, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for entry in tree.types.values() {
        for def in entry {
            if let Type::MethodDef(def) = def {
                if !gen.excluded_items.contains(def.name()) {
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

            pub fn get_api(ctx: &crate::core::Win32Context) -> &dyn Api {
                ctx.get::<dyn Api>()
            }
        }
    } else {
        quote! {}
    }
}

pub fn gen_rusty_x86_magic_functions(tree: &TypeTree, gen: &Gen) -> TokenStream {
    let mut magic_functions = TokenStream::new();

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
                if !gen.excluded_items.contains(def.name()) {
                    magic_functions.combine(&gen_magic_function(def, gen, &namespace));
                }
            }
        }
    }

    magic_functions
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
