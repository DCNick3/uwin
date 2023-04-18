use super::prelude::*;

#[allow(clippy::let_and_return)]
pub fn compute_condition_code<B: Builder>(
    builder: &mut B,
    condition_code: ConditionCode,
) -> B::BoolValue {
    let mut recur = |cc| compute_condition_code(builder, cc);

    use ConditionCode::*;
    match condition_code {
        None => panic!("Can't compute None condition"),

        o => {
            let of = builder.load_flag(Flag::Overflow);
            of
        }
        no => {
            let of = builder.load_flag(Flag::Overflow);
            builder.bool_not(of)
        }

        b => {
            let cf = builder.load_flag(Flag::Carry);
            cf
        }
        ae => {
            let cf = builder.load_flag(Flag::Carry);
            builder.bool_not(cf)
        }

        e => {
            let zf = builder.load_flag(Flag::Zero);
            zf
        }
        ne => {
            let zf = builder.load_flag(Flag::Zero);
            builder.bool_not(zf)
        }

        be => {
            let cf = builder.load_flag(Flag::Carry);
            let zf = builder.load_flag(Flag::Zero);
            let r = builder.bool_or(cf, zf);
            r
        }
        a => {
            let r = recur(be);
            let r = builder.bool_not(r);
            r
        }

        s => {
            let sf = builder.load_flag(Flag::Sign);
            sf
        }
        ns => {
            let sf = builder.load_flag(Flag::Sign);
            builder.bool_not(sf)
        }

        p | np => unimplemented!("condition code {:?}", condition_code),

        l => {
            let sf = builder.load_flag(Flag::Sign);
            let of = builder.load_flag(Flag::Overflow);
            builder.bool_xor(sf, of)
        }
        ge => {
            let r = recur(l);
            builder.bool_not(r)
        }

        le => {
            let sf = builder.load_flag(Flag::Sign);
            let of = builder.load_flag(Flag::Overflow);
            let zf = builder.load_flag(Flag::Zero);
            let is_l = builder.bool_xor(sf, of);
            let r = builder.bool_or(is_l, zf);
            r
        }
        g => {
            let r = recur(le);
            builder.bool_not(r)
        }
    }
}

#[rustfmt::skip]
pub fn is_cmovcc(mnemonic: Mnemonic) -> bool {
    use Mnemonic::*;
    match mnemonic {
        Cmova |
        Cmovae |
        Cmovb |
        Cmovbe |
        Cmove |
        Cmovg |
        Cmovge |
        Cmovl |
        Cmovle |
        Cmovne |
        Cmovno |
        Cmovnp |
        Cmovns |
        Cmovo |
        Cmovp |
        Cmovs  => {
            true
        },
        _ => false,
    }
}

#[rustfmt::skip]
pub fn is_setcc(mnemonic: Mnemonic) -> bool {
    use Mnemonic::*;
    match mnemonic {
        Seta |
        Setae |
        Setb |
        Setbe |
        Sete |
        Setg |
        Setge |
        Setl |
        Setle |
        Setne |
        Setno |
        Setnp |
        Setns |
        Seto |
        Setp |
        Sets => {
            true
        },
        _ => false,
    }
}

pub fn jcc<B: Builder>(builder: &mut B, (code, target): (ConditionCode, Operand)) -> CF<B> {
    let cond = compute_condition_code(builder, code);

    ControlFlow::Conditional(cond, target.as_imm32())
}

pub fn movcc<B: Builder>(
    builder: &mut B,
    (code, dst, src): (ConditionCode, Operand, Operand),
) -> CF<B> {
    let cond = compute_condition_code(builder, code);

    builder.ifelse(
        cond,
        |builder| {
            // move!
            let val = builder.load_operand(src);
            builder.store_operand(dst, val);
        },
        |_builder| {}, // nuff to do,
    );

    ControlFlow::NextInstruction
}

pub fn setcc<B: Builder>(builder: &mut B, (code, dst): (ConditionCode, Operand)) -> CF<B> {
    let cond = compute_condition_code(builder, code);

    let res = builder.bool_to_int(cond, IntType::I8);

    builder.store_operand(dst, res);

    ControlFlow::NextInstruction
}
