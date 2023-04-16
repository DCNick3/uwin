use inkwell::intrinsics::Intrinsic;

pub struct Intrinsics {
    pub sadd_with_overflow: Intrinsic,
    pub uadd_with_overflow: Intrinsic,
    pub ssub_with_overflow: Intrinsic,
    pub usub_with_overflow: Intrinsic,
    pub trap: Intrinsic,
    pub cttz: Intrinsic,
    pub ctlz: Intrinsic,
    pub ctpop: Intrinsic,
}

impl Intrinsics {
    pub fn new() -> Self {
        Self {
            sadd_with_overflow: Intrinsic::find("llvm.sadd.with.overflow").unwrap(),
            uadd_with_overflow: Intrinsic::find("llvm.uadd.with.overflow").unwrap(),
            ssub_with_overflow: Intrinsic::find("llvm.ssub.with.overflow").unwrap(),
            usub_with_overflow: Intrinsic::find("llvm.usub.with.overflow").unwrap(),
            trap: Intrinsic::find("llvm.trap").unwrap(),
            cttz: Intrinsic::find("llvm.cttz").unwrap(),
            ctlz: Intrinsic::find("llvm.ctlz").unwrap(),
            ctpop: Intrinsic::find("llvm.ctpop").unwrap(),
        }
    }
}

impl Default for Intrinsics {
    fn default() -> Self {
        Intrinsics::new()
    }
}
