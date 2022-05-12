use super::*;

// pub fn gen_vtbl_signature(def: &TypeDef, method: &MethodDef, gen: &Gen) -> TokenStream {
//     let is_winrt = def.is_winrt();
//     let signature = method.signature(&def.generics);
//     let hresult = gen_element_name(&Type::HRESULT, gen);
//
//     let (trailing_return_type, return_type, udt_return_type) = if is_winrt {
//         if let Some(return_type) = &signature.return_type {
//             if let Type::WinrtArray(kind) = return_type {
//                 let tokens = gen_abi_element_name(kind, gen);
//                 (
//                     quote! { result_size__: *mut u32, result__: *mut *mut #tokens },
//                     quote! { -> #hresult },
//                     quote! {},
//                 )
//             } else {
//                 let tokens = gen_abi_element_name(return_type, gen);
//                 (
//                     quote! { result__: *mut #tokens },
//                     quote! { -> #hresult },
//                     quote! {},
//                 )
//             }
//         } else {
//             (quote! {}, quote! { -> #hresult }, quote! {})
//         }
//     } else if let Some(return_type) = &signature.return_type {
//         if return_type.is_udt() {
//             let tokens = gen_abi_element_name(return_type, gen);
//             (quote! {}, quote! {}, quote! { result__: *mut #tokens, })
//         } else {
//             let tokens = gen_default_type(return_type, gen);
//             (quote! {}, quote! { -> #tokens }, quote! {})
//         }
//     } else {
//         (quote! {}, quote! {}, quote! {})
//     };
//
//     let params = signature.params.iter().map(|p| {
//         let name = gen_param_name(&p.def);
//         if is_winrt {
//             let abi = gen_abi_element_name(&p.ty, gen);
//             let abi_size_name = gen_ident(&format!("{}_array_size", p.def.name()));
//
//             if p.def.flags().input() {
//                 if p.ty.is_winrt_array() {
//                     quote! { #abi_size_name: u32, #name: *const #abi, }
//                 } else if p.ty.is_winrt_const_ref() {
//                     quote! { #name: &#abi, }
//                 } else {
//                     quote! { #name: #abi, }
//                 }
//             } else if p.ty.is_winrt_array() {
//                 quote! { #abi_size_name: u32, #name: *mut #abi, }
//             } else if p.ty.is_winrt_array_ref() {
//                 quote! { #abi_size_name: *mut u32, #name: *mut *mut #abi, }
//             } else {
//                 quote! { #name: *mut #abi, }
//             }
//         } else {
//             let abi = gen_abi_element_name(&p.ty, gen);
//             quote! { #name: #abi, }
//         }
//     });
//
//     quote! { (this: *mut ::core::ffi::c_void, #udt_return_type #(#params)* #trailing_return_type) #return_type }
// }

fn gen_string_literal(value: &str) -> TokenStream {
    let mut tokens = "\"".to_string();

    for u in value.chars() {
        tokens.push_str(&format!("{}", u.escape_default()));
    }

    tokens.push('\"');
    tokens.into()
}

