//! Here we try to emulate windows virtual memory model
//! To access memory you need to do two things: RESERVE it and COMMIT (See VirtualAlloc docs)
//!
//! Reservation (and unreservation) is done in regions.
//! You cannot unreserve region partially, only full region.
//! (Regions can be listed with new win10 RS1 QueryVirtualMemoryInformation API; does not work on WoW64 for me)
//! Also note that reserved region address must be 64KiB-aligned, but size requires only 4KiB alignment
//! This implies that if, for example, you allocate a region with size=4KiB, you lose 60KiB of address space
//! due to internal fragmentation: no other reservation can use that space, as there are no 64KiB aligned
//! addresses there.
//! Windows 95 has a bit different semantics: it rounds up your reservation request to 64 KiB so you do not lose that memory
//! you can commit it. I do not (yet) implement this semantic and go with the NT one (cuz meh, does anything that is not system program rely on it?)
//! (can be studied with VirtualQuery)
//!
//! Committing, on the other hand, is done with page (4KiB) granularity, you can commit and uncommit any 4 KiB
//! (or more) of the reserved region. Changing page protection can also be done at individual page basis
//!
//! Therefore the following approach is taken:
//! We store a sorted set of reserved regions. Inside those regions we store info about committed pages
//! This storage is implemented as BTreeMap<u32, PageRegionState> and PageRegionState (does not do any allocation, only bookkeeping)
//! Also looks like this is the approach taken in windows, as it does not allow either VirtualProtect or
//! VirtualAlloc with MEM_COMMIT to cross the reserved region boundary, even if they are adjacent

use crate::mapper::Mapper;
use crate::page_region_state::PageRegionState;
use crate::Result;
use std::collections::BTreeMap;

pub struct MemoryManager {
    mapper: Mapper,
    regions: BTreeMap<u32, PageRegionState>,
}

impl MemoryManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            mapper: Mapper::new()?,
            regions: Default::default(),
        })
    }

    //
}
