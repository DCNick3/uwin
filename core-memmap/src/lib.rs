use core_mem::ptr::PtrRepr;
use std::io;
use thiserror::Error;

mod mapper;
mod os;

mod address_range;
mod align;
mod manager;
mod page_region_state;
mod page_state;

pub use crate::{manager::*, page_state::*};

pub const PAGE_SIZE: PtrRepr = 4096;

/// A collection of possible errors.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Unaligned size: {0:#08x}")]
    UnalignedSize(PtrRepr),
    #[error("Unaligned address: {0:#08x}")]
    UnalignedAddress(PtrRepr),

    #[error("Attempt to reserve a provided memory region, but it intersects one of already reserved region")]
    ReservedRegionIntersects,
    #[error("Attempt to reserve the low 64K (well, the zero address)")]
    ReserveZeroAddress,
    #[error("Attempt to unreserve region that is not currently previously reserved")]
    UnreserveNonexistentRegion,

    #[error("A system call failed: {0}")]
    SystemCall(#[from] io::Error),
    #[error("macOS kernel call failed: {0}")]
    MachCall(libc::c_int),
}

pub type Result<T> = std::result::Result<T, Error>;
