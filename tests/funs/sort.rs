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

test_functions! {
    qsort: [
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
        ;         jmp ->qsort

        ; ->partition:
        ;         push    ebp
        ;         push    edi
        ;         push    esi
        ;         push    ebx
        ;         mov     edi, DWORD [esp+20]
        ;         mov     edx, DWORD [esp+28]
        ;         mov     eax, DWORD [esp+24]
        ;         lea     esi, [edi+edx*4]
        ;         mov     edi, DWORD [esi]
        ;         cmp     edx, eax
        ;         jle     ->L8
        ;         mov     ebx, DWORD [esp+20]
        ;         lea     ecx, [eax-1]
        ;         lea     eax, [ebx+eax*4]
        ; ->L7:
        ;         mov     edx, DWORD [eax]
        ;         cmp     edx, edi
        ;         jge     ->L6
        ;         mov     ebx, DWORD [esp+20]
        ;         add     ecx, 1
        ;         lea     ebx, [ebx+ecx*4]
        ;         mov     ebp, DWORD [ebx]
        ;         mov     DWORD [ebx], edx
        ;         mov     DWORD [eax], ebp
        ; ->L6:
        ;         add     eax, 4
        ;         cmp     esi, eax
        ;         jne     ->L7
        ;         add     ecx, 1
        ;         mov     edi, DWORD [esi]
        ;         mov     eax, ecx
        ; ->L5:
        ;         mov     ebx, DWORD [esp+20]
        ;         lea     edx, [ebx+ecx*4]
        ;         mov     ecx, DWORD [edx]
        ;         mov     DWORD [edx], edi
        ;         mov     DWORD [esi], ecx
        ;         pop     ebx
        ;         pop     esi
        ;         pop     edi
        ;         pop     ebp
        ;         ret
        ; ->L8:
        ;         mov     ecx, eax
        ;         jmp     ->L5
        ; ->qsort_impl:
        ;         push    ebp
        ;         push    edi
        ;         push    esi
        ;         push    ebx
        ;         sub     esp, 76
        ;         mov     ebp, DWORD [esp+96]
        ;         mov     edi, DWORD [esp+100]
        ;         cmp     DWORD [esp+104], edi
        ;         jle     ->L11
        ;         mov     edi, ebp
        ;         mov     ebp, DWORD [esp+100]
        ; ->L29:
        ;         push    DWORD [esp+104]
        ;         push    ebp
        ;         push    edi
        ;         call    ->partition
        ;         mov     DWORD [esp+48], eax
        ;         sub     eax, 1
        ;         mov     DWORD [esp+36], eax
        ;         add     esp, 12
        ;         cmp     eax, ebp
        ;         jle     ->L13
        ; ->L28:
        ;         push    DWORD [esp+24]
        ;         push    ebp
        ;         push    edi
        ;         call    ->partition
        ;         mov     DWORD [esp+52], eax
        ;         sub     eax, 1
        ;         mov     DWORD [esp+40], eax
        ;         add     esp, 12
        ;         cmp     eax, ebp
        ;         jle     ->L14
        ; ->L27:
        ;         push    DWORD [esp+28]
        ;         push    ebp
        ;         push    edi
        ;         call    ->partition
        ;         mov     DWORD [esp+56], eax
        ;         sub     eax, 1
        ;         mov     DWORD [esp+44], eax
        ;         add     esp, 12
        ;         cmp     eax, ebp
        ;         jle     ->L15
        ; ->L26:
        ;         push    DWORD [esp+32]
        ;         push    ebp
        ;         push    edi
        ;         call    ->partition
        ;         mov     DWORD [esp+60], eax
        ;         sub     eax, 1
        ;         mov     DWORD [esp+16], eax
        ;         add     esp, 12
        ;         cmp     eax, ebp
        ;         jle     ->L16
        ;         mov     esi, ebp
        ;         mov     ebp, edi
        ; ->L25:
        ;         push    DWORD [esp+4]
        ;         push    esi
        ;         push    ebp
        ;         call    ->partition
        ;         mov     edi, eax
        ;         lea     eax, [eax-1]
        ;         mov     DWORD [esp+20], eax
        ;         add     esp, 12
        ;         cmp     eax, esi
        ;         jle     ->L17
        ;         mov     DWORD [esp+12], edi
        ; ->L24:
        ;         push    DWORD [esp+8]
        ;         push    esi
        ;         push    ebp
        ;         call    ->partition
        ;         mov     DWORD [esp+28], eax
        ;         sub     eax, 1
        ;         mov     DWORD [esp+32], eax
        ;         add     esp, 12
        ;         cmp     eax, esi
        ;         jle     ->L18
        ; ->L23:
        ;         push    DWORD [esp+20]
        ;         push    esi
        ;         push    ebp
        ;         call    ->partition
        ;         lea     edi, [eax-1]
        ;         mov     DWORD [esp+64], eax
        ;         add     esp, 12
        ;         cmp     edi, esi
        ;         jle     ->L19
        ; ->L22:
        ;         push    edi
        ;         push    esi
        ;         push    ebp
        ;         call    ->partition
        ;         add     esp, 12
        ;         lea     edx, [eax-1]
        ;         mov     ebx, eax
        ;         cmp     edx, esi
        ;         jle     ->L20
        ;         mov     ecx, eax
        ;         mov     ebx, esi
        ; ->L21:
        ;         mov     DWORD [esp+60], ecx
        ;         push    edx
        ;         mov     DWORD [esp+60], edx
        ;         push    ebx
        ;         push    ebp
        ;         call    ->partition
        ;         sub     esp, 8
        ;         mov     esi, eax
        ;         lea     eax, [eax-1]
        ;         push    eax
        ;         push    ebx
        ;         lea     ebx, [esi+1]
        ;         push    ebp
        ;         call    ->qsort_impl
        ;         mov     edx, DWORD [esp+88]
        ;         add     esp, 32
        ;         mov     ecx, DWORD [esp+60]
        ;         cmp     edx, ebx
        ;         jg      ->L21
        ;         mov     ebx, ecx
        ; ->L20:
        ;         lea     esi, [ebx+1]
        ;         cmp     edi, esi
        ;         jg      ->L22
        ; ->L19:
        ;         mov     esi, DWORD [esp+52]
        ;         add     esi, 1
        ;         cmp     DWORD [esp+20], esi
        ;         jg      ->L23
        ; ->L18:
        ;         mov     esi, DWORD [esp+16]
        ;         add     esi, 1
        ;         cmp     DWORD [esp+8], esi
        ;         jg      ->L24
        ;         mov     edi, DWORD [esp+12]
        ; ->L17:
        ;         lea     esi, [edi+1]
        ;         cmp     DWORD [esp+4], esi
        ;         jg      ->L25
        ;         mov     edi, ebp
        ; ->L16:
        ;         mov     ebp, DWORD [esp+48]
        ;         add     ebp, 1
        ;         cmp     DWORD [esp+32], ebp
        ;         jg      ->L26
        ; ->L15:
        ;         mov     ebp, DWORD [esp+44]
        ;         add     ebp, 1
        ;         cmp     DWORD [esp+28], ebp
        ;         jg      ->L27
        ; ->L14:
        ;         mov     ebp, DWORD [esp+40]
        ;         add     ebp, 1
        ;         cmp     DWORD [esp+24], ebp
        ;         jg      ->L28
        ; ->L13:
        ;         mov     ebp, DWORD [esp+36]
        ;         add     ebp, 1
        ;         cmp     ebp, DWORD [esp+104]
        ;         jl      ->L29
        ; ->L11:
        ;         add     esp, 76
        ;         pop     ebx
        ;         pop     esi
        ;         pop     edi
        ;         pop     ebp
        ;         ret
        ; ->qsort:
        ;         push    ebp
        ;         mov     ebp, esp
        ;         push    edi
        ;         push    esi
        ;         push    ebx
        ;         sub     esp, 28
        ;         mov     esi, DWORD [ebp+8]
        ;         lea     edi, [0+esi*4]
        ;         lea     eax, [edi+27]
        ;         and     eax, -16
        ;         sub     esp, eax
        ;         lea     eax, [ebp+12]
        ;         lea     ebx, [esp+15]
        ;         add     edi, eax
        ;         and     ebx, -16
        ;         test    esi, esi
        ;         je      ->L40
        ;         mov     edx, ebx
        ; ->L42:
        ;         mov     ecx, DWORD [eax]
        ;         add     eax, 4
        ;         add     edx, 4
        ;         mov     DWORD [edx-4], ecx
        ;         cmp     eax, edi
        ;         jne     ->L42
        ;         sub     esi, 1
        ;         mov     edx, esi
        ;         je      ->L40
        ;         xor     edi, edi
        ; ->L44:
        ;         sub     esp, 4
        ;         mov     DWORD [ebp-28], edx
        ;         push    edx
        ;         push    edi
        ;         push    ebx
        ;         call    ->partition
        ;         add     esp, 12
        ;         mov     esi, eax
        ;         lea     eax, [eax-1]
        ;         push    eax
        ;         push    edi
        ;         lea     edi, [esi+1]
        ;         push    ebx
        ;         call    ->qsort_impl
        ;         mov     edx, DWORD [ebp-28]
        ;         add     esp, 16
        ;         cmp     edx, edi
        ;         jg      ->L44
        ; ->L40:
        ;         lea     esp, [ebp-12]
        ;         pop     ebx
        ;         pop     esi
        ;         pop     edi
        ;         pop     ebp
        ;         ret
    ),
}
