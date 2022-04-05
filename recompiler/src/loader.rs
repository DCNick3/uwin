use memory_image::{MemoryImage, Protection};
use num::Integer;
use object::read::pe::ImageNtHeaders;
use object::{pe, LittleEndian, Object};
use std::cmp::max;

use crate::error::Result;
use crate::pe_file::{LoadedPeInfo, PeFile};

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

impl PeFile {
    pub fn load_into(
        &self,
        addr: u32,
        image: &mut MemoryImage,
        pe_name: &str,
    ) -> Result<LoadedPeInfo> {
        let pe = self.get();

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

        let mut max_rva = 0;

        let strings = pe.nt_headers().symbols(pe.data())?.strings();
        for section in pe.section_table().iter() {
            let name = section.name(strings)?;
            let name = std::str::from_utf8(name).expect("Non UTF-8 PE section name");

            let section_rva = section.virtual_address.get(LE);
            let addr = section_rva.wrapping_add(addr);
            let data = section.pe_data(pe.data())?;

            let size = align_up(section.virtual_size.get(LE) as u32, PAGE_ALIGNMENT);

            max_rva = max(max_rva, section_rva + size);

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
                .relocation_blocks(pe.data(), &pe.section_table())?
            {
                todo!("relocations")
            }
        }

        Ok(LoadedPeInfo::new(addr, max_rva))
    }
}
