use super::*;

pub fn gen_sys_functions(tree: &TypeTree, gen: &Gen) -> TokenStream {
    if gen.sys {
        let mut tokens = quote! {};

        for entry in tree.types.values() {
            tokens.combine(&gen_function_if(entry, gen));
        }

        if !tokens.is_empty() {
            quote! {
                #[link(name = "windows")]
                extern "system" {
                    #tokens
                }
            }
        } else {
            quote! {}
        }
    } else {
        quote! {}
    }
}

pub fn gen_function(def: &MethodDef, gen: &Gen) -> TokenStream {
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

    quote! {}
}

fn gen_function_if(entry: &[Type], gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for def in entry {
        if let Type::MethodDef(def) = def {
            tokens.combine(&gen_sys_function(def, gen));
        }
    }

    tokens
}

fn gen_sys_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());
    let signature = def.signature(&[]);
    let cfg = def.cfg();
    let doc = gen.doc(&cfg);
    let features = gen.cfg(&cfg);
    let mut return_type = gen_return_sig(&signature, gen);

    if return_type.is_empty() {
        return_type = does_not_return(def);
    }

    let params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.def);
        let tokens = gen_default_type(&p.ty, gen);
        quote! { #name: #tokens }
    });

    quote! {
        #doc
        #features
        pub fn #name(#(#params),*) #return_type;
    }
}

#[allow(unused)]
fn gen_win_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());
    let signature = def.signature(&[]);
    let constraints = gen_param_constraints(&signature.params, gen);

    let abi_params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.def);
        let tokens = gen_abi_element_name(&p.ty, gen);
        quote! { #name: #tokens }
    });

    let abi_return_type = gen_return_sig(&signature, gen);

    let link_attr = match def.static_lib() {
        Some(link) => quote! { #[link(name = #link, kind = "static")] },
        None => {
            if gen.namespace.starts_with("Windows.") {
                quote! { #[link(name = "windows")] }
            } else {
                let link = def
                    .impl_map()
                    .expect("Function")
                    .scope()
                    .name()
                    .to_lowercase();

                quote! { #[link(name = #link)] }
            }
        }
    };

    let cfg = def.cfg();
    let doc = gen.doc(&cfg);
    let features = gen.cfg(&cfg);

    match signature.kind() {
        SignatureKind::Query => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(#params) -> ::windows::core::Result<T> {
                    todo!();
                }
            }
        }
        SignatureKind::QueryOptional => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(#params result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
                    todo!();
                }
            }
        }
        SignatureKind::ResultValue => {
            let leading_params = &signature.params[..signature.params.len() - 1];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);
            let return_type = signature.params[signature.params.len() - 1].ty.deref();
            let return_type_tokens = gen_element_name(&return_type, gen);
            let abi_return_type_tokens = gen_abi_element_name(&return_type, gen);

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#constraints>(#params) -> ::windows::core::Result<#return_type_tokens> {
                    todo!();
                }
            }
        }
        SignatureKind::ResultVoid => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#constraints>(#params) -> ::windows::core::Result<()> {
                    todo!();
                }
            }
        }
        SignatureKind::ReturnStruct | SignatureKind::PreserveSig => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#constraints>(#params) #abi_return_type {
                    todo!();
                }
            }
        }
        SignatureKind::ReturnVoid => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);
            let does_not_return = does_not_return(def);

            quote! {
                #doc
                #features
                #[inline]
                pub unsafe fn #name<#constraints>(#params) #does_not_return {
                    todo!()
                }
            }
        }
    }
}

fn does_not_return(def: &MethodDef) -> TokenStream {
    if def.does_not_return() {
        quote! { -> ! }
    } else {
        quote! {}
    }
}
