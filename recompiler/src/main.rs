extern crate core;

use clap::Parser;
use object::LittleEndian;
use recompiler::load_process_image;

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

pub const LE: LittleEndian = LittleEndian {};

fn main() {
    let args = Args::parse();

    let exe = PeFile::parse_from_path(&args.executable).expect("Loading exe file");

    let dlls = args
        .dlls
        .iter()
        .map(|f| PeFile::parse_from_path(f))
        .collect::<Result<Vec<_>, _>>()
        .expect("Loading dlls");

    let memory = load_process_image(exe, dlls).expect("Loading process image");

    println!("{}", memory.map());
}
