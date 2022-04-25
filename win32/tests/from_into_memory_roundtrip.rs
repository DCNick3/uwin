use core_mem::conv::FromIntoMemory;
use core_mem::ptr::MutPtr;
use core_str::PSTR;
use std::fmt::Debug;
use win32::Win32::Foundation::HANDLE;
use win32::Win32::System::Memory::{HEAP_NO_SERIALIZE, HEAP_ZERO_MEMORY};
use win32::Win32::System::Threading::{STARTF_TITLEISAPPID, STARTF_USEHOTKEY, STARTUPINFOA};

fn test_roundtrip<T: FromIntoMemory + Clone + PartialEq + Debug>(value: T) {
    let mut memory = vec![0u8; T::size()];
    let value_copy = value.clone();
    value.into_bytes(&mut memory);
    let value = T::from_bytes(&memory);
    assert_eq!(value, value_copy);
}

#[test]
fn integers() {
    test_roundtrip(0u8);
    test_roundtrip(42u8);

    test_roundtrip(0u32);
    test_roundtrip(42u32);
    test_roundtrip(0x12131415u32);

    test_roundtrip(0i64);
    test_roundtrip(42i64);
    test_roundtrip(0x12131415i64);
    test_roundtrip(-0x12151712131415i64);
}

#[test]
fn enums() {
    test_roundtrip(HEAP_NO_SERIALIZE);
    test_roundtrip(HEAP_NO_SERIALIZE | HEAP_ZERO_MEMORY);
}

#[test]
fn structs() {
    test_roundtrip(STARTUPINFOA {
        cb: 68,
        lpReserved: PSTR(MutPtr::new(0x453f9755)),
        lpDesktop: PSTR(MutPtr::new(0x490fd900)),
        lpTitle: PSTR(MutPtr::new(0x9416c258)),
        dwX: 0xa20f723b,
        dwY: 0xb3c8a2b9,
        dwXSize: 0x7783b309,
        dwYSize: 0x7ce8f736,
        dwXCountChars: 0xfb2c30bd,
        dwYCountChars: 0xec6ec1d1,
        dwFillAttribute: 0,
        dwFlags: STARTF_TITLEISAPPID | STARTF_USEHOTKEY,
        wShowWindow: 0xcc53,
        cbReserved2: 0xc212,
        lpReserved2: MutPtr::new(0x769e3548),
        hStdInput: HANDLE(0xca747761),
        hStdOutput: HANDLE(0x3b7d0639),
        hStdError: HANDLE(0x7d7b25ba),
    });
}
