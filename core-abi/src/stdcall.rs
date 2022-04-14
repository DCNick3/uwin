use crate::context::X86Context;
use core_mem::conv::FromIntoMemory;
use core_mem::ctx::MemoryCtx;
use core_mem::ptr::{ConstPtr, PtrRepr};
use static_assertions::assert_eq_size;

assert_eq_size!(PtrRepr, u32);

pub struct StdCallHelper<'a, MCtx: MemoryCtx, CpuCtx: X86Context> {
    mem_ctx: MCtx,
    ret_addr: PtrRepr,
    cpu_ctx: &'a mut CpuCtx,
    /// offset to the last argument read (or the return addr if nothing was read yet)
    offset: u32,
}

// The stdcall frame is as following:
// |     ...     |
// +-------------+
// | local var 2 | [ESP-4]
// +-------------+
// | local var 1 | [ESP]
// +-------------+ <--- ESP
// | return addr | [ESP+4]
// +-------------+
// | argument 1  | [ESP+8]
// +-------------+
// | argument 2  | [ESP+12]
// +-------------+
// |     ...     |
//
// we don't care about local vars, but we do obviously care about the arguments
// the first value we read is at [ESP+4] which is return address
// this feels kinda weird, but [ESP] actually points to something that was not yet pushed to the stack
// and [ESP+4] gives us the last pushed value
// whatever
//
// we do the same and start offset from 4

impl<'a, MCtx: MemoryCtx, CpuCtx: X86Context> StdCallHelper<'a, MCtx, CpuCtx> {
    pub fn new(mem_ctx: MCtx, cpu_ctx: &'a mut CpuCtx) -> Self {
        let esp = cpu_ctx.get_esp();
        let ret_addr = mem_ctx.read::<PtrRepr>(esp + 4);
        Self {
            mem_ctx,
            ret_addr,
            cpu_ctx,
            offset: 4, // we start from offset 4 to skip the return address which we read previously
        }
    }

    pub fn get_arg<T: FromIntoMemory>(&mut self) -> T {
        // first increase to make esp point to the current argument (instead of the previous one)
        self.offset += 4; // we assume that every argument is stored as size 4 (is this always true?)

        let size = T::size();
        assert!(
            size <= 4,
            "Size of argument is larger that 4. Dunno what ABI becomes in this case :shrug:"
        );

        let ptr = ConstPtr::<T>::new(self.cpu_ctx.get_esp() + self.offset); // and now we can use it to read the value
        ptr.read_with(self.mem_ctx)
    }

    pub fn finish<T: FromIntoMemory>(self, value: T) {
        let size = T::size();
        assert!(
            size <= 4,
            "Size of argument is larger that 4. Dunno what ABI becomes in this case :shrug:"
        );

        let mut bytes = [0u8; 4];
        value.into_bytes(&mut bytes);

        self.cpu_ctx.set_esp(self.offset);
        self.cpu_ctx.set_eax(u32::from_le_bytes(bytes));

        // prevent the calling of the panicking destructor
        std::mem::forget(self)
    }

    pub fn return_address(&self) -> PtrRepr {
        self.ret_addr
    }
}

impl<'a, MCtx: MemoryCtx, CpuCtx: X86Context> Drop for StdCallHelper<'a, MCtx, CpuCtx> {
    fn drop(&mut self) {
        panic!("Please, call finish on the StdCallHelper to complete the call")
    }
}
