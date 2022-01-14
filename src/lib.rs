pub mod backend;
pub mod llvm_backend;
pub mod disasm;
pub mod types;

use iced_x86::{Decoder, Instruction};
use crate::backend::Builder;

//fn get_

// TODO: handle control flow
pub fn codegen_instr<B: Builder>(builder: &mut B, instr: Instruction) {
    use iced_x86::Mnemonic::*;
    match instr.mnemonic() {
        // TODO: there is (going to be) a ton of opcodes, we would want to handle this nicely (a bit of macromagic?)
        Mov => {
            let dst = disasm::get_operand(&instr, 0);
            let src = disasm::get_operand(&instr, 1);

            let val = builder.load_operand(src);
            builder.store_operand(dst, val);
        },
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

        #[test]
        fn simple_llvm() {
            // we get this
            let code = assemble_x86!(
                ; mov ebx, 42
            );

            // and recompile it into this
            // isn't it nice?
            let result = assemble_aarch64!(
                ; mov w8, #0x2a
                ; str w8, [x0, #4]
                ; ret
            );

            let context = Context::create();
            let context = &context;
            let module = context.create_module("test");
            let mut module = &module;
            let types = Types::new(context);

            {
                let mut builder = LlvmBuilder::new(context, module, types, "dickenson");

                let mut decoder = Decoder::new(32, code.as_slice(), DecoderOptions::NONE);

                codegen_instr(&mut builder, decoder.decode());

                let llvm_builder = builder.get_builder();
                llvm_builder.build_return(None);

                let target_machine = get_aarch64_target_machine();

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

                let contents = contents.unwrap();

                assert_eq!(contents, result);
            }
        }
    }


}
