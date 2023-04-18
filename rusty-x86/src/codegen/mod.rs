mod prelude {
    pub use crate::backend::{BoolValue, Builder, ComparisonType, IntValue};
    pub use crate::disasm::Operands;
    pub use crate::types::{ControlFlow, Flag, Flag::*, IntType, Operand, Register, Register::*};
    pub use iced_x86::{Code, ConditionCode, Instruction, Mnemonic};

    pub type CF<B> = ControlFlow<<B as Builder>::IntValue, <B as Builder>::BoolValue>;
}

mod arithmetic;
mod bit_magic;
mod bitwise;
mod conditional;
mod control_flow;
mod convert;
mod flags;
mod mov;
mod shifts;
mod stack;
mod string;
mod trapping;

use log::warn;
use prelude::*;

#[inline]
fn call_handler<B: Builder, O: crate::disasm::FromInstruction, F: Fn(&mut B, O) -> CF<B>>(
    handler: F,
    builder: &mut B,
    instr: &Instruction,
) -> CF<B> {
    handler(builder, O::from_instruction(instr))
}

pub fn codegen_instr<B: Builder>(
    builder: &mut B,
    instr: Instruction,
) -> ControlFlow<B::IntValue, B::BoolValue> {
    if instr.has_lock_prefix() {
        warn!(
            "Instruction with LOCK prefix: {:?} (translating {} @ {:#010x})",
            instr,
            instr,
            instr.next_ip32() as usize - instr.len()
        );
    }
    assert!(!instr.has_xacquire_prefix());
    assert!(!instr.has_xrelease_prefix());

    if instr.is_string_instruction() {
        string::string_instr(builder, instr);
        return ControlFlow::NextInstruction;
    }

    assert!(!instr.has_rep_prefix());
    assert!(!instr.has_repe_prefix());
    assert!(!instr.has_repne_prefix());

    let mnemonic = instr.mnemonic();

    if instr.is_jcc_short_or_near() {
        call_handler(conditional::jcc, builder, &instr)
    } else if conditional::is_cmovcc(instr.mnemonic()) {
        call_handler(conditional::movcc, builder, &instr)
    } else if conditional::is_setcc(instr.mnemonic()) {
        call_handler(conditional::setcc, builder, &instr)
    } else {
        use iced_x86::Mnemonic::*;
        match mnemonic {
            // TODO: there is (going to be) a ton of opcodes, we would want to handle this nicely (a bit of macromagic?)
            Mov => call_handler(mov::mov, builder, &instr),
            Movzx => call_handler(mov::movzx, builder, &instr),
            Movsx => call_handler(mov::movsx, builder, &instr),
            Xchg => call_handler(mov::xchg, builder, &instr),

            Add => call_handler(arithmetic::add, builder, &instr),
            Adc => call_handler(arithmetic::adc, builder, &instr),
            Sub | Cmp => call_handler(arithmetic::sub_cmp, builder, &instr),
            Sbb => call_handler(arithmetic::sbb, builder, &instr),
            Lea => call_handler(arithmetic::lea, builder, &instr),
            Dec => call_handler(arithmetic::dec, builder, &instr),
            Inc => call_handler(arithmetic::inc, builder, &instr),
            Neg => call_handler(arithmetic::neg, builder, &instr),
            Mul => call_handler(arithmetic::mul, builder, &instr),
            Imul => call_handler(arithmetic::imul, builder, &instr),
            Div | Idiv => call_handler(arithmetic::div_idiv, builder, &instr),

            Cwd | Cdq => call_handler(convert::cwd_cdq, builder, &instr),
            Cbw | Cwde => call_handler(convert::cbw_cwde, builder, &instr),

            Xor => call_handler(bitwise::xor, builder, &instr),
            Not => call_handler(bitwise::not, builder, &instr),
            And | Test => call_handler(bitwise::and_test, builder, &instr),
            Or => call_handler(bitwise::or, builder, &instr),

            Shr | Sar | Shl => call_handler(shifts::shr_sar_shl, builder, &instr),
            Rcr => call_handler(shifts::rcr, builder, &instr),
            Shld | Shrd => call_handler(shifts::shld_shrd, builder, &instr),

            Push => call_handler(stack::push, builder, &instr),
            Pop => call_handler(stack::pop, builder, &instr),
            Leave => call_handler(stack::leave, builder, &instr),

            Ret => call_handler(control_flow::ret, builder, &instr),
            Jmp => call_handler(control_flow::jmp, builder, &instr),
            Call => call_handler(control_flow::call, builder, &instr),

            // TODO: implement more bit counting
            Bsr => call_handler(bit_magic::bsr, builder, &instr),
            Bt | Btc | Bts | Btr => call_handler(bit_magic::bt_btc_bts_btr, builder, &instr),

            Pushfd => call_handler(flags::pushfd, builder, &instr),
            Popfd => call_handler(flags::popfd, builder, &instr),
            Stc => call_handler(flags::stc, builder, &instr),
            Clc => call_handler(flags::clc, builder, &instr),
            Std => call_handler(flags::std, builder, &instr),
            Cld => call_handler(flags::cld, builder, &instr),

            Int | Int3 => call_handler(trapping::int_int3, builder, &instr),
            Ud2 => call_handler(trapping::ud2, builder, &instr),

            Nop | Endbr32 => ControlFlow::NextInstruction,

            m => {
                warn!(
                    "Unknown instruction mnemonic: {:?} (translating {} @ {:#010x})",
                    m,
                    instr,
                    instr.next_ip32() as usize - instr.len()
                );

                builder.trap();

                ControlFlow::NextInstruction
            }
        }
    }
}
