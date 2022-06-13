use crate::ProcessContext;
use core_fs::Access;
use core_mem::ptr::{ConstPtr, MutPtr};
use core_str::PCSTR;
use encoding_rs::Encoding;
use std::ffi::c_void;
use std::sync::Arc;
use win32::Win32::Foundation::{BOOL, CHAR, HANDLE};
use win32::Win32::Security::SECURITY_ATTRIBUTES;
use win32::Win32::Storage::FileSystem::{
    FindFileHandle, CREATE_ALWAYS, CREATE_NEW, FILE_ACCESS_FLAGS, FILE_ATTRIBUTE_NORMAL,
    FILE_BEGIN, FILE_CREATION_DISPOSITION, FILE_CURRENT, FILE_END, FILE_FLAGS_AND_ATTRIBUTES,
    FILE_SHARE_MODE, OPEN_ALWAYS, OPEN_EXISTING, SET_FILE_POINTER_MOVE_METHOD, TRUNCATE_EXISTING,
    WIN32_FIND_DATAA,
};
use win32::Win32::System::SystemServices::{GENERIC_READ, GENERIC_WRITE};
use win32::Win32::System::IO::OVERLAPPED;
use win32_fs::{CreationDisposition, FindData, WindowsFsManager};
use win32_io::{IoDispatcher, SeekMethod};

pub struct FileSystem {
    pub process_ctx: ProcessContext,
    pub io_dispatcher: IoDispatcher,
    pub fs_manager: Arc<WindowsFsManager>,
}

fn convert_find_data_ansi(
    ansi_encoding: &'static Encoding,
    find_data: FindData,
) -> WIN32_FIND_DATAA {
    const BUFFER_SIZE: usize = 260;

    let (encoded_name, _, _) = ansi_encoding.encode(find_data.name.as_str());

    assert!(encoded_name.len() + 1 <= BUFFER_SIZE);

    let mut name_buffer: [CHAR; BUFFER_SIZE] = [CHAR(0); BUFFER_SIZE];

    // this loop is not rusty...
    for i in 0..encoded_name.len() {
        name_buffer[i] = CHAR(encoded_name[i]);
    }

    WIN32_FIND_DATAA {
        dwFileAttributes: 0,
        ftCreationTime: find_data.create_time,
        ftLastAccessTime: find_data.access_time,
        ftLastWriteTime: find_data.write_time,
        nFileSizeHigh: (find_data.size >> 32) as u32,
        nFileSizeLow: (find_data.size & 0xffff_ffff) as u32,
        dwReserved0: 0,
        dwReserved1: 0,
        cFileName: name_buffer,
        // this kind of violates the API, because alternate file name may be empty only if filename is in 8.3 format. But I hope nothing we emulate cares about 8.3
        cAlternateFileName: [CHAR(0); 14],
    }
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

        let (creation_disposition, truncate) = if dw_creation_disposition == CREATE_NEW {
            (CreationDisposition::CreateNew, false)
        } else if dw_creation_disposition == CREATE_ALWAYS {
            (CreationDisposition::OpenAlways, true)
        } else if dw_creation_disposition == OPEN_EXISTING {
            (CreationDisposition::OpenExisting, false)
        } else if dw_creation_disposition == OPEN_ALWAYS {
            (CreationDisposition::OpenAlways, false)
        } else if dw_creation_disposition == TRUNCATE_EXISTING {
            (CreationDisposition::OpenExisting, true)
        } else {
            panic!(
                "Invalid dw_creation_disposition supplied: {:?}",
                dw_creation_disposition
            )
        };

        // TODO: some creation dispositions supply info about whether file was created or not using error codes
        let (_was_file_created, handle) =
            self.fs_manager
                .create_file(path.as_ref(), access, creation_disposition, truncate);

        handle
    }

    fn FindClose(&self, h_find_file: FindFileHandle) -> BOOL {
        self.fs_manager.search_close(h_find_file);

        BOOL(1)
    }

    fn FindFirstFileA(
        &self,
        lp_file_name: PCSTR,
        lp_find_file_data: MutPtr<WIN32_FIND_DATAA>,
    ) -> FindFileHandle {
        let ctx = self.process_ctx.memory_ctx;

        let filename = lp_file_name
            .read_with(ctx)
            .as_utf8(self.process_ctx.ansi_encoding)
            .to_string();

        let search_handle = self.fs_manager.create_search(&filename);

        let found = self.fs_manager.search_next(search_handle);

        let found = found.expect("File specified in FindFirstFileA was not found");

        let found = convert_find_data_ansi(self.process_ctx.ansi_encoding, found);

        lp_find_file_data.write_with(ctx, found);

        search_handle
    }

    fn FindNextFileA(
        &self,
        h_find_file: FindFileHandle,
        lp_find_file_data: MutPtr<WIN32_FIND_DATAA>,
    ) -> BOOL {
        let ctx = self.process_ctx.memory_ctx;

        let next = self.fs_manager.search_next(h_find_file);

        if let Some(data) = next {
            lp_find_file_data.write_with(
                ctx,
                convert_find_data_ansi(self.process_ctx.ansi_encoding, data),
            );

            BOOL(1)
        } else {
            // TODO: set an error
            BOOL(0)
        }
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

    fn SetFilePointer(
        &self,
        h_file: HANDLE,
        l_distance_to_move: i32,
        lp_distance_to_move_high: MutPtr<i32>,
        dw_move_method: SET_FILE_POINTER_MOVE_METHOD,
    ) -> u32 {
        let ctx = self.process_ctx.memory_ctx;

        let distance: i64 = if lp_distance_to_move_high.is_null() {
            l_distance_to_move as i64
        } else {
            let high = lp_distance_to_move_high.read_with(ctx);
            (l_distance_to_move as u32 as i64) | ((high as i64) << 32)
        };

        let method = match dw_move_method {
            FILE_BEGIN => SeekMethod::FileBegin,
            FILE_CURRENT => SeekMethod::FileCurrent,
            FILE_END => SeekMethod::FileEnd,
            _ => panic!(
                "Unknown dw_move_method specified: {:#010x}",
                dw_move_method.0
            ),
        };

        let new_position = self.io_dispatcher.seek(h_file, distance, method);

        if !lp_distance_to_move_high.is_null() {
            lp_distance_to_move_high
                .write_with(ctx, ((new_position >> 32) & 0xffff_ffff) as u32 as i32)
        }

        let res = (new_position & 0xffff_ffff) as u32;

        res
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
