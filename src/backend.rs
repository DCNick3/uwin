use crate::types::{Flag, IntType, MemoryOperand, Operand, Register};

pub trait IntValue: Clone + Copy {
    fn size(&self) -> IntType;
}

pub trait BoolValue: Clone + Copy {}

pub enum ComparisonType {
    Equal,
    NotEqual,
    UnsignedGreater,
    UnsignedGreaterOrEqual,
    UnsignedLess,
    UnsignedLessOrEqual,
    SignedGreater,
    SignedGreaterOrEqual,
    SignedLess,
    SignedLessOrEqual,
}

pub trait Builder {
    type IntValue: IntValue;
    type BoolValue: BoolValue;

    fn make_int_value(&self, ty: IntType, value: u64, sign_extend: bool) -> Self::IntValue;

    fn make_u8(&self, value: u8) -> Self::IntValue {
        self.make_int_value(IntType::I8, value as u64, false)
    }
    fn make_u16(&self, value: u16) -> Self::IntValue {
        self.make_int_value(IntType::I16, value as u64, false)
    }
    fn make_u32(&self, value: u32) -> Self::IntValue {
        self.make_int_value(IntType::I32, value as u64, false)
    }
    fn make_u64(&self, value: u64) -> Self::IntValue {
        self.make_int_value(IntType::I64, value as u64, false)
    }
    fn make_i8(&self, value: i8) -> Self::IntValue {
        self.make_int_value(IntType::I8, value as u64, true)
    }
    fn make_i16(&self, value: i16) -> Self::IntValue {
        self.make_int_value(IntType::I16, value as u64, false)
    }
    fn make_i32(&self, value: i32) -> Self::IntValue {
        self.make_int_value(IntType::I32, value as u64, false)
    }
    fn make_i64(&self, value: i64) -> Self::IntValue {
        self.make_int_value(IntType::I64, value as u64, false)
    }

    fn make_true(&self) -> Self::BoolValue;
    fn make_false(&self) -> Self::BoolValue;

    fn load_register(&mut self, register: Register) -> Self::IntValue;
    fn store_register(&mut self, register: Register, value: Self::IntValue);

    fn load_flag(&mut self, flag: Flag) -> Self::BoolValue;
    fn store_flag(&mut self, flag: Flag, value: Self::BoolValue);

    // TODO: not everything fits into IntType box... like 80-bit floats, for example.......
    fn load_memory(&mut self, size: IntType, address: Self::IntValue) -> Self::IntValue;
    fn store_memory(&mut self, address: Self::IntValue, value: Self::IntValue);

    fn add(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn int_neg(&mut self, val: Self::IntValue) -> Self::IntValue;
    fn sub(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn mul(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn int_or(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn int_and(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn int_xor(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn shl(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn lshr(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn ashr(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn udiv(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;

    // bit should be in bounds! otherwise results in ub
    fn extract_bit(&mut self, val: Self::IntValue, bit: Self::IntValue) -> Self::BoolValue;

    fn bool_not(&mut self, val: Self::BoolValue) -> Self::BoolValue;
    fn bool_xor(&mut self, lhs: Self::BoolValue, rhs: Self::BoolValue) -> Self::BoolValue;
    fn bool_or(&mut self, lhs: Self::BoolValue, rhs: Self::BoolValue) -> Self::BoolValue;

    fn uadd_overflow(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::BoolValue;
    fn sadd_overflow(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::BoolValue;
    fn usub_overflow(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::BoolValue;
    fn ssub_overflow(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::BoolValue;

    fn zext(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue;
    fn sext(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue;
    fn trunc(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue;

    fn icmp(
        &mut self,
        cmp: ComparisonType,
        lhs: Self::IntValue,
        rhs: Self::IntValue,
    ) -> Self::BoolValue;

    fn direct_call(&mut self, target: u32, next_eip: u32);

    fn ifelse<T, F>(&mut self, cond: Self::BoolValue, iftrue: T, iffalse: F)
    where
        T: FnOnce(&mut Self),
        F: FnOnce(&mut Self),
        Self: Sized;

    fn compute_memory_operand_address(&mut self, op: MemoryOperand) -> Self::IntValue {
        assert!(op.index.is_none());
        assert!(op.segment.is_none());

        let mut res = self.make_i32(i32::try_from(op.displacement).unwrap());

        if let Some(base) = op.base {
            let base_val = self.load_register(base);
            res = self.add(res, base_val);
        }

        res
    }

    fn load_operand(&mut self, operand: Operand) -> Self::IntValue {
        match operand {
            Operand::Register(reg) => self.load_register(reg),
            Operand::Immediate8(v) => self.make_u8(v),
            Operand::Immediate16(v) => self.make_u16(v),
            Operand::Immediate32(v) => self.make_u32(v),
            Operand::Immediate64(v) => self.make_u64(v),
            Operand::Memory(op) => {
                let addr = self.compute_memory_operand_address(op);
                self.load_memory(op.size.unwrap(), addr)
            }
            op => panic!("Unsupported load operand: {:?}", op),
        }
    }
    fn store_operand(&mut self, operand: Operand, value: Self::IntValue) {
        match operand {
            Operand::Register(reg) => self.store_register(reg, value),
            Operand::Memory(op) => {
                let addr = self.compute_memory_operand_address(op);
                assert_eq!(op.size.unwrap(), value.size());
                self.store_memory(addr, value)
            }
            op => panic!("Unsupported store operand: {:?}", op),
        }
    }

    fn push(&mut self, val: Self::IntValue) {
        let size = val.size().byte_width();
        let size = self.make_u32(size as u32);

        let esp = self.load_register(Register::ESP);
        let esp = self.sub(esp, size);
        self.store_register(Register::ESP, esp.clone());

        self.store_memory(esp, val);
    }

    fn pop(&mut self, size: IntType) -> Self::IntValue {
        let size_bytes = size.byte_width();
        let size_bytes = self.make_u32(size_bytes as u32);

        let esp = self.load_register(Register::ESP);

        let val = self.load_memory(size, esp.clone());

        let esp = self.add(esp, size_bytes);
        self.store_register(Register::ESP, esp);

        val
    }

    fn extract_msb(&mut self, value: Self::IntValue) -> Self::BoolValue {
        let bit_number =
            self.make_int_value(value.size(), (value.size().bit_width() - 1) as u64, false);
        let msb = self.extract_bit(value, bit_number);
        msb
    }

    // TODO: computing the flags eagerly is kind of inefficient actually
    // it might be beneficial to move to lazy computation like https://github.com/nepx/halfix does

    fn compute_and_store_zf(&mut self, value: Self::IntValue) {
        let zero = self.make_int_value(value.size(), 0, false);
        let zf = self.icmp(ComparisonType::Equal, value, zero);
        self.store_flag(Flag::Zero, zf)
    }

    fn compute_and_store_sf(&mut self, value: Self::IntValue) {
        let sign = self.extract_msb(value);
        self.store_flag(Flag::Sign, sign);
    }
}

// trait Backend {
//     type IntValue: IntValue;
//     type Builder: Builder<IntValue = Self::IntValue>;
//
//     // TODO: how do we make a builder? In LLVM it would need to create a basic block and stuff...
//     // leaving this kludge for now
//     fn make_builder(&mut self) -> Self::Builder; // TODO: lifetime?
// }
