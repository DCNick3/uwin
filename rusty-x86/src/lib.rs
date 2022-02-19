extern crate core;

pub mod backend;
pub mod disasm;
pub mod llvm;
pub mod memory_image;
pub mod types;

use crate::backend::{Builder, ComparisonType, IntValue};
use crate::disasm::Operands;
use crate::types::Register::*;
use crate::types::{ControlFlow, Flag, IntType, Operand, Register};
use iced_x86::{ConditionCode, Instruction, Mnemonic};

#[allow(clippy::let_and_return)]
fn compute_condition_code<B: Builder>(
    builder: &mut B,
    condition_code: ConditionCode,
) -> B::BoolValue {
    let mut comp = |cc| compute_condition_code(builder, cc);

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
            let r = comp(be);
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
            let r = comp(l);
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
            let r = comp(le);
            builder.bool_not(r)
        }
    }
}

#[rustfmt::skip]
fn is_cmovcc(mnemonic: Mnemonic) -> bool {
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

fn codegen_string_instr<B: Builder>(builder: &mut B, instr: Instruction) {
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

            Lodsb | Lodsw | Lodsd | Cmpsb | Cmpsw | Cmpsd => {
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
}

// TODO: handle control flow
pub fn codegen_instr<B: Builder>(builder: &mut B, instr: Instruction) -> ControlFlow<B> {
    use crate::Flag::*;
    use iced_x86::Mnemonic::*;

    assert!(!instr.has_lock_prefix());
    assert!(!instr.has_xacquire_prefix());
    assert!(!instr.has_xrelease_prefix());

    if instr.is_string_instruction() {
        codegen_string_instr(builder, instr);
        return ControlFlow::NextInstruction;
    }

    assert!(!instr.has_rep_prefix());
    assert!(!instr.has_repe_prefix());
    assert!(!instr.has_repne_prefix());

    let mnemonic = instr.mnemonic();

    if instr.is_jcc_short_or_near() {
        operands!([target], &instr);

        let code = instr.condition_code();
        let cond = compute_condition_code(builder, code);

        ControlFlow::Conditional(cond, target.as_imm32())
    } else if is_cmovcc(instr.mnemonic()) {
        operands!([dst, src], &instr);

        let code = instr.condition_code();
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
    } else {
        match mnemonic {
            // TODO: there is (going to be) a ton of opcodes, we would want to handle this nicely (a bit of macromagic?)
            Nop => {
                // fuf, this was easy
            }
            Mov => {
                operands!([dst, src], &instr);

                let val = builder.load_operand(src);
                builder.store_operand(dst, val);
            }
            Movzx => {
                operands!([dst, src], &instr);

                let val = builder.load_operand(src);
                let val = builder.zext(val, dst.size());
                builder.store_operand(dst, val);
            }
            Movsx => {
                operands!([dst, src], &instr);

                let val = builder.load_operand(src);
                let val = builder.sext(val, dst.size());
                builder.store_operand(dst, val);
            }
            Add => {
                operands!([dst, src], &instr);

                let lhs = builder.load_operand(dst);
                let rhs = builder.load_operand(src);
                let res = builder.add(lhs, rhs);

                builder.store_operand(dst, res);

                let of = builder.sadd_overflow(lhs, rhs);
                let cf = builder.uadd_overflow(lhs, rhs);

                // The OF, SF, ZF, AF, PF, and CF flags are set according to the result.
                // AF and PF are not implemented rn
                // not that they are actually useful...
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Overflow, of);
                builder.store_flag(Flag::Carry, cf);
            }
            Sub | Cmp => {
                operands!([dst, src], &instr);

                let lhs = builder.load_operand(dst);
                let rhs = builder.load_operand(src);
                let res = builder.sub(lhs, rhs);

                if mnemonic == Sub {
                    builder.store_operand(dst, res);
                }

                let of = builder.ssub_overflow(lhs, rhs);
                let cf = builder.usub_overflow(lhs, rhs);

                // The OF, SF, ZF, AF, PF, and CF flags are set according to the result.
                // AF and PF are not implemented rn
                // not that they are actually useful...
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Overflow, of);
                builder.store_flag(Flag::Carry, cf);
            }
            Sbb => {
                operands!([dst, src], &instr);

                let lhs = builder.load_operand(dst);
                let rhs = builder.load_operand(src);
                let borrow = builder.load_flag(Carry);
                let borrow = builder.bool_to_int(borrow, lhs.size());

                let res = builder.sub(lhs, rhs);

                let of_base = builder.ssub_overflow(lhs, rhs);
                let of_borrow = builder.ssub_overflow(res, borrow);
                let of = builder.bool_or(of_base, of_borrow);

                let cf_base = builder.usub_overflow(lhs, rhs);
                let cf_borrow = builder.usub_overflow(res, borrow);
                let cf = builder.bool_or(cf_base, cf_borrow);

                let res = builder.sub(res, borrow);
                builder.store_operand(dst, res);

                // The OF, SF, ZF, AF, PF, and CF flags are set according to the result.
                // AF and PF are not implemented rn
                // not that they are actually useful...
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Overflow, of);
                builder.store_flag(Flag::Carry, cf);
            }
            Lea => {
                operands!([dst, src], &instr);

                let addr = match src {
                    Operand::Memory(m) => builder.compute_memory_operand_address(m),
                    _ => panic!("Expected 2nd lea operand to be memory reference"),
                };

                // TODO: size conversion (store 32-bit address as 16-bit value for example)
                assert_eq!(dst.size(), addr.size());
                builder.store_operand(dst, addr);
            }
            Dec => {
                operands!([dst], &instr);

                let val = builder.load_operand(dst);

                let one = builder.make_int_value(val.size(), 1, false);

                let res = builder.sub(val, one);

                builder.store_operand(dst, res);

                let of = builder.ssub_overflow(val, one);

                // The CF flag is not affected. The OF, SF, ZF, AF, and PF flags are set according to the result.
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Overflow, of);
            }
            Inc => {
                operands!([dst], &instr);

                let val = builder.load_operand(dst);

                let one = builder.make_int_value(val.size(), 1, false);

                let res = builder.add(val, one);

                builder.store_operand(dst, res);

                let of = builder.sadd_overflow(val, one);

                // The CF flag is not affected. The OF, SF, ZF, AF, and PF flags are set according to the result.
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Overflow, of);
            }
            Neg => {
                operands!([dst], &instr);

                let val = builder.load_operand(dst);

                let zero = builder.make_int_value(val.size(), 0, false);

                let res = builder.int_neg(val);
                builder.store_operand(dst, res);

                let of = builder.ssub_overflow(zero, val);
                let cf = builder.usub_overflow(zero, val);
                // https://stackoverflow.com/questions/44837231/how-does-the-neg-instruction-affect-the-flags-on-x86
                // flags are equivalent to sub 0, dst
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Overflow, of);
                builder.store_flag(Flag::Carry, cf);
            }
            Cwd | Cdq => {
                let (hi, lo) = match mnemonic {
                    Cwd => (DX, AX),
                    Cdq => (EDX, EAX),
                    _ => unreachable!(),
                };
                let val = builder.load_register(lo);
                let extended = builder.extract_msb(val);
                let extended = builder.bool_to_int(extended, val.size());
                let extended = builder.int_neg(extended);
                builder.store_register(hi, extended);
            }
            Imul => {
                let (dst, src1, src2) = match *instr.get_operands().as_slice() {
                    [src] => match src.size() {
                        IntType::I8 => (Operand::Register(AX), Operand::Register(AL), src),
                        IntType::I16 => (Operand::RegisterPair(DX, AX), Operand::Register(AX), src),
                        IntType::I32 => {
                            (Operand::RegisterPair(EDX, EAX), Operand::Register(EAX), src)
                        }
                        IntType::I64 => unimplemented!(),
                    },
                    [dst, src] => {
                        assert_eq!(dst.size(), src.size());

                        (dst, dst, src)
                    }
                    [dst, src1, src2] => {
                        assert_eq!(dst.size(), src1.size());
                        assert_eq!(src1.size(), src2.size());

                        (dst, src1, src2)
                    }
                    _ => unreachable!(),
                };

                let lhs = builder.load_operand(src1);
                let rhs = builder.load_operand(src2);

                let double_size = lhs.size().double_sized();

                let lhs = builder.sext(lhs, double_size);

                let rhs = builder.sext(rhs, double_size);

                let res = builder.mul(lhs, rhs);

                // this one might be single sized or double-sized depending on form of imul used
                let res_stored = builder.trunc(res, dst.size());
                // this one will always be signled sized and is used for overflow computation
                let res_trunc = builder.trunc(res, src1.size());

                let res_trunc_ext = builder.sext(res_trunc, res.size());
                let overflow = builder.icmp(ComparisonType::NotEqual, res, res_trunc_ext);

                // TODO: flags (based on comparison of res and sext(res_trunc))
                // For the one operand form of the instruction, the CF and OF flags are set
                // when significant bits are carried into the upper half of the result and
                // cleared when the result fits exactly in the lower half of the result.

                // For the two- and three-operand forms of the instruction,
                // the CF and OF flags are set when the result must be truncated to fit in the
                // destination operand size and cleared when the result fits exactly
                // in the destination operand size.

                // The SF, ZF, AF, and PF flags are undefined.
                // TODO: do we want to represent ub here? leaving as zero for now
                builder.store_flag(Flag::Zero, builder.make_false());
                builder.store_flag(Flag::Sign, builder.make_false());
                builder.store_flag(Flag::Overflow, overflow);
                builder.store_flag(Flag::Carry, overflow);

                builder.store_operand(dst, res_stored)
            }
            Xor => {
                operands!([dst, src], &instr);

                let lhs = builder.load_operand(dst);
                let rhs = builder.load_operand(src);

                let res = builder.int_xor(lhs, rhs);

                builder.store_operand(dst, res);

                // The OF and CF flags are cleared; the SF, ZF, and PF flags are set according to the result.
                // The state of the AF flag is undefined.
                // TODO: do we want to represent ub here? leaving as zero for now
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Carry, builder.make_false());
                builder.store_flag(Flag::Overflow, builder.make_false());
            }
            Not => {
                operands!([dst], &instr);

                let val = builder.load_operand(dst);
                let val = builder.int_not(val);

                builder.store_operand(dst, val);
            }
            And | Test => {
                operands!([dst, src], &instr);

                let lhs = builder.load_operand(dst);
                let rhs = builder.load_operand(src);

                let res = builder.int_and(lhs, rhs);

                if mnemonic == And {
                    builder.store_operand(dst, res);
                }
                // The OF and CF flags are cleared; the SF, ZF, and PF flags are set according to the result. The state of the AF flag is
                // undefined.
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Carry, builder.make_false());
                builder.store_flag(Flag::Overflow, builder.make_false());
            }
            Or => {
                operands!([dst, src], &instr);

                let lhs = builder.load_operand(dst);
                let rhs = builder.load_operand(src);

                let res = builder.int_or(lhs, rhs);

                builder.store_operand(dst, res);

                // The OF and CF flags are cleared; the SF, ZF, and PF flags are set according to the result. The state of the AF flag is
                // undefined.
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Carry, builder.make_false());
                builder.store_flag(Flag::Overflow, builder.make_false());
            }
            Shr | Sar | Shl => {
                operands!([dst, count], &instr);

                let count = builder.load_operand(count);
                let count = builder.zext(count, IntType::I32);

                let count_mask = builder.make_u32(0x1f);
                let count = builder.int_and(count, count_mask);

                let not_zero = builder.icmp(
                    ComparisonType::NotEqual,
                    count,
                    builder.make_int_value(count.size(), 0, false),
                );

                let (arithmetic, right) = match mnemonic {
                    Shr => (false, true),
                    Sar => (true, true),
                    Shl => (false, false),
                    _ => unreachable!(),
                };

                builder.ifelse(
                    not_zero,
                    |builder| {
                        let val = builder.load_operand(dst);
                        let val = if arithmetic {
                            builder.sext(val, IntType::I32)
                        } else {
                            builder.zext(val, IntType::I32)
                        };

                        let res = match mnemonic {
                            Shr => builder.lshr(val, count),
                            Sar => builder.ashr(val, count),
                            Shl => builder.shl(val, count),
                            _ => unreachable!(),
                        };

                        let count_sub_1 = builder.sub(count, builder.make_u32(1));

                        let res_msb_bit_number =
                            builder.make_u32((dst.size().bit_width() - 1) as u32);
                        let cf = if right {
                            builder.extract_bit(val, count_sub_1)
                        } else {
                            let shifted_one_less = builder.shl(val, count_sub_1);
                            builder.extract_bit(shifted_one_less, res_msb_bit_number)
                        };

                        // OF is defined only for 1-bit shifts, but we'll compute it anyways
                        // maybe we can get better by telling LLVM it's undef?
                        let of = match mnemonic {
                            Shr => builder.extract_msb(val),
                            Sar => builder.make_false(),
                            Shl => {
                                let msb = builder.extract_bit(res, res_msb_bit_number);
                                builder.bool_xor(msb, cf)
                            }
                            _ => unreachable!(),
                        };

                        let res = builder.trunc(res, dst.size());
                        builder.store_operand(dst, res);

                        // The CF flag contains the value of the last bit shifted out of the
                        // destination operand; it is undefined for SHL and SHR instructions where
                        // the count is greater than or equal to the size (in bits) of the
                        // destination operand (TODO: add undef?). The OF flag is affected only
                        // for 1-bit shifts (see “Description” above); otherwise, it is undefined.
                        // The SF, ZF, and PF flags are set according to the result.
                        // If the count is 0, the flags are not affected.
                        // For a non-zero count, the AF flag is undefined.
                        builder.compute_and_store_zf(res);
                        builder.compute_and_store_sf(res);
                        builder.store_flag(Flag::Carry, cf);
                        builder.store_flag(Flag::Overflow, of);
                    },
                    |_| {
                        // nuff to do
                    },
                );
            }
            Div | Idiv => {
                operands!([src], &instr);

                let double_size = src.size().double_sized();

                let (lo, hi, quo_dst, rem_dst) = match src.size() {
                    IntType::I8 => todo!(),
                    IntType::I16 => todo!(),
                    IntType::I32 => (EAX, EDX, EAX, EDX),
                    _ => unreachable!(),
                };

                assert_eq!(lo.size(), hi.size());

                let dividend = builder.load_operand(Operand::RegisterPair(hi, lo));

                let divisor = builder.load_operand(src);
                let divisor = if mnemonic == Div {
                    builder.zext(divisor, double_size)
                } else {
                    builder.sext(divisor, double_size)
                };

                let quotient = if mnemonic == Div {
                    builder.udiv(dividend, divisor)
                } else {
                    builder.sdiv(dividend, divisor)
                };

                // TODO: test overflow and trap if out of bounds

                // calculate the remainder
                let whole = builder.mul(quotient, divisor);
                let remainder = builder.sub(dividend, whole);

                let quotient = builder.trunc(quotient, src.size());
                let remainder = builder.trunc(remainder, src.size());

                builder.store_register(quo_dst, quotient);
                builder.store_register(rem_dst, remainder);

                // all flags are undefined
            }
            Push => {
                operands!([src], &instr);

                let val = builder.load_operand(src);

                builder.push(val);
            }
            Pop => {
                operands!([dst], &instr);

                let val = builder.pop(dst.size());

                builder.store_operand(dst, val);
            }
            Leave => {
                operands!([], &instr);

                let old_ebp = builder.load_register(EBP);
                builder.store_register(ESP, old_ebp);

                let new_ebp = builder.pop(IntType::I32);

                builder.store_register(EBP, new_ebp);
            }
            Ret => {
                // TODO: control flow, no-op for now
                // Pop the return address (TODO: where to store it? we don't have EIP yet)

                let _raddr = builder.pop(IntType::I32);

                return ControlFlow::Return;
            }
            Jmp => {
                operands!([target], &instr);

                return match target {
                    Operand::Immediate8(_) | Operand::Immediate16(_) | Operand::Immediate64(_) => {
                        panic!("Jump to unsupported immediate size")
                    }
                    Operand::Immediate32(target) => ControlFlow::DirectJump(target),
                    target => {
                        let target = builder.load_operand(target);
                        ControlFlow::IndirectJump(target)
                    }
                };
            }
            Call => {
                operands!([target], &instr);

                let ret = instr.next_ip32();
                builder.push(builder.make_u32(ret));

                match target {
                    Operand::Immediate8(_) | Operand::Immediate16(_) | Operand::Immediate64(_) => {
                        panic!("Call to unsupported immediate size")
                    }
                    Operand::Immediate32(target) => {
                        builder.direct_call(target, instr.next_ip32());
                    }
                    _ => todo!(),
                }
            }
            Stc => builder.store_flag(Carry, builder.make_true()),
            Clc => builder.store_flag(Carry, builder.make_false()),
            Int => {
                // TODO: maybe try to handle int 3 and other stuff differently?
                // Also wanna have runtime info on WTF has happened
                builder.trap();
            }

            // TODO: uncomment when unit tests for different direction of string operations will be in place
            //Std => builder.store_flag(Direction, builder.make_true()),
            //Cld => builder.store_flag(Direction, builder.make_false()),
            m => panic!("Unknown instruction mnemonic: {:?}", m),
        };

        ControlFlow::NextInstruction
    }
}

