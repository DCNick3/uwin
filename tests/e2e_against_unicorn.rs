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

const CODE_ADDR: u32 = 0x200000;
const MEM_ADDR: u32 = 0x100000;
const MEM_SIZE: u32 = 0x10000;
const MAGIC_RETURN_ADDR: u32 = 0xCAFEBABE;

#[derive(Clone)]
enum CodeToTest<'a> {
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

fn test_code(code: CodeToTest, flags: Vec<Flag>) {
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
            crate::test_code(crate::CodeToTest::Snippet(code.as_slice()), vec![$(parse_flag!($flags)),*]);
        }
    };
}

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

                        crate::test_code(crate::CodeToTest::Function(code.as_slice(), args), vec![]);
                    }
                }
            )*
        }

        test_functions!($($xs)*);
    };
}

mod instr {
    mod mov {
        test_snippets! {

            /* test name */
            mov_eax_42: (
                /* test body */
                ; mov eax, 42

            /* optional list of flags to test */
            ) [CF ZF SF OF],
            mov_ebx_42: (
                ; mov ebx, 42
            ) [CF ZF SF OF],
        }
    }

    // TODO: do we test all flag combinations (ditto for and)?
    mod sub {
        test_snippets! {
            sub_borrow: (
                ; mov eax, 1
                ; sub eax, 2
            ) [CF ZF SF OF],
            sub_branch_sign: (
                ; mov eax, 1
                ; sub eax, 2
                ; js ->L1 // TODO: cmov is more concise?
                ; mov ebx, 1
                ; jmp ->R
                ; ->L1:
                ; mov ebx, 2
                ; ->R:
                ; mov edx, 1 // necessary because of funky control flow at the end of test snippets...
            ) [CF ZF SF OF],
            sub_cmov_sign: (
                ; mov eax, 1
                ; sub eax, 2
                ; mov ecx, 2
                ; cmovs ebx, ecx
            ) [CF ZF SF OF],
            sub_cmov_sign_2: (
                ; mov eax, 3
                ; sub eax, 2
                ; mov ecx, 2
                ; cmovs ebx, ecx
            ) [CF ZF SF OF],
        }
    }

    mod add {
        test_snippets! {
            add_borrow: (
                ; mov eax, 1
                ; add eax, 2
            ) [CF ZF SF OF],
            add_branch_sign: (
                ; mov eax, 1
                ; add eax, 2
                ; js ->L1 // TODO: cmov is more concise?
                ; mov ebx, 1
                ; jmp ->R
                ; ->L1:
                ; mov ebx, 2
                ; ->R:
                ; mov edx, 1 // necessary because of funky control flow at the end of test snippets...
            ) [CF ZF SF OF],
            add_cmov_sign: (
                ; mov eax, 1
                ; add eax, 2
                ; mov ecx, 2
                ; cmovs ebx, ecx
            ) [CF ZF SF OF],
            add_cmov_sign_2: (
                ; mov eax, 3
                ; add eax, 2
                ; mov ecx, 2
                ; cmovs ebx, ecx
            ) [CF ZF SF OF],
        }
    }

    mod cmp {
        test_snippets! {
            cmp_cmov_eq: (
                ; mov eax, 12
                ; cmp eax, 12
                ; mov ecx, 2
                ; cmovz ebx, ecx
            ) [CF ZF SF OF],
            cmp_cmov_eq_2: (
                ; mov eax, 12
                ; cmp eax, 13
                ; mov ecx, 2
                ; cmovz ebx, ecx
            ) [CF ZF SF OF],
            cmp_less: (
                ; mov eax, 11
                ; cmp eax, 13
            ) [CF ZF SF OF],
            cmp_neg_1: (
                ; mov eax, -1
                ; cmp eax, -2
            ) [CF ZF SF OF],
            cmp_neg_2: (
                ; mov eax, 0
                ; cmp eax, 1
            ) [CF ZF SF OF],
            cmp_neg_3: (
                ; mov eax, -0x80000000
                ; cmp eax, 1
            ) [CF ZF SF OF],
            cmp_rnd_1: (
                ; mov eax, 0x3e9c87ab
                ; cmp eax, 0x47f38608
            ) [CF ZF SF OF],
            cmp_rnd_2: (
                ; mov eax, -0x403f0352
                ; cmp eax, -0x4440a37e
            ) [CF ZF SF OF],
            cmp_rnd_3: (
                ; mov eax, 0x2600bb16
                ; cmp eax, 0x73fc32b6
            ) [CF ZF SF OF],
        }
    }

