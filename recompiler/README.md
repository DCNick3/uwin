
Read the PE files, generate the static memory map, perform relocations, so, produce an actual executable image.

After that - identify basic blocks (potentially with some help from external tools) & recompile them using rusty_x86

The result is an LLVM module containing the recompiled functions along with some generated helpers