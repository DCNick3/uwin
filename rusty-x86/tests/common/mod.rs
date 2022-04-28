mod loader;

use inkwell::execution_engine::JitFunction;
use inkwell::values::BasicMetadataValueEnum;
use inkwell::OptimizationLevel;
use log::{debug, error, trace};
use memchr::memchr;
use memory_image::{MemoryImage, MemoryImageItem, Protection};
use region::Allocation;
use rusty_x86::interp::interpret_simple;
use rusty_x86::llvm::backend::FASTCC_CALLING_CONVENTION;
use rusty_x86::types::{BbFunc, CpuContext, Flag, FullSizeGeneralPurposeRegister};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashSet};
use std::sync::Arc;
use strum::IntoEnumIterator;
use unicorn;
use unicorn::Error::{ETCH_UNMAPPED, EXCEPTION};
use unicorn::{CodeHookType, Cpu, CpuX86, Protection as UniProtection, RegisterX86};

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
    ElfFunction(&'a [u8], &'a [u32]), // elf contents, args
    PeFunction(&'a [u8], &'a [u32]),  // pe contents, args
}

impl<'a> CodeToTest<'a> {
    pub fn get_code(&self) -> (MemoryImage, u32) {
        let mut image = MemoryImage::new();
        let entry;
        match self {
            CodeToTest::Snippet(c) | CodeToTest::Function(c, _) => {
                image.add_region(
                    CODE_ADDR,
                    Protection::READ_EXECUTE,
                    c.to_vec(),
                    "code".to_string(),
                );
                image.add_zeroed_region(
                    MEM_ADDR,
                    Protection::READ_WRITE,
                    MEM_SIZE,
                    "data".to_string(),
                );
                entry = CODE_ADDR;
            }
            CodeToTest::ElfFunction(elf, _) => {
                let (entry_, image_) = loader::load_elf(elf).unwrap();
                image = image_;
                entry = entry_;
            }
            CodeToTest::PeFunction(pe, _) => {
                let (entry_, image_) = loader::load_pe(pe).unwrap();
                image = image_;
                entry = entry_;
            }
        };
        (image, entry)
    }

    pub fn get_args(&self) -> Vec<u32> {
        match self {
            CodeToTest::Snippet(_) => vec![],
            CodeToTest::Function(_, args)
            | CodeToTest::ElfFunction(_, args)
            | CodeToTest::PeFunction(_, args) => args.to_vec(),
        }
    }
}

fn load_unicorn(emu: &mut CpuX86, code_and_args: CodeToTest) -> (u64, Option<u64>, MemoryImage) {
    let (image, entry) = code_and_args.get_code();

    for MemoryImageItem {
        addr,
        protection,
        data,
        ..
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

        // actually, elf allows addresses that are not aligned
        // https://stackoverflow.com/questions/46117065/required-alignment-of-text-versus-data
        // the alignment between offset and addr just should be the same
        // so here we go...
        let addr_aligned = *addr - *addr % PAGE_ALIGN;

        trace!("MAP {:08x} {:08x} {:?}", addr_aligned, len, uprot);
        emu.mem_map(addr_aligned as u64, len, uprot).unwrap();
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
        CodeToTest::ElfFunction(_, _) | CodeToTest::PeFunction(_, _) => (entry as u64, None),
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

    let image = {
        let mut image = image;
        image.add_zeroed_region(
            STACK_ADDR,
            Protection::READ_WRITE,
            STACK_SIZE,
            "stack".to_string(),
        );
        image
    };

    (exec_range.0, exec_range.1, image)
}

fn execute_unicorn(code: CodeToTest) -> (CpuContext, MemoryImage, Vec<u32>) {
    let mut emu = CpuX86::new(unicorn::Mode::MODE_32).unwrap();

    // collect basic block addresses to use in lifting by rusty_x86
    let basic_blocks = Arc::new(RefCell::new(HashSet::new()));

    let local_basic_blocks = basic_blocks.clone();
    emu.add_code_hook(CodeHookType::BLOCK, 1, 0, move |_, addr, _| {
        local_basic_blocks.borrow_mut().insert(addr as u32);
    })
    .unwrap();

    let (base_addr, end, image) = load_unicorn(&mut emu, code);

    let res = emu.emu_start(base_addr, end.unwrap_or(0), 10 * unicorn::SECOND_SCALE, 0);
    let eip = emu.reg_read(RegisterX86::EIP).unwrap();
    if let Err(e) = res {
        if (e == ETCH_UNMAPPED || e == EXCEPTION) && eip as u32 == MAGIC_RETURN_ADDR {
            // all good
        } else {
            error!("Something bad happened with eip @ 0x{eip:08x}");
            res.unwrap();
        }
    };

    let mut ctx = CpuContext::default();

    // convert unicorn context to rust_x86 context
    {
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
    }

    let mem: MemoryImage = image
        .iter()
        .filter(|h| h.protection.contains(Protection::WRITE))
        .map(|h| {
            let mem = emu.mem_read_as_vec(h.addr as u64, h.data.len()).unwrap();
            MemoryImageItem {
                addr: h.addr,
                protection: h.protection,
                comment: h.comment.clone(),
                data: mem,
            }
        })
        .collect();

    let mut basic_blocks: Vec<_> = basic_blocks.take().into_iter().collect();

    basic_blocks.sort_unstable();

    (ctx, mem, basic_blocks)
}

fn execute_rusty_x86_llvm(
    cpu_context: &mut CpuContext,
    memory_base: *mut u8,
    basic_blocks: &[u32],
    image: &MemoryImage,
    entry: u32,
) {
    let context = inkwell::context::Context::create();
    let types = rusty_x86::llvm::backend::Types::new(&context);
    let thunk_functions = &BTreeMap::new();
    let module = rusty_x86::llvm::recompile(
        &context,
        types.clone(),
        thunk_functions,
        image,
        basic_blocks,
    );

    // workaround for https://github.com/TheDan64/inkwell/issues/320
    inkwell::execution_engine::ExecutionEngine::link_in_mc_jit();

    let entry_name = rusty_x86::llvm::backend::LlvmBuilder::get_name_for(entry);

    debug!("rusty_x86 finished, adding an entry");

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

        builder.build_return(Some(&call.try_as_basic_value().unwrap_left()));
    }

    let function_count = module.get_functions().count();
    // do not print large modules (not that the IR is useful at these amounts anyways)
    if function_count < 50 {
        let _ir = module.print_to_string().to_string();
        // CLion is overwhelmed by this output and breaks
        trace!("llvm ir:\n{}", _ir);
    }

    module.verify().unwrap();

    debug!("Compiling the module...");
    let execution_engine = module
        .create_jit_execution_engine(
            OptimizationLevel::Aggressive, /* TODO: do we want optimizations? */
        )
        .expect("Could not get LLVM execution engine");

    let fun: JitFunction<BbFunc> = unsafe { execution_engine.get_function(ENTRY_NAME).unwrap() };

    debug!("Executing!");
    unsafe {
        // do the thing!
        fun.call(cpu_context, memory_base);
    };
}

