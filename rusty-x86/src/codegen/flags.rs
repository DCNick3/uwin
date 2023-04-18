use super::prelude::*;

pub fn pushfd<B: Builder>(builder: &mut B, _: ()) -> CF<B> {
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

    ControlFlow::NextInstruction
}

pub fn popfd<B: Builder>(builder: &mut B, _: ()) -> CF<B> {
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

    ControlFlow::NextInstruction
}

pub fn stc<B: Builder>(builder: &mut B, _: ()) -> CF<B> {
    builder.store_flag(Carry, builder.make_true());

    ControlFlow::NextInstruction
}

pub fn clc<B: Builder>(builder: &mut B, _: ()) -> CF<B> {
    builder.store_flag(Carry, builder.make_false());

    ControlFlow::NextInstruction
}

pub fn std<B: Builder>(builder: &mut B, _: ()) -> CF<B> {
    builder.store_flag(Direction, builder.make_true());

    ControlFlow::NextInstruction
}

pub fn cld<B: Builder>(builder: &mut B, _: ()) -> CF<B> {
    builder.store_flag(Direction, builder.make_false());

    ControlFlow::NextInstruction
}
