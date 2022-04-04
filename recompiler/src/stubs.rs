use crate::loader::PAGE_ALIGNMENT;
use object::pe;
use object::write::pe::{NtHeaders, SectionRange, Writer};
use object::write::Result;
use std::collections::{BTreeMap, HashMap};

fn gen_text_section(stubs: &BTreeMap<String, u32>) -> (Vec<u8>, BTreeMap<String, u32>) {
    let mut output = Vec::new();
    let mut labels = BTreeMap::new();

    for (name, index) in stubs {
        labels.insert(name.clone(), output.len() as u32);
        // FAR CALL opcode
        output.push(0x9a);

        // far call offset
        output.extend(index.to_le_bytes());
        // far call segment (magic number for UW)
        output.extend(0x7775u16.to_le_bytes());

        // RET opcode
        output.push(0xc3);
    }

    (output, labels)
}

type StringTable = (Vec<u8>, HashMap<String, u32>);

fn gen_edata_stringtable(name: &str, text_labels: &BTreeMap<String, u32>) -> StringTable {
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
    for name in text_labels.keys() {
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
    text_labels: &BTreeMap<String, u32>,
    string_table: &StringTable,
    edata_range: SectionRange,
    text_range: SectionRange,
) -> Vec<u8> {
    let mut output = Vec::new();

    let function_count = text_labels.len() as u32;
    let export_address_table_rva = edata_range.virtual_address + 10 * 4 /* header */;
    let export_name_pointer_table_rva = export_address_table_rva + 4 * text_labels.len() as u32;
    let export_ordinal_table = export_name_pointer_table_rva + 4 * text_labels.len() as u32;
    let string_table_rva = export_ordinal_table + 2 * text_labels.len() as u32;

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
    for offset in text_labels.values() {
        let rva = text_range.virtual_address + offset;
        output.extend(rva.to_le_bytes());
    }

    assert_eq!(
        edata_range.virtual_address + output.len() as u32,
        export_name_pointer_table_rva
    );
    // Export Name Pointer Table
    for name in text_labels.keys() {
        let name_offset = string_table.1.get(name).unwrap();
        let name_rva = string_table_rva + name_offset;
        output.extend(name_rva.to_le_bytes());
    }

    assert_eq!(
        edata_range.virtual_address + output.len() as u32,
        export_ordinal_table
    );
    // Export Ordinal Table
    for (i, _) in text_labels.iter().enumerate() {
        output.extend((i as u16).to_le_bytes());
    }

    assert_eq!(
        edata_range.virtual_address + output.len() as u32,
        string_table_rva
    );

    output.extend(&string_table.0);

    output
}

pub fn make_dll_stub(name: &str, stubs: &BTreeMap<String, u32>) -> Result<Vec<u8>> {
    let mut out_data = Vec::new();
    let mut writer = Writer::new(false, PAGE_ALIGNMENT, 0x200, &mut out_data);

    writer.reserve_dos_header();
    writer.reserve_nt_headers(16);
    // writer.reserve_section_headers()
    writer.reserve_section_headers(2); // we are fine with only .text & .edata

    let (text_data, text_labels) = gen_text_section(stubs);

    let text_range = writer.reserve_text_section(text_data.len() as u32);

    let edata_strings = gen_edata_stringtable(name, &text_labels);
    let edata_size = edata_len(name, &text_labels, &edata_strings);

    let edata_range = writer.reserve_edata_section(edata_size);

    let edata = gen_edata(name, &text_labels, &edata_strings, edata_range, text_range);

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

    // writer.name

    Ok(out_data)
}
