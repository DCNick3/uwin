use super::prelude::*;

pub fn add<B: Builder>(builder: &mut B, (dst, src): (Operand, Operand)) -> CF<B> {
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
    builder.store_flag(Overflow, of);
    builder.store_flag(Carry, cf);

    ControlFlow::NextInstruction
}

pub fn adc<B: Builder>(builder: &mut B, (dst, src): (Operand, Operand)) -> CF<B> {
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

    ControlFlow::NextInstruction
}

pub fn sub_cmp<B: Builder>(
    builder: &mut B,
    (mnemonic, dst, src): (Mnemonic, Operand, Operand),
) -> CF<B> {
    let lhs = builder.load_operand(dst);
    let rhs = builder.load_operand(src);
    let res = builder.sub(lhs, rhs);

    if mnemonic == Mnemonic::Sub {
        builder.store_operand(dst, res);
    }

    let of = builder.ssub_overflow(lhs, rhs);
    let cf = builder.usub_overflow(lhs, rhs);

    // The OF, SF, ZF, AF, PF, and CF flags are set according to the result.
    // AF and PF are not implemented rn
    // not that they are actually useful...
    builder.compute_and_store_zf(res);
    builder.compute_and_store_sf(res);
    builder.store_flag(Overflow, of);
    builder.store_flag(Carry, cf);

    ControlFlow::NextInstruction
}

pub fn sbb<B: Builder>(builder: &mut B, (dst, src): (Operand, Operand)) -> CF<B> {
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

    ControlFlow::NextInstruction
}

pub fn lea<B: Builder>(builder: &mut B, (dst, src): (Operand, Operand)) -> CF<B> {
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

    ControlFlow::NextInstruction
}

pub fn dec<B: Builder>(builder: &mut B, (dst,): (Operand,)) -> CF<B> {
    let val = builder.load_operand(dst);

    let one = builder.make_int_value(val.size(), 1);

    let res = builder.sub(val, one);

    builder.store_operand(dst, res);

    let of = builder.ssub_overflow(val, one);

    // The CF flag is not affected. The OF, SF, ZF, AF, and PF flags are set according to the result.
    builder.compute_and_store_zf(res);
    builder.compute_and_store_sf(res);
    builder.store_flag(Overflow, of);

    ControlFlow::NextInstruction
}

pub fn inc<B: Builder>(builder: &mut B, (dst,): (Operand,)) -> CF<B> {
    let val = builder.load_operand(dst);

    let one = builder.make_int_value(val.size(), 1);

    let res = builder.add(val, one);

    builder.store_operand(dst, res);

    let of = builder.sadd_overflow(val, one);

    // The CF flag is not affected. The OF, SF, ZF, AF, and PF flags are set according to the result.
    builder.compute_and_store_zf(res);
    builder.compute_and_store_sf(res);
    builder.store_flag(Overflow, of);

    ControlFlow::NextInstruction
}

pub fn neg<B: Builder>(builder: &mut B, (dst,): (Operand,)) -> CF<B> {
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
    builder.store_flag(Overflow, of);
    builder.store_flag(Carry, cf);

    ControlFlow::NextInstruction
}

pub fn mul<B: Builder>(builder: &mut B, (src,): (Operand,)) -> CF<B> {
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

    ControlFlow::NextInstruction
}

pub fn imul<B: Builder>(builder: &mut B, operands: Vec<Operand>) -> CF<B> {
    let (dst, src1, src2) = match *operands.as_slice() {
        [src] => match src.size() {
            IntType::I8 => (Operand::Register(AX), Operand::Register(AL), src),
            IntType::I16 => (Operand::RegisterPair(DX, AX), Operand::Register(AX), src),
            IntType::I32 => (Operand::RegisterPair(EDX, EAX), Operand::Register(EAX), src),
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

    builder.store_operand(dst, res_stored);

    ControlFlow::NextInstruction
}

pub fn div_idiv<B: Builder>(builder: &mut B, (mnemonic, src): (Mnemonic, Operand)) -> CF<B> {
    let double_size = src.size().double_sized();

    let (dividend_src, quo_dst, rem_dst) = match src.size() {
        IntType::I8 => (Operand::Register(AX), AL, AH),
        IntType::I16 => (Operand::RegisterPair(DX, AX), AX, DX),
        IntType::I32 => (Operand::RegisterPair(EDX, EAX), EAX, EDX),
        _ => unreachable!(),
    };

    let dividend = builder.load_operand(dividend_src);

    let divisor = builder.load_operand(src);
    let divisor = if mnemonic == Mnemonic::Div {
        builder.zext(divisor, double_size)
    } else {
        builder.sext(divisor, double_size)
    };

    let quotient = if mnemonic == Mnemonic::Div {
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

    ControlFlow::NextInstruction
}
