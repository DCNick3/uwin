test_functions!(
    msvc_rand: [
        (0), (1), (2), (3),
        (100), (200), (300),
        (0xffffffff)
    ] (
        ; mov     eax, [esp+4]
        ; imul    eax, eax, 0x343FD
        ; add     eax, 0x269EC3
        ; sar     eax, 0x10
        ; and     eax, 0x7FFF
        ; retn
    ),
);

#[test_log::test]
#[allow(non_snake_case)]
fn bla() {
    let args: &[u32] = &[5, '1' as u32, '1' as u32, '.' as u32, '3' as u32, 0];
    log::info!("Running {} on {:?}", stringify!($name), args);

    let elf = include_bytes!("msvc_float");

    crate::common::test_code(crate::common::CodeToTest::ElfFunction(elf, args), vec![]);
}
