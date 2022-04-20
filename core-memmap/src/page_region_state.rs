use crate::page_state::{PageState, PageStateRepr};
use std::ops::Range;

pub(crate) struct PageRegionState {
    page_states: Vec<PageStateRepr>,
}
impl PageRegionState {
    /// Returns length of the region in __pages__
    pub fn len(&self) -> u32 {
        self.page_states.len() as u32
    }

    pub fn get(&self, index: u32) -> PageState {
        self.page_states[index as usize].into()
    }

    pub fn set(&mut self, index: u32, state: PageState) {
        self.page_states[index as usize] = state.into()
    }

    pub fn set_range(&mut self, range: Range<u32>, state: PageState) {
        let repr = state.into();
        for i in range {
            self.page_states[i as usize] = repr;
        }
    }
}
