use crate::align::aligned;
use crate::page_state::{PageState, PageStateRepr};
use crate::PAGE_SIZE;
use core_mem::ptr::PtrRepr;
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
