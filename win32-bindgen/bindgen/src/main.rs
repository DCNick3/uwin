use rayon::prelude::*;
use std::io::Write;
use win32_bindgenlib as bindgen;

const EXCLUDE_NAMESPACES: [&str; 1] = ["Windows.Win32.Interop"];
const INCLUDE_NAMESPACES: [&str; 2] = [
    "Windows.Win32.Foundation",
    "Windows.Win32.System.SystemInformation",
];

fn main() {
    let mut output = std::path::PathBuf::from("win32/src/Windows");
    let _ = std::fs::remove_dir_all(&output);
    output.pop();

    let reader = metadata::TypeReader::get();
    let root = reader.types.get_namespace("Windows").unwrap();

    let mut trees = Vec::new();
    collect_trees(&output, root.namespace, root, &mut trees);
    trees
        .par_iter()
        .for_each(|tree| gen_tree(&output, root.namespace, tree));

    output.pop();
    output.push("Cargo.toml");

    let mut file = std::fs::File::create(&output).unwrap();

    file.write_all(
        r#"
[package]
name = "win32"
version = "0.0.1"
edition = "2018"

[dependencies]
win32-mem = { path = "../win32-mem" }
"#
        .as_bytes(),
    )
    .unwrap();
}

struct TypeTreeGen<'a> {
    tree: &'a metadata::TypeTree,
    child_namespaces: Vec<String>,
}

fn collect_trees<'a>(
    output: &std::path::Path,
    root: &'static str,
    tree: &'a metadata::TypeTree,
    trees: &mut Vec<TypeTreeGen<'a>>,
) -> bool {
    if EXCLUDE_NAMESPACES.iter().any(|&x| x == tree.namespace) {
        return false;
    }

    let nested_namespaces: Vec<String> = tree
        .namespaces
        .values()
        .filter(|tree| collect_trees(output, root, tree, trees))
        .map(|tt| tt.namespace[tt.namespace.rfind('.').unwrap() + 1..].replace('.', "_"))
        .collect();

    let include_nested = !nested_namespaces.is_empty();

    let include = INCLUDE_NAMESPACES.iter().any(|&x| x == tree.namespace);
    if include || include_nested {
        trees.push(TypeTreeGen {
            tree: tree,
            child_namespaces: nested_namespaces,
        });

        let mut path = std::path::PathBuf::from(output);
        path.push(tree.namespace.replace('.', "/"));
        std::fs::create_dir_all(&path).unwrap();
    }

    include || include_nested
}

fn gen_tree(output: &std::path::Path, _root: &'static str, tree: &TypeTreeGen) {
    let TypeTreeGen {
        tree,
        child_namespaces,
    } = tree;

    println!("{}", tree.namespace);

    let path = std::path::PathBuf::from(output).join(tree.namespace.replace('.', "/"));
    let gen = bindgen::Gen {
        enabled_namespaces: &INCLUDE_NAMESPACES,
        namespace: tree.namespace,
        min_xaml: true,
        cfg: true,
        doc: true,
        ..Default::default()
    };

    let mut tokens = bindgen::gen_namespace(&gen, &child_namespaces);
    //tokens.push_str(r#"#[cfg(feature = "implement")] ::core::include!("impl.rs");"#);
    fmt_tokens(tree.namespace, &mut tokens);

    std::fs::write(path.join("mod.rs"), tokens).unwrap();

    // TODO: what is gen_namespace_impl?
    // do we need it?

    // let mut tokens = bindgen::gen_namespace_impl(&gen);
    // fmt_tokens(tree.namespace, &mut tokens);
    // std::fs::write(path.join("impl.rs"), tokens).unwrap();
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
