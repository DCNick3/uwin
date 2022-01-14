use std::any::Any;
use inkwell::AddressSpace;
use inkwell::context::Context;
use inkwell::module::Module;
use rusty_x86::codegen_instr;

use rusty_x86::llvm_backend::{Types, LlvmBuilder as Builder};

/*fn recompile_instr(context: Context, builder: &mut Builder) {
    //context.i32_type().const_int()
    //builder.build_extract_value()
    context.struct_type()
}*/

fn make_ir(context: &Context) -> Module {
    //let context = Context::create();
    let module = context.create_module("test");
    let types = Types::new(context);

    //let builder = Builder::new(context, &module, types, "dickenson");


    //codegen_instr(&mut builder, )

    //let llvm_builder = builder.get_builder();

    // let builder = context.create_builder();
    //
    //
    //
    // let void_type = context.void_type();
    // let i32_type = context.i32_type();
    // let ex_context_type = context.opaque_struct_type("context");
    // ex_context_type.set_body(&[
    //     i32_type.array_type(8).into() // general-purpose registers
    // ], false);
    //
    // let ex_context_ptr = ex_context_type.ptr_type(AddressSpace::Generic);
    //
    // let fn_type = void_type.fn_type(&[
    //     ex_context_ptr.into()
    // ], false);
    //
    // let function = module.add_function("dickenson", fn_type, None);
    // let bb = context.append_basic_block(function, "entry");
    //
    // builder.position_at_end(bb);
    //
    // let ctx_ptr = function.get_nth_param(0).unwrap().into_pointer_value();
    //
    // let eax = unsafe {
    //     builder.build_in_bounds_gep(ctx_ptr, &[
    //         i32_type.const_zero(),
    //         i32_type.const_zero(),
    //         i32_type.const_zero()
    //     ], "eax")
    // };
    //
    // builder.build_store(eax, i32_type.const_int(42, false));

    //llvm_builder.build_return(None);

    module
}

fn main() {
    let ctx = Context::create();
    make_ir(&ctx).print_to_stderr();
}