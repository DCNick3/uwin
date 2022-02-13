use elfloader::{ElfBinary, ElfLoader, ElfLoaderErr, Flags, LoadableHeaders, Rela, VAddr, P64};
use inkwell::execution_engine::JitFunction;
use inkwell::values::BasicMetadataValueEnum;
use inkwell::OptimizationLevel;
use log::{debug, trace};
use region::Allocation;
use rusty_x86::llvm::backend::{BbFunc, FASTCC_CALLING_CONVENTION};
use rusty_x86::memory_image::{MemoryImage, MemoryImageItem, Protection};
use rusty_x86::types::{CpuContext, Flag, FullSizeGeneralPurposeRegister};
use std::collections::BTreeMap;
use strum::IntoEnumIterator;
use unicorn;
use unicorn::Error::ETCH_UNMAPPED;
use unicorn::{Cpu, CpuX86, Protection as UniProtection, RegisterX86};

pub const CODE_ADDR: u32 = 0x200000;
pub const MEM_ADDR: u32 = 0x100000;
pub const MEM_SIZE: u32 = 0x10000;

const STACK_ADDR: u32 = 0x38000000;
const STACK_SIZE: u32 = 0x10000; // 64 KiB

pub const MAGIC_RETURN_ADDR: u32 = 0xCAFEBABE;
pub const PAGE_ALIGN: u32 = 0x1000;

#[derive(Clone)]
pub enum CodeToTest<'a> {
    Snippet(&'a [u8]),                // just the code
    Function(&'a [u8], &'a [u32]),    // code & args
    ElfFunction(&'a [u8], &'a [u32]), // elf contents & args
}

impl<'a> CodeToTest<'a> {
    pub fn get_code(&self) -> (MemoryImage, u32) {
        let mut image = MemoryImage::new();
        let entry;
        match self {
            CodeToTest::Snippet(c) | CodeToTest::Function(c, _) => {
                image.add_region(CODE_ADDR, Protection::READ_EXECUTE, c.to_vec());
                image.add_zero_region(MEM_ADDR, Protection::READ_WRITE, MEM_SIZE);
                entry = CODE_ADDR;
            }
            CodeToTest::ElfFunction(elf, _) => {
                let elf = ElfBinary::new(elf).unwrap();
                let mut builder_wrap = MemoryImageWrap(image);
                elf.load(&mut builder_wrap).unwrap();
                image = builder_wrap.0;
                entry = elf.entry_point() as u32;
            }
        };
        (image, entry)
    }

    pub fn get_args(&self) -> Vec<u32> {
        match self {
            CodeToTest::Snippet(_) => vec![],
            CodeToTest::Function(_, args) => args.to_vec(),
            CodeToTest::ElfFunction(_, args) => args.to_vec(),
        }
    }
}

struct MemoryImageWrap(MemoryImage);

impl<'a> ElfLoader for MemoryImageWrap {
    fn allocate(&mut self, _: LoadableHeaders) -> Result<(), ElfLoaderErr> {
        Ok(()) // ignore
    }

    fn load(&mut self, flags: Flags, base: VAddr, region: &[u8]) -> Result<(), ElfLoaderErr> {
        // add to the list
        let mut prot = Protection::NONE;
        if flags.is_read() {
            prot |= Protection::READ;
        }
        if flags.is_write() {
            prot |= Protection::WRITE;
        }
        if flags.is_execute() {
            prot |= Protection::EXECUTE;
        }
        self.0.add_region(base as u32, prot, region.to_vec());
        Ok(())
    }

    fn relocate(&mut self, _: &Rela<P64>) -> Result<(), ElfLoaderErr> {
        Ok(()) // ignore
    }
}

