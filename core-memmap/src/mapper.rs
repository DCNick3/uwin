use crate::os;
use crate::Result;
use core_mem::ctx::FlatMemoryCtx;
use core_mem::ptr::PtrRepr;
use memory_image::Protection;
use std::sync::Once;

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
    pub fn new() -> Result<Self> {
        // it might be possible to emulate 4K on different page sizes but needs quite some work
        assert_eq!(
            page_size(),
            crate::PAGE_SIZE,
            "Only 4K page OSes are supported"
        );

        let base = os::reserve()?;

        Ok(Self { base })
    }

    pub fn map(&self, addr: PtrRepr, size: PtrRepr, prot: Protection) -> Result<()> {
        let prot = prot & Protection::READ_WRITE;
        unsafe { os::map(self.base, addr, size, prot) }
    }

    pub fn unmap(&self, addr: PtrRepr, size: PtrRepr) -> Result<()> {
        unsafe { os::unmap(self.base, addr, size) }
    }

    pub fn protect(&self, addr: PtrRepr, size: PtrRepr, prot: Protection) -> Result<()> {
        let prot = prot & Protection::READ_WRITE;
        unsafe { os::protect(self.base, addr, size, prot) }
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

#[cfg(test)]
mod test {
    use super::Mapper;
    use memory_image::Protection;
    #[test]
    fn basic() {
        let map = Mapper::new().unwrap();
        let ctx = unsafe { map.memory_context() };

        let addr = 0x1000;
        let size = 0x2000;

        map.map(addr, size, Protection::READ_WRITE).unwrap();

        let haddr = ctx.to_native_ptr(addr);
        let haddr = unsafe { std::slice::from_raw_parts_mut(haddr, size as usize) };
        haddr[0..0x800].fill(0x12);
        assert!(haddr[0..0x800].iter().all(|&p| p == 0x12));
        assert!(haddr[0x800..].iter().all(|&p| p == 0));

        map.unmap(addr, size).unwrap();
        map.map(addr, size, Protection::READ_WRITE).unwrap();

        assert!(haddr.iter().all(|&p| p == 0));
    }
}
