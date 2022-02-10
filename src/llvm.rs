use std::collections::{HashSet, VecDeque};

use iced_x86::Code::Call_rel32_32;
use iced_x86::{Decoder, DecoderOptions};
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::targets::{
    CodeModel, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};
use inkwell::OptimizationLevel;
use log::debug;

use crate::codegen_instr;
use crate::llvm::backend::{
    Intrinsics, LlvmBuilder, RuntimeHelpers, Types, FASTCC_CALLING_CONVENTION,
};
use crate::memory_image::MemoryImage;

pub mod backend;

pub fn get_aarch64_target_machine() -> TargetMachine {
    Target::initialize_aarch64(&InitializationConfig::default());
    //.expect("Failed to initialize aarch64 target");
    let target_triple = TargetTriple::create("aarch64");
    let target = Target::from_triple(&target_triple).unwrap();
    target
        .create_target_machine(
            &target_triple,
            &"generic",
            &"",
            OptimizationLevel::Aggressive,
            RelocMode::Static,
            CodeModel::Small,
        )
        .unwrap()
}

pub fn recompile<'ctx>(
    context: &'ctx Context,
    types: &'ctx Types,
    rt_funs: &'ctx RuntimeHelpers<'ctx>,
    image: &MemoryImage,
    entrypoint: u32,
) -> Module<'ctx> {
    let module_obj = context.create_module("test");
    let module = &module_obj;

    let indirect_bb_call = module.add_function(
        "indirect_bb_call",
        types.indirect_bb_call,
        Some(Linkage::Internal),
    );
    indirect_bb_call.set_call_conventions(FASTCC_CALLING_CONVENTION);

    let mut queue = VecDeque::new();
    let mut processing = HashSet::new();
    queue.push_back(entrypoint);

    while !queue.is_empty() {
        let address = queue.pop_front().unwrap();
        processing.insert(address);

        debug!("processing bb at 0x{:08x}", address);

        let mut builder =
            LlvmBuilder::new(context, module, types, rt_funs, indirect_bb_call, address);

        // this might be kinda expensive. TODO: how can we recycle decoders? Maybe create one for each region?
        let mut decoder = Decoder::new(32, image.execute_all_at(address), DecoderOptions::NONE);
        decoder.set_ip(address as u64);

        loop {
            // kinda want to assert that we should be able to decode, but some tests without ret's don't work then
            // TODO: ???
            // also probably want to raise an error if we jumped to smth undecodable
            if !decoder.can_decode() {
                break;
            }
            //assert!(decoder.can_decode());

            let instr = decoder.decode();

            let flow = codegen_instr(&mut builder, instr);

            builder.handle_flow(instr.next_ip32(), flow.clone());

            if let Some(addr) = flow.outer_jump_ref() {
                if !processing.contains(&addr) {
                    queue.push_back(addr);
                }
            }
            // kinda meh
            if instr.op_code().code() == Call_rel32_32 {
                let target = instr.near_branch32();
                if !processing.contains(&target) {
                    queue.push_back(target);
                }
            }

            if !flow.can_reach_next_instruction() {
                break;
            }
        }

        let llvm_builder = builder.get_raw_builder();
        llvm_builder.build_return(None);
    }

    // codegen for indirect_bb_call
    {
        let intrinsics = Intrinsics::new();
        let bb = context.append_basic_block(indirect_bb_call, "entry");
        let builder = context.create_builder();
        builder.position_at_end(bb);

        let trap = intrinsics.trap.get_declaration(module, &[]).unwrap();
        builder.build_call(trap, &[], "");
        builder.build_return(None);
    }

    module_obj
}
