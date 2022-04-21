use crate::address_range::AddressRange;
use crate::align::aligned;
use crate::page_state::{PageState, PageStateRepr};
use crate::PAGE_SIZE;
use core_mem::ptr::PtrRepr;
use std::iter::Peekable;
use std::ops::Range;

pub(crate) struct PageRegionState {
    page_states: Vec<PageStateRepr>,
}

impl PageRegionState {
    pub fn new(size_bytes: PtrRepr) -> Self {
        assert!(aligned(size_bytes, PAGE_SIZE));
        Self {
            page_states: vec![PageStateRepr::empty(); (size_bytes / PAGE_SIZE) as usize],
        }
    }

    pub fn len_pages(&self) -> usize {
        self.page_states.len()
    }

    pub fn len_bytes(&self) -> PtrRepr {
        (self.len_pages() as PtrRepr) * PAGE_SIZE
    }

    pub fn get(&self, page_index: usize) -> PageState {
        self.page_states[page_index].into()
    }

    #[allow(unused)]
    pub fn set(&mut self, page_index: usize, state: PageState) {
        self.page_states[page_index] = state.into()
    }

    pub fn set_range(&mut self, range: Range<usize>, state: PageState) {
        let repr = state.into();
        for i in range {
            self.page_states[i] = repr;
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            iter: self.page_states.iter(),
        }
    }

    pub fn iter_in_range(&self, page_indices: Range<usize>) -> Iter {
        Iter {
            iter: self.page_states[page_indices].iter(),
        }
    }

    pub fn iter_page_runs_in_range(
        &self,
        base_addr: PtrRepr,
        page_indices: Range<usize>,
    ) -> PageRunsIter {
        PageRunsIter {
            base_addr,
            iter: Iter {
                iter: self.page_states[page_indices].iter(),
            }
            .peekable(),
        }
    }
}

pub(crate) struct Iter<'a> {
    iter: std::slice::Iter<'a, PageStateRepr>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = PageState;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|&repr| repr.into())
    }
}

pub(crate) struct PageRunsIter<'a> {
    base_addr: PtrRepr,
    iter: Peekable<Iter<'a>>,
}

impl<'a> Iterator for PageRunsIter<'a> {
    type Item = (AddressRange, PageState);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.base_addr;
        let mut count = 1;

        let state = self.iter.next()?;

        while self
            .iter
            .next_if(|&next_state| next_state == state)
            .is_some()
        {
            count += 1;
        }

        self.base_addr += count * PAGE_SIZE;

        Some((AddressRange::new(start, count * PAGE_SIZE), state))
    }
}
