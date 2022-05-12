use crate::callback_token::{StdcallCallbackToken, StdcallCallbackTokenTrait};
use crate::context::{Context, X86Context};
use crate::unwind_token::UnwindToken;
use core_mem::conv::FromIntoMemory;
use core_mem::ctx::MemoryCtx;
use core_mem::ptr::{ConstPtr, PtrRepr};
use static_assertions::assert_eq_size;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use tracing::trace;

assert_eq_size!(PtrRepr, u32);

pub struct StdCalleeHelper<
    'a,
    ExtendedContext: Context<CpuContext = CpuContext>,
    CpuContext: X86Context,
    MemoryContext: MemoryCtx,
> {
    ctx: &'a mut ExtendedContext,
    mem_ctx: MemoryContext,
    /// offset of the argument to be read next
    offset: u32,
    unwind_token: RefCell<UnwindToken>,
}

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

impl<
        'a,
        ExtendedContext: Context<CpuContext = CpuContext, MemoryContext = MemoryContext>,
        CpuContext: X86Context,
        MemoryContext: MemoryCtx,
    > StdCalleeHelper<'a, ExtendedContext, CpuContext, MemoryContext>
{
    pub fn new(ctx: &'a mut ExtendedContext, mem_ctx: MemoryContext) -> Self {
        let esp = ctx.cpu_context().get_esp();
        let ret_addr = mem_ctx.read::<PtrRepr>(esp);
        Self {
            ctx,
            mem_ctx,
            offset: 4, // we start from offset 4 to skip the return address which we read previously
            unwind_token: RefCell::new(UnwindToken::new(ret_addr)),
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

        let ptr = ConstPtr::<T>::new(self.ctx.cpu_context().get_esp() + offset); // and now we can use it to read the value
        ptr.read_with(self.mem_ctx)
    }

    pub fn finish<T: FromIntoMemory>(self, value: T) -> PtrRepr {
        let size = T::size();
        assert!(
            size <= 4,
            "Size of argument is larger that 4. Dunno what ABI becomes in this case :shrug:"
        );

        let mut bytes = [0u8; 4];
        value.into_bytes(&mut bytes[..size]);

        let esp = self.ctx.cpu_context().get_esp() + self.offset;
        self.ctx.cpu_context_mut().set_esp(esp);

        if let Some(reason) = self.unwind_token.borrow().unwind_reason() {
            trace!("Unwind requested (reason = {reason:?})");

            // Notice: in case of unwind we ignore the return value
            // maybe it's not the best way to handle it...

            self.ctx.set_unwind_reason(Some(reason));

            // a special case value, usually you don't have code there ;)
            0
        } else {
            self.ctx
                .cpu_context_mut()
                .set_eax(u32::from_le_bytes(bytes));

            self.unwind_token.borrow().return_addr()
        }
    }

    pub fn return_address(&self) -> PtrRepr {
        self.unwind_token.borrow().return_addr()
    }

    pub fn unwind_token(&self) -> RefMut<UnwindToken> {
        self.unwind_token.borrow_mut()
    }

    pub fn callback_token(
        &mut self,
    ) -> StdcallCallbackToken<ExtendedContext, CpuContext, MemoryContext> {
        StdcallCallbackToken::new(self.ctx, self.mem_ctx)
    }

    pub fn context(&self) -> &ExtendedContext {
        self.ctx
    }
    pub fn context_mut(&mut self) -> &mut ExtendedContext {
        self.ctx
    }
}

pub(crate) struct StdCallerHelper<'a, Tok: StdcallCallbackTokenTrait + 'a + ?Sized> {
    token: &'a mut Tok,
    phantom: PhantomData<&'a ()>,
}

impl<'a, Tok: StdcallCallbackTokenTrait + 'a + ?Sized> StdCallerHelper<'a, Tok> {
    /// # Safety
    ///
    /// Don't forget to run .execute() method, as otherwise ABI crimes will happen
    pub unsafe fn new(token: &'a mut Tok) -> Self {
        Self {
            token,
            phantom: Default::default(),
        }
    }

    pub fn push<T: FromIntoMemory>(&mut self, value: T) {
        let size = T::size();
        assert!(
            size <= 4,
            "Size of argument is larger that 4. Dunno what ABI becomes in this case :shrug:"
        );

        let mut data = [0u8; 4];

        value.into_bytes(&mut data[..size]);

        self.token.push(data);
    }

    pub fn execute<R: FromIntoMemory>(self, target_address: PtrRepr) -> R {
        self.token.push_retaddr();
        let res = self.token.dispatch(target_address);

        let size = R::size();
        assert!(
            size <= 4,
            "Size of argument is larger that 4. Dunno what ABI becomes in this case :shrug:"
        );

        R::from_bytes(&res[..size])
    }
}