// impl<'a> ElfLoader for UnicornEmu<'a> {
//     fn allocate(&mut self, load_headers: LoadableHeaders) -> Result<(), ElfLoaderErr> {
//         for header in load_headers {
//             debug!(
//                 "LOAD addr = 0x{:08x} size = 0x{:08x} flags = {}",
//                 header.virtual_addr(),
//                 header.mem_size(),
//                 header.flags()
//             );
//
//             let mut prot = UniProtection::NONE;
//             if header.flags().is_read() {
//                 prot |= UniProtection::READ
//             }
//             if header.flags().is_write() {
//                 prot |= UniProtection::WRITE
//             }
//             if header.flags().is_execute() {
//                 prot |= UniProtection::EXEC
//             }
//
//             // enforce W^X
//             assert!(!(header.flags().is_write() && header.flags().is_execute()));
//
//             // align up to page size
//             const PAGE_SIZE: u64 = 0x1000;
//             let mut mem_size = header.mem_size();
//             if mem_size % PAGE_SIZE != 0 {
//                 mem_size += PAGE_SIZE - (mem_size % PAGE_SIZE);
//             }
//
//             self.0
//                 .mem_map(header.virtual_addr(), mem_size as usize, prot)
//                 .unwrap()
//         }
//         Ok(())
//     }
//
//     fn load(&mut self, flags: Flags, base: VAddr, region: &[u8]) -> Result<(), ElfLoaderErr> {
//         self.0.mem_write(base, region).unwrap();
//         Ok(())
//     }
//
//     fn relocate(&mut self, entry: &Rela<P64>) -> Result<(), ElfLoaderErr> {
//         // don't relocate, we can always load at exact address
//         Ok(())
//     }
// }

fn load_unicorn(
    emu: &mut CpuX86,
    code_and_args: CodeToTest,
) -> (u64, Option<u64>, Vec<(u64, u64)>) {
    let (image, entry) = code_and_args.get_code();

    for MemoryImageItem {
        addr,
        protection,
        data,
    } in image.iter()
    {
        let mut uprot = UniProtection::NONE;
        if protection.contains(Protection::READ) {
            uprot |= UniProtection::READ
        }
        if protection.contains(Protection::WRITE) {
            uprot |= UniProtection::WRITE
        }
        if protection.contains(Protection::EXECUTE) {
            uprot |= UniProtection::EXEC
        }
        let mut len = data.len();
        while len % PAGE_ALIGN as usize != 0 {
            len += 1;
        }

        emu.mem_map(*addr as u64, len, uprot).unwrap();
        emu.mem_write(*addr as u64, data.as_slice()).unwrap()
    }

    emu.mem_map(
        STACK_ADDR as u64,
        STACK_SIZE as usize,
        UniProtection::READ | UniProtection::WRITE,
    )
    .unwrap();
    let mut esp = STACK_ADDR + STACK_SIZE - 4;

    let exec_range = match code_and_args {
        CodeToTest::Snippet(code) | CodeToTest::Function(code, _) => {
            let base_addr = entry as u64;
            (base_addr, Some(base_addr + code.len() as u64))
        }
        CodeToTest::ElfFunction(_, _) => (entry as u64, None),
    };

    let mut push = |v: u32| {
        esp -= 4;
        emu.mem_write(esp as u64, &v.to_le_bytes()).unwrap();
    };

    // now write all the args (if any)
    for arg in code_and_args.get_args().iter().rev() {
        push(*arg)
    }
    push(MAGIC_RETURN_ADDR); // return address
    emu.reg_write(RegisterX86::ESP, esp as u64).unwrap();

    let mut mem: Vec<(u64, u64)> = image
        .iter()
        .filter(|h| h.protection.contains(Protection::WRITE))
        .map(|h| (h.addr as u64, h.data.len() as u64))
        .collect();

    mem.push((STACK_ADDR as u64, STACK_SIZE as u64));

    (exec_range.0, exec_range.1, mem)
}

