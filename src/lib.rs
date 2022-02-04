pub mod backend;
pub mod disasm;
pub mod llvm;
pub mod types;

use crate::backend::{Builder, ComparisonType, IntValue};
use crate::disasm::Operands;
use crate::types::Register::*;
use crate::types::{ControlFlow, Flag, IntType, Operand};
use iced_x86::{ConditionCode, Instruction, Mnemonic};

fn compute_condition_code<B: Builder>(
    builder: &mut B,
    condition_code: ConditionCode,
) -> B::BoolValue {
    use ConditionCode::*;
    match condition_code {
        None => panic!("Can't compute None condition"),

        e => {
            let zf = builder.load_flag(Flag::Zero);
            zf
        }
        ne => {
            let zf = builder.load_flag(Flag::Zero);
            builder.bool_not(zf)
        }

        s => {
            let sf = builder.load_flag(Flag::Sign);
            sf
        }
        ns => {
            let sf = builder.load_flag(Flag::Sign);
            builder.bool_not(sf)
        }

        b => {
            let cf = builder.load_flag(Flag::Carry);
            cf
        }
        ae => {
            let cf = builder.load_flag(Flag::Carry);
            builder.bool_not(cf)
        }

        be => {
            let cf = builder.load_flag(Flag::Carry);
            let zf = builder.load_flag(Flag::Zero);
            let r = builder.bool_or(cf, zf);
            r
        }
        a => {
            let cf = builder.load_flag(Flag::Carry);
            let zf = builder.load_flag(Flag::Zero);
            let r = builder.bool_or(cf, zf);
            let r = builder.bool_not(r);
            r
        }

        _ => todo!("condition code {:?}", condition_code),
    }
}

#[rustfmt::skip]
fn is_cmovcc(mnemonic: Mnemonic) -> bool {
    use Mnemonic::*;
    match mnemonic {
        Cmova |
        Cmovae |
        Cmovb |
        Cmovbe |
        Cmove |
        Cmovg |
        Cmovge |
        Cmovl |
        Cmovle |
        Cmovne |
        Cmovno |
        Cmovnp |
        Cmovns |
        Cmovo |
        Cmovp |
        Cmovs  => {
            true
        },
        _ => false,
    }
}

