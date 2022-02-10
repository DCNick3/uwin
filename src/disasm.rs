use crate::types::{IntType, MemoryOperand, Operand, Register, SegmentRegister};
use iced_x86::{
    Formatter, Instruction, MemorySize, NasmFormatter, OpKind, Register as IcedRegister,
};
use std::fmt::Write;

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

fn get_opt_register(iced_register: IcedRegister) -> Option<Register> {
    match iced_register {
        IcedRegister::None => None,
        reg => Some(get_register(reg)),
    }
}

pub fn get_operand(instr: &Instruction, operand: u32) -> Operand {
    use crate::types::Operand::*;

    let op_kind = instr.op_kind(operand);

    match op_kind {
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

        OpKind::Memory | OpKind::MemoryESEDI => {
            let memory_size = match instr.memory_size() {
                MemorySize::UInt8 => Some(IntType::I8),
                MemorySize::UInt16 => Some(IntType::I16),
                MemorySize::UInt32 => Some(IntType::I32),
                MemorySize::UInt64 => Some(IntType::I64),

                MemorySize::Int8 => Some(IntType::I8),
                MemorySize::Int16 => Some(IntType::I16),
                MemorySize::Int32 | MemorySize::DwordOffset => Some(IntType::I32),
                MemorySize::Int64 => Some(IntType::I64),

                MemorySize::Unknown => None,

                s => panic!("Unsupported memory size: {:?}", s),
            };

            let op = if op_kind == OpKind::Memory {
                assert_eq!(instr.segment_prefix(), IcedRegister::None); // TODO: segments (they are not that interesting, honestly)

                MemoryOperand {
                    base: get_opt_register(instr.memory_base()),
                    displacement: instr.memory_displacement32() as i32 as i64,
                    scale: instr.memory_index_scale() as u8,
                    index: get_opt_register(instr.memory_index()),
                    size: memory_size,
                    segment: None,
                }
            } else {
                MemoryOperand {
                    base: Some(super::Register::EDI),
                    displacement: 0,
                    scale: 0,
                    index: None,
                    size: memory_size,
                    segment: Some(SegmentRegister::ES),
                }
            };
            Memory(op)
        }
        k => panic!("Unsupported operand kind: {:?}", k),
    }
}

#[macro_export]
macro_rules! operands_ty {
    // just discard the argument
    // used to construct the tuple (type) of $crate::types::Operand the same length as passed pattern
    ($x:tt) => {
        $crate::types::Operand
    };
}
#[macro_export]
macro_rules! operands {
    ([], $instr:expr) => {
        match *$crate::disasm::Operands::get_operands(($instr)).as_slice() {
            [] => (),
            _ => panic!("Instruction operand matching failed"),
        };
    };

    ([$($pattern:tt),*], $instr:expr) => {
        let ($($pattern),*,): ( $(operands_ty!($pattern)),*, ) = {
            match *$crate::disasm::Operands::get_operands(($instr)).as_slice() {
                [$($pattern),*] => ($($pattern),*,),
                _ => panic!("Instruction operand matching failed"),
            }
        };
    }
}

pub trait Operands {
    fn get_operands(&self) -> Vec<Operand>;
}

impl Operands for Instruction {
    fn get_operands(&self) -> Vec<Operand> {
        let mut res = Vec::new();
        res.reserve(self.op_count() as usize);
        for i in 0..self.op_count() {
            res.push(get_operand(self, i))
        }
        res
    }
}

pub fn disassemble(code: &[u8], base_addr: u64) -> String {
    let mut decoder = iced_x86::Decoder::with_ip(32, code, base_addr, 0);
    let mut formatter = NasmFormatter::new();
    let mut output = String::new();
    let mut instruction = Instruction::default();
    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);
        write!(output, "{:08X} ", instruction.ip()).unwrap();

        formatter.format(&instruction, &mut output);

        writeln!(output).unwrap();
    }
    output
}
