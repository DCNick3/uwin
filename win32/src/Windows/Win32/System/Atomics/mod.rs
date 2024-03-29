#![doc = r" do not edit! File auto-generated with win32-bindgen"]
#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    unused_assignments,
    clippy::all
)]
#[allow(unused)]
use win32::core::prelude::*;
pub trait Api {
    #[doc = "InterlockedCompareExchange from kernel32"]
    fn InterlockedCompareExchange(
        &self,
        destination: MutPtr<PtrRepr>,
        exchange: PtrRepr,
        comperand: PtrRepr,
    ) -> i64 {
        todo!("InterlockedCompareExchange")
    }
    #[doc = "InterlockedDecrement from kernel32"]
    fn InterlockedDecrement(&self, lp_addend: MutPtr<i64>) -> i64 {
        todo!("InterlockedDecrement")
    }
    #[doc = "InterlockedExchange from kernel32"]
    fn InterlockedExchange(&self, target: MutPtr<i64>, value: i64) -> i64 {
        todo!("InterlockedExchange")
    }
    #[doc = "InterlockedExchangeAdd from kernel32"]
    fn InterlockedExchangeAdd(&self, target: MutPtr<i64>, value: i64) -> i64 {
        todo!("InterlockedExchangeAdd")
    }
    #[doc = "InterlockedIncrement from kernel32"]
    fn InterlockedIncrement(&self, lp_addend: MutPtr<i64>) -> i64 {
        todo!("InterlockedIncrement")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
