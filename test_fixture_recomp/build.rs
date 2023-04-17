use recompiler::inkwell::context::Context;
use recompiler::{load_process_image, recompile_image, PeFile};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn compile_module(bitcode_path: &Path) {
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
        ],
    );

    let executable = program_dir.join(exe_name);

    let dlls = dll_names.into_iter().map(|p| program_dir.join(p));

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

    let exe = PeFile::parse_from_path(&executable).expect("Loading exe file");
    let dlls = dlls
        .map(|dll| PeFile::parse_from_path(&dll).expect("Loading dll file"))
        .collect::<Vec<_>>();

    let image = load_process_image(exe, dlls).expect("Loading process image");

    let ctx = Context::create();

    let module = recompile_image(&ctx, &image).expect("Recompilation failed");

    module.verify().expect("Module validation failed");

    assert!(
        module.write_bitcode_to_path(&bitcode_path),
        "Could not write the bitcode file"
    );

    println!("cargo:rerun-if-changed={}", executable.to_str().unwrap());
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bitcode_path = out_dir.join("uwin_recomp.bc");
    compile_module(&bitcode_path);

    let object_path = out_dir.join("uwin_recomp.o");

    // don't do this, prefer to use clang APIs to compile the thing
    let res = Command::new("clang")
        .arg(bitcode_path)
        .arg("-O3")
        .arg("-g3")
        .arg("-c")
        .arg("-o")
        .arg(&object_path)
        .status()
        .expect("Compiling the bitcode with clang");

    assert!(
        res.success(),
        "Compiling the bitcode with clang: clang returned unsuccessful error code"
    );

    let archive_path = out_dir.join("libuwin_recomp.a");

    let res = Command::new("ar")
        .arg("crus")
        .arg(&archive_path)
        .arg(&object_path)
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
