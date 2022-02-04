use inkwell::execution_engine::JitFunction;
use inkwell::values::BasicMetadataValueEnum;
use inkwell::OptimizationLevel;
use log::debug;
use rusty_x86::llvm::backend::{BbFunc, FASTCC_CALLING_CONVENTION};
use rusty_x86::types::{CpuContext, Flag, FullSizeGeneralPurposeRegister};
use std::collections::BTreeMap;
use std::io::Write;
use strum::IntoEnumIterator;
use unicorn;
use unicorn::Error::ETCH_UNMAPPED;
use unicorn::{Cpu, CpuX86, MemHookType, Protection, RegisterX86};

pub const CODE_ADDR: u32 = 0x200000;
pub const MEM_ADDR: u32 = 0x100000;
pub const MEM_SIZE: u32 = 0x10000;
pub const MAGIC_RETURN_ADDR: u32 = 0xCAFEBABE;

#[derive(Clone)]
pub enum CodeToTest<'a> {
    Snippet(&'a [u8]),             // just the code
    Function(&'a [u8], &'a [u32]), // code & args
}

impl<'a> CodeToTest<'a> {
    pub fn get_code(&self) -> &'a [u8] {
        match self {
            CodeToTest::Snippet(c) => c,
            CodeToTest::Function(c, _) => c,
        }
    }
}

fn execute_unicorn(code: CodeToTest) -> (CpuContext, Vec<u8>) {
    let pad = (0x1000 - (code.get_code().len() % 0x1000)) % 0x1000;
    let code_page_size = code.get_code().len() + pad;

    let mut emu = CpuX86::new(unicorn::Mode::MODE_32).unwrap();

    let base_addr = CODE_ADDR as u64;

    // map the code
    emu.mem_map(
        base_addr,
        code_page_size,
        Protection::READ | Protection::EXEC,
    )
    .unwrap();
    emu.mem_write(base_addr, code.get_code()).unwrap();

    emu.mem_map(
        MEM_ADDR as u64,
        MEM_SIZE as usize,
        Protection::READ | Protection::WRITE,
    )
    .unwrap();

    let mut esp = MEM_ADDR + MEM_SIZE - 4;

    let mut push = |v: u32| {
        esp = esp - 4;
        emu.mem_write(esp as u64, &v.to_le_bytes()).unwrap();
    };

    // now write all the args (if any)
    if let CodeToTest::Function(_, args) = code {
        for arg in args.iter().rev() {
            push(*arg)
        }
    }
    push(MAGIC_RETURN_ADDR); // return address

    emu.reg_write(RegisterX86::ESP, esp as u64).unwrap();

    emu.add_mem_hook(
        MemHookType::MEM_FETCH,
        MAGIC_RETURN_ADDR as u64,
        MAGIC_RETURN_ADDR as u64,
        |emu, _, _, _, _| {
            emu.emu_stop().unwrap();
            false
        },
    )
    .unwrap();

    // TODO: can we stop on return? Maybe hooks?
    let res = emu.emu_start(
        base_addr,
        base_addr + code.get_code().len() as u64,
        10 * unicorn::SECOND_SCALE,
        0,
    );
    if let Err(e) = res {
        if e == ETCH_UNMAPPED && emu.reg_read(RegisterX86::EIP).unwrap() as u32 == MAGIC_RETURN_ADDR
        {
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

    let mem = emu
        .mem_read_as_vec(MEM_ADDR as u64, MEM_SIZE as usize)
        .unwrap();

    (ctx, mem)
}

fn execute_rusty_x86(code: CodeToTest) -> (CpuContext, Vec<u8>) {
    let context = inkwell::context::Context::create();
    let types = &rusty_x86::llvm::backend::Types::new(&context);
    let module = rusty_x86::llvm::recompile(&context, types, CODE_ADDR, code.get_code());

    let entry_name = rusty_x86::llvm::backend::LlvmBuilder::get_name_for(CODE_ADDR);

    const ENTRY_NAME: &str = "entry";

    let entry = module.add_function(ENTRY_NAME, types.bb_fn, None);
    let bb = context.append_basic_block(entry, ENTRY_NAME);

    {
        let builder = context.create_builder();
        builder.position_at_end(bb);

        let args: Vec<BasicMetadataValueEnum> = entry
            .get_params()
            .iter()
            .map(|f| f.clone().into())
            .collect();

        let call = builder.build_call(
            module.get_function(entry_name.as_str()).unwrap(),
            args.as_slice(),
            "res",
        );
        call.set_call_convention(FASTCC_CALLING_CONVENTION);

        builder.build_return(None);
    }

    let ir = module.print_to_string().to_string();
    debug!("llvm ir:\n{}", ir);

    module.verify().unwrap();

    let execution_engine = module
        .create_jit_execution_engine(
            OptimizationLevel::None, /* TODO: do we want optimizations? */
        )
        .unwrap();

    let fun: JitFunction<BbFunc> = unsafe { execution_engine.get_function(ENTRY_NAME).unwrap() };

    let mut cpu_context = CpuContext::default();

    // SAFETY: dragons ahead
    // map 4 GiB of memory with no protection
    // this way we can control all mappings in the whole virtualized 32-bit address space
    let mut target_mem_region = region::alloc(0x100000000, region::Protection::NONE).unwrap();

    // map a small region of memory
    // TODO: add is UB if we somehow get out of bounds of allocated object. What is object exactly in this case?
    let mapped_start = unsafe { target_mem_region.as_ptr::<u8>().add(MEM_ADDR as usize) };

    let mut mapped = region::alloc_at(
        mapped_start,
        MEM_SIZE as usize,
        region::Protection::READ_WRITE,
    )
    .unwrap();

    // fill the mapped memory with zeroes (is it necessary?)
    // TODO: do we break any language rules by shuffling pointers around this way?
    unsafe { std::slice::from_raw_parts_mut(mapped.as_mut_ptr(), mapped.len()).fill(0u8) }

    let mut esp = MEM_ADDR + MEM_SIZE - 4;

    let mut push = |v: u32| {
        esp = esp - 4;
        unsafe {
            // TODO: do we break any language rules by shuffling pointers around this way?
            let ptr = target_mem_region.as_mut_ptr::<u8>().add(esp as usize);
            std::slice::from_raw_parts_mut(ptr, 4)
                .write(&v.to_le_bytes())
                .unwrap();
        }
    };

    // now write all the args (if any)
    if let CodeToTest::Function(_, args) = code {
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

    let mem = unsafe { std::slice::from_raw_parts(mapped_start, MEM_SIZE as usize) };

    (cpu_context, mem.into())
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
    debug!(
        "CODE:\n{}",
        rusty_x86::disasm::disassemble(code.get_code(), CODE_ADDR as u64)
    );

    let unicorn = execute_unicorn(code.clone());
    let rusty_x86 = execute_rusty_x86(code);

    debug!("RESULT rusty_x86 = {:?}", rusty_x86.0);
    debug!("RESULT unicorn   = {:?}", unicorn.0);

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

    let rusty_x86_mem = pretty_hex::pretty_hex(&rusty_x86.1);
    let unicorn_mem = pretty_hex::pretty_hex(&unicorn.1);

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
                    fn [<on_ $($arg)_*>] () {
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
