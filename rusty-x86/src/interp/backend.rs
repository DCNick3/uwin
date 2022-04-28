use crate::backend::BoolValue;
use crate::types::{CpuContext, FullSizeGeneralPurposeRegister, SegmentRegister};
use crate::{
    Builder, ComparisonType, ControlFlow, Flag, IntType, IntValue as IntValueTrait, Register,
};

#[derive(Clone, Copy, PartialEq)]
pub enum IntValue {
    I8(u8),
    I16(u16),
    I32(u32),
    I64(u64),
}

impl IntValue {
    pub fn into_u16(self) -> Option<u16> {
        match self {
            IntValue::I8(v) => Some(v as _),
            IntValue::I16(v) => Some(v),
            IntValue::I32(_) => None,
            IntValue::I64(_) => None,
        }
    }
    pub fn into_u32(self) -> Option<u32> {
        match self {
            IntValue::I8(v) => Some(v as _),
            IntValue::I16(v) => Some(v as _),
            IntValue::I32(v) => Some(v),
            IntValue::I64(_) => None,
        }
    }
    pub fn into_u64(self) -> Option<u64> {
        match self {
            IntValue::I8(v) => Some(v as _),
            IntValue::I16(v) => Some(v as _),
            IntValue::I32(v) => Some(v as _),
            IntValue::I64(v) => Some(v),
        }
    }

    pub fn into_i16(self) -> Option<i16> {
        match self {
            IntValue::I8(v) => Some(v as i8 as _),
            IntValue::I16(v) => Some(v as _),
            IntValue::I32(_) => None,
            IntValue::I64(_) => None,
        }
    }
    pub fn into_i32(self) -> Option<i32> {
        match self {
            IntValue::I8(v) => Some(v as i8 as _),
            IntValue::I16(v) => Some(v as i16 as _),
            IntValue::I32(v) => Some(v as _),
            IntValue::I64(_) => None,
        }
    }
    pub fn into_i64(self) -> Option<i64> {
        match self {
            IntValue::I8(v) => Some(v as i8 as _),
            IntValue::I16(v) => Some(v as i16 as _),
            IntValue::I32(v) => Some(v as i32 as _),
            IntValue::I64(v) => Some(v as _),
        }
    }

