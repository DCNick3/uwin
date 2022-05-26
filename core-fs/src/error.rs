use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("Entry with this name already exists")]
    Exists,
    #[error("Attempt to create an entry in a read-only tree")]
    Readonly,
}

#[derive(Error, Debug)]
pub enum OpenError {
    #[error("Attempt to open a file with unsupported options (no access?)")]
    InvalidOptions,
    #[error("Attempt to open a file with write, append or truncate option")]
    Readonly,
}
