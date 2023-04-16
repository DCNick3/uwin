use inkwell::context::Context;
use inkwell::types::{FunctionType, IntType as LlvmIntType, PointerType, StructType, VoidType};
use inkwell::AddressSpace;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct Types<'ctx> {
    #[allow(unused)]
    pub void: VoidType<'ctx>,
    pub i1: LlvmIntType<'ctx>,
    pub i8: LlvmIntType<'ctx>,
    pub i16: LlvmIntType<'ctx>,
    pub i32: LlvmIntType<'ctx>,
    pub i64: LlvmIntType<'ctx>,
    #[allow(unused)]
    pub ctx: StructType<'ctx>,
    #[allow(unused)]
    pub ctx_ptr: PointerType<'ctx>,
    pub mem_ptr: PointerType<'ctx>,

    pub bb_fn: FunctionType<'ctx>, // ctx: Context*, mem: u8* -> u32
    pub indirect_bb_call: FunctionType<'ctx>, // ctx: Context*, mem: u8*, eip: u32 -> u32
    pub find_thunk_fn: FunctionType<'ctx>, // thunk_id: u32 -> bb_fn
}

impl<'ctx> Types<'ctx> {
    pub fn new(context: &'ctx Context) -> Arc<Self> {
        let void = context.void_type();

        let i1 = context.bool_type();
        let i8 = context.i8_type();
        let i16 = context.i16_type();
        let i32 = context.i32_type();
        let i64 = context.i64_type();

        let ctx = context.opaque_struct_type("context");
        ctx.set_body(
            &[
                i32.array_type(8).into(), // general-purpose registers
                i8.array_type(8).into(),  // general-purpose registers
                i32.into(),               // fs base
                i32.into(),               // gs base
            ],
            false,
        );
        let ctx_ptr = ctx.ptr_type(AddressSpace::default());
        let mem_ptr = i8.ptr_type(AddressSpace::default());

        let bb_fn = i32.fn_type(
            &[
                ctx_ptr.into(), // ctx
                mem_ptr.into(), // mem - pointer to start of guest address space (same trick as qemu does)
            ],
            false,
        );

        let rt_indirect_bb_call = i32.fn_type(
            &[
                ctx_ptr.into(), // ctx
                mem_ptr.into(), // mem
                i32.into(),     // eip
            ],
            false,
        );

        let find_thunk_fn = bb_fn
            .ptr_type(AddressSpace::default())
            .fn_type(&[i32.into()], false);

        Arc::new(Self {
            void,
            i1,
            i8,
            i16,
            i32,
            i64,
            ctx,
            ctx_ptr,
            mem_ptr,

            bb_fn,
            indirect_bb_call: rt_indirect_bb_call,
            find_thunk_fn,
        })
    }
}
