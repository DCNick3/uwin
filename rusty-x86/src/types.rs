use std::ffi::c_void;
use std::fmt::{Debug, Formatter};

use derive_more::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::Builder;

// the numbers correspond to register numbers in ModR/M encoding
#[derive(Debug, Clone, Copy, EnumIter, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FullSizeGeneralPurposeRegister {
    EAX = 0,
    EBX = 1,
    ECX = 2,
    EDX = 3,
    ESP = 4,
    EBP = 5,
    ESI = 6,
    EDI = 7,
}

impl TryFrom<Register> for FullSizeGeneralPurposeRegister {
    type Error = ();

    fn try_from(value: Register) -> Result<Self, Self::Error> {
        use FullSizeGeneralPurposeRegister::*;
        match value {
            Register::EAX => Ok(EAX),
            Register::EBX => Ok(EBX),
            Register::ECX => Ok(ECX),
            Register::EDX => Ok(EDX),
            Register::ESP => Ok(ESP),
            Register::EBP => Ok(EBP),
            Register::ESI => Ok(ESI),
            Register::EDI => Ok(EDI),
            _ => Err(()),
        }
    }
}

// TODO add more registers
// TODO add sub-registers meta-info (stuff like AX is the lower 16 bits of EAX)
#[derive(Debug, Clone, Copy)]
pub enum Register {
    EAX,
    EBX,
    ECX,
    EDX,
    ESP,
    EBP,
    ESI,
    EDI,

    AX,
    BX,
    CX,
    DX,
    SP,
    BP,
    SI,
    DI,

    AH,
    BH,
    CH,
    DH,

    AL,
    BL,
    CL,
    DL,
}

impl Register {
    pub fn size(self) -> IntType {
        use IntType::*;
        use Register::*;
        match self {
            EAX | EBX | ECX | EDX | ESP | EBP | ESI | EDI => I32,
            AX | BX | CX | DX | SP | BP | SI | DI => I16,
            AH | BH | CH | DH | AL | BL | CL | DL => I8,
        }
    }

    pub fn base_register(self) -> FullSizeGeneralPurposeRegister {
        use Register::*;
        match self {
            EAX | AX | AL | AH => FullSizeGeneralPurposeRegister::EAX,
            EBX | BX | BL | BH => FullSizeGeneralPurposeRegister::EBX,
            ECX | CX | CL | CH => FullSizeGeneralPurposeRegister::ECX,
            EDX | DX | DL | DH => FullSizeGeneralPurposeRegister::EDX,
            ESP | SP => FullSizeGeneralPurposeRegister::ESP,
            EBP | BP => FullSizeGeneralPurposeRegister::EBP,
            ESI | SI => FullSizeGeneralPurposeRegister::ESI,
            EDI | DI => FullSizeGeneralPurposeRegister::EDI,
        }
    }

    pub fn is_hi_reg(self) -> bool {
        use Register::*;
        matches!(self, AH | BH | CH | DH)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SegmentRegister {
    CS,
    DS,
    ES,
    FS,
    GS,
    SS,
}

#[derive(Debug, Display, Clone, Copy, EnumIter, PartialEq, Eq, Ord, PartialOrd)]
pub enum Flag {
    Carry = 0,
    Parity = 1,         // almost definitely can be ignored, not used much
    AuxiliaryCarry = 2, // definitely can be ignored, as it's almost never used in modern (non-DOS) code
    Zero = 3,
    Sign = 4,
    Overflow = 5,
    Direction = 6,
    Id = 7,
    // !!! Make sure not to go out of bounds of CpuContext::flags
}

#[repr(C)] // for interoperability with llvm-generated functions
#[derive(Eq, PartialEq, Clone, Default)]
pub struct CpuContext {
    // !!! If changing this struct - don't forget to update Types::new in llvm_backend.rs
    // also it would be best not to move fields around, as this breaks indices in build_ctx_*_gep
    pub gp_regs: [u32; 8],
    pub flags: [u8; 8],
}

impl std::fmt::Debug for CpuContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        struct FlagsDebug(CpuContext);
        impl std::fmt::Debug for FlagsDebug {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let mut s = f.debug_list();
                for flag in Flag::iter() {
                    if self.0.get_flag(flag) {
                        s.entry(&flag);
                    }
                }
                s.finish()
            }
        }

