use clap::{Args, Parser, Subcommand};
use recompiler::{find_basic_blocks, load_process_image, LoadedProcessImage};
use std::fs::File;

use itertools::Itertools;
use recompiler::PeFile;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Loads the executable & it's DLLs into an address space and spits out serialized process image
    Load(Load),

    /// Inspects serialized process image and dumps some info
    Dump(Dump),
}

#[derive(Args, Debug)]
struct Load {
    /// (main) Executable file to load
    executable: PathBuf,

    /// Path to output
    output: PathBuf,

    /// Dll files to load into the process image. Stubs will be generated automagically, so no need for system libs here
    #[clap(short, long)]
    dlls: Vec<PathBuf>,
}

#[derive(Args, Debug)]
struct Dump {
    /// Path to process image file
    process_image: PathBuf,
}

fn load(args: Load) {
    let exe = PeFile::parse_from_path(&args.executable).expect("Loading exe file");

    let dlls = args
        .dlls
        .iter()
        .map(|f| PeFile::parse_from_path(f))
        .collect::<Result<Vec<_>, _>>()
        .expect("Loading dlls");

    let image = load_process_image(exe, dlls).expect("Loading process image");

    println!("Writing to {:?}", args.output);

    let mut out = File::create(&args.output).expect("Open the output file");
    rmp_serde::encode::write(&mut out, &image).expect("Serializing process image to output file");
}

fn dump(args: Dump) {
    let file = File::open(&args.process_image).expect("Open the process image file");
    let image: LoadedProcessImage =
        rmp_serde::decode::from_read(file).expect("Deserializing process image");

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

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Load(args) => load(args),
        Commands::Dump(args) => dump(args),
    }
}
