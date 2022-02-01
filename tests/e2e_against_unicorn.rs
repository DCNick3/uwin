use inkwell::execution_engine::JitFunction;
use inkwell::values::BasicMetadataValueEnum;
use inkwell::OptimizationLevel;
use log::debug;
use rusty_x86::llvm::backend::{BbFunc, FASTCC_CALLING_CONVENTION};
use rusty_x86::types::{CpuContext, Flag, FullSizeGeneralPurposeRegister};
use std::collections::BTreeMap;
use strum::IntoEnumIterator;
use unicorn;
use unicorn::{Cpu, CpuX86, Protection, RegisterX86};

const CODE_ADDR: u32 = 0x200000;
const MEM_ADDR: u32 = 0x100000;
const MEM_SIZE: u32 = 0x10000;

fn execute_unicorn(code: &[u8]) -> (CpuContext, Vec<u8>) {
    let pad = (0x1000 - (code.len() % 0x1000)) % 0x1000;
    let code_page_size = code.len() + pad;

    let emu = CpuX86::new(unicorn::Mode::MODE_32).unwrap();

    let base_addr = CODE_ADDR as u64;

    // map the code
    emu.mem_map(
        base_addr,
        code_page_size,
        Protection::READ | Protection::EXEC,
    )
    .unwrap();
    emu.mem_write(base_addr, code).unwrap();

    emu.mem_map(
        MEM_ADDR as u64,
        MEM_SIZE as usize,
        Protection::READ | Protection::WRITE,
    )
    .unwrap();

    emu.reg_write(RegisterX86::ESP, (MEM_ADDR + MEM_SIZE - 4) as u64)
        .unwrap();

    // TODO: can we stop on return? Maybe hooks?
    emu.emu_start(
        base_addr,
        base_addr + code.len() as u64,
        10 * unicorn::SECOND_SCALE,
        0,
    )
    .unwrap();

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

fn execute_rusty_x86(code: &[u8]) -> (CpuContext, Vec<u8>) {
    let context = inkwell::context::Context::create();
    let types = &rusty_x86::llvm::backend::Types::new(&context);
    let module = rusty_x86::llvm::recompile(&context, types, CODE_ADDR, code);

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

    // TODO: this is a simplification
    //let mut mem = [0u8; MEM_SIZE as usize];

    cpu_context.set_gp_reg(FullSizeGeneralPurposeRegister::ESP, MEM_ADDR + MEM_SIZE - 4);

    // SAFETY: dragons ahead
    // map 4 GiB of memory with no protection
    // this way we can control all mappings in the whole virtualized 32-bit address space
    let mut target_mem_region = region::alloc(0x100000000, region::Protection::NONE).unwrap();

    // map a small region of memory
    let mapped_start = unsafe { target_mem_region.as_ptr::<u8>().add(MEM_ADDR as usize) };

    let mut mapped = region::alloc_at(
        mapped_start,
        MEM_SIZE as usize,
        region::Protection::READ_WRITE,
    )
    .unwrap();

    // fill the mapped memory with zeroes (is it necessary?)
    unsafe { std::slice::from_raw_parts_mut(mapped.as_mut_ptr(), mapped.len()).fill(0u8) }

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

fn test_code(code: &[u8], flags: Vec<Flag>) {
    debug!(
        "CODE:\n{}",
        rusty_x86::disasm::disassemble(code, CODE_ADDR as u64)
    );

    let rusty_x86 = execute_rusty_x86(code);
    let unicorn = execute_unicorn(code);

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

macro_rules! test_case {
    ($name:ident: ($($value:tt)*) [$($flags:ident)*]) => {
        #[test_log::test]
        fn $name() {
            log::info!("Running {}", stringify!($name));
            let code = rusty_x86::assemble_x86!(
                $($value)*
            );
            crate::test_code(code.as_slice(), vec![$(parse_flag!($flags)),*]);
        }
    };
}

macro_rules! test_cases {
    () => {};
    ($name:ident: ($($value:tt)*) [$($flags:ident)*], $($xs:tt)*) => {
        test_case!($name: ($($value)*) [$($flags)*]);
        test_cases!($($xs)*);
    };
    ($name:ident: ($($value:tt)*), $($xs:tt)*) => {
        test_case!($name: ($($value)*) []);
        test_cases!($($xs)*);
    };
}

mod mov {
    test_cases! {
        mov_eax_42: (
            ; mov eax, 42
        ),
        mov_ebx_42: (
            ; mov ebx, 42
        ),
    }
}

mod sub {
    test_cases! {
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

mod cmp {
    test_cases! {
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
    test_cases! {
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
    test_cases! {
        mem_basic_rw: (
            ; mov eax, 42
            ; mov eax, [crate::MEM_ADDR as i32]
            ; mov [crate::MEM_ADDR as i32], ebx
        ),
    }
}

mod imul {
    test_cases! {
        imul_1op_eax_eax: (
            ; mov eax, 23
            ; imul eax
        ) [CF OF],
        imul_1op: (
            ; mov eax, 23
            ; mov ebx, 24
            ; imul ebx
        ) [CF OF],
        imul_1op_overflow: (
            ; mov eax, 0x7fffffff
            ; mov ebx, 0x7fffffff
            ; imul ebx
        ) [CF OF],

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

        imul_3op_eax_eax: (
            ; mov eax, 23
            ; imul eax, eax, 24
        ) [CF OF],
        imul_3op: (
            ; mov ebx, 24
            ; imul eax, ebx, 23
        ) [CF OF],
        imul_3op_overflow: (
            ; mov ebx, 0x7fffffff
            ; imul eax, ebx, 0x7fffffff
        ) [CF OF],
    }
}

mod xor {
    test_cases! {
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

mod div {
    test_cases!(
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
    test_cases!(
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
