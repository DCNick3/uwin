use super::prelude::*;

pub fn mov<B: Builder>(builder: &mut B, dst: Operand, src: Operand) {
    let val = builder.load_operand(src);
    builder.store_operand(dst, val);
}

pub fn movzx<B: Builder>(builder: &mut B, dst: Operand, src: Operand) {
    let val = builder.load_operand(src);
    let val = builder.zext(val, dst.size());
    builder.store_operand(dst, val);
}

pub fn movsx<B: Builder>(builder: &mut B, dst: Operand, src: Operand) {
    let val = builder.load_operand(src);
    let val = builder.sext(val, dst.size());
    builder.store_operand(dst, val);
}
