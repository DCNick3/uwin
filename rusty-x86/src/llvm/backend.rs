use std::collections::BTreeMap;
use std::sync::Arc;

use inkwell::attributes::{Attribute, AttributeLoc};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::intrinsics::Intrinsic;
use inkwell::module::{Linkage, Module};
use inkwell::types::{FunctionType, IntType as LlvmIntType, PointerType, StructType, VoidType};
use inkwell::values::{
    BasicMetadataValueEnum, BasicValue, CallSiteValue, FunctionValue, IntValue as LlvmIntValue,
    PointerValue,
};
use inkwell::{AddressSpace, IntPredicate};

use crate::backend::{BoolValue, ComparisonType, IntValue};
use crate::types::{Flag, FullSizeGeneralPurposeRegister, IntType, Register, SegmentRegister};
use crate::ControlFlow;

pub struct LlvmBuilder<'ctx, 'a> {
    context: &'ctx Context,
    module: &'a Module<'ctx>,
    function: FunctionValue<'ctx>,
    builder: Builder<'ctx>,
    types: Arc<Types<'ctx>>,
    intrinsics: Intrinsics,
    ctx_ptr: PointerValue<'ctx>,
    mem_ptr: PointerValue<'ctx>,

    // this function should dispatch execution to a bb with address computed in runtime
    indirect_bb_call: FunctionValue<'ctx>,
    // this is for functions to be implemented by a runtime
    #[allow(unused)]
    rt_funs: &'a RuntimeHelpers<'ctx>,
    magic_functions: &'a BTreeMap<u32, String>,
}

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

    pub bb_fn: FunctionType<'ctx>,            // ctx: Context*, mem: u8*
    pub indirect_bb_call: FunctionType<'ctx>, // ctx: Context*, mem: u8*, eip: u32
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
        let ctx_ptr = ctx.ptr_type(AddressSpace::Generic);
        let mem_ptr = i8.ptr_type(AddressSpace::Generic);

        let bb_fn = void.fn_type(
            &[
                ctx_ptr.into(), // ctx
                mem_ptr.into(), // mem - pointer to start of guest address space (same trick as qemu does)
            ],
            false,
        );

        let rt_indirect_bb_call = void.fn_type(
            &[
                ctx_ptr.into(), // ctx
                mem_ptr.into(), // mem
                i32.into(),     // eip
            ],
            false,
        );

        Arc::new(Self {
            void,
            i1,
            i8,
            i16,
            i32,
            i64,
            ctx,
            ctx_ptr,

            bb_fn,
            indirect_bb_call: rt_indirect_bb_call,
        })
    }
}

pub struct Intrinsics {
    pub sadd_with_overflow: Intrinsic,
    pub uadd_with_overflow: Intrinsic,
    pub ssub_with_overflow: Intrinsic,
    pub usub_with_overflow: Intrinsic,
    pub trap: Intrinsic,
}

impl Intrinsics {
    pub fn new() -> Self {
        Self {
            sadd_with_overflow: Intrinsic::find("llvm.sadd.with.overflow").unwrap(),
            uadd_with_overflow: Intrinsic::find("llvm.uadd.with.overflow").unwrap(),
            ssub_with_overflow: Intrinsic::find("llvm.ssub.with.overflow").unwrap(),
            usub_with_overflow: Intrinsic::find("llvm.usub.with.overflow").unwrap(),
            trap: Intrinsic::find("llvm.trap").unwrap(),
        }
    }
}

impl Default for Intrinsics {
    fn default() -> Self {
        Intrinsics::new()
    }
}

pub struct RuntimeHelpers<'ctx> {
    pub missing_bb: FunctionValue<'ctx>,
}

impl<'ctx> RuntimeHelpers<'ctx> {
    pub fn new(module: &Module<'ctx>, types: Arc<Types<'ctx>>) -> Self {
        Self {
            missing_bb: module.add_function(
                "uwin_missing_bb",
                types.indirect_bb_call,
                Some(Linkage::External),
            ),
        }
    }
}

pub const FASTCC_CALLING_CONVENTION: u32 = 8;

