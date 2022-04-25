use crate::{AnsiString, PCSTR, PSTR};
use core_heap::RawHeapBox;
use core_heap::{Error as HeapError, Heap};
use core_mem::ctx::MemoryCtx;
use core_mem::ptr::PtrRepr;
use encoding_rs::Encoding;
use std::sync::{Arc, Mutex};

pub struct AnsiStringHeapBox {
    box_: RawHeapBox,
}

impl AnsiStringHeapBox {
    pub fn new(
        memory_ctx: impl MemoryCtx,
        heap: Arc<Mutex<Heap>>,
        ansi_str: &AnsiString,
    ) -> Result<Self, HeapError> {
        let buffer_size = (ansi_str.len() + 1) as PtrRepr;
        let box_ = RawHeapBox::new(heap, buffer_size)?;
        PSTR::new(box_.repr()).write_with(memory_ctx, buffer_size, &ansi_str);
        Ok(Self { box_ })
    }

    pub fn from_ascii(
        memory_ctx: impl MemoryCtx,
        heap: Arc<Mutex<Heap>>,
        ascii_str: &str,
    ) -> Result<Self, HeapError> {
        Self::new(memory_ctx, heap, &AnsiString::from_ascii(ascii_str))
    }

    pub fn from_utf8(
        memory_ctx: impl MemoryCtx,
        heap: Arc<Mutex<Heap>>,
        utf8_str: &str,
        ansi_encoding: &'static Encoding,
    ) -> Result<Self, HeapError> {
        Self::new(
            memory_ctx,
            heap,
            &AnsiString::from_utf8(utf8_str, ansi_encoding),
        )
    }

    pub fn ptr(&self) -> PCSTR {
        PCSTR::new(self.box_.repr())
    }

    pub fn ptr_mut(&self) -> PSTR {
        PSTR::new(self.box_.repr())
    }
}