fn execute_rusty_x86(
    code_and_args: CodeToTest,
    basic_blocks: &[u32],
    backend: Backend,
) -> (CpuContext, MemoryImage) {
    let (image, entry) = code_and_args.get_code();

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
                 ..
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
    for arg in code_and_args.get_args().iter().rev() {
        push(*arg)
    }
    push(MAGIC_RETURN_ADDR); // return address

    cpu_context.set_gp_reg(FullSizeGeneralPurposeRegister::ESP, esp);

    debug!("Running test with {:?}", backend);
    match backend {
        Backend::Llvm => execute_rusty_x86_llvm(
            &mut cpu_context,
            target_mem_region.as_mut_ptr(),
            basic_blocks,
            &image,
            entry,
        ),
        Backend::Interp => unsafe {
            interpret_simple(&mut cpu_context, target_mem_region.as_mut_ptr(), entry);
        },
    }
    debug!("Done running!");

    let mem: MemoryImage = image
        .iter()
        .filter(|h| h.protection.contains(Protection::WRITE))
        .chain(
            [MemoryImageItem {
                addr: STACK_ADDR,
                protection: Protection::READ_WRITE,
                data: vec![0u8; STACK_SIZE as usize],
                comment: "stack".to_string(),
            }]
            .iter(),
        )
        .map(|h| unsafe {
            let ptr = target_mem_region.as_mut_ptr::<u8>().add(h.addr as usize);
            let mem = std::slice::from_raw_parts(ptr, h.data.len());
            MemoryImageItem {
                addr: h.addr,
                protection: h.protection,
                comment: h.comment.clone(),
                data: mem.to_vec(),
            }
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

#[derive(Debug)]
pub enum Backend {
    Llvm,
    Interp,
}

pub fn test_code(code: CodeToTest, flags: Vec<Flag>, backend: Backend) {
    let unicorn = execute_unicorn(code.clone());

    let unicorn_mem = unicorn.1.dump().to_string();

    let rusty_x86 = execute_rusty_x86(code, &unicorn.2, backend);

    let rusty_x86_mem = rusty_x86.1.dump().to_string();

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

    if rusty_x86_mem != unicorn_mem {
        eprintln!("rusty_x86 and unicorn execution produced different memory images!");
        eprintln!("Diff (rusty_x86 vs unicorn):");
        for (left, right) in rusty_x86_mem.lines().zip(unicorn_mem.lines()) {
            if left != right {
                eprintln!("-{left}");
                eprintln!("+{right}");
            }
        }
        panic!("rusty_x86 and unicorn execution produced different memory images");
    }
}

fn dump_string(image: &MemoryImage, address: u32) -> String {
    let str_data = image.read_all_at(address);
    let nul_index = memchr(0, str_data).unwrap();

    std::str::from_utf8(&str_data[..nul_index])
        .unwrap()
        .to_string()
}

pub fn test_outcode(elf_bytes: &[u8], backend: Backend) {
    let code = CodeToTest::ElfFunction(elf_bytes, &[]);

    let elf = goblin::elf::Elf::parse(elf_bytes).unwrap();
    let output_sym = elf
        .syms
        .iter()
        .find(|s| &elf.strtab[s.st_name] == "output")
        .unwrap();
    let output_addr = output_sym.st_value as u32;

    let unicorn = execute_unicorn(code.clone());

    let unicorn_mem = unicorn.1;
    let unicorn_output = dump_string(&unicorn_mem, output_addr);

    let rusty_x86 = execute_rusty_x86(code, &unicorn.2, backend);

    let rusty_x86_mem = rusty_x86.1;
    let rusty_x86_output = dump_string(&rusty_x86_mem, output_addr);

    if rusty_x86_output != unicorn_output {
        eprintln!("rusty_x86 and unicorn produced different outputs!");
        eprintln!("Diff (rusty_x86 vs unicorn):");
        for (left, right) in rusty_x86_output.lines().zip(unicorn_output.lines()) {
            if left != right {
                eprintln!("-{left}");
                eprintln!("+{right}");
            }
        }
        panic!("rusty_x86 and unicorn produced different outputs");
    }
}
