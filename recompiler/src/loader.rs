use itertools::Itertools;
use lazy_static::lazy_static;
use memory_image::{MemoryImage, Protection};
use num::Integer;
use object::pe::{ImageNtHeaders32, IMAGE_REL_BASED_HIGHLOW};
use object::read::pe::{ExportTarget, ImageNtHeaders, Import};
use object::{pe, LittleEndian, Object};
use serde::{Deserialize, Serialize};
use std::cmp::max;
use std::collections::{BTreeMap, HashSet};
use std::io::Write;
use std::sync::Arc;

use crate::error::{Error, Result};
use crate::make_dll_stub;
use crate::pe_file::{LoadedPeInfo, PeFile, ProcessImageSymbol};

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
            // handle relocations
            if let Some(mut reloc) = pe
                .data_directories()
                .relocation_blocks(pe.data(), &pe.section_table())?
            {
                while let Some(reloc) = reloc.next()? {
                    for reloc in reloc {
                        let addr = addr + reloc.virtual_address;
                        if reloc.typ == IMAGE_REL_BASED_HIGHLOW {
                            let val = u32::from_le_bytes(
                                (&image.read_all_at(addr)[..4]).try_into().unwrap(),
                            );
                            let new_val = val.wrapping_add(image_base_diff);
                            image.modify_all_at(addr)[..4].copy_from_slice(&new_val.to_le_bytes());
                        } else {
                            todo!("Relocation type {:#06x}", reloc.typ)
                        }
                    }
                }
            }
        }

        Ok(LoadedPeInfo {
            base_addr: addr,
            image_size: max_rva,
        })
    }
}

// TODO: populate this list
lazy_static! {
    static ref STUBBUABLE_DLLS: HashSet<&'static str> =
        HashSet::from(["kernel32.dll", "user32.dll", "ddraw.dll"]);
}

#[derive(Serialize, Deserialize)]
pub struct LoadedProcessImage {
    pub memory: MemoryImage,
    pub modules: BTreeMap<String, (PeFile, LoadedPeInfo)>,
    pub symbols: BTreeMap<u32, ProcessImageSymbol>,
    pub thunk_functions: BTreeMap<u32, String>,
    pub exe_entrypoint: u32,
    pub main_module_base: u32,
    pub main_module_name: String,
}

fn bind_imports(
    memory: &mut MemoryImage,
    (pe, info): &(PeFile, LoadedPeInfo),
    exports: &BTreeMap<String, BTreeMap<String, u32>>,
) -> Result<Vec<(String, String)>> {
    let mut missing_imports = Vec::new();

    if let Some(import_table) = pe
        .get()
        .data_directories()
        .import_table(pe.get().data(), &pe.get().section_table())?
    {
        let mut it = import_table.descriptors()?;
        while let Some(desc) = it.next()? {
            // TODO: write datetime into the descriptor or smth
            let dll_name = std::str::from_utf8(import_table.name(desc.name.get(LE))?)
                .unwrap()
                .to_ascii_lowercase();
            let exports = exports.get(&dll_name).unwrap();

            let mut thunk_addr = desc.first_thunk.get(LE);
            let mut thunks = import_table.thunks(thunk_addr)?;

            while let Some(thunk) = thunks.next::<ImageNtHeaders32>()? {
                let import = import_table.import::<ImageNtHeaders32>(thunk)?;

                match import {
                    Import::Ordinal(_) => todo!("Ordinal imports"),
                    Import::Name(_, name) => {
                        let name = std::str::from_utf8(name).unwrap();

                        if let Some(&addr) = exports.get(name) {
                            let mut target =
                                &mut memory.modify_all_at(info.base_addr + thunk_addr)[..4];
                            target.write_all(&addr.to_le_bytes()).unwrap();
                        } else {
                            missing_imports.push((dll_name.to_string(), name.to_string()))
                        }
                    }
                }

                thunk_addr += 4;
            }
        }
    }

    Ok(missing_imports)
}

