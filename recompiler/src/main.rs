use clap::Parser;
use recompiler::{find_basic_blocks, load_process_image};

use itertools::Itertools;
use recompiler::PeFile;
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

fn main() {
    let args = Args::parse();

    let exe = PeFile::parse_from_path(&args.executable).expect("Loading exe file");

    let dlls = args
        .dlls
        .iter()
        .map(|f| PeFile::parse_from_path(f))
        .collect::<Result<Vec<_>, _>>()
        .expect("Loading dlls");

    let image = load_process_image(exe, dlls).expect("Loading process image");

    println!();
    println!("Symbols:");
    for (&addr, sym) in image.symbols.iter().sorted_by_key(|(&addr, _)| addr) {
        println!(
            "{:#010x} {:?} {:>10}!{}",
            addr, sym.kind, sym.module, sym.symbol
        )
    }

    println!();
    println!("Memory map:");

    println!("{}", image.memory.map());

    println!();
    println!("Basic blocks:");
    for addr in find_basic_blocks(&image).into_iter().sorted_by_key(|f| *f) {
        println!("  {:#010x}", addr)
    }

    println!();
    println!("Memory dump:");
    println!("{}", image.memory.dump());
}
