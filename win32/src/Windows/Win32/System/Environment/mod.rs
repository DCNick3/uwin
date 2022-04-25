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
pub struct ENCLAVE_IDENTITY {
    pub OwnerId: [u8; 32],
    pub UniqueId: [u8; 32],
    pub AuthorId: [u8; 32],
    pub FamilyId: [u8; 16],
    pub ImageId: [u8; 16],
    pub EnclaveSvn: u32,
    pub SecureKernelSvn: u32,
    pub PlatformSvn: u32,
    pub Flags: u32,
    pub SigningLevel: u32,
    pub EnclaveType: u32,
}
impl ::core::marker::Copy for ENCLAVE_IDENTITY {}
impl ::core::clone::Clone for ENCLAVE_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for ENCLAVE_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.OwnerId == other.OwnerId
            && self.UniqueId == other.UniqueId
            && self.AuthorId == other.AuthorId
            && self.FamilyId == other.FamilyId
            && self.ImageId == other.ImageId
            && self.EnclaveSvn == other.EnclaveSvn
            && self.SecureKernelSvn == other.SecureKernelSvn
            && self.PlatformSvn == other.PlatformSvn
            && self.Flags == other.Flags
            && self.SigningLevel == other.SigningLevel
            && self.EnclaveType == other.EnclaveType
    }
}
impl ::core::cmp::Eq for ENCLAVE_IDENTITY {}
impl FromIntoMemory for ENCLAVE_IDENTITY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 152u32 as usize);
        let f_OwnerId = <[u8; 32] as FromIntoMemory>::from_bytes(&from[0..0 + 32]);
        let f_UniqueId = <[u8; 32] as FromIntoMemory>::from_bytes(&from[32..32 + 32]);
        let f_AuthorId = <[u8; 32] as FromIntoMemory>::from_bytes(&from[64..64 + 32]);
        let f_FamilyId = <[u8; 16] as FromIntoMemory>::from_bytes(&from[96..96 + 16]);
        let f_ImageId = <[u8; 16] as FromIntoMemory>::from_bytes(&from[112..112 + 16]);
        let f_EnclaveSvn = <u32 as FromIntoMemory>::from_bytes(&from[128..128 + 4]);
        let f_SecureKernelSvn = <u32 as FromIntoMemory>::from_bytes(&from[132..132 + 4]);
        let f_PlatformSvn = <u32 as FromIntoMemory>::from_bytes(&from[136..136 + 4]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[140..140 + 4]);
        let f_SigningLevel = <u32 as FromIntoMemory>::from_bytes(&from[144..144 + 4]);
        let f_EnclaveType = <u32 as FromIntoMemory>::from_bytes(&from[148..148 + 4]);
        Self {
            OwnerId: f_OwnerId,
            UniqueId: f_UniqueId,
            AuthorId: f_AuthorId,
            FamilyId: f_FamilyId,
            ImageId: f_ImageId,
            EnclaveSvn: f_EnclaveSvn,
            SecureKernelSvn: f_SecureKernelSvn,
            PlatformSvn: f_PlatformSvn,
            Flags: f_Flags,
            SigningLevel: f_SigningLevel,
            EnclaveType: f_EnclaveType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 152u32 as usize);
        FromIntoMemory::into_bytes(self.OwnerId, &mut into[0..0 + 32]);
        FromIntoMemory::into_bytes(self.UniqueId, &mut into[32..32 + 32]);
        FromIntoMemory::into_bytes(self.AuthorId, &mut into[64..64 + 32]);
        FromIntoMemory::into_bytes(self.FamilyId, &mut into[96..96 + 16]);
        FromIntoMemory::into_bytes(self.ImageId, &mut into[112..112 + 16]);
        FromIntoMemory::into_bytes(self.EnclaveSvn, &mut into[128..128 + 4]);
        FromIntoMemory::into_bytes(self.SecureKernelSvn, &mut into[132..132 + 4]);
        FromIntoMemory::into_bytes(self.PlatformSvn, &mut into[136..136 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[140..140 + 4]);
        FromIntoMemory::into_bytes(self.SigningLevel, &mut into[144..144 + 4]);
        FromIntoMemory::into_bytes(self.EnclaveType, &mut into[148..148 + 4]);
    }
    fn size() -> usize {
        32 + 32 + 32 + 16 + 16 + 4 + 4 + 4 + 4 + 4 + 4
    }
}
pub struct ENCLAVE_INFORMATION {
    pub EnclaveType: u32,
    pub Reserved: u32,
    pub BaseAddress: MutPtr<::core::ffi::c_void>,
    pub Size: PtrRepr,
    pub Identity: ENCLAVE_IDENTITY,
}
impl ::core::marker::Copy for ENCLAVE_INFORMATION {}
impl ::core::clone::Clone for ENCLAVE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for ENCLAVE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.EnclaveType == other.EnclaveType
            && self.Reserved == other.Reserved
            && self.BaseAddress == other.BaseAddress
            && self.Size == other.Size
            && self.Identity == other.Identity
    }
}
impl ::core::cmp::Eq for ENCLAVE_INFORMATION {}
impl FromIntoMemory for ENCLAVE_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 168u32 as usize);
        let f_EnclaveType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Reserved = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_BaseAddress =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_Size = <PtrRepr as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Identity = <ENCLAVE_IDENTITY as FromIntoMemory>::from_bytes(&from[16..16 + 152]);
        Self {
            EnclaveType: f_EnclaveType,
            Reserved: f_Reserved,
            BaseAddress: f_BaseAddress,
            Size: f_Size,
            Identity: f_Identity,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 168u32 as usize);
        FromIntoMemory::into_bytes(self.EnclaveType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.BaseAddress, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.Size, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Identity, &mut into[16..16 + 152]);
    }
    fn size() -> usize {
        4 + 4 + 4 + 4 + 152
    }
}
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
        assert_eq!(from.len(), 20u32 as usize);
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
        assert_eq!(into.len(), 20u32 as usize);
        FromIntoMemory::into_bytes(self.RequestSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.EnclaveSVN, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.SystemKeyID, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.CurrentSystemKeyID, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        4 + 4 + 4 + 4 + 4
    }
}
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_COMMIT_PAGES = ::core::option::Option<()>;
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = ::core::option::Option<()>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = ::core::option::Option<()>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_DECOMMIT_PAGES = ::core::option::Option<()>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_KEY = ::core::option::Option<()>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_RANDOM_DATA = ::core::option::Option<()>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_REPORT = ::core::option::Option<()>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GET_ENCLAVE_INFORMATION = ::core::option::Option<()>;
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD = ::core::option::Option<()>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD = ::core::option::Option<()>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_PROTECT_PAGES = ::core::option::Option<()>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_ENCLAVE = ::core::option::Option<()>;
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION = ::core::option::Option<()>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION = ::core::option::Option<()>;
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD = ::core::option::Option<()>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD = ::core::option::Option<()>;
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_VERIFY_REPORT = ::core::option::Option<()>;
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
        assert_eq!(from.len(), 40u32 as usize);
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
        assert_eq!(into.len(), 40u32 as usize);
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
        4 + 4 + 12 + 4 + 4 + 4 + 4 + 4
    }
}
pub struct VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    pub ReturnFromEnclave: VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_ENCLAVE,
    pub ReturnFromException: VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION,
    pub TerminateThread: VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD,
    pub InterruptThread: VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD,
    pub CommitPages: VBS_BASIC_ENCLAVE_BASIC_CALL_COMMIT_PAGES,
    pub DecommitPages: VBS_BASIC_ENCLAVE_BASIC_CALL_DECOMMIT_PAGES,
    pub ProtectPages: VBS_BASIC_ENCLAVE_BASIC_CALL_PROTECT_PAGES,
    pub CreateThread: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD,
    pub GetEnclaveInformation: VBS_BASIC_ENCLAVE_BASIC_CALL_GET_ENCLAVE_INFORMATION,
    pub GenerateKey: VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_KEY,
    pub GenerateReport: VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_REPORT,
    pub VerifyReport: VBS_BASIC_ENCLAVE_BASIC_CALL_VERIFY_REPORT,
    pub GenerateRandomData: VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_RANDOM_DATA,
}
impl ::core::marker::Copy for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {}
impl ::core::clone::Clone for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBS_BASIC_ENCLAVE_SYSCALL_PAGE")
            .field("ReturnFromEnclave", &self.ReturnFromEnclave)
            .field("ReturnFromException", &self.ReturnFromException)
            .field("TerminateThread", &self.TerminateThread)
            .field("InterruptThread", &self.InterruptThread)
            .field("CommitPages", &self.CommitPages)
            .field("DecommitPages", &self.DecommitPages)
            .field("ProtectPages", &self.ProtectPages)
            .field("CreateThread", &self.CreateThread)
            .field("GetEnclaveInformation", &self.GetEnclaveInformation)
            .field("GenerateKey", &self.GenerateKey)
            .field("GenerateReport", &self.GenerateReport)
            .field("VerifyReport", &self.VerifyReport)
            .field("GenerateRandomData", &self.GenerateRandomData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    fn eq(&self, other: &Self) -> bool {
        self.ReturnFromEnclave == other.ReturnFromEnclave
            && self.ReturnFromException == other.ReturnFromException
            && self.TerminateThread == other.TerminateThread
            && self.InterruptThread == other.InterruptThread
            && self.CommitPages == other.CommitPages
            && self.DecommitPages == other.DecommitPages
            && self.ProtectPages == other.ProtectPages
            && self.CreateThread == other.CreateThread
            && self.GetEnclaveInformation == other.GetEnclaveInformation
            && self.GenerateKey == other.GenerateKey
            && self.GenerateReport == other.GenerateReport
            && self.VerifyReport == other.VerifyReport
            && self.GenerateRandomData == other.GenerateRandomData
    }
}
impl ::core::cmp::Eq for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {}
impl FromIntoMemory for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52u32 as usize);
        let f_ReturnFromEnclave =
            <VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_ENCLAVE as FromIntoMemory>::from_bytes(
                &from[0..0 + 4],
            );
        let f_ReturnFromException =
            <VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION as FromIntoMemory>::from_bytes(
                &from[4..4 + 4],
            );
        let f_TerminateThread =
            <VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD as FromIntoMemory>::from_bytes(
                &from[8..8 + 4],
            );
        let f_InterruptThread =
            <VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD as FromIntoMemory>::from_bytes(
                &from[12..12 + 4],
            );
        let f_CommitPages =
            <VBS_BASIC_ENCLAVE_BASIC_CALL_COMMIT_PAGES as FromIntoMemory>::from_bytes(
                &from[16..16 + 4],
            );
        let f_DecommitPages =
            <VBS_BASIC_ENCLAVE_BASIC_CALL_DECOMMIT_PAGES as FromIntoMemory>::from_bytes(
                &from[20..20 + 4],
            );
        let f_ProtectPages =
            <VBS_BASIC_ENCLAVE_BASIC_CALL_PROTECT_PAGES as FromIntoMemory>::from_bytes(
                &from[24..24 + 4],
            );
        let f_CreateThread =
            <VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD as FromIntoMemory>::from_bytes(
                &from[28..28 + 4],
            );
        let f_GetEnclaveInformation =
            <VBS_BASIC_ENCLAVE_BASIC_CALL_GET_ENCLAVE_INFORMATION as FromIntoMemory>::from_bytes(
                &from[32..32 + 4],
            );
        let f_GenerateKey =
            <VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_KEY as FromIntoMemory>::from_bytes(
                &from[36..36 + 4],
            );
        let f_GenerateReport =
            <VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_REPORT as FromIntoMemory>::from_bytes(
                &from[40..40 + 4],
            );
        let f_VerifyReport =
            <VBS_BASIC_ENCLAVE_BASIC_CALL_VERIFY_REPORT as FromIntoMemory>::from_bytes(
                &from[44..44 + 4],
            );
        let f_GenerateRandomData =
            <VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_RANDOM_DATA as FromIntoMemory>::from_bytes(
                &from[48..48 + 4],
            );
        Self {
            ReturnFromEnclave: f_ReturnFromEnclave,
            ReturnFromException: f_ReturnFromException,
            TerminateThread: f_TerminateThread,
            InterruptThread: f_InterruptThread,
            CommitPages: f_CommitPages,
            DecommitPages: f_DecommitPages,
            ProtectPages: f_ProtectPages,
            CreateThread: f_CreateThread,
            GetEnclaveInformation: f_GetEnclaveInformation,
            GenerateKey: f_GenerateKey,
            GenerateReport: f_GenerateReport,
            VerifyReport: f_VerifyReport,
            GenerateRandomData: f_GenerateRandomData,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52u32 as usize);
        FromIntoMemory::into_bytes(self.ReturnFromEnclave, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ReturnFromException, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.TerminateThread, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.InterruptThread, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.CommitPages, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.DecommitPages, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ProtectPages, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.CreateThread, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.GetEnclaveInformation, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.GenerateKey, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.GenerateReport, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.VerifyReport, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.GenerateRandomData, &mut into[48..48 + 4]);
    }
    fn size() -> usize {
        4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4
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
        assert_eq!(from.len(), 36u32 as usize);
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
        assert_eq!(into.len(), 36u32 as usize);
        FromIntoMemory::into_bytes(self.ThreadContext, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.EntryPoint, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.StackPointer, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ExceptionEntryPoint, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ExceptionStack, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ExceptionActive, &mut into[32..32 + 4]);
    }
    fn size() -> usize {
        16 + 4 + 4 + 4 + 4 + 4
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
        assert_eq!(from.len(), 72u32 as usize);
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
        assert_eq!(into.len(), 72u32 as usize);
        FromIntoMemory::into_bytes(self.ThreadContext, &mut into[0..0 + 32]);
        FromIntoMemory::into_bytes(self.EntryPoint, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.StackPointer, &mut into[40..40 + 8]);
        FromIntoMemory::into_bytes(self.ExceptionEntryPoint, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.ExceptionStack, &mut into[56..56 + 8]);
        FromIntoMemory::into_bytes(self.ExceptionActive, &mut into[64..64 + 4]);
    }
    fn size() -> usize {
        32 + 8 + 8 + 8 + 8 + 4
    }
}
pub struct VBS_ENCLAVE_REPORT {
    pub ReportSize: u32,
    pub ReportVersion: u32,
    pub EnclaveData: [u8; 64],
    pub EnclaveIdentity: ENCLAVE_IDENTITY,
}
impl ::core::marker::Copy for VBS_ENCLAVE_REPORT {}
impl ::core::clone::Clone for VBS_ENCLAVE_REPORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for VBS_ENCLAVE_REPORT {
    fn eq(&self, other: &Self) -> bool {
        self.ReportSize == other.ReportSize
            && self.ReportVersion == other.ReportVersion
            && self.EnclaveData == other.EnclaveData
            && self.EnclaveIdentity == other.EnclaveIdentity
    }
}
impl ::core::cmp::Eq for VBS_ENCLAVE_REPORT {}
impl FromIntoMemory for VBS_ENCLAVE_REPORT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 224u32 as usize);
        let f_ReportSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ReportVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_EnclaveData = <[u8; 64] as FromIntoMemory>::from_bytes(&from[8..8 + 64]);
        let f_EnclaveIdentity =
            <ENCLAVE_IDENTITY as FromIntoMemory>::from_bytes(&from[72..72 + 152]);
        Self {
            ReportSize: f_ReportSize,
            ReportVersion: f_ReportVersion,
            EnclaveData: f_EnclaveData,
            EnclaveIdentity: f_EnclaveIdentity,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 224u32 as usize);
        FromIntoMemory::into_bytes(self.ReportSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ReportVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.EnclaveData, &mut into[8..8 + 64]);
        FromIntoMemory::into_bytes(self.EnclaveIdentity, &mut into[72..72 + 152]);
    }
    fn size() -> usize {
        4 + 4 + 64 + 152
    }
}
pub struct VBS_ENCLAVE_REPORT_MODULE {
    pub Header: VBS_ENCLAVE_REPORT_VARDATA_HEADER,
    pub UniqueId: [u8; 32],
    pub AuthorId: [u8; 32],
    pub FamilyId: [u8; 16],
    pub ImageId: [u8; 16],
    pub Svn: u32,
    pub ModuleName: [u16; 1],
}
impl ::core::marker::Copy for VBS_ENCLAVE_REPORT_MODULE {}
impl ::core::clone::Clone for VBS_ENCLAVE_REPORT_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for VBS_ENCLAVE_REPORT_MODULE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.UniqueId == other.UniqueId
            && self.AuthorId == other.AuthorId
            && self.FamilyId == other.FamilyId
            && self.ImageId == other.ImageId
            && self.Svn == other.Svn
            && self.ModuleName == other.ModuleName
    }
}
impl ::core::cmp::Eq for VBS_ENCLAVE_REPORT_MODULE {}
impl FromIntoMemory for VBS_ENCLAVE_REPORT_MODULE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 112u32 as usize);
        let f_Header =
            <VBS_ENCLAVE_REPORT_VARDATA_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_UniqueId = <[u8; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 32]);
        let f_AuthorId = <[u8; 32] as FromIntoMemory>::from_bytes(&from[40..40 + 32]);
        let f_FamilyId = <[u8; 16] as FromIntoMemory>::from_bytes(&from[72..72 + 16]);
        let f_ImageId = <[u8; 16] as FromIntoMemory>::from_bytes(&from[88..88 + 16]);
        let f_Svn = <u32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_ModuleName = <[u16; 1] as FromIntoMemory>::from_bytes(&from[108..108 + 1]);
        Self {
            Header: f_Header,
            UniqueId: f_UniqueId,
            AuthorId: f_AuthorId,
            FamilyId: f_FamilyId,
            ImageId: f_ImageId,
            Svn: f_Svn,
            ModuleName: f_ModuleName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 112u32 as usize);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.UniqueId, &mut into[8..8 + 32]);
        FromIntoMemory::into_bytes(self.AuthorId, &mut into[40..40 + 32]);
        FromIntoMemory::into_bytes(self.FamilyId, &mut into[72..72 + 16]);
        FromIntoMemory::into_bytes(self.ImageId, &mut into[88..88 + 16]);
        FromIntoMemory::into_bytes(self.Svn, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.ModuleName, &mut into[108..108 + 1]);
    }
    fn size() -> usize {
        8 + 32 + 32 + 16 + 16 + 4 + 1
    }
}
pub struct VBS_ENCLAVE_REPORT_PKG_HEADER {
    pub PackageSize: u32,
    pub Version: u32,
    pub SignatureScheme: u32,
    pub SignedStatementSize: u32,
    pub SignatureSize: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for VBS_ENCLAVE_REPORT_PKG_HEADER {}
impl ::core::clone::Clone for VBS_ENCLAVE_REPORT_PKG_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for VBS_ENCLAVE_REPORT_PKG_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.PackageSize == other.PackageSize
            && self.Version == other.Version
            && self.SignatureScheme == other.SignatureScheme
            && self.SignedStatementSize == other.SignedStatementSize
            && self.SignatureSize == other.SignatureSize
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for VBS_ENCLAVE_REPORT_PKG_HEADER {}
impl FromIntoMemory for VBS_ENCLAVE_REPORT_PKG_HEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24u32 as usize);
        let f_PackageSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Version = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SignatureScheme = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_SignedStatementSize = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_SignatureSize = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_Reserved = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            PackageSize: f_PackageSize,
            Version: f_Version,
            SignatureScheme: f_SignatureScheme,
            SignedStatementSize: f_SignedStatementSize,
            SignatureSize: f_SignatureSize,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24u32 as usize);
        FromIntoMemory::into_bytes(self.PackageSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Version, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SignatureScheme, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.SignedStatementSize, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.SignatureSize, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        4 + 4 + 4 + 4 + 4 + 4
    }
}
pub const VBS_ENCLAVE_REPORT_PKG_HEADER_VERSION_CURRENT: u32 = 1u32;
pub const VBS_ENCLAVE_REPORT_SIGNATURE_SCHEME_SHA256_RSA_PSS_SHA256: u32 = 1u32;
pub struct VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    pub DataType: u32,
    pub Size: u32,
}
impl ::core::marker::Copy for VBS_ENCLAVE_REPORT_VARDATA_HEADER {}
impl ::core::clone::Clone for VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.DataType == other.DataType && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for VBS_ENCLAVE_REPORT_VARDATA_HEADER {}
impl FromIntoMemory for VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8u32 as usize);
        let f_DataType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Size = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            DataType: f_DataType,
            Size: f_Size,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8u32 as usize);
        FromIntoMemory::into_bytes(self.DataType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Size, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        4 + 4
    }
}
pub const VBS_ENCLAVE_REPORT_VERSION_CURRENT: u32 = 1u32;
pub const VBS_ENCLAVE_VARDATA_INVALID: u32 = 0u32;
pub const VBS_ENCLAVE_VARDATA_MODULE: u32 = 1u32;
pub trait Api {
    fn CallEnclave(
        &self,
        lp_routine: PtrDiffRepr,
        lp_parameter: ConstPtr<::core::ffi::c_void>,
        f_wait_for_thread: super::super::Foundation::BOOL,
        lp_return_value: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> super::super::Foundation::BOOL {
        todo!("CallEnclave")
    }
    fn CreateEnclave(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_address: ConstPtr<::core::ffi::c_void>,
        dw_size: PtrRepr,
        dw_initial_commitment: PtrRepr,
        fl_enclave_type: u32,
        lp_enclave_information: ConstPtr<::core::ffi::c_void>,
        dw_info_length: u32,
        lp_enclave_error: MutPtr<u32>,
    ) -> MutPtr<::core::ffi::c_void> {
        todo!("CreateEnclave")
    }
    fn CreateEnvironmentBlock(
        &self,
        lp_environment: MutPtr<ConstPtr<::core::ffi::c_void>>,
        h_token: super::super::Foundation::HANDLE,
        b_inherit: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("CreateEnvironmentBlock")
    }
    fn DeleteEnclave(
        &self,
        lp_address: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("DeleteEnclave")
    }
    fn DestroyEnvironmentBlock(
        &self,
        lp_environment: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("DestroyEnvironmentBlock")
    }
    fn EnclaveGetAttestationReport(
        &self,
        enclave_data: ConstPtr<u8>,
        report: MutPtr<::core::ffi::c_void>,
        buffer_size: u32,
        output_size: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("EnclaveGetAttestationReport")
    }
    fn EnclaveGetEnclaveInformation(
        &self,
        information_size: u32,
        enclave_information: MutPtr<ENCLAVE_INFORMATION>,
    ) -> crate::core::HRESULT {
        todo!("EnclaveGetEnclaveInformation")
    }
    fn EnclaveSealData(
        &self,
        data_to_encrypt: ConstPtr<::core::ffi::c_void>,
        data_to_encrypt_size: u32,
        identity_policy: ENCLAVE_SEALING_IDENTITY_POLICY,
        runtime_policy: u32,
        protected_blob: MutPtr<::core::ffi::c_void>,
        buffer_size: u32,
        protected_blob_size: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("EnclaveSealData")
    }
    fn EnclaveUnsealData(
        &self,
        protected_blob: ConstPtr<::core::ffi::c_void>,
        protected_blob_size: u32,
        decrypted_data: MutPtr<::core::ffi::c_void>,
        buffer_size: u32,
        decrypted_data_size: MutPtr<u32>,
        sealing_identity: MutPtr<ENCLAVE_IDENTITY>,
        unsealing_flags: MutPtr<u32>,
    ) -> crate::core::HRESULT {
        todo!("EnclaveUnsealData")
    }
    fn EnclaveVerifyAttestationReport(
        &self,
        enclave_type: u32,
        report: ConstPtr<::core::ffi::c_void>,
        report_size: u32,
    ) -> crate::core::HRESULT {
        todo!("EnclaveVerifyAttestationReport")
    }
    fn ExpandEnvironmentStringsA(
        &self,
        lp_src: crate::core::PCSTR,
        lp_dst: crate::core::PSTR,
        n_size: u32,
    ) -> u32 {
        todo!("ExpandEnvironmentStringsA")
    }
    fn ExpandEnvironmentStringsForUserA(
        &self,
        h_token: super::super::Foundation::HANDLE,
        lp_src: crate::core::PCSTR,
        lp_dest: crate::core::PSTR,
        dw_size: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("ExpandEnvironmentStringsForUserA")
    }
    fn ExpandEnvironmentStringsForUserW(
        &self,
        h_token: super::super::Foundation::HANDLE,
        lp_src: crate::core::PCWSTR,
        lp_dest: crate::core::PWSTR,
        dw_size: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("ExpandEnvironmentStringsForUserW")
    }
    fn ExpandEnvironmentStringsW(
        &self,
        lp_src: crate::core::PCWSTR,
        lp_dst: crate::core::PWSTR,
        n_size: u32,
    ) -> u32 {
        todo!("ExpandEnvironmentStringsW")
    }
    fn FreeEnvironmentStringsA(&self, penv: crate::core::PCSTR) -> super::super::Foundation::BOOL {
        todo!("FreeEnvironmentStringsA")
    }
    fn FreeEnvironmentStringsW(&self, penv: crate::core::PCWSTR) -> super::super::Foundation::BOOL {
        todo!("FreeEnvironmentStringsW")
    }
    fn GetCommandLineA(&self) -> crate::core::PSTR {
        todo!("GetCommandLineA")
    }
    fn GetCommandLineW(&self) -> crate::core::PWSTR {
        todo!("GetCommandLineW")
    }
    fn GetCurrentDirectoryA(&self, n_buffer_length: u32, lp_buffer: crate::core::PSTR) -> u32 {
        todo!("GetCurrentDirectoryA")
    }
    fn GetCurrentDirectoryW(&self, n_buffer_length: u32, lp_buffer: crate::core::PWSTR) -> u32 {
        todo!("GetCurrentDirectoryW")
    }
    fn GetEnvironmentStrings(&self) -> crate::core::PSTR {
        todo!("GetEnvironmentStrings")
    }
    fn GetEnvironmentStringsW(&self) -> crate::core::PWSTR {
        todo!("GetEnvironmentStringsW")
    }
    fn GetEnvironmentVariableA(
        &self,
        lp_name: crate::core::PCSTR,
        lp_buffer: crate::core::PSTR,
        n_size: u32,
    ) -> u32 {
        todo!("GetEnvironmentVariableA")
    }
    fn GetEnvironmentVariableW(
        &self,
        lp_name: crate::core::PCWSTR,
        lp_buffer: crate::core::PWSTR,
        n_size: u32,
    ) -> u32 {
        todo!("GetEnvironmentVariableW")
    }
    fn InitializeEnclave(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_address: ConstPtr<::core::ffi::c_void>,
        lp_enclave_information: ConstPtr<::core::ffi::c_void>,
        dw_info_length: u32,
        lp_enclave_error: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("InitializeEnclave")
    }
    fn IsEnclaveTypeSupported(&self, fl_enclave_type: u32) -> super::super::Foundation::BOOL {
        todo!("IsEnclaveTypeSupported")
    }
    fn LoadEnclaveData(
        &self,
        h_process: super::super::Foundation::HANDLE,
        lp_address: ConstPtr<::core::ffi::c_void>,
        lp_buffer: ConstPtr<::core::ffi::c_void>,
        n_size: PtrRepr,
        fl_protect: u32,
        lp_page_information: ConstPtr<::core::ffi::c_void>,
        dw_info_length: u32,
        lp_number_of_bytes_written: MutPtr<PtrRepr>,
        lp_enclave_error: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("LoadEnclaveData")
    }
    fn LoadEnclaveImageA(
        &self,
        lp_enclave_address: ConstPtr<::core::ffi::c_void>,
        lp_image_name: crate::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("LoadEnclaveImageA")
    }
    fn LoadEnclaveImageW(
        &self,
        lp_enclave_address: ConstPtr<::core::ffi::c_void>,
        lp_image_name: crate::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("LoadEnclaveImageW")
    }
    fn NeedCurrentDirectoryForExePathA(
        &self,
        exe_name: crate::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("NeedCurrentDirectoryForExePathA")
    }
    fn NeedCurrentDirectoryForExePathW(
        &self,
        exe_name: crate::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("NeedCurrentDirectoryForExePathW")
    }
    fn SetCurrentDirectoryA(
        &self,
        lp_path_name: crate::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetCurrentDirectoryA")
    }
    fn SetCurrentDirectoryW(
        &self,
        lp_path_name: crate::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetCurrentDirectoryW")
    }
    fn SetEnvironmentStringsW(
        &self,
        new_environment: crate::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetEnvironmentStringsW")
    }
    fn SetEnvironmentVariableA(
        &self,
        lp_name: crate::core::PCSTR,
        lp_value: crate::core::PCSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetEnvironmentVariableA")
    }
    fn SetEnvironmentVariableW(
        &self,
        lp_name: crate::core::PCWSTR,
        lp_value: crate::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetEnvironmentVariableW")
    }
    fn TerminateEnclave(
        &self,
        lp_address: ConstPtr<::core::ffi::c_void>,
        f_wait: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("TerminateEnclave")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> &dyn Api {
    ctx.get::<dyn Api>()
}
