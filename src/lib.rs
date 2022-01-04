mod backend;

use iced_x86::Decoder;



#[cfg(test)]
mod tests {
    /// use dynasm to assemble the provided code to a Vec<u8>
    macro_rules! assemble {
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
            let result = assemble!(
                ; mov eax, 42
            );
            assert_eq!(result, b"\xB8\x2A\x00\x00\x00");
        }
    }


}
