use ascii::AsciiStr;
use std::io::{BufRead, Read, Write};
use tracing::{debug, warn};

// TODO: __actually__ console input and output are a different objects from stand point of win32 api
// The read handle is a "console input buffer"
// The write handle is "screen buffer handle"
// But it's not like we implement any remotely precise console semantics, so this should be fine
pub trait Console {
    fn write(&self, buf: &[u8]) -> Result<usize, Error>;
    fn read(&self, buf: &mut [u8]) -> Result<usize, Error>;
}

pub struct DummyConsole {}
impl Console for DummyConsole {
    fn write(&self, buf: &[u8]) -> Result<usize, Error> {
        warn!("Write to a dummy console of size {}", buf.len());
        Ok(buf.len())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize, Error> {
        assert!(buf.len() >= 2);
        warn!("Returning an empty line for a read from a dummy console");
        buf[0] = b'\r';
        buf[1] = b'\n';
        Ok(2)
    }
}

pub struct StdioConsole {}
impl Console for StdioConsole {
    fn write(&self, buf: &[u8]) -> Result<usize, Error> {
        // TODO: maybe we want to handle the OEM code page somehow...
        // right now just check it's all ascii
        AsciiStr::from_ascii(buf).unwrap();
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        // I don't believe in errors in stdout :shrug:
        stdout.write_all(buf).unwrap();
        stdout.flush().unwrap();

        Ok(buf.len())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let stdin = std::io::stdin();
        let stdin = stdin.lock();
        let mut buffer = Vec::new();

        assert!(buf.len() >= 2);
        stdin
            .take(buf.len() as u64 - 1)
            .read_until(b'\n', &mut buffer)
            .unwrap();

        if buffer.is_empty() {
            return Ok(0);
        }

        if *buffer.last().unwrap() == b'\n' {
            buffer.pop().unwrap();
            buffer.extend(b"\r\n");
        }

        assert!(buffer.len() <= buf.len());

        debug!("ReadFile: {:?}", buffer);

        let mut count = 0;
        for (buf, read) in buf.iter_mut().zip(buffer) {
            *buf = read;
            count += 1;
        }

        Ok(count)
    }
}

pub enum Error {}
