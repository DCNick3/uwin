use rayon::prelude::*;
use recompiler::inkwell::context::Context;
use recompiler::{load_process_image, recompile_image, PeFile};
use rustflags::Flag;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Once;

fn codegen_modules(bitcode_dir: &Path, modules: usize) -> Vec<PathBuf> {
    // let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // let program_dir = manifest_dir.join("..").join("test_exes/msvc");
    let program_dir = PathBuf::from("/home/dcnick3/trash/homm3-switch/media/orig-dist/files/");

    let (exe_name, dll_names): (_, Vec<PathBuf>) = (
        PathBuf::from(
            // "simple_dll_user.exe"
            // "hello_world.exe"
            // "indirect.exe"
            // "hello_console.exe"
            // "cf1031B.exe"
            // "weird_repro.exe"
            // "window_init.exe",
            // "dd_init.exe",
            // "dd_image.exe",
            // "zip_basic.exe",
            "HEROES3.EXE",
        ),
        vec![
            // PathBuf::from("simple_dll.dll")
            PathBuf::from("MSS32.DLL"),
            PathBuf::from("MP3DEC.ASI"),
            PathBuf::from("SMACKW32.DLL"),
            PathBuf::from("BINKW32.DLL"),
            PathBuf::from("IFC20.dll"),
            PathBuf::from("/home/dcnick3/trash/homm3-switch/code/wsock32/wsock32.dll"),
        ],
    );

    let exe_name = program_dir.join(exe_name);

    let dll_names = dll_names
        .into_iter()
        .map(|p| program_dir.join(p))
        .collect::<Vec<_>>();

    // let executable = manifest_dir
    //     .join("..")
    //     // .join("test_exes/msvc/hello_world.exe")
    //     // .join("test_exes/msvc/indirect.exe")
    //     // .join("test_exes/msvc/hello_console.exe")
    //     // .join("test_exes/msvc/cf1031B.exe")
    //     // .join("test_exes/msvc/weird_repro.exe")
    //     // .join("test_exes/test.exe")
    //     // .join("test_exes/test_cp1251.exe")
    //     // ----
    //     ;

    let exe = PeFile::parse_from_path(&exe_name).expect("Loading exe file");
    let dlls = dll_names
        .iter()
        .map(|dll| PeFile::parse_from_path(&dll).expect("Loading dll file"))
        .collect::<Vec<_>>();

    let image = load_process_image(exe, dlls).expect("Loading process image");

    let ctx = Context::create();

    let modules = recompile_image(&ctx, &image, modules).expect("Recompilation failed");

    let mut module_paths = Vec::new();

    for (i, module) in modules.into_iter().enumerate() {
        module.verify().expect("Module validation failed");

        let module_path = bitcode_dir.with_file_name(format!("recomp-{}.bc", i));

        assert!(
            module.write_bitcode_to_path(&module_path),
            "Could not write the bitcode file"
        );

        module_paths.push(module_path);
    }

    println!("cargo:rerun-if-changed={}", exe_name.to_str().unwrap());

    for dll in &dll_names {
        println!("cargo:rerun-if-changed={}", dll.to_str().unwrap());
    }

    module_paths
}

fn compile_module(bitcode: &Path, lto: bool) -> PathBuf {
    let object_path = bitcode.with_extension("o");

    // don't do this, prefer to use clang APIs to compile the thing
    let mut command = Command::new("clang");
    command.arg(bitcode).arg("-O1").arg("-g3");

    if lto {
        command.arg("-flto=thin");
    }

    command.arg("-c").arg("-o").arg(&object_path);

    let res = command.status().expect("Compiling the bitcode with clang");

    assert!(
        res.success(),
        "Compiling the bitcode with clang: clang returned unsuccessful error code"
    );

    object_path
}

fn get_parallelism() -> usize {
    env::var("NUM_JOBS")
        .ok()
        .and_then(|amt| amt.parse().ok())
        .unwrap_or(4)
}

/// Returns a suitable `jobserver::Client` used to coordinate
/// parallelism between build scripts.
fn jobserver() -> &'static jobserver::Client {
    static INIT: Once = Once::new();
    static mut JOBSERVER: Option<jobserver::Client> = None;

    fn _assert_sync<T: Sync>() {}
    _assert_sync::<jobserver::Client>();

    unsafe {
        INIT.call_once(|| {
            let server = default_jobserver();
            JOBSERVER = Some(server);
        });
        JOBSERVER.as_ref().unwrap()
    }
}

unsafe fn default_jobserver() -> jobserver::Client {
    // Try to use the environmental jobserver which Cargo typically
    // initializes for us...
    if let Some(client) = jobserver::Client::from_env() {
        return client;
    }

    let parallelism = get_parallelism();

    // If we create our own jobserver then be sure to reserve one token
    // for ourselves.
    let client = jobserver::Client::new(parallelism).expect("failed to create jobserver");
    client.acquire_raw().expect("failed to acquire initial");
    return client;
}

fn compile_modules(bitcodes: &[PathBuf], lto: bool) -> Vec<PathBuf> {
    bitcodes
        .par_iter()
        .map(|bitcode| {
            let _token = jobserver().acquire().unwrap();
            compile_module(bitcode, lto)
        })
        .collect()
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let num_cpus = num_cpus::get();

    let bitcode_dir = out_dir.join("uwin_recomp");
    let module_paths = codegen_modules(&bitcode_dir, num_cpus);

    let mut enable_lto = false;

    rustflags::from_env().for_each(|f| match &f {
        Flag::Codegen { opt, .. } if opt == "linker-plugin-lto" => enable_lto = true,
        _ => {}
    });

    if enable_lto {
        println!("cargo:warning={}", "Generating objects with thin LTO enabled! This may cause issues if plugin-based LTO is not enabled in the linker.");
    }

    let objects = compile_modules(module_paths.as_slice(), enable_lto);

    let archive_path = out_dir.join("libuwin_recomp.a");

    let res = Command::new("ar")
        .arg("crus")
        .arg(&archive_path)
        .args(objects)
        .status()
        .unwrap();

    assert!(res.success(), "Creating archive failed");

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.to_str().unwrap()
    );
    // see whole-archive here: https://github.com/rust-lang/rust/pull/93901#issuecomment-1041325522
    // it basically means that we would use the full lib, ignoring whether it is used anywhere or not
    // this helps us, because we reference some symbols from rusty_x86_runtime, but link them here, and analysis fails
    // I think, when `bundle` modified will be stabilized (TODO)
    println!("cargo:rustc-link-lib=static:-bundle,+whole-archive=uwin_recomp");
}
