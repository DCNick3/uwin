#[test_log::test]
fn qemu_x86_test() {
    log::info!("Running qemu_x86_test");

    let elf = include_bytes!("qemu-x86-tests/test-i386");

    crate::common::test_outcode(elf);
}
