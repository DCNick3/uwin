#[allow(unused)]
use rayon::prelude::*;
use std::io::Write;
use tokens::{quote, TokenStream};
use win32_bindgenlib as bindgen;
use win32_bindgenlib::GeneratedNamespace;

const EXCLUDE_NAMESPACES: &[&str] = &["Windows.Win32.Interop"];
const INCLUDE_NAMESPACES: &[&str] = &[
    "Windows.Win32.Foundation",
    "Windows.Win32.Globalization",
    "Windows.Win32.Storage.FileSystem",
    "Windows.Win32.System.Console",
    "Windows.Win32.System.Diagnostics.Debug",
    "Windows.Win32.System.Environment",
    "Windows.Win32.System.IO",
    "Windows.Win32.System.Kernel",
    "Windows.Win32.System.LibraryLoader",
    "Windows.Win32.System.Memory",
    "Windows.Win32.System.SystemInformation",
    "Windows.Win32.System.SystemServices",
    "Windows.Win32.System.Threading",
    "Windows.Win32.System.WindowsProgramming",
    "Windows.Win32.UI.WindowsAndMessaging",
];

const EXCLUDE_ITEMS: &[(&str, &[&str])] = &[
    (
        "Windows.Win32.System.Diagnostics.Debug",
        &[
            "DebugPropertyInfo",
            "DebugHelper",
            "DebugStackFrameDescriptor",
            "DebugStackFrameDescriptor64",
            "DebugPropertyInfo",
            "ScriptDebugEventInformation",
            "ExtendedDebugPropertyInfo",
            "PDEBUG_EXTENSION_CALL",
            "PDEBUG_EXTENSION_KNOWN_STRUCT_EX",
            "PDEBUG_EXTENSION_PROVIDE_VALUE",
            "PDEBUG_EXTENSION_QUERY_VALUE_NAMES",
            "TEXT_DOCUMENT_ARRAY",
            "PROFILER_HEAP_OBJECT_RELATIONSHIP",
            "PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST",
            "PROFILER_HEAP_OBJECT_OPTIONAL_INFO",
        ],
    ),
    (
        "Windows.Win32.Foundation",
        &[
            "RtlNtStatusToDosError",
            "SetLastErrorEx",
            "SysAddRefString",
            "SysAllocString",
            "SysAllocStringByteLen",
            "SysAllocStringLen",
            "SysFreeString",
            "SysReAllocString",
            "SysReAllocStringLen",
            "SysReleaseString",
            "SysStringByteLen",
            "SysStringLen",
        ],
    ),
    (
        "Windows.Win32.System.Diagnostics.Debug",
        &[
            "AddVectoredContinueHandler",
            "AddVectoredExceptionHandler",
            "Beep",
            "BindImage",
            "BindImageEx",
            "CheckRemoteDebuggerPresent",
            "CheckSumMappedFile",
            "CheckSumMappedFile",
            "CloseThreadWaitChainSession",
            "ContinueDebugEvent",
            "CopyContext",
            "CreateDataModelManager",
            "DbgHelpCreateUserDump",
            "DbgHelpCreateUserDumpW",
            "DebugActiveProcess",
            "DebugActiveProcessStop",
            "DebugBreak",
            "DebugBreakProcess",
            "DebugConnect",
            "DebugConnectWide",
            "DebugCreate",
            "DebugCreateEx",
            "DebugSetProcessKillOnExit",
            "DecodePointer",
            "DecodeRemotePointer",
            "DecodeSystemPointer",
            "EncodePointer",
            "EncodeRemotePointer",
            "EncodeSystemPointer",
            "EnumDirTree",
            "EnumDirTreeW",
            "EnumerateLoadedModules",
            "EnumerateLoadedModules64",
            "EnumerateLoadedModulesEx",
            "EnumerateLoadedModulesExW",
            "EnumerateLoadedModulesW64",
            "FatalAppExitA",
            "FatalAppExitW",
            "FatalExit",
            "FindDebugInfoFile",
            "FindDebugInfoFileEx",
            "FindDebugInfoFileExW",
            "FindExecutableImage",
            "FindExecutableImageEx",
            "FindExecutableImageExW",
            "FindFileInPath",
            "FindFileInSearchPath",
            "FlushInstructionCache",
            "FormatMessageA",
            "FormatMessageW",
            "GetEnabledXStateFeatures",
            "GetErrorMode",
            "GetImageConfigInformation",
            "GetImageConfigInformation",
            "GetImageUnusedHeaderBytes",
            "GetSymLoadError",
            "GetThreadContext",
            "GetThreadErrorMode",
            "GetThreadSelectorEntry",
            "GetThreadWaitChain",
            "GetTimestampForLoadedLibrary",
            "GetXStateFeaturesMask",
            "ImageAddCertificate",
            "ImageDirectoryEntryToData",
            "ImageDirectoryEntryToDataEx",
            "ImageEnumerateCertificates",
            "ImageGetCertificateData",
            "ImageGetCertificateHeader",
            "ImageGetDigestStream",
            "ImageLoad",
            "ImageNtHeader",
            "ImageNtHeader",
            "ImageRemoveCertificate",
            "ImageRvaToSection",
            "ImageRvaToSection",
            "ImageRvaToVa",
            "ImageRvaToVa",
            "ImageUnload",
            "ImagehlpApiVersion",
            "ImagehlpApiVersionEx",
            "InitializeContext",
            "InitializeContext2",
            "IsDebuggerPresent",
            "LocateXStateFeature",
            "MakeSureDirectoryPathExists",
            "MapAndLoad",
            "MapFileAndCheckSumA",
            "MapFileAndCheckSumW",
            "MessageBeep",
            "MiniDumpReadDumpStream",
            "MiniDumpWriteDump",
            "OpenThreadWaitChainSession",
            "OutputDebugStringA",
            "OutputDebugStringW",
            "RaiseException",
            "RaiseFailFastException",
            "RangeMapAddPeImageSections",
            "RangeMapCreate",
            "RangeMapFree",
            "RangeMapRead",
            "RangeMapRemove",
            "RangeMapWrite",
            "ReBaseImage",
            "ReBaseImage64",
            "ReadProcessMemory",
            "RegisterWaitChainCOMCallback",
            "RemoveInvalidModuleList",
            "RemoveVectoredContinueHandler",
            "RemoveVectoredExceptionHandler",
            "ReportSymbolLoadSummary",
            "RtlAddFunctionTable",
            "RtlAddFunctionTable",
            "RtlAddGrowableFunctionTable",
            "RtlAddGrowableFunctionTable",
            "RtlCaptureContext",
            "RtlCaptureContext2",
            "RtlCaptureStackBackTrace",
            "RtlDeleteFunctionTable",
            "RtlDeleteFunctionTable",
            "RtlDeleteGrowableFunctionTable",
            "RtlGrowFunctionTable",
            "RtlInstallFunctionTableCallback",
            "RtlLookupFunctionEntry",
            "RtlLookupFunctionEntry",
            "RtlPcToFileHeader",
            "RtlRaiseException",
            "RtlRestoreContext",
            "RtlUnwindEx",
            "RtlVirtualUnwind",
            "RtlVirtualUnwind",
            "SearchTreeForFile",
            "SearchTreeForFileW",
            "SetCheckUserInterruptShared",
            "SetErrorMode",
            "SetImageConfigInformation",
            "SetImageConfigInformation",
            "SetSymLoadError",
            "SetThreadContext",
            "SetThreadErrorMode",
            "SetUnhandledExceptionFilter",
            "SetXStateFeaturesMask",
            "StackWalk",
            "StackWalk64",
            "StackWalkEx",
            "TerminateProcessOnMemoryExhaustion",
            "TouchFileTimes",
            "UnDecorateSymbolName",
            "UnDecorateSymbolNameW",
            "UnMapAndLoad",
            "UpdateDebugInfoFile",
            "UpdateDebugInfoFileEx",
            "WaitForDebugEvent",
            "WaitForDebugEventEx",
            "Wow64GetThreadContext",
            "Wow64GetThreadSelectorEntry",
            "Wow64SetThreadContext",
            "WriteProcessMemory",
        ],
    ),
    (
        "Windows.Win32.System.SystemInformation",
        &[
            "DnsHostnameToComputerNameExW",
            "EnumSystemFirmwareTables",
            "GetFirmwareType",
            "GetIntegratedDisplaySize",
            "IsWow64GuestMachineSupported",
            "IsUserCetAvailableInEnvironment",
        ],
    ),
    (
        "Windows.Win32.Storage.FileSystem",
        &[
            "LockFileEx",
            "LockFile",
            "LocalFileTimeToFileTime",
            "BuildIoRingCancelRequest",
            "BuildIoRingReadFile",
        ],
    ),
    (
        "Windows.Win32.System.Console",
        &["GetConsoleFontSize", "GetLargestConsoleWindowSize"],
    ),
    (
        "Windows.Win32.System.SystemServices",
        &["DISPATCHER_CONTEXT_NONVOLREG_ARM64"],
    ),
];
const UNWINDABLE_FUNCTIONS: &[&str] = &["ExitThread", "ExitProcess"];

