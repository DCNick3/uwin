use super::*;
use convert_case::{Case, Casing};

pub fn gen_ident(name: &str) -> TokenStream {
    // keywords list based on https://doc.rust-lang.org/reference/keywords.html
    match name {
        "abstract" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do"
        | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in"
        | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "override" | "priv"
        | "pub" | "ref" | "return" | "static" | "struct" | "super" | "trait" | "true" | "type"
        | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield"
        | "try" | "async" | "await" | "dyn" => format!("r#{}", name).into(),
        "Self" | "self" => format!("{}_", name).into(),
        "_" => "unused".into(),
        _ => trim_tick(name).into(),
    }
}

// TODO: use above instead
pub fn gen_type_ident(def: &TypeDef, gen: &Gen) -> TokenStream {
    gen_type_ident_impl(def, gen, "")
}

// TODO: use above instead
#[allow(unused)]
pub fn gen_vtbl_ident(def: &TypeDef, gen: &Gen) -> TokenStream {
    gen_type_ident_impl(def, gen, "_Vtbl")
}

// TODO: use above instead
#[allow(unused)]
pub fn gen_impl_ident(def: &TypeDef, gen: &Gen) -> TokenStream {
    gen_type_ident_impl(def, gen, "_Impl")
}

// TODO: use above instead
pub fn gen_trait_ident(def: &TypeDef, gen: &Gen) -> TokenStream {
    gen_type_ident_impl(def, gen, "_Trait")
}

fn gen_type_ident_impl(def: &TypeDef, gen: &Gen, vtbl: &str) -> TokenStream {
    let mut name = gen_ident(def.name());
    name.push_str(vtbl);

    if gen.sys || def.generics.is_empty() {
        name
    } else {
        name.push('<');

        for g in &def.generics {
            name.push_str(gen_element_name(g, gen).as_str());
            name.push(',');
        }

        name.push('>');
        name
    }
}

