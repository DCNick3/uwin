#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[allow(unused)]
use win32::core::prelude::*;
pub type LPOVERLAPPED_COMPLETION_ROUTINE = ::core::option::Option<
    unsafe extern "system" fn(
        dw_error_code: u32,
        dw_number_of_bytes_transfered: u32,
        lp_overlapped: MutPtr<OVERLAPPED>,
    ),
>;
pub struct OVERLAPPED {
    pub Internal: PtrRepr,
    pub InternalHigh: PtrRepr,
    pub Anonymous: OVERLAPPED_0,
    pub hEvent: super::super::Foundation::HANDLE,
}
impl ::core::marker::Copy for OVERLAPPED {}
impl ::core::clone::Clone for OVERLAPPED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for OVERLAPPED {
    fn eq(&self, other: &Self) -> bool {
        self.Internal == other.Internal
            && self.InternalHigh == other.InternalHigh
            && self.Anonymous == other.Anonymous
            && self.hEvent == other.hEvent
    }
}
impl ::core::cmp::Eq for OVERLAPPED {}
impl FromIntoMemory for OVERLAPPED {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct OVERLAPPED_0 {
    pub Anonymous: OVERLAPPED_0_0,
    pub Pointer: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for OVERLAPPED_0 {}
impl ::core::clone::Clone for OVERLAPPED_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for OVERLAPPED_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous && self.Pointer == other.Pointer
    }
}
impl ::core::cmp::Eq for OVERLAPPED_0 {}
impl FromIntoMemory for OVERLAPPED_0 {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct OVERLAPPED_0_0 {
    pub Offset: u32,
    pub OffsetHigh: u32,
}
impl ::core::marker::Copy for OVERLAPPED_0_0 {}
impl ::core::clone::Clone for OVERLAPPED_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OVERLAPPED_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OVERLAPPED_0_0")
            .field("Offset", &self.Offset)
            .field("OffsetHigh", &self.OffsetHigh)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OVERLAPPED_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.OffsetHigh == other.OffsetHigh
    }
}
impl ::core::cmp::Eq for OVERLAPPED_0_0 {}
impl FromIntoMemory for OVERLAPPED_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub struct OVERLAPPED_ENTRY {
    pub lpCompletionKey: PtrRepr,
    pub lpOverlapped: MutPtr<OVERLAPPED>,
    pub Internal: PtrRepr,
    pub dwNumberOfBytesTransferred: u32,
}
impl ::core::marker::Copy for OVERLAPPED_ENTRY {}
impl ::core::clone::Clone for OVERLAPPED_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OVERLAPPED_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OVERLAPPED_ENTRY")
            .field("lpCompletionKey", &self.lpCompletionKey)
            .field("lpOverlapped", &self.lpOverlapped)
            .field("Internal", &self.Internal)
            .field(
                "dwNumberOfBytesTransferred",
                &self.dwNumberOfBytesTransferred,
            )
            .finish()
    }
}
impl ::core::cmp::PartialEq for OVERLAPPED_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.lpCompletionKey == other.lpCompletionKey
            && self.lpOverlapped == other.lpOverlapped
            && self.Internal == other.Internal
            && self.dwNumberOfBytesTransferred == other.dwNumberOfBytesTransferred
    }
}
impl ::core::cmp::Eq for OVERLAPPED_ENTRY {}
impl FromIntoMemory for OVERLAPPED_ENTRY {
    fn from_bytes(from: &[u8]) -> Self {
        todo!()
    }
    fn into_bytes(self, into: &mut [u8]) {
        todo!()
    }
    fn size() -> usize {
        todo!()
    }
}
pub trait Api {
    fn BindIoCompletionCallback(
        &self,
        file_handle: super::super::Foundation::HANDLE,
        function: LPOVERLAPPED_COMPLETION_ROUTINE,
        flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CancelIo(&self, h_file: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CancelIoEx(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_overlapped: ConstPtr<OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CancelSynchronousIo(
        &self,
        h_thread: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn CreateIoCompletionPort(
        &self,
        file_handle: super::super::Foundation::HANDLE,
        existing_completion_port: super::super::Foundation::HANDLE,
        completion_key: PtrRepr,
        number_of_concurrent_threads: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!()
    }
    fn DeviceIoControl(
        &self,
        h_device: super::super::Foundation::HANDLE,
        dw_io_control_code: u32,
        lp_in_buffer: ConstPtr<::core::ffi::c_void>,
        n_in_buffer_size: u32,
        lp_out_buffer: MutPtr<::core::ffi::c_void>,
        n_out_buffer_size: u32,
        lp_bytes_returned: MutPtr<u32>,
        lp_overlapped: MutPtr<OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetOverlappedResult(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_overlapped: ConstPtr<OVERLAPPED>,
        lp_number_of_bytes_transferred: MutPtr<u32>,
        b_wait: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetOverlappedResultEx(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_overlapped: ConstPtr<OVERLAPPED>,
        lp_number_of_bytes_transferred: MutPtr<u32>,
        dw_milliseconds: u32,
        b_alertable: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetQueuedCompletionStatus(
        &self,
        completion_port: super::super::Foundation::HANDLE,
        lp_number_of_bytes_transferred: MutPtr<u32>,
        lp_completion_key: MutPtr<PtrRepr>,
        lp_overlapped: MutPtr<ConstPtr<OVERLAPPED>>,
        dw_milliseconds: u32,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn GetQueuedCompletionStatusEx(
        &self,
        completion_port: super::super::Foundation::HANDLE,
        lp_completion_port_entries: MutPtr<OVERLAPPED_ENTRY>,
        ul_count: u32,
        ul_num_entries_removed: MutPtr<u32>,
        dw_milliseconds: u32,
        f_alertable: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
    fn PostQueuedCompletionStatus(
        &self,
        completion_port: super::super::Foundation::HANDLE,
        dw_number_of_bytes_transferred: u32,
        dw_completion_key: PtrRepr,
        lp_overlapped: ConstPtr<OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!()
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> &dyn Api {
    ctx.get::<dyn Api>()
}
