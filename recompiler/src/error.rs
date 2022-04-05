use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot open file")]
    FileOpen(std::io::Error),
    #[error("cannot mmap file")]
    FileMmap(std::io::Error),

    #[error("Couldn't parse pe file")]
    PeParse(#[from] object::Error),

    #[error("Couldn't generate pe file")]
    PeWrite(#[from] object::write::Error),

    #[error("Missing exports")]
    DllExportsNotFound(Vec<(String, String)>),
}

pub type Result<T> = std::result::Result<T, Error>;
