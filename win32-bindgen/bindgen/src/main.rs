#[allow(unused)]
use rayon::prelude::*;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use tokens::{quote, TokenStream};
use win32_bindgenlib as bindgen;
use win32_bindgenlib::{ComClass, GeneratedNamespace};

const CONFIG: &str = include_str!("config.yml");

#[derive(Deserialize)]
struct BindgenConfig {
    pub include_namespaces: BTreeSet<String>,
    pub exclude_items: BTreeMap<String, BTreeSet<String>>,
    pub exclude_libraries: BTreeSet<String>,
    pub unwindable_functions: BTreeSet<String>,
    pub callbacking_functions: BTreeSet<String>,
    pub com_classes: BTreeMap<String, Vec<ComClass>>,
}

fn main() {
    let config: BindgenConfig = serde_yaml::from_str(CONFIG).expect("Parsing config");
    let config = Box::new(config);
    let config = Box::leak(config);

    let mut output = std::path::PathBuf::from("win32/src/Windows");
    let _ = std::fs::remove_dir_all(&output);
    output.pop();

    let reader = metadata::TypeReader::get();
    let root = reader.types.get_namespace("Windows").unwrap();

    let mut trees = Vec::new();
    collect_trees(config, &output, root.namespace, root, &mut trees);
    let thunk_functions = trees
        // .par_iter()
        .iter()
        .map(|tree| gen_tree(config, &output, root.namespace, tree))
        .collect::<Vec<_>>();

    let output = std::path::PathBuf::from("rusty_x86_runtime/src/thunks.rs");
    gen_thunks(&output, thunk_functions);
}

struct TypeTreeGen<'a> {
    tree: &'a metadata::TypeTree,
    child_namespaces: Vec<String>,
}

fn collect_trees<'a>(
    config: &'static BindgenConfig,
    output: &std::path::Path,
    root: &'static str,
    tree: &'a metadata::TypeTree,
    trees: &mut Vec<TypeTreeGen<'a>>,
) -> bool {
    let nested_namespaces: Vec<String> = tree
        .namespaces
        .values()
        .filter(|tree| collect_trees(config, output, root, tree, trees))
        .map(|tt| tt.namespace[tt.namespace.rfind('.').unwrap() + 1..].replace('.', "_"))
        .collect();

    let include_nested = !nested_namespaces.is_empty();

    let include = config.include_namespaces.contains(tree.namespace);
    if include || include_nested {
        trees.push(TypeTreeGen {
            tree,
            child_namespaces: nested_namespaces,
        });

        let mut path = std::path::PathBuf::from(output);
        path.push(tree.namespace.replace('.', "/"));
        std::fs::create_dir_all(&path).unwrap();
    }

    include || include_nested
}

fn gen_tree(
    config: &'static BindgenConfig,
    output: &std::path::Path,
    _root: &'static str,
    tree: &TypeTreeGen,
) -> TokenStream {
    let TypeTreeGen {
        tree,
        child_namespaces,
    } = tree;

    println!("{}", tree.namespace);

    let empty_set = BTreeSet::new();
    let empty_class_list = Vec::new();

    let path = std::path::PathBuf::from(output).join(tree.namespace.replace('.', "/"));
    let gen = bindgen::Gen {
        included_namespaces: &config.include_namespaces,
        excluded_items: config
            .exclude_items
            .get(tree.namespace)
            .unwrap_or(&empty_set),
        excluded_libraries: &config.exclude_libraries,
        unwindable_functions: &config.unwindable_functions,
        callbacking_functions: &config.callbacking_functions,
        com_classes: config
            .com_classes
            .get(tree.namespace)
            .unwrap_or(&empty_class_list),
        namespace: tree.namespace,
        sys: false,
        min_xaml: true,
        cfg: true,
        doc: true,
        min_enum: false,
        flatten: false,
        min_inherit: false,
    };

    let GeneratedNamespace {
        module: mut tokens,
        thunk_functions,
    } = bindgen::gen_namespace(&gen, &child_namespaces);
    fmt_tokens(tree.namespace, &mut tokens);

    std::fs::write(path.join("mod.rs"), tokens).unwrap();

    thunk_functions
}

