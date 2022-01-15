pub mod backend;
pub mod llvm_backend;
pub mod disasm;
pub mod types;

use iced_x86::{Decoder, Instruction};
use crate::backend::{Builder, IntValue};
use crate::types::{IntType, Operand};
use crate::types::Register::*;

//fn get_

// TODO: handle control flow
pub fn codegen_instr<B: Builder>(builder: &mut B, instr: Instruction) {
    use iced_x86::Mnemonic::*;

    assert!(!instr.has_lock_prefix());
    assert!(!instr.has_rep_prefix());
    assert!(!instr.has_repe_prefix());
    assert!(!instr.has_repne_prefix());
    assert!(!instr.has_xacquire_prefix());
    assert!(!instr.has_xrelease_prefix());

    match instr.mnemonic() {
        // TODO: there is (going to be) a ton of opcodes, we would want to handle this nicely (a bit of macromagic?)
        Mov => {
            let dst = disasm::get_operand(&instr, 0);
            let src = disasm::get_operand(&instr, 1);

            let val = builder.load_operand(src);
            builder.store_operand(dst, val);
        },
        Sub => {
            let dst = disasm::get_operand(&instr, 0);
            let src = disasm::get_operand(&instr, 1);

            let lhs = builder.load_operand(dst);
            let rhs = builder.load_operand(src);
            let res = builder.sub(lhs, rhs);

            // TODO: set flags
            builder.store_operand(dst, res);
        },
        Lea => {
            let dst = disasm::get_operand(&instr, 0);
            let src = disasm::get_operand(&instr, 1);

            let addr = match src {
                Operand::Memory(m) => builder.compute_memory_operand_address(m),
                _ => panic!("Expected 2nd lea operand to be memory reference"),
            };

            // TODO: size conversion (store 32-bit address as 16-bit value for example)
            assert_eq!(dst.size(), addr.size());
            builder.store_operand(dst, addr);
        },
        Imul => {
            match instr.op_count() {
                1 => {
                    todo!()
                },
                2 => {
                    let dst = disasm::get_operand(&instr, 0);
                    let src = disasm::get_operand(&instr, 1);

                    assert_eq!(dst.size(), src.size());
                    let double_size = dst.size().double_sized();

                    let lhs = builder.load_operand(dst);
                    let lhs = builder.sext(lhs, double_size);

                    let rhs = builder.load_operand(src);
                    let rhs = builder.sext(rhs, double_size);


                    let res = builder.mul(lhs, rhs);
                    let res_trunc = builder.trunc(res, dst.size());
                    // TODO: flags (based on comparison of res and sext(res_trunc))

                    builder.store_operand(dst, res_trunc)
                },
                3 => {
                    todo!()
                },
                _ => unreachable!(),
            }
        },
        Xor => {
            let dst = disasm::get_operand(&instr, 0);
            let src = disasm::get_operand(&instr, 1);

            let lhs = builder.load_operand(dst);
            let rhs = builder.load_operand(src);

            let res = builder.xor(lhs, rhs);

            // TODO: flags

            builder.store_operand(dst, res)
        },
        Div => {
            let src = disasm::get_operand(&instr, 0);

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
            let hi = builder.shl(hi,
                                 builder.make_int_value(double_size, src.size().bit_width() as u64, false));
            let dividend = builder.or(lo, hi);

            let divisor = builder.load_operand(src);
            let divisor = builder.zext(divisor, double_size);
            let quotient = builder.udiv(dividend.clone(), divisor.clone());

            // calculate the remainder
            let whole = builder.mul(quotient.clone(), divisor);
            let remainder = builder.sub(dividend, whole);

            builder.store_register(quo_dst, quotient);
            builder.store_register(rem_dst, remainder);
        },
        Ret => {
            // TODO: control flow, no-op for now
        }
        m => panic!("Unknown instruction mnemonic: {:?}", m),
    }
}

#[cfg(test)]
mod tests {
    /// use dynasm to assemble the provided code to a Vec<u8>
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
        use std::io;
        use std::io::Write;
        use iced_x86::{Decoder, DecoderOptions};
        use inkwell::context::Context;
        use inkwell::OptimizationLevel;
        use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple};
        use crate::codegen_instr;
        use crate::llvm_backend::{LlvmBuilder, Types};

        fn get_aarch64_target_machine() -> TargetMachine {
            Target::initialize_aarch64(&InitializationConfig::default());
                //.expect("Failed to initialize aarch64 target");
            let target_triple = TargetTriple::create("aarch64");
            let target = Target::from_triple(&target_triple).unwrap();
            target
                .create_target_machine(
                    &target_triple,
                    &"generic",
                    &"",
                    OptimizationLevel::Aggressive,
                    RelocMode::Default,
                    CodeModel::Default,
                )
                .unwrap()
        }

        fn recompile(code: &[u8]) -> Vec<u8> {
            let context = Context::create();
            let context = &context;
            let module = context.create_module("test");
            let mut module = &module;
            let types = Types::new(context);

            {
                let mut builder = LlvmBuilder::new(context, module, types, "dickenson");

                let mut decoder = Decoder::new(32, code, DecoderOptions::NONE);

                while decoder.can_decode() {
                    // TODO: control flow =)
                    codegen_instr(&mut builder, decoder.decode());
                }

                let llvm_builder = builder.get_builder();
                llvm_builder.build_return(None);

                let target_machine = get_aarch64_target_machine();

                let ir = module.print_to_string().to_string();

                println!("{}", ir);

                let memory_buffer = target_machine
                    .write_to_memory_buffer(&mut module, FileType::Object)
                    .unwrap();

                let object_file = memory_buffer.create_object_file().unwrap();

                let secit = object_file.get_sections();

                let mut contents: Option<Vec<u8>> = None;
                for sec in object_file.get_sections() {
                    let name = sec.get_name().and_then(|x| x.to_str().ok());
                    if name == Some(".text") {
                        contents = Some(Vec::from(sec.get_contents()));
                    }
                }

                contents.unwrap()
            }
        }

        #[test]
        fn simple_llvm() {
            // we get this
            let code = assemble_x86!(
                ; mov ebx, 42
            );

            // and recompile it into this
            // isn't it nice?
            let expected_result = assemble_aarch64!(
                ; mov w8, #0x2a
                ; str w8, [x0, #4]
                ; ret
            );

            let result = recompile(code.as_slice());

            assert_eq!(result, expected_result);
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
                ; sub sp, sp, #0x10
                ; ldr w8, [x0, #0x10] // load ESP
                ; add w9, w8, #4
                ; ldr w9, [x1, w9, sxtw] // load [ESP+4] (a)
                ; add w8, w8, #8
                ; str w9, [x0, #0xc] // store [ESP+4] as EDX
                ; ldr w8, [x1, w8, sxtw] // load [ESP+8] (b)
                ; add w10, w9, #0xd // compute EDX+13
                ; str w10, [x0, #8] // store EDX+13 as ECX
                ; stp w8, w9, [sp, #8] // ??? why ???
                ; sub w8, w8, w9 // compute b-a
                ; mul w8, w8, w9 // compute a*(b-a)
                ; udiv x9, x8, x10 // compute division of a*(b-a)/(13+a)
                ; msub x8, x9, x10, x8 // compute remainder
                ; str x9, [x0] // store div result in EAX
                ; stur x8, [x0, #0xc] // store remainder in EDX
                ; add sp, sp, #0x10
                ; ret
            );

            let result = recompile(code.as_slice());

            assert_eq!(result, expected_result);
        }
    }


}
