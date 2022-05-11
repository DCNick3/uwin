use anyhow::{Context, Result};
use object::pe::{IMAGE_DIRECTORY_ENTRY_DEBUG, IMAGE_DIRECTORY_ENTRY_EXPORT};
use object::read::pe::PeFile32;
use object::{LittleEndian, Object, ObjectSymbol};
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;

const LE: LittleEndian = LittleEndian {};

const PE_HEADER_TIMESTAMP_OFFSET: u32 = 8;
const DEBUG_DIRECTORY_ENTRY_SIZE: usize = 28;
const DEBUG_DIRECTORY_ENTRY_TIMESTAMP_OFFSET: u32 = 4;
const EXPORT_DIRECTORY_TIMESTAMP_OFFSET: u32 = 4;

const OPTIONAL_HEADER_SIZE: u32 = 0x78;
const DATA_DIRECTORY_SIZE: u32 = 8;

const SYMBOL_SIZE: u32 = 18;
const SYMBOL_VALUE_OFFSET: u32 = 8;

pub fn fixup_file(
    path: &Path,
    fixup: impl FnOnce(&[u8]) -> Result<Vec<(u32, Vec<u8>)>>,
) -> Result<()> {
    let mut data = Vec::new();
    File::open(path)
        .context("Opening PE file for reading")?
        .read_to_end(&mut data)
        .context("Reading the PE file")?;

    let patches = fixup(&data)?;

    for (offset, val) in patches {
        let offset = offset as usize;
        (&mut data[offset..offset + val.len()])
            .write_all(&val)
            .unwrap();
    }

    File::create(path)
        .context("Opening the PE file for writing")?
        .write_all(&data)
        .context("Writing an updated PE file back")?;

    Ok(())
}

pub fn fixup_msvc_pe(data: &[u8]) -> Result<Vec<(u32, Vec<u8>)>> {
    let pe = PeFile32::parse(data).context("Parsing the PE file")?;

    let pe_header_offset = pe.dos_header().e_lfanew.get(LE);

    let mut patches = Vec::new();
    let header_timestamp_offset = pe_header_offset + PE_HEADER_TIMESTAMP_OFFSET;

    // zero out the timestamp in the header
    patches.push((header_timestamp_offset, 0u32.to_le_bytes().to_vec()));

    if let Some(debug) = pe.data_directory(IMAGE_DIRECTORY_ENTRY_DEBUG) {
        let (offset, size) = pe
            .section_table()
            .pe_file_range_at(debug.virtual_address.get(LE))
            .unwrap();

        assert!(size >= debug.size.get(LE));

        println!("{:?}", debug);
        for entry in (0..debug.size.get(LE)).step_by(DEBUG_DIRECTORY_ENTRY_SIZE) {
            let datetime_offset = offset + entry + DEBUG_DIRECTORY_ENTRY_TIMESTAMP_OFFSET;
            // zero out the timestamp in the debug directory entries
            patches.push((datetime_offset, 0u32.to_le_bytes().to_vec()))
        }
    }

    if let Some(export) = pe.data_directory(IMAGE_DIRECTORY_ENTRY_EXPORT) {
        let (offset, size) = pe
            .section_table()
            .pe_file_range_at(export.virtual_address.get(LE))
            .unwrap();

        assert!(size >= export.size.get(LE));

        println!("{:?}", export);
        let datetime_offset = offset + EXPORT_DIRECTORY_TIMESTAMP_OFFSET;

        patches.push((datetime_offset, 0u32.to_le_bytes().to_vec()));
    }

    let symbol_table_offset = pe.nt_headers().file_header.pointer_to_symbol_table.get(LE);

    for (symbol_index, symbol) in pe.symbols().enumerate() {
        if symbol.address() != 0 {
            let section_index = symbol.section_index().unwrap();
            let section = pe.section_table().section(section_index.0).unwrap();

            // fix a really weird MSVC ¿bug?
            // instead of storing offset from the start of the section the MSVC stores an RVA (offset from the base of the image)
            // object adds both image base & section rva to the value stored so we subtract them both from the address
            // then we subtract section address again to get the correctly stored value
            let address = symbol.address()
                - pe.relative_address_base()
                - (section.virtual_address.get(LE) as u64) * 2;
            let address = address as u32;

            let symbol_offset = symbol_table_offset + SYMBOL_SIZE * symbol_index as u32;

            patches.push((
                symbol_offset + SYMBOL_VALUE_OFFSET,
                address.to_le_bytes().to_vec(),
            ));
        }
    }

    let data_directories_offset = pe_header_offset + OPTIONAL_HEADER_SIZE;

    // we also "remove" debug info directory
    let debug_directory_offset =
        data_directories_offset + IMAGE_DIRECTORY_ENTRY_DEBUG as u32 * DATA_DIRECTORY_SIZE;
    patches.push((
        debug_directory_offset,
        vec![0u8; DATA_DIRECTORY_SIZE as usize],
    ));

    Ok(patches)
}

pub fn patch_msvc_import_lib(file: &mut (impl Read + Write + Seek)) -> Result<()> {
    let mut ar = crate::ar::Archive::new(file);

    ar.patch_timestamps()?;

    Ok(())
}
