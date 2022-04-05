use itertools::Itertools;
use lazy_static::lazy_static;
use memory_image::{MemoryImage, Protection};
use num::Integer;
use object::read::pe::ImageNtHeaders;
use object::{pe, LittleEndian, Object};
use std::cmp::max;
use std::collections::{BTreeMap, HashSet};

use crate::error::Result;
use crate::make_dll_stub;
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

// TODO: populate this list
lazy_static! {
    static ref STUBBUABLE_DLLS: HashSet<&'static str> =
        HashSet::from(["kernel32.dll", "user32.dll"]);
}

pub fn load_process_image(executable: PeFile, dlls: Vec<PeFile>) -> Result<MemoryImage> {
    let mut dlls = dlls
        .into_iter()
        .map(|pe| (pe.name().to_string(), pe))
        .collect::<BTreeMap<_, _>>();

    let mut required_dlls = BTreeMap::new();

    for (dll, group) in &dlls
        .values()
        .chain([&executable])
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

    // start from the base image of the executable.
    // the executable will be loaded at the requested address this way, as we load it first
    let mut free_addr = executable.base_addr();

    let mut memory = MemoryImage::new();

    let mut load_into_first_free = |pe: &PeFile| -> Result<_> {
        println!("LOAD {}", pe.name());
        // TODO: store it somewhere
        let info = pe.load_into(free_addr, &mut memory, pe.name())?;

        free_addr += info.image_size();

        Ok(())
    };

    load_into_first_free(&executable)?;

    for (dll, _) in required_dlls.iter() {
        if let Some(dll) = dlls.get(dll) {
            load_into_first_free(dll)?
        }
    }

    Ok(memory)
}
