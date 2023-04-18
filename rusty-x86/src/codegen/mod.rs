mod prelude {
    pub use crate::backend::{BoolValue, Builder, ComparisonType, IntValue};
    pub use crate::disasm::Operands;
    pub use crate::types::{ControlFlow, Flag, IntType, Operand, Register, Register::*};
    pub use iced_x86::{Code, ConditionCode, Instruction, Mnemonic};
}

use log::warn;

mod conditional;
mod mov;
mod string;

use crate::{operands, operands_ty};
use prelude::*;

pub fn codegen_instr<B: Builder>(
    builder: &mut B,
    instr: Instruction,
) -> ControlFlow<B::IntValue, B::BoolValue> {
    use crate::Flag::*;
    use iced_x86::Mnemonic::*;

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
        string::codegen_string_instr(builder, instr);
        return ControlFlow::NextInstruction;
    }

    assert!(!instr.has_rep_prefix());
    assert!(!instr.has_repe_prefix());
    assert!(!instr.has_repne_prefix());

    let mnemonic = instr.mnemonic();

    if instr.is_jcc_short_or_near() {
        operands!([target], &instr);

        let code = instr.condition_code();
        let cond = conditional::compute_condition_code(builder, code);

        ControlFlow::Conditional(cond, target.as_imm32())
    } else if conditional::is_cmovcc(instr.mnemonic()) {
        operands!([dst, src], &instr);

        let code = instr.condition_code();
        let cond = conditional::compute_condition_code(builder, code);

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
    } else if conditional::is_setcc(instr.mnemonic()) {
        operands!([dst], &instr);

        let code = instr.condition_code();
        let cond = conditional::compute_condition_code(builder, code);

        let res = builder.bool_to_int(cond, IntType::I8);

        builder.store_operand(dst, res);

        ControlFlow::NextInstruction
    } else {
        match mnemonic {
            // TODO: there is (going to be) a ton of opcodes, we would want to handle this nicely (a bit of macromagic?)
            Nop => {
                // fuf, this was easy
            }
            Mov => {
                operands!([dst, src], &instr);
                mov::mov(builder, dst, src);
            }
            Movzx => {
                operands!([dst, src], &instr);
                mov::movzx(builder, dst, src);
            }
            Movsx => {
                operands!([dst, src], &instr);
                mov::movsx(builder, dst, src);
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
            Adc => {
                operands!([dst, src], &instr);

                let lhs = builder.load_operand(dst);
                let rhs = builder.load_operand(src);

                let carry = builder.load_flag(Carry);
                let carry = builder.bool_to_int(carry, lhs.size());

                let res = builder.add(lhs, rhs);

                let of_base = builder.sadd_overflow(lhs, rhs);
                let of_borrow = builder.sadd_overflow(res, carry);
                let of = builder.bool_xor(of_base, of_borrow);

                let cf_base = builder.uadd_overflow(lhs, rhs);
                let cf_borrow = builder.uadd_overflow(res, carry);
                let cf = builder.bool_xor(cf_base, cf_borrow);

                let res = builder.add(res, carry);
                builder.store_operand(dst, res);

                // The OF, SF, ZF, AF, PF, and CF flags are set according to the result.
                // AF and PF are not implemented rn
                // not that they are actually useful...
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Overflow, of);
                builder.store_flag(Carry, cf);
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
                let of = builder.bool_xor(of_base, of_borrow);

                let cf_base = builder.usub_overflow(lhs, rhs);
                let cf_borrow = builder.usub_overflow(res, borrow);
                let cf = builder.bool_xor(cf_base, cf_borrow);

                let res = builder.sub(res, borrow);
                builder.store_operand(dst, res);

                // The OF, SF, ZF, AF, PF, and CF flags are set according to the result.
                // AF and PF are not implemented rn
                // not that they are actually useful...
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Overflow, of);
                builder.store_flag(Carry, cf);
            }
            Lea => {
                operands!([dst, src], &instr);

                let addr = match src {
                    Operand::Memory(m) => builder.compute_memory_operand_address(m),
                    _ => panic!("Expected 2nd lea operand to be memory reference"),
                };

                let addr = if dst.size() == IntType::I16 {
                    builder.trunc(addr, dst.size())
                } else {
                    addr
                };

                // TODO: more size conversion?
                assert_eq!(dst.size(), addr.size());
                builder.store_operand(dst, addr);
            }
            Dec => {
                operands!([dst], &instr);

                let val = builder.load_operand(dst);

                let one = builder.make_int_value(val.size(), 1);

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

                let one = builder.make_int_value(val.size(), 1);

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

                let zero = builder.make_int_value(val.size(), 0);

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
                // TODO: not the best way to write the sign extension?
                let extended = builder.extract_msb(val);
                let extended = builder.bool_to_int(extended, val.size());
                let extended = builder.int_neg(extended);
                builder.store_register(hi, extended);
            }
            Cbw | Cwde => {
                let (dst, src) = match mnemonic {
                    Cbw => (AX, AL),
                    Cwde => (EAX, AX),
                    _ => unreachable!(),
                };
                let val = builder.load_register(src);
                let extended = builder.sext(val, dst.size());
                builder.store_register(dst, extended);
            }
            Mul => {
                operands!([src], &instr);

                let (dst, src1, src2) = match src.size() {
                    IntType::I8 => (Operand::Register(AX), Operand::Register(AL), src),
                    IntType::I16 => (Operand::RegisterPair(DX, AX), Operand::Register(AX), src),
                    IntType::I32 => (Operand::RegisterPair(EDX, EAX), Operand::Register(EAX), src),
                    IntType::I64 => unimplemented!(),
                };

                let lhs = builder.load_operand(src1);
                let rhs = builder.load_operand(src2);

                let double_size = lhs.size().double_sized();

                let lhs = builder.zext(lhs, double_size);
                let rhs = builder.zext(rhs, double_size);

                let res = builder.mul(lhs, rhs);

                let upper_half = builder.ashr(
                    res,
                    builder.make_int_value(res.size(), res.size().bit_width() as u64 / 2),
                );

                let overflow = builder.icmp(
                    ComparisonType::NotEqual,
                    upper_half,
                    builder.make_int_value(res.size(), 0),
                );

                builder.store_operand(dst, res);
                builder.store_flag(Carry, overflow);
                builder.store_flag(Overflow, overflow);
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
                    builder.make_int_value(count.size(), 0),
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
                        builder.store_flag(Carry, cf);
                        builder.store_flag(Overflow, of);
                    },
                    |_| {
                        // nuff to do
                    },
                );
            }
            Rcr => {
                operands!([dst, count], &instr);

                let count = builder.load_operand(count);
                let count = builder.zext(count, dst.size());

                let count_mask = builder.make_int_value(dst.size(), 0x1f);
                let count_orig = builder.int_and(count, count_mask);

                let not_zero = builder.icmp(
                    ComparisonType::NotEqual,
                    count_orig,
                    builder.make_int_value(count_orig.size(), 0),
                );

                builder.ifelse(
                    not_zero,
                    |builder| {
                        let zero = builder.make_int_value(dst.size(), 0);

                        let r#mod =
                            builder.make_int_value(dst.size(), dst.size().bit_width() as u64 + 1);
                        let count = builder.urem(count_orig, r#mod);

                        let value = builder.load_operand(dst);
                        let cf_bool = builder.load_flag(Carry);
                        let cf = builder.bool_to_int(cf_bool, dst.size());

                        let safe_shr = |builder: &mut B,
                                        value: B::IntValue,
                                        amount: B::IntValue|
                         -> B::IntValue {
                            let sz = builder
                                .make_int_value(value.size(), value.size().bit_width() as u64);
                            let overflow =
                                builder.icmp(ComparisonType::UnsignedGreaterOrEqual, amount, sz);

                            let shift = builder.lshr(value, amount);
                            builder.select(overflow, zero, shift)
                        };
                        let safe_shl = |builder: &mut B,
                                        value: B::IntValue,
                                        amount: B::IntValue|
                         -> B::IntValue {
                            let sz = builder
                                .make_int_value(value.size(), value.size().bit_width() as u64);
                            let overflow =
                                builder.icmp(ComparisonType::UnsignedGreaterOrEqual, amount, sz);

                            let shift = builder.shl(value, amount);
                            builder.select(overflow, zero, shift)
                        };

                        let new_value_lo = safe_shr(builder, value, count);
                        let new_value_hi = {
                            let amount = dst.size().bit_width() + 1;
                            let amount = builder.make_int_value(dst.size(), amount as u64);
                            let amount = builder.sub(amount, count);
                            safe_shl(builder, value, amount)
                        };
                        let new_value_mid = {
                            let amount = dst.size().bit_width();
                            let amount = builder.make_int_value(dst.size(), amount as u64);
                            let amount = builder.sub(amount, count);
                            safe_shl(builder, cf, amount)
                        };

                        let new_value = {
                            let lohi = builder.int_or(new_value_lo, new_value_hi);
                            builder.int_or(lohi, new_value_mid)
                        };

                        let new_cf = {
                            let one = builder.make_int_value(dst.size(), 1);
                            let amount = builder.sub(count, one);
                            let new_cf = safe_shr(builder, value, amount);
                            let new_cf = builder.int_and(new_cf, one);

                            let count_is_zero = builder.icmp(ComparisonType::Equal, count, zero);
                            builder.select(count_is_zero, cf, new_cf)
                        };
                        let new_cf = builder.icmp(
                            ComparisonType::NotEqual,
                            new_cf,
                            builder.make_int_value(dst.size(), 0),
                        );

                        // if count_orig != 1 this is undef
                        // TODO: use llvm's undef?
                        let new_of = {
                            let msb = builder.extract_bit(
                                value,
                                builder
                                    .make_int_value(dst.size(), dst.size().bit_width() as u64 - 1),
                            );
                            builder.bool_xor(msb, cf_bool)
                        };

                        builder.store_operand(dst, new_value);
                        builder.store_flag(Carry, new_cf);
                        builder.store_flag(Overflow, new_of);
                    },
                    |_| {
                        // no shift - no game
                    },
                );
            }
            Shld | Shrd => {
                operands!([dst, src, count], &instr);

                let count = builder.load_operand(count);
                let count_mask = builder.make_int_value(count.size(), 0x1f);
                let count = builder.int_and(count, count_mask);
                let count = builder.zext(count, dst.size());

                let count_not_zero = builder.icmp(
                    ComparisonType::NotEqual,
                    count,
                    builder.make_int_value(count.size(), 0),
                );
                builder.ifelse(
                    count_not_zero,
                    |builder| {
                        let val = builder.load_operand(dst);

                        let cf_bit_number = if mnemonic == Shld {
                            builder.sub(
                                builder.make_int_value(dst.size(), dst.size().bit_width() as u64),
                                count,
                            )
                        } else {
                            builder.sub(count, builder.make_int_value(dst.size(), 1))
                        };
                        let cf = builder.extract_bit(val, cf_bit_number);

                        let old_sign = builder.extract_msb(val);

                        let shift_in = builder.load_operand(src);

                        let count = builder.zext(count, val.size().double_sized());

                        let res = if mnemonic == Shld {
                            let double_val = builder.int_concat(val, shift_in);
                            // shift left
                            let shifted = builder.shl(double_val, count);
                            // extract the hi part (our result)
                            builder.lshr(
                                shifted,
                                builder.make_int_value(
                                    double_val.size(),
                                    dst.size().bit_width() as u64,
                                ),
                            )
                        } else {
                            let double_val = builder.int_concat(shift_in, val);
                            builder.lshr(double_val, count)
                        };

                        // clip off the hi part (we don't store it)
                        let res = builder.trunc(res, dst.size());

                        let new_sign = builder.extract_msb(res);
                        let of = builder.bool_xor(old_sign, new_sign);

                        builder.compute_and_store_zf(res);
                        builder.compute_and_store_sf(res);
                        builder.store_flag(Carry, cf);
                        builder.store_flag(Overflow, of);

                        builder.store_operand(dst, res);
                    },
                    |_| {},
                );
            }
            Div | Idiv => {
                operands!([src], &instr);

                let double_size = src.size().double_sized();

                let (dividend_src, quo_dst, rem_dst) = match src.size() {
                    IntType::I8 => (Operand::Register(AX), AL, AH),
                    IntType::I16 => (Operand::RegisterPair(DX, AX), AX, DX),
                    IntType::I32 => (Operand::RegisterPair(EDX, EAX), EAX, EDX),
                    _ => unreachable!(),
                };

                let dividend = builder.load_operand(dividend_src);

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
            Xchg => {
                operands!([op1, op2], &instr);

                let val1 = builder.load_operand(op1);
                let val2 = builder.load_operand(op2);

                builder.store_operand(op1, val2);
                builder.store_operand(op2, val1);
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
            Pushfd => {
                let mut flags = builder.make_u32(1 << 1);

                let mut extract_flag = |builder: &mut B, flag: Flag, bit_number: u32| {
                    let flag_value = builder.load_flag(flag);
                    let flag_value = builder.bool_to_int(flag_value, IntType::I32);
                    let flag_value = builder.shl(flag_value, builder.make_u32(bit_number));

                    flags = builder.int_or(flags, flag_value);
                };

                extract_flag(builder, Carry, 0);
                // bit 1 is reserved, always 1 (set above)
                // ignore parity
                // ignore AUX carry
                extract_flag(builder, Zero, 6);
                extract_flag(builder, Sign, 7);
                extract_flag(builder, Overflow, 11);
                extract_flag(builder, Direction, 10);
                extract_flag(builder, Id, 21);

                builder.push(flags);
            }
            Popfd => {
                let flags = builder.pop(IntType::I32);

                let extract_flag = |builder: &mut B, flag: Flag, bit_number: u32| {
                    let flag_value = builder.extract_bit(flags, builder.make_u32(bit_number));
                    builder.store_flag(flag, flag_value)
                };

                extract_flag(builder, Carry, 0);
                // ignore parity
                // ignore AUX carry
                extract_flag(builder, Zero, 6);
                extract_flag(builder, Sign, 7);
                extract_flag(builder, Overflow, 11);
                extract_flag(builder, Direction, 10);
                extract_flag(builder, Id, 21);
            }
            Leave => {
                operands!([], &instr);

                let old_ebp = builder.load_register(EBP);
                builder.store_register(ESP, old_ebp);

                let new_ebp = builder.pop(IntType::I32);

                builder.store_register(EBP, new_ebp);
            }
            Ret => {
                match instr.code() {
                    Code::Retnd => {
                        operands!([], &instr);

                        let addr = builder.pop(IntType::I32);
                        return ControlFlow::Return(addr);
                    }
                    // TODO: TEST!!!
                    Code::Retnd_imm16 => {
                        operands!([size], &instr);

                        let addr = builder.pop(IntType::I32);
                        let res = ControlFlow::Return(addr);

                        let size_bytes = builder.load_operand(size);
                        let size_bytes = builder.zext(size_bytes, IntType::I32);

                        let esp = builder.load_register(ESP);
                        let esp = builder.add(esp, size_bytes);
                        builder.store_register(ESP, esp);

                        return res;
                    }
                    // don't support weirdly-sized rets (like 'w' and 'q' versions) and far rets
                    _ => unimplemented!("Unsupported ret instruction kind: {:?}", instr.code()),
                }
            }
            Jmp => {
                operands!([target], &instr);

                return match target {
                    Operand::Immediate8(_) | Operand::Immediate16(_) | Operand::Immediate64(_) => {
                        panic!("Jump to unsupported immediate size")
                    }
                    Operand::Immediate32(target) => ControlFlow::DirectJump(target),
                    Operand::FarBranch(segment, offset) => {
                        if segment != 0x7775
                        // magic value encoded as "uw" in little endian (for uwin)
                        // recompiler uses it when it generates stubs
                        {
                            unimplemented!("Far jumps that are not targeting uwin magic segment")
                        }
                        builder.thunk_jump(offset)
                    }
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

                return match target {
                    Operand::Immediate8(_) | Operand::Immediate16(_) | Operand::Immediate64(_) => {
                        panic!("Call to unsupported immediate size")
                    }
                    Operand::Immediate32(target) => builder.direct_call(target),
                    target => {
                        let target = builder.load_operand(target);
                        builder.indirect_call(target)
                    }
                };
            }
            // TODO: implement more bit counting
            Bsr => {
                operands!([dst, src], &instr);

                let value = builder.load_operand(src);

                let is_zero = builder.icmp(
                    ComparisonType::Equal,
                    value,
                    builder.make_int_value(value.size(), 0),
                );
                builder.ifelse(
                    is_zero,
                    |builder| {
                        builder.store_flag(Zero, builder.make_true());
                    },
                    |builder| {
                        let leading_zeroes = builder.ctlz(value);
                        let sz = value.size().bit_width() - 1;
                        let sz = builder.make_int_value(value.size(), sz as u64);
                        let res = builder.sub(sz, leading_zeroes);

                        builder.store_operand(dst, res);
                        builder.store_flag(Zero, builder.make_false());
                    },
                );
            }
            Bt | Btc | Bts | Btr => {
                operands!([base, offset], &instr);

                let base_size = base.size();

                let offset = match offset {
                    Operand::Register(reg) => {
                        assert_eq!(base_size, reg.size());
                        builder.load_register(reg)
                    }
                    Operand::Immediate8(imm) => {
                        assert!(imm < base_size.bit_width());
                        builder.make_int_value(base_size, imm as u64)
                    }
                    _ => unreachable!(),
                };

                fn update_value<B: Builder>(
                    builder: &mut B,
                    mnemonic: Mnemonic,
                    val: <B as Builder>::IntValue,
                    bit_number: <B as Builder>::IntValue,
                ) -> <B as Builder>::IntValue {
                    if mnemonic == Btr {
                        // reset bit
                        builder.reset_bit(val, bit_number)
                    } else if mnemonic == Bts {
                        // set bit
                        builder.set_bit(val, bit_number)
                    } else if mnemonic == Btc {
                        // complement bit
                        let reset = builder.reset_bit(val, bit_number);
                        let set = builder.set_bit(val, bit_number);

                        let current = builder.extract_bit(val, bit_number);

                        builder.select(current, reset, set)
                    } else {
                        // leave it be
                        val
                    }
                }

                match base {
                    Operand::Register(reg) => {
                        let modulo = reg.size().bit_width() as u64;
                        let modulo = builder.make_int_value(base_size, modulo);

                        let bit_number = builder.urem(offset, modulo);

                        let val = builder.load_register(reg);

                        let bit_val = builder.extract_bit(val, bit_number);

                        let val = update_value(builder, mnemonic, val, bit_number);

                        builder.store_register(reg, val);

                        builder.store_flag(Carry, bit_val);
                    }
                    Operand::Memory(mem) => {
                        let addr = builder.compute_memory_operand_address(mem);

                        let offset = builder.sext(offset, IntType::I32);

                        // definitions for T-division and F-division are from the
                        //    Division and Modulus for Computer Scientists paper by Daan Leijen from MS Reasearch:
                        // https://www.microsoft.com/en-us/research/publication/division-and-modulus-for-computer-scientists/

                        let byte_size = builder.make_u32(8);

                        let (byte_offset, bit_number) = {
                            // this uses T-division (as both llvm and most processors do)
                            // but here we want F-division for our calculations
                            // so we do ugly selects
                            let byte_offset = builder.sdiv(offset, byte_size);

                            let bit_number = builder.srem(offset, byte_size);

                            let bit_number_negative = builder.icmp(
                                ComparisonType::SignedLess,
                                bit_number,
                                builder.make_u32(0),
                            );

                            // basically, in case we have a negative offset, adjust byte offset and bit number so that bit number is positive
                            let byte_adj = builder.select(
                                bit_number_negative,
                                builder.make_i32(-1),
                                builder.make_i32(0),
                            );

                            let bit_adj =
                                builder.select(bit_number_negative, byte_size, builder.make_i32(0));

                            let byte_offset = builder.add(byte_offset, byte_adj);
                            let bit_number = builder.add(bit_number, bit_adj);

                            (byte_offset, bit_number)
                        };

                        let bit_number = builder.trunc(bit_number, IntType::I8);

                        let addr = builder.add(addr, byte_offset);

                        let val = builder.load_memory(IntType::I8, addr);

                        let bit_val = builder.extract_bit(val, bit_number);

                        let val = update_value(builder, mnemonic, val, bit_number);

                        builder.store_memory(addr, val);

                        builder.store_flag(Carry, bit_val);
                    }
                    _ => unreachable!(),
                }
            }
            Stc => builder.store_flag(Carry, builder.make_true()),
            Clc => builder.store_flag(Carry, builder.make_false()),
            Int3 | Int => {
                // maybe we can handle Ints different than int3 differently?
                // TODO: give runtime info on WTF has happened
                builder.trap();
            }
            Ud2 => {
                // TODO: give runtime info on WTF has happened
                // probably want an intrinsic
                builder.trap();
            }
            Endbr32 => {
                // don't care, just some noise
            }

            Std => builder.store_flag(Direction, builder.make_true()),
            Cld => builder.store_flag(Direction, builder.make_false()),
            m => {
                warn!(
                    "Unknown instruction mnemonic: {:?} (translating {} @ {:#010x})",
                    m,
                    instr,
                    instr.next_ip32() as usize - instr.len()
                );

                builder.trap();
            }
        };

        ControlFlow::NextInstruction
    }
}