/// use dynasm to assemble the provided code to a Vec<u8>
#[macro_export]
macro_rules! assemble_x86 {
    ($($assembly:tt)*) => {
        {
            #[allow(unused)]
            use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
            let mut ops = dynasmrt::x86::Assembler::new().unwrap();

            dynasm!(ops
                ; .arch x86
                $($assembly)*
            );

            let result: Vec<u8> = ops.finalize().unwrap().to_vec();
            result
        }
    }
}

#[macro_export]
macro_rules! assemble_aarch64 {
    ($($assembly:tt)*) => {
        {
            #[allow(unused)]
            use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
            let mut ops = dynasmrt::aarch64::Assembler::new().unwrap();

            dynasm!(ops
                ; .arch aarch64
                $($assembly)*
            );

            let result: Vec<u8> = ops.finalize().unwrap().to_vec();
            result
        }
    }
}

#[cfg(test)]
mod tests {
    // test that above helper macro works =)
    mod assembly {
        #[test]
        fn basic_assembly() {
            use dynasmrt::{dynasm, DynasmApi};
            let mut ops = dynasmrt::x86::Assembler::new().unwrap();

            dynasm!(ops
                ; .arch x86
                ; mov eax, 42
            );

            let result = ops.finalize().unwrap().to_vec();
            assert_eq!(&*result, b"\xB8\x2A\x00\x00\x00");
        }