// TODO: handle control flow
pub fn codegen_instr<B: Builder>(builder: &mut B, instr: Instruction) -> ControlFlow<B> {
    use iced_x86::Mnemonic::*;

    assert!(!instr.has_lock_prefix());
    assert!(!instr.has_rep_prefix());
    assert!(!instr.has_repe_prefix());
    assert!(!instr.has_repne_prefix());
    assert!(!instr.has_xacquire_prefix());
    assert!(!instr.has_xrelease_prefix());

    let mnemonic = instr.mnemonic();

    if instr.is_jcc_short_or_near() {
        operands!([target], &instr);

        let code = instr.condition_code();
        let cond = compute_condition_code(builder, code);

        ControlFlow::Conditional(cond, target.as_imm32())
    } else if is_cmovcc(instr.mnemonic()) {
        operands!([dst, src], &instr);

        let code = instr.condition_code();
        let cond = compute_condition_code(builder, code);

        builder.ifelse(
            cond,
            |builder| {
                // move!
                let val = builder.load_operand(src);
                builder.store_operand(dst, val);
            },
            |_builder| {}, // nuff to do,
        );

        ControlFlow::NextInstruction
    } else {
        match mnemonic {
            // TODO: there is (going to be) a ton of opcodes, we would want to handle this nicely (a bit of macromagic?)
            Nop => {
                // fuf, this was easy
            }
            Mov => {
                operands!([dst, src], &instr);

                let val = builder.load_operand(src);
                builder.store_operand(dst, val);
            }
            Add => {
                operands!([dst, src], &instr);

                let lhs = builder.load_operand(dst);
                let rhs = builder.load_operand(src);
                let res = builder.add(lhs, rhs);

                builder.store_operand(dst, res);

                let of = builder.sadd_overflow(lhs, rhs);
                let cf = builder.uadd_overflow(lhs, rhs);

                // TODO: flags
                // The OF, SF, ZF, AF, PF, and CF flags are set according to the result.
                // AF and PF are not implemented rn
                // not that they are actually useful...
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Overflow, of);
                builder.store_flag(Flag::Carry, cf);
            }
            Sub | Cmp => {
                operands!([dst, src], &instr);

                let lhs = builder.load_operand(dst);
                let rhs = builder.load_operand(src);
                let res = builder.sub(lhs, rhs);

                if mnemonic == Sub {
                    builder.store_operand(dst, res);
                }

                let of = builder.ssub_overflow(lhs, rhs);
                let cf = builder.usub_overflow(lhs, rhs);

                // TODO: flags
                // The OF, SF, ZF, AF, PF, and CF flags are set according to the result.
                // AF and PF are not implemented rn
                // not that they are actually useful...
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Overflow, of);
                builder.store_flag(Flag::Carry, cf);
            }
            Lea => {
                operands!([dst, src], &instr);

                let addr = match src {
                    Operand::Memory(m) => builder.compute_memory_operand_address(m),
                    _ => panic!("Expected 2nd lea operand to be memory reference"),
                };

                // TODO: size conversion (store 32-bit address as 16-bit value for example)
                assert_eq!(dst.size(), addr.size());
                builder.store_operand(dst, addr);
            }
            Imul => {
                match *instr.get_operands().as_slice() {
                    [_] => {
                        todo!()
                    }
                    [dst, src] => {
                        assert_eq!(dst.size(), src.size());
                        let double_size = dst.size().double_sized();

                        let lhs = builder.load_operand(dst);
                        let lhs = builder.sext(lhs, double_size);

                        let rhs = builder.load_operand(src);
                        let rhs = builder.sext(rhs, double_size);

                        let res = builder.mul(lhs, rhs);
                        let res_trunc = builder.trunc(res, dst.size());
                        // TODO: flags (based on comparison of res and sext(res_trunc))
                        // For the one operand form of the instruction, the CF and OF flags are set when significant bits are carried into the
                        // upper half of the result and cleared when the result fits exactly in the lower half of the result. For the two- and
                        // three-operand forms of the instruction, the CF and OF flags are set when the result must be truncated to fit in the
                        // destination operand size and cleared when the result fits exactly in the destination operand size.

                        // The SF, ZF, AF, and PF flags are undefined.
                        // TODO: do we want to represent ub here? leaving as zero for now
                        builder.store_flag(Flag::Zero, builder.make_false());
                        builder.store_flag(Flag::Sign, builder.make_false());

                        builder.store_operand(dst, res_trunc)
                    }
                    [_, _, _] => {
                        todo!()
                    }
                    _ => unreachable!(),
                }
            }
            Xor => {
                operands!([dst, src], &instr);

                let lhs = builder.load_operand(dst);
                let rhs = builder.load_operand(src);

                let res = builder.int_xor(lhs, rhs);

                builder.store_operand(dst, res);

                // The OF and CF flags are cleared; the SF, ZF, and PF flags are set according to the result.
                // The state of the AF flag is undefined.
                // TODO: do we want to represent ub here? leaving as zero for now
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Carry, builder.make_false());
                builder.store_flag(Flag::Overflow, builder.make_false());
            }
            And | Test => {
                operands!([dst, src], &instr);

                let lhs = builder.load_operand(dst);
                let rhs = builder.load_operand(src);

                let res = builder.int_and(lhs, rhs);

                if mnemonic == And {
                    builder.store_operand(dst, res);
                }
                // The OF and CF flags are cleared; the SF, ZF, and PF flags are set according to the result. The state of the AF flag is
                // undefined.
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Carry, builder.make_false());
                builder.store_flag(Flag::Overflow, builder.make_false());
            }
            Shr | Sar => {
                operands!([dst, count], &instr);

                let count = builder.load_operand(count);

                let count_mask = builder.make_int_value(count.size(), 0x1f, false);
                let count = builder.int_and(count, count_mask);
                let count = builder.zext(count, dst.size());

                let not_zero = builder.icmp(
                    ComparisonType::NotEqual,
                    count,
                    builder.make_int_value(count.size(), 0, false),
                );

                builder.ifelse(
                    not_zero,
                    |builder| {
                        let val = builder.load_operand(dst);

                        let res = if mnemonic == Shr {
                            builder.lshr(val, count)
                        } else {
                            builder.ashr(val, count)
                        };
                        let count_cf =
                            builder.sub(count, builder.make_int_value(count.size(), 1, false));

                        builder.store_operand(dst, res);

                        let cf = builder.extract_bit(val, count_cf);
                        // OF is defined only for 1-bit shifts, but we'll compute it anyways
                        // maybe we can get better by telling LLVM it's undef?
                        let of = if mnemonic == Shr {
                            builder.extract_msb(val)
                        } else {
                            builder.make_false()
                        };

                        // The CF flag contains the value of the last bit shifted out of the
                        // destination operand; it is undefined for SHL and SHR instructions where
                        // the count is greater than or equal to the size (in bits) of the
                        // destination operand (TODO: add undef?). The OF flag is affected only
                        // for 1-bit shifts (see “Description” above); otherwise, it is undefined.
                        // The SF, ZF, and PF flags are set according to the result.
                        // If the count is 0, the flags are not affected.
                        // For a non-zero count, the AF flag is undefined.
                        builder.compute_and_store_zf(res);
                        builder.compute_and_store_sf(res);
                        builder.store_flag(Flag::Carry, cf);
                        builder.store_flag(Flag::Overflow, of);
                    },
                    |_| {
                        // nuff to do
                    },
                );
            }
            Div => {
                operands!([src], &instr);

                let double_size = src.size().double_sized();

                let (lo, hi, quo_dst, rem_dst) = match src.size() {
                    IntType::I8 => todo!(),
                    IntType::I16 => todo!(),
                    IntType::I32 => (EAX, EDX, EAX, EDX),
                    _ => unreachable!(),
                };

                assert_eq!(lo.size(), hi.size());

                let lo = builder.load_register(lo);
                let hi = builder.load_register(hi);

                let lo = builder.zext(lo, double_size);
                let hi = builder.zext(hi, double_size);
                let hi = builder.shl(
                    hi,
                    builder.make_int_value(double_size, src.size().bit_width() as u64, false),
                );
                let dividend = builder.int_or(lo, hi);

                let divisor = builder.load_operand(src);
                let divisor = builder.zext(divisor, double_size);
                let quotient = builder.udiv(dividend, divisor);

                // calculate the remainder
                let whole = builder.mul(quotient, divisor);
                let remainder = builder.sub(dividend, whole);

                let quotient = builder.trunc(quotient, src.size());
                let remainder = builder.trunc(remainder, src.size());

                builder.store_register(quo_dst, quotient);
                builder.store_register(rem_dst, remainder);

                // all flags are undefined
            }
            Push => {
                operands!([src], &instr);

                let val = builder.load_operand(src);

                builder.push(val);
            }
            Pop => {
                operands!([dst], &instr);

                let val = builder.pop(dst.size());

                builder.store_operand(dst, val);
            }
            Ret => {
                // TODO: control flow, no-op for now
                // Pop the return address (TODO: where to store it? we don't have EIP yet)

                let _raddr = builder.pop(IntType::I32);

                return ControlFlow::Return;
            }
            Jmp => {
                operands!([target], &instr);

                return match target {
                    Operand::Immediate8(_) | Operand::Immediate16(_) | Operand::Immediate64(_) => {
                        panic!("Jump to unsupported immediate size")
                    }
                    Operand::Immediate32(target) => ControlFlow::DirectJump(target),
                    _ => todo!(),
                };
            }
            Call => {
                operands!([target], &instr);

                let ret = instr.next_ip32();
                builder.push(builder.make_u32(ret));

                match target {
                    Operand::Immediate8(_) | Operand::Immediate16(_) | Operand::Immediate64(_) => {
                        panic!("Call to unsupported immediate size")
                    }
                    Operand::Immediate32(target) => {
                        builder.direct_call(target, instr.next_ip32());
                    }
                    _ => todo!(),
                }
            }
            m => panic!("Unknown instruction mnemonic: {:?}", m),
        };

        ControlFlow::NextInstruction
    }
}