    pub fn as_u8(self) -> Option<u8> {
        match self {
            IntValue::I8(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_u16(self) -> Option<u16> {
        match self {
            IntValue::I16(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_u32(self) -> Option<u32> {
        match self {
            IntValue::I32(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_u64(self) -> Option<u64> {
        match self {
            IntValue::I64(v) => Some(v),
            _ => None,
        }
    }
}

impl IntValueTrait for IntValue {
    fn size(&self) -> IntType {
        match self {
            IntValue::I8(_) => IntType::I8,
            IntValue::I16(_) => IntType::I16,
            IntValue::I32(_) => IntType::I32,
            IntValue::I64(_) => IntType::I64,
        }
    }
}
impl BoolValue for bool {}

pub struct InterpBuilder<'a> {
    cpu_context: &'a mut CpuContext,
    memory_base: *mut u8,
    call_basic_block: fn(&mut CpuContext, *mut u8, u32) -> u32,
    call_thunk: fn(&mut CpuContext, *mut u8, u32) -> u32,
}

impl<'a> Builder for InterpBuilder<'a> {
    type IntValue = IntValue;
    type BoolValue = bool;

    fn make_int_value(&self, ty: IntType, value: u64) -> Self::IntValue {
        match ty {
            IntType::I8 => IntValue::I8(value as u8),
            IntType::I16 => IntValue::I16(value as u16),
            IntType::I32 => IntValue::I32(value as u32),
            IntType::I64 => IntValue::I64(value as u64),
        }
    }

    fn make_true(&self) -> Self::BoolValue {
        true
    }

    fn make_false(&self) -> Self::BoolValue {
        false
    }

    fn load_segment_base(&mut self, segment: SegmentRegister) -> Self::IntValue {
        use SegmentRegister::*;
        match segment {
            CS | DS | ES | SS => self.make_u32(0),
            FS => self.make_u32(self.cpu_context.fs_base),
            GS => self.make_u32(self.cpu_context.gs_base),
        }
    }

    fn load_register(&mut self, register: Register) -> Self::IntValue {
        let base = register.base_register();
        assert!(!register.is_hi_reg());
        let value = self.cpu_context.gp_regs[base as usize];
        self.make_int_value(register.size(), value as u64)
    }

    fn store_register(&mut self, register: Register, value: Self::IntValue) {
        let base = register.base_register();
        assert!(!register.is_hi_reg());
        let old_value = self.cpu_context.gp_regs[base as usize];
        let mask = ((1u64 << value.size().bit_width()) - 1) as u32;
        let value = (old_value & (!mask)) | (value.into_u32().unwrap() & mask);
        self.cpu_context.gp_regs[base as usize] = value;
    }

    fn load_flag(&mut self, flag: Flag) -> Self::BoolValue {
        self.cpu_context.flags[flag as usize] != 0
    }

    fn store_flag(&mut self, flag: Flag, value: Self::BoolValue) {
        self.cpu_context.flags[flag as usize] = if value { 1 } else { 0 };
    }

    fn load_memory(&mut self, size: IntType, address: Self::IntValue) -> Self::IntValue {
        let address = address.as_u32().unwrap();
        // SAFETY: memory_base points to a chunk of address space with 2 GiBs reserved
        let address = unsafe { self.memory_base.add(address as usize) };

        // SAFETY: well, we are reading byte arrays, this should be pretty safe
        // the memory __may__ be unmapped there, but the segfault should be pretty predictable
        let value = match size {
            IntType::I8 => unsafe {
                u8::from_le_bytes(*std::mem::transmute::<_, *mut [u8; 1]>(address)) as u64
            },
            IntType::I16 => unsafe {
                u16::from_le_bytes(*std::mem::transmute::<_, *mut [u8; 2]>(address)) as u64
            },
            IntType::I32 => unsafe {
                u32::from_le_bytes(*std::mem::transmute::<_, *mut [u8; 4]>(address)) as u64
            },
            IntType::I64 => unsafe {
                u64::from_le_bytes(*std::mem::transmute::<_, *mut [u8; 8]>(address)) as u64
            },
        };
        self.make_int_value(size, value)
    }

    fn store_memory(&mut self, address: Self::IntValue, value: Self::IntValue) {
        let address = address.as_u32().unwrap();
        // SAFETY: memory_base points to a chunk of address space with 2 GiBs reserved
        let address = unsafe { self.memory_base.add(address as usize) };

        // SAFETY: pretty much the same as load_memory
        match value {
            IntValue::I8(v) => unsafe {
                *std::mem::transmute::<_, *mut [u8; 1]>(address) = v.to_le_bytes()
            },
            IntValue::I16(v) => unsafe {
                *std::mem::transmute::<_, *mut [u8; 2]>(address) = v.to_le_bytes()
            },
            IntValue::I32(v) => unsafe {
                *std::mem::transmute::<_, *mut [u8; 4]>(address) = v.to_le_bytes()
            },
            IntValue::I64(v) => unsafe {
                *std::mem::transmute::<_, *mut [u8; 8]>(address) = v.to_le_bytes()
            },
        };
    }

    fn add(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => I8(lhs.wrapping_add(rhs)),
            (I16(lhs), I16(rhs)) => I16(lhs.wrapping_add(rhs)),
            (I32(lhs), I32(rhs)) => I32(lhs.wrapping_add(rhs)),
            (I64(lhs), I64(rhs)) => I64(lhs.wrapping_add(rhs)),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn int_neg(&mut self, val: Self::IntValue) -> Self::IntValue {
        use IntValue::*;
        match val {
            I8(v) => I8(-(v as i8) as _),
            I16(v) => I16(-(v as i16) as _),
            I32(v) => I32(-(v as i32) as _),
            I64(v) => I64(-(v as i64) as _),
        }
    }

    fn sub(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => I8(lhs.wrapping_sub(rhs)),
            (I16(lhs), I16(rhs)) => I16(lhs.wrapping_sub(rhs)),
            (I32(lhs), I32(rhs)) => I32(lhs.wrapping_sub(rhs)),
            (I64(lhs), I64(rhs)) => I64(lhs.wrapping_sub(rhs)),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn mul(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => I8(lhs.wrapping_mul(rhs)),
            (I16(lhs), I16(rhs)) => I16(lhs.wrapping_mul(rhs)),
            (I32(lhs), I32(rhs)) => I32(lhs.wrapping_mul(rhs)),
            (I64(lhs), I64(rhs)) => I64(lhs.wrapping_mul(rhs)),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn int_not(&mut self, val: Self::IntValue) -> Self::IntValue {
        use IntValue::*;
        match val {
            I8(v) => I8(!v),
            I16(v) => I16(!v),
            I32(v) => I32(!v),
            I64(v) => I64(!v),
        }
    }

    fn int_or(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => I8(lhs | rhs),
            (I16(lhs), I16(rhs)) => I16(lhs | rhs),
            (I32(lhs), I32(rhs)) => I32(lhs | rhs),
            (I64(lhs), I64(rhs)) => I64(lhs | rhs),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn int_and(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => I8(lhs & rhs),
            (I16(lhs), I16(rhs)) => I16(lhs & rhs),
            (I32(lhs), I32(rhs)) => I32(lhs & rhs),
            (I64(lhs), I64(rhs)) => I64(lhs & rhs),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn int_xor(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => I8(lhs ^ rhs),
            (I16(lhs), I16(rhs)) => I16(lhs ^ rhs),
            (I32(lhs), I32(rhs)) => I32(lhs ^ rhs),
            (I64(lhs), I64(rhs)) => I64(lhs ^ rhs),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn shl(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        assert!(rhs.into_u64().unwrap() < lhs.size().bit_width() as u64);
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => I8(lhs << rhs),
            (I16(lhs), I16(rhs)) => I16(lhs << rhs),
            (I32(lhs), I32(rhs)) => I32(lhs << rhs),
            (I64(lhs), I64(rhs)) => I64(lhs << rhs),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn lshr(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        assert!(rhs.into_u64().unwrap() < lhs.size().bit_width() as u64);
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => I8(lhs >> rhs),
            (I16(lhs), I16(rhs)) => I16(lhs >> rhs),
            (I32(lhs), I32(rhs)) => I32(lhs >> rhs),
            (I64(lhs), I64(rhs)) => I64(lhs >> rhs),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn ashr(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        assert!(rhs.into_u64().unwrap() < lhs.size().bit_width() as u64);
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => I8((lhs as i8 >> rhs) as _),
            (I16(lhs), I16(rhs)) => I16((lhs as i16 >> rhs) as _),
            (I32(lhs), I32(rhs)) => I32((lhs as i32 >> rhs) as _),
            (I64(lhs), I64(rhs)) => I64((lhs as i64 >> rhs) as _),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn udiv(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => I8(lhs / rhs),
            (I16(lhs), I16(rhs)) => I16(lhs / rhs),
            (I32(lhs), I32(rhs)) => I32(lhs / rhs),
            (I64(lhs), I64(rhs)) => I64(lhs / rhs),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn sdiv(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => I8((lhs as i8 / rhs as i8) as _),
            (I16(lhs), I16(rhs)) => I16((lhs as i16 / rhs as i16) as _),
            (I32(lhs), I32(rhs)) => I32((lhs as i32 / rhs as i32) as _),
            (I64(lhs), I64(rhs)) => I64((lhs as i64 / rhs as i64) as _),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn extract_bit(&mut self, val: Self::IntValue, bit: Self::IntValue) -> Self::BoolValue {
        assert!(bit.into_u64().unwrap() < bit.size().bit_width() as u64);
        let bit = bit.into_u64().unwrap() as u8;
        match val {
            IntValue::I8(v) => ((v >> bit) & 1) != 0,
            IntValue::I16(v) => ((v >> bit) & 1) != 0,
            IntValue::I32(v) => ((v >> bit) & 1) != 0,
            IntValue::I64(v) => ((v >> bit) & 1) != 0,
        }
    }

    fn bool_not(&mut self, val: Self::BoolValue) -> Self::BoolValue {
        !val
    }

    fn bool_or(&mut self, lhs: Self::BoolValue, rhs: Self::BoolValue) -> Self::BoolValue {
        lhs || rhs
    }

    fn bool_and(&mut self, lhs: Self::BoolValue, rhs: Self::BoolValue) -> Self::BoolValue {
        lhs && rhs
    }

    fn bool_xor(&mut self, lhs: Self::BoolValue, rhs: Self::BoolValue) -> Self::BoolValue {
        lhs ^ rhs
    }

    fn uadd_overflow(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::BoolValue {
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => lhs.checked_add(rhs).is_none(),
            (I16(lhs), I16(rhs)) => lhs.checked_add(rhs).is_none(),
            (I32(lhs), I32(rhs)) => lhs.checked_add(rhs).is_none(),
            (I64(lhs), I64(rhs)) => lhs.checked_add(rhs).is_none(),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn sadd_overflow(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::BoolValue {
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => (lhs as i8).checked_add(rhs as i8).is_none(),
            (I16(lhs), I16(rhs)) => (lhs as i16).checked_add(rhs as i16).is_none(),
            (I32(lhs), I32(rhs)) => (lhs as i32).checked_add(rhs as i32).is_none(),
            (I64(lhs), I64(rhs)) => (lhs as i64).checked_add(rhs as i64).is_none(),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn usub_overflow(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::BoolValue {
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => lhs.checked_sub(rhs).is_none(),
            (I16(lhs), I16(rhs)) => lhs.checked_sub(rhs).is_none(),
            (I32(lhs), I32(rhs)) => lhs.checked_sub(rhs).is_none(),
            (I64(lhs), I64(rhs)) => lhs.checked_sub(rhs).is_none(),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn ssub_overflow(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::BoolValue {
        use IntValue::*;
        match (lhs, rhs) {
            (I8(lhs), I8(rhs)) => (lhs as i8).checked_sub(rhs as i8).is_none(),
            (I16(lhs), I16(rhs)) => (lhs as i16).checked_sub(rhs as i16).is_none(),
            (I32(lhs), I32(rhs)) => (lhs as i32).checked_sub(rhs as i32).is_none(),
            (I64(lhs), I64(rhs)) => (lhs as i64).checked_sub(rhs as i64).is_none(),
            _ => panic!("Operation on incompatible types"),
        }
    }

    fn zext(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue {
        use IntValue::*;
        match to {
            IntType::I8 => I8(val.as_u8().unwrap()),
            IntType::I16 => I16(val.into_u16().unwrap()),
            IntType::I32 => I32(val.into_u32().unwrap()),
            IntType::I64 => I64(val.into_u64().unwrap()),
        }
    }

    fn sext(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue {
        use IntValue::*;
        match to {
            IntType::I8 => I8(val.as_u8().unwrap() as _),
            IntType::I16 => I16(val.into_i16().unwrap() as _),
            IntType::I32 => I32(val.into_i32().unwrap() as _),
            IntType::I64 => I64(val.into_i64().unwrap() as _),
        }
    }

    fn trunc(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue {
        use IntValue::*;
        let v = val.into_u64().unwrap();
        match to {
            IntType::I8 => I8(v as u8),
            IntType::I16 => I16(v as u16),
            IntType::I32 => I32(v as u32),
            IntType::I64 => I64(v as u64),
        }
    }

    fn icmp(
        &mut self,
        cmp: ComparisonType,
        lhs: Self::IntValue,
        rhs: Self::IntValue,
    ) -> Self::BoolValue {
        assert_eq!(lhs.size(), rhs.size());
        use ComparisonType::*;
        match cmp {
            Equal => lhs == rhs,
            NotEqual => lhs != rhs,
            UnsignedGreater => !self.icmp(UnsignedLessOrEqual, rhs, lhs),
            UnsignedGreaterOrEqual => !self.icmp(UnsignedLess, rhs, lhs),
            UnsignedLess => lhs.into_u64().unwrap() < rhs.into_u64().unwrap(),
            UnsignedLessOrEqual => lhs.into_u64().unwrap() <= rhs.into_u64().unwrap(),

            SignedGreater => !self.icmp(SignedLessOrEqual, rhs, lhs),
            SignedGreaterOrEqual => !self.icmp(SignedLess, rhs, lhs),
            SignedLess => lhs.into_i64().unwrap() < rhs.into_i64().unwrap(),
            SignedLessOrEqual => lhs.into_i64().unwrap() <= rhs.into_i64().unwrap(),
        }
    }

    fn direct_call(&mut self, target: u32) -> ControlFlow<Self::IntValue, Self::BoolValue> {
        let res = (self.call_basic_block)(self.cpu_context, self.memory_base, target);
        ControlFlow::CallCheck(IntValue::I32(res))
    }

    fn indirect_call(
        &mut self,
        target: Self::IntValue,
    ) -> ControlFlow<Self::IntValue, Self::BoolValue> {
        self.direct_call(target.as_u32().unwrap())
    }

    fn thunk_jump(&mut self, target: u32) -> ControlFlow<Self::IntValue, Self::BoolValue> {
        let res = (self.call_thunk)(self.cpu_context, self.memory_base, target);
        ControlFlow::Return(IntValue::I32(res))
    }

    fn select(
        &mut self,
        cond: Self::BoolValue,
        iftrue: Self::IntValue,
        iffalse: Self::IntValue,
    ) -> Self::IntValue {
        if cond {
            iftrue
        } else {
            iffalse
        }
    }

    fn ifelse<T, F>(&mut self, cond: Self::BoolValue, iftrue: T, iffalse: F)
    where
        T: FnOnce(&mut Self),
        F: FnOnce(&mut Self),
        Self: Sized,
    {
        if cond {
            iftrue(self)
        } else {
            iffalse(self)
        }
    }

    fn trap(&mut self) {
        panic!("Interpreter trapped!")
    }

    fn ctpop(&mut self, value: Self::IntValue) -> Self::IntValue {
        todo!()
    }

    fn ctlz(&mut self, value: Self::IntValue) -> Self::IntValue {
        todo!()
    }

    fn cttz(&mut self, value: Self::IntValue) -> Self::IntValue {
        todo!()
    }

    fn repeat_until<B>(&mut self, body: B)
    where
        B: Fn(&mut Self) -> Self::BoolValue,
        Self: Sized,
    {
        while body(self) {}
    }
}