fn execute_unicorn(code: CodeToTest) -> (CpuContext, Vec<(u32, Vec<u8>)>) {
    let mut emu = CpuX86::new(unicorn::Mode::MODE_32).unwrap();

    // emu.add_mem_hook(
    //     MemHookType::MEM_FETCH,
    //     MAGIC_RETURN_ADDR as u64,
    //     MAGIC_RETURN_ADDR as u64,
    //     |emu, _, _, _, _| {
    //         emu.emu_stop().unwrap();
    //         false
    //     },
    // )
    // .unwrap();

    let (base_addr, end, regions) = load_unicorn(&mut emu, code);

    let res = emu.emu_start(base_addr, end.unwrap_or(0), 10 * unicorn::SECOND_SCALE, 0);
    let eip = emu.reg_read(RegisterX86::EIP).unwrap();
    if let Err(e) = res {
        if e == ETCH_UNMAPPED && eip as u32 == MAGIC_RETURN_ADDR {
            // all good
        } else {
            res.unwrap();
        }
    };

    let mut ctx = CpuContext::default();

    ctx.set_gp_reg(
        FullSizeGeneralPurposeRegister::EAX,
        emu.reg_read(RegisterX86::EAX).unwrap() as u32,
    );
    ctx.set_gp_reg(
        FullSizeGeneralPurposeRegister::EBX,
        emu.reg_read(RegisterX86::EBX).unwrap() as u32,
    );
    ctx.set_gp_reg(
        FullSizeGeneralPurposeRegister::ECX,
        emu.reg_read(RegisterX86::ECX).unwrap() as u32,
    );
    ctx.set_gp_reg(
        FullSizeGeneralPurposeRegister::EDX,
        emu.reg_read(RegisterX86::EDX).unwrap() as u32,
    );
    ctx.set_gp_reg(
        FullSizeGeneralPurposeRegister::ESP,
        emu.reg_read(RegisterX86::ESP).unwrap() as u32,
    );
    ctx.set_gp_reg(
        FullSizeGeneralPurposeRegister::EBP,
        emu.reg_read(RegisterX86::EBP).unwrap() as u32,
    );
    ctx.set_gp_reg(
        FullSizeGeneralPurposeRegister::ESI,
        emu.reg_read(RegisterX86::ESI).unwrap() as u32,
    );
    ctx.set_gp_reg(
        FullSizeGeneralPurposeRegister::EDI,
        emu.reg_read(RegisterX86::EDI).unwrap() as u32,
    );

    let flags = emu.reg_read(RegisterX86::EFLAGS).unwrap() as u32;

    ctx.set_flag(Flag::Carry, flags & 0x1 != 0);
    ctx.set_flag(Flag::Parity, flags & 0x2 != 0);
    ctx.set_flag(Flag::AuxiliaryCarry, flags & 0x10 != 0);
    ctx.set_flag(Flag::Zero, flags & 0x40 != 0);
    ctx.set_flag(Flag::Sign, flags & 0x80 != 0);
    ctx.set_flag(Flag::Overflow, flags & 0x800 != 0);

    let mem = regions
        .iter()
        .map(|r| (r.0 as u32, emu.mem_read_as_vec(r.0, r.1 as usize).unwrap()))
        .collect();

    (ctx, mem)
}

