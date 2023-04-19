#![doc = r" do not edit! File auto-generated with win32-bindgen"]
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
pub const ENCLAVE_FLAG_DYNAMIC_DEBUG_ACTIVE: u32 = 4u32;
pub const ENCLAVE_FLAG_DYNAMIC_DEBUG_ENABLED: u32 = 2u32;
pub const ENCLAVE_FLAG_FULL_DEBUG_ENABLED: u32 = 1u32;
pub const ENCLAVE_REPORT_DATA_LENGTH: u32 = 64u32;
pub const ENCLAVE_RUNTIME_POLICY_ALLOW_DYNAMIC_DEBUG: u32 = 2u32;
pub const ENCLAVE_RUNTIME_POLICY_ALLOW_FULL_DEBUG: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ENCLAVE_SEALING_IDENTITY_POLICY(pub i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_INVALID: ENCLAVE_SEALING_IDENTITY_POLICY =
    ENCLAVE_SEALING_IDENTITY_POLICY(0i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_EXACT_CODE: ENCLAVE_SEALING_IDENTITY_POLICY =
    ENCLAVE_SEALING_IDENTITY_POLICY(1i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_PRIMARY_CODE: ENCLAVE_SEALING_IDENTITY_POLICY =
    ENCLAVE_SEALING_IDENTITY_POLICY(2i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_IMAGE: ENCLAVE_SEALING_IDENTITY_POLICY =
    ENCLAVE_SEALING_IDENTITY_POLICY(3i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_FAMILY: ENCLAVE_SEALING_IDENTITY_POLICY =
    ENCLAVE_SEALING_IDENTITY_POLICY(4i32);
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_AUTHOR: ENCLAVE_SEALING_IDENTITY_POLICY =
    ENCLAVE_SEALING_IDENTITY_POLICY(5i32);
impl ::core::marker::Copy for ENCLAVE_SEALING_IDENTITY_POLICY {}
impl ::core::clone::Clone for ENCLAVE_SEALING_IDENTITY_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENCLAVE_SEALING_IDENTITY_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENCLAVE_SEALING_IDENTITY_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENCLAVE_SEALING_IDENTITY_POLICY")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ENCLAVE_SEALING_IDENTITY_POLICY {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const ENCLAVE_UNSEAL_FLAG_STALE_KEY: u32 = 1u32;
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_DEBUG_KEY: u32 = 8u32;
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_FAMILY_ID: u32 = 2u32;
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_IMAGE_ID: u32 = 4u32;
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_MEASUREMENT: u32 = 1u32;
pub struct ENCLAVE_VBS_BASIC_KEY_REQUEST {
    pub RequestSize: u32,
    pub Flags: u32,
    pub EnclaveSVN: u32,
    pub SystemKeyID: u32,
    pub CurrentSystemKeyID: u32,
}
impl ::core::marker::Copy for ENCLAVE_VBS_BASIC_KEY_REQUEST {}
impl ::core::clone::Clone for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCLAVE_VBS_BASIC_KEY_REQUEST")
            .field("RequestSize", &self.RequestSize)
            .field("Flags", &self.Flags)
            .field("EnclaveSVN", &self.EnclaveSVN)
            .field("SystemKeyID", &self.SystemKeyID)
            .field("CurrentSystemKeyID", &self.CurrentSystemKeyID)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.RequestSize == other.RequestSize
            && self.Flags == other.Flags
            && self.EnclaveSVN == other.EnclaveSVN
            && self.SystemKeyID == other.SystemKeyID
            && self.CurrentSystemKeyID == other.CurrentSystemKeyID
    }
}
impl ::core::cmp::Eq for ENCLAVE_VBS_BASIC_KEY_REQUEST {}
impl FromIntoMemory for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_RequestSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_EnclaveSVN = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_SystemKeyID = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_CurrentSystemKeyID = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            RequestSize: f_RequestSize,
            Flags: f_Flags,
            EnclaveSVN: f_EnclaveSVN,
            SystemKeyID: f_SystemKeyID,
            CurrentSystemKeyID: f_CurrentSystemKeyID,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.RequestSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.EnclaveSVN, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.SystemKeyID, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.CurrentSystemKeyID, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_COMMIT_PAGES = StdCallFnPtr<
    (
        ConstPtr<::core::ffi::c_void>,
        PtrRepr,
        ConstPtr<::core::ffi::c_void>,
        u32,
    ),
    i32,
>;
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD =
    StdCallFnPtr<(ConstPtr<VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64>,), i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD =
    StdCallFnPtr<(ConstPtr<VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32>,), i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_DECOMMIT_PAGES =
    StdCallFnPtr<(ConstPtr<::core::ffi::c_void>, PtrRepr), i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_KEY =
    StdCallFnPtr<(MutPtr<ENCLAVE_VBS_BASIC_KEY_REQUEST>, u32, MutPtr<u8>), i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_RANDOM_DATA =
    StdCallFnPtr<(MutPtr<u8>, u32, MutPtr<u64>), i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_REPORT =
    StdCallFnPtr<(ConstPtr<u8>, MutPtr<::core::ffi::c_void>, u32, MutPtr<u32>), i32>;
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD =
    StdCallFnPtr<(ConstPtr<VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64>,), i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD =
    StdCallFnPtr<(ConstPtr<VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32>,), i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_PROTECT_PAGES =
    StdCallFnPtr<(ConstPtr<::core::ffi::c_void>, PtrRepr, u32), i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_ENCLAVE = StdCallFnPtr<(PtrRepr,), ()>;
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION =
    StdCallFnPtr<(ConstPtr<VBS_BASIC_ENCLAVE_EXCEPTION_AMD64>,), i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION =
    StdCallFnPtr<(ConstPtr<::core::ffi::c_void>,), i32>;
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD =
    StdCallFnPtr<(ConstPtr<VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64>,), i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD =
    StdCallFnPtr<(ConstPtr<VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32>,), i32>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_VERIFY_REPORT =
    StdCallFnPtr<(ConstPtr<::core::ffi::c_void>, u32), i32>;
pub struct VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    pub ExceptionCode: u32,
    pub NumberParameters: u32,
    pub ExceptionInformation: [PtrRepr; 3],
    pub ExceptionRAX: PtrRepr,
    pub ExceptionRCX: PtrRepr,
    pub ExceptionRIP: PtrRepr,
    pub ExceptionRFLAGS: PtrRepr,
    pub ExceptionRSP: PtrRepr,
}
impl ::core::marker::Copy for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {}
impl ::core::clone::Clone for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBS_BASIC_ENCLAVE_EXCEPTION_AMD64")
            .field("ExceptionCode", &self.ExceptionCode)
            .field("NumberParameters", &self.NumberParameters)
            .field("ExceptionInformation", &self.ExceptionInformation)
            .field("ExceptionRAX", &self.ExceptionRAX)
            .field("ExceptionRCX", &self.ExceptionRCX)
            .field("ExceptionRIP", &self.ExceptionRIP)
            .field("ExceptionRFLAGS", &self.ExceptionRFLAGS)
            .field("ExceptionRSP", &self.ExceptionRSP)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionCode == other.ExceptionCode
            && self.NumberParameters == other.NumberParameters
            && self.ExceptionInformation == other.ExceptionInformation
            && self.ExceptionRAX == other.ExceptionRAX
            && self.ExceptionRCX == other.ExceptionRCX
            && self.ExceptionRIP == other.ExceptionRIP
            && self.ExceptionRFLAGS == other.ExceptionRFLAGS
            && self.ExceptionRSP == other.ExceptionRSP
    }
}
impl ::core::cmp::Eq for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {}
impl FromIntoMemory for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_ExceptionCode = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_NumberParameters = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ExceptionInformation = <[PtrRepr; 3] as FromIntoMemory>::from_bytes(&from[8..8 + 12]);
        let f_ExceptionRAX = <PtrRepr as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ExceptionRCX = <PtrRepr as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ExceptionRIP = <PtrRepr as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_ExceptionRFLAGS = <PtrRepr as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_ExceptionRSP = <PtrRepr as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            ExceptionCode: f_ExceptionCode,
            NumberParameters: f_NumberParameters,
            ExceptionInformation: f_ExceptionInformation,
            ExceptionRAX: f_ExceptionRAX,
            ExceptionRCX: f_ExceptionRCX,
            ExceptionRIP: f_ExceptionRIP,
            ExceptionRFLAGS: f_ExceptionRFLAGS,
            ExceptionRSP: f_ExceptionRSP,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.ExceptionCode, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.NumberParameters, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ExceptionInformation, &mut into[8..8 + 12]);
        FromIntoMemory::into_bytes(self.ExceptionRAX, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ExceptionRCX, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ExceptionRIP, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ExceptionRFLAGS, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.ExceptionRSP, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    pub ThreadContext: [u32; 4],
    pub EntryPoint: u32,
    pub StackPointer: u32,
    pub ExceptionEntryPoint: u32,
    pub ExceptionStack: u32,
    pub ExceptionActive: u32,
}
impl ::core::marker::Copy for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {}
impl ::core::clone::Clone for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32")
            .field("ThreadContext", &self.ThreadContext)
            .field("EntryPoint", &self.EntryPoint)
            .field("StackPointer", &self.StackPointer)
            .field("ExceptionEntryPoint", &self.ExceptionEntryPoint)
            .field("ExceptionStack", &self.ExceptionStack)
            .field("ExceptionActive", &self.ExceptionActive)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadContext == other.ThreadContext
            && self.EntryPoint == other.EntryPoint
            && self.StackPointer == other.StackPointer
            && self.ExceptionEntryPoint == other.ExceptionEntryPoint
            && self.ExceptionStack == other.ExceptionStack
            && self.ExceptionActive == other.ExceptionActive
    }
}
impl ::core::cmp::Eq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {}
impl FromIntoMemory for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_ThreadContext = <[u32; 4] as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_EntryPoint = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_StackPointer = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ExceptionEntryPoint = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ExceptionStack = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_ExceptionActive = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        Self {
            ThreadContext: f_ThreadContext,
            EntryPoint: f_EntryPoint,
            StackPointer: f_StackPointer,
            ExceptionEntryPoint: f_ExceptionEntryPoint,
            ExceptionStack: f_ExceptionStack,
            ExceptionActive: f_ExceptionActive,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.ThreadContext, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.EntryPoint, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.StackPointer, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ExceptionEntryPoint, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ExceptionStack, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ExceptionActive, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        36
    }
}
pub struct VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    pub ThreadContext: [u64; 4],
    pub EntryPoint: u64,
    pub StackPointer: u64,
    pub ExceptionEntryPoint: u64,
    pub ExceptionStack: u64,
    pub ExceptionActive: u32,
}
impl ::core::marker::Copy for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {}
impl ::core::clone::Clone for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64")
            .field("ThreadContext", &self.ThreadContext)
            .field("EntryPoint", &self.EntryPoint)
            .field("StackPointer", &self.StackPointer)
            .field("ExceptionEntryPoint", &self.ExceptionEntryPoint)
            .field("ExceptionStack", &self.ExceptionStack)
            .field("ExceptionActive", &self.ExceptionActive)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadContext == other.ThreadContext
            && self.EntryPoint == other.EntryPoint
            && self.StackPointer == other.StackPointer
            && self.ExceptionEntryPoint == other.ExceptionEntryPoint
            && self.ExceptionStack == other.ExceptionStack
            && self.ExceptionActive == other.ExceptionActive
    }
}
impl ::core::cmp::Eq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {}
impl FromIntoMemory for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_ThreadContext = <[u64; 4] as FromIntoMemory>::from_bytes(&from[0..0 + 32]);
        let f_EntryPoint = <u64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_StackPointer = <u64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        let f_ExceptionEntryPoint = <u64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_ExceptionStack = <u64 as FromIntoMemory>::from_bytes(&from[56..56 + 8]);
        let f_ExceptionActive = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        Self {
            ThreadContext: f_ThreadContext,
            EntryPoint: f_EntryPoint,
            StackPointer: f_StackPointer,
            ExceptionEntryPoint: f_ExceptionEntryPoint,
            ExceptionStack: f_ExceptionStack,
            ExceptionActive: f_ExceptionActive,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.ThreadContext, &mut into[0..0 + 32]);
        FromIntoMemory::into_bytes(self.EntryPoint, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.StackPointer, &mut into[40..40 + 8]);
        FromIntoMemory::into_bytes(self.ExceptionEntryPoint, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.ExceptionStack, &mut into[56..56 + 8]);
        FromIntoMemory::into_bytes(self.ExceptionActive, &mut into[64..64 + 4]);
    }
    fn size() -> usize {
        72
    }
}
pub const VBS_ENCLAVE_REPORT_PKG_HEADER_VERSION_CURRENT: u32 = 1u32;
pub const VBS_ENCLAVE_REPORT_SIGNATURE_SCHEME_SHA256_RSA_PSS_SHA256: u32 = 1u32;
pub const VBS_ENCLAVE_REPORT_VERSION_CURRENT: u32 = 1u32;
pub const VBS_ENCLAVE_VARDATA_INVALID: u32 = 0u32;
pub const VBS_ENCLAVE_VARDATA_MODULE: u32 = 1u32;
pub trait Api {
    #[doc = "ExpandEnvironmentStringsA from KERNEL32"]
    fn ExpandEnvironmentStringsA(&self, lp_src: PCSTR, lp_dst: PSTR, n_size: u32) -> u32 {
        todo!("ExpandEnvironmentStringsA")
    }
    #[doc = "ExpandEnvironmentStringsW from KERNEL32"]
    fn ExpandEnvironmentStringsW(&self, lp_src: PCWSTR, lp_dst: PWSTR, n_size: u32) -> u32 {
        todo!("ExpandEnvironmentStringsW")
    }
    #[doc = "FreeEnvironmentStringsA from KERNEL32"]
    fn FreeEnvironmentStringsA(&self, penv: PCSTR) -> super::super::Foundation::BOOL {
        todo!("FreeEnvironmentStringsA")
    }
    #[doc = "FreeEnvironmentStringsW from KERNEL32"]
    fn FreeEnvironmentStringsW(&self, penv: PCWSTR) -> super::super::Foundation::BOOL {
        todo!("FreeEnvironmentStringsW")
    }
    #[doc = "GetCommandLineA from KERNEL32"]
    fn GetCommandLineA(&self) -> PSTR {
        todo!("GetCommandLineA")
    }
    #[doc = "GetCommandLineW from KERNEL32"]
    fn GetCommandLineW(&self) -> PWSTR {
        todo!("GetCommandLineW")
    }
    #[doc = "GetCurrentDirectoryA from KERNEL32"]
    fn GetCurrentDirectoryA(&self, n_buffer_length: u32, lp_buffer: PSTR) -> u32 {
        todo!("GetCurrentDirectoryA")
    }
    #[doc = "GetCurrentDirectoryW from KERNEL32"]
    fn GetCurrentDirectoryW(&self, n_buffer_length: u32, lp_buffer: PWSTR) -> u32 {
        todo!("GetCurrentDirectoryW")
    }
    #[doc = "GetEnvironmentStrings from KERNEL32"]
    fn GetEnvironmentStrings(&self) -> PSTR {
        todo!("GetEnvironmentStrings")
    }
    #[doc = "GetEnvironmentStringsW from KERNEL32"]
    fn GetEnvironmentStringsW(&self) -> PWSTR {
        todo!("GetEnvironmentStringsW")
    }
    #[doc = "GetEnvironmentVariableA from KERNEL32"]
    fn GetEnvironmentVariableA(&self, lp_name: PCSTR, lp_buffer: PSTR, n_size: u32) -> u32 {
        todo!("GetEnvironmentVariableA")
    }
    #[doc = "GetEnvironmentVariableW from KERNEL32"]
    fn GetEnvironmentVariableW(&self, lp_name: PCWSTR, lp_buffer: PWSTR, n_size: u32) -> u32 {
        todo!("GetEnvironmentVariableW")
    }
    #[doc = "SetCurrentDirectoryA from KERNEL32"]
    fn SetCurrentDirectoryA(&self, lp_path_name: PCSTR) -> super::super::Foundation::BOOL {
        todo!("SetCurrentDirectoryA")
    }
    #[doc = "SetCurrentDirectoryW from KERNEL32"]
    fn SetCurrentDirectoryW(&self, lp_path_name: PCWSTR) -> super::super::Foundation::BOOL {
        todo!("SetCurrentDirectoryW")
    }
    #[doc = "SetEnvironmentStringsW from KERNEL32"]
    fn SetEnvironmentStringsW(&self, new_environment: PCWSTR) -> super::super::Foundation::BOOL {
        todo!("SetEnvironmentStringsW")
    }
    #[doc = "SetEnvironmentVariableA from KERNEL32"]
    fn SetEnvironmentVariableA(
        &self,
        lp_name: PCSTR,
        lp_value: PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetEnvironmentVariableA")
    }
    #[doc = "SetEnvironmentVariableW from KERNEL32"]
    fn SetEnvironmentVariableW(
        &self,
        lp_name: PCWSTR,
        lp_value: PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetEnvironmentVariableW")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
