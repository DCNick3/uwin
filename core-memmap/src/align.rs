use core_mem::ptr::PtrRepr;

#[inline]
pub fn floor(address: PtrRepr, align: PtrRepr) -> PtrRepr {
    address & !(align - 1)
}

#[inline]
pub fn ceil(address: PtrRepr, align: PtrRepr) -> PtrRepr {
    if aligned(address, align) {
        address
    } else {
        // TODO: overflow?
        address.checked_add(align - address % align).unwrap()
    }
}

#[inline]
pub fn aligned(address: PtrRepr, align: PtrRepr) -> bool {
    address % align == 0
}
