use super::prelude::*;
use crate::disasm::NextEip;

pub fn ret<B: Builder>(builder: &mut B, (code, operands): (Code, Vec<Operand>)) -> CF<B> {
    match code {
        Code::Retnd => {
            assert_eq!(operands.len(), 0);

            let addr = builder.pop(IntType::I32);
            ControlFlow::Return(addr)
        }
        // TODO: TEST!!!
        Code::Retnd_imm16 => {
            let size = match *operands.as_slice() {
                [op] => op,
                _ => unreachable!(),
            };

            let addr = builder.pop(IntType::I32);

            let size_bytes = builder.load_operand(size);
            let size_bytes = builder.zext(size_bytes, IntType::I32);

            let esp = builder.load_register(ESP);
            let esp = builder.add(esp, size_bytes);
            builder.store_register(ESP, esp);

            ControlFlow::Return(addr)
        }
        // don't support weirdly-sized rets (like 'w' and 'q' versions) and far rets
        _ => unimplemented!("Unsupported ret instruction kind: {:?}", code),
    }
}

pub fn jmp<B: Builder>(builder: &mut B, (target,): (Operand,)) -> CF<B> {
    match target {
        Operand::Immediate8(_) | Operand::Immediate16(_) | Operand::Immediate64(_) => {
            panic!("Jump to unsupported immediate size")
        }
        Operand::Immediate32(target) => ControlFlow::DirectJump(target),
        Operand::FarBranch(segment, offset) => {
            if segment != 0x7775
            // magic value encoded as "uw" in little endian (for uwin)
            // recompiler uses it when it generates stubs
            {
                unimplemented!("Far jumps that are not targeting uwin magic segment")
            }
            builder.thunk_jump(offset)
        }
        target => {
            let target = builder.load_operand(target);
            ControlFlow::IndirectJump(target)
        }
    }
}

pub fn call<B: Builder>(builder: &mut B, (next_eip, target): (NextEip, Operand)) -> CF<B> {
    builder.push(builder.make_u32(next_eip.into()));

    match target {
        Operand::Immediate8(_) | Operand::Immediate16(_) | Operand::Immediate64(_) => {
            panic!("Call to unsupported immediate size")
        }
        Operand::Immediate32(target) => builder.direct_call(target),
        target => {
            let target = builder.load_operand(target);
            builder.indirect_call(target)
        }
    }
}
