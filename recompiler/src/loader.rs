use memmap2::Mmap;
use memory_image::{MemoryImage, Protection};
use num::Integer;
use object::read::pe::{ImageNtHeaders, PeFile32};
use object::{pe, LittleEndian, Object};
use stable_deref_trait::StableDeref;
use std::fs::File;
use std::io::ErrorKind;
use std::ops::Deref;
use std::path::Path;
use yoke::either::EitherCart;
use yoke::{Yoke, Yokeable};

use crate::error::{Error, Result};

pub const LE: LittleEndian = LittleEndian {};

pub const PAGE_ALIGNMENT: u32 = 0x1000;

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

pub fn load_into(image: &mut MemoryImage, addr: u32, pe: &PeFile32, pe_name: &str) {
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
            format!("{:>20}: headers", pe_name),
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
            format!("{:>20}:{}", pe_name, name),
        );
    }

    // no relocations for now
    if image_base_diff != 0 {
        // "handle" relocations
        if let Some(_reloc) = pe
            .data_directories()
            .relocation_blocks(pe.data(), &pe.section_table())
            .unwrap()
        {
            todo!("relocations")
        }
    }
}

struct PeFileWrapper<'a>(pub PeFile32<'a>);

unsafe impl<'a> Yokeable<'a> for PeFileWrapper<'static> {
    type Output = PeFileWrapper<'a>;

    fn transform(&'a self) -> &'a Self::Output {
        self
    }

    fn transform_owned(self) -> Self::Output {
        self
    }

    // pizdetc
    unsafe fn make(from: Self::Output) -> Self {
        // SAFETY: спаси и сохрани

        // We're just doing mem::transmute() here, however Rust is
        // not smart enough to realize that Bar<'a> and Bar<'static> are of
        // the same size, so instead we use transmute_copy

        // This assert will be optimized out, but is included for additional
        // peace of mind as we are using transmute_copy
        debug_assert!(std::mem::size_of::<Self::Output>() == std::mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        std::mem::forget(from);
        std::ptr::read(ptr)
    }

    fn transform_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        // SAFETY: спаси и сохрани
        unsafe { f(std::mem::transmute::<&mut Self, &mut Self::Output>(self)) }
    }
}

impl<'a> Deref for PeFileWrapper<'a> {
    type Target = PeFile32<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

type PeFileYoke = Yoke<PeFileWrapper<'static>, EitherCart<Mmap, Vec<u8>>>;

pub struct PeFile {
    yoke: PeFileYoke,
    name: String,
}

impl PeFile {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let file = std::fs::File::open(path).map_err(Error::FileOpen)?;
        Self::load_from_file(
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_ascii_lowercase(),
            &file,
        )
    }

    pub fn load_from_file(name: String, file: &File) -> Result<Self> {
        // SAFETY: the file is (hopefully) not modified during the execution
        let mmap = unsafe { Mmap::map(file) }.map_err(Error::FileMmap)?;
        let yoke = Yoke::try_attach_to_cart_badly::<object::Error>(mmap, |c| {
            Ok(PeFileWrapper(PeFile32::parse(c)?))
        })?;
        let yoke = yoke.wrap_cart_in_either_a();
        Ok(PeFile { yoke, name })
    }

    pub fn load_from_memory(name: String, data: Vec<u8>) -> Result<Self> {
        let yoke = Yoke::try_attach_to_cart_badly::<object::Error>(data, |c| {
            Ok(PeFileWrapper(PeFile32::parse(c)?))
        })?;
        let yoke = yoke.wrap_cart_in_either_b();
        Ok(PeFile { yoke, name })
    }

    pub fn get(&self) -> &PeFile32 {
        self.yoke.get()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
