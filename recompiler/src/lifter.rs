use crate::pe_file::SymbolKind;
use crate::LoadedProcessImage;
use inkwell::context::Context;
use inkwell::module::Module;
use itertools::Itertools;
use rusty_x86::llvm::backend::{RuntimeHelpers, Types};

pub fn find_basic_blocks(image: &LoadedProcessImage) -> Vec<u32> {
    let sym_heads = image
        .symbols
        .iter()
        .filter(|(_, sym)| sym.kind == SymbolKind::Code)
        .map(|(&addr, _)| addr);

    let entry_heads = image
        .modules
        .iter()
        .map(|(_, (pe, info))| (info, pe.entry()))
        .filter(|(_, entry)| *entry != 0)
        .map(|(info, entry)| info.base_addr + entry);

    // TODO: do some actual work like traversing instructions or smth...
    // maybe use some heuristics. Also definitely should be able to use externally-supplied bb addresses
    let heads = sym_heads.chain(entry_heads).unique().collect::<Vec<_>>();

    heads
}

pub fn lift<'ctx>(llvm_context: &'ctx Context, image: &LoadedProcessImage) -> Module<'ctx> {
    let types = Types::new(llvm_context);
    let runtime_helpers = RuntimeHelpers::dummy(types.clone());

    let basic_blocks = find_basic_blocks(image);

    rusty_x86::llvm::recompile(
        llvm_context,
        types,
        &runtime_helpers,
        &image.magic_functions,
        &image.memory,
        &basic_blocks,
    )
}
