use crate::ProcessContext;
use core_mem::ptr::{ConstPtr, MutPtr};
use core_str::PCSTR;
use std::ffi::c_void;
use win32::Win32::Foundation::{BOOL, HANDLE};
use win32::Win32::Security::SECURITY_ATTRIBUTES;
use win32::Win32::Storage::FileSystem::{
    CREATE_ALWAYS, CREATE_NEW, FILE_ACCESS_FLAGS, FILE_ATTRIBUTE_NORMAL, FILE_CREATION_DISPOSITION,
    FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE, OPEN_ALWAYS, OPEN_EXISTING, TRUNCATE_EXISTING,
};
use win32::Win32::System::SystemServices::{GENERIC_READ, GENERIC_WRITE};
use win32::Win32::System::IO::OVERLAPPED;
use win32_fs::{Access, CreationDisposition, WindowsFsManager};
use win32_io::IoDispatcher;

pub struct FileSystem {
    pub process_ctx: ProcessContext,
    pub io_dispatcher: IoDispatcher,
    pub fs_manager: WindowsFsManager,
}

#[allow(non_snake_case)]
impl win32::Win32::Storage::FileSystem::Api for FileSystem {
    fn CreateFileA(
        &self,
        lp_file_name: PCSTR,
        dw_desired_access: FILE_ACCESS_FLAGS,
        _dw_share_mode: FILE_SHARE_MODE,
        _lp_security_attributes: ConstPtr<SECURITY_ATTRIBUTES>,
        dw_creation_disposition: FILE_CREATION_DISPOSITION,
        dw_flags_and_attributes: FILE_FLAGS_AND_ATTRIBUTES,
        h_template_file: HANDLE,
    ) -> HANDLE {
        assert_eq!(
            h_template_file,
            HANDLE(0),
            "using a template for file creation is not supported"
        );

        let ctx = self.process_ctx.memory_ctx;
        let path = lp_file_name.read_with(ctx);
        let path = path.as_utf8(self.process_ctx.ansi_encoding);

        let filtered_access = dw_desired_access.0 & (GENERIC_READ | GENERIC_WRITE);

        assert_eq!(
            filtered_access, dw_desired_access.0,
            "Some specified access bits are unsupported"
        );

        assert_eq!(
            dw_flags_and_attributes, FILE_ATTRIBUTE_NORMAL,
            "flags and attributes besides FILE_ATTRIBUTE_NORMAL not supported yet"
        );

        let access = if filtered_access == GENERIC_READ {
            Access::Read
        } else if filtered_access == GENERIC_WRITE {
            Access::Write
        } else if filtered_access == GENERIC_READ | GENERIC_WRITE {
            Access::ReadWrite
        } else {
            panic!("Attempt to open a file with no access");
        };

        let creation_disposition = if dw_creation_disposition == CREATE_NEW {
            CreationDisposition::CreateNew
        } else if dw_creation_disposition == CREATE_ALWAYS {
            CreationDisposition::CreateAlways
        } else if dw_creation_disposition == OPEN_EXISTING {
            CreationDisposition::OpenExisting
        } else if dw_creation_disposition == OPEN_ALWAYS {
            CreationDisposition::OpenAlways
        } else if dw_creation_disposition == TRUNCATE_EXISTING {
            CreationDisposition::TruncateExisting
        } else {
            panic!(
                "Invalid dw_creation_disposition supplied: {:?}",
                dw_creation_disposition
            )
        };

        self.fs_manager
            .create_file(path.as_ref(), access, creation_disposition)
    }

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
