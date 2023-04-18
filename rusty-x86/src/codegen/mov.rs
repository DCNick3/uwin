use super::prelude::*;

pub fn mov<B: Builder>(builder: &mut B, (dst, src): (Operand, Operand)) -> CF<B> {
    let val = builder.load_operand(src);
    builder.store_operand(dst, val);

    ControlFlow::NextInstruction
}

pub fn movzx<B: Builder>(builder: &mut B, (dst, src): (Operand, Operand)) -> CF<B> {
    let val = builder.load_operand(src);
    let val = builder.zext(val, dst.size());
    builder.store_operand(dst, val);

    ControlFlow::NextInstruction
}

pub fn movsx<B: Builder>(builder: &mut B, (dst, src): (Operand, Operand)) -> CF<B> {
    let val = builder.load_operand(src);
    let val = builder.sext(val, dst.size());
    builder.store_operand(dst, val);

    ControlFlow::NextInstruction
}

pub fn xchg<B: Builder>(builder: &mut B, (op1, op2): (Operand, Operand)) -> CF<B> {
    let val1 = builder.load_operand(op1);
    let val2 = builder.load_operand(op2);

    builder.store_operand(op1, val2);
    builder.store_operand(op2, val1);

    ControlFlow::NextInstruction
}