        #[test]
        fn macro_assembly() {
            let result = assemble_x86!(
                ; mov eax, 42
            );
            assert_eq!(result, b"\xB8\x2A\x00\x00\x00");
        }
    }

    mod llvm {
        use crate::llvm;
        use crate::memory_image::MemoryImage;
        use inkwell::context::Context;
        use inkwell::targets::FileType;
        #[allow(unused_imports)]
        use log::{debug, error, info, trace, warn};
        use std::fmt::Write;
        use test_log::test;

        fn recompile(code: &[u8]) -> Vec<u8> {
            let context = &Context::create();
            let types = &llvm::backend::Types::new(context);
            let rt_funs = &llvm::backend::RuntimeHelpers::dummy(types);

            let code = MemoryImage::from_code_region(0x1000, code);

            let module = llvm::recompile(context, types, rt_funs, &code, &[0x1000]);

            let target_machine = llvm::get_aarch64_target_machine();

            let ir = module.print_to_string().to_string();

            trace!("llvm ir:\n{}", ir);

            module.verify().unwrap();

            let memory_buffer = target_machine
                .write_to_memory_buffer(&module, FileType::Object)
                .unwrap();

            let _raw_buffer = format!("{:?}", memory_buffer.as_slice());

            let object_file = memory_buffer.create_object_file().unwrap();

            let mut contents: Option<Vec<u8>> = None;
            for sec in object_file.get_sections() {
                let name = sec.get_name().and_then(|x| x.to_str().ok());
                if name == Some(".text") {
                    contents = Some(Vec::from(sec.get_contents()));
                }
            }

            contents.unwrap()
        }

