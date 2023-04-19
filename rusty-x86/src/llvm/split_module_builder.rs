use crate::llvm::backend::{RuntimeHelpers, Types, FASTCC_CALLING_CONVENTION};
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::values::FunctionValue;
use inkwell::GlobalVisibility;
use std::sync::Arc;

pub struct ModuleContext<'ctx> {
    pub rt_funs: RuntimeHelpers<'ctx>,
    /// this function should dispatch execution to a bb with address computed in runtime
    ///
    /// it is generated as a part of the main module and is a big switch basically
    pub indirect_bb_call_impl: FunctionValue<'ctx>,
    /// The linkage that should be used for functions in the module
    ///
    /// If there is only one module, we can use internal linkage to allow llvm to inline more stuff
    pub fn_linkage: Linkage,
    pub module: Module<'ctx>,
}

pub struct SplitModuleBuilder<'ctx> {
    modules: Vec<ModuleContext<'ctx>>,
}

impl<'ctx> SplitModuleBuilder<'ctx> {
    pub fn new(context: &'ctx Context, types: &Arc<Types<'ctx>>, modules_count: usize) -> Self {
        let mut modules = Vec::with_capacity(modules_count);

        let fn_linkage = if modules_count > 1 {
            Linkage::External
        } else {
            // a special case for llvm basic codegen where we have only one module
            // and we check that the generated code matches the assembly __exactly__
            // making it internal allows llvm to inline it and remove it from the final code
            Linkage::Internal
        };

        for i in 0..modules_count {
            let module = context.create_module(&format!("rusty-x86-{}", i));
            let rt_funs = RuntimeHelpers::new(&module, types.clone());

            let indirect_bb_call_impl = module.add_function(
                "indirect_bb_call_impl",
                types.indirect_bb_call,
                Some(fn_linkage),
            );
            indirect_bb_call_impl
                .as_global_value()
                .set_visibility(GlobalVisibility::Hidden);
            indirect_bb_call_impl.set_call_conventions(FASTCC_CALLING_CONVENTION);

            modules.push(ModuleContext {
                rt_funs,
                indirect_bb_call_impl,
                fn_linkage,
                module,
            });
        }

        Self { modules }
    }

    pub fn get_module_for(&self, bb_addr: u32) -> &ModuleContext<'ctx> {
        let module_index = bb_addr as usize % self.modules.len();
        &self.modules[module_index]
    }

    pub fn get_main_module(&self) -> &ModuleContext<'ctx> {
        &self.modules[0]
    }

    pub fn into_modules(self) -> Vec<Module<'ctx>> {
        self.modules.into_iter().map(|m| m.module).collect()
    }
}
