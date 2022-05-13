use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    gen_win_interface(def, gen)
}

// fn gen_sys_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
//     let name = gen_type_ident(def, gen);
//
//     if def.is_exclusive() {
//         quote! {}
//     } else if def.kind() == TypeKind::Interface || def.default_interface().is_some() {
//         // TODO: should be *const?
//         quote! {
//             pub type #name = *mut ::core::ffi::c_void;
//         }
//     } else {
//         quote! {}
//     }
// }
//
fn gen_win_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let is_exclusive = def.is_exclusive();
    let phantoms = gen_phantoms(def, gen);
    let constraints = gen_type_constraints(def, gen);
    let cfg = def.cfg();
    let doc = gen.doc(&cfg);
    let features = gen.cfg(&cfg);

    let mut tokens = if is_exclusive {
        quote! { #[doc(hidden)] }
    } else {
        quote! { #doc }
    };

    tokens.combine(&quote! {
        #features
        pub struct #name(crate::core::IUnknown, #(#phantoms)*) where #(#constraints)*;
    });

    if !is_exclusive {
        // tokens.combine(&gen_methods(def, &cfg, gen));
        // tokens.combine(&gen_conversions(def, &cfg, gen));
        tokens.combine(&gen_std_traits(def, &cfg, gen));
        // tokens.combine(&gen_runtime_trait(def, &cfg, gen));
        // tokens.combine(&gen_async(def, &cfg, gen));
        // TODO: what does this mean?
        // Do we want to care?
        //tokens.combine(&gen_agile(def, gen));
    }

    tokens.combine(&gen_interface_trait(def, &cfg, gen));
    // tokens.combine(&gen_vtbl(def, &cfg, gen));
    tokens
}

pub fn gen_std_traits(def: &TypeDef, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let ident = gen_type_ident(def, gen);
    let name = ident.as_str();
    let constraints = gen_type_constraints(def, gen);
    let phantoms = gen_phantoms(def, gen);
    let cfg = gen.cfg(cfg);

    quote! {
        #cfg
        impl<#(#constraints)*> ::core::clone::Clone for #ident {
            fn clone(&self) -> Self {
                Self(self.0.clone(), #(#phantoms)*)
            }
        }
        #cfg
        impl<#(#constraints)*> ::core::marker::Copy for #ident {}
        #cfg
        impl<#(#constraints)*> ::core::cmp::PartialEq for #ident {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        #cfg
        impl<#(#constraints)*> ::core::cmp::Eq for #ident {}
        #cfg
        impl<#(#constraints)*> ::core::fmt::Debug for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple(#name).field(&self.0).finish()
            }
        }
        #cfg
        impl FromIntoMemory for #ident {
            fn from_bytes(from: &[u8]) -> Self {
                Self(<crate::core::IUnknown as FromIntoMemory>::from_bytes(from))
            }
            fn into_bytes(self, into: &mut [u8]) {
                FromIntoMemory::into_bytes(self.0, into)
            }
            fn size() -> usize {
                std::mem::size_of::<crate::core::IUnknown>()
            }
        }
    }
}

pub fn gen_interface_trait(def: &TypeDef, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let cfg = gen.cfg(cfg);

    let base = def.base_interface();
    let base = match base {
        BaseInterface::IUnknown => quote! {
            crate::core::IUnknown
        },
        BaseInterface::TypeDef(t) => {
            let ident = gen_type_ident(&t, gen);
            let namespace = gen.namespace(t.namespace());
            quote! {
                #namespace #ident
            }
        }
    };

    if let Some(_default) = def.default_interface() {
        unimplemented!("What is this?");

        // let name = gen_type_ident(def, gen);
        // let default_name = gen_type_ident(&default, gen);
        // let vtbl = gen_vtbl_ident(&default, gen);
        // let namespace = gen.namespace(default.namespace());

        // quote! {
        //     #cfg
        //     impl crate::core::ComInterface for #name {
        //         const IID: ::windows::core::GUID = <#namespace #default_name as crate::core::ComInterface>::IID;
        //     }
        // }
    } else {
        let name = gen_type_ident(def, gen);
        let constraints = gen_type_constraints(def, gen);
        // let vtbl = gen_vtbl_ident(def, gen);
        let guid = gen_type_guid(def, gen, &"Self".into());

        quote! {
            #cfg
            impl<#(#constraints)*> crate::core::ComInterface for #name {
                type Super = #base;
                const IID: crate::core::GUID = #guid;
            }
        }
    }
}

fn gen_type_guid(def: &TypeDef, gen: &Gen, _type_name: &TokenStream) -> TokenStream {
    if def.generics.is_empty() {
        match GUID::from_attributes(def.attributes()) {
            Some(guid) => gen_guid(&guid, gen),
            None => {
                quote! {
                    crate::core::GUID::zeroed()
                }
            }
        }
    } else {
        unimplemented!("Generics")
        // quote! {
        //     crate::core::GUID::from_signature(<#type_name as crate::core::RuntimeType>::SIGNATURE)
        // }
    }
}

