use crate::context::X86Context;
use crate::unwind_token::{UnwindReason, UnwindToken};
use core_mem::conv::FromIntoMemory;
use core_mem::ctx::MemoryCtx;
use core_mem::ptr::{ConstPtr, PtrRepr};
use log::trace;
use static_assertions::assert_eq_size;

assert_eq_size!(PtrRepr, u32);

pub struct StdCallHelper<'a, MCtx: MemoryCtx, CpuCtx: X86Context> {
    mem_ctx: MCtx,
    cpu_ctx: &'a mut CpuCtx,
    /// offset of the argument to be read next
    offset: u32,
    unwind_token: UnwindToken,
    unwind_reason: &'a mut Option<UnwindReason>,
}

// TODO: THIS IS ALL WRONG, I HAVE COMMITTED ABI CRIMES AND SHOULD GO TO ABI JAIL
// The stdcall frame is as following:
// |     ...     |
// +-------------+
// | local var 2 | [ESP-8]
// +-------------+
// | local var 1 | [ESP-4]
// +-------------+ <--- ESP
// | return addr | [ESP]
// +-------------+
// | argument 1  | [ESP+4]
// +-------------+
// | argument 2  | [ESP+8]
// +-------------+
// |     ...     |
//
// we don't care about local vars, but we do obviously care about the arguments
// the first value we read is at [ESP] which is return address

impl<'a, MCtx: MemoryCtx, CpuCtx: X86Context> StdCallHelper<'a, MCtx, CpuCtx> {
    pub fn new(
        mem_ctx: MCtx,
        cpu_ctx: &'a mut CpuCtx,
        unwind_reason: &'a mut Option<UnwindReason>,
    ) -> Self {
        let esp = cpu_ctx.get_esp();
        let ret_addr = mem_ctx.read::<PtrRepr>(esp);
        Self {
            mem_ctx,
            cpu_ctx,
            offset: 4, // we start from offset 4 to skip the return address which we read previously
            unwind_token: UnwindToken::new(ret_addr),
            unwind_reason,
        }
    }

    pub fn get_arg<T: FromIntoMemory>(&mut self) -> T {
        let offset = self.offset;
        self.offset += 4;

        let size = T::size();
        assert!(
            size <= 4,
            "Size of argument is larger that 4. Dunno what ABI becomes in this case :shrug:"
        );

        let ptr = ConstPtr::<T>::new(self.cpu_ctx.get_esp() + offset); // and now we can use it to read the value
        ptr.read_with(self.mem_ctx)
    }

    pub fn finish<T: FromIntoMemory>(self, value: T) -> PtrRepr {
        let size = T::size();
        assert!(
            size <= 4,
            "Size of argument is larger that 4. Dunno what ABI becomes in this case :shrug:"
        );

        let mut bytes = [0u8; 4];
        value.into_bytes(&mut bytes);

        self.cpu_ctx.set_esp(self.cpu_ctx.get_esp() + self.offset);

        if let Some(reason) = self.unwind_token.unwind_reason() {
            trace!("Unwind requested (reason = {reason:?})");

            // Notice: in case of unwind we ignore the return value
            // maybe it's not the best way to handle it...

            *self.unwind_reason = Some(reason);

            // a special case value, usually you don't have code there ;)
            0
        } else {
            self.cpu_ctx.set_eax(u32::from_le_bytes(bytes));

            self.unwind_token.return_addr()
        }
    }

    pub fn return_address(&self) -> PtrRepr {
        self.unwind_token.return_addr()
    }

    pub fn unwind_token(&mut self) -> &mut UnwindToken {
        &mut self.unwind_token
    }
}
