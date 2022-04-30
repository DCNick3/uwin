use std::sync::{Arc, Mutex};
use win32::Win32::Foundation::HANDLE;
use win32::Win32::System::WindowsProgramming::{FILE_TYPE_CHAR, FILE_TYPE_UNKNOWN};
use win32_kobj::{KernelHandleTable, KernelObject};

pub struct IoDispatcher {
    handle_table: Arc<Mutex<KernelHandleTable>>,
}

impl IoDispatcher {
    pub fn new(handle_table: Arc<Mutex<KernelHandleTable>>) -> Self {
        Self { handle_table }
    }

    pub fn get_file_type(&self, handle: HANDLE) -> u32 /* ehh, kinda want a stronger type on this... */
    {
        let handle_table = self.handle_table.lock().unwrap();
        handle_table
            .find(handle)
            .map(|obj| match *obj {
                KernelObject::Console(_) => FILE_TYPE_CHAR,
                _ => FILE_TYPE_UNKNOWN,
            })
            .unwrap_or(FILE_TYPE_UNKNOWN)
    }

    pub fn write_file(&self, handle: HANDLE, bytes: &[u8]) -> (bool, u32) {
        let handle_table = self.handle_table.lock().unwrap();
        let file = match handle_table.find(handle) {
            Some(f) => f,
            None => return (false, 0),
        };
        match &*file {
            KernelObject::Console(console) => {
                match console.write(bytes) {
                    Ok(written) => (true, written.try_into().unwrap()),
                    Err(_) => {
                        // TODO: map errors to win32 errors somehow
                        (false, 0)
                    }
                }
            }
            _ => unimplemented!("write_file to unknown object type"),
        }
    }
}
