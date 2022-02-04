use std::collections::{HashSet, VecDeque};

use iced_x86::Code::Call_rel32_32;
use iced_x86::{Decoder, DecoderOptions};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{
    CodeModel, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};
use inkwell::OptimizationLevel;
use log::debug;

use crate::codegen_instr;
use crate::llvm::backend::{LlvmBuilder, Types};

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
    base_address: u32,
    code: &[u8],
) -> Module<'ctx> {
    let module_obj = context.create_module("test");
    let module = &module_obj;

    let mut decoder = Decoder::new(32, code, DecoderOptions::NONE);

    let mut queue = VecDeque::new();
    let mut processing = HashSet::new();
    queue.push_back(base_address);

    while !queue.is_empty() {
        let address = queue.pop_front().unwrap();
        processing.insert(address);

        let offset = address.checked_sub(base_address).unwrap();

        debug!("processing bb at 0x{:08x}", address);

        let mut builder = LlvmBuilder::new(context, module, types, address);
        decoder.set_ip(address as u64);
        decoder.set_position(offset as usize).unwrap();

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

    module_obj
}
