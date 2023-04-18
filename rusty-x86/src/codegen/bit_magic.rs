use super::prelude::*;

pub fn bsr<B: Builder>(builder: &mut B, (dst, src): (Operand, Operand)) -> CF<B> {
    let value = builder.load_operand(src);

    let is_zero = builder.icmp(
        ComparisonType::Equal,
        value,
        builder.make_int_value(value.size(), 0),
    );
    builder.ifelse(
        is_zero,
        |builder| {
            builder.store_flag(Zero, builder.make_true());
        },
        |builder| {
            let leading_zeroes = builder.ctlz(value);
            let sz = value.size().bit_width() - 1;
            let sz = builder.make_int_value(value.size(), sz as u64);
            let res = builder.sub(sz, leading_zeroes);

            builder.store_operand(dst, res);
            builder.store_flag(Zero, builder.make_false());
        },
    );

    ControlFlow::NextInstruction
}

pub fn bt_btc_bts_btr<B: Builder>(
    builder: &mut B,
    (mnemonic, base, offset): (Mnemonic, Operand, Operand),
) -> CF<B> {
    let base_size = base.size();

    let offset = match offset {
        Operand::Register(reg) => {
            assert_eq!(base_size, reg.size());
            builder.load_register(reg)
        }
        Operand::Immediate8(imm) => {
            assert!(imm < base_size.bit_width());
            builder.make_int_value(base_size, imm as u64)
        }
        _ => unreachable!(),
    };

    fn update_value<B: Builder>(
        builder: &mut B,
        mnemonic: Mnemonic,
        val: <B as Builder>::IntValue,
        bit_number: <B as Builder>::IntValue,
    ) -> <B as Builder>::IntValue {
        if mnemonic == Mnemonic::Btr {
            // reset bit
            builder.reset_bit(val, bit_number)
        } else if mnemonic == Mnemonic::Bts {
            // set bit
            builder.set_bit(val, bit_number)
        } else if mnemonic == Mnemonic::Btc {
            // complement bit
            let reset = builder.reset_bit(val, bit_number);
            let set = builder.set_bit(val, bit_number);

            let current = builder.extract_bit(val, bit_number);

            builder.select(current, reset, set)
        } else {
            // leave it be
            val
        }
    }

    match base {
        Operand::Register(reg) => {
            let modulo = reg.size().bit_width() as u64;
            let modulo = builder.make_int_value(base_size, modulo);

            let bit_number = builder.urem(offset, modulo);

            let val = builder.load_register(reg);

            let bit_val = builder.extract_bit(val, bit_number);

            let val = update_value(builder, mnemonic, val, bit_number);

            builder.store_register(reg, val);

            builder.store_flag(Carry, bit_val);
        }
        Operand::Memory(mem) => {
            let addr = builder.compute_memory_operand_address(mem);

            let offset = builder.sext(offset, IntType::I32);

            // definitions for T-division and F-division are from the
            //    Division and Modulus for Computer Scientists paper by Daan Leijen from MS Reasearch:
            // https://www.microsoft.com/en-us/research/publication/division-and-modulus-for-computer-scientists/

            let byte_size = builder.make_u32(8);

            let (byte_offset, bit_number) = {
                // this uses T-division (as both llvm and most processors do)
                // but here we want F-division for our calculations
                // so we do ugly selects
                let byte_offset = builder.sdiv(offset, byte_size);

                let bit_number = builder.srem(offset, byte_size);

                let bit_number_negative =
                    builder.icmp(ComparisonType::SignedLess, bit_number, builder.make_u32(0));

                // basically, in case we have a negative offset, adjust byte offset and bit number so that bit number is positive
                let byte_adj = builder.select(
                    bit_number_negative,
                    builder.make_i32(-1),
                    builder.make_i32(0),
                );

                let bit_adj = builder.select(bit_number_negative, byte_size, builder.make_i32(0));

                let byte_offset = builder.add(byte_offset, byte_adj);
                let bit_number = builder.add(bit_number, bit_adj);

                (byte_offset, bit_number)
            };

            let bit_number = builder.trunc(bit_number, IntType::I8);

            let addr = builder.add(addr, byte_offset);

            let val = builder.load_memory(IntType::I8, addr);

            let bit_val = builder.extract_bit(val, bit_number);

            let val = update_value(builder, mnemonic, val, bit_number);

            builder.store_memory(addr, val);

            builder.store_flag(Carry, bit_val);
        }
        _ => unreachable!(),
    }

    ControlFlow::NextInstruction
}
