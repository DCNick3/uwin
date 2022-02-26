use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};
use uwin_win32_bindgen::{gen_type, Gen};

fn reformat(text: impl std::fmt::Display) -> Result<String, Box<dyn Error>> {
    let mut rustfmt = Command::new("rustfmt")
        .arg("--emit=stdout")
        .arg("--edition=2021")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    write!(rustfmt.stdin.take().unwrap(), "{}", text)?;
    let output = rustfmt.wait_with_output()?;
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
}

fn main() {
    let types = &[
        "Windows.Win32.Foundation.BOOL",
        //"Windows.Win32.Foundation.BSTR",
        "Windows.Win32.Foundation.CLASS_E_CLASSNOTAVAILABLE",
        //"Windows.Win32.Foundation.CloseHandle",
        "Windows.Win32.Foundation.CO_E_NOTINITIALIZED",
        "Windows.Win32.Foundation.E_NOINTERFACE",
        "Windows.Win32.Foundation.E_OUTOFMEMORY",
        "Windows.Win32.Foundation.FARPROC",
        // "Windows.Win32.Foundation.GetLastError",
        "Windows.Win32.Foundation.HANDLE",
        "Windows.Win32.Foundation.HINSTANCE",
        "Windows.Win32.Foundation.S_OK",
        // "Windows.Win32.Foundation.SysAllocStringLen",
        // "Windows.Win32.Foundation.SysFreeString",
        // "Windows.Win32.Foundation.SysStringLen",
        "Windows.Win32.Foundation.WIN32_ERROR",
        "Windows.Win32.Security.SECURITY_ATTRIBUTES",
        "Windows.Win32.System.Diagnostics.Debug.FORMAT_MESSAGE_OPTIONS",
        // "Windows.Win32.System.Diagnostics.Debug.FormatMessageW",
        // "Windows.Win32.System.LibraryLoader.FreeLibrary",
        // "Windows.Win32.System.LibraryLoader.GetProcAddress",
        // "Windows.Win32.System.LibraryLoader.LoadLibraryA",
        // "Windows.Win32.System.Memory.GetProcessHeap",
        "Windows.Win32.System.Memory.HEAP_FLAGS",
        // "Windows.Win32.System.Memory.HeapAlloc",
        // "Windows.Win32.System.Memory.HeapFree",
        // "Windows.Win32.System.Memory.HeapHandle",
        // "Windows.Win32.System.Threading.CreateEventA",
        // "Windows.Win32.System.Threading.SetEvent",
        // "Windows.Win32.System.Threading.WaitForSingleObject",
    ];

    // stuff to change:
    // - namespace (there are a lot of refs to ::windows::code)
    // - fix generation of code with different namespaces (it should actually work!)
    // - plain old pointers (also need a wrapper type, though this one is implemented)
    // - generate conversion trait implementations

    // questions to answer:
    // - representation of function pointers
    // - representation of COM interface implementations (extra hard!)

    let mut tokens = "#![allow(non_snake_case, non_upper_case_globals, dead_code, non_camel_case_types, clippy::upper_case_acronyms, clippy::derivable_impls)]".to_string();

    let gen = Gen {
        namespace: "Windows.",
        min_enum: true,
        min_inherit: true,
        flatten: true,
        ..Default::default()
    };

    for name in types {
        tokens += &gen_type(name, &gen);
    }

    let tokens = reformat(tokens).unwrap();

    println!("{}", tokens);
}
