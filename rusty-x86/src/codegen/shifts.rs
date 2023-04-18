use super::prelude::*;

pub fn shr_sar_shl<B: Builder>(
    builder: &mut B,
    (mnemonic, dst, count): (Mnemonic, Operand, Operand),
) -> CF<B> {
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
        Mnemonic::Shr => (false, true),
        Mnemonic::Sar => (true, true),
        Mnemonic::Shl => (false, false),
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
                Mnemonic::Shr => builder.lshr(val, count),
                Mnemonic::Sar => builder.ashr(val, count),
                Mnemonic::Shl => builder.shl(val, count),
                _ => unreachable!(),
            };

            let count_sub_1 = builder.sub(count, builder.make_u32(1));

            let res_msb_bit_number = builder.make_u32((dst.size().bit_width() - 1) as u32);
            let cf = if right {
                builder.extract_bit(val, count_sub_1)
            } else {
                let shifted_one_less = builder.shl(val, count_sub_1);
                builder.extract_bit(shifted_one_less, res_msb_bit_number)
            };

            // OF is defined only for 1-bit shifts, but we'll compute it anyways
            // maybe we can get better by telling LLVM it's undef?
            let of = match mnemonic {
                Mnemonic::Shr => builder.extract_msb(val),
                Mnemonic::Sar => builder.make_false(),
                Mnemonic::Shl => {
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

    ControlFlow::NextInstruction
}

pub fn rcr<B: Builder>(builder: &mut B, (dst, count): (Operand, Operand)) -> CF<B> {
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

            let r#mod = builder.make_int_value(dst.size(), dst.size().bit_width() as u64 + 1);
            let count = builder.urem(count_orig, r#mod);

            let value = builder.load_operand(dst);
            let cf_bool = builder.load_flag(Carry);
            let cf = builder.bool_to_int(cf_bool, dst.size());

            let safe_shr =
                |builder: &mut B, value: B::IntValue, amount: B::IntValue| -> B::IntValue {
                    let sz = builder.make_int_value(value.size(), value.size().bit_width() as u64);
                    let overflow = builder.icmp(ComparisonType::UnsignedGreaterOrEqual, amount, sz);

                    let shift = builder.lshr(value, amount);
                    builder.select(overflow, zero, shift)
                };
            let safe_shl =
                |builder: &mut B, value: B::IntValue, amount: B::IntValue| -> B::IntValue {
                    let sz = builder.make_int_value(value.size(), value.size().bit_width() as u64);
                    let overflow = builder.icmp(ComparisonType::UnsignedGreaterOrEqual, amount, sz);

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
                    builder.make_int_value(dst.size(), dst.size().bit_width() as u64 - 1),
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

    ControlFlow::NextInstruction
}

pub fn shld_shrd<B: Builder>(
    builder: &mut B,
    (mnemonic, dst, src, count): (Mnemonic, Operand, Operand, Operand),
) -> CF<B> {
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

            let cf_bit_number = if mnemonic == Mnemonic::Shld {
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

            let res = if mnemonic == Mnemonic::Shld {
                let double_val = builder.int_concat(val, shift_in);
                // shift left
                let shifted = builder.shl(double_val, count);
                // extract the hi part (our result)
                builder.lshr(
                    shifted,
                    builder.make_int_value(double_val.size(), dst.size().bit_width() as u64),
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

    ControlFlow::NextInstruction
}