fn gen_thunks(output: &std::path::Path, tokens: Vec<TokenStream>) {
    // output rusty_x86 thunk functions separately
    let mut tokens = quote! {
        #![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all, unused_mut, unused_variables)]

        #[allow(unused)]
        use crate::ExtendedContext;

        #[allow(unused)]
        use core_mem::ctx::FlatMemoryCtx;

        #[allow(unused)]
        use core_mem::ptr::PtrRepr;

        #[allow(unused)]
        use tracing::Callsite;

        #(#tokens)*
    }
    .into_string();

    fmt_tokens("thunks", &mut tokens);

    std::fs::write(output, tokens).unwrap();
}

fn fmt_tokens(namespace: &str, tokens: &mut String) {
    let mut child = std::process::Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to spawn `rustfmt`");
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(tokens.as_bytes()).unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();

    if output.status.success() {
        *tokens = String::from_utf8(output.stdout).expect("Failed to parse UTF-8");
    } else {
        println!("** {} - rustfmt failed", namespace);
    }
}

//
// fn main() {
//     let types = &[
//         "Windows.Win32.Foundation.BOOL",
//         //"Windows.Win32.Foundation.BSTR",
//         "Windows.Win32.Foundation.CLASS_E_CLASSNOTAVAILABLE",
//         //"Windows.Win32.Foundation.CloseHandle",
//         "Windows.Win32.Foundation.CO_E_NOTINITIALIZED",
//         "Windows.Win32.Foundation.E_NOINTERFACE",
//         "Windows.Win32.Foundation.E_OUTOFMEMORY",
//         "Windows.Win32.Foundation.FARPROC",
//         // "Windows.Win32.Foundation.GetLastError",
//         "Windows.Win32.Foundation.HANDLE",
//         "Windows.Win32.Foundation.HINSTANCE",
//         "Windows.Win32.Foundation.S_OK",
//         // "Windows.Win32.Foundation.SysAllocStringLen",
//         // "Windows.Win32.Foundation.SysFreeString",
//         // "Windows.Win32.Foundation.SysStringLen",
//         "Windows.Win32.Foundation.WIN32_ERROR",
//         "Windows.Win32.Security.SECURITY_ATTRIBUTES",
//         "Windows.Win32.System.Diagnostics.Debug.FORMAT_MESSAGE_OPTIONS",
//         // "Windows.Win32.System.Diagnostics.Debug.FormatMessageW",
//         // "Windows.Win32.System.LibraryLoader.FreeLibrary",
//         // "Windows.Win32.System.LibraryLoader.GetProcAddress",
//         // "Windows.Win32.System.LibraryLoader.LoadLibraryA",
//         // "Windows.Win32.System.Memory.GetProcessHeap",
//         "Windows.Win32.System.Memory.HEAP_FLAGS",
//         // "Windows.Win32.System.Memory.HeapAlloc",
//         // "Windows.Win32.System.Memory.HeapFree",
//         // "Windows.Win32.System.Memory.HeapHandle",
//         // "Windows.Win32.System.Threading.CreateEventA",
//         // "Windows.Win32.System.Threading.SetEvent",
//         // "Windows.Win32.System.Threading.WaitForSingleObject",
//     ];
//
//     // stuff to change:
//     // - namespace (there are a lot of refs to ::windows::code)
//     // - fix generation of code with different namespaces (it should actually work!)
//     // - plain old pointers (also need a wrapper type, though this one is implemented)
//     // - generate conversion trait implementations
//
//     // questions to answer:
//     // - representation of function pointers
//     // - representation of COM interface implementations (extra hard!)
//
//     let mut tokens = "#![allow(non_snake_case, non_upper_case_globals, dead_code, non_camel_case_types, clippy::upper_case_acronyms, clippy::derivable_impls)]".to_string();
//
//     let gen = Gen {
//         namespace: "Windows.",
//         min_enum: true,
//         min_inherit: true,
//         flatten: true,
//         ..Default::default()
//     };
//
//     for name in types {
//         tokens += &gen_type(name, &gen);
//     }
//
//     let tokens = reformat(tokens).unwrap();
//
//     println!("{}", tokens);
// }
