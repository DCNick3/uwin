// I use varargs to pass arrays
// first arg tells the size, the remaining ones are the numbers to sort
test_functions! {
    bsort: [
        (0),
        (1, 1),
        (1, 0xffffffff),
        (2, 1, 2),
        (2, 2, 1),
        (2, 1, 1),
        (2, 1, 0xffffffff),
        (3, 1, 2, 3),
        (3, 3, 2, 1),
        (3, 1, 3, 2),
        (3, 3, 0xffffffff, 1),
        (10, 9, 10, 1, 6, 8, 3, 2, 5, 4, 7)
    ] (
        ; ->bsort:
        ;         push    ebp
        ;         mov     ebp, esp
        ;         push    edi
        ;         push    esi
        ;         push    ebx
        ;         sub     esp, 12
        ;         mov     edi, DWORD [ebp+8]
        ;         lea     ebx, [0+edi*4]
        ;         lea     eax, [ebx+27]
        ;         and     eax, -16
        ;         sub     esp, eax
        ;         lea     esi, [esp+15]
        ;         and     esi, -16
        ;         test    edi, edi
        ;         je      ->L17
        ;         lea     eax, [ebp+12]
        ;         mov     edx, esi
        ;         add     ebx, eax
        ; ->L19:
        ;         mov     ecx, DWORD [eax]
        ;         add     eax, 4
        ;         add     edx, 4
        ;         mov     DWORD [edx-4], ecx
        ;         cmp     eax, ebx
        ;         jne     ->L19
        ;         sub     esp, 8
        ;         push    edi
        ;         push    esi
        ;         call    ->bsort_impl_part_0
        ;         add     esp, 16
        ; ->L17:
        ;         lea     esp, [ebp-12]
        ;         pop     ebx
        ;         pop     esi
        ;         pop     edi
        ;         pop     ebp
        ;         ret

        ; ->bsort_impl_part_0:
        ;         push    edi
        ;         push    esi
        ;         push    ebx
        ;         mov     esi, DWORD [esp+20]
        ;         mov     edi, DWORD [esp+16]
        ;         sub     esi, 1
        ;         je      ->L1
        ;         lea     ebx, [edi+esi*4]
        ; ->L3:
        ;         mov     eax, edi
        ; ->L5:
        ;         mov     edx, DWORD [eax]
        ;         mov     ecx, DWORD [eax+4]
        ;         cmp     edx, ecx
        ;         jle     ->L4
        ;         mov     DWORD [eax], ecx
        ;         mov     DWORD [eax+4], edx
        ; ->L4:
        ;         add     eax, 4
        ;         cmp     eax, ebx
        ;         jne     ->L5
        ;         sub     ebx, 4
        ;         sub     esi, 1
        ;         jne     ->L3
        ; ->L1:
        ;         pop     ebx
        ;         pop     esi
        ;         pop     edi
        ;         ret
    ),
}