/// use dynasm to assemble the provided code to a Vec<u8>
#[macro_export]
macro_rules! assemble_x86 {
    ($($assembly:tt)*) => {
        {
            #[allow(unused)]
            use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
            let mut ops = dynasmrt::x86::Assembler::new().unwrap();

            dynasm!(ops
                ; .arch x86
                $($assembly)*
            );

            let result: Vec<u8> = ops.finalize().unwrap().to_vec();
            result
        }
    }
}

#[macro_export]
macro_rules! assemble_aarch64 {
    ($($assembly:tt)*) => {
        {
            #[allow(unused)]
            use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
            let mut ops = dynasmrt::aarch64::Assembler::new().unwrap();

            dynasm!(ops
                ; .arch aarch64
                $($assembly)*
            );

            let result: Vec<u8> = ops.finalize().unwrap().to_vec();
            result
        }
    }
}

#[cfg(test)]
mod tests {
    // test that above helper macro works =)
    mod assembly {
        #[test]
        fn basic_assembly() {
            use dynasmrt::{dynasm, DynasmApi};
            let mut ops = dynasmrt::x86::Assembler::new().unwrap();

            dynasm!(ops
                ; .arch x86
                ; mov eax, 42
            );

            let result = ops.finalize().unwrap().to_vec();
            assert_eq!(&*result, b"\xB8\x2A\x00\x00\x00");
        }