impl<'ctx, 'a> LlvmBuilder<'ctx, 'a> {
    pub fn new(
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        types: Arc<Types<'ctx>>,
        rt_funs: &'a RuntimeHelpers<'ctx>,
        magic_functions: &'a BTreeMap<u32, String>,
        indirect_bb_call: FunctionValue<'ctx>,
        basic_block_addr: u32,
    ) -> Self {
        let function =
            Self::get_basic_block_fun_internal(context, module, types.clone(), basic_block_addr);
        let bb = context.append_basic_block(function, "entry");

        let builder = context.create_builder();
        builder.position_at_end(bb);

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

            indirect_bb_call,
            rt_funs,
            magic_functions,
        }
    }

    pub fn get_raw_builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    pub fn get_function(&self) -> FunctionValue<'ctx> {
        self.function
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
                    i32_type.const_zero(),                  // deref the pointer itself
                    i32_type.const_int(1, false),           // select the flags array
                    i32_type.const_int(flag as u64, false), // then select the concrete flag
                ],
                &*format!("flag_{:?}_ptr", flag),
            )
        };
        debug_assert_eq!(r.get_type().get_element_type().into_int_type(), i8_type);
        r
    }

    fn build_ctx_fs_base_gep(&mut self, ctx_ptr: PointerValue<'ctx>) -> PointerValue<'ctx> {
        // TODO: cache the pointers at (generated) function level
        let i32_type = self.context.i32_type();
        // SAFETY: ¯\_(ツ)_/¯
        let r = unsafe {
            self.builder.build_gep(
                ctx_ptr,
                &[
                    i32_type.const_zero(),        // deref the strct pointer itself
                    i32_type.const_int(2, false), // select fs base value
                ],
                "fs_base",
            )
        };
        debug_assert_eq!(r.get_type().get_element_type().into_int_type(), i32_type);
        r
    }

    fn build_ctx_gs_base_gep(&mut self, ctx_ptr: PointerValue<'ctx>) -> PointerValue<'ctx> {
        // TODO: cache the pointers at (generated) function level
        let i32_type = self.context.i32_type();
        // SAFETY: ¯\_(ツ)_/¯
        let r = unsafe {
            self.builder.build_gep(
                ctx_ptr,
                &[
                    i32_type.const_zero(),        // deref the strct pointer itself
                    i32_type.const_int(3, false), // select gs base value
                ],
                "fs_base",
            )
        };
        debug_assert_eq!(r.get_type().get_element_type().into_int_type(), i32_type);
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
        let target_ptr_ext = self
            .builder
            .build_int_z_extend(target_ptr, self.types.i64, "");

        unsafe {
            self.builder
                .build_gep(self.mem_ptr, &[target_ptr_ext], "hptr")
        }
    }

    // TODO: name map
    pub fn get_name_for(addr: u32) -> String {
        format!("sub_{:08x}", addr)
    }

    fn get_basic_block_fun_internal(
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        types: Arc<Types<'ctx>>,
        addr: u32,
    ) -> FunctionValue<'ctx> {
        let name = Self::get_name_for(addr);
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

    pub fn call_basic_block(&mut self, target: u32, tail_call: bool) {
        let target = self.get_basic_block_fun(target);
        let args = &[self.ctx_ptr.into(), self.mem_ptr.into()];
        let call = self.fastcall(target, args);
        call.set_tail_call(tail_call)
    }

    pub fn call_basic_block_indirect(&mut self, target: LlvmIntValue<'ctx>, tail_call: bool) {
        let args = &[self.ctx_ptr.into(), self.mem_ptr.into(), target.into()];
        let call = self.fastcall(self.indirect_bb_call, args);
        call.set_tail_call(tail_call)
    }

    pub fn find_magic_function(&mut self, index: u32) -> FunctionValue<'ctx> {
        let name = self
            .magic_functions
            .get(&index)
            .expect("Call to unknown magic function");

        if let Some(function) = self.module.get_function(name) {
            function
        } else {
            self.module.add_function(
                &format!("magic_{}", name),
                self.types.bb_fn,
                Some(Linkage::External),
            )
        }
    }

    pub fn handle_flow(&mut self, next_ip: u32, flow: ControlFlow<Self>) {
        match flow {
            ControlFlow::NextInstruction => {
                // create a new basic block for ease of reading ir
                let next_bb = self
                    .context
                    .append_basic_block(self.function, format!("bb_{:08x}", next_ip).as_str());

                self.builder.build_unconditional_branch(next_bb);
                self.builder.position_at_end(next_bb);
            }
            ControlFlow::DirectJump(addr) => {
                self.call_basic_block(addr, true);
                // no need for ret; all recompiled funs are terminated with ret
            }
            ControlFlow::IndirectJump(addr) => {
                self.call_basic_block_indirect(addr, true);
            }
            ControlFlow::Return => {
                // no need for ret; all recompiled funs are terminated with ret
            }
            ControlFlow::Conditional(cond, target) => {
                let branch_to = self
                    .context
                    .append_basic_block(self.function, format!("br_to_{:08x}", target).as_str());

                let next_bb = self
                    .context
                    .append_basic_block(self.function, format!("bb_{:08x}", next_ip).as_str());

                self.builder
                    .build_conditional_branch(cond, branch_to, next_bb);

                self.builder.position_at_end(branch_to);
                self.call_basic_block(target, true);
                self.builder.build_return(None);

                self.builder.position_at_end(next_bb);
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

    fn make_int_value(&self, ty: IntType, value: u64, sign_extend: bool) -> Self::IntValue {
        self.int_type(ty).const_int(value, sign_extend)
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
            FS => {
                let ptr = self.build_ctx_fs_base_gep(self.ctx_ptr);
                self.builder.build_load(ptr, "gs").into_int_value()
            }
            GS => {
                let ptr = self.build_ctx_gs_base_gep(self.ctx_ptr);
                self.builder.build_load(ptr, "fs").into_int_value()
            }
        }
    }

    fn load_register(&mut self, register: Register) -> Self::IntValue {
        let base = register.base_register();
        let base_ptr = self.build_ctx_gp_gep(self.ctx_ptr, base);
        let mut base_val = self
            .builder
            .build_load(base_ptr, &*format!("{:?}", base))
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
        let base_ptr = self.build_ctx_gp_gep(self.ctx_ptr, base);

        if FullSizeGeneralPurposeRegister::try_from(register).is_ok() {
            self.builder.build_store(base_ptr, value);
        } else {
            // ehh, this is kinda ugly. Maybe we can index directly into the base value? how 'bout aliasing?
            let base_val = self
                .builder
                .build_load(base_ptr, &*format!("{:?}", base))
                .into_int_value();

            let zero = self.make_int_value(register.size(), 0, false);
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

        let ptr = self.build_ctx_flag_gep(self.ctx_ptr, flag);
        let i8_val = self.builder.build_load(ptr, "").into_int_value();

        let zero = self.make_u8(0);

        self.builder
            .build_int_compare(IntPredicate::NE, i8_val, zero, &*format!("{:?}", flag))
    }

    fn store_flag(&mut self, flag: Flag, value: Self::BoolValue) {
        let ptr = self.build_ctx_flag_gep(self.ctx_ptr, flag);
        let value = self.zext(value, IntType::I8);
        self.builder.build_store(ptr, value);
    }

    fn load_memory(&mut self, size: IntType, address: Self::IntValue) -> Self::IntValue {
        let hptr = self.get_host_pointer(address);
        let hptr = self.builder.build_pointer_cast(
            hptr,
            self.int_type(size).ptr_type(AddressSpace::Generic),
            "",
        );

        let val = self.builder.build_load(hptr, "");
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
            value.get_type().ptr_type(AddressSpace::Generic),
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

    fn direct_call(&mut self, target: u32, _next_eip: u32) {
        self.call_basic_block(target, false);
        // TODO: compare EIP to expected return address
        // else we fail in case the binary mis-uses call or ret
    }

    fn indirect_call(&mut self, target: Self::IntValue, _next_eip: u32) {
        self.call_basic_block_indirect(target, false);
        // TODO: compare EIP to expected return address
        // else we fail in case the binary mis-uses call or ret
    }

    fn magic_call(&mut self, target: u32, _next_eip: u32) {
        let function = self.find_magic_function(target);

        let args = &[self.ctx_ptr.into(), self.mem_ptr.into()];
        self.builder.build_call(function, args, "");

        // No need to compare next_eip with the real one because ¿I think? we don't change it in any native code?
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
