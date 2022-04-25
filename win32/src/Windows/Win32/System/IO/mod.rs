#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    unused_assignments,
    clippy::all
)]
#[allow(unused)]
use win32::core::prelude::*;
pub type LPOVERLAPPED_COMPLETION_ROUTINE = ::core::option::Option<()>;
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
        assert_eq!(from.len(), 24u32 as usize);
        let f_Internal = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_InternalHigh = <PtrRepr as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Anonymous = <OVERLAPPED_0 as FromIntoMemory>::from_bytes(&from[8..8 + 12]);
        let f_hEvent =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            Internal: f_Internal,
            InternalHigh: f_InternalHigh,
            Anonymous: f_Anonymous,
            hEvent: f_hEvent,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24u32 as usize);
        FromIntoMemory::into_bytes(self.Internal, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.InternalHigh, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[8..8 + 12]);
        FromIntoMemory::into_bytes(self.hEvent, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24u32 as usize
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
        assert_eq!(from.len(), 8u32 as usize);
        let f_Offset = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_OffsetHigh = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Offset: f_Offset,
            OffsetHigh: f_OffsetHigh,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8u32 as usize);
        FromIntoMemory::into_bytes(self.Offset, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.OffsetHigh, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8u32 as usize
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
        assert_eq!(from.len(), 16u32 as usize);
        let f_lpCompletionKey = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpOverlapped = <MutPtr<OVERLAPPED> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Internal = <PtrRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwNumberOfBytesTransferred = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            lpCompletionKey: f_lpCompletionKey,
            lpOverlapped: f_lpOverlapped,
            Internal: f_Internal,
            dwNumberOfBytesTransferred: f_dwNumberOfBytesTransferred,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16u32 as usize);
        FromIntoMemory::into_bytes(self.lpCompletionKey, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpOverlapped, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Internal, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfBytesTransferred, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16u32 as usize
    }
}
pub trait Api {
    fn BindIoCompletionCallback(
        &self,
        file_handle: super::super::Foundation::HANDLE,
        function: LPOVERLAPPED_COMPLETION_ROUTINE,
        flags: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("BindIoCompletionCallback")
    }
    fn CancelIo(&self, h_file: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
        todo!("CancelIo")
    }
    fn CancelIoEx(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_overlapped: ConstPtr<OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!("CancelIoEx")
    }
    fn CancelSynchronousIo(
        &self,
        h_thread: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("CancelSynchronousIo")
    }
    fn CreateIoCompletionPort(
        &self,
        file_handle: super::super::Foundation::HANDLE,
        existing_completion_port: super::super::Foundation::HANDLE,
        completion_key: PtrRepr,
        number_of_concurrent_threads: u32,
    ) -> super::super::Foundation::HANDLE {
        todo!("CreateIoCompletionPort")
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
        todo!("DeviceIoControl")
    }
    fn GetOverlappedResult(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_overlapped: ConstPtr<OVERLAPPED>,
        lp_number_of_bytes_transferred: MutPtr<u32>,
        b_wait: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("GetOverlappedResult")
    }
    fn GetOverlappedResultEx(
        &self,
        h_file: super::super::Foundation::HANDLE,
        lp_overlapped: ConstPtr<OVERLAPPED>,
        lp_number_of_bytes_transferred: MutPtr<u32>,
        dw_milliseconds: u32,
        b_alertable: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("GetOverlappedResultEx")
    }
    fn GetQueuedCompletionStatus(
        &self,
        completion_port: super::super::Foundation::HANDLE,
        lp_number_of_bytes_transferred: MutPtr<u32>,
        lp_completion_key: MutPtr<PtrRepr>,
        lp_overlapped: MutPtr<ConstPtr<OVERLAPPED>>,
        dw_milliseconds: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("GetQueuedCompletionStatus")
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
        todo!("GetQueuedCompletionStatusEx")
    }
    fn PostQueuedCompletionStatus(
        &self,
        completion_port: super::super::Foundation::HANDLE,
        dw_number_of_bytes_transferred: u32,
        dw_completion_key: PtrRepr,
        lp_overlapped: ConstPtr<OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!("PostQueuedCompletionStatus")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> &dyn Api {
    ctx.get::<dyn Api>()
}
