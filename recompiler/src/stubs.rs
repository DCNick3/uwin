//! This module generates stub dlls that contain thunks to native functions
//!
//! The generated DLL contains only two sections: .text and .edata
//!
//! The text section contains a series of thunks (one for each win32 function used),
//!  which are basically x86 far jumps with the 0x7775 segment and address being the thunk index.
//!
//! Those jumps are treated specially by rusty-x86 and calls to runtime are issued directly.
//!
//! Currently it consists of mostly manual generation (because the object crate does not provide that much abstractions)

use crate::error::Result;
use crate::loader::PAGE_ALIGNMENT;
use crate::pe_file::{PeFile, PeSymbol, SymbolKind};
use crate::thunk_id_allocator::ThunkIdAllocator;
use crate::OwnedImport;
use object::pe;
use object::write::pe::{NtHeaders, SectionRange, Writer};
use std::collections::{BTreeMap, BTreeSet, HashMap};

fn gen_text_section(
    thunk_id_allocator: &mut ThunkIdAllocator,
    functions: &BTreeSet<&str>,
) -> (Vec<u8>, BTreeMap<String, u32>) {
    let mut output = Vec::new();
    let mut exports = BTreeMap::new();

    for name in functions {
        let name = name.as_ref();
        let index = thunk_id_allocator.get_plain_function(name);

        exports.insert(name.to_string(), output.len() as u32);
        // FAR JUMP opcode
        output.push(0xea);

        // far call offset
        output.extend(index.to_le_bytes());
        // far call segment (magic number for UW)
        output.extend(0x7775u16.to_le_bytes());
    }

    (output, exports)
}

type StringTable = (Vec<u8>, HashMap<String, u32>);

fn gen_edata_string_table(name: &str, text_exports: &BTreeMap<String, u32>) -> StringTable {
    let mut res = Vec::new();
    let mut positions = HashMap::new();

    let mut put_str = |s: &str| {
        if !positions.contains_key(s) {
            let pos = res.len() as u32;
            positions.insert(s.to_string(), pos);
            res.extend(s.bytes());
            res.push(0);
        }
    };

    put_str(name);
    for name in text_exports.keys() {
        put_str(name)
    }

    (res, positions)
}

fn edata_len(_name: &str, text_labels: &BTreeMap<String, u32>, string_table: &StringTable) -> u32 {
    let res = 10 * 4 /* header */
        + text_labels.len()
            * (4 + /* Export Address Table */
               4 + /* Export Name Pointer Table */
               2/* Export Ordinal Table */)
        + string_table.0.len();

    res as u32
}

fn gen_edata(
    name: &str,
    text_exports: &BTreeMap<String, u32>,
    string_table: &StringTable,
    edata_range: SectionRange,
    text_range: SectionRange,
) -> Vec<u8> {
    let mut output = Vec::new();

    let function_count = text_exports.len() as u32;
    let export_address_table_rva = edata_range.virtual_address + 10 * 4 /* header */;
    let export_name_pointer_table_rva = export_address_table_rva + 4 * text_exports.len() as u32;
    let export_ordinal_table = export_name_pointer_table_rva + 4 * text_exports.len() as u32;
    let string_table_rva = export_ordinal_table + 2 * text_exports.len() as u32;

    // flags
    output.extend(0u32.to_le_bytes());
    // date time
    output.extend(0u32.to_le_bytes());
    // Major Version
    output.extend(0u16.to_le_bytes());
    // Minor Version
    output.extend(0u16.to_le_bytes());
    // Name RVA
    output.extend((string_table_rva + string_table.1.get(name).unwrap()).to_le_bytes());
    // Ordinal Base
    output.extend(1u32.to_le_bytes());
    // Address Table Entries
    output.extend(function_count.to_le_bytes());
    // Number of Name Pointers
    output.extend(function_count.to_le_bytes());
    // Export Address Table RVA
    output.extend(export_address_table_rva.to_le_bytes());
    // Name Pointer RVA
    output.extend(export_name_pointer_table_rva.to_le_bytes());
    // Ordinal Table RVA
    output.extend(export_ordinal_table.to_le_bytes());

    assert_eq!(
        edata_range.virtual_address + output.len() as u32,
        export_address_table_rva
    );
    // Export Address Table
    for offset in text_exports.values() {
        let rva = text_range.virtual_address + offset;
        output.extend(rva.to_le_bytes());
    }

    assert_eq!(
        edata_range.virtual_address + output.len() as u32,
        export_name_pointer_table_rva
    );
    // Export Name Pointer Table
    for name in text_exports.keys() {
        let name_offset = string_table.1.get(name).unwrap();
        let name_rva = string_table_rva + name_offset;
        output.extend(name_rva.to_le_bytes());
    }

    assert_eq!(
        edata_range.virtual_address + output.len() as u32,
        export_ordinal_table
    );
    // Export Ordinal Table
    for (i, _) in text_exports.iter().enumerate() {
        output.extend((i as u16).to_le_bytes());
    }

    assert_eq!(
        edata_range.virtual_address + output.len() as u32,
        string_table_rva
    );

    output.extend(&string_table.0);

    output
}

