use iced_x86::{Formatter, Instruction, NasmFormatter};
use inkwell::execution_engine::JitFunction;
use inkwell::values::BasicMetadataValueEnum;
use inkwell::OptimizationLevel;
use log::debug;
use rusty_x86::llvm::backend::{BbFunc, FASTCC_CALLING_CONVENTION};
use rusty_x86::types::{CpuContext, Flag, FullSizeGeneralPurposeRegister};
use std::collections::BTreeMap;
use std::fmt::Write;
use strum::IntoEnumIterator;
use unicorn;
use unicorn::{Cpu, CpuX86, Protection, RegisterX86};

const CODE_ADDR: u32 = 0x200000;
const MEM_ADDR: u32 = 0x100000;
const MEM_SIZE: u32 = 0x10000;

fn execute_unicorn(code: &[u8]) -> CpuContext {
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

    // TODO: can we stop on return? Maybe hooks?
    // TODO: setup stack?
    emu.emu_start(
        base_addr,
        base_addr + code.len() as u64,
        10 * unicorn::SECOND_SCALE,
        0,
    )
    .unwrap();

    let mut res = CpuContext::default();

    res.set_gp_reg(
        FullSizeGeneralPurposeRegister::EAX,
        emu.reg_read(RegisterX86::EAX).unwrap() as u32,
    );
    res.set_gp_reg(
        FullSizeGeneralPurposeRegister::EBX,
        emu.reg_read(RegisterX86::EBX).unwrap() as u32,
    );
    res.set_gp_reg(
        FullSizeGeneralPurposeRegister::ECX,
        emu.reg_read(RegisterX86::ECX).unwrap() as u32,
    );
    res.set_gp_reg(
        FullSizeGeneralPurposeRegister::EDX,
        emu.reg_read(RegisterX86::EDX).unwrap() as u32,
    );
    res.set_gp_reg(
        FullSizeGeneralPurposeRegister::ESP,
        emu.reg_read(RegisterX86::ESP).unwrap() as u32,
    );
    res.set_gp_reg(
        FullSizeGeneralPurposeRegister::EBP,
        emu.reg_read(RegisterX86::EBP).unwrap() as u32,
    );
    res.set_gp_reg(
        FullSizeGeneralPurposeRegister::ESI,
        emu.reg_read(RegisterX86::ESI).unwrap() as u32,
    );
    res.set_gp_reg(
        FullSizeGeneralPurposeRegister::EDI,
        emu.reg_read(RegisterX86::EDI).unwrap() as u32,
    );

    let flags = emu.reg_read(RegisterX86::EFLAGS).unwrap() as u32;

    res.set_flag(Flag::Carry, flags & 0x1 != 0);
    res.set_flag(Flag::Parity, flags & 0x2 != 0);
    res.set_flag(Flag::AuxiliaryCarry, flags & 0x10 != 0);
    res.set_flag(Flag::Zero, flags & 0x40 != 0);
    res.set_flag(Flag::Sign, flags & 0x80 != 0);
    res.set_flag(Flag::Overflow, flags & 0x800 != 0);

    res
}

fn execute_rusty_x86(code: &[u8]) -> CpuContext {
    let context = inkwell::context::Context::create();
    let module = rusty_x86::llvm::recompile(&context, CODE_ADDR, code);

    let types = rusty_x86::llvm::backend::Types::new(&context);

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

    let execution_engine = module
        .create_jit_execution_engine(
            OptimizationLevel::None, /* TODO: do we want optimizations? */
        )
        .unwrap();

    let fun: JitFunction<BbFunc> = unsafe { execution_engine.get_function(ENTRY_NAME).unwrap() };

    let mut cpu_context = CpuContext::default();

    // TODO: this is a simplification
    let mut mem = [0u8; MEM_SIZE as usize];

    // !!! we can't really ensure that memory accesses are safe w/o implementing softmmu
    // implementing it helps with portability & safety, but increases complexity & decreases performance
    unsafe {
        fun.call(
            &mut cpu_context,
            mem.as_mut_ptr().offset(-(MEM_ADDR as isize)),
        );
    };

    cpu_context
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

fn disassemble(code: &[u8]) -> String {
    let mut decoder = iced_x86::Decoder::with_ip(32, code, CODE_ADDR as u64, 0);
    let mut formatter = NasmFormatter::new();
    let mut output = String::new();
    let mut instruction = Instruction::default();
    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);
        write!(output, "{:08X} ", instruction.ip()).unwrap();

        formatter.format(&instruction, &mut output);

        writeln!(output).unwrap();
    }
    output
}

fn test_code(code: &[u8], flags: Vec<Flag>) {
    debug!("CODE:\n{}", disassemble(code));

    let rusty_x86 = execute_rusty_x86(code);
    let unicorn = execute_unicorn(code);

    debug!("RESULT rusty_x86 = {:?}", rusty_x86);
    debug!("RESULT unicorn   = {:?}", unicorn);

    // We can't directly compare contexts because of flags (sometimes they are undefined on x86)
    // So we compare separately the values of registers and specified flags
    let rusty_x86_gp = context_to_gp_map(&rusty_x86);
    let unicorn_gp = context_to_gp_map(&unicorn);

    assert_eq!(rusty_x86_gp, unicorn_gp);

    let rusty_x86_flags = context_to_flag_list(&rusty_x86, flags.as_slice());
    let unicorn_flags = context_to_flag_list(&unicorn, flags.as_slice());

    debug!("FLAGS (filtered) unicorn   = {:?}", unicorn_flags);
    debug!("FLAGS (filtered) rusty_x86 = {:?}", rusty_x86_flags);

    assert_eq!(rusty_x86_flags, unicorn_flags);
    // TODO: flags? (they are sometimes undefined...)
}

#[allow(unused_macros)]
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