    mod lea {
        test_snippets! {
            lea_disp: (
                ; mov eax, 1228
                ; lea ecx, [eax + 7]
            ),
            lea_idx: (
                ; mov eax, 1228
                ; mov ebx, 337
                ; lea ecx, [eax + ebx*4]
            ),
            lea_idx_disp: (
                ; mov eax, 1228
                ; mov ebx, 337
                ; lea ecx, [eax + ebx*4 + 7]
            ),
        }
    }

    mod mem {
        test_snippets! {
            mem_basic_rw: (
                ; mov eax, 42
                ; mov eax, [crate::MEM_ADDR as i32]
                ; mov [crate::MEM_ADDR as i32], ebx
            ),
        }
    }

    mod imul {
        test_snippets! {
            // imul_1op_eax_eax: (
            //     ; mov eax, 23
            //     ; imul eax
            // ) [CF OF],
            // imul_1op: (
            //     ; mov eax, 23
            //     ; mov ebx, 24
            //     ; imul ebx
            // ) [CF OF],
            // imul_1op_overflow: (
            //     ; mov eax, 0x7fffffff
            //     ; mov ebx, 0x7fffffff
            //     ; imul ebx
            // ) [CF OF],

            imul_2op_eax_eax: (
                ; mov eax, 23
                ; imul eax, eax
            ) [CF OF],
            imul_2op: (
                ; mov eax, 23
                ; mov ebx, 24
                ; imul eax, ebx
            ) [CF OF],
            imul_2op_overflow: (
                ; mov eax, 0x7fffffff
                ; mov ebx, 0x7fffffff
                ; imul eax, ebx
            ) [CF OF],
            imul_2op_rnd1: (
                ; mov eax, -0x2c333634
                ; mov ebx, 0x47ec9023
                ; imul eax, ebx
            ) [CF OF],
            imul_2op_rnd2: (
                ; mov eax, -0x23f11f0a
                ; mov ebx, -0x2073452e
                ; imul eax, ebx
            ) [CF OF],
            imul_2op_rnd3: (
                ; mov eax, 0x4f0e4a0c
                ; mov ebx, -0xefd25f
                ; imul eax, ebx
            ) [CF OF],

            // imul_3op_eax_eax: (
            //     ; mov eax, 23
            //     ; imul eax, eax, 24
            // ) [CF OF],
            // imul_3op: (
            //     ; mov ebx, 24
            //     ; imul eax, ebx, 23
            // ) [CF OF],
            // imul_3op_overflow: (
            //     ; mov ebx, 0x7fffffff
            //     ; imul eax, ebx, 0x7fffffff
            // ) [CF OF],
        }
    }

    mod xor {
        test_snippets! {
            xor_zero_eax: (
                ; mov eax, 228
                ; xor eax, eax
            ) [CF ZF SF OF],
            xor_zero_eax_with_ebx: (
                ; mov eax, 228
                ; mov ebx, 228
                ; xor eax, ebx
            ) [CF ZF SF OF],
            xor_eax_ebx_rnd1: (
                ; mov eax, 0x79d1e0e9
                ; mov ebx, -0x16d29593
                ; xor eax, ebx
            ) [CF ZF SF OF],
            xor_eax_ebx_rnd2: (
                ; mov eax, 0x79f9322a
                ; mov ebx, 0x801efd8
                ; xor eax, ebx
            ) [CF ZF SF OF],
        }
    }
    mod and {
        test_snippets! {
            and_same_eax_eax: (
                ; mov eax, 228
                ; and eax, eax
            ) [CF ZF SF OF],
            and_same_eax_ebx: (
                ; mov eax, 228
                ; mov ebx, 228
                ; and eax, ebx
            ) [CF ZF SF OF],
            and_eax_ebx_rnd1: (
                ; mov eax, 0x79d1e0e9
                ; mov ebx, -0x16d29593
                ; and eax, ebx
            ) [CF ZF SF OF],
            and_eax_ebx_rnd2: (
                ; mov eax, 0x79f9322a
                ; mov ebx, 0x801efd8
                ; and eax, ebx
            ) [CF ZF SF OF],
        }
    }