        let mut s = f.debug_struct("CpuContext");
        for gp in FullSizeGeneralPurposeRegister::iter() {
            s.field(format!("{:?}", gp).as_str(), &self.get_gp_reg(gp));
        }
        s.field("flags", &FlagsDebug { 0: self.clone() });
        s.finish()
    }
}

impl CpuContext {
    pub fn get_gp_reg(&self, reg: FullSizeGeneralPurposeRegister) -> u32 {
        self.gp_regs[reg as usize]
    }

    pub fn set_gp_reg(&mut self, reg: FullSizeGeneralPurposeRegister, val: u32) {
        self.gp_regs[reg as usize] = val
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        self.flags[flag as usize] != 0
    }

    pub fn set_flag(&mut self, flag: Flag, val: bool) {
        self.flags[flag as usize] = if val { 1 } else { 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntType {
    I8,
    I16,
    I32,
    I64,
}

impl IntType {
    pub fn double_sized(self) -> Self {
        use IntType::*;
        match self {
            I8 => I16,
            I16 => I32,
            I32 => I64,
            I64 => panic!("Can't created a double-sided type for I64"),
        }
    }

    pub fn bit_width(self) -> u8 {
        use IntType::*;
        match self {
            I8 => 8,
            I16 => 16,
            I32 => 32,
            I64 => 64,
        }
    }

    pub fn byte_width(self) -> u8 {
        self.bit_width() / 8
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryOperand {
    pub base: Option<Register>,
    pub displacement: i64,
    pub scale: u8,
    pub index: Option<Register>,
    pub size: Option<IntType>,
    pub segment: Option<SegmentRegister>,
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Register(Register),

    // pair of HI:LO registers
    RegisterPair(Register, Register),

    Immediate8(u8),
    Immediate16(u16),
    Immediate32(u32),
    Immediate64(u64),

    FarBranch(u16, u32),

    Memory(MemoryOperand),
}

impl Operand {
    pub fn size(self) -> IntType {
        match self {
            Operand::Register(reg) => reg.size(),
            Operand::RegisterPair(reg1, reg2) => {
                assert_eq!(reg1.size(), reg2.size());
                reg1.size().double_sized()
            }
            Operand::Immediate8(_) => IntType::I8,
            Operand::Immediate16(_) => IntType::I16,
            Operand::Immediate32(_) => IntType::I32,
            Operand::Immediate64(_) => IntType::I64,
            Operand::FarBranch(_, _) => todo!(),
            Operand::Memory(m) => m.size.unwrap(),
        }
    }

    pub fn as_imm32(self) -> u32 {
        match self {
            Operand::Immediate32(r) => r,
            _ => panic!("Attempt to use smth not being a imm32 as one"),
        }
    }
}

#[derive(Debug)]
pub enum ControlFlow<B: Builder> {
    NextInstruction, /* Just execute the next instruction, no need to touch EIP at all */
    DirectJump(u32 /* next EIP is known and stored */),
    IndirectJump(B::IntValue /* next EIP is dynamic and stored */),
    Return, /* return from a function. Value should be popped from the stack by the instruction implementation */
    Conditional(B::BoolValue, u32), /* if cond is true - jump to u32,  */
}

impl<B: Builder> ControlFlow<B> {
    pub fn can_reach_next_instruction(&self) -> bool {
        match self {
            ControlFlow::NextInstruction => true,
            ControlFlow::DirectJump(_) => false,
            ControlFlow::IndirectJump(_) => false,
            ControlFlow::Return => false,
            ControlFlow::Conditional(_, _) => true,
        }
    }

    pub fn outer_jump_ref(&self) -> Option<u32> {
        match self {
            ControlFlow::NextInstruction => None,
            ControlFlow::DirectJump(r) => Some(*r),
            ControlFlow::IndirectJump(_) => None, /* can't statically know the addr */
            ControlFlow::Return => None,
            ControlFlow::Conditional(_, r) => Some(*r),
        }
    }
}

// for some reason #[derive(Clone)] doesn't work (TODO: why?)
impl<B: Builder> Clone for ControlFlow<B> {
    fn clone(&self) -> Self {
        use ControlFlow::*;
        match self {
            NextInstruction => NextInstruction,
            DirectJump(r) => DirectJump(*r),
            IndirectJump(r) => IndirectJump(*r),
            Return => Return,
            Conditional(cond, r) => Conditional(*cond, *r),
        }
    }
}

pub type BbFunc = unsafe extern "C" fn(*mut CpuContext, *mut u8) -> c_void;

#[repr(u32)]
pub enum RuntimeError {
    Generic,
}