fn execute_rusty_x86(code_and_args: CodeToTest) -> (CpuContext, Vec<(u32, Vec<u8>)>) {
    let context = inkwell::context::Context::create();
    let types = &rusty_x86::llvm::backend::Types::new(&context);
    let rt_funs = &rusty_x86::llvm::backend::RuntimeHelpers::dummy(types);
    let (image, entry) = code_and_args.get_code();
    let module = rusty_x86::llvm::recompile(&context, types, rt_funs, &image, entry);

    let entry_name = rusty_x86::llvm::backend::LlvmBuilder::get_name_for(CODE_ADDR);

    const ENTRY_NAME: &str = "entry";

    let entry = module.add_function(ENTRY_NAME, types.bb_fn, None);
    let bb = context.append_basic_block(entry, ENTRY_NAME);

    {
        let builder = context.create_builder();
        builder.position_at_end(bb);

        let args: Vec<BasicMetadataValueEnum> =
            entry.get_params().iter().map(|f| (*f).into()).collect();

        let call = builder.build_call(
            module.get_function(entry_name.as_str()).unwrap(),
            args.as_slice(),
            "res",
        );
        call.set_call_convention(FASTCC_CALLING_CONVENTION);

        builder.build_return(None);
    }

    let _ir = module.print_to_string().to_string();
    // CLion is overwhelmed by this output and breaks
    trace!("llvm ir:\n{}", _ir);

    module.verify().unwrap();

    let execution_engine = module
        .create_jit_execution_engine(
            OptimizationLevel::Aggressive, /* TODO: do we want optimizations? */
        )
        .unwrap();

    let fun: JitFunction<BbFunc> = unsafe { execution_engine.get_function(ENTRY_NAME).unwrap() };

    let mut cpu_context = CpuContext::default();

    // SAFETY: dragons ahead
    // map 4 GiB of memory with no protection
    // this way we can control all mappings in the whole virtualized 32-bit address space
    let mut target_mem_region = region::alloc(0x100000000, region::Protection::NONE).unwrap();

    let map_region = |addr: u32, protection: Protection, data: &[u8]| -> Allocation {
        let addr = unsafe { target_mem_region.as_ptr::<u8>().add(addr as usize) };

        let mut rprot = region::Protection::NONE;
        if protection.contains(Protection::READ) {
            rprot |= region::Protection::READ
        }
        if protection.contains(Protection::WRITE) {
            rprot |= region::Protection::WRITE
        }
        let mut len = data.len();
        while len % PAGE_ALIGN as usize != 0 {
            len += 1;
        }

        // firstly map the page as read-write to pre-fill it with our data
        let mut alloc = region::alloc_at(addr, len, region::Protection::READ_WRITE).unwrap();

        unsafe {
            std::slice::from_raw_parts_mut(alloc.as_mut_ptr(), data.len()).copy_from_slice(data)
        };

        // now map the page as it will be used by the target
        unsafe { region::protect(alloc.as_ptr::<u8>(), alloc.len(), rprot).unwrap() };

        alloc
    };

    let mut allocated_regions: Vec<Allocation> = image
        .iter()
        .map(
            |MemoryImageItem {
                 addr,
                 protection,
                 data,
             }| { map_region(*addr, *protection, data.as_slice()) },
        )
        .collect();

    allocated_regions.push(map_region(
        STACK_ADDR,
        Protection::READ_WRITE,
        &[0u8; STACK_SIZE as usize],
    ));

    let mut esp = STACK_ADDR + STACK_SIZE - 4;

    let mut push = |v: u32| {
        esp -= 4;
        unsafe {
            // TODO: do we break any language rules by shuffling pointers around this way?
            let ptr = target_mem_region.as_mut_ptr::<u8>().add(esp as usize);
            std::slice::from_raw_parts_mut(ptr, 4).copy_from_slice(&v.to_le_bytes());
        }
    };

    // now write all the args (if any)
    if let CodeToTest::Function(_, args) = code_and_args {
        for arg in args.iter().rev() {
            push(*arg)
        }
    }
    push(MAGIC_RETURN_ADDR); // return address

    cpu_context.set_gp_reg(FullSizeGeneralPurposeRegister::ESP, esp);

    unsafe {
        // do the thing!
        fun.call(&mut cpu_context, target_mem_region.as_mut_ptr());
    };

    let mem = image
        .iter()
        .filter(|h| h.protection.contains(Protection::WRITE))
        .chain(
            [MemoryImageItem {
                addr: STACK_ADDR,
                protection: Protection::READ_WRITE,
                data: vec![0u8; STACK_SIZE as usize],
            }]
            .iter(),
        )
        .map(|h| unsafe {
            let ptr = target_mem_region.as_mut_ptr::<u8>().add(h.addr as usize);
            let mem = std::slice::from_raw_parts(ptr, h.data.len());
            (h.addr, mem.to_vec())
        })
        .collect();

    (cpu_context, mem)
}

fn context_to_gp_map(context: &CpuContext) -> BTreeMap<FullSizeGeneralPurposeRegister, u32> {
    FullSizeGeneralPurposeRegister::iter()
        .map(|reg| (reg, context.get_gp_reg(reg)))
        .collect()
}

fn context_to_flag_list(context: &CpuContext, flags: &[Flag]) -> Vec<Flag> {
    Flag::iter()
        .filter(|flag| flags.contains(flag) && context.get_flag(*flag))
        .collect()
}

