use crate::{Error, Result, PAGE_SIZE};
use core_mem::ptr::PtrRepr;
use range_ext::intersect::{Intersect, Intersection};
use std::ops::Range;

// EDGE CASE: this cannot encode the address range of the whole address space (because the size is 1 << 32)
// The solution?
// Well, don't do stupid (don't encode it)
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct AddressRange {
    pub start: PtrRepr,
    pub size: PtrRepr,
}

impl AddressRange {
    #[inline]
    pub fn new(start: PtrRepr, size: PtrRepr) -> Self {
        Self::try_new(start, size).unwrap()
    }

    #[inline]
    pub fn try_new(start: PtrRepr, size: PtrRepr) -> Result<Self> {
        if size == 0 {
            return Err(Error::RangeWithZeroSize);
        }

        // check that start + size doesn't cross the end of the address space
        // but allow for case where start + size is one past the end of address space
        // (it's 0 when used as wrapping_add)
        if start.checked_add(size).is_none() && start.wrapping_add(size) != 0 {
            return Err(Error::RangeCrossesBoundary);
        };
        Ok(Self { start, size })
    }

    #[inline]
    pub fn from_range(start: PtrRepr, end: PtrRepr) -> Self {
        // end = 0 is interpreted as the end of the address space
        assert!(start <= end || end == 0);
        Self::new(start, end.wrapping_sub(start))
    }

    #[inline]
    pub fn range(&self) -> Range<PtrRepr> {
        self.start..self.end()
    }

    #[inline]
    pub fn end(&self) -> PtrRepr {
        self.start + self.size
    }

    #[inline]
    pub fn intersect(&self, other: &AddressRange) -> Intersection {
        self.range().intersect(&other.range())
    }

    #[inline]
    pub fn pages_indices(&self, range: AddressRange) -> Range<usize> {
        assert!(self.intersect(&range).is_over());

        let offset = ((range.start - self.start) / PAGE_SIZE) as usize;
        let size = (range.size / PAGE_SIZE) as usize;

        offset..(offset + size)
    }
}
