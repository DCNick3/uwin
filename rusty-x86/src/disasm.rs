use crate::types::{IntType, MemoryOperand, Operand, Register, SegmentRegister};
use iced_x86::{
    Code, ConditionCode, Formatter, Instruction, MemorySize, Mnemonic, NasmFormatter, OpKind,
    Register as IcedRegister,
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

fn get_opt_segment(iced_segment: IcedRegister) -> Option<SegmentRegister> {
    match iced_segment {
        IcedRegister::None => None,
        IcedRegister::CS => Some(SegmentRegister::CS),
        IcedRegister::DS => Some(SegmentRegister::DS),
        IcedRegister::ES => Some(SegmentRegister::ES),
        IcedRegister::FS => Some(SegmentRegister::FS),
        IcedRegister::GS => Some(SegmentRegister::GS),
        IcedRegister::SS => Some(SegmentRegister::SS),
        _ => unreachable!(),
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

        OpKind::Memory | OpKind::MemoryESEDI | OpKind::MemorySegESI => {
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

            let op = match op_kind {
                OpKind::Memory => MemoryOperand {
                    base: get_opt_register(instr.memory_base()),
                    displacement: instr.memory_displacement32() as i32 as i64,
                    scale: instr.memory_index_scale() as u8,
                    index: get_opt_register(instr.memory_index()),
                    size: memory_size,
                    segment: get_opt_segment(instr.segment_prefix()),
                },
                OpKind::MemoryESEDI => MemoryOperand {
                    base: Some(super::Register::EDI),
                    displacement: 0,
                    scale: 0,
                    index: None,
                    size: memory_size,
                    segment: Some(SegmentRegister::ES),
                },
                OpKind::MemorySegESI => MemoryOperand {
                    base: Some(super::Register::ESI),
                    displacement: 0,
                    scale: 0,
                    index: None,
                    size: memory_size,
                    segment: get_opt_segment(instr.segment_prefix()),
                },
                _ => unreachable!(),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NextEip(pub u32);

impl From<NextEip> for u32 {
    fn from(next_eip: NextEip) -> Self {
        next_eip.0
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

pub trait FromInstruction {
    fn from_instruction(instr: &Instruction) -> Self;
}

impl FromInstruction for Vec<Operand> {
    fn from_instruction(instr: &Instruction) -> Self {
        instr.get_operands()
    }
}
impl FromInstruction for () {
    fn from_instruction(_instr: &Instruction) -> Self {}
}

impl FromInstruction for (Code, Vec<Operand>) {
    fn from_instruction(instr: &Instruction) -> Self {
        (instr.code(), instr.get_operands())
    }
}

impl FromInstruction for (Operand,) {
    fn from_instruction(instr: &Instruction) -> Self {
        operands!([op0], instr);
        (op0,)
    }
}
impl FromInstruction for (Operand, Operand) {
    fn from_instruction(instr: &Instruction) -> Self {
        operands!([op0, op1], instr);
        (op0, op1)
    }
}
impl FromInstruction for (Operand, Operand, Operand) {
    fn from_instruction(instr: &Instruction) -> Self {
        operands!([op0, op1, op2], instr);
        (op0, op1, op2)
    }
}

impl FromInstruction for (Mnemonic,) {
    fn from_instruction(instr: &Instruction) -> Self {
        (instr.mnemonic(),)
    }
}
impl FromInstruction for (Mnemonic, Operand) {
    fn from_instruction(instr: &Instruction) -> Self {
        operands!([op0], instr);
        (instr.mnemonic(), op0)
    }
}
impl FromInstruction for (Mnemonic, Operand, Operand) {
    fn from_instruction(instr: &Instruction) -> Self {
        operands!([op0, op1], instr);
        (instr.mnemonic(), op0, op1)
    }
}
impl FromInstruction for (Mnemonic, Operand, Operand, Operand) {
    fn from_instruction(instr: &Instruction) -> Self {
        operands!([op0, op1, op2], instr);
        (instr.mnemonic(), op0, op1, op2)
    }
}

impl FromInstruction for (ConditionCode,) {
    fn from_instruction(instr: &Instruction) -> Self {
        operands!([], instr);
        (instr.condition_code(),)
    }
}
impl FromInstruction for (ConditionCode, Operand) {
    fn from_instruction(instr: &Instruction) -> Self {
        operands!([op0], instr);
        (instr.condition_code(), op0)
    }
}
impl FromInstruction for (ConditionCode, Operand, Operand) {
    fn from_instruction(instr: &Instruction) -> Self {
        operands!([op0, op1], instr);
        (instr.condition_code(), op0, op1)
    }
}
impl FromInstruction for (ConditionCode, Operand, Operand, Operand) {
    fn from_instruction(instr: &Instruction) -> Self {
        operands!([op0, op1, op2], instr);
        (instr.condition_code(), op0, op1, op2)
    }
}

impl FromInstruction for (NextEip,) {
    fn from_instruction(instr: &Instruction) -> Self {
        operands!([], instr);
        (NextEip(instr.next_ip32()),)
    }
}
impl FromInstruction for (NextEip, Operand) {
    fn from_instruction(instr: &Instruction) -> Self {
        operands!([op0], instr);
        (NextEip(instr.next_ip32()), op0)
    }
}
impl FromInstruction for (NextEip, Operand, Operand) {
    fn from_instruction(instr: &Instruction) -> Self {
        operands!([op0, op1], instr);
        (NextEip(instr.next_ip32()), op0, op1)
    }
}
impl FromInstruction for (NextEip, Operand, Operand, Operand) {
    fn from_instruction(instr: &Instruction) -> Self {
        operands!([op0, op1, op2], instr);
        (NextEip(instr.next_ip32()), op0, op1, op2)
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
