use crate::address_range::AddressRange;
use crate::os;
use core_mem::ctx::FlatMemoryCtx;
use core_mem::ptr::PtrRepr;
use memory_image::Protection;
use std::sync::Once;
use thiserror::Error;

/// A wrapper around native APIs that implements the mappings for FlatMemoryCtx
pub(crate) struct Mapper {
    base: *const (),
}

#[inline]
pub fn page_size() -> PtrRepr {
    static INIT: Once = Once::new();
    static mut PAGE_SIZE: PtrRepr = 0;

    unsafe {
        INIT.call_once(|| PAGE_SIZE = os::page_size());
        PAGE_SIZE
    }
}

impl Mapper {
    pub fn new() -> MapperResult<Self> {
        // it might be possible to emulate 4K on different page sizes but needs quite some work
        assert_eq!(
            page_size(),
            crate::PAGE_SIZE,
            "Only 4K page OSes are supported"
        );

        let base = os::reserve()?;

        Ok(Self { base })
    }

    /// Map page range in simulated address space with specified protection
    ///
    /// Protection::EXECUTE bit is ignored
    ///
    /// Don't map an already mapped region; semantics on windows and linux are different.
    /// If you want to remap (clean the page and stuff) - unmap and map again, if you want the mapping to stay the same - don't do anything  
    pub fn map(&self, range: AddressRange, prot: Protection) -> MapperResult<()> {
        let prot = prot & Protection::READ_WRITE;
        unsafe { os::map(self.base, range.start, range.size, prot) }
    }

    /// Unmap page range in simulated address space with specified protection
    ///
    /// Don't unmap already unmapped page, the results are not specified in this case
    pub fn unmap(&self, range: AddressRange) -> MapperResult<()> {
        unsafe { os::unmap(self.base, range.start, range.size) }
    }

    /// Change page range protection in simulated address space to specified protection
    ///
    /// Protection::EXECUTE bit is ignored
    ///
    /// Don't change protection on unmapped pages, the results are not specified in this case
    pub fn protect(&self, range: AddressRange, prot: Protection) -> MapperResult<()> {
        let prot = prot & Protection::READ_WRITE;
        unsafe { os::protect(self.base, range.start, range.size, prot) }
    }

    /// # Safety
    ///
    /// You must ensure that lifetime of FlatMemoryCtx will not outlive the lifetime of Mapper
    /// (Rust lifetimes not used here because it's hard to statically verify lifetime of every place where memory context is used)
    pub unsafe fn memory_context(&self) -> FlatMemoryCtx {
        FlatMemoryCtx::new(self.base as *mut u8)
    }
}

impl Drop for Mapper {
    fn drop(&mut self) {
        // SAFETY: Mapper can only be created through Mapper::new(), which sets base to result of os::reserve()
        unsafe { os::unreserve(self.base) }.unwrap() // well, this _shouldn't_ fail (but technically may, lol)
    }
}

#[derive(Debug, Error)]
pub enum MapperError {
    #[error("Unaligned size: {0:#08x}")]
    UnalignedSize(PtrRepr),
    #[error("Unaligned address: {0:#08x}")]
    UnalignedAddress(PtrRepr),

    #[error("A system call failed: {0}")]
    SystemCall(#[from] std::io::Error),
    #[error("macOS kernel call failed: {0}")]
    MachCall(libc::c_int),
}

pub type MapperResult<T> = std::result::Result<T, MapperError>;

#[cfg(test)]
mod test {
    use super::Mapper;
    use crate::address_range::AddressRange;
    use memory_image::Protection;

    #[test]
    fn basic() {
        let map = Mapper::new().unwrap();
        let ctx = unsafe { map.memory_context() };

        let range = AddressRange::new(0x1000, 0x2000);

        map.map(range, Protection::READ_WRITE).unwrap();

        let haddr = ctx.to_native_ptr(range.start);
        let haddr = unsafe { std::slice::from_raw_parts_mut(haddr, range.size as usize) };
        haddr[0..0x800].fill(0x12);
        assert!(haddr[0..0x800].iter().all(|&p| p == 0x12));
        assert!(haddr[0x800..].iter().all(|&p| p == 0));

        map.unmap(range).unwrap();
        map.map(range, Protection::READ_WRITE).unwrap();

        assert!(haddr.iter().all(|&p| p == 0));
    }
}