pub fn gen_constant_type_value(value: &ConstantValue) -> TokenStream {
    match value {
        ConstantValue::Bool(value) => quote! { bool = #value },
        ConstantValue::U8(value) => quote! { u8 = #value },
        ConstantValue::I8(value) => quote! { i8 = #value },
        ConstantValue::U16(value) => quote! { u16 = #value },
        ConstantValue::I16(value) => quote! { i16 = #value },
        ConstantValue::U32(value) => quote! { u32 = #value },
        ConstantValue::I32(value) => quote! { i32 = #value },
        ConstantValue::U64(value) => quote! { u64 = #value },
        ConstantValue::I64(value) => quote! { i64 = #value },
        ConstantValue::F32(value) => quote! { f32 = #value },
        ConstantValue::F64(value) => quote! { f64 = #value },
        ConstantValue::String(value) => {
            let value = gen_string_literal(value);
            quote! { &'static str = #value }
        }
        _ => unimplemented!(),
    }
}

pub fn gen_guid(value: &GUID, gen: &Gen) -> TokenStream {
    let guid = gen_element_name(&Type::GUID, gen);

    if gen.sys {
        let a = Literal::u32_unsuffixed(value.0);
        let b = Literal::u16_unsuffixed(value.1);
        let c = Literal::u16_unsuffixed(value.2);
        let d = Literal::u8_unsuffixed(value.3);
        let e = Literal::u8_unsuffixed(value.4);
        let f = Literal::u8_unsuffixed(value.5);
        let g = Literal::u8_unsuffixed(value.6);
        let h = Literal::u8_unsuffixed(value.7);
        let i = Literal::u8_unsuffixed(value.8);
        let j = Literal::u8_unsuffixed(value.9);
        let k = Literal::u8_unsuffixed(value.10);

        // TODO: once code complete measure how much longer it takes if-any to use from_u128 to produce a more compact package

        quote! {
            #guid { data1:#a, data2:#b, data3:#c, data4:[#d, #e, #f, #g, #h, #i, #j, #k] }
        }
    } else {
        format!("{}::from_u128(0x{:08x?}_{:04x?}_{:04x?}_{:02x?}{:02x?}_{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?})", guid.into_string(), value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8, value.9, value.10).into()
    }
}

pub fn gen_constant_value(value: &ConstantValue) -> TokenStream {
    match value {
        ConstantValue::Bool(value) => quote! { #value },
        ConstantValue::U8(value) => quote! { #value },
        ConstantValue::I8(value) => quote! { #value },
        ConstantValue::U16(value) => quote! { #value },
        ConstantValue::I16(value) => quote! { #value },
        ConstantValue::U32(value) => quote! { #value },
        ConstantValue::I32(value) => quote! { #value },
        ConstantValue::U64(value) => quote! { #value },
        ConstantValue::I64(value) => quote! { #value },
        ConstantValue::F32(value) => quote! { #value },
        ConstantValue::F64(value) => quote! { #value },
        ConstantValue::String(value) => gen_string_literal(value),
        _ => unimplemented!(),
    }
}

#[allow(unused)]
pub fn gen_runtime_name(def: &TypeDef, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let cfg = gen.cfg(cfg);

    if def.is_winrt() {
        let constraints = gen_type_constraints(def, gen);
        let runtime_name = format!("{}", def.type_name());

        quote! {
            #cfg
            impl<#(#constraints)*> ::windows::core::RuntimeName for #name {
                const NAME: &'static str = #runtime_name;
            }
        }
    } else if def.vtable_types().iter().any(|e| e == &Type::IInspectable) {
        quote! {
            #cfg
            impl ::windows::core::RuntimeName for #name {
                const NAME: &'static str = "";
            }
        }
    } else {
        quote! {}
    }
}

#[allow(unused)]
pub fn gen_win32_upcall(sig: &Signature, inner: TokenStream) -> TokenStream {
    match sig.kind() {
        // SignatureKind::ResultValue => {
        //     todo!()
        //
        //     // let invoke_args = sig.params[..sig.params.len() - 1]
        //     //     .iter()
        //     //     .map(gen_win32_invoke_arg);
        //     //
        //     // let result = gen_param_name(&sig.params[sig.params.len() - 1].def);
        //     //
        //     // quote! {
        //     //     match #inner(#(#invoke_args,)*) {
        //     //         ::::core::result::Result::Ok(ok__) => {
        //     //             *#result = ::core::mem::transmute(ok__);
        //     //             ::#crate_name::core::HRESULT(0)
        //     //         }
        //     //         ::core::result::Result::Err(err) => err.into()
        //     //     }
        //     // }
        // }
        SignatureKind::Query | SignatureKind::QueryOptional | SignatureKind::ResultVoid => {
            let invoke_args = sig.params.iter().map(gen_win32_invoke_arg);

            quote! {
                #inner(#(#invoke_args,)*).into()
            }
        }
        // SignatureKind::ReturnStruct => {
        //     let invoke_args = sig.params.iter().map(gen_win32_invoke_arg);
        //
        //     quote! {
        //         *result__ = #inner(#(#invoke_args,)*)
        //     }
        // }
        _ => {
            let invoke_args = sig.params.iter().map(gen_win32_invoke_arg);

            quote! {
                #inner(#(#invoke_args,)*)
            }
        }
    }
}

#[allow(unused)]
pub fn gen_winrt_upcall(sig: &Signature, inner: TokenStream) -> TokenStream {
    let invoke_args = sig.params.iter().map(gen_winrt_invoke_arg);

    match &sig.return_type {
        Some(return_type) if return_type.is_winrt_array() => {
            unimplemented!()
            // quote! {
            //     match #inner(#(#invoke_args,)*) {
            //         ::core::result::Result::Ok(ok__) => {
            //             let (ok_data__, ok_data_len__) = ok__.into_abi();
            //             *result__ = ok_data__;
            //             *result_size__ = ok_data_len__;
            //             ::#crate_name::core::HRESULT(0)
            //         }
            //         ::core::result::Result::Err(err) => err.into()
            //     }
            // }
        }
        Some(_) => {
            todo!()
            // quote! {
            //     match #inner(#(#invoke_args,)*) {
            //         ::core::result::Result::Ok(ok__) => {
            //             *result__ = ::core::mem::transmute_copy(&ok__);
            //             ::core::mem::forget(ok__);
            //             ::#crate_name::core::HRESULT(0)
            //         }
            //         ::core::result::Result::Err(err) => err.into()
            //     }
            // }
        }
        None => quote! {
            #inner(#(#invoke_args,)*).into()
        },
    }
}

#[allow(unused)]
fn gen_win32_invoke_arg(param: &MethodParam) -> TokenStream {
    let name = gen_param_name(&param.def);

    if (!param.ty.is_pointer() && param.ty.is_nullable())
        || (param.def.flags().input() && !param.ty.is_primitive())
    {
        quote! { ::core::mem::transmute(&#name) }
    } else {
        quote! { ::core::mem::transmute_copy(&#name) }
    }
}

#[allow(unused)]
fn gen_winrt_invoke_arg(param: &MethodParam) -> TokenStream {
    let name = gen_param_name(&param.def);
    let abi_size_name: TokenStream = format!("{}_array_size", param.def.name()).into();

    if param.def.flags().input() {
        if param.ty.is_winrt_array() {
            quote! { ::core::slice::from_raw_parts(::core::mem::transmute_copy(&#name), #abi_size_name as _) }
        } else if param.ty.is_primitive() {
            quote! { #name }
        } else if param.ty.is_winrt_const_ref() {
            quote! { ::core::mem::transmute_copy(&#name) }
        } else {
            quote! { ::core::mem::transmute(&#name) }
        }
    } else if param.ty.is_winrt_array() {
        quote! { ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&#name), #abi_size_name as _) }
    } else if param.ty.is_winrt_array_ref() {
        quote! { ::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&#name), #abi_size_name).as_array() }
    } else {
        quote! { ::core::mem::transmute_copy(&#name) }
    }
}

// pub fn gen_impl_signature(def: &TypeDef, method: &MethodDef, gen: &Gen) -> TokenStream {
//     let signature = method.signature(&def.generics);
//
//     if def.is_winrt() {
//         let is_delegate = def.kind() == TypeKind::Delegate;
//         let params = signature
//             .params
//             .iter()
//             .map(|p| gen_winrt_produce_type(p, !is_delegate, gen));
//
//         let return_type = if let Some(return_type) = &signature.return_type {
//             let tokens = gen_element_name(return_type, gen);
//
//             if return_type.is_winrt_array() {
//                 quote! { ::windows::core::Array<#tokens> }
//             } else {
//                 tokens
//             }
//         } else {
//             quote! { () }
//         };
//
//         let this = if is_delegate {
//             quote! {}
//         } else {
//             quote! { &self, }
//         };
//
//         quote! { (#this #(#params),*) -> ::windows::core::Result<#return_type> }
//     } else {
//         let signature_kind = signature.kind();
//         let mut params = quote! {};
//
//         if signature_kind == SignatureKind::ResultValue {
//             for param in &signature.params[..signature.params.len() - 1] {
//                 params.combine(&gen_win32_produce_type(param, gen));
//             }
//         } else {
//             for param in &signature.params {
//                 params.combine(&gen_win32_produce_type(param, gen));
//             }
//         }
//
//         let return_type = match signature_kind {
//             SignatureKind::ReturnVoid => quote! {},
//             SignatureKind::Query | SignatureKind::QueryOptional | SignatureKind::ResultVoid => {
//                 quote! { -> ::windows::core::Result<()> }
//             }
//             SignatureKind::ResultValue => {
//                 let return_type = signature.params[signature.params.len() - 1].ty.deref();
//                 let return_type = gen_element_name(&return_type, gen);
//
//                 quote! { -> ::windows::core::Result<#return_type> }
//             }
//             _ => gen_return_sig(&signature, gen),
//         };
//
//         quote! { (&self, #params) #return_type }
//     }
// }

#[allow(unused)]
fn gen_win32_produce_type(param: &MethodParam, gen: &Gen) -> TokenStream {
    let name = gen_param_name(&param.def);
    let kind = gen_default_type(&param.ty, gen);

    if param.def.flags().input() && !param.ty.is_primitive() {
        quote! { #name: &#kind, }
    } else {
        quote! { #name: #kind, }
    }
}

pub fn gen_default_type(def: &Type, gen: &Gen) -> TokenStream {
    if let Type::WinrtArray(_) = def {
        todo!()
        // gen_default_type(def, gen)
    } else {
        let kind = gen_element_name(def, gen);

        // if def.is_generic() {
        //     quote! { <#kind as ::windows::core::RuntimeType>::DefaultType }
        // } else if def.is_nullable() && !gen.sys {
        //     quote! { ::core::option::Option<#kind> }
        // } else {
        kind
        // }
    }
}

#[allow(unused)]
fn gen_winrt_produce_type(
    param: &MethodParam,
    include_param_names: bool,
    gen: &Gen,
) -> TokenStream {
    let default_type = gen_default_type(&param.ty, gen);

    let sig = if param.def.flags().input() {
        if param.ty.is_winrt_array() {
            quote! { &[#default_type] }
        } else if param.ty.is_primitive() {
            quote! { #default_type }
        } else {
            quote! { &#default_type }
        }
    } else if param.ty.is_winrt_array() {
        quote! { &mut [#default_type] }
    } else if param.ty.is_winrt_array_ref() {
        let kind = gen_element_name(&param.ty, gen);
        quote! { &mut ::windows::core::Array<#kind> }
    } else {
        quote! { &mut #default_type }
    };

    if include_param_names {
        let name = gen_param_name(&param.def);
        quote! { #name: #sig }
    } else {
        sig
    }
}
