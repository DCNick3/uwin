use std::io::ErrorKind;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CreateError {
    #[error("Entry with this name already exists")]
    Exists,
    #[error("Attempt to create an entry in a read-only tree")]
    Readonly,
}

#[derive(Error, Debug, Clone)]
pub enum RemoveError {
    #[error("Entry with this name does not exist")]
    DoesNotExist,
    #[error("Attempt to remove an entry in a read-only tree")]
    Readonly,
    #[error("Attempt to remove a directory that is not empty")]
    DirectoryNotEmpty,
    #[error("Attempt to remove a directory while not a directory was specified")]
    NotADirectory,
    #[error("Attempt to remove a file while not a file was specified")]
    NotAFile,
}

#[derive(Error, Debug, Clone)]
pub enum OpenError {
    #[error("Attempt to open a file with unsupported options (no access?)")]
    InvalidOptions,
    #[error(
        "Attempt to open a file with write, append or truncate option, while the file is readonly"
    )]
    Readonly,
}

#[derive(Error, Debug, Clone)]
pub enum WriteError {
    #[error("An attempt to write to a handle not open for writing was done")]
    NoAccess,
}

impl From<WriteError> for ErrorKind {
    fn from(e: WriteError) -> Self {
        match e {
            WriteError::NoAccess => ErrorKind::Other, // TODO: ???
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum ReadError {
    #[error("An attempt to read from a handle not open for reading was done")]
    NoAccess,
}

impl From<ReadError> for ErrorKind {
    fn from(e: ReadError) -> Self {
        match e {
            ReadError::NoAccess => ErrorKind::Other, // TODO: ???
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum SeekError {
    #[error("An attempt to seek was performed, but the resulting position is negative")]
    NegativePosition,
    #[error("An attempt to seek on a unseekable file handle was performed")]
    Unseekable,
}

impl From<SeekError> for ErrorKind {
    fn from(e: SeekError) -> Self {
        match e {
            SeekError::NegativePosition => ErrorKind::InvalidInput,
            SeekError::Unseekable => ErrorKind::Other, // TODO: ???
        }
    }
}
