use crate::backend::{BoolValue, IntValue};
use crate::types::{ControlFlow, Flag, FullSizeGeneralPurposeRegister, IntType, Register};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{FunctionType, IntType as LlvmIntType, PointerType, StructType, VoidType};
use inkwell::values::{FunctionValue, IntValue as LlvmIntValue, PointerValue};
use inkwell::{AddressSpace, IntPredicate};

pub struct LlvmBuilder<'ctx, 'a> {
    context: &'ctx Context,
    module: &'a Module<'ctx>,
    function: FunctionValue<'ctx>,
    builder: Builder<'ctx>,
    types: Types<'ctx>,
    ctx_ptr: PointerValue<'ctx>,
    mem_ptr: PointerValue<'ctx>,
}

#[derive(Clone, Copy)]
pub struct Types<'ctx> {
    void: VoidType<'ctx>,
    i1: LlvmIntType<'ctx>,
    i8: LlvmIntType<'ctx>,
    i16: LlvmIntType<'ctx>,
    i32: LlvmIntType<'ctx>,
    i64: LlvmIntType<'ctx>,
    ctx: StructType<'ctx>,
    ctx_ptr: PointerType<'ctx>,
    bb_fn: FunctionType<'ctx>,
}

impl<'ctx> Types<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
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
                i8.array_type(8).into(), // general-purpose registers
            ],
            false,
        );
        let ctx_ptr = ctx.ptr_type(AddressSpace::Generic);
        let mem_ptr = i8.ptr_type(AddressSpace::Generic);

        let bb_fn = void.fn_type(
            &[
                ctx_ptr.into(),
                mem_ptr.into(), // pointer to start of guest address space (same trick as qemu does)
            ],
            false,
        );

        Self {
            void,
            i1,
            i8,
            i16,
            i32,
            i64,
            ctx,
            ctx_ptr,
            bb_fn,
        }
    }
}

impl<'ctx, 'a> LlvmBuilder<'ctx, 'a> {
    pub fn new(
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        types: Types<'ctx>,
        fn_name: &str,
    ) -> Self {
        let function = module.add_function(fn_name, types.bb_fn, None);
        let bb = context.append_basic_block(function, "entry");

        let builder = context.create_builder();
        builder.position_at_end(bb);

        let ctx_ptr = function.get_nth_param(0).unwrap().into_pointer_value();
        let mem_ptr = function.get_nth_param(1).unwrap().into_pointer_value();

        Self {
            context,
            module,
            function,
            builder,
            types,
            ctx_ptr,
            mem_ptr,
        }
    }

    pub fn get_builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    fn build_ctx_gp_gep(
        &mut self,
        ctx_ptr: PointerValue<'ctx>,
        reg: FullSizeGeneralPurposeRegister,
    ) -> PointerValue<'ctx> {
        // TODO: cache the pointers at (generated) function level
        let i32_type = self.context.i32_type();
        // SAFETY: ¯\_(ツ)_/¯
        let r = unsafe {
            self.builder.build_gep(
                ctx_ptr,
                &[
                    i32_type.const_zero(),                 // deref the pointer itself
                    i32_type.const_zero(),                 // select the gp array
                    i32_type.const_int(reg as u64, false), // then select the concrete register
                ],
                &*(format!("{:?}_ptr", reg)),
            )
        };
        debug_assert_eq!(r.get_type().get_element_type().into_int_type(), i32_type);
        r
    }

    fn build_ctx_flag_gep(
        &mut self,
        ctx_ptr: PointerValue<'ctx>,
        flag: Flag,
    ) -> PointerValue<'ctx> {
        // TODO: cache the pointers at (generated) function level
        // SAFETY: ¯\_(ツ)_/¯
        let i8_type = self.context.i8_type();
        let i32_type = self.context.i32_type();
        let r = unsafe {
            self.builder.build_gep(
                ctx_ptr,
                &[
                    i32_type.const_zero(),                                 // deref the pointer itself
                    i32_type.const_int(1, false),          // select the flags array
                    i32_type.const_int(flag as u64, false), // then select the concrete flag
                ],
                &*format!("flag_{:?}_ptr", flag),
            )
        };
        debug_assert_eq!(r.get_type().get_element_type().into_int_type(), i8_type);
        r
    }

    fn int_type(&self, ty: IntType) -> LlvmIntType<'ctx> {
        match ty {
            IntType::I8 => self.types.i8,
            IntType::I16 => self.types.i16,
            IntType::I32 => self.types.i32,
            IntType::I64 => self.types.i64,
        }
    }

    fn get_host_pointer(&mut self, target_ptr: LlvmIntValue<'ctx>) -> PointerValue<'ctx> {
        unsafe { self.builder.build_gep(self.mem_ptr, &[target_ptr], "hptr") }
    }
}

impl IntValue for LlvmIntValue<'_> {
    fn size(&self) -> IntType {
        use IntType::*;
        match self.get_type().get_bit_width() {
            8 => I8,
            16 => I16,
            32 => I32,
            64 => I64,
            _ => unreachable!(),
        }
    }
}

impl BoolValue for LlvmIntValue<'_> {

}

