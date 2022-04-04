extern crate core;

use clap::Parser;
use itertools::Itertools;
use lazy_static::lazy_static;
use memmap2::Mmap;
use memory_image::MemoryImage;
use object::read::pe::PeFile32;
use object::Object;
use recompiler::{make_dll_stub, LE};

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// (main) Executable file to recompile
    #[clap(short, long)]
    executable: PathBuf,

    /// Dll files to include into recompilation. Stubs will be generated automagically
    #[clap(short, long)]
    dlls: Vec<PathBuf>,
}

fn read_file(path: &Path) -> Mmap {
    let file = std::fs::File::open(path).expect("Cannot open file to read");
    // SAFETY: the file is (hopefully) not modified during the execution
    unsafe { Mmap::map(&file) }.expect("Cannot mmap file")
}

lazy_static! {
    static ref STUBBUABLE_DLLS: HashSet<&'static str> =
        HashSet::from(["kernel32.dll", "user32.dll"]);
}

// fn import_libraries(pe: &PeFile32) -> Vec<String> {
//     match pe.import_table().unwrap() {
//         None => Vec::new(),
//         Some(import_table) => {
//             let mut res = Vec::new();
//
//             let mut iter = import_table.descriptors().unwrap();
//             while let Some(desc) = iter.next().unwrap() {
//                 res.push(
//                     std::str::from_utf8(import_table.name(desc.name.get(LE)).unwrap())
//                         .unwrap()
//                         .to_string(),
//                 );
//             }
//
//             res
//         }
//     }
// }

fn main() {
    let args = Args::parse();

    let exe = read_file(&args.executable);
    let exe = PeFile32::parse(&*exe).expect("Parsing the exe file");

    let dlls = args
        .dlls
        .iter()
        .map(|f| {
            (
                f.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_ascii_lowercase(),
                read_file(f),
            )
        })
        .collect::<Vec<_>>();
    let mut dlls = dlls
        .iter()
        .map(|(f, mmap)| (f.clone(), PeFile32::parse(&**mmap).expect("Parsing a dll")))
        .collect::<BTreeMap<_, _>>();

    let mut required_dlls = BTreeMap::new();

    for (dll, group) in &dlls
        .values()
        .chain([&exe])
        .flat_map(|f| f.imports().unwrap())
        .map(|import| {
            (
                std::str::from_utf8(import.library())
                    .unwrap()
                    .to_ascii_lowercase(),
                std::str::from_utf8(import.name()).unwrap(),
            )
        })
        .sorted_by_key(|(dll, _)| dll.clone()) // clone here is just me vs the borrow checker
        .group_by(|(dll, _)| dll.clone())
    {
        let functions = group.map(|(_, name)| name).unique().collect::<Vec<_>>();
        required_dlls.insert(dll, functions);
    }

    let fn_indices = required_dlls
        .values()
        .flatten()
        .unique()
        .enumerate()
        .map(|(i, name)| (name.to_string(), i as u32))
        .collect::<BTreeMap<_, _>>();

    println!("All dep dlls collected: {:#?}", required_dlls);

    // let mut stub_storage = Vec::new();
    for (dll_name, fns) in required_dlls.iter() {
        if dlls.contains_key(dll_name.as_str()) {
            println!("FOUND {}", dll_name)
        } else if STUBBUABLE_DLLS.contains(dll_name.as_str()) {
            println!("STUB  {}", dll_name);
            let fns = fns
                .iter()
                .map(|&name| (name.to_string(), *fn_indices.get(name).unwrap()))
                .collect();

            let stub = make_dll_stub(&dll_name, &fns).unwrap();
            let stub = stub.leak() as &[u8]; // FIXME !!! ah, shit, lifetimes are hard
            let stub = PeFile32::parse(stub).expect("Parsing the stub");
            dlls.insert(dll_name.clone(), stub);
        } else {
            println!("WHERE {}", dll_name);
            panic!("Can't find dll with name {}. It was not provided as an input & is not included in stubbable allow-list", dll_name)
        }
    }

    let mut free_addr = exe.nt_headers().optional_header.image_base.get(LE);

    let mut memory = MemoryImage::new();

    let mut load_free = |pe: &PeFile32, name: &str| {
        println!("LOAD {}", name);
        // TODO: store it somewhere
        recompiler::load_into(&mut memory, free_addr, pe, name);

        free_addr += pe.nt_headers().optional_header.size_of_image.get(LE); // Do we want to rely on this? It might kinda lie...
    };

    load_free(&exe, args.executable.file_name().unwrap().to_str().unwrap());

    for (dll_name, _) in required_dlls.iter() {
        if let Some(dll) = dlls.get(dll_name) {
            load_free(dll, dll_name)
        }
    }

    // recompiler::load_into(
    //     &mut memory,
    //     free_addr,
    //     &exe,
    //     args.executable.file_name().unwrap().to_str().unwrap(),
    // );

    println!("{}", memory.map());

    // let stub = make_dll_stub(
    //     "kernel32.dll",
    //     &BTreeMap::from(
    //         [("GetLastError", 1), ("SetLastError", 2)].map(|(nm, idx)| (nm.to_string(), idx)),
    //     ),
    // )
    // .unwrap();

    // std::fs::write("kernel32.dll", &stub).unwrap();
}
