use crate::unwind_token::UnwindReason;
use core_mem::ctx::MemoryCtx;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Register {
    EAX,
    EBX,
    ECX,
    EDX,
    ESP,
    EBP,
    ESI,
    EDI,
}

// this type abstracts away the use of rusty-x86 context and reduces cohesion :shrug:
pub trait X86Context {
    fn get_register(&self, register: Register) -> u32;
    fn set_register(&mut self, register: Register, value: u32);
    // TODO: flags

    #[inline]
    fn get_eax(&self) -> u32 {
        self.get_register(Register::EAX)
    }
    #[inline]
    fn get_ebx(&self) -> u32 {
        self.get_register(Register::EBX)
    }
    #[inline]
    fn get_ecx(&self) -> u32 {
        self.get_register(Register::ECX)
    }
    #[inline]
    fn get_edx(&self) -> u32 {
        self.get_register(Register::EDX)
    }
    #[inline]
    fn get_esp(&self) -> u32 {
        self.get_register(Register::ESP)
    }
    #[inline]
    fn get_ebp(&self) -> u32 {
        self.get_register(Register::EBP)
    }
    #[inline]
    fn get_esi(&self) -> u32 {
        self.get_register(Register::ESI)
    }
    #[inline]
    fn get_edi(&self) -> u32 {
        self.get_register(Register::EDI)
    }

    #[inline]
    fn set_eax(&mut self, value: u32) {
        self.set_register(Register::EAX, value);
    }
    #[inline]
    fn set_ebx(&mut self, value: u32) {
        self.set_register(Register::EBX, value);
    }
    #[inline]
    fn set_ecx(&mut self, value: u32) {
        self.set_register(Register::ECX, value);
    }
    #[inline]
    fn set_edx(&mut self, value: u32) {
        self.set_register(Register::EDX, value);
    }
    #[inline]
    fn set_esp(&mut self, value: u32) {
        self.set_register(Register::ESP, value);
    }
    #[inline]
    fn set_ebp(&mut self, value: u32) {
        self.set_register(Register::EBP, value);
    }
    #[inline]
    fn set_esi(&mut self, value: u32) {
        self.set_register(Register::ESI, value);
    }
    #[inline]
    fn set_edi(&mut self, value: u32) {
        self.set_register(Register::EDI, value);
    }
}

#[cfg(feature = "rusty-x86")]
impl From<Register> for rusty_x86::types::FullSizeGeneralPurposeRegister {
    #[inline]
    fn from(reg: Register) -> Self {
        use rusty_x86::types::FullSizeGeneralPurposeRegister::*;
        match reg {
            Register::EAX => EAX,
            Register::EBX => EBX,
            Register::ECX => ECX,
            Register::EDX => EDX,
            Register::ESP => ESP,
            Register::EBP => EBP,
            Register::ESI => ESI,
            Register::EDI => EDI,
        }
    }
}

#[cfg(feature = "rusty-x86")]
impl X86Context for rusty_x86::types::CpuContext {
    #[inline]
    fn get_register(&self, register: Register) -> u32 {
        self.get_gp_reg(register.into())
    }

    #[inline]
    fn set_register(&mut self, register: Register, value: u32) {
        self.set_gp_reg(register.into(), value)
    }
}

pub trait Context {
    type CpuContext: X86Context;
    type MemoryContext: MemoryCtx;

    fn cpu_context(&self) -> &Self::CpuContext;
    fn cpu_context_mut(&mut self) -> &mut Self::CpuContext;

    fn get_unwind_reason(&self) -> Option<UnwindReason>;
    fn set_unwind_reason(&mut self, reason: Option<UnwindReason>);

    fn execute_recompiled_code(&mut self, memory: Self::MemoryContext, eip: u32) -> u32;
}

pub trait Executor {
    type Context: Context<CpuContext = Self::CpuContext>;
    type CpuContext: X86Context;
    type MemoryContext: MemoryCtx;

    fn execute_recompiled_code(
        &self,
        context: &mut Self::Context,
        memory: Self::MemoryContext,
        eip: u32,
    ) -> u32;
}
