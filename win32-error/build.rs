use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct ErrorRecord {
    code: u32,
    slug: String,
    desc: String,
}

fn main() {
    let csv_path = Path::new("src/winerror.csv");
    let mut reader = csv::Reader::from_path(csv_path).expect("Creating winerror.csv reader");

    let mut phf_map = phf_codegen::Map::new();

    let mut error_enum_body = String::new();

    for result in reader.deserialize() {
        let record: ErrorRecord = result.unwrap();
        phf_map.entry(record.code, &format!("{:?}", record));

        writeln!(
            &mut error_enum_body,
            "    {} = {},",
            record.slug, record.code
        )
        .unwrap();
    }

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    writeln!(
        &mut file,
        "static ERROR_CODES: phf::Map<u32, ErrorRecord> = {};",
        phf_map.build()
    )
    .unwrap();
    writeln!(
        &mut file,
        r"#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Win32Error {{
{}
}}",
        error_enum_body
    )
    .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/winerror.csv");
}
