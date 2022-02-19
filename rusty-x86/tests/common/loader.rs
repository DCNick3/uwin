use rusty_x86::memory_image::{MemoryImage, Protection};
use std::error::Error;

use goblin::elf::Elf;
use goblin::pe::section_table::{IMAGE_SCN_MEM_EXECUTE, IMAGE_SCN_MEM_READ, IMAGE_SCN_MEM_WRITE};
use goblin::pe::{options, PE};

pub fn load_elf(elf_data: &[u8]) -> Result<(u32, MemoryImage), Box<dyn Error>> {
    let elf = Elf::parse(elf_data)?;

    let mut res = MemoryImage::new();
    for section in elf.section_headers {
        if !section.is_alloc() {
            continue;
        }

        let mut prot = Protection::READ;
        if section.is_writable() {
            prot |= Protection::WRITE;
        }

        if section.is_executable() {
            prot |= Protection::EXECUTE;
        }

        let base = section.sh_addr as u32;
        let region = section.file_range().map_or(&[] as &[u8], |r| &elf_data[r]);
        res.add_region(base, prot, region.to_vec());
    }

    Ok((elf.entry as u32, res))
}

#[allow(unused)]
pub fn load_pe(pe_data: &[u8]) -> Result<(u32, MemoryImage), Box<dyn Error>> {
    let pe = PE::parse_with_opts(pe_data, &options::ParseOptions { resolve_rva: false })?;

    let image_base = pe.image_base as u32;

    let mut res = MemoryImage::new();
    for section in pe.sections {
        let mut prot = Protection::READ;
        if section.characteristics & IMAGE_SCN_MEM_READ != 0 {
            prot |= Protection::READ;
        }
        if section.characteristics & IMAGE_SCN_MEM_WRITE != 0 {
            prot |= Protection::WRITE;
        }
        if section.characteristics & IMAGE_SCN_MEM_EXECUTE != 0 {
            prot |= Protection::EXECUTE;
        }

        let base = section.virtual_address as u32;
        let ptr = section.pointer_to_raw_data as usize;
        let data_size = section.size_of_raw_data as usize;
        let virt_size = section.virtual_size as usize;
        let region = if ptr != 0 {
            &pe_data[ptr..][..data_size]
        } else {
            &[] as &[u8]
        };

        let mut region = region.to_vec();
        if virt_size > region.len() {
            region.extend(vec![0u8; virt_size - region.len()]);
        }

        res.add_region(image_base + base, prot, region);
    }

    Ok((image_base + pe.entry as u32, res))
}