const EXCLUDE_LIBRARIES: &[&str] = &["icu", "clfsw32", "dbghelp", "mrmsupport", "dciman32"];

fn main() {
    let mut output = std::path::PathBuf::from("win32/src/Windows");
    let _ = std::fs::remove_dir_all(&output);
    output.pop();

    let reader = metadata::TypeReader::get();
    let root = reader.types.get_namespace("Windows").unwrap();

    let mut trees = Vec::new();
    collect_trees(&output, root.namespace, root, &mut trees);
    let thunk_functions = trees
        // .par_iter()
        .iter()
        .map(|tree| gen_tree(&output, root.namespace, tree))
        .collect::<Vec<_>>();

    let output = std::path::PathBuf::from("rusty_x86_runtime/src/thunks.rs");
    gen_thunks(&output, thunk_functions);
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
            tree,
            child_namespaces: nested_namespaces,
        });

        let mut path = std::path::PathBuf::from(output);
        path.push(tree.namespace.replace('.', "/"));
        std::fs::create_dir_all(&path).unwrap();
    }

    include || include_nested
}

fn gen_tree(output: &std::path::Path, _root: &'static str, tree: &TypeTreeGen) -> TokenStream {
    let TypeTreeGen {
        tree,
        child_namespaces,
    } = tree;

    println!("{}", tree.namespace);

    let path = std::path::PathBuf::from(output).join(tree.namespace.replace('.', "/"));
    let gen = bindgen::Gen {
        enabled_namespaces: &INCLUDE_NAMESPACES,
        excluded_items: EXCLUDE_ITEMS
            .iter()
            .filter(|(&ref namespace, _)| namespace == tree.namespace)
            .flat_map(|(_, &ref items)| items)
            .copied()
            .collect(),
        excluded_libraries: EXCLUDE_LIBRARIES.iter().copied().collect(),
        unwindable_functions: UNWINDABLE_FUNCTIONS.iter().copied().collect(),
        namespace: tree.namespace,
        min_xaml: true,
        cfg: true,
        doc: true,
        ..Default::default()
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
