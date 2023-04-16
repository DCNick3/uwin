mod intrinsics;
mod prologue_builder;
mod runtime_helpers;
mod types;

pub use intrinsics::Intrinsics;
pub use runtime_helpers::RuntimeHelpers;
pub use types::Types;

use std::collections::BTreeMap;
use std::sync::Arc;

use inkwell::attributes::{Attribute, AttributeLoc};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::intrinsics::Intrinsic;
use inkwell::module::{Linkage, Module};
use inkwell::types::IntType as LlvmIntType;
use inkwell::values::{
    BasicMetadataValueEnum, BasicValue, CallSiteValue, FunctionValue, IntValue as LlvmIntValue,
    PointerValue,
};
use inkwell::{AddressSpace, IntPredicate};

use crate::backend::{BoolValue, ComparisonType, IntValue};
use crate::llvm::backend::prologue_builder::PrologueBuilder;
use crate::types::{Flag, FullSizeGeneralPurposeRegister, IntType, Register, SegmentRegister};
use crate::{Builder as BackendBuilder, ControlFlow};

pub struct LlvmBuilder<'ctx, 'a> {
    context: &'ctx Context,
    module: &'a Module<'ctx>,
    function: FunctionValue<'ctx>,
    builder: Builder<'ctx>,
    types: Arc<Types<'ctx>>,
    intrinsics: Intrinsics,
    ctx_ptr: PointerValue<'ctx>,
    mem_ptr: PointerValue<'ctx>,

    /// PrologueBuilder is what allocates local variables for registers and flags
    /// and dumps them back to the context before returning or calling a function
    ///
    /// Therefore all access to cpu state should go through it
    prologue_builder: PrologueBuilder<'ctx>,

    // this function should dispatch execution to a bb with address computed in runtime
    indirect_bb_call: FunctionValue<'ctx>,
    // this is for functions to be implemented by a runtime
    #[allow(unused)]
    rt_funs: &'a RuntimeHelpers<'ctx>,
    thunk_functions: &'a BTreeMap<u32, String>,
}

pub const FASTCC_CALLING_CONVENTION: u32 = 8;

impl<'ctx, 'a> LlvmBuilder<'ctx, 'a> {
    pub fn new(
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        types: Arc<Types<'ctx>>,
        rt_funs: &'a RuntimeHelpers<'ctx>,
        thunk_functions: &'a BTreeMap<u32, String>,
        indirect_bb_call: FunctionValue<'ctx>,
        basic_block_addr: u32,
    ) -> Self {
        let function =
            Self::get_basic_block_fun_internal(context, module, types.clone(), basic_block_addr);

        // prologue_bb (allocate variables) -> first_bb (actual code)

        let (mut prologue_builder, prologue_bb) =
            PrologueBuilder::new(context, types.clone(), function);

        let first_bb =
            context.append_basic_block(function, Self::get_name_for_bb(basic_block_addr).as_str());
        prologue_builder.set_first_block(first_bb);

        let builder = context.create_builder();
        builder.position_at_end(first_bb);

        let intrinsics = Intrinsics::new();

        let ctx_ptr = function.get_nth_param(0).unwrap().into_pointer_value();
        let mem_ptr = function.get_nth_param(1).unwrap().into_pointer_value();

        Self {
            context,
            module,
            function,
            builder,
            types,
            intrinsics,
            ctx_ptr,
            mem_ptr,

            prologue_builder,

            indirect_bb_call,
            rt_funs,
            thunk_functions,
        }
    }

    pub fn get_raw_builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    pub fn get_function(&self) -> FunctionValue<'ctx> {
        self.function
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
        let target_ptr_ext = self
            .builder
            .build_int_z_extend(target_ptr, self.types.i64, "");

