use crate::{Error, Result, PAGE_SIZE};
use libc::{
    c_void, MAP_ANON, MAP_FAILED, MAP_FIXED, MAP_PRIVATE, PROT_EXEC, PROT_NONE, PROT_READ,
    PROT_WRITE,
};
use memory_image::Protection;

#[cfg(any(target_pointer_width = "32", target_pointer_width = "16"))]
compile_error!("32 bits (or less, lol) address spaces are not supported, as we want to map a whole 4 GiB of address space");

const MAP_SIZE: usize = 1 << 32;

pub fn page_size() -> u32 {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as u32 }
}

pub fn reserve() -> Result<*const ()> {
    match unsafe {
        // should be safe to map a non-accessible region
        libc::mmap(
            std::ptr::null_mut::<c_void>(),
            MAP_SIZE,
            PROT_NONE,
            MAP_PRIVATE | MAP_ANON,
            0,
            0,
        )
    } {
        MAP_FAILED => Err(std::io::Error::last_os_error().into()),
        addr => Ok(addr as *const ()),
    }
}

/// # Safety:
///
/// The base pointer comes from the reserve()
pub unsafe fn unreserve(base: *const ()) -> Result<()> {
    match libc::munmap(base as *mut _, MAP_SIZE) {
        0 => Ok(()),
        _ => Err(std::io::Error::last_os_error().into()),
    }
}

fn prot_to_native(prot: Protection) -> libc::c_int {
    const MAPPINGS: &[(Protection, libc::c_int)] = &[
        (Protection::READ, PROT_READ),
        (Protection::WRITE, PROT_WRITE),
        (Protection::EXECUTE, PROT_EXEC),
    ];

    MAPPINGS
        .iter()
        .filter(|(flag, _)| prot & *flag == *flag)
        .fold(PROT_NONE, |acc, (_, prot)| acc | *prot)
}

fn check_addr(addr: u32) -> Result<()> {
    if addr % PAGE_SIZE != 0 {
        Err(Error::UnalignedAddress(addr))
    } else {
        Ok(())
    }
}

fn check_size(size: u32) -> Result<()> {
    if size % PAGE_SIZE != 0 {
        Err(Error::UnalignedSize(size))
    } else {
        Ok(())
    }
}

/// # Safety:
///
/// The base pointer comes from the reserve()
fn convert_region(base: *const (), addr: u32, size: u32) -> Result<(*mut c_void, usize)> {
    check_addr(addr)?;
    check_size(size)?;

    let addr = (base as *const u8).wrapping_add(addr as usize) as *mut c_void;
    let size = size as usize;

    Ok((addr, size))
}

/// # Safety:
///
/// The base pointer comes from the reserve()
pub unsafe fn map(base: *const (), addr: u32, size: u32, prot: Protection) -> Result<()> {
    let (addr, size) = convert_region(base, addr, size)?;

    // SAFETY: just mapping should be safe
    match libc::mmap(
        addr,
        size,
        prot_to_native(prot),
        MAP_PRIVATE | MAP_ANON | MAP_FIXED,
        0,
        0,
    ) {
        MAP_FAILED => Err(std::io::Error::last_os_error().into()),
        _ => Ok(()),
    }
}

/// # Safety:
///
/// The base pointer comes from the reserve()
pub unsafe fn unmap(base: *const (), addr: u32, size: u32) -> Result<()> {
    let (addr, size) = convert_region(base, addr, size)?;

    // re-map the region as non-accessible instead of unmapping to keep the reservation
    // SAFETY: Should be safe too? Works if you don't do stupid stuff with the pointers, which is itself unsafe..
    match libc::mmap(
        addr,
        size,
        PROT_NONE,
        MAP_PRIVATE | MAP_ANON | MAP_FIXED,
        0,
        0,
    ) {
        MAP_FAILED => Err(std::io::Error::last_os_error().into()),
        _ => Ok(()),
    }
}

pub unsafe fn protect(base: *const (), addr: u32, size: u32, protection: Protection) -> Result<()> {
    let (addr, size) = convert_region(base, addr, size)?;

    match libc::mprotect(addr, size, prot_to_native(protection)) {
        0 => Ok(()),
        _ => Err(std::io::Error::last_os_error().into()),
    }
}
