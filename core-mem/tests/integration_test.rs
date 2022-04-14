use bencher::black_box;
use core_mem::conv::FromIntoMemory;
use core_mem::ctx::FlatMemoryCtx;
use core_mem::ptr::{MutPtr, PtrRepr};
use core_mem::thread_ctx::set_thread_ctx;
use region::{Allocation, Protection};

// TODO: most probably the region crate does not provide the level of control required
// should research if it is so and maybe create smth NIH that fits
struct DummyFlatMemoryManager {
    base_allocation: Allocation,
    // TODO: do we have to store them or can we clear all the address space in one go?
    // AFAIK it is possible on linux, not sure about other platforms though
    child_allocations: Vec<Allocation>,
}

impl DummyFlatMemoryManager {
    fn new() -> Self {
        let alloc = region::alloc(1 << 32, Protection::NONE).unwrap();
        Self {
            base_allocation: alloc,
            child_allocations: Vec::new(),
        }
    }

    fn to_native_ptr(&self, ptr: PtrRepr) -> *mut u8 {
        self.get_ctx().to_native_ptr(ptr)
    }

    fn map_rw(&mut self, ptr: PtrRepr, len: PtrRepr) -> PtrRepr {
        let addr = self.to_native_ptr(ptr);
        let alloc = region::alloc_at(addr, len as usize, Protection::READ_WRITE).unwrap();
        self.child_allocations.push(alloc);

        ptr
    }

    fn get_ctx(&self) -> FlatMemoryCtx {
        unsafe { FlatMemoryCtx::new(self.base_allocation.as_ptr::<u8>() as *mut u8) }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct TestStruct {
    val1: u8,
    val2: u32,
    val3: u64,
}

impl FromIntoMemory for TestStruct {
    fn try_from_bytes(from: &[u8]) -> Self {
        // TODO: probably want some helpers to make this less verbose
        Self {
            val1: u8::try_from_bytes(&from[0..1]),
            val2: u32::try_from_bytes(&from[1..5]),
            val3: u64::try_from_bytes(&from[5..13]),
        }
    }

    fn try_into_bytes(self, into: &mut [u8]) {
        self.val1.try_into_bytes(&mut into[0..1]);
        self.val2.try_into_bytes(&mut into[1..5]);
        self.val3.try_into_bytes(&mut into[5..13]);
    }

    fn size() -> usize {
        1 + 4 + 8
    }
}

/// rust optimizes this down to
///  mov eax, esi
///  mov [rdi + rax], edx
///  ret
/// (nice)
/// !!! this is no longer true when thread-local context was introduced
/// TODO: investigate possible improvements
#[inline(never)]
fn write_int(mut_ptr: &MutPtr<u32>, val: u32) {
    mut_ptr.write(val);
}

/// rust optimizes this down to
///  mov eax, esi
///  mov eax, [rdi + rax]
///  ret
/// (nice)
/// !!! this is no longer true when thread-local context was introduced
/// TODO: investigate possible improvements
#[inline(never)]
fn read_int(mut_ptr: &MutPtr<u32>) -> u32 {
    mut_ptr.read()
}

/// rust optimizes this down to
///  mov rax, rcx
///  shr rax, 32
///  mov esi, esi
///  mov [rdi + rsi], al
///  mov [rdi + rsi + 1], ecx
///  mov [rdi + rsi + 5], rdx
///  ret
/// (nice)
/// note: on aarch64 the result is actually suboptimal (the compiler generates dead stores to the stack)
/// !!! this is no longer true when thread-local context was introduced
/// TODO: investigate possible improvements
#[inline(never)]
fn write_struct(mut_ptr: &MutPtr<TestStruct>, val: TestStruct) {
    mut_ptr.write(val)
}

/// rust optimizes this down to
///  mov   eax, esi
///  movzx edx, [rdi + rax]
///  mov   ecx, [rdi + rax + 1]
///  mov   rax, [rdi + rax + 5]
///  shl   rdx, 32
///  or    rdx, rcx
///  ret
/// (nice)
/// !!! this is no longer true when thread-local context was introduced
/// TODO: investigate possible improvements
#[inline(never)]
fn read_struct(mut_ptr: &MutPtr<TestStruct>) -> TestStruct {
    mut_ptr.read()
}

#[test]
fn simple_int_fiddle() {
    let mut mgr = DummyFlatMemoryManager::new();

    set_thread_ctx(mgr.get_ctx());

    // map one page to play around with
    let page = mgr.map_rw(0x37000, 0x1000);

    let mut_ptr = MutPtr::<u32>::new(page);

    write_int(black_box(&mut_ptr), black_box(12));
    assert_eq!(read_int(black_box(&mut_ptr)), 12);
}

#[test]
fn simple_struct_fiddle() {
    let mut mgr = DummyFlatMemoryManager::new();

    set_thread_ctx(mgr.get_ctx());

    // map one page to play around with
    let page = mgr.map_rw(0x37000, 0x1000);

    let mut_ptr = MutPtr::<TestStruct>::new(page);

    let test_struct = TestStruct {
        val1: 12,
        val2: 3456,
        val3: 7890,
    };

    write_struct(black_box(&mut_ptr), black_box(test_struct));
    assert_eq!(read_struct(black_box(&mut_ptr)), test_struct);
}