    mod test {
        test_snippets! {
            test_same_eax_eax: (
                ; mov eax, 228
                ; test eax, eax
            ) [CF ZF SF OF],
            test_same_eax_ebx: (
                ; mov eax, 228
                ; mov ebx, 228
                ; test eax, ebx
            ) [CF ZF SF OF],
            test_eax_ebx_rnd1: (
                ; mov eax, 0x79d1e0e9
                ; mov ebx, -0x16d29593
                ; test eax, ebx
            ) [CF ZF SF OF],
            test_eax_ebx_rnd2: (
                ; mov eax, 0x79f9322a
                ; mov ebx, 0x801efd8
                ; test eax, ebx
            ) [CF ZF SF OF],
        }
    }

    mod shr {
        test_snippets! {
            shr_zero: (
                ; mov eax, 228
                ; shr eax, 0
            ) [CF ZF SF OF],

            shr_228_one: (
                ; mov eax, 228
                ; shr eax, 1
            ) [CF ZF SF OF],
            shr_229_one: (
                ; mov eax, 229
                ; shr eax, 1
            ) [CF ZF SF OF],
            shr_neg_228_one: (
                ; mov eax, -228
                ; shr eax, 1
            ) [CF ZF SF OF],
            shr_neg_229_one: (
                ; mov eax, -229
                ; shr eax, 1
            ) [CF ZF SF OF],

            shr_228_two: (
                ; mov eax, 228
                ; shr eax, 2
            ) [CF ZF SF],
            shr_229_two: (
                ; mov eax, 229
                ; shr eax, 2
            ) [CF ZF SF],
            shr_neg_228_two: (
                ; mov eax, -228
                ; shr eax, 2
            ) [CF ZF SF],
            shr_neg_229_two: (
                ; mov eax, -229
                ; shr eax, 2
            ) [CF ZF SF],

            shr_228_zero_wrap: (
                ; mov eax, 228
                ; shr eax, 32
            ) [CF ZF SF OF],

            shr_228_one_wrap: (
                ; mov eax, 228
                ; shr eax, 33
            ) [CF ZF SF OF],
            shr_229_one_wrap: (
                ; mov eax, 229
                ; shr eax, 33
            ) [CF ZF SF OF],
            shr_neg_228_one_wrap: (
                ; mov eax, -228
                ; shr eax, 33
            ) [CF ZF SF OF],
            shr_neg_229_one_wrap: (
                ; mov eax, -229
                ; shr eax, 33
            ) [CF ZF SF OF],

            shr_228_two_wrap: (
                ; mov eax, 228
                ; shr eax, 34
            ) [CF ZF SF],
            shr_229_two_wrap: (
                ; mov eax, 229
                ; shr eax, 34
            ) [CF ZF SF],
            shr_neg_228_two_wrap: (
                ; mov eax, -228
                ; shr eax, 34
            ) [CF ZF SF],
            shr_neg_229_two_wrap: (
                ; mov eax, -229
                ; shr eax, 34
            ) [CF ZF SF],
        }
    }

