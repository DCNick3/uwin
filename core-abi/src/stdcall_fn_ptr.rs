use crate::callback_token::StdcallCallbackToken;
use crate::stdcall::StdCallerHelper;
use core_mem::conv::FromIntoMemory;
use core_mem::ptr::PtrRepr;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

pub struct StdCallFnPtr<ParamTy, RetTy> {
    ptr: PtrRepr,
    phantom: PhantomData<(ParamTy, RetTy)>,
}

impl<ParamTy, RetTy> StdCallFnPtr<ParamTy, RetTy> {
    pub fn new(repr: PtrRepr) -> Self {
        Self {
            ptr: repr,
            phantom: Default::default(),
        }
    }
}

impl<ParamTy, RetTy> Clone for StdCallFnPtr<ParamTy, RetTy> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            phantom: Default::default(),
        }
    }
}

impl<R: FromIntoMemory> StdCallFnPtr<(), R> {
    pub fn call<'a, Tok: StdcallCallbackToken + 'a>(&self, token: Tok) -> R {
        let call = unsafe { StdCallerHelper::new(token) };

        call.execute::<R>(self.ptr)
    }
}

impl<T1: FromIntoMemory, T2: FromIntoMemory, T3: FromIntoMemory, R: FromIntoMemory>
    StdCallFnPtr<(T1, T2, T3), R>
{
    pub fn call<'a, Tok: StdcallCallbackToken + 'a>(
        &self,
        token: Tok,
        arg1: T1,
        arg2: T2,
        arg3: T3,
    ) -> R {
        let mut call = unsafe { StdCallerHelper::new(token) };

        call.push(arg3);
        call.push(arg2);
        call.push(arg1);
        call.execute::<R>(self.ptr)
    }
}

impl<ParamTy, RetTy> Copy for StdCallFnPtr<ParamTy, RetTy> {}

impl<ParamTy, RetTy> FromIntoMemory for StdCallFnPtr<ParamTy, RetTy> {
    fn from_bytes(from: &[u8]) -> Self {
        Self {
            ptr: FromIntoMemory::from_bytes(from),
            phantom: Default::default(),
        }
    }

    fn into_bytes(self, into: &mut [u8]) {
        self.ptr.into_bytes(into)
    }

    fn size() -> usize {
        PtrRepr::size()
    }
}

impl<ParamTy, RetTy> PartialEq for StdCallFnPtr<ParamTy, RetTy> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<ParamTy, RetTy> Eq for StdCallFnPtr<ParamTy, RetTy> {}

impl<ParamTy, RetTy> Debug for StdCallFnPtr<ParamTy, RetTy> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FnPtr {:#010x}", self.ptr)
    }
}
