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

use crate::address_range::AddressRange;
use crate::mapper::Mapper;
use crate::page_region_state::PageRegionState;
use crate::{align, Error, Result, PAGE_SIZE};
use core_mem::ptr::PtrRepr;
use std::collections::BTreeMap;

const REGION_ALIGNMENT: PtrRepr = 0x10000; // 64K
const START_ADDR: PtrRepr = 0x400000;
const LAST_ADDR: PtrRepr = PtrRepr::MAX;

// invariant: all regions are aligned to REGION_ALIGNMENT
pub struct MemoryManager {
    mapper: Mapper,
    regions: BTreeMap<PtrRepr, PageRegionState>,
}

/// Iterates over memory holes
/// All holes yielded are 64K-aligned, so unaligned parts are skipped
/// Used to search for free region
struct HoleIter<'a> {
    regions: std::collections::btree_map::Iter<'a, PtrRepr, PageRegionState>,
    last_addr: PtrRepr,
}

impl<'a> Iterator for HoleIter<'a> {
    type Item = AddressRange;

    fn next(&mut self) -> Option<Self::Item> {
        // a special value to nominate the end of iteration
        if self.last_addr == LAST_ADDR {
            return None;
        }
        let r = loop {
            if let Some((addr, state)) = self.regions.next() {
                let end = addr + state.len_bytes();
                let end = align::ceil(end, REGION_ALIGNMENT);
                if end > self.last_addr {
                    break Some((addr, end));
                }
            } else {
                break None;
            }
        };

        Some(match r {
            // yield the last hole: from the end of last region till the end of address space
            None => {
                let res = AddressRange::new(self.last_addr, LAST_ADDR - self.last_addr + 1);
                self.last_addr = LAST_ADDR;

                res
            }
            Some((start, end)) => {
                let res = AddressRange::new(self.last_addr, start - self.last_addr);
                self.last_addr = end;

                res
            }
        })
    }
}

impl MemoryManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            mapper: Mapper::new()?,
            regions: Default::default(),
        })
    }

    fn iter_holes(&self) -> HoleIter {
        HoleIter {
            regions: self.regions.iter(),
            last_addr: START_ADDR,
        }
    }

    pub fn reserve_dynamic(&mut self, size: PtrRepr) -> Result<PtrRepr> {
        todo!()
    }

    pub fn reserve_static(&mut self, addr: PtrRepr, size: PtrRepr) -> Result<PtrRepr> {
        // should follow VirtualAlloc's semantics: https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc
        // addr: If the memory is being reserved, the specified address is rounded down to the nearest multiple of the allocation granularity
        // size: The allocated pages include all pages containing one or more bytes in the range from lpAddress to lpAddress+dwSize.
        //        This means that a 2-byte range straddling a page boundary causes both pages to be included in the allocated region.
        let aligned_addr = align::floor(addr, REGION_ALIGNMENT);
        let range = AddressRange::new(
            aligned_addr,
            align::ceil(addr.checked_add(size).unwrap(), PAGE_SIZE) - aligned_addr, // align up the .end()
        );

        if range.start == 0 {
            return Err(Error::ReserveZeroAddress);
        }

        if self.regions.iter().any(|(&addr, state)| {
            AddressRange::new(addr, state.len_bytes())
                .intersect(&range)
                .is_any()
        }) {
            return Err(Error::ReservedRegionIntersects);
        }

        assert!(self
            .regions
            .insert(range.start, PageRegionState::new(range.size))
            .is_none());

        Ok(range.start)
    }

    pub fn unreserve(&mut self, addr: PtrRepr) -> Result<()> {
        let state = self
            .regions
            .remove(&addr)
            .ok_or(Error::UnreserveNonexistentRegion)?;

        // TODO: maybe we should uncommit these pages?
        assert!(state.iter().all(|s| !s.committed));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::address_range::AddressRange;
    use crate::manager::{REGION_ALIGNMENT, START_ADDR};
    use crate::MemoryManager;

    #[test]
    fn reserve_low() {
        let mut mgr = MemoryManager::new().unwrap();
        let addr = mgr.reserve_static(0x10001, 0x1000).unwrap();
        assert_eq!(addr, 0x10000);

        assert_eq!(mgr.regions.get(&addr).unwrap().len_bytes(), 0x2000);

        let mut hole_iter = mgr.iter_holes();
        assert_eq!(
            hole_iter.next(),
            Some(AddressRange::from_range(START_ADDR, 0))
        );
        assert_eq!(hole_iter.next(), None);

        mgr.unreserve(addr).unwrap();

        assert!(mgr.regions.is_empty());
    }

    #[test]
    fn initially_empty() {
        let mgr = MemoryManager::new().unwrap();
        assert!(mgr.regions.is_empty());
        let mut hole_iter = mgr.iter_holes();

        assert_eq!(
            hole_iter.next(),
            Some(AddressRange::from_range(START_ADDR, 0))
        );
        assert_eq!(hole_iter.next(), None);
        assert_eq!(hole_iter.next(), None);
    }

    #[test]
    fn reserve_hi() {
        let mut mgr = MemoryManager::new().unwrap();
        let provided_size = 0xfff;
        let expected_size = 0x2000;
        let addr = mgr.reserve_static(0x10111000, provided_size).unwrap();
        assert_eq!(addr, 0x10110000);

        assert_eq!(mgr.regions.get(&addr).unwrap().len_bytes(), expected_size);

        let mut hole_iter = mgr.iter_holes();
        assert_eq!(
            hole_iter.next(),
            Some(AddressRange::from_range(START_ADDR, addr))
        );
        assert_eq!(
            hole_iter.next(), // we use REGION_ALIGNMENT instead of expected size because holes' address are aligned to REGION_ALIGNED (and expected_size is not aligned to it)
            Some(AddressRange::from_range(addr + REGION_ALIGNMENT, 0))
        );
        assert_eq!(hole_iter.next(), None);

        mgr.unreserve(addr).unwrap();

        assert!(mgr.regions.is_empty());
    }
}