use super::prelude::*;

pub fn xor<B: Builder>(builder: &mut B, (dst, src): (Operand, Operand)) -> CF<B> {
    let lhs = builder.load_operand(dst);
    let rhs = builder.load_operand(src);

    let res = builder.int_xor(lhs, rhs);

    builder.store_operand(dst, res);

    // The OF and CF flags are cleared; the SF, ZF, and PF flags are set according to the result.
    // The state of the AF flag is undefined.
    // TODO: do we want to represent ub here? leaving as zero for now
    builder.compute_and_store_zf(res);
    builder.compute_and_store_sf(res);
    builder.store_flag(Carry, builder.make_false());
    builder.store_flag(Overflow, builder.make_false());

    ControlFlow::NextInstruction
}

pub fn not<B: Builder>(builder: &mut B, (dst,): (Operand,)) -> CF<B> {
    let val = builder.load_operand(dst);
    let val = builder.int_not(val);

    builder.store_operand(dst, val);

    ControlFlow::NextInstruction
}

pub fn and_test<B: Builder>(
    builder: &mut B,
    (mnemonic, dst, src): (Mnemonic, Operand, Operand),
) -> CF<B> {
    let lhs = builder.load_operand(dst);
    let rhs = builder.load_operand(src);

    let res = builder.int_and(lhs, rhs);

    if mnemonic == Mnemonic::And {
        builder.store_operand(dst, res);
    }
    // The OF and CF flags are cleared; the SF, ZF, and PF flags are set according to the result. The state of the AF flag is
    // undefined.
    builder.compute_and_store_zf(res);
    builder.compute_and_store_sf(res);
    builder.store_flag(Carry, builder.make_false());
    builder.store_flag(Overflow, builder.make_false());

    ControlFlow::NextInstruction
}

pub fn or<B: Builder>(builder: &mut B, (dst, src): (Operand, Operand)) -> CF<B> {
    let lhs = builder.load_operand(dst);
    let rhs = builder.load_operand(src);

    let res = builder.int_or(lhs, rhs);

    builder.store_operand(dst, res);

    // The OF and CF flags are cleared; the SF, ZF, and PF flags are set according to the result. The state of the AF flag is
    // undefined.
    builder.compute_and_store_zf(res);
    builder.compute_and_store_sf(res);
    builder.store_flag(Carry, builder.make_false());
    builder.store_flag(Overflow, builder.make_false());

    ControlFlow::NextInstruction
}