// fn gen_methods(def: &TypeDef, cfg: &Cfg, gen: &Gen) -> TokenStream {
//     let name = gen_type_ident(def, gen);
//     let constraints = gen_type_constraints(def, gen);
//     let mut methods = quote! {};
//     let is_winrt = def.is_winrt();
//     let mut method_names = MethodNames::new();
//     let mut virtual_names = MethodNames::new();
//     let cfg = gen.cfg(cfg);
//     let vtable_types = def.vtable_types();
//     let mut bases = vtable_types.len();
//
//     for def in vtable_types {
//         match def {
//             Type::IUnknown | Type::IInspectable => {}
//             Type::TypeDef(def) => {
//                 methods.combine(&gen_methods_impl(
//                     &def,
//                     InterfaceKind::Default,
//                     &mut method_names,
//                     &mut virtual_names,
//                     bases,
//                     gen,
//                 ));
//             }
//             _ => unimplemented!(),
//         }
//
//         bases -= 1;
//     }
//
//     // Methods for vtable bases are added first (above) so that any overloads are renamed accordingly.
//     methods.combine(&gen_methods_impl(
//         def,
//         InterfaceKind::Default,
//         &mut method_names,
//         &mut virtual_names,
//         0,
//         gen,
//     ));
//
//     if is_winrt && !gen.min_inherit {
//         for def in def.required_interfaces() {
//             methods.combine(&gen_methods_impl(
//                 &def,
//                 InterfaceKind::NonDefault,
//                 &mut method_names,
//                 &mut virtual_names,
//                 0,
//                 gen,
//             ));
//         }
//     }
//
//     quote! {
//         #cfg
//         impl<#(#constraints)*> #name {
//             #methods
//         }
//     }
// }
//
// fn gen_methods_impl(
//     def: &TypeDef,
//     kind: InterfaceKind,
//     method_names: &mut MethodNames,
//     virtual_names: &mut MethodNames,
//     bases: usize,
//     gen: &Gen,
// ) -> TokenStream {
//     let mut methods = quote! {};
//     let is_winrt = def.is_winrt();
//
//     for method in def.methods() {
//         if is_winrt {
//             methods.combine(&gen_winrt_method(
//                 def,
//                 kind,
//                 &method,
//                 method_names,
//                 virtual_names,
//                 gen,
//             ));
//         } else {
//             methods.combine(&gen_com_method(
//                 def,
//                 &method,
//                 method_names,
//                 virtual_names,
//                 bases,
//                 gen,
//             ));
//         }
//     }
//
//     methods
// }
//
// fn gen_conversions(def: &TypeDef, cfg: &Cfg, gen: &Gen) -> TokenStream {
//     let name = gen_type_ident(def, gen);
//     let constraints = gen_type_constraints(def, gen);
//     let mut tokens = quote! {};
//
//     for def in def.vtable_types() {
//         let into = gen_element_name(&def, gen);
//         let cfg = gen.cfg(&cfg.union(&def.cfg()));
//         tokens.combine(&quote! {
//             #cfg
//             impl<#(#constraints)*> ::core::convert::From<#name> for #into {
//                 fn from(value: #name) -> Self {
//                     unsafe { ::core::mem::transmute(value) }
//                 }
//             }
//             #cfg
//             impl<#(#constraints)*> ::core::convert::From<&#name> for #into {
//                 fn from(value: &#name) -> Self {
//                     ::core::convert::From::from(::core::clone::Clone::clone(value))
//                 }
//             }
//             #cfg
//             impl<'a, #(#constraints)*> ::windows::core::IntoParam<'a, #into> for #name {
//                 fn into_param(self) -> ::windows::core::Param<'a, #into> {
//                     ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
//                 }
//             }
//             #cfg
//             impl<'a, #(#constraints)*> ::windows::core::IntoParam<'a, #into> for &'a #name {
//                 fn into_param(self) -> ::windows::core::Param<'a, #into> {
//                     ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
//                 }
//             }
//         });
//     }
//
//     if def.is_winrt() {
//         for def in def.required_interfaces() {
//             let into = gen_type_name(&def, gen);
//             let cfg = gen.cfg(&cfg.union(&def.cfg()));
//             tokens.combine(&quote! {
//                 #cfg
//                 impl<#(#constraints)*> ::core::convert::TryFrom<#name> for #into {
//                     type Error = ::windows::core::Error;
//                     fn try_from(value: #name) -> ::windows::core::Result<Self> {
//                         ::core::convert::TryFrom::try_from(&value)
//                     }
//                 }
//                 #cfg
//                 impl<#(#constraints)*> ::core::convert::TryFrom<&#name> for #into {
//                     type Error = ::windows::core::Error;
//                     fn try_from(value: &#name) -> ::windows::core::Result<Self> {
//                         ::windows::core::Interface::cast(value)
//                     }
//                 }
//                 #cfg
//                 impl<'a, #(#constraints)*> ::windows::core::IntoParam<'a, #into> for #name {
//                     fn into_param(self) -> ::windows::core::Param<'a, #into> {
//                         ::windows::core::IntoParam::into_param(&self)
//                     }
//                 }
//                 #cfg
//                 impl<'a, #(#constraints)*> ::windows::core::IntoParam<'a, #into> for &#name {
//                     fn into_param(self) -> ::windows::core::Param<'a, #into> {
//                         ::core::convert::TryInto::<#into>::try_into(self)
//                             .map(::windows::core::Param::Owned)
//                             .unwrap_or(::windows::core::Param::None)
//                     }
//                 }
//             });
//         }
//     }
//
//     tokens
// }
//
// fn gen_agile(def: &TypeDef, gen: &Gen) -> TokenStream {
//     if def.type_name() == TypeName::IRestrictedErrorInfo || def.async_kind() != AsyncKind::None {
//         let name = gen_type_ident(def, gen);
//         let constraints = gen_type_constraints(def, gen);
//         quote! {
//             unsafe impl<#(#constraints)*> ::core::marker::Send for #name {}
//             unsafe impl<#(#constraints)*> ::core::marker::Sync for #name {}
//         }
//     } else {
//         TokenStream::new()
//     }
// }
