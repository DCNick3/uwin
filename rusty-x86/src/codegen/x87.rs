use super::prelude::*;

pub fn wait<B: Builder>(builder: &mut B, _: ()) -> CF<B> {
    ControlFlow::NextInstruction
}

pub fn fnstcw<B: Builder>(builder: &mut B, (dst,): (Operand,)) -> CF<B> {
    let value = builder.make_u16(0x037f);

    builder.store_operand(dst, value);

    ControlFlow::NextInstruction
}

pub fn fldcw<B: Builder>(_builder: &mut B, (_src,): (Operand,)) -> CF<B> {
    ControlFlow::NextInstruction
}

pub fn fnclex<B: Builder>(_builder: &mut B, (): ()) -> CF<B> {
    ControlFlow::NextInstruction
}
