use clap::Parser;
use memmap2::Mmap;
use memory_image::{MemoryImage, Protection};
use num::Integer;
use object::read::pe::{ImageNtHeaders, PeFile32};
use object::{pe, LittleEndian, Object};
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

const LE: LittleEndian = LittleEndian {};

const PAGE_ALIGNMENT: u32 = 0x1000;

fn align_up<T: Integer + Copy>(mut value: T, alignment: T) -> T {
    while value % alignment != T::zero() {
        value = value + T::one();
    }
    value
}

fn characteristics_to_prot(characteristics: u32) -> Protection {
    let mut prot = Protection::empty();

    if characteristics & pe::IMAGE_SCN_MEM_READ != 0 {
        prot |= Protection::READ;
    }
    if characteristics & pe::IMAGE_SCN_MEM_WRITE != 0 {
        prot |= Protection::WRITE;
    }
    if characteristics & pe::IMAGE_SCN_MEM_EXECUTE != 0 {
        prot |= Protection::EXECUTE;
    }

    prot
}

fn load_into(image: &mut MemoryImage, addr: u32, pe: &PeFile32, pe_name: &str) {
    let headers_size = pe.nt_headers().optional_header.size_of_headers.get(LE) as usize;
    let headers_mem_size = align_up(headers_size, PAGE_ALIGNMENT as usize);

    // first of all - map the headers
    {
        let mut headers = Vec::from(&pe.data()[..headers_size]);
        headers.resize(headers_mem_size, 0);
        image.add_region(
            addr,
            Protection::READ,
            headers,
            format!("{}:headers", pe_name),
        )
    }

    let image_base_diff = addr.wrapping_sub(pe.relative_address_base() as u32);

    let strings = pe.nt_headers().symbols(pe.data()).unwrap().strings();
    for section in pe.section_table().iter() {
        let name = section.name(strings).unwrap();
        let name = std::str::from_utf8(name).expect("Non UTF-8 PE section name");

        let addr = section.virtual_address.get(LE).wrapping_add(addr);
        let data = section.pe_data(pe.data()).unwrap();

        let size = align_up(section.virtual_size.get(LE) as u32, PAGE_ALIGNMENT);

        let mut data = Vec::from(data);
        data.resize(size as usize, 0);

        image.add_region(
            addr,
            characteristics_to_prot(section.characteristics.get(LE)),
            data,
            format!("{}:{}", pe_name, name),
        );
    }

    // no relocations for now
    assert_eq!(image_base_diff, 0);
}

fn main() {
    let args = Args::parse();

    let exe = read_file(&args.executable);
    let exe = PeFile32::parse(&*exe).expect("Parsing the exe file");

    let mut image = MemoryImage::new();
    load_into(
        &mut image,
        exe.nt_headers().optional_header.image_base.get(LE),
        &exe,
        args.executable.file_name().unwrap().to_str().unwrap(),
    );

    println!("{}", image.map())
}
