extern crate core;

mod r#impl;

use crate::r#impl::*;
use core_mem::thread_ctx::set_thread_ctx;
use recompiler::memory_image::{MemoryImageItem, Protection};
use region::Allocation;
use rusty_x86_runtime::{CpuContext, ExtendedContext, FlatMemoryCtx, PROGRAM_IMAGE};
use std::io::Write;
use std::sync::Arc;
use tracing::trace;
use tracing_subscriber::EnvFilter;
use win32::core::Win32Context;

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
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mut mapper = MemoryMapper::new().expect("Mapping the base region");

    let memory_ctx = unsafe { mapper.flat_memory_ctx() };
    set_thread_ctx(memory_ctx);

    for item in PROGRAM_IMAGE.memory.iter() {
        mapper.map_item(item).expect("Mapping program memory")
    }

    let entry = PROGRAM_IMAGE.exe_entrypoint;

    let mut context = ExtendedContext {
        cpu: CpuContext::default(),
        win32: Win32Context::new(),
        unwind_reason: None,
    };

    context.win32.insert(
        Arc::new(WindowsAndMessaging {}) as Arc<dyn win32::Win32::UI::WindowsAndMessaging::Api>
    );
    context.win32.insert(
        Arc::new(SystemInformation {}) as Arc<dyn win32::Win32::System::SystemInformation::Api>
    );
    context
        .win32
        .insert(Arc::new(Memory {}) as Arc<dyn win32::Win32::System::Memory::Api>);
    context
        .win32
        .insert(Arc::new(LibraryLoader {}) as Arc<dyn win32::Win32::System::LibraryLoader::Api>);
    context
        .win32
        .insert(Arc::new(Threading {}) as Arc<dyn win32::Win32::System::Threading::Api>);

    let tlb = mapper
        .map(0x1000, Protection::READ_WRITE)
        .expect("Mapping TLB");

    let stack_size = 0x10000;
    let stack_top = mapper
        .map(stack_size, Protection::READ_WRITE)
        .expect("Mapping stack");
    // set ESP to the bottom of the stack
    // TODO: put some return value to the stack when calling
    // probably could be done through a generalized way of callbacks implementation
    // for now the zero is left there (hopefully it will be zero...)
    context.cpu.gp_regs[4] = stack_top + stack_size - 4;
    context.cpu.fs_base = tlb;

    let res = rusty_x86_runtime::execute_recompiled_code(&mut context, memory_ctx, entry);
    trace!("execute_recompiled_code returned 0x{:08x}", res);

    if res == 0 {
        if let Some(reason) = context.unwind_reason {
            trace!("The recompiled stack was unwound, reason = {reason:?}")
        } else {
            panic!(
                "Zero continuation address without an unwind reason set. Sounds like an ABI crime"
            )
        }
    } else {
        assert!(
            context.unwind_reason.is_none(),
            "Non-zero address return with some unwind reason... Sounds like an ABI crime"
        );
        todo!("Re-entering the code execution after yielding") // Not sure of all the consequences of just doing it
    }
}
