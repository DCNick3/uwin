use crate::ProcessContext;
use std::sync::{Arc, Mutex};
use win32::Win32::Foundation::{BOOL, HANDLE};
use win32_kobj::KernelHandleTable;

pub struct Foundation {
    pub process_ctx: ProcessContext,
    pub handle_table: Arc<Mutex<KernelHandleTable>>,
}

impl win32::Win32::Foundation::Api for Foundation {
    fn CloseHandle(&self, h_object: HANDLE) -> BOOL {
        let mut handle_table = self.handle_table.lock().unwrap();

        handle_table.remove(h_object);

        BOOL(1)
    }
}
