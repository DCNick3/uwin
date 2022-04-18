use core_mem::ptr::PtrRepr;

#[derive(Debug, Clone)]
pub enum UnwindReason {
    ExitProcess(u32),
    ExitThread(u32),
}

pub struct UnwindToken {
    return_address: PtrRepr,
    unwind_reason: Option<UnwindReason>,
}

impl UnwindToken {
    pub fn new(return_address: PtrRepr) -> Self {
        Self {
            return_address,
            unwind_reason: None,
        }
    }

    pub fn unwind(&mut self, reason: UnwindReason) {
        self.unwind_reason = Some(reason);
        self.return_address = 0;
    }

    pub fn return_addr(&self) -> PtrRepr {
        self.return_address
    }

    pub fn unwind_reason(&self) -> Option<UnwindReason> {
        self.unwind_reason.clone()
    }
}
