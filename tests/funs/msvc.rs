mod funs {
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
    );

    test_functions!(
        // void* my_memset(void* dst, int v, size_t ln) {
        //     return memset(dst, v, ln);
        // }
        msvc_memset: [
            (MEM_ADDR, 0, 14),
            (MEM_ADDR, 12, 18),
            (MEM_ADDR, 255, 133),
            (MEM_ADDR, 0, 16),
            (MEM_ADDR, 12, 15),
            (MEM_ADDR, 255, 14),
            (MEM_ADDR, 255, 13)
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

    test_functions!(
        // void* my_memcpy(unsigned char* buf, unsigned seed, size_t ln) {
        //     int i;
        //     for (i = 0; i < ln; i++) {
        //         buf[i] = seed % 256;
        //         // https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use
        //         // Borland C++
        //         seed = 22695477 * seed + 1;
        //     }
        //
        //     return memcpy(buf + ln, buf, ln);
        // }
        msvc_memcpy: [
            (MEM_ADDR, 0, 16),
            (MEM_ADDR, 12, 15),
            (MEM_ADDR, 255, 14),
            (MEM_ADDR, 255, 13)
        ] (
    ;->my_memcpy:
            ; mov     ecx, DWORD [esp-4+16]
            ; xor     eax, eax
            ; push    esi
            ; mov     esi, DWORD [esp+8]
            ; test    ecx, ecx
            ; push    edi
            ; jbe     ->L355
            ; mov     edx, DWORD [esp+4+12]
        ;->L353:
            ; mov     BYTE [eax+esi], dl
            ; imul    edx, edx, 0x15a4e35
            ; inc     edx
            ; inc     eax
            ; cmp     eax, ecx
            ; jb      ->L353
        ;->L355:
            ; lea     edi, [esi+ecx]
            ; mov     edx, ecx
            ; mov     eax, edi
            ; shr     ecx, 2
            ; rep movsd
            ; mov     ecx, edx
            ; and     ecx, 3
            ; rep movsb
            ; pop     edi
            ; pop     esi
            ; ret     0
        ),
    );

    test_functions!(
        // size_t my_strlen(size_t count, ...) {
        //     char* buffer;
        //     va_list argp;
        //     size_t i;
        //     size_t r;
        //     va_start(argp, count);
        //
        //     buffer = alloca(count);
        //     for (i = 0; i < count; i++) {
        //         buffer[i] = va_arg(argp, int);
        //     }
        //
        //     r = strlen(buffer);
        //
        //     va_end(argp);
        //
        //     return r;
        // }
        msvc_strlen: [
            (0),
            (1, 1),
            (1, 0),
            (5, 1, 2, 3, 4, 0),
            (4, 1, 2, 3, 0),
            (4, 1, 0, 0, 4)
        ] (
    ; ->my_strlen:
            ; push    ebp
            ; mov     ebp, esp
            ; push    esi
            ; mov     esi, DWORD [ebp+8]
            ; mov     eax, esi
            ; push    edi
            ; add     eax, 3
            ; and     al, -4
            ; call    ->__alloca_probe
            ; xor     eax, eax
            ; mov     edi, esp
            ; test    esi, esi
            ; jbe     ->L400
            ; lea     ecx, [ebp+8]
        ; ->L398:
            ; mov     dl, BYTE [ecx+4]
            ; add     ecx, 4
            ; mov     BYTE [eax+edi], dl
            ; inc     eax
            ; cmp     eax, esi
            ; jb      ->L398
        ; ->L400:
            ; or      ecx, -1
            ; xor     eax, eax
            ; repne scasb
            ; not     ecx
            ; lea     esp, [ebp-8]
            ; dec     ecx
            ; mov     eax, ecx
            ; pop     edi
            ; pop     esi
            ; pop     ebp
            ; ret     0

    ; ->__alloca_probe:
            ; push    ecx                     // save ecx
            ; cmp     eax, 0x1000             // more than one page requested?
            ; lea     ecx, [esp + 8]          //   compute new stack pointer in ecx
                                              //   correct for return address and
                                              //   saved ecx
            ; jb      ->lastpage              // no

        ; ->probepages:
            ; sub     ecx, 0x1000             // yes, move down a page
            ; sub     eax, 0x1000             // adjust request and...
            ; test    DWORD [ecx],eax         // ...probe it
            ; cmp     eax, 0x1000             // more than one page requested?
            ; jae     ->probepages            // no
        ; ->lastpage:
            ; sub     ecx, eax                // move stack down by eax
            ; mov     eax, esp                // save current tos and do a...

            ; test    DWORD [ecx], eax        // ...probe in case a page was crossed

            ; mov     esp,ecx                 // set the new stack pointer

            ; mov     ecx, DWORD [eax]        // recover ecx
            ; mov     eax, DWORD [eax + 4]    // recover return address

            ; push    eax                     // prepare return address
                                              // ...probe in case a page was crossed
            ; ret
        ),
    );
}

#[test_log::test]
#[allow(non_snake_case)]
fn bla() {
    let args: &[u32] = &[5, '1' as u32, '1' as u32, '.' as u32, '3' as u32, 0];
    log::info!("Running {} on {:?}", stringify!($name), args);

    let elf = include_bytes!("msvc_objs/float/msvc_float");

    crate::common::test_code(crate::common::CodeToTest::ElfFunction(elf, args), vec![]);
}
