use anyhow::{Context, Result};
use object::pe::IMAGE_DIRECTORY_ENTRY_DEBUG;
use object::read::pe::PeFile32;
use object::LittleEndian;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const LE: LittleEndian = LittleEndian {};

const PE_HEADER_TIMESTAMP_OFFSET: u32 = 8;
const DEBUG_DIRECTORY_ENTRY_SIZE: usize = 28;
const DEBUG_DIRECTORY_ENTRY_TIMESTAMP_OFFSET: u32 = 4;

pub fn zero_out_pe_timestamps(path: &Path) -> Result<()> {
    let mut data = Vec::new();
    File::open(path)
        .context("Opening PE file for reading")?
        .read_to_end(&mut data)
        .context("Reading the PE file")?;

    let pe = PeFile32::parse(data.as_slice()).context("Parsing the PE file")?;

    let pe_header_offset = pe.dos_header().e_lfanew.get(LE);

    let mut datetime_offsets = Vec::new();
    let header_timestamp_offset = pe_header_offset + PE_HEADER_TIMESTAMP_OFFSET;

    datetime_offsets.push(header_timestamp_offset);

    if let Some(debug) = pe.data_directory(IMAGE_DIRECTORY_ENTRY_DEBUG) {
        let (offset, size) = pe
            .section_table()
            .pe_file_range_at(debug.virtual_address.get(LE))
            .unwrap();

        assert!(size >= debug.size.get(LE));

        println!("{:?}", debug);
        for entry in (0..debug.size.get(LE)).step_by(DEBUG_DIRECTORY_ENTRY_SIZE) {
            let datetime_offset = offset + entry + DEBUG_DIRECTORY_ENTRY_TIMESTAMP_OFFSET;
            datetime_offsets.push(datetime_offset)
        }
    }

    for offset in datetime_offsets {
        let offset = offset as usize;
        (&mut data[offset..offset + 4]).fill(0);
    }

    File::create(path)
        .context("Opening the PE file for writing")?
        .write_all(&data)
        .context("Writing an updated PE file back")?;

    Ok(())
}
