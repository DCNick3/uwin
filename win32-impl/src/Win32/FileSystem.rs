use crate::ProcessContext;
use core_mem::ptr::{ConstPtr, MutPtr};
use std::ffi::c_void;
use win32::Win32::Foundation::{BOOL, HANDLE};
use win32::Win32::System::IO::OVERLAPPED;
use win32_io::IoDispatcher;

pub struct FileSystem {
    pub process_ctx: ProcessContext,
    pub io_dispatcher: IoDispatcher,
}

#[allow(non_snake_case)]
impl win32::Win32::Storage::FileSystem::Api for FileSystem {
    fn GetFileType(&self, h_file: HANDLE) -> u32 {
        self.io_dispatcher.get_file_type(h_file)
    }

    fn ReadFile(
        &self,
        h_file: HANDLE,
        lp_buffer: MutPtr<c_void>,
        n_number_of_bytes_to_read: u32,
        lp_number_of_bytes_read: MutPtr<u32>,
        lp_overlapped: MutPtr<OVERLAPPED>,
    ) -> BOOL {
        assert_eq!(lp_overlapped, MutPtr::null());
        let ctx = self.process_ctx.memory_ctx;

        let mut buffer = vec![0u8; n_number_of_bytes_to_read as usize];

        let (ok, read) = self.io_dispatcher.read_file(h_file, &mut buffer);

        lp_buffer
            .pun::<u8>()
            .write_bytes(ctx, &buffer[..read as usize]);

        assert_ne!(lp_number_of_bytes_read, MutPtr::null());
        lp_number_of_bytes_read.write_with(ctx, read);

        ok.into()
    }

    fn WriteFile(
        &self,
        h_file: HANDLE,
        lp_buffer: ConstPtr<c_void>,
        n_number_of_bytes_to_write: u32,
        lp_number_of_bytes_written: MutPtr<u32>,
        lp_overlapped: MutPtr<OVERLAPPED>,
    ) -> BOOL {
        assert_eq!(lp_overlapped, MutPtr::null());
        let ctx = self.process_ctx.memory_ctx;

        let bytes = lp_buffer
            .pun::<u8>()
            .read_bytes(ctx, n_number_of_bytes_to_write);

        let (ok, written) = self.io_dispatcher.write_file(h_file, &bytes);

        assert_ne!(lp_number_of_bytes_written, MutPtr::null());
        lp_number_of_bytes_written.write_with(ctx, written);

        ok.into()
    }
}
