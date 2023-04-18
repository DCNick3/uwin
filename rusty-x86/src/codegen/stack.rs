use super::prelude::*;

pub fn push<B: Builder>(builder: &mut B, (src,): (Operand,)) -> CF<B> {
    let val = builder.load_operand(src);

    builder.push(val);

    ControlFlow::NextInstruction
}

pub fn pop<B: Builder>(builder: &mut B, (dst,): (Operand,)) -> CF<B> {
    let val = builder.pop(dst.size());

    builder.store_operand(dst, val);

    ControlFlow::NextInstruction
}

pub fn leave<B: Builder>(builder: &mut B, _: ()) -> CF<B> {
    let old_ebp = builder.load_register(EBP);
    builder.store_register(ESP, old_ebp);

    let new_ebp = builder.pop(IntType::I32);

    builder.store_register(EBP, new_ebp);

    ControlFlow::NextInstruction
}
