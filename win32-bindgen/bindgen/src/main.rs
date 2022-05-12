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
    "Windows.Win32.Graphics.Gdi",
    "Windows.Win32.Graphics.DirectDraw",
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
            "PDEBUG_EXTENSION_CALL",
            "PDEBUG_EXTENSION_KNOWN_STRUCT_EX",
            "PDEBUG_EXTENSION_PROVIDE_VALUE",
            "PDEBUG_EXTENSION_QUERY_VALUE_NAMES",
            "TEXT_DOCUMENT_ARRAY",
            "PROFILER_HEAP_OBJECT_RELATIONSHIP",
            "PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST",
            "PROFILER_HEAP_OBJECT_OPTIONAL_INFO",
            "WHEA_PCI_SLOT_NUMBER",
            "WHEA_AER_ROOTPORT_DESCRIPTOR",
            "AER_ROOTPORT_DESCRIPTOR_FLAGS",
            "WHEA_AER_ENDPOINT_DESCRIPTOR",
            "AER_ENDPOINT_DESCRIPTOR_FLAGS",
            "WHEA_AER_BRIDGE_DESCRIPTOR",
            "AER_BRIDGE_DESCRIPTOR_FLAGS",
            "WHEA_DEVICE_DRIVER_DESCRIPTOR",
            "WHEA_DRIVER_BUFFER_SET",
            "WHEA_ERROR_SOURCE_CONFIGURATION_DD",
            "WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER",
            "WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1",
            "WHEA_ERROR_SOURCE_CORRECT_DEVICE_DRIVER",
            "WHEA_ERROR_SOURCE_DESCRIPTOR",
            "WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER",
            "WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER",
            "WHEA_GENERIC_ERROR_DESCRIPTOR",
            "WHEA_GENERIC_ERROR_DESCRIPTOR_V2",
            "WHEA_IPF_CMC_DESCRIPTOR",
            "WHEA_IPF_CPE_DESCRIPTOR",
            "WHEA_IPF_MCA_DESCRIPTOR",
            "WHEA_NOTIFICATION_FLAGS",
            "WHEA_NOTIFICATION_DESCRIPTOR",
            "WHEA_PCI_SLOT_NUMBER",
            "WHEA_XPF_CMC_DESCRIPTOR",
            "WHEA_XPF_MCE_DESCRIPTOR",
            "WHEA_XPF_MC_BANK_DESCRIPTOR",
            "WHEA_XPF_NMI_DESCRIPTOR",
            "XPF_MCE_FLAGS",
            "XPF_MC_BANK_FLAGS",
            "XPF_MC_BANK_FLAGS",
            "XSTATE_CONFIG_FEATURE_MSC_INFO",
            "CPU_INFORMATION",
            "MINIDUMP_SYSTEM_INFO",
            "IMAGE_FUNCTION_ENTRY64",
            "IMAGE_LOAD_CONFIG_DIRECTORY64",
            "IMAGE_OPTIONAL_HEADER64",
            "IMAGE_NT_HEADERS64",
            "LOADED_IMAGE",
            "IPMI_OS_SEL_RECORD",
            "MINIDUMP_CALLBACK_INFORMATION",
            "MINIDUMP_MODULE_CALLBACK",
            "MINIDUMP_CALLBACK_INPUT",
            "MINIDUMP_CALLBACK_OUTPUT",
            "MINIDUMP_CALLBACK_ROUTINE",
            "MINIDUMP_EXCEPTION",
            "MINIDUMP_EXCEPTION_INFORMATION",
            "MINIDUMP_EXCEPTION_INFORMATION64",
            "MINIDUMP_EXCEPTION_STREAM",
            "MINIDUMP_FUNCTION_TABLE_DESCRIPTOR",
            "MINIDUMP_HANDLE_DESCRIPTOR",
            "MINIDUMP_HANDLE_DESCRIPTOR_2",
            "MINIDUMP_HEADER",
            "MINIDUMP_INCLUDE_MODULE_CALLBACK",
            "MINIDUMP_INCLUDE_THREAD_CALLBACK",
            "MINIDUMP_IO_CALLBACK",
            "MINIDUMP_LOCATION_DESCRIPTOR64",
            "MINIDUMP_MEMORY64_LIST",
            "MINIDUMP_MEMORY_DESCRIPTOR",
            "MINIDUMP_MEMORY_DESCRIPTOR64",
            "MINIDUMP_MEMORY_INFO",
            "MINIDUMP_MEMORY_INFO_LIST",
            "MINIDUMP_MEMORY_INFO_LIST",
            "MINIDUMP_MEMORY_LIST",
            "MINIDUMP_MISC_INFO_5",
            "MINIDUMP_MODULE",
            "MINIDUMP_MODULE_CALLBACK",
            "MINIDUMP_MODULE_LIST",
            "MINIDUMP_PROCESS_VM_COUNTERS_1",
            "MINIDUMP_PROCESS_VM_COUNTERS_2",
            "MINIDUMP_READ_MEMORY_FAILURE_CALLBACK",
            "MINIDUMP_SYSTEM_BASIC_INFORMATION",
            "MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION",
            "MINIDUMP_SYSTEM_FILECACHE_INFORMATION",
            "MINIDUMP_SYSTEM_INFO",
            "MINIDUMP_SYSTEM_MEMORY_INFO_1",
            "MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION",
            "MINIDUMP_THREAD",
            "MINIDUMP_THREAD_CALLBACK",
            "MINIDUMP_THREAD_EX",
            "MINIDUMP_THREAD_EX_CALLBACK",
            "MINIDUMP_THREAD_EX_LIST",
            "MINIDUMP_THREAD_INFO",
            "MINIDUMP_THREAD_LIST",
            "MINIDUMP_THREAD_NAME",
            "MINIDUMP_THREAD_NAME_LIST",
            "MINIDUMP_TOKEN_INFO_HEADER",
            "MINIDUMP_TOKEN_INFO_LIST",
            "MINIDUMP_UNLOADED_MODULE",
            "MINIDUMP_UNLOADED_MODULE_LIST",
            "MINIDUMP_UNLOADED_MODULE_LIST",
            "MINIDUMP_USER_STREAM",
            "MINIDUMP_USER_STREAM_INFORMATION",
            "MINIDUMP_VM_POST_READ_CALLBACK",
            "MINIDUMP_VM_PRE_READ_CALLBACK",
            "MINIDUMP_VM_QUERY_CALLBACK",
            "ExtendedDebugPropertyInfo",
            "DebugPropertyInfo",
            "DebugHelper",
            "DebugStackFrameDescriptor",
            "DebugStackFrameDescriptor64",
            "DebugPropertyInfo",
            "ScriptDebugEventInformation",
            "AddVectoredContinueHandler",
            "AddVectoredExceptionHandler",
            "Beep",
            "BindImage",
            "BindImageEx",
            "CheckRemoteDebuggerPresent",
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
            "ImageRemoveCertificate",
            "ImageRvaToSection",
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
            "SearchTreeForFile",
            "SearchTreeForFileW",
            "SetCheckUserInterruptShared",
            "SetErrorMode",
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
        "Windows.Win32.System.Environment",
        &[
            "ENCLAVE_IDENTITY",
            "ENCLAVE_INFORMATION",
            "VBS_BASIC_ENCLAVE_BASIC_CALL_GET_ENCLAVE_INFORMATION",
            "VBS_ENCLAVE_REPORT",
            "VBS_ENCLAVE_REPORT_MODULE",
            "VBS_ENCLAVE_REPORT",
            "VBS_ENCLAVE_REPORT_PKG_HEADER",
            "VBS_ENCLAVE_REPORT_VARDATA_HEADER",
            "VBS_BASIC_ENCLAVE_SYSCALL_PAGE",
            "EnclaveGetEnclaveInformation",
            "EnclaveUnsealData",
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
            "TXF_ID",
            "TXF_LOG_RECORD_AFFECTED_FILE",
            "TXF_LOG_RECORD_TRUNCATE",
            "TXF_LOG_RECORD_WRITE",
            "LockFileEx",
            "LockFile",
            "LocalFileTimeToFileTime",
            "BuildIoRingCancelRequest",
            "BuildIoRingReadFile",
            "TxfLogCreateFileReadContext",
            "TxfLogCreateRangeReadContext",
            "TxfLogDestroyReadContext",
            "TxfLogReadRecords",
            "TxfReadMetadataInfo",
            "TxfLogRecordGetFileName",
            "TxfLogRecordGetGenericType",
            "TxfSetThreadMiniVersionForCreate",
            "TxfSetThreadMiniVersionForCreate",
        ],
    ),
    (
        "Windows.Win32.System.Console",
        &["GetConsoleFontSize", "GetLargestConsoleWindowSize"],
    ),
    (
        "Windows.Win32.System.SystemServices",
        &[
            "DISPATCHER_CONTEXT_NONVOLREG_ARM64",
            "IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY",
            "IMAGE_AUX_SYMBOL",
            "IMAGE_AUX_SYMBOL_EX",
            "IMAGE_AUX_SYMBOL_TOKEN_DEF",
            "IMAGE_DOS_HEADER",
            "IMAGE_DYNAMIC_RELOCATION32",
            "IMAGE_DYNAMIC_RELOCATION32_V2",
            "IMAGE_DYNAMIC_RELOCATION64",
            "IMAGE_DYNAMIC_RELOCATION64_V2",
            "IMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER",
            "IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION",
            "IMAGE_IMPORT_DESCRIPTOR",
            "IMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION",
            "IMAGE_LINENUMBER",
            "IMAGE_OS2_HEADER",
            "IMAGE_POLICY_ENTRY",
            "IMAGE_POLICY_METADATA",
            "IMAGE_RELOCATION",
            "IMAGE_RESOURCE_DIRECTORY_ENTRY",
            "IMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION",
            "IMAGE_SYMBOL",
            "IMAGE_SYMBOL_EX",
            "IMAGE_TLS_DIRECTORY32",
            "IMAGE_TLS_DIRECTORY64",
            "IMAGE_VXD_HEADER",
            "IMPORT_OBJECT_HEADER",
            "KERNEL_CET_CONTEXT",
            "NON_PAGED_DEBUG_INFO",
            "NON_PAGED_DEBUG_INFO",
            "NT_TIB32",
            "NT_TIB64",
        ],
    ),
    (
        "Windows.Win32.Graphics.Gdi",
        &[
            "EMRSETMITERLIMIT",
            "EMRPOLYTEXTOUTA",
            "EMREXTTEXTOUTA",
            "EMRANGLEARC",
            "ABCFLOAT",
            "XFORM",
            "EMRTRANSPARENTBLT",
            "EMRSTRETCHBLT",
            "EMRSETWORLDTRANSFORM",
            "EMRPLGBLT",
            "EMRMODIFYWORLDTRANSFORM",
            "EMRMASKBLT",
            "EMRBITBLT",
            "EMRALPHABLEND",
            "BITMAPFILEHEADER",
            "METAHEADER",
            "SetWorldTransform",
            "ModifyWorldTransform",
            "GetWorldTransform",
            "GetCharABCWidthsFloatW",
            "GetCharABCWidthsFloatA",
            "ExtCreateRegion",
            "CombineTransform",
            "AngleArc",
            "SetMiterLimit",
        ],
    ),
    ("Windows.Win32.Graphics.DirectDraw", &[]),
    (
        "Windows.Win32.UI.WindowsAndMessaging",
        &[
            "DLGITEMTEMPLATE",
            "DLGTEMPLATE",
            "CreateDialogIndirectParamA",
            "CreateDialogIndirectParamW",
            "DialogBoxIndirectParamA",
            "DialogBoxIndirectParamW",
        ],
    ),
];
const UNWINDABLE_FUNCTIONS: &[&str] = &["ExitThread", "ExitProcess"];
const CALLBACKING_FUNCTIONS: &[&str] = &["CreateWindowExA", "DispatchMessageA"];

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
        callbacking_functions: CALLBACKING_FUNCTIONS.iter().copied().collect(),
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
