mod funs {
    test_functions! {
        msvc_rand: [
            (0), (1), (2), (3),
            (100), (200), (300),
            (-1)
        ] (
            ; mov     eax, [esp+4]
            ; imul    eax, eax, 0x343FD
            ; add     eax, 0x269EC3
            ; sar     eax, 0x10
            ; and     eax, 0x7FFF
            ; retn
        ),
    }

    test_functions! {
        // void* my_memset(void* dst, int v, size_t ln) {
        //     return memset(dst, v, ln);
        // }
        msvc_memset: [
            (0x100000, 0, 14),
            (0x100000, 12, 18),
            (0x100000, 255, 133),
            (0x100000, 0, 16),
            (0x100000, 12, 15),
            (0x100000, 255, 14),
            (0x100000, 255, 13)
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
    }

    test_functions! {
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
            (0x100000, 0, 16),
            (0x100000, 12, 15),
            (0x100000, 255, 14),
            (0x100000, 255, 13)
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
    }

    test_functions! {
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
            b"1\0",
            b"12\0",
            b"1\02",
            b"123456\02",
            b"123456\0",
            b"123456\0\0",
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
    }
}

test_pe_functions! {
    msvc_float: [
        b"11.3\0",
        b"1\0",
        b"31.1\0",
        b"1e44\0",
        b"-1.87e44\0",
        b"uwu\0",
    ]
    ("msvc_objs/float/main.exe"),
}

test_pe_functions! {
    msvc_base45: [
        (1, 0), (2, 1), (3, 64), (4, 128), (5, 256), (6, 512), (7, 1024), (8, 2048), (9, 4096),
    ]
    ("msvc_objs/base45/main.exe"),
}
