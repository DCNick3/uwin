use recompiler::inkwell::context::Context;
use recompiler::{load_process_image, recompile_image, PeFile};
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let executable = manifest_dir
        .join("..")
        .join("test_exes/msvc/hello_world.exe")
        // .join("test_exes/test.exe")
        // .join("test_exes/test_cp1251.exe")
        // ----
        ;

    let exe = PeFile::parse_from_path(&executable).expect("Loading exe file");

    let image = load_process_image(exe, Vec::new()).expect("Loading process image");

    let ctx = Context::create();

    let module = recompile_image(&ctx, &image).expect("Recompilation failed");

    module.verify().expect("Module validation failed");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bitcode_path = out_dir.join("uwin_recomp.bc");

    assert!(
        module.write_bitcode_to_path(&bitcode_path),
        "Could not write the bitcode file"
    );

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
    println!("cargo:rustc-link-lib=static=uwin_recomp");
}