fn build_exports_index(
    modules: &BTreeMap<String, (PeFile, LoadedPeInfo)>,
) -> Result<BTreeMap<String, BTreeMap<String, u32>>> {
    let exports = modules
        .iter()
        .map(|(dll_name, (pe, info))| -> Result<_> {
            Ok(
                match pe
                    .get()
                    .data_directories()
                    .export_table(pe.get().data(), &pe.get().section_table())?
                {
                    Some(table) => Some((
                        dll_name.to_string(),
                        table
                            .exports()?
                            .into_iter()
                            .filter_map(|export| match export.target {
                                ExportTarget::Address(addr) => Some((
                                    std::str::from_utf8(export.name?).unwrap().to_string(),
                                    info.base_addr + addr,
                                )),
                                ExportTarget::ForwardByOrdinal(_, _)
                                | ExportTarget::ForwardByName(_, _) => todo!("Forwarding exports"),
                            })
                            .collect::<BTreeMap<_, _>>(),
                    )),
                    None => None,
                },
            )
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(exports.into_iter().flatten().collect())
}

const IMAGE_ALIGN: u32 = 0x10000;

pub fn load_process_image(executable: PeFile, dlls: Vec<PeFile>) -> Result<LoadedProcessImage> {
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

    let thunk_function_indices = required_dlls
        .iter()
        .filter(|(nm, _)| !dlls.contains_key(nm.as_str()))
        .flat_map(|(_, vals)| vals)
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
                .map(|name| (name.to_string(), *thunk_function_indices.get(name).unwrap()))
                .collect();

            let stub = make_dll_stub(dll_name, &fns).unwrap();
            dlls.insert(dll_name.clone(), stub);
        } else {
            println!("WHERE {}", dll_name);
            panic!("Can't find dll with name {}. It was not provided as an input & is not included in stubbable allow-list", dll_name)
        }
    }

    let thunk_functions = thunk_function_indices
        .into_iter()
        .map(|(name, idx)| (idx, name))
        .collect::<BTreeMap<_, _>>();

    let exe_entrypoint = executable.entry() + executable.base_addr();
    let exe_base_addr = executable.base_addr();
    let exe_name = executable.name().to_string();

    // start from the base image of the executable.
    // the executable will be loaded at the requested address this way, as we load it first
    let mut free_addr = executable.base_addr();
    let mut memory = MemoryImage::new();
    let mut modules = BTreeMap::new();

    let mut load_into_first_free = |pe: PeFile| -> Result<_> {
        println!("LOAD {}", pe.name());
        let info = pe.load_into(free_addr, &mut memory, pe.name())?;

        free_addr += info.image_size;
        // Image bases will be reserved as a memory region, so this alignment is required
        if free_addr % IMAGE_ALIGN != 0 {
            free_addr += IMAGE_ALIGN - free_addr % IMAGE_ALIGN;
        }
        modules.insert(pe.name().to_string(), (pe, info));

        Ok(())
    };

    load_into_first_free(executable)?;

    for (dll, _) in required_dlls.iter() {
        let dll = dlls.remove(dll).unwrap();
        load_into_first_free(dll)?;
    }

    let exports = build_exports_index(&modules)?;

    let mut missing_imports = Vec::new();
    for (_, module) in modules.iter() {
        missing_imports.append(&mut bind_imports(&mut memory, module, &exports)?);
    }
    if !missing_imports.is_empty() {
        return Err(Error::DllExportsNotFound(missing_imports));
    }

    let symbols = modules
        .values()
        .map(|(pe, info)| (pe, info, pe.name().into()))
        .flat_map(|(pe, info, name): (&PeFile, &LoadedPeInfo, Arc<str>)| {
            if let Some(sym) = pe.collect_symbols() {
                sym.into_iter()
                    .map(|(addr, nm)| {
                        (
                            addr + info.base_addr,
                            ProcessImageSymbol {
                                module: name.clone(),
                                symbol: nm.name,
                                kind: nm.kind,
                            },
                        )
                    })
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        })
        .collect::<BTreeMap<_, _>>();

    Ok(LoadedProcessImage {
        memory,
        modules,
        symbols,
        thunk_functions,
        exe_entrypoint,
        main_module_base: exe_base_addr,
        main_module_name: exe_name,
    })
}