pub fn test_code(code: CodeToTest, flags: Vec<Flag>) {
    // TODO: make it work
    // debug!(
    //     "CODE:\n{}",
    //     rusty_x86::disasm::disassemble(code.get_code())
    // );

    let unicorn = execute_unicorn(code.clone());

    // TODO: custom dumps with more control (over addresses, for example)
    let unicorn_mem = unicorn
        .1
        .iter()
        .map(|mem| format!("0x{:08x}:\n{}\n", mem.0, pretty_hex::pretty_hex(&mem.1)))
        .fold("".to_string(), |acc, el| acc + el.as_str());

    //debug!("MEM:\n{}", unicorn_mem);

    let rusty_x86 = execute_rusty_x86(code);

    let rusty_x86_mem = rusty_x86
        .1
        .iter()
        .map(|mem| format!("0x{:08x}:\n{}\n", mem.0, pretty_hex::pretty_hex(&mem.1)))
        .fold("".to_string(), |acc, el| acc + el.as_str());

    debug!("RESULT rusty_x86 = {:?}", rusty_x86.0);
    debug!("RESULT unicorn   = {:?}", unicorn.0);

    debug!("Limiting flags to the following: {:?}", flags);

    // We can't directly compare contexts because of flags (sometimes they are undefined on x86)
    // So we compare separately the values of registers and specified flags
    let rusty_x86_gp = context_to_gp_map(&rusty_x86.0);
    let unicorn_gp = context_to_gp_map(&unicorn.0);

    assert_eq!(rusty_x86_gp, unicorn_gp);

    let rusty_x86_flags = context_to_flag_list(&rusty_x86.0, flags.as_slice());
    let unicorn_flags = context_to_flag_list(&unicorn.0, flags.as_slice());

    debug!("FLAGS (filtered) unicorn   = {:?}", unicorn_flags);
    debug!("FLAGS (filtered) rusty_x86 = {:?}", rusty_x86_flags);

    assert_eq!(rusty_x86_flags, unicorn_flags);

    assert_eq!(rusty_x86_mem, unicorn_mem);
}

macro_rules! parse_flag {
    (CF) => {
        rusty_x86::types::Flag::Carry
    };
    (ZF) => {
        rusty_x86::types::Flag::Zero
    };
    (SF) => {
        rusty_x86::types::Flag::Sign
    };
    (OF) => {
        rusty_x86::types::Flag::Overflow
    };
}

macro_rules! test_snippet {
    ($name:ident: ($($code:tt)*) [$($flags:ident)*]) => {
        #[test_log::test]
        fn $name() {
            log::info!("Running {}", stringify!($name));
            let code = rusty_x86::assemble_x86!(
                $($code)*
            );
            crate::common::test_code(crate::common::CodeToTest::Snippet(code.as_slice()), vec![$(parse_flag!($flags)),*]);
        }
    };
}

#[macro_export]
macro_rules! test_snippets {
    () => {};
    ($name:ident: ($($code:tt)*) [$($flags:ident)*], $($xs:tt)*) => {
        test_snippet!($name: ($($code)*) [$($flags)*]);
        test_snippets!($($xs)*);
    };
    ($name:ident: ($($code:tt)*), $($xs:tt)*) => {
        test_snippet!($name: ($($code)*) []);
        test_snippets!($($xs)*);
    };
}

#[macro_export]
macro_rules! test_functions {
    () => {};
    ($name:ident: [$(($($arg:expr),*)),*] ($($code:tt)*), $($xs:tt)*) => {
        mod $name {

            fn get_code() -> Vec<u8> {
                rusty_x86::assemble_x86!(
                    $($code)*
                )
            }

            $(
                paste::paste! {
                    #[test_log::test]
                    #[allow(non_snake_case)]
                    fn [<on_ $($arg)_*>] () {
                        #[allow(unused)]
                        use super::*; // hygiene???

                        let args: &[u32] = &[$($arg as u32),*];
                        log::info!("Running {} on {:?}", stringify!($name), args);

                        let code = get_code();

                        crate::common::test_code(crate::common::CodeToTest::Function(code.as_slice(), args), vec![]);
                    }
                }
            )*
        }

        test_functions!($($xs)*);
    };
}
