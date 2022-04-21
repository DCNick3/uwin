use core_mem::ptr::PtrRepr;

#[inline]
pub fn floor(value: PtrRepr, align: PtrRepr) -> PtrRepr {
    value & !(align - 1)
}

#[inline]
pub fn ceil(value: PtrRepr, align: PtrRepr) -> PtrRepr {
    if aligned(value, align) {
        value
    } else {
        // TODO: overflow?
        value.checked_add(align - value % align).unwrap()
    }
}

#[inline]
pub fn aligned(value: PtrRepr, align: PtrRepr) -> bool {
    value % align == 0
}
