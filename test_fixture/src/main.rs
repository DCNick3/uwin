use recompiler::memory_image::{MemoryImageItem, Protection};
use region::Allocation;
use rusty_x86_runtime::{CpuContext, FlatMemoryCtx, StdCallHelper, PROGRAM_IMAGE};
use std::ffi::CStr;
use std::io::Write;
use std::os::raw::c_char;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process::abort;
use win32::core::PCSTR;
use win32::Win32::Foundation::HWND;
use win32::Win32::UI::WindowsAndMessaging::{Api, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE};
use win32_mem::thread_ctx::{get_thread_ctx, set_thread_ctx};

struct WindowsAndMessaging {}

#[allow(non_snake_case)]
impl win32::Win32::UI::WindowsAndMessaging::Api for WindowsAndMessaging {
    fn MessageBoxA(
        &self,
        hWnd: HWND,
        lpText: ::win32::core::PCSTR,
        lpCaption: ::win32::core::PCSTR,
        uType: MESSAGEBOX_STYLE,
    ) -> MESSAGEBOX_RESULT {
        let memory = get_thread_ctx();

        let text = unsafe { CStr::from_ptr(memory.to_native_ptr(lpText.0) as *const c_char) }
            .to_str()
            .unwrap();
        let caption = unsafe { CStr::from_ptr(memory.to_native_ptr(lpCaption.0) as *const c_char) }
            .to_str()
            .unwrap();

        println!(
            "MessageBoxA({:?}, {:?}, {:?}, {:?})",
            hWnd, text, caption, uType
        );

        MESSAGEBOX_RESULT(0)
    }
}

#[no_mangle]
extern "C" fn magic_MessageBoxA(context: &mut CpuContext, memory: FlatMemoryCtx) {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let mut call = StdCallHelper::new(memory, context);

        let h_wnd = call.get_arg();
        let lp_text = call.get_arg();
        let lp_caption = call.get_arg();
        let u_type = call.get_arg();

        let api = WindowsAndMessaging {};

        let res = api.MessageBoxA(h_wnd, PCSTR(lp_text), PCSTR(lp_caption), u_type);

        call.finish(res);
    }));

    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");

        abort();
    }
}

pub struct MemoryMapper {
    base_region: Allocation,
    subregions: Vec<Allocation>,
}

impl MemoryMapper {
    pub fn new() -> region::Result<MemoryMapper> {
        Ok(Self {
            base_region: region::alloc(1 << 32, region::Protection::NONE)?,
            subregions: Vec::new(),
        })
    }

    unsafe fn flat_memory_ctx(&mut self) -> FlatMemoryCtx {
        FlatMemoryCtx::new(self.base_region.as_mut_ptr())
    }

    fn conv_prot(prot: Protection) -> region::Protection {
        let mut res = region::Protection::NONE;
        if prot.contains(Protection::READ) {
            res |= region::Protection::READ
        }
        if prot.contains(Protection::WRITE) {
            res |= region::Protection::WRITE
        }
        if prot.contains(Protection::EXECUTE) {
            res |= region::Protection::EXECUTE
        }
        res
    }

    pub fn map_item(&mut self, item: &MemoryImageItem) -> region::Result<()> {
        let addr = unsafe { self.flat_memory_ctx() }.to_native_ptr(item.addr);
        let mut alloc = region::alloc_at(addr, item.data.len(), region::Protection::READ_WRITE)?;

        let mut region =
            unsafe { std::slice::from_raw_parts_mut::<u8>(alloc.as_mut_ptr(), item.data.len()) };
        region.write_all(&item.data).unwrap();

        unsafe { region::protect(addr, item.data.len(), Self::conv_prot(item.protection))? };
        self.subregions.push(alloc);
        Ok(())
    }

    pub fn map(&mut self, size: u32, protection: Protection) -> region::Result<u32> {
        // TODO: this is not a general-purpose algorithm
        let addr = self
            .subregions
            .iter()
            .map(|v| v.as_ptr::<u8>() as usize + v.len())
            .max()
            .unwrap_or(0x10000) as *const u8;
        let alloc = region::alloc_at(addr, size as usize, Self::conv_prot(protection))?;
        self.subregions.push(alloc);
        Ok((addr as usize - self.base_region.as_ptr::<u8>() as usize) as u32)
    }
}

fn main() {
    let mut mapper = MemoryMapper::new().expect("Mapping the base region");

    let memory_ctx = unsafe { mapper.flat_memory_ctx() };
    set_thread_ctx(memory_ctx);

    for item in PROGRAM_IMAGE.memory.iter() {
        mapper.map_item(item).expect("Mapping program memory")
    }

    let entry = PROGRAM_IMAGE.exe_entrypoint;

    let mut context = CpuContext::default();

    let stack_size = 0x10000;
    let stack_top = mapper
        .map(stack_size, Protection::READ_WRITE)
        .expect("Mapping stack");
    // set ESP to the bottom of the stack
    context.gp_regs[4] = stack_top + stack_size;

    rusty_x86_runtime::execute_recompiled_code(&mut context, memory_ctx, entry);
}
