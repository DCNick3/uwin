use crate::ctx::DefaultMemoryCtx;
use std::cell::RefCell;
use std::thread_local;

thread_local! {
    pub static THREAD_CTX: RefCell<Option<DefaultMemoryCtx>> = RefCell::new(None);
}

pub fn set_thread_ctx(ctx: DefaultMemoryCtx) {
    THREAD_CTX.with(|thread_ctx| *thread_ctx.borrow_mut() = Some(ctx))
}

#[allow(clippy::clone_on_copy)]
#[inline]
pub fn get_thread_ctx() -> DefaultMemoryCtx {
    THREAD_CTX.with(|thread_ctx| thread_ctx.borrow().unwrap().clone())
}

/// # Safety
///
/// There must be a memory context set for a current thread
#[allow(clippy::clone_on_copy)]
#[inline]
pub unsafe fn get_thread_ctx_unchecked() -> DefaultMemoryCtx {
    THREAD_CTX.with(|thread_ctx| thread_ctx.borrow().unwrap_unchecked().clone())
}

#[allow(clippy::clone_on_copy)]
pub fn try_get_thread_ctx() -> Option<DefaultMemoryCtx> {
    THREAD_CTX.with(|thread_ctx| thread_ctx.borrow().clone())
}