        fn test_recomp(x86_code: Vec<u8>, expected_aarch64_code: Vec<u8>) {
            debug!(
                "CODE:\n{}",
                crate::disasm::disassemble(x86_code.as_slice(), 0x1000)
            );

            let result = recompile(x86_code.as_slice());

            fn disasm_aarch64(aarch64_code: Vec<u8>) -> String {
                let mut res = String::new();
                for maybe_decoded in bad64::disasm(aarch64_code.as_slice(), 0x1000) {
                    let decoded = maybe_decoded.unwrap();
                    writeln!(res, "{:08x} {}", decoded.address(), decoded).unwrap();
                }
                res
            }

            let result = disasm_aarch64(result);
            let expected = disasm_aarch64(expected_aarch64_code);

            debug!("expected aarch64:\n{}", expected);
            debug!("actual aarch64:\n{}", result);

            assert_eq!(result, expected);
        }

        #[test]
        fn simple_llvm() {
            // we get this
            let code = assemble_x86!(
                ; mov ebx, 42
                ; ret
            );

            // and recompile it into this
            // isn't it nice?
            let expected_result = assemble_aarch64!(
            ; ->indirect_bb_call:
                ; cmp w2, #0x1, lsl #0xc
                ; b.ne >FAIL
                ; b ->bb_0x1000
                ; FAIL:
                ; brk #0x1

            ; ->bb_0x1000:
                ; ldr w8, [x0, #0x10]
                ; mov w9, #0x2a
                ; str w9, [x0, #0x4]
                ; add w8, w8, #0x4
                ; str w8, [x0, #0x10]
                ; ret
            );

            test_recomp(code, expected_result);
        }

        #[test]
        fn magic_llvm() {
            // unsigned magic(unsigned a, unsigned b) {
            //     return a * (b - a) / (13 + a);
            // }
            let code = assemble_x86!(
                ; mov     edx, DWORD [esp+4]
                ; mov     eax, DWORD [esp+8]
                ; sub     eax, edx
                ; lea     ecx, [edx+13]
                ; imul    eax, edx
                ; xor     edx, edx
                ; div     ecx
                ; ret
            );

            /*  ; ->fact:
            ; mov     eax, DWORD [esp+4]
            ; mov     edx, 1
            ; cmp     eax, 1
            ; jbe     ->L1
            ; ->L2:
            ; mov     ecx, eax
            ; sub     eax, 1
            ; imul    edx, ecx
            ; cmp     eax, 1
            ; jne     ->L2
            ; ->L1:
            ; mov     eax, edx
            ; ret*/

            // and recompile it into this
            // isn't it nice?
            let expected_result = assemble_aarch64!(
            ; ->indirect_bb_call:
                ; cmp w2, #0x1, lsl #0xc
                ; b.ne >FAIL
                ; b ->bb_0x1000
                ; FAIL:
                ; brk #0x1

                // ; ldr w8, [x0, #0x10] // load ESP
                // ; add w9, w8, #4
                // ; ldr w9, [x1, w9, sxtw] // load [ESP+4] (a)
                // ; add w8, w8, #8
                // ; str w9, [x0, #0xc] // store [ESP+4] as EDX
                // ; ldr w8, [x1, w8, sxtw] // load [ESP+8] (b)
                // ; add w10, w9, #0xd // compute EDX+13
                // ; str w10, [x0, #8] // store EDX+13 as ECX
                // ; sub w8, w8, w9 // compute b-a
                // ; mul w8, w8, w9 // compute a*(b-a)
                // ; udiv x9, x8, x10 // compute division of a*(b-a)/(13+a)
                // ; msub x8, x9, x10, x8 // compute remainder
                // ; str x9, [x0] // store div result in EAX
                // ; stur x8, [x0, #0xc] // store remainder in EDX
                // ; ret

            ; ->bb_0x1000:
                ; ldr w8, [x0, #0x10]
                ; add w9, w8, #0x4
                ; ldrsw x10, [x1, w9, uxtw]
                ; add w8, w8, #0x8
                ; str w10, [x0, #0xc]
                ; ldr w8, [x1, w8, uxtw]
                ; strb wzr, [x0, #0x20]
                ; str w9, [x0, #0x10]
                ; subs w8, w8, w10
                ; cset w11, eq
                ; strb w11, [x0, #0x23]
                ; lsr w11, w8, #0x1f
                ; sxtw x8, w8
                ; strb w11, [x0, #0x24]
                ; mov w11, #0x1
                ; mul x8, x8, x10
                ; add w10, w10, #0xd
                ; sturh w11, [x0, #0x23]
                ; and x11, x8, #0xffffffff
                ; udiv x11, x11, x10
                ; msub w8, w11, w10, w8
                ; strh wzr, [x0, #0x24]
                ; str w11, [x0]
                ; stp w10, w8, [x0, #0x8]
                ; ret
            );

            test_recomp(code, expected_result);
        }

        #[test]
        fn answer_llvm() {
            // int answer(int a) {
            //     if (a == 1)
            //         return 42;
            //     return -1;
            // }
            let code = assemble_x86!(
                ; push ebp
                ; mov ebp, esp
                ; cmp DWORD [ebp+8], 1
                ; jne ->L2
                ; mov eax, 42
                ; jmp ->L3
                ; ->L2:
                ; mov eax, -1
                ; ->L3:
                ; pop ebp
                ; ret
            );

            let expected_result = assemble_aarch64!(
                ; ->indirect_bb_call:
                ; cmp w2, #0x1, lsl #0xc
                ; b.eq >L1
                ; mov w8, #0x101a
                ; cmp w2, w8
                ; b.eq >L2
                ; mov w8, #0x101f
                ; cmp w2, w8
                ; b.ne >FAIL
                ; b ->basic_block_0000101F
                ;L1:
                ; b ->basic_block_00001000
                ;L2:
                ; b ->basic_block_0000101A
                ;FAIL:
                ; brk #0x1

                ;->basic_block_00001000:
                ; ldp w8, w9, [x0, #0x10]
                ; sub w8, w8, #4
                ; str w8, [x0, #0x10]
                ; str w9, [x1, w8, uxtw]
                ; ldr w8, [x0, #0x10]
                ; str w8, [x0, #0x14]
                ; add w8, w8, #8
                ; ldr w8, [x1, w8, uxtw]
                ; subs w8, w8, #1
                ; cset w11, eq
                ; cset w9, vs
                ; cset w10, cc
                ; strb w11, [x0, #0x23]
                ; lsr w11, w8, #0x1f
                ; strb w11, [x0, #0x24]
                ; strb w9, [x0, #0x25]
                ; strb w10, [x0, #0x20]
                ; cbz w8, ->FALSE

                ; b ->basic_block_0000101A

                ; ->FALSE:
                ; mov w8, #0x2a
                ; str w8, [x0]
                ; b ->basic_block_0000101F

                ; ->basic_block_0000101A:
                ; ldr w8, [x0, #0x10]
                ; movn w9, #0
                ; str w9, [x0]
                ; ldr w9, [x1, x8]
                ; add w8, w8, #8
                ; stp w8, w9, [x0, #0x10]
                ; ret

                ; ->basic_block_0000101F:
                ; ldr w8, [x0, #0x10]
                ; ldr w9, [x1, x8]
                ; add w8, w8, #0x8
                ; stp w8, w9, [x0, #0x10]
                ; ret
            );

            test_recomp(code, expected_result);
        }

        #[test]
        fn jump_over_llvm() {
            let code = assemble_x86!(
                ; jmp ->LABEL
                ; add eax, 1
                ; ->LABEL:
                ; mov eax, 42
            );

            let expected_result = assemble_aarch64!(
            ; ->indirect_bb_call:
                ; cmp w2, #0x1, lsl #0xc
                ; b.eq >jump_bb_0x1000
                ; mov w8, #0x1008
                ; cmp w2, w8
                ; b.ne >FAIL
                ; b ->bb_0x1005
            ; jump_bb_0x1000:
                ; b ->bb_0x1000
                ; FAIL:
                ; brk #0x1

            ; ->bb_0x1000:
                ; b ->bb_0x1005

            ; ->bb_0x1005:
                ; mov w8, #0x2a
                ; str w8, [x0, #0]
                ; ret
            );

            test_recomp(code, expected_result);
        }

        #[test]
        fn div_llvm() {
            let code = assemble_x86!(
                ; mov eax, 1
                ; mov ebx, 2
                ; div ebx
            );

            let expected_result = assemble_aarch64!(
                ; ->indirect_bb_call:
                ; cmp w2, #0x1, lsl #0xc
                ; b.ne >FAIL
                ; b ->bb_0x1000
                ; FAIL:
                ; brk #0x1

                ; ->bb_0x1000:
                ; ldr w8, [x0, #0xc]  // load EDX
                ; mov w9, #0x2
                ; mov w10, #0x1
                ; str w10, [x0, #0xc] // store 0x1 to EDX
                ; lsl w8, w8, #0x1f   // it was division of EDX:EAX by 2, so the lo bit of EDX becomes the MSB of EAX
                ; stp w8, w9, [x0]    // store w8->EAX, 2->EBX
                ; ret

            );

            test_recomp(code, expected_result);
        }

        #[test]
        fn mov_al_42_llvm() {
            let code = assemble_x86!(
                ; mov al, 42
            );

            let expected_result = assemble_aarch64!(
                ; ->indirect_bb_call:
                ; cmp w2, #0x1, lsl #0xc
                ; b.ne >FAIL
                ; b ->bb_0x1000
                ; FAIL:
                ; brk #0x1

                // it's all optimized down to just storing a byte, nice
                ; ->bb_0x1000:
                ; mov w8, #0x2a
                ; strb w8, [x0]
                ; ret
            );

            test_recomp(code, expected_result);
        }

        #[test]
        fn mov_ax_42_llvm() {
            let code = assemble_x86!(
                ; mov ax, 42
            );

            let expected_result = assemble_aarch64!(
                ; ->indirect_bb_call:
                ; cmp w2, #0x1, lsl #0xc
                ; b.ne >FAIL
                ; b ->bb_0x1000
                ; FAIL:
                ; brk #0x1

                // it's all optimized down to just storing a half-word, nice
                ; ->bb_0x1000:
                ; mov w8, #0x2a
                ; strh w8, [x0]
                ; ret
            );

            test_recomp(code, expected_result);
        }
    }
}
