use crate::{DynRustyExecutor, ExtendedContext};
use core_abi::callback_token::StdcallCallbackToken;
use core_abi::context::X86Context;
use core_mem::ctx::DefaultMemoryCtx;
use core_mem::ptr::{MutPtr, PtrRepr};
use std::panic::panic_any;

const FN_PTR_RET_ADDR: PtrRepr = 0x6e667775;

pub struct RustyX86CallbackToken<'a> {
    executor: &'a DynRustyExecutor,
    context: &'a mut ExtendedContext,
    memory_ctx: DefaultMemoryCtx,
    ret_esp: PtrRepr,
}

impl<'a> RustyX86CallbackToken<'a> {
    pub fn new(
        executor: &'a DynRustyExecutor,
        context: &'a mut ExtendedContext,
        memory_ctx: DefaultMemoryCtx,
    ) -> Self {
        Self {
            executor,
            context,
            memory_ctx,
            ret_esp: 0,
        }
    }
}

impl<'a> StdcallCallbackToken for RustyX86CallbackToken<'a> {
    fn push_retaddr(&mut self) {
        // as per stdcall, the callee must cleanup the stack
        // here we save the old esp value to check that the esp value was properly restored by the caller
        self.ret_esp = self.context.cpu.get_esp();

        self.push(FN_PTR_RET_ADDR.to_le_bytes());
    }

    fn push(&mut self, value: [u8; 4]) {
        let esp = self.context.cpu.get_esp() - 4;
        self.context.cpu.set_esp(esp);

        MutPtr::<u8>::new(esp).write_bytes(self.memory_ctx, &value);
    }

    fn dispatch(&mut self, address: PtrRepr) -> [u8; 4] {
        let ret_addr =
            self.executor
                .execute_recompiled_code(self.context, self.memory_ctx, address);

        if let Some(reason) = &self.context.unwind_reason {
            assert_eq!(ret_addr, 0);

            // well, just panic
            panic_any(reason.clone());
        } else {
            assert_eq!(ret_addr, FN_PTR_RET_ADDR);
            assert_eq!(self.context.cpu.get_esp(), self.ret_esp);

            let res = self.context.cpu.get_eax().to_le_bytes();

            return res;
        }
    }
}
