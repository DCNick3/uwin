use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::Arc;

use iced_x86::Code::Call_rel32_32;
use iced_x86::{Decoder, DecoderOptions};
use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::targets::{
    CodeModel, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};
use inkwell::values::{FunctionValue, IntValue};
use inkwell::OptimizationLevel;
use log::debug;

use crate::codegen_instr;
use crate::llvm::backend::{LlvmBuilder, RuntimeHelpers, Types, FASTCC_CALLING_CONVENTION};
use crate::llvm::BasicBlockSource::{DiscoveryCall, DiscoveryJump};
use memory_image::MemoryImage;

pub mod backend;

pub fn get_aarch64_target_machine() -> TargetMachine {
    Target::initialize_aarch64(&InitializationConfig::default());
    //.expect("Failed to initialize aarch64 target");
    let target_triple = TargetTriple::create("aarch64");
    let target = Target::from_triple(&target_triple).unwrap();
    target
        .create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::Aggressive,
            RelocMode::Static,
            CodeModel::Small,
        )
        .unwrap()
}

fn codegen_dynamic_dispatcher<'ctx, 'a>(
    context: &'ctx Context,
    _module: &'a Module,
    types: Arc<Types<'ctx>>,
    lifted_functions: &HashMap<u32, FunctionValue<'ctx>>,
    indirect_bb_call_impl: FunctionValue<'ctx>,
    runtime_helpers: &RuntimeHelpers,
) {
    let bb = context.append_basic_block(indirect_bb_call_impl, "entry");
    let builder = context.create_builder();

    let ctx_ptr = indirect_bb_call_impl
        .get_nth_param(0)
        .unwrap()
        .into_pointer_value();
    let mem_ptr = indirect_bb_call_impl
        .get_nth_param(1)
        .unwrap()
        .into_pointer_value();
    let eip = indirect_bb_call_impl
        .get_nth_param(2)
        .unwrap()
        .into_int_value();

    // for now - just generate a switch
    // this doesn't really scale for bigger executables, so we'll need to do some custom stuff
    // but it works for now

    let else_bb = context.append_basic_block(indirect_bb_call_impl, "not_found");

    builder.position_at_end(else_bb);
    let call = builder.build_call(
        runtime_helpers.missing_bb,
        &[ctx_ptr.into(), mem_ptr.into(), eip.into()],
        "",
    );
    call.set_tail_call(true);
    builder.build_return(Some(
        &call.try_as_basic_value().unwrap_left().into_int_value(),
    ));

    let args = [ctx_ptr.into(), mem_ptr.into()];

    let cases: Vec<(IntValue, BasicBlock)> = lifted_functions
        .iter()
        .map(|(&addr, &fun)| {
            let bb = context.append_basic_block(indirect_bb_call_impl, &format!("bb_{addr:08x}"));
            builder.position_at_end(bb);

            let call = builder.build_call(fun, &args, "");
            call.set_call_convention(FASTCC_CALLING_CONVENTION);
            call.set_tail_call(true);
            builder.build_return(Some(&call.try_as_basic_value().unwrap_left()));

            (types.i32.const_int(addr as u64, false), bb)
        })
        .collect();

    builder.position_at_end(bb);
    builder.build_switch(eip, else_bb, &cases);
}

fn codegen_dynamic_dispatcher_wrapper<'ctx, 'a>(
    context: &'ctx Context,
    module: &'a Module<'ctx>,
    types: Arc<Types<'ctx>>,
    indirect_bb_call_impl: FunctionValue<'ctx>,
) -> FunctionValue<'ctx> {
    let indirect_bb_call = module.add_function(
        "uwin_indirect_bb_call",
        types.indirect_bb_call,
        Some(Linkage::External),
    );
    let bb = context.append_basic_block(indirect_bb_call, "entry");
    let builder = context.create_builder();

    let ctx_ptr = indirect_bb_call
        .get_nth_param(0)
        .unwrap()
        .into_pointer_value();
    let mem_ptr = indirect_bb_call
        .get_nth_param(1)
        .unwrap()
        .into_pointer_value();
    let eip = indirect_bb_call.get_nth_param(2).unwrap().into_int_value();

    builder.position_at_end(bb);

    let res = builder.build_call(
        indirect_bb_call_impl,
        &[ctx_ptr.into(), mem_ptr.into(), eip.into()],
        "",
    );
    res.set_call_convention(FASTCC_CALLING_CONVENTION);
    builder.build_return(Some(&res.try_as_basic_value().unwrap_left()));

    indirect_bb_call
}

