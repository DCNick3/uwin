use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("Entry with this name already exists")]
    Exists,
    #[error("Attempt to create an entry in a read-only tree")]
    Readonly,
}

#[derive(Error, Debug)]
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

#[derive(Error, Debug)]
pub enum OpenError {
    #[error("Attempt to open a file with unsupported options (no access?)")]
    InvalidOptions,
    #[error(
        "Attempt to open a file with write, append or truncate option, while the file is readonly"
    )]
    Readonly,
}
