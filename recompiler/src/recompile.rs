use crate::error::Result;
use crate::pe_file::SymbolKind;
use crate::LoadedProcessImage;
use itertools::Itertools;
use rusty_x86::inkwell::context::Context;
use rusty_x86::inkwell::module::Module;
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

pub fn recompile_image<'ctx>(
    llvm_context: &'ctx Context,
    image: &LoadedProcessImage,
) -> Result<Module<'ctx>> {
    let types = Types::new(llvm_context);
    let runtime_helpers = RuntimeHelpers::dummy(types.clone());

    let basic_blocks = find_basic_blocks(image);

    let module = rusty_x86::llvm::recompile(
        llvm_context,
        types.clone(),
        &runtime_helpers,
        &image.magic_functions,
        &image.memory,
        &basic_blocks,
    );

    let serialized_process_image = rmp_serde::to_vec(&image)?;

    let ty = types.i8.array_type(
        serialized_process_image
            .len()
            .try_into()
            .expect("Serialized process image too large"),
    );

    let process_image_glob = module.add_global(ty, None, "uwin_serialized_process_image");

    process_image_glob
        .set_initializer(&llvm_context.const_string(&serialized_process_image, false));
    process_image_glob.set_constant(true);

    let process_image_size_glob =
        module.add_global(types.i32, None, "uwin_serialized_process_image_size");

    process_image_size_glob.set_initializer(
        &types
            .i32
            .const_int(serialized_process_image.len() as u64, false),
    );
    process_image_size_glob.set_constant(true);

    Ok(module)
}
