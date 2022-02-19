mod fib;
mod msvc;
mod sort;

use crate::common::MEM_ADDR;

static_assertions::const_assert_eq!(MEM_ADDR, 0x100000);

test_functions! {
    // test name
    test: [
        // arguments to test on (all are u32)
        (1, 2),
        (2, 3),
        (3, 4)
    ] (
        // test body
        ; mov eax, 42
        ; ret
    ),

    min: [
        (0, 0),
        (-1, -2),
        (100, 200),
        (-100, -200),
        (-1, -1),
        (-1, -0x80000000)
    ] (
        ; ->min:
        ;   mov     eax, DWORD [esp+4]
        ;   mov     edx, DWORD [esp+8]
        ;   cmp     eax, edx
        ;   cmovg   eax, edx
        ;   ret
    ),
    max: [
        (0, 0),
        (-1, -2),
        (100, 200),
        (-100, -200),
        (-1, -1),
        (-1, -0x80000000)
    ] (
        ; ->max:
        ; mov     eax, DWORD [esp+4]
        ; mov     edx, DWORD [esp+8]
        ; cmp     eax, edx
        ; cmovl   eax, edx
        ; ret
    ),

    byte_fiddle: [
        (0x100000, 0),
        (0x100000, 1),
        (0x100000, 0xff00),
        (0x100000, 0x0102),
        (0x100000, 0x01020304),
        (0x100000, 0xfffe1234)
    ] (
        ; ->byte_fiddle:
        ;         mov     eax, DWORD [esp+8]
        ;         mov     edx, DWORD [esp+4]
        ;         mov     ecx, eax
        ;         shr     ecx, 25
        ;         mov     BYTE [edx], cl
        ;         mov     ecx, eax
        ;         shr     ecx, 17
        ;         mov     BYTE [edx+1], cl
        ;         mov     ecx, eax
        ;         shr     eax, 1
        ;         shr     ecx, 9
        ;         mov     BYTE [edx+3], al
        ;         mov     BYTE [edx+2], cl
        ;         ret
    ),

    strlen: [
        b"\0",
        b"1\0",
        b"123\0",
        b"1234\0",
        b"1\0\04",
    ] (
        ;         jmp ->strlen

        ; ->strlen_impl:
        ;         mov     edx, DWORD [esp+4]
        ;         xor     eax, eax
        ;         cmp     BYTE [edx], 0
        ;         je      ->L4
        ; ->L3:
        ;         add     eax, 1
        ;         cmp     BYTE [edx+eax], 0
        ;         jne     ->L3
        ;         ret
        ; ->L4:
        ;         ret
        ; ->strlen:
        ;         push    ebp
        ;         mov     ebp, esp
        ;         push    esi
        ;         push    ebx
        ;         mov     ebx, DWORD [ebp+8]
        ;         lea     eax, [ebx+27]
        ;         and     eax, -16
        ;         sub     esp, eax
        ;         lea     esi, [esp+15]
        ;         and     esi, -16
        ;         test    ebx, ebx
        ;         je      ->L8
        ;         lea     edx, [ebp+12]
        ;         mov     eax, esi
        ;         add     ebx, esi
        ; ->L9:
        ;         mov     ecx, DWORD [edx]
        ;         add     eax, 1
        ;         add     edx, 4
        ;         mov     BYTE [eax-1], cl
        ;         cmp     eax, ebx
        ;         jne     ->L9
        ; ->L8:
        ;         sub     esp, 12
        ;         push    esi
        ;         call    ->strlen_impl
        ;         add     esp, 16
        ;         lea     esp, [ebp-8]
        ;         pop     ebx
        ;         pop     esi
        ;         pop     ebp
        ;         ret
    ),

    strlen_scasb: [
        b"\0",
        b"1\0",
        b"123\0",
        b"1234\0",
        b"1\0\04",
    ] (
        ;         jmp ->strlen

        ; ->strlen_impl:
        ;         xor    ecx, ecx      // ecx = 0x00000000
        ;         not    ecx           // initialize ecx to largest value possible (4,294,967,295 in 32-bit)
        ;         xor    al, al        // al = 0x00
        ;         mov    edi, [esp+4]  // edi points to start of string
        ;         mov    ebx, edi      // store original pointer
        ;         repne  scasb         // repeatedly compare bytes
        ;         sub    edi, ebx      // subtract to get length + 1
        ;         dec    edi           // decrement edi
        ;         mov    eax, edi

        ; ->L4:
        ;         ret
        ; ->strlen:
        ;         push    ebp
        ;         mov     ebp, esp
        ;         push    esi
        ;         push    ebx
        ;         mov     ebx, DWORD [ebp+8]
        ;         lea     eax, [ebx+27]
        ;         and     eax, -16
        ;         sub     esp, eax
        ;         lea     esi, [esp+15]
        ;         and     esi, -16
        ;         test    ebx, ebx
        ;         je      ->L8
        ;         lea     edx, [ebp+12]
        ;         mov     eax, esi
        ;         add     ebx, esi
        ; ->L9:
        ;         mov     ecx, DWORD [edx]
        ;         add     eax, 1
        ;         add     edx, 4
        ;         mov     BYTE [eax-1], cl
        ;         cmp     eax, ebx
        ;         jne     ->L9
        ; ->L8:
        ;         sub     esp, 12
        ;         push    esi
        ;         call    ->strlen_impl
        ;         add     esp, 16
        ;         lea     esp, [ebp-8]
        ;         pop     ebx
        ;         pop     esi
        ;         pop     ebp
        ;         ret
    ),

}
