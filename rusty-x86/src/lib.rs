extern crate core;

pub mod backend;
mod codegen;
pub mod disasm;
pub mod interp;
#[cfg(feature = "llvm")]
pub mod llvm;
pub mod types;

use crate::backend::{Builder, ComparisonType, IntValue};
use crate::types::{ControlFlow, Flag, IntType, Register};

#[cfg(feature = "llvm")]
pub use inkwell;

pub use codegen::codegen_instr;

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

    #[cfg(feature = "llvm")]
    mod llvm {
        use crate::llvm;
        use inkwell::context::Context;
        use inkwell::passes::PassBuilderOptions;
        use inkwell::targets::FileType;
        #[allow(unused_imports)]
        use log::{debug, error, info, trace, warn};
        use memory_image::MemoryImage;
        use std::collections::BTreeMap;
        use std::fmt::Write;
        use test_log::test;

        fn recompile(code: &[u8]) -> Vec<u8> {
            let context = &Context::create();
            let types = llvm::backend::Types::new(context);
            let thunk_functions = &BTreeMap::new();

            let code = MemoryImage::from_code_region(0x1000, code);

            let module =
                llvm::recompile_single_module(context, types, thunk_functions, &code, &[0x1000]);

            let target_machine = llvm::get_aarch64_target_machine();

            let ir = module.print_to_string().to_string();

            trace!("llvm ir:\n{}", ir);

            module.verify().unwrap();

            let pass_options = PassBuilderOptions::create();
            pass_options.set_verify_each(true);
            pass_options.set_debug_logging(false);
            pass_options.set_loop_interleaving(true);
            pass_options.set_loop_vectorization(true);
            pass_options.set_loop_slp_vectorization(true);
            pass_options.set_loop_unrolling(true);
            pass_options.set_forget_all_scev_in_loop_unroll(true);
            pass_options.set_licm_mssa_opt_cap(1);
            pass_options.set_licm_mssa_no_acc_for_promotion_cap(10);
            pass_options.set_call_graph_profile(true);
            pass_options.set_merge_functions(true);

            module
                .run_passes("default<O3>", &target_machine, pass_options)
                .unwrap();

            let memory_buffer = target_machine
                .write_to_memory_buffer(&module, FileType::Object)
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
        fn mov_ax_42_llvm() {
            let code = assemble_x86!(
                ; mov ax, 42
            );

            let expected_result = assemble_aarch64!(
                ; ->uwin_indirect_bb_call:
                ; mov x8, x0
                ; cmp w2, #0x1, lsl #0xc
                ; b.ne >bb_missing

                ; ->bb_0x1000:
                ; mov w9, #0x2a
                ; mov w0, wzr
                ; strh w9, [x8]
                ; ret

                ; bb_missing:
                ; mov x0, x8
                ; loopa: // actually it's not as loop, but just an unlinked call to uwin_missing_bb, but whatever
                ; b <loopa


                ; ->uwin_find_thunk:
                ; mov x0, xzr
                ; ret
            );

            test_recomp(code, expected_result);
        }
    }
}