        #[test]
        fn macro_assembly() {
            let result = assemble_x86!(
                ; mov eax, 42
            );
            assert_eq!(result, b"\xB8\x2A\x00\x00\x00");
        }
    }

    mod llvm {
        use crate::llvm;
        use inkwell::context::Context;
        use inkwell::targets::FileType;
        #[allow(unused_imports)]
        use log::{debug, error, info, trace, warn};
        use std::fmt::Write;
        use test_log::test;

        fn recompile(code: &[u8]) -> Vec<u8> {
            let context = &Context::create();
            let types = &llvm::backend::Types::new(&context);
            let mut module = llvm::recompile(context, types, 0x1000, code);

            let target_machine = llvm::get_aarch64_target_machine();

            let ir = module.print_to_string().to_string();

            debug!("llvm ir:\n{}", ir);

            module.verify().unwrap();

            let memory_buffer = target_machine
                .write_to_memory_buffer(&mut module, FileType::Object)
                .unwrap();

            let _raw_buffer = format!("{:?}", memory_buffer.as_slice());

            let object_file = memory_buffer.create_object_file().unwrap();

            let mut contents: Option<Vec<u8>> = None;
            for sec in object_file.get_sections() {
                let name = sec.get_name().and_then(|x| x.to_str().ok());
                if name == Some(".text") {
                    contents = Some(Vec::from(sec.get_contents()));
                }
            }

            contents.unwrap()
        }

        fn test_recomp(x86_code: Vec<u8>, expected_aarch64_code: Vec<u8>) {
            debug!(
                "CODE:\n{}",
                crate::disasm::disassemble(x86_code.as_slice(), 0x1000)
            );

            let result = recompile(x86_code.as_slice());

            fn disasm_aarch64(aarch64_code: Vec<u8>) -> String {
                let mut res = String::new();
                for maybe_decoded in bad64::disasm(aarch64_code.as_slice(), 0x1000) {
                    let decoded = maybe_decoded.unwrap();
                    writeln!(res, "{:08x} {}", decoded.address(), decoded).unwrap();
                }
                res
            }

            let result = disasm_aarch64(result);
            let expected = disasm_aarch64(expected_aarch64_code);

            debug!("expected aarch64:\n{}", expected);
            debug!("actual aarch64:\n{}", result);

            assert_eq!(result, expected);
        }

