pub mod backend;
pub mod disasm;
pub mod types;
pub mod llvm;

use crate::backend::{Builder, IntValue};
use crate::disasm::Operands;
use crate::types::Register::*;
use crate::types::{ControlFlow, Flag, IntType, Operand};
use iced_x86::{ConditionCode, Instruction, Mnemonic};
//use crate::disasm::;

//fn get_

fn compute_condition_code<B: Builder>(builder: &mut B, condition_code: ConditionCode) -> B::BoolValue {
    use ConditionCode::*;
    match condition_code {
        None => panic!("Can't compute None condition"),

        e => {
            let zf = builder.load_flag(Flag::Zero);
            zf
        }
        ne => {
            let zf = builder.load_flag(Flag::Zero);
            builder.bool_neg(zf)
        },

        s => {
            let sf = builder.load_flag(Flag::Sign);
            sf
        },
        ns => {
            let sf = builder.load_flag(Flag::Sign);
            builder.bool_neg(sf)
        }

        _ => todo!("condition code {:?}", condition_code),
    }
}

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

    if instr.is_jcc_short_or_near() {
        operands!([target], &instr);

        let code = instr.condition_code();
        let cond = compute_condition_code(builder, code);

        builder.ifelse(cond, |_builder| {
            // jump!
            ControlFlow::DirectJump(target.as_imm32())
        }, |_builder| {
            // do not jump
            ControlFlow::NextInstruction
        })
    } else if is_cmovcc(instr.mnemonic()) {
        operands!([dst, src], &instr);

        let code = instr.condition_code();
        let cond = compute_condition_code(builder, code);

        builder.ifelse(cond, |builder| {
            // move!
            let val = builder.load_operand(src);
            builder.store_operand(dst, val);
            ControlFlow::NextInstruction
        }, |_builder| {
            // do not move
            ControlFlow::NextInstruction
        })
    } else {
        match instr.mnemonic() {
            // TODO: there is (going to be) a ton of opcodes, we would want to handle this nicely (a bit of macromagic?)
            Nop => {
                // fuf, this was easy
            }
            Mov => {
                operands!([dst, src], &instr);

                let val = builder.load_operand(src);
                builder.store_operand(dst, val);
            }
            Sub => {
                operands!([dst, src], &instr);

                let lhs = builder.load_operand(dst);
                let rhs = builder.load_operand(src);
                let res = builder.sub(lhs, rhs);

                builder.store_operand(dst, res);

                // TODO: flags
                // The OF, SF, ZF, AF, PF, and CF flags are set according to the result.
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
            }
            Cmp => {
                operands!([src1, src2], &instr);

                let src1 = builder.load_operand(src1);
                let src2 = builder.load_operand(src2);

                let src2 = builder.sext(src2, src1.size());

                let tmp = builder.sub(src1, src2);

                // TODO: flags
                // The OF, SF, ZF, AF, PF, and CF flags are set according to the result.
                builder.compute_and_store_zf(tmp);
                builder.compute_and_store_sf(tmp);
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

                let res = builder.xor(lhs, rhs);

                builder.store_operand(dst, res);

                // TODO: flags
                // The OF and CF flags are cleared; the SF, ZF, and PF flags are set according to the result.
                // The state of the AF flag is undefined.
                // TODO: do we want to represent ub here? leaving as zero for now
                builder.compute_and_store_zf(res);
                builder.compute_and_store_sf(res);
                builder.store_flag(Flag::Carry, builder.make_false());
                builder.store_flag(Flag::Overflow, builder.make_false());
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
                let dividend = builder.or(lo, hi);

                let divisor = builder.load_operand(src);
                let divisor = builder.zext(divisor, double_size);
                let quotient = builder.udiv(dividend, divisor);

                // calculate the remainder
                let whole = builder.mul(quotient, divisor);
                let remainder = builder.sub(dividend, whole);

                builder.store_register(quo_dst, quotient);
                builder.store_register(rem_dst, remainder);
            }
            Push => {
                operands!([src], &instr);

                let val = builder.load_operand(src);

                let size = val.size().byte_width();
                let size = builder.make_u32(size as u32);

                let esp = builder.load_register(ESP);
                let esp = builder.sub(esp, size);
                builder.store_register(ESP, esp);

                builder.store_memory(esp, val);
            }
            Pop => {
                operands!([dst], &instr);

                let size = dst.size().byte_width();
                let size = builder.make_u32(size as u32);


                let esp = builder.load_register(ESP);

                let val = builder.load_memory(dst.size(), esp);

                let esp = builder.add(esp, size);
                builder.store_register(ESP, esp);

                builder.store_operand(dst, val);
            }
            Ret => {
                // TODO: control flow, no-op for now
            }
            Jmp => {
                operands!([target], &instr);

                return ControlFlow::DirectJump(target.as_imm32());
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
        use test_log::test;
        #[allow(unused_imports)]
        use log::{debug, error, info, trace, warn};

        fn recompile(code: &[u8]) -> Vec<u8> {
            let context = &Context::create();
            let mut module = llvm::recompile(context, 0, code);


            let target_machine = llvm::get_aarch64_target_machine();

            let ir = module.print_to_string().to_string();

            println!("{}", ir);

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
                ; mov w8, #0x2a
                ; str w8, [x0, #4]
                ; ret
            );

            let result = recompile(code.as_slice());

            assert_eq!(expected_result, result);
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
                ; add w9, w8, #4
                ; ldr w9, [x1, w9, uxtw]
                ; add w8, w8, #8
                ; str w9, [x0, #0xc]
                ; ldr w8, [x1, w8, uxtw]
                ; strb wzr, [x0, #0x20]
                ; subs w8, w8, w9
                ; cset w10, eq
                ; strb w10, [x0, #0x23]
                ; lsr w10, w8, #0x1f
                ; strb w10, [x0, #0x24]
                ; mov w10, #1
                ; mul w8, w8, w9
                ; add w9, w9, #0xd
                ; sturh w10, [x0, #0x23]
                ; udiv x10, x8, x9
                ; msub x8, x10, x9, x8
                ; str w9, [x0, #8]
                ; strh wzr, [x0, #0x24]
                ; str x10, [x0]
                ; stur x8, [x0, #0xc]
                ; ret
            );

            let result = recompile(code.as_slice());

            assert_eq!(expected_result, result);
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
                ; str w8, [x0, #0x14]
                ; add w8, w8, #8
                ; ldr w8, [x1, w8, uxtw]
                ; subs w8, w8, #1
                ; cset w9, eq
                ; strb w9, [x0, #0x23]
                ; mov w9, #2
                ; lsr w8, w8, #0x1f
                ; csinc w9, w9, wzr, eq
                ; strb w8, [x0, #0x24]
                ; tbnz w9, #0, ->FALSE

                ; b #0

                ; ->FALSE:
                ; mov w8, #0x2a
                ; str w8, [x0]
                ; ret

                ; ->basic_block_00000010:
                ; ldr w8, [x0, #0x10]
                ; movn w9, #0
                ; str w9, [x0]
                ; ldr w9, [x1, x8]
                ; add w8, w8, #4
                ; stp w8, w9, [x0, #0x10]
                ; ret
            );

            let result = recompile(code.as_slice());

            assert_eq!(expected_result, result);
        }
    }
}
