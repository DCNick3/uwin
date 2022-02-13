use crate::common::MEM_ADDR;

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

    // void* my_memset(void* dst, int v, size_t ln) {
    //     return memset(dst, v, ln);
    // }
    msvc_memset: [
        (MEM_ADDR, 0, 14),
        (MEM_ADDR, 12, 18),
        (MEM_ADDR, 255, 133)
    ] (
        ; mov     al, BYTE [esp-4+12]
        ; mov     ecx, DWORD [esp-4+16]
        ; push    ebx
        ; mov     bl, al
        ; mov     bh, bl
        ; push    esi
        ; mov     eax, ebx
        ; push    edi
        ; mov     edi, DWORD [esp+8+8]
        ; mov     edx, ecx
        ; shl     eax, 16
        ; mov     ax, bx
        ; mov     esi, edi
        ; shr     ecx, 2
        ; rep stosd
        ; mov     ecx, edx
        ; and     ecx, 3
        ; rep stosb
        ; mov     eax, esi
        ; pop     edi
        ; pop     esi
        ; pop     ebx
        ; ret     0
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
