use std::io;
use thiserror::Error;

mod mapper;
mod os;

mod manager;
mod page_region_state;
mod page_state;

pub use crate::{manager::*, page_state::*};

const PAGE_SIZE: u32 = 4096;

/// A collection of possible errors.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Unaligned size: {0:#08x}")]
    UnalignedSize(u32),
    #[error("Unaligned address: {0:#08x}")]
    UnalignedAddress(u32),
    #[error("A system call failed: {0}")]
    SystemCall(#[from] io::Error),
    #[error("macOS kernel call failed: {0}")]
    MachCall(libc::c_int),
}

pub type Result<T> = std::result::Result<T, Error>;