fn make_dll_stub_impl(
    name: &str,
    thunk_id_allocator: &mut ThunkIdAllocator,
    imported_functions: &[OwnedImport],
) -> Result<(Vec<u8>, BTreeMap<u32, PeSymbol>)> {
    let mut out_data = Vec::new();
    let mut writer = Writer::new(false, PAGE_ALIGNMENT, 0x200, &mut out_data);

    writer.reserve_dos_header();
    writer.reserve_nt_headers(16);
    // writer.reserve_section_headers()
    writer.reserve_section_headers(2); // we are fine with only .text & .edata

    let Some(exports) = crate::dll_exports::HOST_DLL_EXPORTS.get(name) else {
        panic!("No exports for {}", name);
    };

    let ordinals = crate::stubs_list::DLL_ORDINALS.get(name);

    let missing_exports = imported_functions
        .iter()
        .map(|f| match f {
            OwnedImport::ByName(name) => name.as_str(),
            OwnedImport::ByOrdinal(ordinal) => {
                if let Some(ordinals) = ordinals {
                    if let Some(&name) = ordinals.get(ordinal) {
                        name
                    } else {
                        panic!("No name for ordinal {} in {}", ordinal, name)
                    }
                } else {
                    panic!("No ordinals for {}", name)
                }
            }
        })
        .filter(|name| !exports.contains(name))
        .map(|f| f.to_string())
        .collect::<Vec<_>>();

    if !missing_exports.is_empty() {
        panic!("Missing exports: {:?}", missing_exports);
    }

    let (text_data, text_exports) = gen_text_section(thunk_id_allocator, exports);

    let text_range = writer.reserve_text_section(text_data.len() as u32);

    let edata_strings = gen_edata_string_table(name, &text_exports);
    let edata_size = edata_len(name, &text_exports, &edata_strings);

    let edata_range = writer.reserve_edata_section(edata_size);

    let edata = gen_edata(name, &text_exports, &edata_strings, edata_range, text_range);

    assert_eq!(edata.len() as u32, edata_size);

    writer.write_empty_dos_header()?;
    writer.write_nt_headers(NtHeaders {
        machine: pe::IMAGE_FILE_MACHINE_I386,
        time_date_stamp: 0,
        characteristics: pe::IMAGE_FILE_EXECUTABLE_IMAGE
            | pe::IMAGE_FILE_LINE_NUMS_STRIPPED
            | pe::IMAGE_FILE_LOCAL_SYMS_STRIPPED
            | pe::IMAGE_FILE_32BIT_MACHINE,
        major_linker_version: 0,
        minor_linker_version: 0,
        address_of_entry_point: 0,
        image_base: 0,
        major_operating_system_version: 0,
        minor_operating_system_version: 0,
        major_image_version: 0,
        minor_image_version: 0,
        major_subsystem_version: 0,
        minor_subsystem_version: 0,
        subsystem: pe::IMAGE_SUBSYSTEM_WINDOWS_GUI,
        dll_characteristics: 0,
        size_of_stack_reserve: 0,
        size_of_stack_commit: 0,
        size_of_heap_reserve: 0,
        size_of_heap_commit: 0,
    });
    writer.write_section_headers();
    writer.write_section(text_range.file_offset, &text_data);
    writer.write_section(edata_range.file_offset, &edata);

    let symbols = text_exports
        .into_iter()
        .map(|(name, addr)| {
            (
                text_range.virtual_address + addr,
                PeSymbol {
                    name,
                    kind: SymbolKind::Code,
                },
            )
        })
        .collect();
    Ok((out_data, symbols))
}

pub fn make_dll_stub(
    name: &str,
    thunk_id_allocator: &mut ThunkIdAllocator,
    functions: &[OwnedImport],
) -> Result<PeFile> {
    let (bytes, symbols) = make_dll_stub_impl(name, thunk_id_allocator, functions)?;

    let pe = PeFile::parse_from_memory(name.to_string(), bytes)?.with_symbols(symbols);

    Ok(pe)
}
