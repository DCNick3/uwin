mod fib;

mod sort;

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
}
