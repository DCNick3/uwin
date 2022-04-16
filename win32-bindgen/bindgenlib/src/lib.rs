mod callbacks;
mod classes;
mod constants;
mod enums;
mod functions;
mod gen;
mod handles;
mod helpers;
mod implements;
mod interfaces;
mod method_names;
mod methods;
mod names;
mod replacements;
mod signatures;
mod structs;

use functions::*;
pub use gen::*;
use helpers::*;
use metadata::*;
// use method_names::*;
// use methods::*;
use names::*;
use signatures::*;
use tokens::*;

// pub fn gen_type(name: &str, gen: &Gen) -> String {
//     let reader = TypeReader::get();
//     let mut tokens = String::new();
//
//     for def in reader
//         .get_type_entry(TypeName::parse(name))
//         .iter()
//         .flat_map(|entry| entry.iter())
//     {
//         tokens.push_str(gen_type_impl(def, gen).as_str());
//     }
//
//     assert!(!tokens.is_empty(), "`{}` not found", name);
//     tokens
// }

pub struct GeneratedNamespace {
    pub module: String,
    pub thunk_functions: TokenStream,
}

pub fn gen_namespace(gen: &Gen, child_namespaces: &Vec<String>) -> GeneratedNamespace {
    let tree = TypeReader::get()
        .get_namespace(gen.namespace)
        .expect("Namespace not found");

    let namespaces = child_namespaces.iter().map(|child| {
        let name = gen_ident(child);

        quote! {
            pub mod #name;
        }
    });

    // let functions = gen_sys_functions(tree, gen);
    let types = gen_non_sys_function_types(tree, gen);
    let api_trait = gen_functions(tree, gen);

    let tokens = quote! {
        #![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
        #[allow(unused)]
        use ::win32::core::prelude::*;
        #(#namespaces)*
        #types
        #api_trait
    };

    let thunk_functions = gen_rusty_x86_thunk_functions(tree, gen);

    GeneratedNamespace {
        module: tokens.into_string(),
        thunk_functions,
    }
}

// TODO: wtf is this?
// #[allow(unused)]
// pub fn gen_namespace_impl(gen: &Gen) -> String {
//     let tree = TypeReader::get()
//         .get_namespace(gen.namespace)
//         .expect("Namespace not found");
//     let mut tokens = TokenStream::new();
//
//     for entry in tree.types.values() {
//         for def in entry {
//             if let Type::TypeDef(def) = def {
//                 let def = &def.clone().with_generics();
//                 tokens.combine(&implements::gen(def, gen));
//             }
//         }
//     }
//
//     tokens.into_string()
// }

fn gen_non_sys_function_types(tree: &TypeTree, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for entry in tree.types.values() {
        for def in entry {
            tokens.combine(&gen_type_impl(def, gen));
        }
    }

    tokens
}

fn gen_type_impl(def: &Type, gen: &Gen) -> TokenStream {
    match def {
        Type::Field(def) => {
            if gen.excluded_items.contains(def.name()) {
                return quote! {};
            }
            constants::gen(def, gen)
        }
        Type::TypeDef(def) => {
            if gen.excluded_items.contains(def.name()) {
                return quote! {};
            }
            let def = &def.clone().with_generics();
            match def.kind() {
                TypeKind::Class => classes::gen(def, gen),
                TypeKind::Interface => interfaces::gen(def, gen),
                TypeKind::Enum => enums::gen(def, gen),
                TypeKind::Struct => structs::gen(def, gen),
                TypeKind::Delegate => {
                    assert!(!def.is_winrt());
                    callbacks::gen(def, gen)
                }
            }
        }
        // Type::MethodDef(def) => {
        //     if gen.excluded_items.contains(def.name()) {
        //         return quote! {};
        //     }
        //     if !gen.sys {
        //         gen_function(def, gen)
        //     } else {
        //         quote! {}
        //     }
        // }
        _ => quote! {},
    }
}
