use derive_more::Display;

// the numbers correspond to register numbers in ModR/M encoding
#[derive(Debug, Display, Clone, Copy)]
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
            _ => Err(())
        }
    }
}

// TODO add more registers
// TODO add subregisters metainfo (stuff like AX is the lower 16 bits of EAX)
#[derive(Debug, Display, Clone, Copy)]
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

#[derive(Debug)]
pub enum SegmentRegister {
    CS,
    DS,
    ES,
    FS,
    GS,
    SS
}

#[repr(C)] // for interoperability with llvm-generated functions
pub struct CpuContext {
    pub gp_regs: [u32; 8],
}

#[derive(Debug)]
pub enum IntType {
    I8,
    I16,
    I32,
    I64
}

#[derive(Debug)]
pub struct MemoryOperand {
    pub base: Option<Register>,
    pub displacement: i64,
    pub scale: u8,
    pub index: Option<Register>,
    pub size: IntType,
    pub segment: Option<SegmentRegister>,
}

#[derive(Debug)]
pub enum Operand {
    Register(Register),

    Immediate8(u8),
    Immediate16(u16),
    Immediate32(u32),
    Immediate64(u64),

    FarBranch(u16, u32),

    Memory(MemoryOperand),
}
