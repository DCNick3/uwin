use super::prelude::*;
use crate::{operands, operands_ty};

pub fn string_instr<B: Builder>(builder: &mut B, instr: Instruction) -> CF<B> {
    let advance_reg = |builder: &mut B, size: IntType, reg: Register| {
        let size = builder.make_u32(size.byte_width() as u32);
        let edi = builder.load_register(reg);

        // if DF = 1 => EDI += size, else => EDI -= size
        let df = builder.load_flag(Flag::Direction);
        builder.ifelse(
            df,
            |builder| {
                let edi = builder.sub(edi, size);
                builder.store_register(reg, edi);
            },
            |builder| {
                let edi = builder.add(edi, size);
                builder.store_register(reg, edi);
            },
        );
    };

    let advance_edi = |builder: &mut B, size: IntType| advance_reg(builder, size, Register::EDI);
    let advance_esi = |builder: &mut B, size: IntType| advance_reg(builder, size, Register::ESI);

    let execute_instr = |builder: &mut B| {
        use Mnemonic::*;
        // this handles the core instruction
        match instr.mnemonic() {
            // no port IO for you
            Insb | Insw | Insd | Outsb | Outsw | Outsd => unimplemented!(),

            Lodsb | Lodsw | Lodsd => {
                todo!("{:?}", instr.mnemonic())
            }

            Movsb | Movsw | Movsd => {
                operands!([dst, src], &instr);

                let val = builder.load_operand(src);
                builder.store_operand(dst, val);

                advance_esi(builder, dst.size());
                advance_edi(builder, dst.size());
            }

            Stosb | Stosw | Stosd => {
                operands!([dst, val], &instr);

                let val = builder.load_operand(val);
                builder.store_operand(dst, val);

                advance_edi(builder, dst.size());
            }

            Scasb | Scasw | Scasd => {
                operands!([cmp, src], &instr);

                // this code duplicates Sub & Cmp...
                let lhs = builder.load_operand(cmp);
                let rhs = builder.load_operand(src);
                let res = builder.sub(lhs, rhs);

                let of = builder.ssub_overflow(lhs, rhs);
                let cf = builder.usub_overflow(lhs, rhs);

                // The OF, SF, ZF, AF, PF, and CF flags are set according
                //   to the temporary result of the comparison.
                // AF and PF are not implemented rn
                // not that they are actually useful...
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Overflow, of);
                builder.store_flag(Flag::Carry, cf);

                advance_edi(builder, src.size());
            }
            Cmpsb | Cmpsw | Cmpsd => {
                operands!([lhs, rhs], &instr);

                // this code duplicates Sub & Cmp...
                let lhs = builder.load_operand(lhs);
                let rhs = builder.load_operand(rhs);
                let res = builder.sub(lhs, rhs);

                let of = builder.ssub_overflow(lhs, rhs);
                let cf = builder.usub_overflow(lhs, rhs);

                // The OF, SF, ZF, AF, PF, and CF flags are set according
                //   to the temporary result of the comparison.
                // AF and PF are not implemented rn
                // not that they are actually useful...
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Overflow, of);
                builder.store_flag(Flag::Carry, cf);

                advance_esi(builder, lhs.size());
                advance_edi(builder, lhs.size());
            }
            _ => unreachable!(),
        }
    };

    // those handle the repetition
    enum Prefix {
        Rep,
        Repe,
        Repne,
    }

    // REP and REPE are actually encoded the same way
    // Semantics depend on the instruction encoded
    let prefix = if instr.has_rep_prefix() {
        use Mnemonic::*;
        match instr.mnemonic() {
            Scasb | Scasw | Scasd | Cmpsb | Cmpsw | Cmpsd => Some(Prefix::Repe),
            _ => Some(Prefix::Rep),
        }
    } else if instr.has_repne_prefix() {
        Some(Prefix::Repne)
    } else {
        None
    };

    if let Some(prefix) = prefix {
        let count_reg = Register::ECX;

        let start_count = builder.load_register(count_reg);
        let should_enter = builder.icmp(ComparisonType::NotEqual, start_count, builder.make_u32(0));
        builder.ifelse(
            should_enter,
            |builder| {
                builder.repeat_until(|builder| {
                    execute_instr(builder);

                    let counter = builder.load_register(count_reg);
                    let counter = builder.sub(counter, builder.make_u32(1));

                    builder.store_register(count_reg, counter);

                    let counter_continue =
                        builder.icmp(ComparisonType::NotEqual, counter, builder.make_u32(0));

                    let additional_continue = match prefix {
                        Prefix::Rep => builder.make_true(),
                        Prefix::Repe => builder.load_flag(Flag::Zero),
                        Prefix::Repne => {
                            let zf = builder.load_flag(Flag::Zero);
                            builder.bool_not(zf)
                        }
                    };

                    builder.bool_and(counter_continue, additional_continue)
                });
            },
            |_| {},
        );
    } else {
        execute_instr(builder)
    }

    ControlFlow::NextInstruction
}
