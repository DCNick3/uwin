use super::prelude::*;

pub fn int_int3<B: Builder>(builder: &mut B, _: ()) -> CF<B> {
    // maybe we can handle Ints different than int3 differently?
    // TODO: give runtime info on WTF has happened
    builder.trap();

    ControlFlow::NextInstruction
}

pub fn ud2<B: Builder>(builder: &mut B, _: ()) -> CF<B> {
    // TODO: give runtime info on WTF has happened
    // probably want an intrinsic
    builder.trap();

    ControlFlow::NextInstruction
}
