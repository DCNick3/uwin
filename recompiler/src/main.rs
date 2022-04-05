extern crate core;

use clap::Parser;
use itertools::Itertools;
use lazy_static::lazy_static;
use memory_image::MemoryImage;
use object::{LittleEndian, Object};
use recompiler::make_dll_stub;

use recompiler::PeFile;
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

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

pub const LE: LittleEndian = LittleEndian {};

lazy_static! {
    static ref STUBBUABLE_DLLS: HashSet<&'static str> =
        HashSet::from(["kernel32.dll", "user32.dll"]);
}

fn main() {
    let args = Args::parse();

    let exe = PeFile::parse_from_path(&args.executable).expect("Loading exe file");

    let dlls = args
        .dlls
        .iter()
        .map(|f| PeFile::parse_from_path(f))
        .collect::<Result<Vec<_>, _>>()
        .expect("Loading dlls");

    let mut dlls = dlls
        .into_iter()
        .map(|pe| (pe.name().to_string(), pe))
        .collect::<BTreeMap<_, _>>();

    let mut required_dlls = BTreeMap::new();

    for (dll, group) in &dlls
        .values()
        .chain([&exe])
        .flat_map(|f| f.get().imports().unwrap())
        .map(|import| {
            (
                std::str::from_utf8(import.library())
                    .expect("Library with non-ascii name")
                    .to_ascii_lowercase(),
                std::str::from_utf8(import.name()).expect("Imported function with non-ascii name"),
            )
        })
        .sorted_by_key(|(dll, _)| dll.clone()) // clone here is just me vs the borrow checker
        .group_by(|(dll, _)| dll.clone())
    {
        let functions = group
            .map(|(_, name)| name.to_string())
            .unique()
            .collect::<Vec<_>>();
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

    for (dll_name, fns) in required_dlls.iter() {
        if dlls.contains_key(dll_name.as_str()) {
            println!("FOUND {}", dll_name)
        } else if STUBBUABLE_DLLS.contains(dll_name.as_str()) {
            println!("STUB  {}", dll_name);
            let fns = fns
                .iter()
                .map(|name| (name.to_string(), *fn_indices.get(name).unwrap()))
                .collect();

            let stub = make_dll_stub(dll_name, &fns).unwrap();
            dlls.insert(dll_name.clone(), stub);
        } else {
            println!("WHERE {}", dll_name);
            panic!("Can't find dll with name {}. It was not provided as an input & is not included in stubbable allow-list", dll_name)
        }
    }

    let mut free_addr = exe.base_addr();

    let mut memory = MemoryImage::new();

    let mut load_free = |pe: &PeFile| {
        println!("LOAD {}", pe.name());
        // TODO: store it somewhere
        let info = pe
            .load_into(free_addr, &mut memory, pe.name())
            .expect("Loading a PE file");

        free_addr += info.image_size();
    };

    load_free(&exe);

    for (dll, _) in required_dlls.iter() {
        if let Some(dll) = dlls.get(dll) {
            load_free(dll)
        }
    }

    println!("{}", memory.map());
}