fn codegen_thunk_finder<'ctx, 'a>(
    context: &'ctx Context,
    module: &'a Module<'ctx>,
    types: Arc<Types<'ctx>>,
    thunk_functions: &'a BTreeMap<u32, String>,
) {
    let uwin_find_thunk = module.add_function(
        "uwin_find_thunk",
        types.find_thunk_fn,
        Some(Linkage::External),
    );

    let res_type = uwin_find_thunk
        .get_type()
        .get_return_type()
        .unwrap()
        .into_pointer_type();

    let thunk_id_param = uwin_find_thunk.get_nth_param(0).unwrap().into_int_value();

    let builder = context.create_builder();
    let entry = context.append_basic_block(uwin_find_thunk, "entry");

    let not_found = context.append_basic_block(uwin_find_thunk, "not_found");
    builder.position_at_end(not_found);
    builder.build_return(Some(&res_type.const_null()));

    let mut blocks = Vec::new();
    for (&thunk_id, thunk_name) in thunk_functions {
        let thunk_name = format!("thunk_{}", thunk_name);
        let function = if let Some(function) = module.get_function(&thunk_name) {
            function
        } else {
            module.add_function(&thunk_name, types.bb_fn, Some(Linkage::External))
        };
        let function = function.as_global_value().as_pointer_value();

        let bb = context.append_basic_block(uwin_find_thunk, &format!("thunk_{}", thunk_id));
        builder.position_at_end(bb);

        builder.build_return(Some(&function));

        blocks.push((types.i32.const_int(thunk_id as u64, false), bb))
    }

    builder.position_at_end(entry);
    builder.build_switch(thunk_id_param, not_found, &blocks);
}

#[derive(Debug)]
enum BasicBlockSource {
    DiscoveryJump(u32),
    DiscoveryCall(u32),
    Provided,
}

pub fn recompile<'ctx, 'a>(
    context: &'ctx Context,
    types: Arc<Types<'ctx>>,
    thunk_functions: &'a BTreeMap<u32, String>,
    image: &'a MemoryImage,
    basic_blocks: &[u32],
) -> Module<'ctx> {
    let module_obj = context.create_module("test");
    let module = &module_obj;

    let rt_funs = RuntimeHelpers::new(module, types.clone());

    let indirect_bb_call_impl = module.add_function(
        "indirect_bb_call_impl",
        types.indirect_bb_call,
        Some(Linkage::Internal),
    );
    indirect_bb_call_impl.set_call_conventions(FASTCC_CALLING_CONVENTION);

    let mut queue = VecDeque::<(u32, BasicBlockSource)>::new();
    let mut lifted_functions = HashMap::new();
    queue.extend(
        basic_blocks
            .iter()
            .map(|&addr| (addr, BasicBlockSource::Provided)),
    );

    while let Some((address, source)) = queue.pop_front() {
        if lifted_functions.contains_key(&address) {
            continue;
        }

        debug!("processing bb at 0x{:08x} (source = {:?})", address, source);

        let mut builder = LlvmBuilder::new(
            context,
            module,
            types.clone(),
            &rt_funs,
            thunk_functions,
            indirect_bb_call_impl,
            address,
        );

        lifted_functions.insert(address, builder.get_function());

        // this might be kinda expensive. TODO: how can we recycle decoders? Maybe create one for each region?
        let mut decoder = Decoder::new(32, image.execute_all_at(address), DecoderOptions::NONE);
        decoder.set_ip(address as u64);

        let ret = loop {
            // kinda want to assert that we should be able to decode, but some tests without ret's don't work then
            // TODO: ???
            // also probably want to raise an error if we jumped to smth undecodable
            if !decoder.can_decode() {
                // TODO: store info on __why__ it happened
                // maybe even call some runtime function?
                break types.i32.const_zero();
            }
            //assert!(decoder.can_decode());

            let instr = decoder.decode();

            let flow = codegen_instr(&mut builder, instr);

            let ret = builder.handle_flow(instr.next_ip32(), flow.clone());

            if let Some(addr) = flow.outer_jump_ref() {
                if !lifted_functions.contains_key(&addr) {
                    queue.push_back((addr, DiscoveryJump(instr.next_ip32() - instr.len() as u32)));
                }
            }
            // kinda meh
            if instr.op_code().code() == Call_rel32_32 {
                let target = instr.near_branch32();
                if !lifted_functions.contains_key(&target) {
                    queue.push_back((
                        target,
                        DiscoveryCall(instr.next_ip32() - instr.len() as u32),
                    ));
                }
            }

            if let Some(ret) = ret {
                break ret;
            }
        };

        let llvm_builder = builder.get_raw_builder();
        llvm_builder.build_return(Some(&ret));
    }

    // codegen indirect_bb_call_impl
    // the difference between indirect_bb_call and indirect_bb_call_impl is the calling convention
    // indirect_bb_call_impl is fastcc and is eligible for tail call optimization (very important for generated code)
    // indirect_bb_call (exported as uwin_indirect_bb_call) has c calling convention and is used as an exported entrypoint into the recompiled code
    codegen_dynamic_dispatcher(
        context,
        module,
        types.clone(),
        &lifted_functions,
        indirect_bb_call_impl,
        &rt_funs,
    );

    // codegen indirect_bb_call (exported as uwin_indirect_bb_call)
    codegen_dynamic_dispatcher_wrapper(context, module, types.clone(), indirect_bb_call_impl);

    // codegen uwin_find_thunk
    codegen_thunk_finder(context, module, types, thunk_functions);

    module_obj
}
