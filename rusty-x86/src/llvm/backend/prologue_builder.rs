use crate::llvm::backend::Types;

use std::sync::Arc;

use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;

use crate::types::{Flag, FullSizeGeneralPurposeRegister};
use inkwell::types::IntType as LlvmIntType;
use inkwell::values::{FunctionValue, IntValue as LlvmIntValue, PointerValue};
use strum::IntoEnumIterator;

pub struct PrologueBuilder<'ctx> {
    context: &'ctx Context,
    types: Arc<Types<'ctx>>,
    function: FunctionValue<'ctx>,
    builder: Builder<'ctx>,
    ctx_ptr: PointerValue<'ctx>,
    mem_ptr: PointerValue<'ctx>,

    prologue_bb: BasicBlock<'ctx>,
    // stores pointer to the local-variable versions of the registers and flags
    // they are allocated in the prologue as-needed (to prevent the explosion of IR size)
    // it's very important to dump them back to the context before returning or calling a function
    gp_regs: [Option<PointerValue<'ctx>>; 8],
    flags: [Option<PointerValue<'ctx>>; 8],
    fs_base: Option<LlvmIntValue<'ctx>>,
    gs_base: Option<LlvmIntValue<'ctx>>,
    dumped: bool,
}

impl<'ctx> PrologueBuilder<'ctx> {
    pub fn new(
        context: &'ctx Context,
        types: Arc<Types<'ctx>>,
        function: FunctionValue<'ctx>,
    ) -> (Self, BasicBlock<'ctx>) {
        let builder = context.create_builder();
        let prologue_bb = context.append_basic_block(function, "prologue");

        let ctx_ptr = function.get_nth_param(0).unwrap().into_pointer_value();
        let mem_ptr = function.get_nth_param(1).unwrap().into_pointer_value();

        (
            Self {
                context,
                types,
                function,
                builder,
                ctx_ptr,
                mem_ptr,

                prologue_bb,
                gp_regs: [None; 8],
                flags: [None; 8],
                fs_base: None,
                gs_base: None,
                dumped: false,
            },
            prologue_bb,
        )
    }

    pub fn set_first_block(&mut self, after_prologue: BasicBlock<'ctx>) {
        self.builder.position_at_end(self.prologue_bb);
        let ret_instr = self.builder.build_unconditional_branch(after_prologue);
        self.builder.position_before(&ret_instr);
    }
}

// cpu context accessors (getelementptr is ugly...)
impl<'ctx> PrologueBuilder<'ctx> {
    fn get_ctx_gp_ptr(
        &mut self,
        ctx_ptr: PointerValue<'ctx>,
        reg: FullSizeGeneralPurposeRegister,
    ) -> PointerValue<'ctx> {
        let i32_type = self.types.i32;
        // SAFETY: ¯\_(ツ)_/¯
        let r = unsafe {
            self.builder.build_gep(
                self.types.ctx,
                ctx_ptr,
                &[
                    i32_type.const_zero(),                 // deref the pointer itself
                    i32_type.const_zero(),                 // select the gp array
                    i32_type.const_int(reg as u64, false), // then select the concrete register
                ],
                &*(format!("{:?}_ptr", reg)),
            )
        };
        r
    }

    fn get_ctx_flag_ptr(&mut self, ctx_ptr: PointerValue<'ctx>, flag: Flag) -> PointerValue<'ctx> {
        // SAFETY: ¯\_(ツ)_/¯
        let i32_type = self.types.i32;
        let r = unsafe {
            self.builder.build_gep(
                self.types.ctx,
                ctx_ptr,
                &[
                    i32_type.const_zero(),                  // deref the pointer itself
                    i32_type.const_int(1, false),           // select the flags array
                    i32_type.const_int(flag as u64, false), // then select the concrete flag
                ],
                &*format!("flag_{:?}_ptr", flag),
            )
        };
        r
    }

    fn get_ctx_fs_base_ptr(&mut self, ctx_ptr: PointerValue<'ctx>) -> PointerValue<'ctx> {
        let i32_type = self.types.i32;
        // SAFETY: ¯\_(ツ)_/¯
        let r = unsafe {
            self.builder.build_gep(
                self.types.ctx,
                ctx_ptr,
                &[
                    i32_type.const_zero(),        // deref the strct pointer itself
                    i32_type.const_int(2, false), // select fs base value
                ],
                "fs_base",
            )
        };
        r
    }

    fn get_ctx_gs_base_ptr(&mut self, ctx_ptr: PointerValue<'ctx>) -> PointerValue<'ctx> {
        let i32_type = self.types.i32;
        // SAFETY: ¯\_(ツ)_/¯
        let r = unsafe {
            self.builder.build_gep(
                self.types.ctx,
                ctx_ptr,
                &[
                    i32_type.const_zero(),        // deref the strct pointer itself
                    i32_type.const_int(3, false), // select gs base value
                ],
                "gs_base",
            )
        };
        r
    }
}

impl<'ctx> PrologueBuilder<'ctx> {
    /// Create a new local register/flag variable, inserting the alloca and init code to the prologue
    ///
    /// NOTE: this does not register the variable in the gp_regs or flags array, as it is a lower level routine
    fn build_prologue_var(
        &mut self,
        ty: LlvmIntType<'ctx>,
        name: &str,
        src_ptr: PointerValue<'ctx>,
    ) -> PointerValue<'ctx> {
        self.builder
            .position_before(&self.prologue_bb.get_last_instruction().unwrap());

        let ptr = self.builder.build_alloca(ty, name);
        let init_val = self
            .builder
            .build_load(ty, src_ptr, format!("{}_init", name).as_str());
        self.builder.build_store(ptr, init_val);

        ptr
    }

    /// Get a pointer to a full-sized general purpose register variable
    ///
    /// This will create a new alloca in the prologue if the register is not already stored in a local variable
    pub fn get_gp_ptr(&mut self, reg: FullSizeGeneralPurposeRegister) -> PointerValue<'ctx> {
        match self.gp_regs[reg as usize] {
            None => {
                let src_ptr = self.get_ctx_gp_ptr(self.ctx_ptr, reg);
                let ptr =
                    self.build_prologue_var(self.types.i32, &format!("{:?}_local", reg), src_ptr);
                self.gp_regs[reg as usize] = Some(ptr);
                ptr
            }
            Some(ptr) => ptr,
        }
    }

    /// Get a pointer to a flag variable
    ///
    /// This will create a new alloca in the prologue if the flag is not already stored in a local variable
    pub fn get_flag_ptr(&mut self, flag: Flag) -> PointerValue<'ctx> {
        match self.flags[flag as usize] {
            None => {
                let src_ptr = self.get_ctx_flag_ptr(self.ctx_ptr, flag);
                let ptr =
                    self.build_prologue_var(self.types.i8, &format!("{:?}_local", flag), src_ptr);
                self.flags[flag as usize] = Some(ptr);
                ptr
            }
            Some(ptr) => ptr,
        }
    }

    /// Get the FS segment base address (assumed constant)
    pub fn get_fs_base(&mut self) -> LlvmIntValue<'ctx> {
        match self.fs_base {
            None => {
                let ptr = self.get_ctx_fs_base_ptr(self.ctx_ptr);
                let fs_base = self
                    .builder
                    .build_load(self.types.i32, ptr, "fs_base")
                    .into_int_value();
                self.fs_base = Some(fs_base);
                fs_base
            }
            Some(v) => v,
        }
    }

    /// Get the GS segment base address (assumed constant)
    pub fn get_gs_base(&mut self) -> LlvmIntValue<'ctx> {
        match self.gs_base {
            None => {
                let ptr = self.get_ctx_gs_base_ptr(self.ctx_ptr);
                let gs_base = self
                    .builder
                    .build_load(self.types.i32, ptr, "gs_base")
                    .into_int_value();
                self.gs_base = Some(gs_base);
                gs_base
            }
            Some(v) => v,
        }
    }

    pub fn dump(mut self, builder: &mut Builder<'ctx>) {
        for (reg_var, reg) in self
            .gp_regs
            .into_iter()
            .zip(FullSizeGeneralPurposeRegister::iter())
        {
            if let Some(reg_var) = reg_var {
                let reg_ptr = self.get_ctx_gp_ptr(self.ctx_ptr, reg);
                let reg_val =
                    builder.build_load(self.types.i32, reg_var, &format!("{:?}_final", reg));
                builder.build_store(reg_ptr, reg_val);
            }
        }

        for (flag_var, flag) in self.flags.into_iter().zip(Flag::iter()) {
            if let Some(flag_var) = flag_var {
                let flag_ptr = self.get_ctx_flag_ptr(self.ctx_ptr, flag);
                let flag_val =
                    builder.build_load(self.types.i8, flag_var, &format!("{:?}_final", flag));
                builder.build_store(flag_ptr, flag_val);
            }
        }

        self.dumped = true;
    }
}

impl Drop for PrologueBuilder<'_> {
    fn drop(&mut self) {
        if !self.dumped {
            if !std::thread::panicking() {
                panic!("PrologueBuilder was not dumped before being dropped")
            }
        }
    }
}