    mod sar {
        test_snippets! {
            sar_zero: (
                ; mov eax, 228
                ; sar eax, 0
            ) [CF ZF SF OF],

            sar_228_one: (
                ; mov eax, 228
                ; sar eax, 1
            ) [CF ZF SF OF],
            sar_229_one: (
                ; mov eax, 229
                ; sar eax, 1
            ) [CF ZF SF OF],
            sar_neg_228_one: (
                ; mov eax, -228
                ; sar eax, 1
            ) [CF ZF SF OF],
            sar_neg_229_one: (
                ; mov eax, -229
                ; sar eax, 1
            ) [CF ZF SF OF],

            sar_228_two: (
                ; mov eax, 228
                ; sar eax, 2
            ) [CF ZF SF],
            sar_229_two: (
                ; mov eax, 229
                ; sar eax, 2
            ) [CF ZF SF],
            sar_neg_228_two: (
                ; mov eax, -228
                ; sar eax, 2
            ) [CF ZF SF],
            sar_neg_229_two: (
                ; mov eax, -229
                ; sar eax, 2
            ) [CF ZF SF],

            sar_228_zero_wrap: (
                ; mov eax, 228
                ; sar eax, 32
            ) [CF ZF SF OF],

            sar_228_one_wrap: (
                ; mov eax, 228
                ; sar eax, 33
            ) [CF ZF SF OF],
            sar_229_one_wrap: (
                ; mov eax, 229
                ; sar eax, 33
            ) [CF ZF SF OF],
            sar_neg_228_one_wrap: (
                ; mov eax, -228
                ; sar eax, 33
            ) [CF ZF SF OF],
            sar_neg_229_one_wrap: (
                ; mov eax, -229
                ; sar eax, 33
            ) [CF ZF SF OF],

            sar_228_two_wrap: (
                ; mov eax, 228
                ; sar eax, 34
            ) [CF ZF SF],
            sar_229_two_wrap: (
                ; mov eax, 229
                ; sar eax, 34
            ) [CF ZF SF],
            sar_neg_228_two_wrap: (
                ; mov eax, -228
                ; sar eax, 34
            ) [CF ZF SF],
            sar_neg_229_two_wrap: (
                ; mov eax, -229
                ; sar eax, 34
            ) [CF ZF SF],
        }
    }

    mod div {
        test_snippets!(
            div_basic1: (
                ; mov eax, 42
                ; mov ebx, 24
                ; div ebx
            ),
            div_basic2: (
                ; mov eax, 1
                ; mov ebx, 888
                ; div ebx
            ),
            div_basic3: (
                ; mov eax, 888
                ; mov ebx, 1
                ; div ebx
            ),
            div_basic4: (
                ; mov eax, 1
                ; mov ebx, 2
                ; div ebx
            ),
            div_rnd1: (
                ; mov eax, -0x57549d35
                ; mov ebx, 0x4003cb02
                ; div ebx
            ),
            div_rnd2: (
                ; mov eax, 0x37ab7947
                ; mov ebx, -0x6d61d34
                ; div ebx
            ),
            div_rnd3: (
                ; mov eax, 0x3a64b162
                ; mov ebx, -0x502df7b4
                ; div ebx
            ),
            div_big1: (
                ; mov eax, 0
                ; mov edx, 1
                ; mov ebx, 2
                ; div ebx
            ),
            // this should cause a division error
            // TODO: how can we test this? (it's not how it behaves rn btw)
            // ditto for division by zero
            // div_big2: (
            //     ; mov eax, 0
            //     ; mov edx, 1
            //     ; mov ebx, 1
            //     ; div ebx
            // ),
            div_big_rnd1: (
                ; mov eax, -0x1895c25a
                ; mov edx, 0x6c8300d6
                ; mov ebx, 0x70a45624
                ; div ebx
            ),
            div_big_rnd2: (
                ; mov eax, -0x21c0f
                ; mov edx, 0x338001
                ; mov ebx, 0x90ed24d
                ; div ebx
            ),
            div_big_rnd3: (
                ; mov eax, 0x74f1d28c
                ; mov edx, 0x7507473a
                ; mov ebx, -0x7d79c77f
                ; div ebx
            ),
        );
    }

    mod stack {
        test_snippets!(
            push_eax_pop_ebx: (
                ; mov eax, 42
                ; push eax
                ; pop ebx
            ),
            push_eax_ebx: (
                ; mov eax, 42
                ; push eax
                ; push ebx
            ),
        );
    }
}

