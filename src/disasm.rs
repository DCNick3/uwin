
use iced_x86::{Decoder, Instruction, OpKind, Register as IcedRegister};
use crate::types::{Operand, Register};

fn get_register(iced_register: IcedRegister) -> Register {
    use Register::*;
    match iced_register {
        IcedRegister::None => panic!("Attempt to access a None register"),

        IcedRegister::AL => AL,
        IcedRegister::CL => CL,
        IcedRegister::DL => DL,
        IcedRegister::BL => BL,

        IcedRegister::AH => AH,
        IcedRegister::CH => CH,
        IcedRegister::DH => DH,
        IcedRegister::BH => BH,


        IcedRegister::AX => AX,
        IcedRegister::CX => CX,
        IcedRegister::DX => DX,
        IcedRegister::BX => BX,

        IcedRegister::SP => SP,
        IcedRegister::BP => BP,
        IcedRegister::SI => SI,
        IcedRegister::DI => DI,


        IcedRegister::EAX => EAX,
        IcedRegister::ECX => ECX,
        IcedRegister::EDX => EDX,
        IcedRegister::EBX => EBX,

        IcedRegister::ESP => ESP,
        IcedRegister::EBP => EBP,
        IcedRegister::ESI => ESI,
        IcedRegister::EDI => EDI,

        // accessing EIP is TODO (it's kinda special, you know)
        //IcedRegister::EIP => {}

        _ => panic!("Unsupported register: {:?}", iced_register),
    }
}

pub fn get_operand(instr: &Instruction, operand: u32) -> Operand {
    use crate::types::Operand::*;

    match instr.op_kind(operand) {
        OpKind::Register => Register(get_register(instr.op_register(operand))),

        OpKind::NearBranch16 => panic!("unsupported branch address size (16)"),
        OpKind::NearBranch32 => Immediate32(instr.near_branch32()),
        OpKind::NearBranch64 => panic!("unsupported branch address size (64)"),

        OpKind::FarBranch16 => panic!("unsupported far branch address size (16)"),
        OpKind::FarBranch32 => FarBranch(instr.far_branch_selector(), instr.far_branch32()),

        OpKind::Immediate8 => Immediate8(instr.immediate8()),
        OpKind::Immediate8_2nd => Immediate8(instr.immediate8_2nd()),
        OpKind::Immediate16 => Immediate16(instr.immediate16()),
        OpKind::Immediate32 => Immediate32(instr.immediate32()),
        OpKind::Immediate64 => Immediate64(instr.immediate64()),

        OpKind::Immediate8to16 => Immediate16(instr.immediate8to16() as u16),
        OpKind::Immediate8to32 => Immediate32(instr.immediate8to32() as u32),
        OpKind::Immediate8to64 => Immediate64(instr.immediate8to64() as u64),
        OpKind::Immediate32to64 => Immediate64(instr.immediate32to64() as u64),

        OpKind::Memory => todo!(),
        k => panic!("Unsupported operand kind: {:?}", k)
    }
}