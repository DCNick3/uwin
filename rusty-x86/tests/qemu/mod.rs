mod qemu_x86_test {
    const ELF: &[u8] = include_bytes!("qemu-x86-tests/test-i386");

    #[test_log::test]
    fn llvm() {
        log::info!("Running qemu_x86_test::llvm");

        crate::common::test_outcode(ELF, crate::common::Backend::Llvm);
    }

    #[test_log::test]
    fn interp() {
        log::info!("Running qemu_x86_test::interp");

        crate::common::test_outcode(ELF, crate::common::Backend::Interp);
    }
}
