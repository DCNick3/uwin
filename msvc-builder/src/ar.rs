// based on https://crates.io/crates/ar

use std::io::{self, Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
use std::str;

// ========================================================================= //

const GLOBAL_HEADER_LEN: usize = 8;
const GLOBAL_HEADER: &[u8; GLOBAL_HEADER_LEN] = b"!<arch>\n";

const ENTRY_HEADER_LEN: usize = 60;

const GNU_NAME_TABLE_ID: &str = "//";
const GNU_SYMBOL_LOOKUP_TABLE_ID: &[u8] = b"/";

// ========================================================================= //

/// Representation of an archive entry header.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header {
    identifier: Vec<u8>,
    mtime: u64,
    uid: u32,
    gid: u32,
    mode: u32,
    size: u64,
}

impl Header {
    /// Creates a header with the given file identifier and size, and all
    /// other fields set to zero.
    pub fn new(identifier: Vec<u8>, size: u64) -> Header {
        Header {
            identifier,
            mtime: 0,
            uid: 0,
            gid: 0,
            mode: 0,
            size,
        }
    }

    /// Returns the length of the file, in bytes.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Parses and returns the next header and its length.  Returns `Ok(None)`
    /// if we are at EOF.
    fn read<R>(reader: &mut R, name_table: &mut Vec<u8>) -> Result<Option<(Header, u64)>>
    where
        R: Read,
    {
        let mut buffer = [0; 60];
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(None);
        } else if bytes_read < buffer.len() {
            if let Err(error) = reader.read_exact(&mut buffer[bytes_read..]) {
                return if error.kind() == ErrorKind::UnexpectedEof {
                    let msg = "unexpected EOF in the middle of archive entry \
                               header";
                    Err(Error::new(ErrorKind::UnexpectedEof, msg))
                } else {
                    let msg = "failed to read archive entry header";
                    Err(annotate(error, msg))
                };
            }
        }
        let mut identifier = buffer[0..16].to_vec();
        while identifier.last() == Some(&b' ') {
            identifier.pop();
        }
        let size = parse_number("file size", &buffer[48..58], 10)?;
        let header_len = ENTRY_HEADER_LEN as u64;
        if identifier.starts_with(b"/") {
            if identifier == GNU_SYMBOL_LOOKUP_TABLE_ID {
                io::copy(&mut reader.by_ref().take(size), &mut io::sink())?;
                return Ok(Some((Header::new(identifier, size), header_len)));
            } else if identifier == GNU_NAME_TABLE_ID.as_bytes() {
                *name_table = vec![0; size as usize];
                reader
                    .read_exact(name_table as &mut [u8])
                    .map_err(|err| annotate(err, "failed to read name table"))?;
                return Ok(Some((Header::new(identifier, size), header_len)));
            }
            let start = parse_number("GNU filename index", &buffer[1..16], 10)? as usize;
            let end = match name_table[start..]
                .iter()
                .position(|&ch| ch == b'/' || ch == b'\x00')
            {
                Some(len) => start + len,
                None => name_table.len(),
            };
            identifier = name_table[start..end].to_vec();
        } else if identifier.ends_with(b"/") {
            identifier.pop();
        }
        let mtime = parse_number("timestamp", &buffer[16..28], 10)?;
        let uid = parse_number_permitting_empty("owner ID", &buffer[28..34], 10)? as u32;
        let gid = parse_number_permitting_empty("group ID", &buffer[34..40], 10)? as u32;
        let mode = parse_number("file mode", &buffer[40..48], 8)? as u32;
        Ok(Some((
            Header {
                identifier,
                mtime,
                uid,
                gid,
                mode,
                size,
            },
            header_len,
        )))
    }
}