mod funs {
    mod fib {
        test_functions! {
            fib_recur: [
                (0), (1), (2), (3), (4),
                (5), (6), (7), (8), (9),
                (10), (20)
            ] (
                ; ->fibonacci:                              //  @fibonacci
                ;         push    edi
                ;         push    esi
                ;         push    eax
                ;         mov     edi, [esp + 16]
                ;         xor     esi, esi
                ;         cmp     edi, 2
                ;         jb      ->LBB0_3
                ;         xor     esi, esi
                ; ->LBB0_2:                                // =>This Inner Loop Header: Depth=1
                ;         lea     eax, [edi - 1]
                ;         mov     [esp], eax
                ;         call    ->fibonacci
                ;         add     edi, -2
                ;         add     esi, eax
                ;         cmp     edi, 1
                ;         ja      ->LBB0_2
                ; ->LBB0_3:
                ;         add     esi, edi
                ;         mov     eax, esi
                ;         add     esp, 4
                ;         pop     esi
                ;         pop     edi
                ;         ret
            ),
            fib_smartass: [
                (0), (1), (2), (3), (4),
                (5), (6), (7), (8), (9),
                (10), (20), (100), (1000), (10000)
            ] (
                ; ->fib:
                ;         sub     esp, 28
                ;         mov     eax, [esp+32]
                ;         mov     DWORD [esp], 1
                ;         mov     DWORD [esp+4], 1
                ;         mov     DWORD [esp+8], 1
                ;         mov     DWORD [esp+12], 0
                ;         test    eax, eax
                ;         je      ->L13
                ;         sub     esp, 8
                ;         sub     eax, 1
                ;         push    eax
                ;         lea     eax, [esp+12]
                ;         push    eax
                ;         call    ->power
                ;         mov     eax, [esp+16]
                ;         add     esp, 16
                ; ->L13:
                ;         add     esp, 28
                ;         ret

                ; ->multiply:
                ;         push    ebp
                ;         push    edi
                ;         push    esi
                ;         push    ebx
                ;         sub     esp, 8
                ;         mov     edi, [esp+32]
                ;         mov     eax, [esp+28]
                ;         mov     edx, [edi+4]
                ;         mov     ebp, [edi+12]
                ;         mov     ecx, [edi]
                ;         mov     esi, [edi+8]
                ;         mov     ebx, [eax]
                ;         mov     edi, [eax+8]
                ;         mov     [esp], edx
                ;         mov     edx, [eax+4]
                ;         mov     [esp+4], edi
                ;         mov     edi, ebx
                ;         imul    ebx, [esp]
                ;         imul    edx, esi
                ;         imul    edi, ecx
                ;         imul    esi, [eax+12]
                ;         add     edi, edx
                ;         mov     edx, [eax+4]
                ;         mov     [eax], edi
                ;         imul    edx, ebp
                ;         add     ebx, edx
                ;         mov     edx, [esp]
                ;         mov     [eax+4], ebx
                ;         mov     ebx, [esp+4]
                ;         imul    ecx, ebx
                ;         imul    edx, ebx
                ;         add     ecx, esi
                ;         mov     [eax+8], ecx
                ;         mov     ecx, [eax+12]
                ;         imul    ecx, ebp
                ;         add     edx, ecx
                ;         mov     [eax+12], edx
                ;         add     esp, 8
                ;         pop     ebx
                ;         pop     esi
                ;         pop     edi
                ;         pop     ebp
                ;         ret

                ; ->power:
                ;         push    esi
                ;         push    ebx
                ;         sub     esp, 4
                ;         mov     esi, [esp+20]
                ;         mov     ebx, [esp+16]
                ;         cmp     esi, 1
                ;         jbe     ->L4
                ;         mov     eax, esi
                ;         sub     esp, 8
                ;         shr     eax, 31
                ;         add     eax, esi
                ;         sar     eax, 1
                ;         push    eax
                ;         push    ebx
                ;         call    ->power
                ;         pop     eax
                ;         pop     edx
                ;         push    ebx
                ;         push    ebx
                ;         call    ->multiply
                ;         add     esp, 16
                ;         and     esi, 1
                ;         jne     ->L11
                ; ->L4:
                ;         add     esp, 4
                ;         pop     ebx
                ;         pop     esi
                ;         ret
                ; ->L11:
                ;         mov     edx, [ebx]
                ;         mov     eax, [ebx+8]
                ;         mov     ecx, [ebx+12]
                ;         mov     esi, [ebx+4]
                ;         mov     [ebx+12], eax
                ;         add     esi, edx
                ;         add     ecx, eax
                ;         mov     [ebx+4], edx
                ;         mov     [ebx], esi
                ;         mov     [ebx+8], ecx
                ;         add     esp, 4
                ;         pop     ebx
                ;         pop     esi
                ;         ret
            ),
        }
    }

