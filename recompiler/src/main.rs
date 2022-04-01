use clap::Parser;
use memmap2::Mmap;
use object::read::pe::PeFile32;
use object::{NativeEndian, Object, ObjectSection};
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
    let file = unsafe { Mmap::map(&file) }.expect("Cannot mmap file");
    file
}

fn main() {
    let args = Args::parse();

    let exe = read_file(&args.executable);
    let exe = PeFile32::parse(&*exe).expect("Parsing the exe file");

    let endian = NativeEndian::default();

    println!(
        "Size of headers = {:#010x}",
        exe.nt_headers().optional_header.size_of_headers.get(endian)
    );

    for section in exe.sections() {
        println!(
            "{:15} {:#010x} {:#010x}",
            section.name().unwrap(),
            section.address(),
            section.size()
        )
    }
}
