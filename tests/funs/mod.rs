mod fib;
mod sort;

use crate::common::MEM_ADDR;

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
        // negative values can't be put here because it can't be part of a valid identifier
        // TODO: maybe we can patch the crate that does it/find another/???
        (0, 0),
        (0xffffffff, 0xfffffffe),
        (100, 200),
        (0xffffff9c, 0xffffff38),
        (0xffffffff, 0xffffffff),
        (0xffffffff, 0x80000000)
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
        (0xffffffff, 0xfffffffe),
        (100, 200),
        (0xffffff9c, 0xffffff38),
        (0xffffffff, 0xffffffff),
        (0xffffffff, 0x80000000)
    ] (
        ; ->max:
        ; mov     eax, DWORD [esp+4]
        ; mov     edx, DWORD [esp+8]
        ; cmp     eax, edx
        ; cmovl   eax, edx
        ; ret
    ),

    byte_fiddle: [
        (MEM_ADDR, 0),
        (MEM_ADDR, 1),
        (MEM_ADDR, 0xff00),
        (MEM_ADDR, 0x0102),
        (MEM_ADDR, 0x01020304),
        (MEM_ADDR, 0xfffe1234)
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
}
