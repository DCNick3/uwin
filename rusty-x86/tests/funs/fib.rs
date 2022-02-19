test_functions! {
    fib_recur: [
        (0), (1), (2), (3), (4),
        (5), (6), (7), (8), (9),
        (10), (20)
    ] (
        ; ->fibonacci:                              //  @fibonacci
        ;         push    edi
        ;         push    esi
        ;         push    eax
        ;         mov     edi, [esp + 16]
        ;         xor     esi, esi
        ;         cmp     edi, 2
        ;         jb      ->LBB0_3
        ;         xor     esi, esi
        ; ->LBB0_2:                                // =>This Inner Loop Header: Depth=1
        ;         lea     eax, [edi - 1]
        ;         mov     [esp], eax
        ;         call    ->fibonacci
        ;         add     edi, -2
        ;         add     esi, eax
        ;         cmp     edi, 1
        ;         ja      ->LBB0_2
        ; ->LBB0_3:
        ;         add     esi, edi
        ;         mov     eax, esi
        ;         add     esp, 4
        ;         pop     esi
        ;         pop     edi
        ;         ret
    ),
    fib_smartass: [
        (0), (1), (2), (3), (4),
        (5), (6), (7), (8), (9),
        (10), (20), (100), (1000), (10000)
    ] (
        ; ->fib:
        ;         sub     esp, 28
        ;         mov     eax, [esp+32]
        ;         mov     DWORD [esp], 1
        ;         mov     DWORD [esp+4], 1
        ;         mov     DWORD [esp+8], 1
        ;         mov     DWORD [esp+12], 0
        ;         test    eax, eax
        ;         je      ->L13
        ;         sub     esp, 8
        ;         sub     eax, 1
        ;         push    eax
        ;         lea     eax, [esp+12]
        ;         push    eax
        ;         call    ->power
        ;         mov     eax, [esp+16]
        ;         add     esp, 16
        ; ->L13:
        ;         add     esp, 28
        ;         ret

        ; ->multiply:
        ;         push    ebp
        ;         push    edi
        ;         push    esi
        ;         push    ebx
        ;         sub     esp, 8
        ;         mov     edi, [esp+32]
        ;         mov     eax, [esp+28]
        ;         mov     edx, [edi+4]
        ;         mov     ebp, [edi+12]
        ;         mov     ecx, [edi]
        ;         mov     esi, [edi+8]
        ;         mov     ebx, [eax]
        ;         mov     edi, [eax+8]
        ;         mov     [esp], edx
        ;         mov     edx, [eax+4]
        ;         mov     [esp+4], edi
        ;         mov     edi, ebx
        ;         imul    ebx, [esp]
        ;         imul    edx, esi
        ;         imul    edi, ecx
        ;         imul    esi, [eax+12]
        ;         add     edi, edx
        ;         mov     edx, [eax+4]
        ;         mov     [eax], edi
        ;         imul    edx, ebp
        ;         add     ebx, edx
        ;         mov     edx, [esp]
        ;         mov     [eax+4], ebx
        ;         mov     ebx, [esp+4]
        ;         imul    ecx, ebx
        ;         imul    edx, ebx
        ;         add     ecx, esi
        ;         mov     [eax+8], ecx
        ;         mov     ecx, [eax+12]
        ;         imul    ecx, ebp
        ;         add     edx, ecx
        ;         mov     [eax+12], edx
        ;         add     esp, 8
        ;         pop     ebx
        ;         pop     esi
        ;         pop     edi
        ;         pop     ebp
        ;         ret

        ; ->power:
        ;         push    esi
        ;         push    ebx
        ;         sub     esp, 4
        ;         mov     esi, [esp+20]
        ;         mov     ebx, [esp+16]
        ;         cmp     esi, 1
        ;         jbe     ->L4
        ;         mov     eax, esi
        ;         sub     esp, 8
        ;         shr     eax, 31
        ;         add     eax, esi
        ;         sar     eax, 1
        ;         push    eax
        ;         push    ebx
        ;         call    ->power
        ;         pop     eax
        ;         pop     edx
        ;         push    ebx
        ;         push    ebx
        ;         call    ->multiply
        ;         add     esp, 16
        ;         and     esi, 1
        ;         jne     ->L11
        ; ->L4:
        ;         add     esp, 4
        ;         pop     ebx
        ;         pop     esi
        ;         ret
        ; ->L11:
        ;         mov     edx, [ebx]
        ;         mov     eax, [ebx+8]
        ;         mov     ecx, [ebx+12]
        ;         mov     esi, [ebx+4]
        ;         mov     [ebx+12], eax
        ;         add     esi, edx
        ;         add     ecx, eax
        ;         mov     [ebx+4], edx
        ;         mov     [ebx], esi
        ;         mov     [ebx+8], ecx
        ;         add     esp, 4
        ;         pop     ebx
        ;         pop     esi
        ;         ret
    ),
}