fn parse_number(field_name: &str, bytes: &[u8], radix: u32) -> Result<u64> {
    if let Ok(string) = str::from_utf8(bytes) {
        if let Ok(value) = u64::from_str_radix(string.trim_end(), radix) {
            return Ok(value);
        }
    }
    let msg = format!(
        "Invalid {} field in entry header ({:?})",
        field_name,
        String::from_utf8_lossy(bytes)
    );
    Err(Error::new(ErrorKind::InvalidData, msg))
}

/*
 * Equivalent to parse_number() except for the case of bytes being
 * all spaces (eg all 0x20) as MS tools emit for UID/GID
 */
fn parse_number_permitting_empty(field_name: &str, bytes: &[u8], radix: u32) -> Result<u64> {
    if let Ok(string) = str::from_utf8(bytes) {
        let trimmed = string.trim_end();
        if trimmed.is_empty() {
            return Ok(0);
        } else if let Ok(value) = u64::from_str_radix(trimmed, radix) {
            return Ok(value);
        }
    }
    let msg = format!(
        "Invalid {} field in entry header ({:?})",
        field_name,
        String::from_utf8_lossy(bytes)
    );
    Err(Error::new(ErrorKind::InvalidData, msg))
}

// ========================================================================= //

/// A structure for reading archives.
pub struct Archive<R: Read> {
    reader: R,
    name_table: Vec<u8>,
    new_entry_start: u64,
    started: bool, // True if we've read past the global header.
    error: bool,   // True if we have encountered an error.
}

impl<R: Read> Archive<R> {
    /// Create a new archive reader with the underlying reader object as the
    /// source of all data read.
    pub fn new(reader: R) -> Archive<R> {
        Archive {
            reader,
            name_table: Vec::new(),
            new_entry_start: GLOBAL_HEADER_LEN as u64,
            started: false,
            error: false,
        }
    }

    fn read_global_header_if_necessary(&mut self) -> Result<()> {
        if self.started {
            return Ok(());
        }
        let mut buffer = [0; GLOBAL_HEADER_LEN];
        match self.reader.read_exact(&mut buffer) {
            Ok(()) => {}
            Err(error) => {
                self.error = true;
                return Err(annotate(error, "failed to read global header"));
            }
        }
        if &buffer != GLOBAL_HEADER {
            self.error = true;
            let msg = "Not an archive file (invalid global header)";
            return Err(Error::new(ErrorKind::InvalidData, msg));
        }
        self.started = true;
        Ok(())
    }
}

impl<R: Write + Read + Seek> Archive<R> {
    pub fn patch_timestamps(&mut self) -> Result<()> {
        self.read_global_header_if_necessary()?;
        loop {
            let header_start = self.new_entry_start;
            self.reader.seek(SeekFrom::Start(header_start))?;
            if let Some((header, header_len)) =
                Header::read(&mut self.reader, &mut self.name_table)?
            {
                let mut short_import_header = [0u8; 4];
                self.reader.read_exact(&mut short_import_header)?;
                // zero out the stuff in coff header
                if short_import_header[..2].as_ref() == b"\x4c\x01" {
                    // zero out the timestamp!
                    self.reader.write_all(b"\x00\x00\x00\x00")?;
                }
                // also zero out the stuff in "import header"
                if short_import_header[..4].as_ref() == b"\x00\x00\xff\xff" {
                    self.reader.seek(SeekFrom::Current(4))?;
                    self.reader.write_all(b"\x00\x00\x00\x00")?;
                }

                let size = header.size();
                self.reader.seek(SeekFrom::Start(header_start + 16))?;

                // zero out the timestamp
                self.reader.write_all(b"0           ")?;

                self.new_entry_start += header_len + size + (size % 2);
            } else {
                break;
            }
        }
        Ok(())
    }
}

// ========================================================================= //

fn annotate(error: Error, msg: &str) -> Error {
    let kind = error.kind();
    if let Some(inner) = error.into_inner() {
        Error::new(kind, format!("{}: {}", msg, inner))
    } else {
        Error::new(kind, msg)
    }
}

// ========================================================================= //
