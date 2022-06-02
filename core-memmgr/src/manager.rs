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
use crate::{Error, PageState, Result, PAGE_SIZE};
use core_mem::align;
use core_mem::ctx::FlatMemoryCtx;
use core_mem::ptr::PtrRepr;
use memory_image::Protection;
use std::cmp::max;
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
            if let Some((&addr, state)) = self.regions.next() {
                let end = addr.checked_add(state.len_bytes());
                let end = end.map(|end| align::ceil(end, REGION_ALIGNMENT));

                if addr > self.last_addr {
                    break Some((addr, end));
                }

                if let Some(end) = end {
                    self.last_addr = max(self.last_addr, end);
                } else {
                    // edge cases are bad =(
                    self.last_addr = LAST_ADDR;
                    break None;
                }
            } else {
                break None;
            }
        };

        Some(match r {
            // yield the last hole: from the end of last region till the end of address space
            None => {
                if self.last_addr == LAST_ADDR {
                    return None;
                }
                let size = LAST_ADDR - self.last_addr + 1;

                let res = AddressRange::new(self.last_addr, size);
                self.last_addr = LAST_ADDR;

                res
            }
            Some((start, end)) => {
                let res = AddressRange::new(self.last_addr, start - self.last_addr);
                self.last_addr = end.unwrap_or(LAST_ADDR);

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

    pub fn reserve_dynamic(&mut self, size: PtrRepr) -> Result<AddressRange> {
        let size = align::ceil(size, PAGE_SIZE);

        let mut iter = self.iter_holes();
        let hole = loop {
            if let Some(hole) = iter.next() {
                if hole.size >= size {
                    // found a suitable hole, yay
                    break hole;
                }
            } else {
                return Err(Error::ReserveNoAddressSpace);
            }
        };

        assert!(self
            .regions
            .insert(hole.start, PageRegionState::new(size))
            .is_none());

        Ok(AddressRange::new(hole.start, size))
    }

    /// Align the range with the VirtualAlloc semantics: https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc
    ///
    // addr: If the memory is being reserved, the specified address is rounded down to the nearest multiple of the allocation granularity
    // size: The allocated pages include all pages containing one or more bytes in the range from lpAddress to lpAddress+dwSize.
    //        This means that a 2-byte range straddling a page boundary causes both pages to be included in the allocated region.
    ///
    /// Address will be aligned to addr_alignment (usually PAGE_SIZE or REGION_ALIGNMENT) and size will be aligned to PAGE_SIZE
    fn align_range(range: AddressRange, addr_alignment: PtrRepr) -> Result<AddressRange> {
        let addr = range.start;
        let size = range.size;

        // unaligned portion of the address
        let addr_unalign = addr % addr_alignment;

        let aligned_addr = align::floor(addr, addr_alignment);
        let aligned_size = align::ceil(size.checked_add(addr_unalign).unwrap(), PAGE_SIZE);

        AddressRange::try_new(aligned_addr, aligned_size)
    }

    pub fn reserve_static(&mut self, range: AddressRange) -> Result<AddressRange> {
        let range = Self::align_range(range, REGION_ALIGNMENT)?;

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

        Ok(range)
    }

    pub fn unreserve(&mut self, addr: PtrRepr) -> Result<()> {
        let state = self
            .regions
            .remove(&addr)
            .ok_or(Error::UnreserveNonexistentRegion)?;

        // TODO: maybe we should uncommit these pages?
        assert!(state.iter().all(|s| !s.committed()));

        Ok(())
    }

    /// # Safety
    ///
    /// You must ensure that lifetime of FlatMemoryCtx will not outlive the lifetime of Mapper
    /// (Rust lifetimes not used here because it's hard to statically verify lifetime of every place where memory context is used)
    pub unsafe fn memory_context(&self) -> FlatMemoryCtx {
        self.mapper.memory_context()
    }

    fn find_region_mut(
        regions: &mut BTreeMap<PtrRepr, PageRegionState>,
        needle_range: AddressRange,
    ) -> Result<(AddressRange, &mut PageRegionState)> {
        {
            let mut it = regions.range_mut(..=needle_range.start);
            if let Some((&region_addr, state)) = it.next_back() {
                let range = AddressRange::new(region_addr, state.len_bytes());
                if range.intersect(&needle_range).is_over() {
                    return Ok((range, state));
                }
            }
        }
        Err(Error::NoRegionContainsRangeFully)
    }

    pub fn commit(&mut self, range: AddressRange, protection: Protection) -> Result<AddressRange> {
        let range = Self::align_range(range, PAGE_SIZE)?;
        let region_range = Self::align_range(range, REGION_ALIGNMENT)?;

        let (region_range, state) = Self::find_region_mut(&mut self.regions, region_range)?;

        let region_page_indices = region_range.pages_indices(range);

        // Win32's commit is a weird one...
        // The operation is roughly equivalent to the following atomic operation:
        // for each page:
        // - if a page is not committed - commit it with the specified protection
        // - if a page is committed - change its protection to match the specified protection

        // it's not __that__ difficult to implement it, but doing it atomically is a bit more tricky
        // to do it reliably we need to assume that at least changing protection and unmapping cannot fail (and I do exactly that =))

        // generally, we try to apply the changes and roll the back if they fail
        // finally, if we succeeded, we update the info about page states

        enum RollbackOp {
            Protect(AddressRange, Protection),
            Uncommit(AddressRange),
        }

        let mut rollback_ops = Vec::new();

        for (address_range, state) in
            state.iter_page_runs_in_range(range.start, region_page_indices.clone())
        {
            let res = match state {
                PageState::Uncommitted => {
                    rollback_ops.push(RollbackOp::Uncommit(address_range));

                    self.mapper.map(address_range, protection)
                }
                PageState::Committed(cur_protection) => {
                    if cur_protection != protection {
                        rollback_ops.push(RollbackOp::Protect(address_range, cur_protection));

                        self.mapper.protect(address_range, protection)
                    } else {
                        Ok(())
                    }
                }
            };

            if let Err(e) = res {
                for op in rollback_ops {
                    match op {
                        RollbackOp::Protect(range, prot) => self
                            .mapper
                            .protect(range, prot)
                            .expect("Changing memory protection for rollback due to another error"),
                        RollbackOp::Uncommit(range) => self
                            .mapper
                            .unmap(range)
                            .expect("Unmapping memory for rollback due to another error"),
                    }
                }

                return Err(e.into());
            }
        }

        // yay, update the stored states
        state.set_range(region_page_indices, PageState::Committed(protection));

        Ok(range)
    }

    /// Changes protection of the specified page range
    ///
    /// The range must be within one reserved page region
    ///
    /// Returns the "old" protection of the first page
    pub fn protect(&mut self, range: AddressRange, protection: Protection) -> Result<Protection> {
        let range = Self::align_range(range, PAGE_SIZE)?;
        let region_range = Self::align_range(range, REGION_ALIGNMENT)?;

        let (region_range, state) = Self::find_region_mut(&mut self.regions, region_range)?;

        let region_page_indices = region_range.pages_indices(range);

        if !state
            .iter_in_range(region_page_indices.clone())
            .all(|s| s.committed())
        {
            return Err(Error::ProtectUncommitted);
        }

        let old_prot = match state.get(region_page_indices.start) {
            PageState::Uncommitted => unreachable!(), // checked above
            PageState::Committed(prot) => prot,
        };

        for (address_range, state) in
            state.iter_page_runs_in_range(range.start, region_page_indices.clone())
        {
            match state {
                PageState::Uncommitted => unreachable!(), // checked above
                PageState::Committed(current_protection) => {
                    if current_protection != protection {
                        // we assume that this never fails
                        // this is not true, but rare enough, I guess...
                        // if it is needed, it would be possible to do the same thing as commit does with the rollback
                        self.mapper
                            .protect(address_range, protection)
                            .expect("Could not change protection")
                    }
                }
            }
        }

        state.set_range(region_page_indices, PageState::Committed(protection));

        Ok(old_prot)
    }

    pub fn uncommit(&mut self, range: AddressRange) -> Result<()> {
        let range = Self::align_range(range, PAGE_SIZE)?;
        let region_range = Self::align_range(range, REGION_ALIGNMENT)?;

        let (region_range, state) = Self::find_region_mut(&mut self.regions, region_range)?;

        let region_page_indices = region_range.pages_indices(range);

        for (address_range, state) in
            state.iter_page_runs_in_range(range.start, region_page_indices.clone())
        {
            match state {
                PageState::Uncommitted => {} // already uncommitted =)
                PageState::Committed(_) => {
                    // we assume that this never fails
                    // this is not true, but rare enough, I guess...
                    // if it is needed, it would be possible to do the same thing as commit does with the rollback
                    self.mapper
                        .unmap(address_range)
                        .expect("Could not unmap page")
                }
            }
        }

        state.set_range(region_page_indices, PageState::Uncommitted);

        Ok(())
    }

    // TODO: cover this function with tests
    pub fn reserve_and_commit_static(
        &mut self,
        range: AddressRange,
        protection: Protection,
    ) -> Result<AddressRange> {
        let reservation = self.reserve_static(range)?;

        if let Err(e) = self.commit(reservation, protection) {
            self.unreserve(reservation.start).unwrap();
            return Err(e);
        }

        Ok(reservation)
    }

    // TODO: cover this function with tests
    pub fn reserve_and_commit_dynamic(
        &mut self,
        size: PtrRepr,
        protection: Protection,
    ) -> Result<AddressRange> {
        let reservation = self.reserve_dynamic(size)?;

        if let Err(e) = self.commit(reservation, protection) {
            self.unreserve(reservation.start).unwrap();
            return Err(e);
        }

        Ok(reservation)
    }

    // TODO: cover this function with tests
    pub fn uncommit_and_unreserve(&mut self, addr: PtrRepr) -> Result<()> {
        let region_size = self
            .regions
            .get(&addr)
            .ok_or(Error::UnreserveNonexistentRegion)?
            .len_bytes();

        self.uncommit(AddressRange::new(addr, region_size))?;

        // should be there
        self.regions.remove(&addr).unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod reserve {
        use crate::address_range::AddressRange;
        use crate::manager::{LAST_ADDR, REGION_ALIGNMENT, START_ADDR};
        use crate::{Error, MemoryManager};

        #[test]
        fn reserve_low() {
            let mut mgr = MemoryManager::new().unwrap();
            let range = mgr
                .reserve_static(AddressRange::new(0x10001, 0x1000))
                .unwrap();
            assert_eq!(range.start, 0x10000);
            assert_eq!(range.size, 0x2000);

            assert_eq!(mgr.regions.get(&range.start).unwrap().len_bytes(), 0x2000);

            let mut hole_iter = mgr.iter_holes();
            assert_eq!(
                hole_iter.next(),
                Some(AddressRange::from_range(START_ADDR, 0))
            );
            assert_eq!(hole_iter.next(), None);

            mgr.unreserve(range.start).unwrap();

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
            let range = mgr
                .reserve_static(AddressRange::new(0x10111000, provided_size))
                .unwrap();
            assert_eq!(range.start, 0x10110000);
            assert_eq!(range.size, expected_size);

            let mut hole_iter = mgr.iter_holes();
            assert_eq!(
                hole_iter.next(),
                Some(AddressRange::from_range(START_ADDR, range.start))
            );
            assert_eq!(
                hole_iter.next(), // we use REGION_ALIGNMENT instead of expected size because holes' address are aligned to REGION_ALIGNED (and expected_size is not aligned to it)
                Some(AddressRange::from_range(range.start + REGION_ALIGNMENT, 0))
            );
            assert_eq!(hole_iter.next(), None);

            mgr.unreserve(range.start).unwrap();

            assert!(mgr.regions.is_empty());
        }

        #[test]
        fn reserve_start_holes_correct() {
            let mut mgr = MemoryManager::new().unwrap();
            let range = mgr
                .reserve_static(AddressRange::new(START_ADDR, 0x1000))
                .unwrap();

            let mut hole_iter = mgr.iter_holes();
            assert_eq!(
                hole_iter.next(), // we use REGION_ALIGNMENT instead of expected size because holes' address are aligned to REGION_ALIGNED (and expected_size is not aligned to it)
                Some(AddressRange::from_range(range.start + REGION_ALIGNMENT, 0))
            );
            assert_eq!(hole_iter.next(), None);
        }

        #[test]
        fn reserve_start_holes_correct2() {
            let mut mgr = MemoryManager::new().unwrap();
            let range = mgr
                .reserve_static(AddressRange::new(START_ADDR, 0x1000))
                .unwrap();
            let range1 = mgr
                .reserve_static(AddressRange::new(START_ADDR + REGION_ALIGNMENT, 0x1000))
                .unwrap();

            mgr.unreserve(range.start).unwrap();

            let mut hole_iter = mgr.iter_holes();
            assert_eq!(
                hole_iter.next(),
                Some(AddressRange::from_range(
                    START_ADDR,
                    START_ADDR + REGION_ALIGNMENT
                ))
            );
            assert_eq!(
                hole_iter.next(),
                Some(AddressRange::from_range(range1.start + REGION_ALIGNMENT, 0))
            );
            assert_eq!(hole_iter.next(), None);
        }

        #[test]
        fn reserve_dynamic() {
            let mut mgr = MemoryManager::new().unwrap();

            let addr = mgr.reserve_dynamic(0x1000).unwrap();
            assert_eq!(addr.start, START_ADDR);

            let addr1 = mgr.reserve_dynamic(0x1000).unwrap();
            assert_eq!(addr1.start, START_ADDR + REGION_ALIGNMENT);

            mgr.unreserve(addr.start).unwrap();

            let addr = mgr.reserve_dynamic(0x4000).unwrap();
            assert_eq!(addr.start, START_ADDR);
        }

        #[test]
        fn reserve_whole_space() {
            let mut mgr = MemoryManager::new().unwrap();
            mgr.reserve_dynamic(LAST_ADDR - START_ADDR + 1).unwrap();

            let mut hole_iter = mgr.iter_holes();
            assert_eq!(hole_iter.next(), None);
        }

        #[test]
        fn reserve_whole_space2() {
            let mut mgr = MemoryManager::new().unwrap();
            mgr.reserve_static(AddressRange::new(
                START_ADDR - REGION_ALIGNMENT,
                LAST_ADDR - START_ADDR + 1,
            ))
            .unwrap();

            let mut hole_iter = mgr.iter_holes();
            assert_eq!(
                hole_iter.next(),
                Some(AddressRange::from_range(
                    LAST_ADDR - REGION_ALIGNMENT + 1,
                    0
                ))
            );
            assert_eq!(hole_iter.next(), None);
        }

        #[test]
        fn reserve_whole_space3() {
            let mut mgr = MemoryManager::new().unwrap();
            mgr.reserve_static(AddressRange::new(
                START_ADDR + REGION_ALIGNMENT,
                LAST_ADDR - START_ADDR - REGION_ALIGNMENT + 1,
            ))
            .unwrap();

            let mut hole_iter = mgr.iter_holes();
            assert_eq!(
                hole_iter.next(),
                Some(AddressRange::from_range(
                    START_ADDR,
                    START_ADDR + REGION_ALIGNMENT,
                ))
            );
            assert_eq!(hole_iter.next(), None);
        }

        #[test]
        fn reserve_out_of_range() {
            assert!(matches!(
                // well, we can't really raise this error inside the manager, it's caught by the AddressSpace constructor
                AddressRange::try_new(LAST_ADDR - REGION_ALIGNMENT + 1, REGION_ALIGNMENT * 2)
                    .unwrap_err(),
                Error::RangeCrossesBoundary
            ));
        }

        #[test]
        fn reserve_already_reserved() {
            let mut mgr = MemoryManager::new().unwrap();
            mgr.reserve_static(AddressRange::new(START_ADDR, 0x1000))
                .unwrap();
            assert!(matches!(
                mgr.reserve_static(AddressRange::new(START_ADDR, 0x2000))
                    .unwrap_err(),
                Error::ReservedRegionIntersects
            ));

            mgr.reserve_static(AddressRange::new(
                START_ADDR + REGION_ALIGNMENT,
                REGION_ALIGNMENT * 2,
            ))
            .unwrap();
            assert!(matches!(
                mgr.reserve_static(AddressRange::new(START_ADDR + REGION_ALIGNMENT * 2, 0x2000))
                    .unwrap_err(),
                Error::ReservedRegionIntersects
            ));
        }

        #[test]
        fn unreserve_nonreserved() {
            let mut mgr = MemoryManager::new().unwrap();

            assert!(matches!(
                mgr.unreserve(START_ADDR + 1212121).unwrap_err(),
                Error::UnreserveNonexistentRegion
            ));

            mgr.reserve_static(AddressRange::new(START_ADDR, 0x1000))
                .unwrap();
            mgr.unreserve(START_ADDR).unwrap();
            assert!(matches!(
                mgr.unreserve(START_ADDR).unwrap_err(),
                Error::UnreserveNonexistentRegion
            ));
        }

        #[test]
        fn reserve_zero_address() {
            let mut mgr = MemoryManager::new().unwrap();
            assert!(matches!(
                mgr.reserve_static(AddressRange::new(0x1000, 0x1000))
                    .unwrap_err(),
                Error::ReserveZeroAddress
            ));
        }
    }

    mod commit {
        use crate::address_range::AddressRange;
        use crate::{Error, MemoryManager};
        use bulletproof::Bulletproof;
        use memory_image::Protection;
        use rusty_fork::rusty_fork_test;

        rusty_fork_test! {
            #[test]
            fn basic() {
                let mut mgr = MemoryManager::new().unwrap();
                let ctx = unsafe { mgr.memory_context() };

                let range = mgr.reserve_dynamic(0x1000).unwrap();

                mgr.commit(range, Protection::READ_WRITE).unwrap();

                let ptr = ctx.to_native_ptr(range.start);
                unsafe { std::ptr::write(ptr, 12) };

                mgr.commit(range, Protection::READ_WRITE).unwrap();

                assert_eq!(unsafe { std::ptr::read(ptr) }, 12);

                mgr.protect(range, Protection::READ).unwrap();

                assert_eq!(unsafe { std::ptr::read(ptr) }, 12);

                unsafe {
                    // TODO: probably want to unregister this
                    let bulletproof = Bulletproof::new();
                    assert!(bulletproof.store(ptr, &13).is_err());
                }
            }

            #[test]
            fn unmap_multiple() {
                let mut mgr = MemoryManager::new().unwrap();
                let ctx = unsafe { mgr.memory_context() };

                let range = mgr.reserve_dynamic(0x4000).unwrap();

                mgr.commit(
                    AddressRange::new(range.start, 0x1000),
                    Protection::READ_WRITE,
                )
                .unwrap();
                mgr.commit(
                    AddressRange::new(range.start + 0x2000, 0x1000),
                    Protection::READ_WRITE,
                )
                .unwrap();

                unsafe {
                    let bulletproof = Bulletproof::new();

                    let ptr1 = ctx.to_native_ptr(range.start);
                    bulletproof.store(ptr1, &12).unwrap();

                    let ptr2 = ctx.to_native_ptr(range.start + 0x2000);
                    bulletproof.store(ptr2, &13).unwrap();

                    mgr.uncommit(range).unwrap();

                    assert!(bulletproof.load(ptr1).is_err());
                    assert!(bulletproof.load(ptr2).is_err());
                }
            }
        }

        #[test]
        fn protect_unmapped() {
            let mut mgr = MemoryManager::new().unwrap();
            let range = mgr.reserve_dynamic(0x4000).unwrap();

            mgr.commit(
                AddressRange::new(range.start + 0x2000, 0x1000),
                Protection::READ,
            )
            .unwrap();

            assert!(matches!(
                mgr.protect(range, Protection::READ_WRITE).unwrap_err(),
                Error::ProtectUncommitted
            ));
        }

        #[test]
        fn commit_alignment() {
            let mut mgr = MemoryManager::new().unwrap();
            let range = mgr.reserve_dynamic(0x4000).unwrap();

            let subrange = AddressRange::new(range.start + 0x1000, 0x1000);

            // subrange is already aligned, should not change
            assert_eq!(
                mgr.commit(subrange, Protection::READ_WRITE).unwrap(),
                subrange
            );
        }

        #[test]
        fn commit_in_large_region() {
            // this test is a boiled down version of a test case that broke the buggy find_region_mut function
            // it incorrectly searched for regions that start __after__ the needle region (start..), while it should have done it the other way around (..=start)
            // it worked for reservations < than 64k in size (I think?)

            let mut mgr = MemoryManager::new().unwrap();
            let rg = mgr.reserve_dynamic(0x30000).unwrap(); // reserve the 3 * 64 KiB region
            let reserve_rg = AddressRange::new(rg.start + 0x22000, 0x1000);
            mgr.commit(reserve_rg, Protection::READ_WRITE).unwrap();
        }
    }
}
