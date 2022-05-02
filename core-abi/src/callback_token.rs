use core_mem::ptr::PtrRepr;

pub trait StdcallCallbackToken {
    fn push_retaddr(&mut self);
    fn push(&mut self, value: [u8; 4]);
    fn dispatch(&mut self, address: PtrRepr) -> [u8; 4];
}