        unsafe {
            self.builder
                .build_gep(self.types.i8, self.mem_ptr, &[target_ptr_ext], "hptr")
        }
    }

    // TODO: name map (use symbols to derive recompiled function names to ease debugging)
    pub fn get_name_for_sub(addr: u32) -> String {
        format!("bb_sub_{:08x}", addr)
    }

    pub fn get_name_for_bb(addr: u32) -> String {
        format!("bb_{:08x}", addr)
    }

    fn get_basic_block_fun_internal(
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        types: Arc<Types<'ctx>>,
        addr: u32,
    ) -> FunctionValue<'ctx> {
        let name = Self::get_name_for_sub(addr);
        if let Some(fun) = module.get_function(name.as_str()) {
            fun
        } else {
            let res = module.add_function(name.as_str(), types.bb_fn, Some(Linkage::Internal));
            let noalias = Attribute::get_named_enum_kind_id("noalias"); // TODO: cache it somewhere or smth
            assert_ne!(noalias, 0);

            // set noalias on the context pointer and the memory pointer
            res.add_attribute(
                AttributeLoc::Param(0),
                context.create_enum_attribute(noalias, 0),
            );
            res.add_attribute(
                AttributeLoc::Param(1),
                context.create_enum_attribute(noalias, 0),
            );
            res.set_call_conventions(FASTCC_CALLING_CONVENTION);
            // TODO: I really want to attach metadata telling that this a basic block function and it's (original) address
            res
        }
    }

    pub fn get_basic_block_fun(&mut self, addr: u32) -> FunctionValue<'ctx> {
        Self::get_basic_block_fun_internal(self.context, self.module, self.types.clone(), addr)
    }

    fn fastcall(
        &mut self,
        target: FunctionValue<'ctx>,
        args: &[BasicMetadataValueEnum<'ctx>],
    ) -> CallSiteValue<'ctx> {
        let call = self.builder.build_call(target, args, "");
        call.set_call_convention(FASTCC_CALLING_CONVENTION);
        call
    }

    pub fn call_basic_block(&mut self, target: u32, tail_call: bool) -> LlvmIntValue<'ctx> {
        todo!("Dump the register state before calling another basic block");

        let target = self.get_basic_block_fun(target);
        let args = &[self.ctx_ptr.into(), self.mem_ptr.into()];
        let call = self.fastcall(target, args);
        call.set_tail_call(tail_call);
        call.try_as_basic_value().unwrap_left().into_int_value()
    }

    pub fn call_basic_block_indirect(
        &mut self,
        target: LlvmIntValue<'ctx>,
        tail_call: bool,
    ) -> LlvmIntValue<'ctx> {
        todo!("Dump the register state before calling another basic block");
        let args = &[self.ctx_ptr.into(), self.mem_ptr.into(), target.into()];
        let call = self.fastcall(self.indirect_bb_call, args);
        call.set_tail_call(tail_call);
        call.try_as_basic_value().unwrap_left().into_int_value()
    }

    pub fn find_thunk_function(&mut self, index: u32) -> FunctionValue<'ctx> {
        let name = self
            .thunk_functions
            .get(&index)
            .expect("Call to unknown thunk function");
        let thunk_name = format!("thunk_{}", name);

        if let Some(function) = self.module.get_function(&thunk_name) {
            function
        } else {
            self.module
                .add_function(&thunk_name, self.types.bb_fn, Some(Linkage::External))
        }
    }

    pub fn handle_flow(
        &mut self,
        next_ip: u32,
        flow: ControlFlow<<Self as BackendBuilder>::IntValue, <Self as BackendBuilder>::IntValue>,
    ) -> Option<LlvmIntValue<'ctx>> {
        match flow {
            ControlFlow::NextInstruction => {
                // create a new basic block for ease of reading ir
                let next_bb = self
                    .context
                    .append_basic_block(self.function, Self::get_name_for_bb(next_ip).as_str());

                self.builder.build_unconditional_branch(next_bb);
                self.builder.position_at_end(next_bb);

                None
            }
            ControlFlow::DirectJump(addr) => {
                Some(self.call_basic_block(addr, true))
                // no need for ret; all recompiled funs are terminated with ret
            }
            ControlFlow::IndirectJump(addr) => Some(self.call_basic_block_indirect(addr, true)),
            ControlFlow::CallCheck(addr) => {
                let ne = self.icmp(ComparisonType::NotEqual, addr, self.make_u32(next_ip));

                let check_fail = self.context.append_basic_block(
                    self.function,
                    format!("callcheck_fail_{:08x}", next_ip).as_str(),
                );

                let next_bb = self
                    .context
                    .append_basic_block(self.function, Self::get_name_for_bb(next_ip).as_str());

                self.builder
                    .build_conditional_branch(ne, check_fail, next_bb);

                self.builder.position_at_end(check_fail);
                self.builder.build_return(Some(&addr)); // TODO: this is not correct

                self.builder.position_at_end(next_bb);

                None
            }
            ControlFlow::Return(addr) => {
                // no need for ret; all recompiled funs are terminated with ret
                Some(addr)
            }
            ControlFlow::Conditional(cond, target) => {
                let branch_to = self
                    .context
                    .append_basic_block(self.function, format!("br_to_{:08x}", target).as_str());

                let next_bb = self
                    .context
                    .append_basic_block(self.function, Self::get_name_for_bb(next_ip).as_str());

                self.builder
                    .build_conditional_branch(cond, branch_to, next_bb);

                self.builder.position_at_end(branch_to);
                let res = self.call_basic_block(target, true);
                todo!("Dump the register state before returning");
                self.builder.build_return(Some(&res));

                self.builder.position_at_end(next_bb);

                None
            }
        }
    }

    fn call_binary_overflow_intrinsic(
        &mut self,
        intrinsic: Intrinsic,
        lhs: LlvmIntValue<'ctx>,
        rhs: LlvmIntValue<'ctx>,
    ) -> LlvmIntValue<'ctx> {
        let usub = intrinsic
            .get_declaration(self.module, &[lhs.get_type().into()])
            .unwrap();

        let r = self
            .builder
            .build_call(usub, &[lhs.into(), rhs.into()], "")
            .try_as_basic_value()
            .unwrap_left()
            .into_struct_value();
        return self
            .builder
            .build_extract_value(r, 1, "")
            .unwrap()
            .into_int_value();
    }

    /// Dump the cpu context to the ctx ptr and return from the function
    pub fn finish_bb_sub(mut self, return_value: LlvmIntValue<'ctx>) {
        self.prologue_builder.dump(&mut self.builder);
        self.builder.build_return(Some(&return_value));
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

impl BoolValue for LlvmIntValue<'_> {}

impl From<ComparisonType> for IntPredicate {
    fn from(comp: ComparisonType) -> Self {
        use ComparisonType::*;
        use IntPredicate::*;
        match comp {
            Equal => EQ,
            NotEqual => NE,
            UnsignedGreater => UGT,
            UnsignedGreaterOrEqual => UGE,
            UnsignedLess => ULT,
            UnsignedLessOrEqual => ULE,
            SignedGreater => SGT,
            SignedGreaterOrEqual => SGE,
            SignedLess => SLT,
            SignedLessOrEqual => SLE,
        }
    }
}

impl<'ctx, 'a> crate::backend::Builder for LlvmBuilder<'ctx, 'a> {
    // kinda meh that we alias them, but this way we are fine without any newtype wrappers

    // this represents i{8,16,32,64}
    type IntValue = LlvmIntValue<'ctx>;

    // this represents i1
    type BoolValue = LlvmIntValue<'ctx>;

    fn make_int_value(&self, ty: IntType, value: u64) -> Self::IntValue {
        self.int_type(ty).const_int(value, false)
    }

    fn make_true(&self) -> Self::BoolValue {
        self.types.i1.const_int(1, false)
    }

    fn make_false(&self) -> Self::BoolValue {
        self.types.i1.const_int(0, false)
    }

    fn load_segment_base(&mut self, segment: SegmentRegister) -> Self::IntValue {
        use SegmentRegister::*;
        match segment {
            CS | DS | ES | SS => self.make_u32(0),
            FS => self.prologue_builder.get_fs_base(),
            GS => self.prologue_builder.get_gs_base(),
        }
    }

    fn load_register(&mut self, register: Register) -> Self::IntValue {
        let base = register.base_register();
        let base_ptr = self.prologue_builder.get_gp_ptr(base);
        let mut base_val = self
            .builder
            .build_load(self.types.i32, base_ptr, &*format!("{:?}", base))
            .into_int_value();

        if FullSizeGeneralPurposeRegister::try_from(register).is_ok() {
            base_val
        } else {
            if register.is_hi_reg() {
                base_val = self.lshr(base_val, self.make_u32(8));
            }
            self.builder.build_int_truncate(
                base_val,
                self.int_type(register.size()),
                &*format!("{:?}", register),
            )
        }
    }

    fn store_register(&mut self, register: Register, value: Self::IntValue) {
        assert_eq!(register.size(), IntValue::size(&value));

        let base = register.base_register();
        let base_ptr = self.prologue_builder.get_gp_ptr(base);

        if FullSizeGeneralPurposeRegister::try_from(register).is_ok() {
            self.builder.build_store(base_ptr, value);
        } else {
            // ehh, this is kinda ugly. Maybe we can index directly into the base value? how 'bout aliasing?
            let base_val = self
                .builder
                .build_load(self.types.i32, base_ptr, &*format!("{:?}", base))
                .into_int_value();

            let zero = self.make_int_value(register.size(), 0);
            let ones = self.builder.build_not(zero, "");
            let mut ext = self.zext(ones, IntType::I32);
            if register.is_hi_reg() {
                ext = self.shl(ext, self.make_u32(8));
            }
            let mask = self.builder.build_not(ext, "");
            // the mask is smth like FFFF0000 (for 16-bit case, for example)
            // It allows to clear the lower half of the register

            let base_clean_val = self.int_and(base_val, mask);

            let mut val_ext = self.zext(value, IntType::I32);
            if register.is_hi_reg() {
                val_ext = self.shl(val_ext, self.make_u32(8));
            }

            let res = self.int_or(base_clean_val, val_ext);

            self.builder.build_store(base_ptr, res);
        }
    }

    fn load_flag(&mut self, flag: Flag) -> Self::BoolValue {
        match flag {
            Flag::Carry => {}
            Flag::Parity => unimplemented!(),
            Flag::AuxiliaryCarry => unimplemented!(),
            Flag::Zero => {}
            Flag::Sign => {}
            Flag::Overflow => {}
            Flag::Direction => {}
            Flag::Id => {}
        };

        let ptr = self.prologue_builder.get_flag_ptr(flag);
        let i8_val = self
            .builder
            .build_load(self.types.i8, ptr, "")
            .into_int_value();

        let zero = self.make_u8(0);

        self.builder
            .build_int_compare(IntPredicate::NE, i8_val, zero, &*format!("{:?}", flag))
    }

    fn store_flag(&mut self, flag: Flag, value: Self::BoolValue) {
        let ptr = self.prologue_builder.get_flag_ptr(flag);
        let value = self.zext(value, IntType::I8);
        self.builder.build_store(ptr, value);
    }

    fn load_memory(&mut self, size: IntType, address: Self::IntValue) -> Self::IntValue {
        let pointee_ty = self.int_type(size);

        let hptr = self.get_host_pointer(address);
        let hptr =
            self.builder
                .build_pointer_cast(hptr, pointee_ty.ptr_type(AddressSpace::default()), "");

        let val = self.builder.build_load(pointee_ty, hptr, "");
        val.as_instruction_value()
            .unwrap()
            .set_alignment(1)
            .unwrap();
        val.into_int_value()
    }

    fn store_memory(&mut self, address: Self::IntValue, value: Self::IntValue) {
        let hptr = self.get_host_pointer(address);
        let hptr = self.builder.build_pointer_cast(
            hptr,
            value.get_type().ptr_type(AddressSpace::default()),
            "",
        );

        self.builder
            .build_store(hptr, value)
            .set_alignment(1)
            .unwrap();
    }

    fn add(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_add(lhs, rhs, "")
    }

    fn int_neg(&mut self, val: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_neg(val, "")
    }

    fn sub(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_sub(lhs, rhs, "")
    }

    fn mul(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_mul(lhs, rhs, "")
    }

    fn int_not(&mut self, val: Self::IntValue) -> Self::IntValue {
        self.builder.build_not(val, "")
    }

    fn int_or(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_or(lhs, rhs, "")
    }

    fn int_and(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_and(lhs, rhs, "")
    }

    fn int_xor(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_xor(lhs, rhs, "")
    }

    fn shl(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_left_shift(lhs, rhs, "")
    }

    fn lshr(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_right_shift(lhs, rhs, false, "")
    }

    fn ashr(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_right_shift(lhs, rhs, true, "")
    }

    fn udiv(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_unsigned_div(lhs, rhs, "")
    }

    fn sdiv(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_signed_div(lhs, rhs, "")
    }

    fn extract_bit(&mut self, val: Self::IntValue, bit: Self::IntValue) -> Self::BoolValue {
        let shifted = self.builder.build_right_shift(val, bit, false, "");
        self.builder.build_int_truncate(shifted, self.types.i1, "")
    }

    fn bool_not(&mut self, val: Self::BoolValue) -> Self::BoolValue {
        self.builder.build_not(val, "")
    }

    fn bool_or(&mut self, lhs: Self::BoolValue, rhs: Self::BoolValue) -> Self::BoolValue {
        self.builder.build_or(lhs, rhs, "")
    }

    fn bool_and(&mut self, lhs: Self::BoolValue, rhs: Self::BoolValue) -> Self::BoolValue {
        self.builder.build_and(lhs, rhs, "")
    }

    fn bool_xor(&mut self, lhs: Self::BoolValue, rhs: Self::BoolValue) -> Self::BoolValue {
        self.builder.build_xor(lhs, rhs, "")
    }

    fn uadd_overflow(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::BoolValue {
        self.call_binary_overflow_intrinsic(self.intrinsics.uadd_with_overflow, lhs, rhs)
    }

    fn sadd_overflow(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::BoolValue {
        self.call_binary_overflow_intrinsic(self.intrinsics.sadd_with_overflow, lhs, rhs)
    }

    fn usub_overflow(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::BoolValue {
        self.call_binary_overflow_intrinsic(self.intrinsics.usub_with_overflow, lhs, rhs)
    }

    fn ssub_overflow(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::BoolValue {
        self.call_binary_overflow_intrinsic(self.intrinsics.ssub_with_overflow, lhs, rhs)
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

    fn icmp(
        &mut self,
        cmp: ComparisonType,
        lhs: Self::IntValue,
        rhs: Self::IntValue,
    ) -> Self::BoolValue {
        self.builder.build_int_compare(cmp.into(), lhs, rhs, "")
    }

    fn direct_call(&mut self, target: u32) -> ControlFlow<Self::IntValue, Self::BoolValue> {
        let res = self.call_basic_block(target, false);
        ControlFlow::CallCheck(res)
    }

    fn indirect_call(
        &mut self,
        target: Self::IntValue,
    ) -> ControlFlow<Self::IntValue, Self::BoolValue> {
        let res = self.call_basic_block_indirect(target, false);
        ControlFlow::CallCheck(res)
    }

    // even though it's technically a jump, it works more like a tail call
    // therefore we use "Return" control flow for it (well, tail calls __are__ a way to return anyways...)
    fn thunk_jump(&mut self, target: u32) -> ControlFlow<Self::IntValue, Self::BoolValue> {
        todo!("Dump the register state before calling another basic block");

        let function = self.find_thunk_function(target);

        let args = &[self.ctx_ptr.into(), self.mem_ptr.into()];
        let res = self.builder.build_call(function, args, "");
        let res = res.try_as_basic_value().unwrap_left().into_int_value();
        ControlFlow::Return(res)
    }

    fn select(
        &mut self,
        cond: Self::BoolValue,
        iftrue: Self::IntValue,
        iffalse: Self::IntValue,
    ) -> Self::IntValue {
        self.builder
            .build_select(cond, iftrue, iffalse, "")
            .into_int_value()
    }

    fn ifelse<T, F>(&mut self, cond: Self::BoolValue, iftrue: T, iffalse: F)
    where
        T: FnOnce(&mut Self),
        F: FnOnce(&mut Self),
    {
        let true_bb = self.context.append_basic_block(self.function, "");
        let false_bb = self.context.append_basic_block(self.function, "");
        let cont_bb = self.context.append_basic_block(self.function, "");

        self.builder
            .build_conditional_branch(cond, true_bb, false_bb);

        self.builder.position_at_end(true_bb);
        (iftrue)(self);
        self.builder.build_unconditional_branch(cont_bb);

        self.builder.position_at_end(false_bb);
        (iffalse)(self);
        self.builder.build_unconditional_branch(cont_bb);

        self.builder.position_at_end(cont_bb);
    }

    fn trap(&mut self) {
        let trap = self
            .intrinsics
            .trap
            .get_declaration(self.module, &[])
            .unwrap();
        self.builder.build_call(trap, &[], "");
    }

    fn ctpop(&mut self, _value: Self::IntValue) -> Self::IntValue {
        todo!()
    }

    fn ctlz(&mut self, value: Self::IntValue) -> Self::IntValue {
        let ctlz = self
            .intrinsics
            .ctlz
            .get_declaration(self.module, &[value.get_type().into()])
            .unwrap();
        self.builder
            .build_call(
                ctlz,
                &[
                    value.into(),
                    self.make_false().into(), /* do not make zero a poison (at least for now) TODO: undef? */
                ],
                "",
            )
            .try_as_basic_value()
            .unwrap_left()
            .into_int_value()
    }

    fn cttz(&mut self, _value: Self::IntValue) -> Self::IntValue {
        todo!()
    }

    fn repeat_until<B>(&mut self, body: B)
    where
        B: Fn(&mut Self) -> Self::BoolValue,
        Self: Sized,
    {
        let loop_bb = self.context.append_basic_block(self.function, "repeat");

        self.builder.build_unconditional_branch(loop_bb);

        self.builder.position_at_end(loop_bb);

        let cont = (body)(self);

        let exit_bb = self.context.append_basic_block(self.function, "");
        self.builder
            .build_conditional_branch(cont, loop_bb, exit_bb);

        self.builder.position_at_end(exit_bb);
    }
}