pub fn gen_phantoms(def: &TypeDef, gen: &Gen) -> Vec<TokenStream> {
    def.generics
        .iter()
        .map(|g| {
            let name = gen_element_name(g, gen);
            quote! { ::core::marker::PhantomData::<#name>, }
        })
        .collect()
}

#[allow(unused)]
pub fn gen_named_phantoms(def: &TypeDef, gen: &Gen) -> Vec<TokenStream> {
    def.generics
        .iter()
        .map(|g| {
            let name = gen_element_name(g, gen);
            quote! { #name: ::core::marker::PhantomData::<#name>, }
        })
        .collect()
}

#[allow(unused)]
pub fn gen_type_generics(def: &TypeDef, gen: &Gen) -> Vec<TokenStream> {
    def.generics
        .iter()
        .map(|g| {
            let name = gen_element_name(g, gen);
            quote! { #name, }
        })
        .collect()
}

pub fn gen_type_constraints(def: &TypeDef, gen: &Gen) -> Vec<TokenStream> {
    def.generics
        .iter()
        .map(|g| {
            let name = gen_element_name(g, gen);
            quote! { #name: ::windows::core::RuntimeType + 'static, }
        })
        .collect()
}

pub fn gen_param_name(param: &Param) -> TokenStream {
    gen_ident(&param.name().to_case(Case::Snake))
}

pub fn gen_element_name(def: &Type, gen: &Gen) -> TokenStream {
    match def {
        Type::Void => quote! { ::core::ffi::c_void },
        Type::Bool => quote! { bool },
        Type::Char => quote! { u16 },
        Type::I8 => quote! { i8 },
        Type::U8 => quote! { u8 },
        Type::I16 => quote! { i16 },
        Type::U16 => quote! { u16 },
        Type::I32 => quote! { i32 },
        Type::U32 => quote! { u32 },
        Type::I64 => quote! { i64 },
        Type::U64 => quote! { u64 },
        Type::F32 => quote! { f32 },
        Type::F64 => quote! { f64 },
        Type::ISize => quote! { PtrDiffRepr },
        Type::USize => quote! { PtrRepr },
        Type::String => quote! { crate::core::HSTRING },
        Type::IInspectable => quote! { crate::core::IInspectable },
        Type::GUID => quote! { crate::core::GUID },
        Type::IUnknown => quote! { crate::core::IUnknown },
        Type::HRESULT => quote! { crate::core::HRESULT },
        Type::PSTR => quote! { PSTR },
        Type::PWSTR => quote! { PWSTR },
        Type::PCSTR => quote! { PCSTR },
        Type::PCWSTR => quote! { PCWSTR },
        Type::Win32Array((kind, len)) => {
            let name = gen_default_type(kind, gen);
            let len = Literal::u32_unsuffixed(*len);
            quote! { [#name; #len] }
        }
        Type::GenericParam(generic) => (*generic).into(),
        Type::MethodDef(def) => def.name().into(),
        Type::Field(field) => field.name().into(),
        Type::TypeDef(t) => gen_type_name(t, gen),
        Type::MutPtr((kind, pointers)) => {
            let kind = gen_default_type(kind, gen);
            let pointers = gen_mut_ptrs(*pointers, kind);
            quote! { #pointers }
        }
        Type::ConstPtr((kind, pointers)) => {
            let kind = gen_default_type(kind, gen);
            let pointers = gen_const_ptrs(*pointers, kind);
            quote! { #pointers }
        }
        Type::WinrtArray(kind) => gen_element_name(kind, gen),
        Type::WinrtArrayRef(kind) => gen_element_name(kind, gen),
        Type::WinrtConstRef(kind) => gen_element_name(kind, gen),
        Type::TypeName => unimplemented!(),
    }
}

fn gen_mut_ptrs(pointers: usize, kind: TokenStream) -> TokenStream {
    if pointers == 0 {
        return kind;
    }
    let kind = gen_const_ptrs(pointers - 1, kind);
    return quote! {MutPtr<#kind>};
}

fn gen_const_ptrs(pointers: usize, kind: TokenStream) -> TokenStream {
    if pointers == 0 {
        return kind;
    }
    let kind = gen_const_ptrs(pointers - 1, kind);
    return quote! {ConstPtr<#kind>};
}

// pub fn gen_abi_element_name(ty: &Type, gen: &Gen) -> TokenStream {
//     gen_abi_element_name_impl(ty, false, gen)
// }

// TODO: this is only because we're trying to avoid the ManuallyDrop below - I don't think that matters so may want to scrap this once we have parity.
// fn gen_abi_element_name_impl(ty: &Type, ptr: bool, gen: &Gen) -> TokenStream {
//     match ty {
//         Type::String => {
//             todo!()
//             // quote! { ::core::mem::ManuallyDrop<::windows::core::HSTRING> }
//         }
//         Type::IUnknown | Type::IInspectable => {
//             todo!()
//             // quote! { *mut ::core::ffi::c_void }
//         }
//         Type::Win32Array((kind, len)) => {
//             todo!()
//             // let name = gen_abi_element_name_impl(kind, ptr, gen);
//             // let len = Literal::u32_unsuffixed(*len);
//             // quote! { [#name; #len] }
//         }
//         Type::GenericParam(_) => {
//             unimplemented!();
//             //let name = gen_ident(generic);
//             //quote! { <#name as ::windows::core::Abi>::Abi }
//         }
//         Type::TypeDef(def) => match def.kind() {
//             TypeKind::Enum => gen_type_name(def, gen),
//             TypeKind::Struct => {
//                 let tokens = gen_type_name(def, gen);
//                 if def.is_blittable() || ptr {
//                     tokens
//                 } else {
//                     quote! { ::core::mem::ManuallyDrop<#tokens> }
//                 }
//             }
//             _ => quote! { ::windows::core::RawPtr },
//         },
//         Type::MutPtr((kind, pointers)) => {
//             let pointers = gen_mut_ptrs(*pointers);
//             let kind = gen_abi_element_name_impl(kind, true, gen);
//             quote! { #pointers #kind }
//         }
//         Type::ConstPtr((kind, pointers)) => {
//             let pointers = gen_const_ptrs(*pointers);
//             let kind = gen_abi_element_name_impl(kind, true, gen);
//             quote! { #pointers #kind }
//         }
//         Type::WinrtArray(kind) => gen_abi_element_name_impl(kind, ptr, gen),
//         Type::WinrtArrayRef(kind) => gen_abi_element_name_impl(kind, ptr, gen),
//         _ => gen_element_name(ty, gen),
//     }
// }

pub fn gen_type_name(def: &TypeDef, gen: &Gen) -> TokenStream {
    format_name(def, gen, gen_ident, false)
}

fn format_name<F>(def: &TypeDef, gen: &Gen, format_name: F, turbo: bool) -> TokenStream
where
    F: FnOnce(&str) -> TokenStream,
{
    let type_name = def.type_name();

    if type_name.namespace.is_empty() {
        format_name(&scoped_name(def))
    } else {
        let mut namespace = gen.namespace(type_name.namespace);
        let name = format_name(type_name.name);

        if def.generics.is_empty() || gen.sys {
            namespace.combine(&name);
            namespace
        } else {
            let colon_separated = if turbo || !namespace.as_str().is_empty() {
                quote! { :: }
            } else {
                quote! {}
            };

            let generics = def.generics.iter().map(|g| gen_element_name(g, gen));
            quote! { #namespace #name #colon_separated <#(#generics),*> }
        }
    }
}

fn scoped_name(def: &TypeDef) -> String {
    if let Some(enclosing_type) = def.enclosing_type() {
        if let Some(nested_types) = enclosing_type.nested_types() {
            for (index, (nested_type, _)) in nested_types.iter().enumerate() {
                if *nested_type == def.name() {
                    return format!("{}_{}", &scoped_name(&enclosing_type), index);
                }
            }
        }
    }

    def.name().to_string()
}
