use core_mem::ptr::PtrRepr;
use thiserror::Error;

mod mapper;
mod os;

mod address_range;
mod align;
mod manager;
mod page_region_state;
mod page_state;

pub use crate::{manager::*, mapper::MapperError, page_state::*};

pub const PAGE_SIZE: PtrRepr = 4096;

/// A collection of possible errors.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Attempt to reserve a provided memory region, but it intersects one of already reserved region")]
    ReservedRegionIntersects,
    #[error("Attempt to reserve the low 64K (well, the zero address)")]
    ReserveZeroAddress,
    #[error(
        "Attempt to reserve a region, but no available place found for it in the address space"
    )]
    ReserveNoAddressSpace,
    #[error("Attempt to reserve a region, but it is out of range for 32-bit address space")]
    ReserveOutOfRange,
    #[error("Attempt to unreserve region that is not currently previously reserved")]
    UnreserveNonexistentRegion,

    #[error("No reserved region fully contained the specified memory range fully")]
    NoRegionContainsRangeFully,

    #[error("Mapper error occurred: {0}")]
    MapperError(#[from] MapperError),
}

pub type Result<T> = std::result::Result<T, Error>;