        #[test]
        fn simple_llvm() {
            // we get this
            let code = assemble_x86!(
                ; mov ebx, 42
                ; ret
            );

            // and recompile it into this
            // isn't it nice?
            let expected_result = assemble_aarch64!(
                ; ldr w8, [x0, #0x10]
                ; mov w9, #0x2a
                ; str w9, [x0, #0x4]
                ; add w8, w8, #0x4
                ; str w8, [x0, #0x10]
                ; ret
            );

            test_recomp(code, expected_result);
        }

        #[test]
        fn magic_llvm() {
            // unsigned magic(unsigned a, unsigned b) {
            //     return a * (b - a) / (13 + a);
            // }
            let code = assemble_x86!(
                ; mov     edx, DWORD [esp+4]
                ; mov     eax, DWORD [esp+8]
                ; sub     eax, edx
                ; lea     ecx, [edx+13]
                ; imul    eax, edx
                ; xor     edx, edx
                ; div     ecx
                ; ret
            );

            /*  ; ->fact:
            ; mov     eax, DWORD [esp+4]
            ; mov     edx, 1
            ; cmp     eax, 1
            ; jbe     ->L1
            ; ->L2:
            ; mov     ecx, eax
            ; sub     eax, 1
            ; imul    edx, ecx
            ; cmp     eax, 1
            ; jne     ->L2
            ; ->L1:
            ; mov     eax, edx
            ; ret*/

            // and recompile it into this
            // isn't it nice?
            let expected_result = assemble_aarch64!(
                // ; ldr w8, [x0, #0x10] // load ESP
                // ; add w9, w8, #4
                // ; ldr w9, [x1, w9, sxtw] // load [ESP+4] (a)
                // ; add w8, w8, #8
                // ; str w9, [x0, #0xc] // store [ESP+4] as EDX
                // ; ldr w8, [x1, w8, sxtw] // load [ESP+8] (b)
                // ; add w10, w9, #0xd // compute EDX+13
                // ; str w10, [x0, #8] // store EDX+13 as ECX
                // ; sub w8, w8, w9 // compute b-a
                // ; mul w8, w8, w9 // compute a*(b-a)
                // ; udiv x9, x8, x10 // compute division of a*(b-a)/(13+a)
                // ; msub x8, x9, x10, x8 // compute remainder
                // ; str x9, [x0] // store div result in EAX
                // ; stur x8, [x0, #0xc] // store remainder in EDX
                // ; ret

                ; ldr w8, [x0, #0x10]
                ; add w9, w8, #0x4
                ; ldrsw x10, [x1, w9, uxtw]
                ; add w8, w8, #0x8
                ; str w10, [x0, #0xc]
                ; ldr w8, [x1, w8, uxtw]
                ; strb wzr, [x0, #0x20]
                ; str w9, [x0, #0x10]
                ; subs w8, w8, w10
                ; cset w11, eq
                ; strb w11, [x0, #0x23]
                ; lsr w11, w8, #0x1f
                ; sxtw x8, w8
                ; strb w11, [x0, #0x24]
                ; mov w11, #0x1
                ; mul x8, x8, x10
                ; add w10, w10, #0xd
                ; sturh w11, [x0, #0x23]
                ; and x11, x8, #0xffffffff
                ; udiv x11, x11, x10
                ; msub w8, w11, w10, w8
                ; strh wzr, [x0, #0x24]
                ; str w11, [x0]
                ; stp w10, w8, [x0, #0x8]
                ; ret
            );

            test_recomp(code, expected_result);
        }

        #[test]
        fn answer_llvm() {
            // int answer(int a) {
            //     if (a == 1)
            //         return 42;
            //     return -1;
            // }
            let code = assemble_x86!(
                ; push ebp
                ; mov ebp, esp
                ; cmp DWORD [ebp+8], 1
                ; jne ->L2
                ; mov eax, 42
                ; jmp ->L3
                ; ->L2:
                ; mov eax, -1
                ; ->L3:
                ; pop ebp
                ; ret
            );

            let expected_result = assemble_aarch64!(
                ; ldp w8, w9, [x0, #0x10]
                ; sub w8, w8, #4
                ; str w8, [x0, #0x10]
                ; str w9, [x1, w8, uxtw]
                ; ldr w8, [x0, #0x10]
                ; mov w9, #0x2
                ; str w8, [x0, #0x14]
                ; add w8, w8, #8
                ; ldr w8, [x1, w8, uxtw]
                ; subs w8, w8, #1
                ; cset w11, eq
                ; cset w10, vs
                ; strb w11, [x0, #0x23]
                ; cset w11, cc
                ; lsr w8, w8, #0x1f
                ; csinc w9, w9, wzr, eq
                ; strb w8, [x0, #0x24]
                ; strb w10, [x0, #0x25]
                ; strb w11, [x0, #0x20]
                ; tbnz w9, #0, ->FALSE

                ; b ->basic_block_0000101A

                ; ->FALSE:
                ; mov w8, #0x2a
                ; str w8, [x0]
                ; b ->basic_block_0000101F

                ; ->basic_block_0000101A:
                ; ldr w8, [x0, #0x10]
                ; movn w9, #0
                ; str w9, [x0]
                ; ldr w9, [x1, x8]
                ; add w8, w8, #8
                ; stp w8, w9, [x0, #0x10]
                ; ret

                ; ->basic_block_0000101F:
                ; ldr w8, [x0, #0x10]
                ; ldr w9, [x1, x8]
                ; add w8, w8, #0x8
                ; stp w8, w9, [x0, #0x10]
                ; ret
            );

            test_recomp(code, expected_result);
        }

        #[test]
        fn jump_over_llvm() {
            let code = assemble_x86!(
                ; jmp ->LABEL
                ; add eax, 1
                ; ->LABEL:
                ; mov eax, 42
            );

            let expected_result = assemble_aarch64!(
                ; ->bb_0x1000:
                ; b ->bb_0x1005

                ; ->bb_0x1005:
                ; mov w8, #0x2a
                ; str w8, [x0, #0]
                ; ret
            );

            test_recomp(code, expected_result);
        }

        #[test]
        fn div_llvm() {
            let code = assemble_x86!(
                ; mov eax, 1
                ; mov ebx, 2
                ; div ebx
            );

            let expected_result = assemble_aarch64!(
                ; ldr w8, [x0, #0xc]  // load EDX
                ; mov w9, #0x2
                ; mov w10, #0x1
                ; str w10, [x0, #0xc] // store 0x1 to EDX
                ; lsl w8, w8, #0x1f   // it was division of EDX:EAX by 2, so the lo bit of EDX becomes the MSB of EAX
                ; stp w8, w9, [x0]    // store w8->EAX, 2->EBX
                ; ret

            );

            test_recomp(code, expected_result);
        }
    }
}
