use crate::llvm::backend::Types;
use inkwell::module::{Linkage, Module};
use inkwell::values::FunctionValue;
use std::sync::Arc;

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