impl<'ctx, 'a> crate::backend::Builder for LlvmBuilder<'ctx, 'a> {
    // kinda meh that we alias them, but this way we are fine without any newtype wrappers

    // this represents i{8,16,32,64}
    type IntValue = LlvmIntValue<'ctx>;

    // this represents i1
    type BoolValue = LlvmIntValue<'ctx>;

    fn make_int_value(&self, ty: IntType, value: u64, sign_extend: bool) -> Self::IntValue {
        self.int_type(ty).const_int(value, sign_extend)
    }

    fn load_register(&mut self, register: Register) -> Self::IntValue {
        if let Ok(gp) = FullSizeGeneralPurposeRegister::try_from(register) {
            let ptr = self.build_ctx_gp_gep(self.ctx_ptr, gp);
            self.builder
                .build_load(ptr, &*format!("{:?}", gp))
                .into_int_value()
        } else {
            todo!()
        }
    }

    fn store_register(&mut self, register: Register, value: Self::IntValue) {
        if let Ok(gp) = FullSizeGeneralPurposeRegister::try_from(register) {
            let ptr = self.build_ctx_gp_gep(self.ctx_ptr, gp);
            self.builder.build_store(ptr, value);
        } else {
            todo!()
        }
    }

    fn load_flag(&mut self, flag: Flag) -> Self::BoolValue {
        let ptr = self.build_ctx_flag_gep(self.ctx_ptr, flag);
        let i8_val = self.builder.build_load(ptr, "")
            .into_int_value();

        let zero = self.make_u8(0);

        self.builder.build_int_compare(IntPredicate::NE, i8_val, zero, &*format!("{:?}", flag))
    }

    fn store_flag(&mut self, flag: Flag, value: Self::BoolValue) {
        todo!();
        //let ptr = self.build_ctx_flag_gep(self.ctx_ptr, flag);
        //self.builder.build_store(ptr, value);
    }

    fn load_memory(&mut self, size: IntType, address: Self::IntValue) -> Self::IntValue {
        let hptr = self.get_host_pointer(address);
        let hptr = self.builder.build_pointer_cast(hptr,
                                                   self.int_type(size)
                                                       .ptr_type(AddressSpace::Generic),
                                                   "");

        self.builder.build_load_aligned(hptr, 1, "").into_int_value()
    }

    fn store_memory(&mut self, address: Self::IntValue, value: Self::IntValue) {
        let hptr = self.get_host_pointer(address);
        let hptr = self.builder.build_pointer_cast(hptr,
                                                   value.get_type()
                                                       .ptr_type(AddressSpace::Generic),
                                                   "");

        self.builder.build_store_aligned(hptr, value, 1);
    }

    fn add(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_add(lhs, rhs, "")
    }

    fn int_neg(&mut self, val: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_neg(val, "")
    }

    fn bool_neg(&mut self, val: Self::BoolValue) -> Self::BoolValue { self.int_neg(val) }

    fn sub(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_sub(lhs, rhs, "")
    }

    fn mul(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_mul(lhs, rhs, "")
    }

    fn xor(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_xor(lhs, rhs, "")
    }

    fn or(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_or(lhs, rhs, "")
    }

    fn shl(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_left_shift(lhs, rhs, "")
    }

    fn lshr(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        todo!()
    }

    fn ashr(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        todo!()
    }

    fn udiv(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_unsigned_div(lhs, rhs, "")
    }

    fn zext(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue {
        self.builder.build_int_z_extend(val, self.int_type(to), "")
    }

    fn sext(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue {
        self.builder.build_int_s_extend(val, self.int_type(to), "")
    }

    fn trunc(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue {
        self.builder.build_int_truncate(val, self.int_type(to), "")
    }

    fn ifelse<T, F>(&mut self,
                    cond: Self::BoolValue,
                    iftrue: T,
                    iffalse: F)
                    -> ControlFlow<Self>
    where
        T: FnOnce(&mut Self) -> ControlFlow<Self>,
        F: FnOnce(&mut Self) -> ControlFlow<Self>
    {
        let true_bb = self.context.append_basic_block(self.function, "");
        let false_bb = self.context.append_basic_block(self.function, "");
        let cont_bb = self.context.append_basic_block(self.function, "");

        self.builder.build_conditional_branch(cond, true_bb, false_bb);

        let mut res = vec![];

        let mut handle_flow = |builder: &Builder, flow: ControlFlow<Self>| {
            match flow {
                ControlFlow::NextInstruction => {
                    builder.build_unconditional_branch(cont_bb);
                },
                ControlFlow::DirectJump(target) => {
                    // stub
                    // TODO: how do we tell the codegen that we should continue codegen here?
                    builder.build_return(None);
                },
                _ => todo!(),
            };

            if let ControlFlow::Conditional(mut cc) = flow {
                res.append(&mut cc);
            } else {
                res.push(flow);
            };
        };

        self.builder.position_at_end(true_bb);
        let left_flow = (iftrue)(self);
        handle_flow(&self.builder, left_flow);

        self.builder.position_at_end(false_bb);
        let right_flow = (iffalse)(self);
        handle_flow(&self.builder, right_flow);

        self.builder.position_at_end(cont_bb);

        return ControlFlow::Conditional(res);
    }
}
