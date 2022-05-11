use crate::context::{Context, X86Context};
use core_mem::ctx::MemoryCtx;
use core_mem::ptr::{MutPtr, PtrRepr};
use std::panic::panic_any;

const FN_PTR_RET_ADDR: PtrRepr = 0x6e667775;

pub struct StdcallCallbackToken<
    'a,
    ExtendedContext: Context<CpuContext = CpuContext>,
    CpuContext: X86Context,
    MemoryContext: MemoryCtx,
> {
    context: &'a mut ExtendedContext,
    memory_ctx: MemoryContext,
    ret_esp: Option<PtrRepr>,
}

impl<
        'a,
        ExtendedContext: Context<CpuContext = CpuContext>,
        CpuContext: X86Context,
        MemoryContext: MemoryCtx,
    > StdcallCallbackToken<'a, ExtendedContext, CpuContext, MemoryContext>
{
    pub fn new(context: &'a mut ExtendedContext, memory_ctx: MemoryContext) -> Self {
        Self {
            context,
            memory_ctx,
            ret_esp: None,
        }
    }
}

impl<
        'a,
        ExtendedContext: Context<CpuContext = CpuContext, MemoryContext = MemoryContext>,
        CpuContext: X86Context,
        MemoryContext: MemoryCtx,
    > StdcallCallbackTokenTrait
    for StdcallCallbackToken<'a, ExtendedContext, CpuContext, MemoryContext>
{
    fn push_retaddr(&mut self) {
        self.push(FN_PTR_RET_ADDR.to_le_bytes());
    }

    fn push(&mut self, value: [u8; 4]) {
        // as per stdcall, the callee must cleanup the stack
        // here we save the old esp value on first argument push to check that the esp value was properly restored by the caller
        self.ret_esp = self
            .ret_esp
            .or_else(|| Some(self.context.cpu_context().get_esp()));

        let esp = self.context.cpu_context().get_esp() - 4;
        self.context.cpu_context_mut().set_esp(esp);

        MutPtr::<u8>::new(esp).write_bytes(self.memory_ctx, &value);
    }

    fn dispatch(&mut self, address: PtrRepr) -> [u8; 4] {
        let ret_addr = self
            .context
            .execute_recompiled_code(self.memory_ctx, address);

        if let Some(reason) = &self.context.get_unwind_reason() {
            assert_eq!(ret_addr, 0);

            // well, just panic
            panic_any(reason.clone());
        } else {
            assert_eq!(ret_addr, FN_PTR_RET_ADDR);
            assert_eq!(self.context.cpu_context().get_esp(), self.ret_esp.unwrap());

            self.context.cpu_context().get_eax().to_le_bytes()
        }
    }
}

pub trait StdcallCallbackTokenTrait {
    fn push_retaddr(&mut self);
    fn push(&mut self, value: [u8; 4]);
    fn dispatch(&mut self, address: PtrRepr) -> [u8; 4];
}
