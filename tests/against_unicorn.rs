use std::collections::BTreeMap;
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;
use inkwell::values::BasicMetadataValueEnum;
use log::{info, debug};
use unicorn;
use unicorn::{Cpu, CpuX86, Protection, RegisterX86};
use rusty_x86::types::{CpuContext, FullSizeGeneralPurposeRegister};
use rusty_x86::assemble_x86;
use rusty_x86::llvm::backend::{BbFunc, FASTCC_CALLING_CONVENTION};
use test_log::test;
use strum::IntoEnumIterator;

const BASE_ADDR: u32 = 0x1000;

fn execute_unicorn(code: &[u8]) -> CpuContext {
    let pad = (0x1000 - (code.len() % 0x1000)) % 0x1000;
    let code_page_size = code.len() + pad;

    let emu = CpuX86::new(unicorn::Mode::MODE_32).unwrap();

    let base_addr = BASE_ADDR as u64;

    // map the code
    emu.mem_map(base_addr, code_page_size, Protection::READ | Protection::EXEC).unwrap();
    emu.mem_write(base_addr, code).unwrap();

    // TODO: can we stop on return? Maybe hooks?
    // TODO: setup stack?
    emu.emu_start(base_addr, base_addr + code.len() as u64, 10 * unicorn::SECOND_SCALE, 0).unwrap();

    let mut res = CpuContext::default();

    res.set_gp_reg(FullSizeGeneralPurposeRegister::EAX, emu.reg_read(RegisterX86::EAX).unwrap() as u32);
    res.set_gp_reg(FullSizeGeneralPurposeRegister::EBX, emu.reg_read(RegisterX86::EBX).unwrap() as u32);
    res.set_gp_reg(FullSizeGeneralPurposeRegister::ECX, emu.reg_read(RegisterX86::ECX).unwrap() as u32);
    res.set_gp_reg(FullSizeGeneralPurposeRegister::EDX, emu.reg_read(RegisterX86::EDX).unwrap() as u32);
    res.set_gp_reg(FullSizeGeneralPurposeRegister::ESP, emu.reg_read(RegisterX86::ESP).unwrap() as u32);
    res.set_gp_reg(FullSizeGeneralPurposeRegister::EBP, emu.reg_read(RegisterX86::EBP).unwrap() as u32);
    res.set_gp_reg(FullSizeGeneralPurposeRegister::ESI, emu.reg_read(RegisterX86::ESI).unwrap() as u32);
    res.set_gp_reg(FullSizeGeneralPurposeRegister::EDI, emu.reg_read(RegisterX86::EDI).unwrap() as u32);

    // TODO: read & set flags

    res
}

fn execute_rusty_x86(code: &[u8]) -> CpuContext {
    let context = inkwell::context::Context::create();
    let module = rusty_x86::llvm::recompile(&context, BASE_ADDR, code);

    let types = rusty_x86::llvm::backend::Types::new(&context);

    let entry_name = rusty_x86::llvm::backend::LlvmBuilder::get_name_for(BASE_ADDR);

    const ENTRY_NAME: &str = "entry";

    let entry = module.add_function(ENTRY_NAME, types.bb_fn, None);
    let bb = context.append_basic_block(entry, ENTRY_NAME);

    {
        let builder = context.create_builder();
        builder.position_at_end(bb);

        let args: Vec<BasicMetadataValueEnum> = entry.get_params()
            .iter()
            .map(|f| f.clone().into())
            .collect();

        let call = builder.build_call(module.get_function(entry_name.as_str()).unwrap(),
                                      args.as_slice(), "res");
        call.set_call_convention(FASTCC_CALLING_CONVENTION);

        builder.build_return(None);
    }



    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None /* TODO: do we want optimizations? */)
        .unwrap();

    let fun: JitFunction<BbFunc> = unsafe { execution_engine.get_function(ENTRY_NAME).unwrap() };

    let mut cpu_context = CpuContext::default();

    // TODO: this is a simplification
    let mut mem = [0u8; 0x1000];

    unsafe {
        fun.call(&mut cpu_context, mem.as_mut_ptr());
    };

    cpu_context
}

fn context_to_map(context: CpuContext) -> BTreeMap<FullSizeGeneralPurposeRegister, u32> {
    FullSizeGeneralPurposeRegister::iter()
        .map(|reg| (reg, context.get_gp_reg(reg)))
        .collect()
}

fn test_code(code: &[u8]) {
    let unicorn = execute_unicorn(code);
    let rusty_x86 = execute_rusty_x86(code);

    debug!("unicorn   = {}", unicorn);
    debug!("rusty_x86 = {}", rusty_x86);

    //rusty_x86.set_gp_reg(FullSizeGeneralPurposeRegister::EBX, 32);

    // convert it to BTreeMap for better debugging view
    // or maybe we would be better off implementing Debug?..
    let unicorn = context_to_map(unicorn);
    let rusty_x86 = context_to_map(rusty_x86);

    assert_eq!(unicorn, rusty_x86);

    // TODO: flags? (they are sometimes undefined...)
}

macro_rules! test_cases {
    ($($name:ident: ($($value:tt)*),)*) => {
    $(
        #[test]
        fn $name() {
            info!("Running {}", stringify!($name));
            let code = assemble_x86!(
                $($value)*
            );
            test_code(code.as_slice());
        }
    )*
    }
}

test_cases! {
    mov_eax_42: (
        ; mov eax, 42
    ),
    sub_borrow: (
        ; mov eax, 1
        ; sub eax, 2
    ),
    sub_branch_sign: (
        ; mov eax, 1
        ; sub eax, 2
        ; js ->L1 // TODO: cmov is more concise?
        ; mov ebx, 1
        ; jmp ->R
        ; ->L1:
        ; mov ebx, 2
        ; ->R:
        ; nop // necessary because of funky control flow at the end of test snippets...
    ),
    sub_cmov_sign: (
        ; mov eax, 1
        ; sub eax, 2
        ; mov ecx, 2
        ; cmovs ebx, ecx
    ),
    sub_cmov_sign_2: (
        ; mov eax, 3
        ; sub eax, 2
        ; mov ecx, 2
        ; cmovs ebx, ecx
    ),
    cmp_cmov_eq: (
        ; mov eax, 12
        ; cmp eax, 12
        ; mov ecx, 2
        ; cmovz ebx, ecx
    ),
    cmp_cmov_eq_2: (
        ; mov eax, 12
        ; cmp eax, 13
        ; mov ecx, 2
        ; cmovz ebx, ecx
    ),
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
