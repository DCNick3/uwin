
use crate::types::{IntType, Operand, Register};

pub trait IntValue {

}

pub trait Builder {
    type IntValue: IntValue;

    fn make_int_value(&mut self, ty: IntType, value: u64, sign_extend: bool) -> Self::IntValue;

    // TODO: implement all the variants with all the sizes
    fn make_u8(&mut self, value: u8) -> Self::IntValue {
        self.make_int_value(IntType::I8, value as u64, false)
    }
    fn make_u16(&mut self, value: u16) -> Self::IntValue {
        self.make_int_value(IntType::I16, value as u64, false)
    }
    fn make_u32(&mut self, value: u32) -> Self::IntValue {
        self.make_int_value(IntType::I32, value as u64, false)
    }
    fn make_u64(&mut self, value: u64) -> Self::IntValue {
        self.make_int_value(IntType::I64, value as u64, false)
    }

    fn load_register(&mut self, register: Register) -> Self::IntValue;
    fn store_register(&mut self, register: Register, value: Self::IntValue);

    // TODO: not everything fits into IntType box... like 80-bit floats, for example.......
    fn load_memory(&mut self, size: IntType, address: Self::IntValue) -> Self::IntValue;
    fn store_memory(&mut self, size: IntType, address: Self::IntValue, value: Self::IntValue);

    fn load_operand(&mut self, operand: Operand) -> Self::IntValue {
        match operand {
            Operand::Register(reg) => self.load_register(reg),
            Operand::Immediate8(v) => self.make_u8(v),
            Operand::Immediate16(v) => self.make_u16(v),
            Operand::Immediate32(v) => self.make_u32(v),
            Operand::Immediate64(v) => self.make_u64(v),
            Operand::Memory(_) => todo!(), // this is damn hard actually
            op => panic!("Unsupported load operand: {:?}", op),
        }
    }
    fn store_operand(&mut self, operand: Operand, value: Self::IntValue) {
        match operand {
            Operand::Register(reg) => self.store_register(reg, value),
            Operand::Memory(_) => todo!(),
            op => panic!("Unsupported store operand: {:?}", op),
        }
    }


    // TODO: maybe (probably?) we will need a way to express branches here. Not the branch instructions, but conditional execution in the context of the instruction itself
}

// trait Backend {
//     type IntValue: IntValue;
//     type Builder: Builder<IntValue = Self::IntValue>;
//
//     // TODO: how do we make a builder? In LLVM it would need to create a basic block and stuff...
//     // leaving this kludge for now
//     fn make_builder(&mut self) -> Self::Builder; // TODO: lifetime?
// }
