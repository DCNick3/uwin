use crate::{AddressRange, MemoryManager};
use std::sync::{Arc, Mutex};

pub struct RegionHolder {
    region: AddressRange,
    mgr: Arc<Mutex<MemoryManager>>,
}

impl RegionHolder {
    pub fn new(mgr: Arc<Mutex<MemoryManager>>, region: AddressRange) -> Self {
        Self { region, mgr }
    }

    pub fn region(&self) -> AddressRange {
        self.region
    }
}

impl Drop for RegionHolder {
    fn drop(&mut self) {
        let mut mgr = self.mgr.lock().unwrap();
        mgr.uncommit_and_unreserve(self.region.start).unwrap()
    }
}
