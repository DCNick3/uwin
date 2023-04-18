use super::prelude::*;

pub fn cwd_cdq<B: Builder>(builder: &mut B, (mnemonic,): (Mnemonic,)) -> CF<B> {
    let (hi, lo) = match mnemonic {
        Mnemonic::Cwd => (DX, AX),
        Mnemonic::Cdq => (EDX, EAX),
        _ => unreachable!(),
    };
    let val = builder.load_register(lo);
    // TODO: not the best way to write the sign extension?
    let extended = builder.extract_msb(val);
    let extended = builder.bool_to_int(extended, val.size());
    let extended = builder.int_neg(extended);
    builder.store_register(hi, extended);

    ControlFlow::NextInstruction
}

pub fn cbw_cwde<B: Builder>(builder: &mut B, (mnemonic,): (Mnemonic,)) -> CF<B> {
    let (dst, src) = match mnemonic {
        Mnemonic::Cbw => (AX, AL),
        Mnemonic::Cwde => (EAX, AX),
        _ => unreachable!(),
    };
    let val = builder.load_register(src);
    let extended = builder.sext(val, dst.size());
    builder.store_register(dst, extended);

    ControlFlow::NextInstruction
}
