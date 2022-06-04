use core_fs::path::WindowsPath;
use std::sync::{Arc, Mutex};
use win32::Win32::Foundation::HANDLE;
use win32_kobj::KernelHandleTable;

pub struct WindowsFsManager {
    handle_table: Arc<Mutex<KernelHandleTable>>,
}

// windows can actually have more fine-grained access rights, but this is good enough
pub enum Access {
    Read,
    Write,
    ReadWrite,
}

// TODO: ignored for now, probably can implement later (is it needed in a single process context though?)
// pub enum ShareMode {}

pub enum CreationDisposition {
    CreateNew,
    CreateAlways,
    OpenExisting,
    OpenAlways,
    TruncateExisting,
}

impl WindowsFsManager {
    pub fn new(handle_table: Arc<Mutex<KernelHandleTable>>) -> Self {
        Self { handle_table }
    }

    pub fn create_file(
        &self,
        path: &str,
        access: Access,
        creation_disposition: CreationDisposition,
    ) -> HANDLE {
        let path = WindowsPath::parse(path).expect("Invalid path passed to create_file");

        let path_str = format!("{}", path);

        // TODO: handle errors
        todo!()
    }
}
