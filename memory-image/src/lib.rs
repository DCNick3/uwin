use bitflags::bitflags;
use std::slice::Iter;

bitflags! {
  #[derive(Default)]
  pub struct Protection: usize {
    /// No access allowed at all.
    const NONE = 0;
    /// Read access; writing and/or executing data will panic.
    const READ = (1 << 1);
    /// Write access; this flag alone may not be supported on all OSs.
    const WRITE = (1 << 2);
    /// Execute access; this may not be allowed depending on DEP.
    const EXECUTE = (1 << 3);
    /// Read and execute shorthand.
    const READ_EXECUTE = (Self::READ.bits | Self::EXECUTE.bits);
    /// Read and write shorthand.
    const READ_WRITE = (Self::READ.bits | Self::WRITE.bits);
    /// Read, write and execute shorthand.
    const READ_WRITE_EXECUTE = (Self::READ.bits | Self::WRITE.bits | Self::EXECUTE.bits);
    /// Write and execute shorthand.
    const WRITE_EXECUTE = (Self::WRITE.bits | Self::EXECUTE.bits);
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryImageItem {
    pub addr: u32,
    pub protection: Protection,
    pub data: Vec<u8>,
}

impl MemoryImageItem {
    pub fn new(addr: u32, protection: Protection, data: Vec<u8>) -> Self {
        Self {
            addr,
            protection,
            data,
        }
    }

    pub fn contains(&self, addr: u32) -> bool {
        self.addr <= addr && addr < self.end()
    }

    pub fn end(&self) -> u32 {
        self.addr
            .checked_add(self.data.len() as u32)
            .expect("The end of the region is out of bounds")
    }

    pub fn intersects(&self, other: &MemoryImageItem) -> bool {
        std::cmp::max(self.addr, other.addr) < std::cmp::min(self.end(), other.end())
    }
}

/// Represents a executable image
/// Is implemented as a collection of memory regions & references to their contents
pub struct MemoryImage {
    regions: Vec<MemoryImageItem>,
}

impl MemoryImage {
    pub fn from_code_region(address: u32, contents: &[u8]) -> Self {
        Self {
            regions: vec![MemoryImageItem::new(
                address,
                Protection::READ_EXECUTE,
                contents.to_vec(),
            )],
        }
    }
}

impl<'a> FromIterator<&'a MemoryImageItem> for MemoryImage {
    fn from_iter<T: IntoIterator<Item = &'a MemoryImageItem>>(iter: T) -> Self {
        Self {
            regions: iter.into_iter().cloned().collect(),
        }
    }
}

impl MemoryImage {
    // TODO: validate that we have no intersecting regions
    pub fn new() -> Self {
        MemoryImage {
            regions: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &MemoryImageItem> {
        self.regions.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut MemoryImageItem> {
        self.regions.iter_mut()
    }

    fn find_region(&self, addr: u32) -> Option<&MemoryImageItem> {
        // TODO: this may be made more optimal, but we may as well not care =)
        self.iter().find(|item| item.contains(addr))
    }

    fn find_region_mut(&mut self, addr: u32) -> Option<&mut MemoryImageItem> {
        self.iter_mut().find(|item| item.contains(addr))
    }

    // TODO: maybe we want to merge the regions that are next to each other
    // this is kinda corner-case, but it will not allow us to recompile code between memory rg boundaries as of now
    fn access_all_at_prot(&self, access_addr: u32, required_prot: Protection) -> &[u8] {
        self.find_region(access_addr)
            .filter(|item| item.protection.contains(required_prot))
            .map(|item| &item.data[(access_addr - item.addr) as usize..])
            .unwrap_or(&[])
    }

    /// Get slice containing data from the specified address
    ///
    /// Returns an empty slice if the protection doesn't have READ flag
    pub fn read_all_at(&self, addr: u32) -> &[u8] {
        self.access_all_at_prot(addr, Protection::READ)
    }

    /// Get slice containing data from the specified address
    ///
    /// Returns an empty slice if the protection doesn't have EXECUTE flag
    pub fn execute_all_at(&self, addr: u32) -> &[u8] {
        self.access_all_at_prot(addr, Protection::EXECUTE)
    }

    /// Get slice containing data from the specified address
    ///
    /// Does not perform any protection checks
    pub fn access_all_at(&self, addr: u32) -> &[u8] {
        self.find_region(addr)
            .map(|item| &item.data[(addr - item.addr) as usize..])
            .unwrap_or(&[])
    }

    /// Get mutable slice containing data from the specified address
    ///
    /// Does not perform any protection checks
    pub fn modify_all_at(&mut self, addr: u32) -> &mut [u8] {
        self.find_region_mut(addr)
            .map(|item| &mut item.data[(addr - item.addr) as usize..])
            .unwrap_or(&mut [])
    }

    pub fn push(&mut self, value: MemoryImageItem) {
        assert!(!self.iter().any(|region| region.intersects(&value)));
        self.regions.push(value)
    }

    pub fn add_region(&mut self, base_addr: u32, prot: Protection, data: Vec<u8>) {
        self.push(MemoryImageItem::new(base_addr, prot, data))
    }

    pub fn add_zeroed_region(&mut self, base_addr: u32, prot: Protection, len: u32) {
        self.add_region(base_addr, prot, vec![0; len as usize])
    }
}

impl Default for MemoryImage {
    fn default() -> Self {
        MemoryImage::new()
    }
}

#[cfg(test)]
mod tests {
    use super::MemoryImage;
    use super::MemoryImageItem;
    use crate::Protection;

    fn readonly_image() -> MemoryImage {
        [
            MemoryImageItem::new(0, Protection::READ, vec![1, 2, 3]),
            MemoryImageItem::new(5, Protection::READ, vec![5, 6, 7]),
            MemoryImageItem::new(8, Protection::READ, vec![8]),
        ]
        .iter()
        .collect()
    }

    // fn read_execute_image() -> MemoryImage {
    //     [
    //         MemoryImageItem::new(0, Protection::READ_EXECUTE, vec![1, 2, 3]),
    //         MemoryImageItem::new(5, Protection::READ_EXECUTE, vec![5, 6, 7]),
    //         MemoryImageItem::new(8, Protection::READ_EXECUTE, vec![8]),
    //     ]
    //     .iter()
    //     .collect()
    // }

    #[test]
    #[rustfmt::skip]
    fn iter() {
        let image: MemoryImage = readonly_image();

        let image_vec: Vec<MemoryImageItem> = image.iter().cloned().collect();

        assert_eq!(*image_vec.as_slice(), [
            MemoryImageItem::new(0, Protection::READ, vec![1, 2, 3]),
            MemoryImageItem::new(5, Protection::READ, vec![5, 6, 7]),
            MemoryImageItem::new(8, Protection::READ, vec![8])
        ]);
    }

    #[test]
    #[rustfmt::skip]
    fn access() {
        let image: MemoryImage = readonly_image();

        assert_eq!(*image.read_all_at(0), [1, 2, 3]);
        assert_eq!(*image.read_all_at(1), [2, 3]);
        assert_eq!(*image.read_all_at(2), [3]);
        assert_eq!(*image.read_all_at(3), []);
        assert_eq!(*image.read_all_at(4), []);
        assert_eq!(*image.read_all_at(5), [5, 6, 7]);
        assert_eq!(*image.read_all_at(6), [6, 7]);
        assert_eq!(*image.read_all_at(7), [7]);
        assert_eq!(*image.read_all_at(8), [8]);
        assert_eq!(*image.read_all_at(9), []);
        assert_eq!(*image.read_all_at(10), []);
    }

    #[test]
    #[rustfmt::skip]
    fn access_bad() {
        let image: MemoryImage = readonly_image();

        assert_eq!(*image.execute_all_at(0), []);
        assert_eq!(*image.execute_all_at(1), []);
        assert_eq!(*image.execute_all_at(2), []);
        assert_eq!(*image.execute_all_at(3), []);
        assert_eq!(*image.execute_all_at(4), []);
        assert_eq!(*image.execute_all_at(5), []);
        assert_eq!(*image.execute_all_at(6), []);
        assert_eq!(*image.execute_all_at(7), []);
        assert_eq!(*image.execute_all_at(8), []);
        assert_eq!(*image.execute_all_at(9), []);
        assert_eq!(*image.execute_all_at(10), []);
    }

    #[test]
    #[rustfmt::skip]
    fn from_region() {
        let image = MemoryImage::from_code_region(13, &[1, 2, 3]);

        assert_eq!(*image.read_all_at(12), []);
        assert_eq!(*image.read_all_at(13), [1, 2, 3]);
        assert_eq!(*image.read_all_at(14), [2, 3]);
        assert_eq!(*image.read_all_at(15), [3]);
        assert_eq!(*image.read_all_at(16), []);
        assert_eq!(*image.read_all_at(17), []);
    }

    #[test]
    #[rustfmt::skip]
    fn modify_region() {
        let mut image = readonly_image();
        let rg = image.modify_all_at(6);
        rg[0] = 12;
        rg[1] = 13;
        
        assert_eq!(*image.read_all_at(4), []);
        assert_eq!(*image.read_all_at(5), [5, 12, 13]);
        assert_eq!(*image.read_all_at(6), [12, 13]);
        assert_eq!(*image.read_all_at(7), [13]);
        assert_eq!(*image.read_all_at(8), [8]);
    }
}
