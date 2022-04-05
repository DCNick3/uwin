use crate::pe_file::SymbolKind;
use crate::LoadedProcessImage;
use itertools::Itertools;

pub fn find_basic_blocks(image: &LoadedProcessImage) -> Vec<u32> {
    let sym_heads = image
        .symbols
        .iter()
        .filter(|(_, sym)| sym.kind == SymbolKind::Code)
        .map(|(&addr, _)| addr);

    let entry_heads = image
        .modules
        .iter()
        .map(|(pe, info)| (info, pe.entry()))
        .filter(|(_, entry)| *entry != 0)
        .map(|(info, entry)| info.base_addr + entry);

    // TODO: do some actual work like traversing instructions or smth...
    // maybe use some heuristics. Also definitely should be able to use externally-supplied bb addresses
    let heads = sym_heads.chain(entry_heads).unique().collect::<Vec<_>>();

    heads
}