    mod sort {
        // I use varargs to pass arrays
        // first arg tells the size, the remaining ones are the numbers to sort
        test_functions! {
            bsort: [
                (0),
                (1, 1),
                (1, 0xffffffff),
                (2, 1, 2),
                (2, 2, 1),
                (2, 1, 1),
                (2, 1, 0xffffffff),
                (3, 1, 2, 3),
                (3, 3, 2, 1),
                (3, 1, 3, 2),
                (3, 3, 0xffffffff, 1),
                (10, 9, 10, 1, 6, 8, 3, 2, 5, 4, 7)
            ] (
                ; ->bsort:
                ;         push    ebp
                ;         mov     ebp, esp
                ;         push    edi
                ;         push    esi
                ;         push    ebx
                ;         sub     esp, 12
                ;         mov     edi, DWORD [ebp+8]
                ;         lea     ebx, [0+edi*4]
                ;         lea     eax, [ebx+27]
                ;         and     eax, -16
                ;         sub     esp, eax
                ;         lea     esi, [esp+15]
                ;         and     esi, -16
                ;         test    edi, edi
                ;         je      ->L17
                ;         lea     eax, [ebp+12]
                ;         mov     edx, esi
                ;         add     ebx, eax
                ; ->L19:
                ;         mov     ecx, DWORD [eax]
                ;         add     eax, 4
                ;         add     edx, 4
                ;         mov     DWORD [edx-4], ecx
                ;         cmp     eax, ebx
                ;         jne     ->L19
                ;         sub     esp, 8
                ;         push    edi
                ;         push    esi
                ;         call    ->bsort_impl_part_0
                ;         add     esp, 16
                ; ->L17:
                ;         lea     esp, [ebp-12]
                ;         pop     ebx
                ;         pop     esi
                ;         pop     edi
                ;         pop     ebp
                ;         ret

                ; ->bsort_impl_part_0:
                ;         push    edi
                ;         push    esi
                ;         push    ebx
                ;         mov     esi, DWORD [esp+20]
                ;         mov     edi, DWORD [esp+16]
                ;         sub     esi, 1
                ;         je      ->L1
                ;         lea     ebx, [edi+esi*4]
                ; ->L3:
                ;         mov     eax, edi
                ; ->L5:
                ;         mov     edx, DWORD [eax]
                ;         mov     ecx, DWORD [eax+4]
                ;         cmp     edx, ecx
                ;         jle     ->L4
                ;         mov     DWORD [eax], ecx
                ;         mov     DWORD [eax+4], edx
                ; ->L4:
                ;         add     eax, 4
                ;         cmp     eax, ebx
                ;         jne     ->L5
                ;         sub     ebx, 4
                ;         sub     esi, 1
                ;         jne     ->L3
                ; ->L1:
                ;         pop     ebx
                ;         pop     esi
                ;         pop     edi
                ;         ret
            ),
        }
    }

    test_functions! {
        // test name
        test: [
            // arguments to test on (all are u32)
            (1, 2),
            (2, 3),
            (3, 4)
        ] (
            // test body
            ; mov eax, 42
            ; ret
        ),
        min: [
            // negative values can't be put here because it can't be part of a valid identifier
            // TODO: maybe we can patch the crate that does it/find another/???
            (0, 0),
            (0xffffffff, 0xfffffffe),
            (100, 200),
            (0xffffff9c, 0xffffff38),
            (0xffffffff, 0xffffffff),
            (0xffffffff, 0x80000000)
        ] (
            ; ->min:
            ;   mov     eax, DWORD [esp+4]
            ;   mov     edx, DWORD [esp+8]
            ;   cmp     eax, edx
            ;   cmovg   eax, edx
            ;   ret
        ),
        max: [
            (0, 0),
            (0xffffffff, 0xfffffffe),
            (100, 200),
            (0xffffff9c, 0xffffff38),
            (0xffffffff, 0xffffffff),
            (0xffffffff, 0x80000000)
        ] (
            ; ->max:
            ; mov     eax, DWORD [esp+4]
            ; mov     edx, DWORD [esp+8]
            ; cmp     eax, edx
            ; cmovl   eax, edx
            ; ret
        ),
    }
}
