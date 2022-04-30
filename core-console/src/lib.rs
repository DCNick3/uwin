use ascii::AsciiStr;
use std::io::Write;
use tracing::warn;

pub trait Console {
    fn write(&self, buf: &[u8]) -> Result<usize, Error>;
}

pub struct DummyConsole {}
impl Console for DummyConsole {
    fn write(&self, buf: &[u8]) -> Result<usize, Error> {
        warn!("Write to a dummy terminal of size {}", buf.len());
        Ok(buf.len())
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

        Ok(buf.len())
    }
}

pub enum Error {}
