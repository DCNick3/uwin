#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[allow(unused)]
use win32::core::prelude::*;
pub const APPMODEL_ERROR_DYNAMIC_PROPERTY_INVALID: i32 = 15705i32;
pub const APPMODEL_ERROR_DYNAMIC_PROPERTY_READ_FAILED: i32 = 15704i32;
pub const APPMODEL_ERROR_NO_APPLICATION: i32 = 15703i32;
pub const APPMODEL_ERROR_NO_MUTABLE_DIRECTORY: i32 = 15707i32;
pub const APPMODEL_ERROR_NO_PACKAGE: i32 = 15700i32;
pub const APPMODEL_ERROR_PACKAGE_IDENTITY_CORRUPT: i32 = 15702i32;
pub const APPMODEL_ERROR_PACKAGE_NOT_AVAILABLE: i32 = 15706i32;
pub const APPMODEL_ERROR_PACKAGE_RUNTIME_CORRUPT: i32 = 15701i32;
pub const APPX_E_BLOCK_HASH_INVALID: crate::core::HRESULT = crate::core::HRESULT(-2146958841i32);
pub const APPX_E_CORRUPT_CONTENT: crate::core::HRESULT = crate::core::HRESULT(-2146958842i32);
pub const APPX_E_DELTA_APPENDED_PACKAGE_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2146958832i32);
pub const APPX_E_DELTA_BASELINE_VERSION_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2146958835i32);
pub const APPX_E_DELTA_PACKAGE_MISSING_FILE: crate::core::HRESULT =
    crate::core::HRESULT(-2146958834i32);
pub const APPX_E_FILE_COMPRESSION_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2146958828i32);
pub const APPX_E_INTERLEAVING_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2146958847i32);
pub const APPX_E_INVALID_APPINSTALLER: crate::core::HRESULT = crate::core::HRESULT(-2146958836i32);
pub const APPX_E_INVALID_BLOCKMAP: crate::core::HRESULT = crate::core::HRESULT(-2146958843i32);
pub const APPX_E_INVALID_CONTENTGROUPMAP: crate::core::HRESULT =
    crate::core::HRESULT(-2146958837i32);
pub const APPX_E_INVALID_DELTA_PACKAGE: crate::core::HRESULT = crate::core::HRESULT(-2146958833i32);
pub const APPX_E_INVALID_ENCRYPTION_EXCLUSION_FILE_LIST: crate::core::HRESULT =
    crate::core::HRESULT(-2146958826i32);
pub const APPX_E_INVALID_KEY_INFO: crate::core::HRESULT = crate::core::HRESULT(-2146958838i32);
pub const APPX_E_INVALID_MANIFEST: crate::core::HRESULT = crate::core::HRESULT(-2146958844i32);
pub const APPX_E_INVALID_PACKAGESIGNCONFIG: crate::core::HRESULT =
    crate::core::HRESULT(-2146958830i32);
pub const APPX_E_INVALID_PACKAGE_FOLDER_ACLS: crate::core::HRESULT =
    crate::core::HRESULT(-2146958825i32);
pub const APPX_E_INVALID_PACKAGING_LAYOUT: crate::core::HRESULT =
    crate::core::HRESULT(-2146958831i32);
pub const APPX_E_INVALID_PAYLOAD_PACKAGE_EXTENSION: crate::core::HRESULT =
    crate::core::HRESULT(-2146958827i32);
pub const APPX_E_INVALID_PUBLISHER_BRIDGING: crate::core::HRESULT =
    crate::core::HRESULT(-2146958824i32);
pub const APPX_E_INVALID_SIP_CLIENT_DATA: crate::core::HRESULT =
    crate::core::HRESULT(-2146958839i32);
pub const APPX_E_MISSING_REQUIRED_FILE: crate::core::HRESULT = crate::core::HRESULT(-2146958845i32);
pub const APPX_E_PACKAGING_INTERNAL: crate::core::HRESULT = crate::core::HRESULT(-2146958848i32);
pub const APPX_E_RELATIONSHIPS_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2146958846i32);
pub const APPX_E_REQUESTED_RANGE_TOO_LARGE: crate::core::HRESULT =
    crate::core::HRESULT(-2146958840i32);
pub const APPX_E_RESOURCESPRI_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2146958829i32);
pub struct APP_LOCAL_DEVICE_ID {
    pub value: [u8; 32],
}
impl ::core::marker::Copy for APP_LOCAL_DEVICE_ID {}
impl ::core::clone::Clone for APP_LOCAL_DEVICE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APP_LOCAL_DEVICE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APP_LOCAL_DEVICE_ID")
            .field("value", &self.value)
            .finish()
    }
}
impl ::core::cmp::PartialEq for APP_LOCAL_DEVICE_ID {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl ::core::cmp::Eq for APP_LOCAL_DEVICE_ID {}
impl FromIntoMemory for APP_LOCAL_DEVICE_ID {
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
pub const APP_LOCAL_DEVICE_ID_SIZE: u32 = 32u32;
pub struct BOOL(pub i32);
impl BOOL {
    #[inline]
    pub fn as_bool(self) -> bool {
        self.0 != 0
    }
}
impl ::core::default::Default for BOOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::clone::Clone for BOOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for BOOL {}
impl ::core::cmp::PartialEq for BOOL {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BOOL {}
impl ::core::fmt::Debug for BOOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BOOL").field(&self.0).finish()
    }
}
impl ::core::convert::From<BOOL> for bool {
    fn from(value: BOOL) -> Self {
        value.as_bool()
    }
}
impl ::core::convert::From<&BOOL> for bool {
    fn from(value: &BOOL) -> Self {
        value.as_bool()
    }
}
impl ::core::convert::From<bool> for BOOL {
    fn from(value: bool) -> Self {
        if value {
            BOOL(1)
        } else {
            BOOL(0)
        }
    }
}
impl ::core::convert::From<&bool> for BOOL {
    fn from(value: &bool) -> Self {
        (*value).into()
    }
}
impl ::core::cmp::PartialEq<bool> for BOOL {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}
impl ::core::cmp::PartialEq<BOOL> for bool {
    fn eq(&self, other: &BOOL) -> bool {
        *self == other.as_bool()
    }
}
impl ::core::ops::Not for BOOL {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self.as_bool() {
            BOOL(0)
        } else {
            BOOL(1)
        }
    }
}
impl FromIntoMemory for BOOL {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BOOLEAN(pub u8);
impl BOOLEAN {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BOOLEAN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for BOOLEAN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for BOOLEAN {}
impl ::core::fmt::Debug for BOOLEAN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BOOLEAN").field(&self.0).finish()
    }
}
impl FromIntoMemory for BOOLEAN {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u8 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<u8>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BSTR(pub MutPtr<u16>);
impl BSTR {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BSTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for BSTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for BSTR {}
impl ::core::fmt::Debug for BSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BSTR").field(&self.0).finish()
    }
}
impl FromIntoMemory for BSTR {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<MutPtr<u16> as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<MutPtr<u16>>()
    }
}
pub const BT_E_SPURIOUS_ACTIVATION: crate::core::HRESULT = crate::core::HRESULT(-2146958592i32);
pub const CACHE_E_FIRST: i32 = -2147221136i32;
pub const CACHE_E_LAST: i32 = -2147221121i32;
pub const CACHE_E_NOCACHE_UPDATED: crate::core::HRESULT = crate::core::HRESULT(-2147221136i32);
pub const CACHE_S_FIRST: i32 = 262512i32;
pub const CACHE_S_FORMATETC_NOTSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(262512i32);
pub const CACHE_S_LAST: i32 = 262527i32;
pub const CACHE_S_SAMECACHE: crate::core::HRESULT = crate::core::HRESULT(262513i32);
pub const CACHE_S_SOMECACHES_NOTUPDATED: crate::core::HRESULT = crate::core::HRESULT(262514i32);
pub const CAT_E_CATIDNOEXIST: crate::core::HRESULT = crate::core::HRESULT(-2147221152i32);
pub const CAT_E_FIRST: i32 = -2147221152i32;
pub const CAT_E_LAST: i32 = -2147221151i32;
pub const CAT_E_NODESCRIPTION: crate::core::HRESULT = crate::core::HRESULT(-2147221151i32);
pub const CERTSRV_E_ADMIN_DENIED_REQUEST: crate::core::HRESULT =
    crate::core::HRESULT(-2146877420i32);
pub const CERTSRV_E_ALIGNMENT_FAULT: crate::core::HRESULT = crate::core::HRESULT(-2146877424i32);
pub const CERTSRV_E_ARCHIVED_KEY_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2146875388i32);
pub const CERTSRV_E_ARCHIVED_KEY_UNEXPECTED: crate::core::HRESULT =
    crate::core::HRESULT(-2146875376i32);
pub const CERTSRV_E_BAD_RENEWAL_CERT_ATTRIBUTE: crate::core::HRESULT =
    crate::core::HRESULT(-2146877426i32);
pub const CERTSRV_E_BAD_RENEWAL_SUBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-2146875386i32);
pub const CERTSRV_E_BAD_REQUESTSTATUS: crate::core::HRESULT = crate::core::HRESULT(-2146877437i32);
pub const CERTSRV_E_BAD_REQUESTSUBJECT: crate::core::HRESULT = crate::core::HRESULT(-2146877439i32);
pub const CERTSRV_E_BAD_REQUEST_KEY_ARCHIVAL: crate::core::HRESULT =
    crate::core::HRESULT(-2146877428i32);
pub const CERTSRV_E_BAD_TEMPLATE_VERSION: crate::core::HRESULT =
    crate::core::HRESULT(-2146875385i32);
pub const CERTSRV_E_CERT_TYPE_OVERLAP: crate::core::HRESULT = crate::core::HRESULT(-2146875372i32);
pub const CERTSRV_E_CORRUPT_KEY_ATTESTATION: crate::core::HRESULT =
    crate::core::HRESULT(-2146875365i32);
pub const CERTSRV_E_DOWNLEVEL_DC_SSL_OR_UPGRADE: crate::core::HRESULT =
    crate::core::HRESULT(-2146877421i32);
pub const CERTSRV_E_ENCODING_LENGTH: crate::core::HRESULT = crate::core::HRESULT(-2146877433i32);
pub const CERTSRV_E_ENCRYPTION_CERT_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2146877416i32);
pub const CERTSRV_E_ENROLL_DENIED: crate::core::HRESULT = crate::core::HRESULT(-2146877423i32);
pub const CERTSRV_E_EXPIRED_CHALLENGE: crate::core::HRESULT = crate::core::HRESULT(-2146875364i32);
pub const CERTSRV_E_INVALID_ATTESTATION: crate::core::HRESULT =
    crate::core::HRESULT(-2146875367i32);
pub const CERTSRV_E_INVALID_CA_CERTIFICATE: crate::core::HRESULT =
    crate::core::HRESULT(-2146877435i32);
pub const CERTSRV_E_INVALID_EK: crate::core::HRESULT = crate::core::HRESULT(-2146875369i32);
pub const CERTSRV_E_INVALID_IDBINDING: crate::core::HRESULT = crate::core::HRESULT(-2146875368i32);
pub const CERTSRV_E_INVALID_REQUESTID: crate::core::HRESULT = crate::core::HRESULT(-2146875362i32);
pub const CERTSRV_E_INVALID_RESPONSE: crate::core::HRESULT = crate::core::HRESULT(-2146875363i32);
pub const CERTSRV_E_ISSUANCE_POLICY_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2146875380i32);
pub const CERTSRV_E_KEY_ARCHIVAL_NOT_CONFIGURED: crate::core::HRESULT =
    crate::core::HRESULT(-2146877430i32);
pub const CERTSRV_E_KEY_ATTESTATION: crate::core::HRESULT = crate::core::HRESULT(-2146875366i32);
pub const CERTSRV_E_KEY_ATTESTATION_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2146877417i32);
pub const CERTSRV_E_KEY_LENGTH: crate::core::HRESULT = crate::core::HRESULT(-2146875375i32);
pub const CERTSRV_E_NO_CAADMIN_DEFINED: crate::core::HRESULT = crate::core::HRESULT(-2146877427i32);
pub const CERTSRV_E_NO_CERT_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2146875391i32);
pub const CERTSRV_E_NO_DB_SESSIONS: crate::core::HRESULT = crate::core::HRESULT(-2146877425i32);
pub const CERTSRV_E_NO_POLICY_SERVER: crate::core::HRESULT = crate::core::HRESULT(-2146877419i32);
pub const CERTSRV_E_NO_REQUEST: crate::core::HRESULT = crate::core::HRESULT(-2146877438i32);
pub const CERTSRV_E_NO_VALID_KRA: crate::core::HRESULT = crate::core::HRESULT(-2146877429i32);
pub const CERTSRV_E_PENDING_CLIENT_RESPONSE: crate::core::HRESULT =
    crate::core::HRESULT(-2146875360i32);
pub const CERTSRV_E_PROPERTY_EMPTY: crate::core::HRESULT = crate::core::HRESULT(-2146877436i32);
pub const CERTSRV_E_RENEWAL_BAD_PUBLIC_KEY: crate::core::HRESULT =
    crate::core::HRESULT(-2146875370i32);
pub const CERTSRV_E_REQUEST_PRECERTIFICATE_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2146875361i32);
pub const CERTSRV_E_RESTRICTEDOFFICER: crate::core::HRESULT = crate::core::HRESULT(-2146877431i32);
pub const CERTSRV_E_ROLECONFLICT: crate::core::HRESULT = crate::core::HRESULT(-2146877432i32);
pub const CERTSRV_E_SERVER_SUSPENDED: crate::core::HRESULT = crate::core::HRESULT(-2146877434i32);
pub const CERTSRV_E_SIGNATURE_COUNT: crate::core::HRESULT = crate::core::HRESULT(-2146875382i32);
pub const CERTSRV_E_SIGNATURE_POLICY_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2146875383i32);
pub const CERTSRV_E_SIGNATURE_REJECTED: crate::core::HRESULT = crate::core::HRESULT(-2146875381i32);
pub const CERTSRV_E_SMIME_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2146875387i32);
pub const CERTSRV_E_SUBJECT_ALT_NAME_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2146875389i32);
pub const CERTSRV_E_SUBJECT_DIRECTORY_GUID_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2146875378i32);
pub const CERTSRV_E_SUBJECT_DNS_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2146875377i32);
pub const CERTSRV_E_SUBJECT_EMAIL_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2146875374i32);
pub const CERTSRV_E_SUBJECT_UPN_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2146875379i32);
pub const CERTSRV_E_TEMPLATE_CONFLICT: crate::core::HRESULT = crate::core::HRESULT(-2146875390i32);
pub const CERTSRV_E_TEMPLATE_DENIED: crate::core::HRESULT = crate::core::HRESULT(-2146877422i32);
pub const CERTSRV_E_TEMPLATE_POLICY_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2146875384i32);
pub const CERTSRV_E_TOO_MANY_SIGNATURES: crate::core::HRESULT =
    crate::core::HRESULT(-2146875371i32);
pub const CERTSRV_E_UNKNOWN_CERT_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2146875373i32);
pub const CERTSRV_E_UNSUPPORTED_CERT_TYPE: crate::core::HRESULT =
    crate::core::HRESULT(-2146875392i32);
pub const CERTSRV_E_WEAK_SIGNATURE_OR_KEY: crate::core::HRESULT =
    crate::core::HRESULT(-2146877418i32);
pub const CERT_E_CHAINING: crate::core::HRESULT = crate::core::HRESULT(-2146762486i32);
pub const CERT_E_CN_NO_MATCH: crate::core::HRESULT = crate::core::HRESULT(-2146762481i32);
pub const CERT_E_CRITICAL: crate::core::HRESULT = crate::core::HRESULT(-2146762491i32);
pub const CERT_E_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-2146762495i32);
pub const CERT_E_INVALID_NAME: crate::core::HRESULT = crate::core::HRESULT(-2146762476i32);
pub const CERT_E_INVALID_POLICY: crate::core::HRESULT = crate::core::HRESULT(-2146762477i32);
pub const CERT_E_ISSUERCHAINING: crate::core::HRESULT = crate::core::HRESULT(-2146762489i32);
pub const CERT_E_MALFORMED: crate::core::HRESULT = crate::core::HRESULT(-2146762488i32);
pub const CERT_E_PATHLENCONST: crate::core::HRESULT = crate::core::HRESULT(-2146762492i32);
pub const CERT_E_PURPOSE: crate::core::HRESULT = crate::core::HRESULT(-2146762490i32);
pub const CERT_E_REVOCATION_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2146762482i32);
pub const CERT_E_REVOKED: crate::core::HRESULT = crate::core::HRESULT(-2146762484i32);
pub const CERT_E_ROLE: crate::core::HRESULT = crate::core::HRESULT(-2146762493i32);
pub const CERT_E_UNTRUSTEDCA: crate::core::HRESULT = crate::core::HRESULT(-2146762478i32);
pub const CERT_E_UNTRUSTEDROOT: crate::core::HRESULT = crate::core::HRESULT(-2146762487i32);
pub const CERT_E_UNTRUSTEDTESTROOT: crate::core::HRESULT = crate::core::HRESULT(-2146762483i32);
pub const CERT_E_VALIDITYPERIODNESTING: crate::core::HRESULT = crate::core::HRESULT(-2146762494i32);
pub const CERT_E_WRONG_USAGE: crate::core::HRESULT = crate::core::HRESULT(-2146762480i32);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CHAR(pub u8);
impl CHAR {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CHAR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CHAR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CHAR {}
impl ::core::fmt::Debug for CHAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHAR").field(&self.0).finish()
    }
}
impl FromIntoMemory for CHAR {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u8 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<u8>()
    }
}
pub const CI_CORRUPT_CATALOG: crate::core::HRESULT = crate::core::HRESULT(-1073473535i32);
pub const CI_CORRUPT_DATABASE: crate::core::HRESULT = crate::core::HRESULT(-1073473536i32);
pub const CI_CORRUPT_FILTER_BUFFER: crate::core::HRESULT = crate::core::HRESULT(-1073473529i32);
pub const CI_E_ALREADY_INITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2147215350i32);
pub const CI_E_BUFFERTOOSMALL: crate::core::HRESULT = crate::core::HRESULT(-2147215348i32);
pub const CI_E_CARDINALITY_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2147215321i32);
pub const CI_E_CLIENT_FILTER_ABORT: crate::core::HRESULT = crate::core::HRESULT(-1073473500i32);
pub const CI_E_CONFIG_DISK_FULL: crate::core::HRESULT = crate::core::HRESULT(-2147215320i32);
pub const CI_E_DISK_FULL: crate::core::HRESULT = crate::core::HRESULT(-2147215343i32);
pub const CI_E_DISTRIBUTED_GROUPBY_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2147215319i32);
pub const CI_E_DUPLICATE_NOTIFICATION: crate::core::HRESULT = crate::core::HRESULT(-2147215337i32);
pub const CI_E_ENUMERATION_STARTED: crate::core::HRESULT = crate::core::HRESULT(-1073473502i32);
pub const CI_E_FILTERING_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2147215344i32);
pub const CI_E_INVALID_FLAGS_COMBINATION: crate::core::HRESULT =
    crate::core::HRESULT(-2147215335i32);
pub const CI_E_INVALID_STATE: crate::core::HRESULT = crate::core::HRESULT(-2147215345i32);
pub const CI_E_LOGON_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2147215332i32);
pub const CI_E_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2147215339i32);
pub const CI_E_NOT_INITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2147215349i32);
pub const CI_E_NOT_RUNNING: crate::core::HRESULT = crate::core::HRESULT(-2147215328i32);
pub const CI_E_NO_CATALOG: crate::core::HRESULT = crate::core::HRESULT(-2147215331i32);
pub const CI_E_OUTOFSEQ_INCREMENT_DATA: crate::core::HRESULT = crate::core::HRESULT(-2147215334i32);
pub const CI_E_PROPERTY_NOT_CACHED: crate::core::HRESULT = crate::core::HRESULT(-2147215347i32);
pub const CI_E_PROPERTY_TOOLARGE: crate::core::HRESULT = crate::core::HRESULT(-1073473501i32);
pub const CI_E_SHARING_VIOLATION: crate::core::HRESULT = crate::core::HRESULT(-2147215333i32);
pub const CI_E_SHUTDOWN: crate::core::HRESULT = crate::core::HRESULT(-2147215342i32);
pub const CI_E_STRANGE_PAGEORSECTOR_SIZE: crate::core::HRESULT =
    crate::core::HRESULT(-2147215330i32);
pub const CI_E_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2147215329i32);
pub const CI_E_UPDATES_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2147215336i32);
pub const CI_E_USE_DEFAULT_PID: crate::core::HRESULT = crate::core::HRESULT(-2147215338i32);
pub const CI_E_WORKID_NOTVALID: crate::core::HRESULT = crate::core::HRESULT(-2147215341i32);
pub const CI_INCORRECT_VERSION: crate::core::HRESULT = crate::core::HRESULT(-1073473503i32);
pub const CI_INVALID_INDEX: crate::core::HRESULT = crate::core::HRESULT(-1073473528i32);
pub const CI_INVALID_PARTITION: crate::core::HRESULT = crate::core::HRESULT(-1073473534i32);
pub const CI_INVALID_PRIORITY: crate::core::HRESULT = crate::core::HRESULT(-1073473533i32);
pub const CI_NO_CATALOG: crate::core::HRESULT = crate::core::HRESULT(-1073473530i32);
pub const CI_NO_STARTING_KEY: crate::core::HRESULT = crate::core::HRESULT(-1073473532i32);
pub const CI_OUT_OF_INDEX_IDS: crate::core::HRESULT = crate::core::HRESULT(-1073473531i32);
pub const CI_PROPSTORE_INCONSISTENCY: crate::core::HRESULT = crate::core::HRESULT(-1073473527i32);
pub const CI_S_CAT_STOPPED: crate::core::HRESULT = crate::core::HRESULT(268326i32);
pub const CI_S_END_OF_ENUMERATION: crate::core::HRESULT = crate::core::HRESULT(268308i32);
pub const CI_S_NO_DOCSTORE: crate::core::HRESULT = crate::core::HRESULT(268325i32);
pub const CI_S_WORKID_DELETED: crate::core::HRESULT = crate::core::HRESULT(268302i32);
pub const CLASSFACTORY_E_FIRST: i32 = -2147221232i32;
pub const CLASSFACTORY_E_LAST: i32 = -2147221217i32;
pub const CLASSFACTORY_S_FIRST: i32 = 262416i32;
pub const CLASSFACTORY_S_LAST: i32 = 262431i32;
pub const CLASS_E_CLASSNOTAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2147221231i32);
pub const CLASS_E_NOAGGREGATION: crate::core::HRESULT = crate::core::HRESULT(-2147221232i32);
pub const CLASS_E_NOTLICENSED: crate::core::HRESULT = crate::core::HRESULT(-2147221230i32);
pub const CLIENTSITE_E_FIRST: i32 = -2147221104i32;
pub const CLIENTSITE_E_LAST: i32 = -2147221089i32;
pub const CLIENTSITE_S_FIRST: i32 = 262544i32;
pub const CLIENTSITE_S_LAST: i32 = 262559i32;
pub const CLIPBRD_E_BAD_DATA: crate::core::HRESULT = crate::core::HRESULT(-2147221037i32);
pub const CLIPBRD_E_CANT_CLOSE: crate::core::HRESULT = crate::core::HRESULT(-2147221036i32);
pub const CLIPBRD_E_CANT_EMPTY: crate::core::HRESULT = crate::core::HRESULT(-2147221039i32);
pub const CLIPBRD_E_CANT_OPEN: crate::core::HRESULT = crate::core::HRESULT(-2147221040i32);
pub const CLIPBRD_E_CANT_SET: crate::core::HRESULT = crate::core::HRESULT(-2147221038i32);
pub const CLIPBRD_E_FIRST: i32 = -2147221040i32;
pub const CLIPBRD_E_LAST: i32 = -2147221025i32;
pub const CLIPBRD_S_FIRST: i32 = 262608i32;
pub const CLIPBRD_S_LAST: i32 = 262623i32;
pub const COMADMIN_E_ALREADYINSTALLED: crate::core::HRESULT = crate::core::HRESULT(-2146368508i32);
pub const COMADMIN_E_AMBIGUOUS_APPLICATION_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-2146368420i32);
pub const COMADMIN_E_AMBIGUOUS_PARTITION_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-2146368419i32);
pub const COMADMIN_E_APPDIRNOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2146368481i32);
pub const COMADMIN_E_APPLICATIONEXISTS: crate::core::HRESULT = crate::core::HRESULT(-2146368501i32);
pub const COMADMIN_E_APPLID_MATCHES_CLSID: crate::core::HRESULT =
    crate::core::HRESULT(-2146368442i32);
pub const COMADMIN_E_APP_FILE_READFAIL: crate::core::HRESULT = crate::core::HRESULT(-2146368504i32);
pub const COMADMIN_E_APP_FILE_VERSION: crate::core::HRESULT = crate::core::HRESULT(-2146368503i32);
pub const COMADMIN_E_APP_FILE_WRITEFAIL: crate::core::HRESULT =
    crate::core::HRESULT(-2146368505i32);
pub const COMADMIN_E_APP_NOT_RUNNING: crate::core::HRESULT = crate::core::HRESULT(-2146367478i32);
pub const COMADMIN_E_AUTHENTICATIONLEVEL: crate::core::HRESULT =
    crate::core::HRESULT(-2146368493i32);
pub const COMADMIN_E_BADPATH: crate::core::HRESULT = crate::core::HRESULT(-2146368502i32);
pub const COMADMIN_E_BADREGISTRYLIBID: crate::core::HRESULT = crate::core::HRESULT(-2146368482i32);
pub const COMADMIN_E_BADREGISTRYPROGID: crate::core::HRESULT = crate::core::HRESULT(-2146368494i32);
pub const COMADMIN_E_BASEPARTITION_REQUIRED_IN_SET: crate::core::HRESULT =
    crate::core::HRESULT(-2146367457i32);
pub const COMADMIN_E_BASE_PARTITION_ONLY: crate::core::HRESULT =
    crate::core::HRESULT(-2146368432i32);
pub const COMADMIN_E_CANNOT_ALIAS_EVENTCLASS: crate::core::HRESULT =
    crate::core::HRESULT(-2146367456i32);
pub const COMADMIN_E_CANTCOPYFILE: crate::core::HRESULT = crate::core::HRESULT(-2146368499i32);
pub const COMADMIN_E_CANTMAKEINPROCSERVICE: crate::core::HRESULT =
    crate::core::HRESULT(-2146367468i32);
pub const COMADMIN_E_CANTRECYCLELIBRARYAPPS: crate::core::HRESULT =
    crate::core::HRESULT(-2146367473i32);
pub const COMADMIN_E_CANTRECYCLESERVICEAPPS: crate::core::HRESULT =
    crate::core::HRESULT(-2146367471i32);
pub const COMADMIN_E_CANT_SUBSCRIBE_TO_COMPONENT: crate::core::HRESULT =
    crate::core::HRESULT(-2146368435i32);
pub const COMADMIN_E_CAN_NOT_EXPORT_APP_PROXY: crate::core::HRESULT =
    crate::core::HRESULT(-2146368438i32);
pub const COMADMIN_E_CAN_NOT_EXPORT_SYS_APP: crate::core::HRESULT =
    crate::core::HRESULT(-2146368436i32);
pub const COMADMIN_E_CAN_NOT_START_APP: crate::core::HRESULT = crate::core::HRESULT(-2146368437i32);
pub const COMADMIN_E_CAT_BITNESSMISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2146368382i32);
pub const COMADMIN_E_CAT_DUPLICATE_PARTITION_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-2146368425i32);
pub const COMADMIN_E_CAT_IMPORTED_COMPONENTS_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2146368421i32);
pub const COMADMIN_E_CAT_INVALID_PARTITION_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-2146368424i32);
pub const COMADMIN_E_CAT_PARTITION_IN_USE: crate::core::HRESULT =
    crate::core::HRESULT(-2146368423i32);
pub const COMADMIN_E_CAT_PAUSE_RESUME_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2146368379i32);
pub const COMADMIN_E_CAT_SERVERFAULT: crate::core::HRESULT = crate::core::HRESULT(-2146368378i32);
pub const COMADMIN_E_CAT_UNACCEPTABLEBITNESS: crate::core::HRESULT =
    crate::core::HRESULT(-2146368381i32);
pub const COMADMIN_E_CAT_WRONGAPPBITNESS: crate::core::HRESULT =
    crate::core::HRESULT(-2146368380i32);
pub const COMADMIN_E_CLSIDORIIDMISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2146368488i32);
pub const COMADMIN_E_COMPFILE_BADTLB: crate::core::HRESULT = crate::core::HRESULT(-2146368472i32);
pub const COMADMIN_E_COMPFILE_CLASSNOTAVAIL: crate::core::HRESULT =
    crate::core::HRESULT(-2146368473i32);
pub const COMADMIN_E_COMPFILE_DOESNOTEXIST: crate::core::HRESULT =
    crate::core::HRESULT(-2146368476i32);
pub const COMADMIN_E_COMPFILE_GETCLASSOBJ: crate::core::HRESULT =
    crate::core::HRESULT(-2146368474i32);
pub const COMADMIN_E_COMPFILE_LOADDLLFAIL: crate::core::HRESULT =
    crate::core::HRESULT(-2146368475i32);
pub const COMADMIN_E_COMPFILE_NOREGISTRAR: crate::core::HRESULT =
    crate::core::HRESULT(-2146368460i32);
pub const COMADMIN_E_COMPFILE_NOTINSTALLABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2146368471i32);
pub const COMADMIN_E_COMPONENTEXISTS: crate::core::HRESULT = crate::core::HRESULT(-2146368455i32);
pub const COMADMIN_E_COMP_MOVE_BAD_DEST: crate::core::HRESULT =
    crate::core::HRESULT(-2146368466i32);
pub const COMADMIN_E_COMP_MOVE_DEST: crate::core::HRESULT = crate::core::HRESULT(-2146367459i32);
pub const COMADMIN_E_COMP_MOVE_LOCKED: crate::core::HRESULT = crate::core::HRESULT(-2146368467i32);
pub const COMADMIN_E_COMP_MOVE_PRIVATE: crate::core::HRESULT = crate::core::HRESULT(-2146367458i32);
pub const COMADMIN_E_COMP_MOVE_SOURCE: crate::core::HRESULT = crate::core::HRESULT(-2146367460i32);
pub const COMADMIN_E_COREQCOMPINSTALLED: crate::core::HRESULT =
    crate::core::HRESULT(-2146368459i32);
pub const COMADMIN_E_DEFAULT_PARTITION_NOT_IN_SET: crate::core::HRESULT =
    crate::core::HRESULT(-2146367466i32);
pub const COMADMIN_E_DLLLOADFAILED: crate::core::HRESULT = crate::core::HRESULT(-2146368483i32);
pub const COMADMIN_E_DLLREGISTERSERVER: crate::core::HRESULT = crate::core::HRESULT(-2146368486i32);
pub const COMADMIN_E_EVENTCLASS_CANT_BE_SUBSCRIBER: crate::core::HRESULT =
    crate::core::HRESULT(-2146368434i32);
pub const COMADMIN_E_FILE_PARTITION_DUPLICATE_FILES: crate::core::HRESULT =
    crate::core::HRESULT(-2146368422i32);
pub const COMADMIN_E_INVALIDUSERIDS: crate::core::HRESULT = crate::core::HRESULT(-2146368496i32);
pub const COMADMIN_E_INVALID_PARTITION: crate::core::HRESULT = crate::core::HRESULT(-2146367477i32);
pub const COMADMIN_E_KEYMISSING: crate::core::HRESULT = crate::core::HRESULT(-2146368509i32);
pub const COMADMIN_E_LEGACYCOMPS_NOT_ALLOWED_IN_1_0_FORMAT: crate::core::HRESULT =
    crate::core::HRESULT(-2146367462i32);
pub const COMADMIN_E_LEGACYCOMPS_NOT_ALLOWED_IN_NONBASE_PARTITIONS: crate::core::HRESULT =
    crate::core::HRESULT(-2146367461i32);
pub const COMADMIN_E_LIB_APP_PROXY_INCOMPATIBLE: crate::core::HRESULT =
    crate::core::HRESULT(-2146368433i32);
pub const COMADMIN_E_MIG_SCHEMANOTFOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2146368383i32);
pub const COMADMIN_E_MIG_VERSIONNOTSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2146368384i32);
pub const COMADMIN_E_NOREGISTRYCLSID: crate::core::HRESULT = crate::core::HRESULT(-2146368495i32);
pub const COMADMIN_E_NOSERVERSHARE: crate::core::HRESULT = crate::core::HRESULT(-2146368485i32);
pub const COMADMIN_E_NOTCHANGEABLE: crate::core::HRESULT = crate::core::HRESULT(-2146368470i32);
pub const COMADMIN_E_NOTDELETEABLE: crate::core::HRESULT = crate::core::HRESULT(-2146368469i32);
pub const COMADMIN_E_NOTINREGISTRY: crate::core::HRESULT = crate::core::HRESULT(-2146368450i32);
pub const COMADMIN_E_NOUSER: crate::core::HRESULT = crate::core::HRESULT(-2146368497i32);
pub const COMADMIN_E_OBJECTERRORS: crate::core::HRESULT = crate::core::HRESULT(-2146368511i32);
pub const COMADMIN_E_OBJECTEXISTS: crate::core::HRESULT = crate::core::HRESULT(-2146368456i32);
pub const COMADMIN_E_OBJECTINVALID: crate::core::HRESULT = crate::core::HRESULT(-2146368510i32);
pub const COMADMIN_E_OBJECTNOTPOOLABLE: crate::core::HRESULT = crate::core::HRESULT(-2146368449i32);
pub const COMADMIN_E_OBJECT_DOES_NOT_EXIST: crate::core::HRESULT =
    crate::core::HRESULT(-2146367479i32);
pub const COMADMIN_E_OBJECT_PARENT_MISSING: crate::core::HRESULT =
    crate::core::HRESULT(-2146367480i32);
pub const COMADMIN_E_PARTITIONS_DISABLED: crate::core::HRESULT =
    crate::core::HRESULT(-2146367452i32);
pub const COMADMIN_E_PARTITION_ACCESSDENIED: crate::core::HRESULT =
    crate::core::HRESULT(-2146367464i32);
pub const COMADMIN_E_PARTITION_MSI_ONLY: crate::core::HRESULT =
    crate::core::HRESULT(-2146367463i32);
pub const COMADMIN_E_PAUSEDPROCESSMAYNOTBERECYCLED: crate::core::HRESULT =
    crate::core::HRESULT(-2146367469i32);
pub const COMADMIN_E_PRIVATE_ACCESSDENIED: crate::core::HRESULT =
    crate::core::HRESULT(-2146367455i32);
pub const COMADMIN_E_PROCESSALREADYRECYCLED: crate::core::HRESULT =
    crate::core::HRESULT(-2146367470i32);
pub const COMADMIN_E_PROGIDINUSEBYCLSID: crate::core::HRESULT =
    crate::core::HRESULT(-2146367467i32);
pub const COMADMIN_E_PROPERTYSAVEFAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2146368457i32);
pub const COMADMIN_E_PROPERTY_OVERFLOW: crate::core::HRESULT = crate::core::HRESULT(-2146368452i32);
pub const COMADMIN_E_RECYCLEDPROCESSMAYNOTBEPAUSED: crate::core::HRESULT =
    crate::core::HRESULT(-2146367465i32);
pub const COMADMIN_E_REGDB_ALREADYRUNNING: crate::core::HRESULT =
    crate::core::HRESULT(-2146368395i32);
pub const COMADMIN_E_REGDB_NOTINITIALIZED: crate::core::HRESULT =
    crate::core::HRESULT(-2146368398i32);
pub const COMADMIN_E_REGDB_NOTOPEN: crate::core::HRESULT = crate::core::HRESULT(-2146368397i32);
pub const COMADMIN_E_REGDB_SYSTEMERR: crate::core::HRESULT = crate::core::HRESULT(-2146368396i32);
pub const COMADMIN_E_REGFILE_CORRUPT: crate::core::HRESULT = crate::core::HRESULT(-2146368453i32);
pub const COMADMIN_E_REGISTERTLB: crate::core::HRESULT = crate::core::HRESULT(-2146368464i32);
pub const COMADMIN_E_REGISTRARFAILED: crate::core::HRESULT = crate::core::HRESULT(-2146368477i32);
pub const COMADMIN_E_REGISTRY_ACCESSDENIED: crate::core::HRESULT =
    crate::core::HRESULT(-2146367453i32);
pub const COMADMIN_E_REMOTEINTERFACE: crate::core::HRESULT = crate::core::HRESULT(-2146368487i32);
pub const COMADMIN_E_REQUIRES_DIFFERENT_PLATFORM: crate::core::HRESULT =
    crate::core::HRESULT(-2146368439i32);
pub const COMADMIN_E_ROLEEXISTS: crate::core::HRESULT = crate::core::HRESULT(-2146368500i32);
pub const COMADMIN_E_ROLE_DOES_NOT_EXIST: crate::core::HRESULT =
    crate::core::HRESULT(-2146368441i32);
pub const COMADMIN_E_SAFERINVALID: crate::core::HRESULT = crate::core::HRESULT(-2146367454i32);
pub const COMADMIN_E_SERVICENOTINSTALLED: crate::core::HRESULT =
    crate::core::HRESULT(-2146368458i32);
pub const COMADMIN_E_SESSION: crate::core::HRESULT = crate::core::HRESULT(-2146368468i32);
pub const COMADMIN_E_START_APP_DISABLED: crate::core::HRESULT =
    crate::core::HRESULT(-2146368431i32);
pub const COMADMIN_E_START_APP_NEEDS_COMPONENTS: crate::core::HRESULT =
    crate::core::HRESULT(-2146368440i32);
pub const COMADMIN_E_SVCAPP_NOT_POOLABLE_OR_RECYCLABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2146367475i32);
pub const COMADMIN_E_SYSTEMAPP: crate::core::HRESULT = crate::core::HRESULT(-2146368461i32);
pub const COMADMIN_E_USERPASSWDNOTVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2146368492i32);
pub const COMADMIN_E_USER_IN_SET: crate::core::HRESULT = crate::core::HRESULT(-2146367474i32);
pub const COMQC_E_APPLICATION_NOT_QUEUED: crate::core::HRESULT =
    crate::core::HRESULT(-2146368000i32);
pub const COMQC_E_BAD_MESSAGE: crate::core::HRESULT = crate::core::HRESULT(-2146367996i32);
pub const COMQC_E_NO_IPERSISTSTREAM: crate::core::HRESULT = crate::core::HRESULT(-2146367997i32);
pub const COMQC_E_NO_QUEUEABLE_INTERFACES: crate::core::HRESULT =
    crate::core::HRESULT(-2146367999i32);
pub const COMQC_E_QUEUING_SERVICE_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2146367998i32);
pub const COMQC_E_UNAUTHENTICATED: crate::core::HRESULT = crate::core::HRESULT(-2146367995i32);
pub const COMQC_E_UNTRUSTED_ENQUEUER: crate::core::HRESULT = crate::core::HRESULT(-2146367994i32);
pub const CONTEXT_E_ABORTED: crate::core::HRESULT = crate::core::HRESULT(-2147164158i32);
pub const CONTEXT_E_ABORTING: crate::core::HRESULT = crate::core::HRESULT(-2147164157i32);
pub const CONTEXT_E_FIRST: i32 = -2147164160i32;
pub const CONTEXT_E_LAST: i32 = -2147164113i32;
pub const CONTEXT_E_NOCONTEXT: crate::core::HRESULT = crate::core::HRESULT(-2147164156i32);
pub const CONTEXT_E_NOJIT: crate::core::HRESULT = crate::core::HRESULT(-2147164122i32);
pub const CONTEXT_E_NOTRANSACTION: crate::core::HRESULT = crate::core::HRESULT(-2147164121i32);
pub const CONTEXT_E_OLDREF: crate::core::HRESULT = crate::core::HRESULT(-2147164153i32);
pub const CONTEXT_E_ROLENOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147164148i32);
pub const CONTEXT_E_SYNCH_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2147164154i32);
pub const CONTEXT_E_TMNOTAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2147164145i32);
pub const CONTEXT_E_WOULD_DEADLOCK: crate::core::HRESULT = crate::core::HRESULT(-2147164155i32);
pub const CONTEXT_S_FIRST: i32 = 319488i32;
pub const CONTEXT_S_LAST: i32 = 319535i32;
pub const CONTROL_C_EXIT: NTSTATUS = NTSTATUS(-1073741510i32);
pub const CONVERT10_E_FIRST: i32 = -2147221056i32;
pub const CONVERT10_E_LAST: i32 = -2147221041i32;
pub const CONVERT10_E_OLESTREAM_BITMAP_TO_DIB: crate::core::HRESULT =
    crate::core::HRESULT(-2147221053i32);
pub const CONVERT10_E_OLESTREAM_FMT: crate::core::HRESULT = crate::core::HRESULT(-2147221054i32);
pub const CONVERT10_E_OLESTREAM_GET: crate::core::HRESULT = crate::core::HRESULT(-2147221056i32);
pub const CONVERT10_E_OLESTREAM_PUT: crate::core::HRESULT = crate::core::HRESULT(-2147221055i32);
pub const CONVERT10_E_STG_DIB_TO_BITMAP: crate::core::HRESULT =
    crate::core::HRESULT(-2147221050i32);
pub const CONVERT10_E_STG_FMT: crate::core::HRESULT = crate::core::HRESULT(-2147221052i32);
pub const CONVERT10_E_STG_NO_STD_STREAM: crate::core::HRESULT =
    crate::core::HRESULT(-2147221051i32);
pub const CONVERT10_S_FIRST: i32 = 262592i32;
pub const CONVERT10_S_LAST: i32 = 262607i32;
pub const CONVERT10_S_NO_PRESENTATION: crate::core::HRESULT = crate::core::HRESULT(262592i32);
pub const CO_E_ACCESSCHECKFAILED: crate::core::HRESULT = crate::core::HRESULT(-2147417814i32);
pub const CO_E_ACESINWRONGORDER: crate::core::HRESULT = crate::core::HRESULT(-2147417798i32);
pub const CO_E_ACNOTINITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2147417793i32);
pub const CO_E_ACTIVATIONFAILED: crate::core::HRESULT = crate::core::HRESULT(-2147164127i32);
pub const CO_E_ACTIVATIONFAILED_CATALOGERROR: crate::core::HRESULT =
    crate::core::HRESULT(-2147164125i32);
pub const CO_E_ACTIVATIONFAILED_EVENTLOGGED: crate::core::HRESULT =
    crate::core::HRESULT(-2147164126i32);
pub const CO_E_ACTIVATIONFAILED_TIMEOUT: crate::core::HRESULT =
    crate::core::HRESULT(-2147164124i32);
pub const CO_E_ALREADYINITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2147221007i32);
pub const CO_E_APPDIDNTREG: crate::core::HRESULT = crate::core::HRESULT(-2147220994i32);
pub const CO_E_APPNOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147221003i32);
pub const CO_E_APPSINGLEUSE: crate::core::HRESULT = crate::core::HRESULT(-2147221002i32);
pub const CO_E_ASYNC_WORK_REJECTED: crate::core::HRESULT = crate::core::HRESULT(-2147467223i32);
pub const CO_E_ATTEMPT_TO_CREATE_OUTSIDE_CLIENT_CONTEXT: crate::core::HRESULT =
    crate::core::HRESULT(-2147467228i32);
pub const CO_E_BAD_PATH: crate::core::HRESULT = crate::core::HRESULT(-2146959356i32);
pub const CO_E_BAD_SERVER_NAME: crate::core::HRESULT = crate::core::HRESULT(-2147467244i32);
pub const CO_E_CALL_OUT_OF_TX_SCOPE_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2147164112i32);
pub const CO_E_CANCEL_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2147417792i32);
pub const CO_E_CANTDETERMINECLASS: crate::core::HRESULT = crate::core::HRESULT(-2147221006i32);
pub const CO_E_CANT_REMOTE: crate::core::HRESULT = crate::core::HRESULT(-2147467245i32);
pub const CO_E_CLASSSTRING: crate::core::HRESULT = crate::core::HRESULT(-2147221005i32);
pub const CO_E_CLASS_CREATE_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2146959359i32);
pub const CO_E_CLASS_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2147467225i32);
pub const CO_E_CLRNOTAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2147467224i32);
pub const CO_E_CLSREG_INCONSISTENT: crate::core::HRESULT = crate::core::HRESULT(-2147467233i32);
pub const CO_E_CONVERSIONFAILED: crate::core::HRESULT = crate::core::HRESULT(-2147417810i32);
pub const CO_E_CREATEPROCESS_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2147467240i32);
pub const CO_E_DBERROR: crate::core::HRESULT = crate::core::HRESULT(-2147164117i32);
pub const CO_E_DECODEFAILED: crate::core::HRESULT = crate::core::HRESULT(-2147417795i32);
pub const CO_E_DLLNOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147221000i32);
pub const CO_E_ELEVATION_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2146959337i32);
pub const CO_E_ERRORINAPP: crate::core::HRESULT = crate::core::HRESULT(-2147221001i32);
pub const CO_E_ERRORINDLL: crate::core::HRESULT = crate::core::HRESULT(-2147220999i32);
pub const CO_E_EXCEEDSYSACLLIMIT: crate::core::HRESULT = crate::core::HRESULT(-2147417799i32);
pub const CO_E_EXIT_TRANSACTION_SCOPE_NOT_CALLED: crate::core::HRESULT =
    crate::core::HRESULT(-2147164111i32);
pub const CO_E_FAILEDTOCLOSEHANDLE: crate::core::HRESULT = crate::core::HRESULT(-2147417800i32);
pub const CO_E_FAILEDTOCREATEFILE: crate::core::HRESULT = crate::core::HRESULT(-2147417801i32);
pub const CO_E_FAILEDTOGENUUID: crate::core::HRESULT = crate::core::HRESULT(-2147417802i32);
pub const CO_E_FAILEDTOGETSECCTX: crate::core::HRESULT = crate::core::HRESULT(-2147417820i32);
pub const CO_E_FAILEDTOGETTOKENINFO: crate::core::HRESULT = crate::core::HRESULT(-2147417818i32);
pub const CO_E_FAILEDTOGETWINDIR: crate::core::HRESULT = crate::core::HRESULT(-2147417804i32);
pub const CO_E_FAILEDTOIMPERSONATE: crate::core::HRESULT = crate::core::HRESULT(-2147417821i32);
pub const CO_E_FAILEDTOOPENPROCESSTOKEN: crate::core::HRESULT =
    crate::core::HRESULT(-2147417796i32);
pub const CO_E_FAILEDTOOPENTHREADTOKEN: crate::core::HRESULT = crate::core::HRESULT(-2147417819i32);
pub const CO_E_FAILEDTOQUERYCLIENTBLANKET: crate::core::HRESULT =
    crate::core::HRESULT(-2147417816i32);
pub const CO_E_FAILEDTOSETDACL: crate::core::HRESULT = crate::core::HRESULT(-2147417815i32);
pub const CO_E_FIRST: i32 = -2147221008i32;
pub const CO_E_IIDREG_INCONSISTENT: crate::core::HRESULT = crate::core::HRESULT(-2147467232i32);
pub const CO_E_IIDSTRING: crate::core::HRESULT = crate::core::HRESULT(-2147221004i32);
pub const CO_E_INCOMPATIBLESTREAMVERSION: crate::core::HRESULT =
    crate::core::HRESULT(-2147417797i32);
pub const CO_E_INITIALIZATIONFAILED: crate::core::HRESULT = crate::core::HRESULT(-2147164123i32);
pub const CO_E_INIT_CLASS_CACHE: crate::core::HRESULT = crate::core::HRESULT(-2147467255i32);
pub const CO_E_INIT_MEMORY_ALLOCATOR: crate::core::HRESULT = crate::core::HRESULT(-2147467256i32);
pub const CO_E_INIT_ONLY_SINGLE_THREADED: crate::core::HRESULT =
    crate::core::HRESULT(-2147467246i32);
pub const CO_E_INIT_RPC_CHANNEL: crate::core::HRESULT = crate::core::HRESULT(-2147467254i32);
pub const CO_E_INIT_SCM_EXEC_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2147467247i32);
pub const CO_E_INIT_SCM_FILE_MAPPING_EXISTS: crate::core::HRESULT =
    crate::core::HRESULT(-2147467249i32);
pub const CO_E_INIT_SCM_MAP_VIEW_OF_FILE: crate::core::HRESULT =
    crate::core::HRESULT(-2147467248i32);
pub const CO_E_INIT_SCM_MUTEX_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2147467250i32);
pub const CO_E_INIT_SHARED_ALLOCATOR: crate::core::HRESULT = crate::core::HRESULT(-2147467257i32);
pub const CO_E_INIT_TLS: crate::core::HRESULT = crate::core::HRESULT(-2147467258i32);
pub const CO_E_INIT_TLS_CHANNEL_CONTROL: crate::core::HRESULT =
    crate::core::HRESULT(-2147467252i32);
pub const CO_E_INIT_TLS_SET_CHANNEL_CONTROL: crate::core::HRESULT =
    crate::core::HRESULT(-2147467253i32);
pub const CO_E_INIT_UNACCEPTED_USER_ALLOCATOR: crate::core::HRESULT =
    crate::core::HRESULT(-2147467251i32);
pub const CO_E_INVALIDSID: crate::core::HRESULT = crate::core::HRESULT(-2147417811i32);
pub const CO_E_ISOLEVELMISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2147164113i32);
pub const CO_E_LAST: i32 = -2147220993i32;
pub const CO_E_LAUNCH_PERMSSION_DENIED: crate::core::HRESULT = crate::core::HRESULT(-2147467237i32);
pub const CO_E_LOOKUPACCNAMEFAILED: crate::core::HRESULT = crate::core::HRESULT(-2147417806i32);
pub const CO_E_LOOKUPACCSIDFAILED: crate::core::HRESULT = crate::core::HRESULT(-2147417808i32);
pub const CO_E_MALFORMED_SPN: crate::core::HRESULT = crate::core::HRESULT(-2147467213i32);
pub const CO_E_MISSING_DISPLAYNAME: crate::core::HRESULT = crate::core::HRESULT(-2146959339i32);
pub const CO_E_MSI_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2147467229i32);
pub const CO_E_NETACCESSAPIFAILED: crate::core::HRESULT = crate::core::HRESULT(-2147417813i32);
pub const CO_E_NOCOOKIES: crate::core::HRESULT = crate::core::HRESULT(-2147164118i32);
pub const CO_E_NOIISINTRINSICS: crate::core::HRESULT = crate::core::HRESULT(-2147164119i32);
pub const CO_E_NOMATCHINGNAMEFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147417807i32);
pub const CO_E_NOMATCHINGSIDFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147417809i32);
pub const CO_E_NOSYNCHRONIZATION: crate::core::HRESULT = crate::core::HRESULT(-2147164114i32);
pub const CO_E_NOTCONSTRUCTED: crate::core::HRESULT = crate::core::HRESULT(-2147164115i32);
pub const CO_E_NOTINITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2147221008i32);
pub const CO_E_NOTPOOLED: crate::core::HRESULT = crate::core::HRESULT(-2147164116i32);
pub const CO_E_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2147467231i32);
pub const CO_E_NO_SECCTX_IN_ACTIVATE: crate::core::HRESULT = crate::core::HRESULT(-2147467221i32);
pub const CO_E_OBJISREG: crate::core::HRESULT = crate::core::HRESULT(-2147220996i32);
pub const CO_E_OBJNOTCONNECTED: crate::core::HRESULT = crate::core::HRESULT(-2147220995i32);
pub const CO_E_OBJNOTREG: crate::core::HRESULT = crate::core::HRESULT(-2147220997i32);
pub const CO_E_OBJSRV_RPC_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2146959354i32);
pub const CO_E_OLE1DDE_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2147467242i32);
pub const CO_E_PATHTOOLONG: crate::core::HRESULT = crate::core::HRESULT(-2147417803i32);
pub const CO_E_PREMATURE_STUB_RUNDOWN: crate::core::HRESULT = crate::core::HRESULT(-2147467211i32);
pub const CO_E_RELEASED: crate::core::HRESULT = crate::core::HRESULT(-2147220993i32);
pub const CO_E_RELOAD_DLL: crate::core::HRESULT = crate::core::HRESULT(-2147467230i32);
pub const CO_E_REMOTE_COMMUNICATION_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-2147467235i32);
pub const CO_E_RUNAS_CREATEPROCESS_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-2147467239i32);
pub const CO_E_RUNAS_LOGON_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2147467238i32);
pub const CO_E_RUNAS_SYNTAX: crate::core::HRESULT = crate::core::HRESULT(-2147467241i32);
pub const CO_E_RUNAS_VALUE_MUST_BE_AAA: crate::core::HRESULT = crate::core::HRESULT(-2146959338i32);
pub const CO_E_SCM_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146959358i32);
pub const CO_E_SCM_RPC_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2146959357i32);
pub const CO_E_SERVER_EXEC_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2146959355i32);
pub const CO_E_SERVER_INIT_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2147467222i32);
pub const CO_E_SERVER_NOT_PAUSED: crate::core::HRESULT = crate::core::HRESULT(-2147467226i32);
pub const CO_E_SERVER_PAUSED: crate::core::HRESULT = crate::core::HRESULT(-2147467227i32);
pub const CO_E_SERVER_START_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2147467234i32);
pub const CO_E_SERVER_STOPPING: crate::core::HRESULT = crate::core::HRESULT(-2146959352i32);
pub const CO_E_SETSERLHNDLFAILED: crate::core::HRESULT = crate::core::HRESULT(-2147417805i32);
pub const CO_E_START_SERVICE_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2147467236i32);
pub const CO_E_SXS_CONFIG: crate::core::HRESULT = crate::core::HRESULT(-2147467214i32);
pub const CO_E_THREADINGMODEL_CHANGED: crate::core::HRESULT = crate::core::HRESULT(-2147164120i32);
pub const CO_E_THREADPOOL_CONFIG: crate::core::HRESULT = crate::core::HRESULT(-2147467215i32);
pub const CO_E_TRACKER_CONFIG: crate::core::HRESULT = crate::core::HRESULT(-2147467216i32);
pub const CO_E_TRUSTEEDOESNTMATCHCLIENT: crate::core::HRESULT =
    crate::core::HRESULT(-2147417817i32);
pub const CO_E_UNREVOKED_REGISTRATION_ON_APARTMENT_SHUTDOWN: crate::core::HRESULT =
    crate::core::HRESULT(-2147467212i32);
pub const CO_E_WRONGOSFORAPP: crate::core::HRESULT = crate::core::HRESULT(-2147220998i32);
pub const CO_E_WRONGTRUSTEENAMESYNTAX: crate::core::HRESULT = crate::core::HRESULT(-2147417812i32);
pub const CO_E_WRONG_SERVER_IDENTITY: crate::core::HRESULT = crate::core::HRESULT(-2147467243i32);
pub const CO_S_FIRST: i32 = 262640i32;
pub const CO_S_LAST: i32 = 262655i32;
pub const CO_S_MACHINENAMENOTFOUND: crate::core::HRESULT = crate::core::HRESULT(524307i32);
pub const CO_S_NOTALLINTERFACES: crate::core::HRESULT = crate::core::HRESULT(524306i32);
pub const CRYPT_E_ALREADY_DECRYPTED: crate::core::HRESULT = crate::core::HRESULT(-2146889719i32);
pub const CRYPT_E_ASN1_BADARGS: crate::core::HRESULT = crate::core::HRESULT(-2146881271i32);
pub const CRYPT_E_ASN1_BADPDU: crate::core::HRESULT = crate::core::HRESULT(-2146881272i32);
pub const CRYPT_E_ASN1_BADREAL: crate::core::HRESULT = crate::core::HRESULT(-2146881270i32);
pub const CRYPT_E_ASN1_BADTAG: crate::core::HRESULT = crate::core::HRESULT(-2146881269i32);
pub const CRYPT_E_ASN1_CHOICE: crate::core::HRESULT = crate::core::HRESULT(-2146881268i32);
pub const CRYPT_E_ASN1_CONSTRAINT: crate::core::HRESULT = crate::core::HRESULT(-2146881275i32);
pub const CRYPT_E_ASN1_CORRUPT: crate::core::HRESULT = crate::core::HRESULT(-2146881277i32);
pub const CRYPT_E_ASN1_EOD: crate::core::HRESULT = crate::core::HRESULT(-2146881278i32);
pub const CRYPT_E_ASN1_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146881280i32);
pub const CRYPT_E_ASN1_EXTENDED: crate::core::HRESULT = crate::core::HRESULT(-2146881023i32);
pub const CRYPT_E_ASN1_INTERNAL: crate::core::HRESULT = crate::core::HRESULT(-2146881279i32);
pub const CRYPT_E_ASN1_LARGE: crate::core::HRESULT = crate::core::HRESULT(-2146881276i32);
pub const CRYPT_E_ASN1_MEMORY: crate::core::HRESULT = crate::core::HRESULT(-2146881274i32);
pub const CRYPT_E_ASN1_NOEOD: crate::core::HRESULT = crate::core::HRESULT(-2146881022i32);
pub const CRYPT_E_ASN1_NYI: crate::core::HRESULT = crate::core::HRESULT(-2146881228i32);
pub const CRYPT_E_ASN1_OVERFLOW: crate::core::HRESULT = crate::core::HRESULT(-2146881273i32);
pub const CRYPT_E_ASN1_PDU_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2146881229i32);
pub const CRYPT_E_ASN1_RULE: crate::core::HRESULT = crate::core::HRESULT(-2146881267i32);
pub const CRYPT_E_ASN1_UTF8: crate::core::HRESULT = crate::core::HRESULT(-2146881266i32);
pub const CRYPT_E_ATTRIBUTES_MISSING: crate::core::HRESULT = crate::core::HRESULT(-2146889713i32);
pub const CRYPT_E_AUTH_ATTR_MISSING: crate::core::HRESULT = crate::core::HRESULT(-2146889722i32);
pub const CRYPT_E_BAD_ENCODE: crate::core::HRESULT = crate::core::HRESULT(-2146885630i32);
pub const CRYPT_E_BAD_LEN: crate::core::HRESULT = crate::core::HRESULT(-2146885631i32);
pub const CRYPT_E_BAD_MSG: crate::core::HRESULT = crate::core::HRESULT(-2146885619i32);
pub const CRYPT_E_CONTROL_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2146889716i32);
pub const CRYPT_E_DELETED_PREV: crate::core::HRESULT = crate::core::HRESULT(-2146885624i32);
pub const CRYPT_E_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2146885627i32);
pub const CRYPT_E_FILERESIZED: crate::core::HRESULT = crate::core::HRESULT(-2146885595i32);
pub const CRYPT_E_FILE_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146885629i32);
pub const CRYPT_E_HASH_VALUE: crate::core::HRESULT = crate::core::HRESULT(-2146889721i32);
pub const CRYPT_E_INVALID_IA5_STRING: crate::core::HRESULT = crate::core::HRESULT(-2146885598i32);
pub const CRYPT_E_INVALID_INDEX: crate::core::HRESULT = crate::core::HRESULT(-2146889720i32);
pub const CRYPT_E_INVALID_MSG_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2146889724i32);
pub const CRYPT_E_INVALID_NUMERIC_STRING: crate::core::HRESULT =
    crate::core::HRESULT(-2146885600i32);
pub const CRYPT_E_INVALID_PRINTABLE_STRING: crate::core::HRESULT =
    crate::core::HRESULT(-2146885599i32);
pub const CRYPT_E_INVALID_X500_STRING: crate::core::HRESULT = crate::core::HRESULT(-2146885597i32);
pub const CRYPT_E_ISSUER_SERIALNUMBER: crate::core::HRESULT = crate::core::HRESULT(-2146889715i32);
pub const CRYPT_E_MISSING_PUBKEY_PARA: crate::core::HRESULT = crate::core::HRESULT(-2146885588i32);
pub const CRYPT_E_MSG_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146889727i32);
pub const CRYPT_E_NOT_CHAR_STRING: crate::core::HRESULT = crate::core::HRESULT(-2146885596i32);
pub const CRYPT_E_NOT_DECRYPTED: crate::core::HRESULT = crate::core::HRESULT(-2146889718i32);
pub const CRYPT_E_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2146885628i32);
pub const CRYPT_E_NOT_IN_CTL: crate::core::HRESULT = crate::core::HRESULT(-2146885590i32);
pub const CRYPT_E_NOT_IN_REVOCATION_DATABASE: crate::core::HRESULT =
    crate::core::HRESULT(-2146885612i32);
pub const CRYPT_E_NO_DECRYPT_CERT: crate::core::HRESULT = crate::core::HRESULT(-2146885620i32);
pub const CRYPT_E_NO_KEY_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-2146885621i32);
pub const CRYPT_E_NO_MATCH: crate::core::HRESULT = crate::core::HRESULT(-2146885623i32);
pub const CRYPT_E_NO_PROVIDER: crate::core::HRESULT = crate::core::HRESULT(-2146885626i32);
pub const CRYPT_E_NO_REVOCATION_CHECK: crate::core::HRESULT = crate::core::HRESULT(-2146885614i32);
pub const CRYPT_E_NO_REVOCATION_DLL: crate::core::HRESULT = crate::core::HRESULT(-2146885615i32);
pub const CRYPT_E_NO_SIGNER: crate::core::HRESULT = crate::core::HRESULT(-2146885618i32);
pub const CRYPT_E_NO_TRUSTED_SIGNER: crate::core::HRESULT = crate::core::HRESULT(-2146885589i32);
pub const CRYPT_E_NO_VERIFY_USAGE_CHECK: crate::core::HRESULT =
    crate::core::HRESULT(-2146885592i32);
pub const CRYPT_E_NO_VERIFY_USAGE_DLL: crate::core::HRESULT = crate::core::HRESULT(-2146885593i32);
pub const CRYPT_E_OBJECT_LOCATOR_OBJECT_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2146885587i32);
pub const CRYPT_E_OID_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-2146889725i32);
pub const CRYPT_E_OSS_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146881536i32);
pub const CRYPT_E_PENDING_CLOSE: crate::core::HRESULT = crate::core::HRESULT(-2146885617i32);
pub const CRYPT_E_RECIPIENT_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2146889717i32);
pub const CRYPT_E_REVOCATION_OFFLINE: crate::core::HRESULT = crate::core::HRESULT(-2146885613i32);
pub const CRYPT_E_REVOKED: crate::core::HRESULT = crate::core::HRESULT(-2146885616i32);
pub const CRYPT_E_SECURITY_SETTINGS: crate::core::HRESULT = crate::core::HRESULT(-2146885594i32);
pub const CRYPT_E_SELF_SIGNED: crate::core::HRESULT = crate::core::HRESULT(-2146885625i32);
pub const CRYPT_E_SIGNER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2146889714i32);
pub const CRYPT_E_STREAM_INSUFFICIENT_DATA: crate::core::HRESULT =
    crate::core::HRESULT(-2146889711i32);
pub const CRYPT_E_STREAM_MSG_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-2146889712i32);
pub const CRYPT_E_UNEXPECTED_ENCODING: crate::core::HRESULT = crate::core::HRESULT(-2146889723i32);
pub const CRYPT_E_UNEXPECTED_MSG_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2146885622i32);
pub const CRYPT_E_UNKNOWN_ALGO: crate::core::HRESULT = crate::core::HRESULT(-2146889726i32);
pub const CRYPT_E_VERIFY_USAGE_OFFLINE: crate::core::HRESULT = crate::core::HRESULT(-2146885591i32);
pub const CRYPT_I_NEW_PROTECTION_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(593938i32);
pub const CS_E_ADMIN_LIMIT_EXCEEDED: crate::core::HRESULT = crate::core::HRESULT(-2147221139i32);
pub const CS_E_CLASS_NOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147221146i32);
pub const CS_E_FIRST: i32 = -2147221148i32;
pub const CS_E_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2147221137i32);
pub const CS_E_INVALID_PATH: crate::core::HRESULT = crate::core::HRESULT(-2147221141i32);
pub const CS_E_INVALID_VERSION: crate::core::HRESULT = crate::core::HRESULT(-2147221145i32);
pub const CS_E_LAST: i32 = -2147221137i32;
pub const CS_E_NETWORK_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2147221140i32);
pub const CS_E_NOT_DELETABLE: crate::core::HRESULT = crate::core::HRESULT(-2147221147i32);
pub const CS_E_NO_CLASSSTORE: crate::core::HRESULT = crate::core::HRESULT(-2147221144i32);
pub const CS_E_OBJECT_ALREADY_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2147221142i32);
pub const CS_E_OBJECT_NOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147221143i32);
pub const CS_E_PACKAGE_NOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147221148i32);
pub const CS_E_SCHEMA_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2147221138i32);
pub const D2DERR_BAD_NUMBER: crate::core::HRESULT = crate::core::HRESULT(-2003238895i32);
pub const D2DERR_BITMAP_BOUND_AS_TARGET: crate::core::HRESULT =
    crate::core::HRESULT(-2003238875i32);
pub const D2DERR_BITMAP_CANNOT_DRAW: crate::core::HRESULT = crate::core::HRESULT(-2003238879i32);
pub const D2DERR_CYCLIC_GRAPH: crate::core::HRESULT = crate::core::HRESULT(-2003238880i32);
pub const D2DERR_DISPLAY_FORMAT_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2003238903i32);
pub const D2DERR_DISPLAY_STATE_INVALID: crate::core::HRESULT = crate::core::HRESULT(-2003238906i32);
pub const D2DERR_EFFECT_IS_NOT_REGISTERED: crate::core::HRESULT =
    crate::core::HRESULT(-2003238872i32);
pub const D2DERR_EXCEEDS_MAX_BITMAP_SIZE: crate::core::HRESULT =
    crate::core::HRESULT(-2003238883i32);
pub const D2DERR_INCOMPATIBLE_BRUSH_TYPES: crate::core::HRESULT =
    crate::core::HRESULT(-2003238888i32);
pub const D2DERR_INSUFFICIENT_DEVICE_CAPABILITIES: crate::core::HRESULT =
    crate::core::HRESULT(-2003238874i32);
pub const D2DERR_INTERMEDIATE_TOO_LARGE: crate::core::HRESULT =
    crate::core::HRESULT(-2003238873i32);
pub const D2DERR_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2003238904i32);
pub const D2DERR_INVALID_CALL: crate::core::HRESULT = crate::core::HRESULT(-2003238902i32);
pub const D2DERR_INVALID_GLYPH_IMAGE: crate::core::HRESULT = crate::core::HRESULT(-2003238866i32);
pub const D2DERR_INVALID_GRAPH_CONFIGURATION: crate::core::HRESULT =
    crate::core::HRESULT(-2003238882i32);
pub const D2DERR_INVALID_INTERNAL_GRAPH_CONFIGURATION: crate::core::HRESULT =
    crate::core::HRESULT(-2003238881i32);
pub const D2DERR_INVALID_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-2003238871i32);
pub const D2DERR_INVALID_TARGET: crate::core::HRESULT = crate::core::HRESULT(-2003238876i32);
pub const D2DERR_LAYER_ALREADY_IN_USE: crate::core::HRESULT = crate::core::HRESULT(-2003238893i32);
pub const D2DERR_MAX_TEXTURE_SIZE_EXCEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-2003238897i32);
pub const D2DERR_NOT_INITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2003238910i32);
pub const D2DERR_NO_HARDWARE_DEVICE: crate::core::HRESULT = crate::core::HRESULT(-2003238901i32);
pub const D2DERR_NO_SUBPROPERTIES: crate::core::HRESULT = crate::core::HRESULT(-2003238870i32);
pub const D2DERR_ORIGINAL_TARGET_NOT_BOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2003238877i32);
pub const D2DERR_OUTSTANDING_BITMAP_REFERENCES: crate::core::HRESULT =
    crate::core::HRESULT(-2003238878i32);
pub const D2DERR_POP_CALL_DID_NOT_MATCH_PUSH: crate::core::HRESULT =
    crate::core::HRESULT(-2003238892i32);
pub const D2DERR_PRINT_FORMAT_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2003238868i32);
pub const D2DERR_PRINT_JOB_CLOSED: crate::core::HRESULT = crate::core::HRESULT(-2003238869i32);
pub const D2DERR_PUSH_POP_UNBALANCED: crate::core::HRESULT = crate::core::HRESULT(-2003238890i32);
pub const D2DERR_RECREATE_TARGET: crate::core::HRESULT = crate::core::HRESULT(-2003238900i32);
pub const D2DERR_RENDER_TARGET_HAS_LAYER_OR_CLIPRECT: crate::core::HRESULT =
    crate::core::HRESULT(-2003238889i32);
pub const D2DERR_SCANNER_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2003238908i32);
pub const D2DERR_SCREEN_ACCESS_DENIED: crate::core::HRESULT = crate::core::HRESULT(-2003238907i32);
pub const D2DERR_SHADER_COMPILE_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2003238898i32);
pub const D2DERR_TARGET_NOT_GDI_COMPATIBLE: crate::core::HRESULT =
    crate::core::HRESULT(-2003238886i32);
pub const D2DERR_TEXT_EFFECT_IS_WRONG_TYPE: crate::core::HRESULT =
    crate::core::HRESULT(-2003238885i32);
pub const D2DERR_TEXT_RENDERER_NOT_RELEASED: crate::core::HRESULT =
    crate::core::HRESULT(-2003238884i32);
pub const D2DERR_TOO_MANY_SHADER_ELEMENTS: crate::core::HRESULT =
    crate::core::HRESULT(-2003238899i32);
pub const D2DERR_TOO_MANY_TRANSFORM_INPUTS: crate::core::HRESULT =
    crate::core::HRESULT(-2003238867i32);
pub const D2DERR_UNSUPPORTED_OPERATION: crate::core::HRESULT = crate::core::HRESULT(-2003238909i32);
pub const D2DERR_UNSUPPORTED_VERSION: crate::core::HRESULT = crate::core::HRESULT(-2003238896i32);
pub const D2DERR_WIN32_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2003238887i32);
pub const D2DERR_WRONG_FACTORY: crate::core::HRESULT = crate::core::HRESULT(-2003238894i32);
pub const D2DERR_WRONG_RESOURCE_DOMAIN: crate::core::HRESULT = crate::core::HRESULT(-2003238891i32);
pub const D2DERR_WRONG_STATE: crate::core::HRESULT = crate::core::HRESULT(-2003238911i32);
pub const D2DERR_ZERO_VECTOR: crate::core::HRESULT = crate::core::HRESULT(-2003238905i32);
pub const D3D10_ERROR_FILE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2005336062i32);
pub const D3D10_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS: crate::core::HRESULT =
    crate::core::HRESULT(-2005336063i32);
pub const D3D11_ERROR_DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD: crate::core::HRESULT =
    crate::core::HRESULT(-2005139452i32);
pub const D3D11_ERROR_FILE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2005139454i32);
pub const D3D11_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS: crate::core::HRESULT =
    crate::core::HRESULT(-2005139455i32);
pub const D3D11_ERROR_TOO_MANY_UNIQUE_VIEW_OBJECTS: crate::core::HRESULT =
    crate::core::HRESULT(-2005139453i32);
pub const D3D12_ERROR_ADAPTER_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2005008383i32);
pub const D3D12_ERROR_DRIVER_VERSION_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2005008382i32);
pub const D3D12_ERROR_INVALID_REDIST: crate::core::HRESULT = crate::core::HRESULT(-2005008381i32);
pub const DATA_E_FIRST: i32 = -2147221200i32;
pub const DATA_E_LAST: i32 = -2147221185i32;
pub const DATA_S_FIRST: i32 = 262448i32;
pub const DATA_S_LAST: i32 = 262463i32;
pub const DATA_S_SAMEFORMATETC: crate::core::HRESULT = crate::core::HRESULT(262448i32);
pub const DBG_APP_NOT_IDLE: NTSTATUS = NTSTATUS(-1073676286i32);
pub const DBG_COMMAND_EXCEPTION: NTSTATUS = NTSTATUS(1073807369i32);
pub const DBG_CONTINUE: NTSTATUS = NTSTATUS(65538i32);
pub const DBG_CONTROL_BREAK: NTSTATUS = NTSTATUS(1073807368i32);
pub const DBG_CONTROL_C: NTSTATUS = NTSTATUS(1073807365i32);
pub const DBG_EXCEPTION_HANDLED: NTSTATUS = NTSTATUS(65537i32);
pub const DBG_EXCEPTION_NOT_HANDLED: NTSTATUS = NTSTATUS(-2147418111i32);
pub const DBG_NO_STATE_CHANGE: NTSTATUS = NTSTATUS(-1073676287i32);
pub const DBG_PRINTEXCEPTION_C: NTSTATUS = NTSTATUS(1073807366i32);
pub const DBG_PRINTEXCEPTION_WIDE_C: NTSTATUS = NTSTATUS(1073807370i32);
pub const DBG_REPLY_LATER: NTSTATUS = NTSTATUS(1073807361i32);
pub const DBG_RIPEXCEPTION: NTSTATUS = NTSTATUS(1073807367i32);
pub const DBG_TERMINATE_PROCESS: NTSTATUS = NTSTATUS(1073807364i32);
pub const DBG_TERMINATE_THREAD: NTSTATUS = NTSTATUS(1073807363i32);
pub const DBG_UNABLE_TO_PROVIDE_HANDLE: NTSTATUS = NTSTATUS(1073807362i32);
pub const DCOMPOSITION_ERROR_SURFACE_BEING_RENDERED: crate::core::HRESULT =
    crate::core::HRESULT(-2003302399i32);
pub const DCOMPOSITION_ERROR_SURFACE_NOT_BEING_RENDERED: crate::core::HRESULT =
    crate::core::HRESULT(-2003302398i32);
pub const DCOMPOSITION_ERROR_WINDOW_ALREADY_COMPOSED: crate::core::HRESULT =
    crate::core::HRESULT(-2003302400i32);
pub struct DECIMAL {
    pub wReserved: u16,
    pub Anonymous1: DECIMAL_0,
    pub Hi32: u32,
    pub Anonymous2: DECIMAL_1,
}
impl ::core::marker::Copy for DECIMAL {}
impl ::core::clone::Clone for DECIMAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for DECIMAL {
    fn eq(&self, other: &Self) -> bool {
        self.wReserved == other.wReserved
            && self.Anonymous1 == other.Anonymous1
            && self.Hi32 == other.Hi32
            && self.Anonymous2 == other.Anonymous2
    }
}
impl ::core::cmp::Eq for DECIMAL {}
impl FromIntoMemory for DECIMAL {
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
pub struct DECIMAL_0 {
    pub Anonymous: DECIMAL_0_0,
    pub signscale: u16,
}
impl ::core::marker::Copy for DECIMAL_0 {}
impl ::core::clone::Clone for DECIMAL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for DECIMAL_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous && self.signscale == other.signscale
    }
}
impl ::core::cmp::Eq for DECIMAL_0 {}
impl FromIntoMemory for DECIMAL_0 {
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
pub struct DECIMAL_0_0 {
    pub scale: u8,
    pub sign: u8,
}
impl ::core::marker::Copy for DECIMAL_0_0 {}
impl ::core::clone::Clone for DECIMAL_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DECIMAL_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DECIMAL_0_0")
            .field("scale", &self.scale)
            .field("sign", &self.sign)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DECIMAL_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.scale == other.scale && self.sign == other.sign
    }
}
impl ::core::cmp::Eq for DECIMAL_0_0 {}
impl FromIntoMemory for DECIMAL_0_0 {
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
pub struct DECIMAL_1 {
    pub Anonymous: DECIMAL_1_0,
    pub Lo64: u64,
}
impl ::core::marker::Copy for DECIMAL_1 {}
impl ::core::clone::Clone for DECIMAL_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::cmp::PartialEq for DECIMAL_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous && self.Lo64 == other.Lo64
    }
}
impl ::core::cmp::Eq for DECIMAL_1 {}
impl FromIntoMemory for DECIMAL_1 {
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
pub struct DECIMAL_1_0 {
    pub Lo32: u32,
    pub Mid32: u32,
}
impl ::core::marker::Copy for DECIMAL_1_0 {}
impl ::core::clone::Clone for DECIMAL_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DECIMAL_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DECIMAL_1_0")
            .field("Lo32", &self.Lo32)
            .field("Mid32", &self.Mid32)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DECIMAL_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Lo32 == other.Lo32 && self.Mid32 == other.Mid32
    }
}
impl ::core::cmp::Eq for DECIMAL_1_0 {}
impl FromIntoMemory for DECIMAL_1_0 {
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
pub const DIGSIG_E_CRYPTO: crate::core::HRESULT = crate::core::HRESULT(-2146762744i32);
pub const DIGSIG_E_DECODE: crate::core::HRESULT = crate::core::HRESULT(-2146762746i32);
pub const DIGSIG_E_ENCODE: crate::core::HRESULT = crate::core::HRESULT(-2146762747i32);
pub const DIGSIG_E_EXTENSIBILITY: crate::core::HRESULT = crate::core::HRESULT(-2146762745i32);
pub const DISP_E_ARRAYISLOCKED: crate::core::HRESULT = crate::core::HRESULT(-2147352563i32);
pub const DISP_E_BADCALLEE: crate::core::HRESULT = crate::core::HRESULT(-2147352560i32);
pub const DISP_E_BADINDEX: crate::core::HRESULT = crate::core::HRESULT(-2147352565i32);
pub const DISP_E_BADPARAMCOUNT: crate::core::HRESULT = crate::core::HRESULT(-2147352562i32);
pub const DISP_E_BADVARTYPE: crate::core::HRESULT = crate::core::HRESULT(-2147352568i32);
pub const DISP_E_BUFFERTOOSMALL: crate::core::HRESULT = crate::core::HRESULT(-2147352557i32);
pub const DISP_E_DIVBYZERO: crate::core::HRESULT = crate::core::HRESULT(-2147352558i32);
pub const DISP_E_EXCEPTION: crate::core::HRESULT = crate::core::HRESULT(-2147352567i32);
pub const DISP_E_MEMBERNOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147352573i32);
pub const DISP_E_NONAMEDARGS: crate::core::HRESULT = crate::core::HRESULT(-2147352569i32);
pub const DISP_E_NOTACOLLECTION: crate::core::HRESULT = crate::core::HRESULT(-2147352559i32);
pub const DISP_E_OVERFLOW: crate::core::HRESULT = crate::core::HRESULT(-2147352566i32);
pub const DISP_E_PARAMNOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147352572i32);
pub const DISP_E_PARAMNOTOPTIONAL: crate::core::HRESULT = crate::core::HRESULT(-2147352561i32);
pub const DISP_E_TYPEMISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2147352571i32);
pub const DISP_E_UNKNOWNINTERFACE: crate::core::HRESULT = crate::core::HRESULT(-2147352575i32);
pub const DISP_E_UNKNOWNLCID: crate::core::HRESULT = crate::core::HRESULT(-2147352564i32);
pub const DISP_E_UNKNOWNNAME: crate::core::HRESULT = crate::core::HRESULT(-2147352570i32);
pub const DM_COPY: u32 = 2u32;
pub const DM_IN_BUFFER: u32 = 8u32;
pub const DM_IN_PROMPT: u32 = 4u32;
pub const DM_MODIFY: u32 = 8u32;
pub const DM_OUT_BUFFER: u32 = 2u32;
pub const DM_OUT_DEFAULT: u32 = 1u32;
pub const DM_PROMPT: u32 = 4u32;
pub const DM_UPDATE: u32 = 1u32;
pub const DNS_INFO_ADDED_LOCAL_WINS: i32 = 9753i32;
pub const DNS_INFO_AXFR_COMPLETE: i32 = 9751i32;
pub const DNS_INFO_NO_RECORDS: i32 = 9501i32;
pub const DNS_REQUEST_PENDING: i32 = 9506i32;
pub const DNS_STATUS_CONTINUE_NEEDED: i32 = 9801i32;
pub const DNS_STATUS_DOTTED_NAME: i32 = 9558i32;
pub const DNS_STATUS_FQDN: i32 = 9557i32;
pub const DNS_STATUS_SINGLE_PART_NAME: i32 = 9559i32;
pub const DNS_WARNING_DOMAIN_UNDELETED: i32 = 9716i32;
pub const DNS_WARNING_PTR_CREATE_FAILED: i32 = 9715i32;
pub const DRAGDROP_E_ALREADYREGISTERED: crate::core::HRESULT = crate::core::HRESULT(-2147221247i32);
pub const DRAGDROP_E_CONCURRENT_DRAG_ATTEMPTED: crate::core::HRESULT =
    crate::core::HRESULT(-2147221245i32);
pub const DRAGDROP_E_FIRST: i32 = -2147221248i32;
pub const DRAGDROP_E_INVALIDHWND: crate::core::HRESULT = crate::core::HRESULT(-2147221246i32);
pub const DRAGDROP_E_LAST: i32 = -2147221233i32;
pub const DRAGDROP_E_NOTREGISTERED: crate::core::HRESULT = crate::core::HRESULT(-2147221248i32);
pub const DRAGDROP_S_CANCEL: crate::core::HRESULT = crate::core::HRESULT(262401i32);
pub const DRAGDROP_S_DROP: crate::core::HRESULT = crate::core::HRESULT(262400i32);
pub const DRAGDROP_S_FIRST: i32 = 262400i32;
pub const DRAGDROP_S_LAST: i32 = 262415i32;
pub const DRAGDROP_S_USEDEFAULTCURSORS: crate::core::HRESULT = crate::core::HRESULT(262402i32);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DUPLICATE_HANDLE_OPTIONS(pub u32);
pub const DUPLICATE_CLOSE_SOURCE: DUPLICATE_HANDLE_OPTIONS = DUPLICATE_HANDLE_OPTIONS(1u32);
pub const DUPLICATE_SAME_ACCESS: DUPLICATE_HANDLE_OPTIONS = DUPLICATE_HANDLE_OPTIONS(2u32);
impl ::core::marker::Copy for DUPLICATE_HANDLE_OPTIONS {}
impl ::core::clone::Clone for DUPLICATE_HANDLE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DUPLICATE_HANDLE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DUPLICATE_HANDLE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DUPLICATE_HANDLE_OPTIONS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for DUPLICATE_HANDLE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DUPLICATE_HANDLE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DUPLICATE_HANDLE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DUPLICATE_HANDLE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DUPLICATE_HANDLE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for DUPLICATE_HANDLE_OPTIONS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<u32>()
    }
}
pub const DV_E_CLIPFORMAT: crate::core::HRESULT = crate::core::HRESULT(-2147221398i32);
pub const DV_E_DVASPECT: crate::core::HRESULT = crate::core::HRESULT(-2147221397i32);
pub const DV_E_DVTARGETDEVICE: crate::core::HRESULT = crate::core::HRESULT(-2147221403i32);
pub const DV_E_DVTARGETDEVICE_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2147221396i32);
pub const DV_E_FORMATETC: crate::core::HRESULT = crate::core::HRESULT(-2147221404i32);
pub const DV_E_LINDEX: crate::core::HRESULT = crate::core::HRESULT(-2147221400i32);
pub const DV_E_NOIVIEWOBJECT: crate::core::HRESULT = crate::core::HRESULT(-2147221395i32);
pub const DV_E_STATDATA: crate::core::HRESULT = crate::core::HRESULT(-2147221401i32);
pub const DV_E_STGMEDIUM: crate::core::HRESULT = crate::core::HRESULT(-2147221402i32);
pub const DV_E_TYMED: crate::core::HRESULT = crate::core::HRESULT(-2147221399i32);
pub const DWMERR_CATASTROPHIC_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2003302654i32);
pub const DWMERR_STATE_TRANSITION_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2003302656i32);
pub const DWMERR_THEME_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2003302655i32);
pub const DWM_E_ADAPTER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144980987i32);
pub const DWM_E_COMPOSITIONDISABLED: crate::core::HRESULT = crate::core::HRESULT(-2144980991i32);
pub const DWM_E_NOT_QUEUING_PRESENTS: crate::core::HRESULT = crate::core::HRESULT(-2144980988i32);
pub const DWM_E_NO_REDIRECTION_SURFACE_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2144980989i32);
pub const DWM_E_REMOTING_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2144980990i32);
pub const DWM_E_TEXTURE_TOO_LARGE: crate::core::HRESULT = crate::core::HRESULT(-2144980985i32);
pub const DWM_S_GDI_REDIRECTION_SURFACE: crate::core::HRESULT = crate::core::HRESULT(2502661i32);
pub const DWM_S_GDI_REDIRECTION_SURFACE_BLT_VIA_GDI: crate::core::HRESULT =
    crate::core::HRESULT(2502664i32);
pub const DWRITE_E_ALREADYREGISTERED: crate::core::HRESULT = crate::core::HRESULT(-2003283962i32);
pub const DWRITE_E_CACHEFORMAT: crate::core::HRESULT = crate::core::HRESULT(-2003283961i32);
pub const DWRITE_E_CACHEVERSION: crate::core::HRESULT = crate::core::HRESULT(-2003283960i32);
pub const DWRITE_E_FILEACCESS: crate::core::HRESULT = crate::core::HRESULT(-2003283964i32);
pub const DWRITE_E_FILEFORMAT: crate::core::HRESULT = crate::core::HRESULT(-2003283968i32);
pub const DWRITE_E_FILENOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2003283965i32);
pub const DWRITE_E_FLOWDIRECTIONCONFLICTS: crate::core::HRESULT =
    crate::core::HRESULT(-2003283957i32);
pub const DWRITE_E_FONTCOLLECTIONOBSOLETE: crate::core::HRESULT =
    crate::core::HRESULT(-2003283963i32);
pub const DWRITE_E_NOCOLOR: crate::core::HRESULT = crate::core::HRESULT(-2003283956i32);
pub const DWRITE_E_NOFONT: crate::core::HRESULT = crate::core::HRESULT(-2003283966i32);
pub const DWRITE_E_TEXTRENDERERINCOMPATIBLE: crate::core::HRESULT =
    crate::core::HRESULT(-2003283958i32);
pub const DWRITE_E_UNEXPECTED: crate::core::HRESULT = crate::core::HRESULT(-2003283967i32);
pub const DWRITE_E_UNSUPPORTEDOPERATION: crate::core::HRESULT =
    crate::core::HRESULT(-2003283959i32);
pub const DXCORE_ERROR_EVENT_NOT_UNREGISTERED: crate::core::HRESULT =
    crate::core::HRESULT(-2004877311i32);
pub const DXGI_DDI_ERR_NONEXCLUSIVE: crate::core::HRESULT = crate::core::HRESULT(-2005204989i32);
pub const DXGI_DDI_ERR_UNSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2005204990i32);
pub const DXGI_DDI_ERR_WASSTILLDRAWING: crate::core::HRESULT = crate::core::HRESULT(-2005204991i32);
pub const DXGI_STATUS_CLIPPED: crate::core::HRESULT = crate::core::HRESULT(142213122i32);
pub const DXGI_STATUS_DDA_WAS_STILL_DRAWING: crate::core::HRESULT =
    crate::core::HRESULT(142213130i32);
pub const DXGI_STATUS_GRAPHICS_VIDPN_SOURCE_IN_USE: crate::core::HRESULT =
    crate::core::HRESULT(142213126i32);
pub const DXGI_STATUS_MODE_CHANGED: crate::core::HRESULT = crate::core::HRESULT(142213127i32);
pub const DXGI_STATUS_MODE_CHANGE_IN_PROGRESS: crate::core::HRESULT =
    crate::core::HRESULT(142213128i32);
pub const DXGI_STATUS_NO_DESKTOP_ACCESS: crate::core::HRESULT = crate::core::HRESULT(142213125i32);
pub const DXGI_STATUS_NO_REDIRECTION: crate::core::HRESULT = crate::core::HRESULT(142213124i32);
pub const DXGI_STATUS_OCCLUDED: crate::core::HRESULT = crate::core::HRESULT(142213121i32);
pub const DXGI_STATUS_PRESENT_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(142213167i32);
pub const DXGI_STATUS_UNOCCLUDED: crate::core::HRESULT = crate::core::HRESULT(142213129i32);
pub const EAS_E_ADMINS_CANNOT_CHANGE_PASSWORD: crate::core::HRESULT =
    crate::core::HRESULT(-2141913080i32);
pub const EAS_E_ADMINS_HAVE_BLANK_PASSWORD: crate::core::HRESULT =
    crate::core::HRESULT(-2141913081i32);
pub const EAS_E_CONNECTED_ADMINS_NEED_TO_CHANGE_PASSWORD: crate::core::HRESULT =
    crate::core::HRESULT(-2141913077i32);
pub const EAS_E_CURRENT_CONNECTED_USER_NEED_TO_CHANGE_PASSWORD: crate::core::HRESULT =
    crate::core::HRESULT(-2141913075i32);
pub const EAS_E_CURRENT_USER_HAS_BLANK_PASSWORD: crate::core::HRESULT =
    crate::core::HRESULT(-2141913084i32);
pub const EAS_E_LOCAL_CONTROLLED_USERS_CANNOT_CHANGE_PASSWORD: crate::core::HRESULT =
    crate::core::HRESULT(-2141913079i32);
pub const EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CONNECTED_ADMINS: crate::core::HRESULT =
    crate::core::HRESULT(-2141913078i32);
pub const EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CURRENT_CONNECTED_USER: crate::core::HRESULT =
    crate::core::HRESULT(-2141913076i32);
pub const EAS_E_POLICY_COMPLIANT_WITH_ACTIONS: crate::core::HRESULT =
    crate::core::HRESULT(-2141913086i32);
pub const EAS_E_POLICY_NOT_MANAGED_BY_OS: crate::core::HRESULT =
    crate::core::HRESULT(-2141913087i32);
pub const EAS_E_REQUESTED_POLICY_NOT_ENFORCEABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2141913085i32);
pub const EAS_E_REQUESTED_POLICY_PASSWORD_EXPIRATION_INCOMPATIBLE: crate::core::HRESULT =
    crate::core::HRESULT(-2141913083i32);
pub const EAS_E_USER_CANNOT_CHANGE_PASSWORD: crate::core::HRESULT =
    crate::core::HRESULT(-2141913082i32);
pub const ENUM_E_FIRST: i32 = -2147221072i32;
pub const ENUM_E_LAST: i32 = -2147221057i32;
pub const ENUM_S_FIRST: i32 = 262576i32;
pub const ENUM_S_LAST: i32 = 262591i32;
pub const EPT_NT_CANT_CREATE: NTSTATUS = NTSTATUS(-1073610676i32);
pub const EPT_NT_CANT_PERFORM_OP: NTSTATUS = NTSTATUS(-1073610699i32);
pub const EPT_NT_INVALID_ENTRY: NTSTATUS = NTSTATUS(-1073610700i32);
pub const EPT_NT_NOT_REGISTERED: NTSTATUS = NTSTATUS(-1073610698i32);
pub const ERROR_ALLOWED_PORT_TYPE_RESTRICTION: u32 = 941u32;
pub const ERROR_ALL_SIDS_FILTERED: crate::core::HRESULT = crate::core::HRESULT(-1073151998i32);
pub const ERROR_ALREADY_CONNECTED: u32 = 901u32;
pub const ERROR_ALREADY_CONNECTING: u32 = 910u32;
pub const ERROR_ATTRIBUTE_NOT_PRESENT: crate::core::HRESULT = crate::core::HRESULT(-2138898422i32);
pub const ERROR_AUDITING_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-1073151999i32);
pub const ERROR_AUTHENTICATOR_MISMATCH: u32 = 955u32;
pub const ERROR_AUTH_PROTOCOL_REJECTED: u32 = 917u32;
pub const ERROR_AUTH_PROTOCOL_RESTRICTION: u32 = 942u32;
pub const ERROR_AUTH_SERVER_TIMEOUT: u32 = 930u32;
pub const ERROR_BAP_DISCONNECTED: u32 = 936u32;
pub const ERROR_BAP_REQUIRED: u32 = 943u32;
pub const ERROR_BIZRULES_NOT_ENABLED: crate::core::HRESULT = crate::core::HRESULT(-1073151997i32);
pub const ERROR_CLIENT_INTERFACE_ALREADY_EXISTS: u32 = 915u32;
pub const ERROR_CLIP_DEVICE_LICENSE_MISSING: crate::core::HRESULT =
    crate::core::HRESULT(-1058406397i32);
pub const ERROR_CLIP_KEYHOLDER_LICENSE_MISSING_OR_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-1058406395i32);
pub const ERROR_CLIP_LICENSE_DEVICE_ID_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-1058406390i32);
pub const ERROR_CLIP_LICENSE_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-1058406394i32);
pub const ERROR_CLIP_LICENSE_HARDWARE_ID_OUT_OF_TOLERANCE: crate::core::HRESULT =
    crate::core::HRESULT(-1058406391i32);
pub const ERROR_CLIP_LICENSE_INVALID_SIGNATURE: crate::core::HRESULT =
    crate::core::HRESULT(-1058406396i32);
pub const ERROR_CLIP_LICENSE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-1058406398i32);
pub const ERROR_CLIP_LICENSE_NOT_SIGNED: crate::core::HRESULT =
    crate::core::HRESULT(-1058406392i32);
pub const ERROR_CLIP_LICENSE_SIGNED_BY_UNKNOWN_SOURCE: crate::core::HRESULT =
    crate::core::HRESULT(-1058406393i32);
pub const ERROR_CRED_REQUIRES_CONFIRMATION: crate::core::HRESULT =
    crate::core::HRESULT(-2146865127i32);
pub const ERROR_DBG_ATTACH_PROCESS_FAILURE_LOCKDOWN: crate::core::HRESULT =
    crate::core::HRESULT(-2135949310i32);
pub const ERROR_DBG_CONNECT_SERVER_FAILURE_LOCKDOWN: crate::core::HRESULT =
    crate::core::HRESULT(-2135949309i32);
pub const ERROR_DBG_CREATE_PROCESS_FAILURE_LOCKDOWN: crate::core::HRESULT =
    crate::core::HRESULT(-2135949311i32);
pub const ERROR_DBG_START_SERVER_FAILURE_LOCKDOWN: crate::core::HRESULT =
    crate::core::HRESULT(-2135949308i32);
pub const ERROR_DDM_NOT_RUNNING: u32 = 903u32;
pub const ERROR_DIALIN_HOURS_RESTRICTION: u32 = 940u32;
pub const ERROR_DIALOUT_HOURS_RESTRICTION: u32 = 944u32;
pub const ERROR_FLT_ALREADY_ENLISTED: crate::core::HRESULT = crate::core::HRESULT(-2145452005i32);
pub const ERROR_FLT_CBDQ_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2145452018i32);
pub const ERROR_FLT_CONTEXT_ALLOCATION_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2145452010i32);
pub const ERROR_FLT_CONTEXT_ALREADY_DEFINED: crate::core::HRESULT =
    crate::core::HRESULT(-2145452030i32);
pub const ERROR_FLT_CONTEXT_ALREADY_LINKED: crate::core::HRESULT =
    crate::core::HRESULT(-2145452004i32);
pub const ERROR_FLT_DELETING_OBJECT: crate::core::HRESULT = crate::core::HRESULT(-2145452021i32);
pub const ERROR_FLT_DISALLOW_FAST_IO: crate::core::HRESULT = crate::core::HRESULT(-2145452028i32);
pub const ERROR_FLT_DO_NOT_ATTACH: crate::core::HRESULT = crate::core::HRESULT(-2145452017i32);
pub const ERROR_FLT_DO_NOT_DETACH: crate::core::HRESULT = crate::core::HRESULT(-2145452016i32);
pub const ERROR_FLT_DUPLICATE_ENTRY: crate::core::HRESULT = crate::core::HRESULT(-2145452019i32);
pub const ERROR_FLT_FILTER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2145452013i32);
pub const ERROR_FLT_FILTER_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-2145452024i32);
pub const ERROR_FLT_INSTANCE_ALTITUDE_COLLISION: crate::core::HRESULT =
    crate::core::HRESULT(-2145452015i32);
pub const ERROR_FLT_INSTANCE_NAME_COLLISION: crate::core::HRESULT =
    crate::core::HRESULT(-2145452014i32);
pub const ERROR_FLT_INSTANCE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2145452011i32);
pub const ERROR_FLT_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2145452022i32);
pub const ERROR_FLT_INVALID_ASYNCHRONOUS_REQUEST: crate::core::HRESULT =
    crate::core::HRESULT(-2145452029i32);
pub const ERROR_FLT_INVALID_CONTEXT_REGISTRATION: crate::core::HRESULT =
    crate::core::HRESULT(-2145452009i32);
pub const ERROR_FLT_INVALID_NAME_REQUEST: crate::core::HRESULT =
    crate::core::HRESULT(-2145452027i32);
pub const ERROR_FLT_IO_COMPLETE: crate::core::HRESULT = crate::core::HRESULT(2031617i32);
pub const ERROR_FLT_MUST_BE_NONPAGED_POOL: crate::core::HRESULT =
    crate::core::HRESULT(-2145452020i32);
pub const ERROR_FLT_NAME_CACHE_MISS: crate::core::HRESULT = crate::core::HRESULT(-2145452008i32);
pub const ERROR_FLT_NOT_INITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2145452025i32);
pub const ERROR_FLT_NOT_SAFE_TO_POST_OPERATION: crate::core::HRESULT =
    crate::core::HRESULT(-2145452026i32);
pub const ERROR_FLT_NO_DEVICE_OBJECT: crate::core::HRESULT = crate::core::HRESULT(-2145452007i32);
pub const ERROR_FLT_NO_HANDLER_DEFINED: crate::core::HRESULT = crate::core::HRESULT(-2145452031i32);
pub const ERROR_FLT_NO_WAITER_FOR_REPLY: crate::core::HRESULT =
    crate::core::HRESULT(-2145452000i32);
pub const ERROR_FLT_POST_OPERATION_CLEANUP: crate::core::HRESULT =
    crate::core::HRESULT(-2145452023i32);
pub const ERROR_FLT_REGISTRATION_BUSY: crate::core::HRESULT = crate::core::HRESULT(-2145451997i32);
pub const ERROR_FLT_VOLUME_ALREADY_MOUNTED: crate::core::HRESULT =
    crate::core::HRESULT(-2145452006i32);
pub const ERROR_FLT_VOLUME_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2145452012i32);
pub const ERROR_FLT_WCOS_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2145451996i32);
pub const ERROR_GRAPHICS_ADAPTER_ACCESS_NOT_EXCLUDED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242181i32);
pub const ERROR_GRAPHICS_ADAPTER_CHAIN_NOT_READY: crate::core::HRESULT =
    crate::core::HRESULT(-1071242189i32);
pub const ERROR_GRAPHICS_ADAPTER_MUST_HAVE_AT_LEAST_ONE_SOURCE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242456i32);
pub const ERROR_GRAPHICS_ADAPTER_MUST_HAVE_AT_LEAST_ONE_TARGET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242455i32);
pub const ERROR_GRAPHICS_ADAPTER_WAS_RESET: crate::core::HRESULT =
    crate::core::HRESULT(-1071243261i32);
pub const ERROR_GRAPHICS_ALLOCATION_BUSY: crate::core::HRESULT =
    crate::core::HRESULT(-1071243006i32);
pub const ERROR_GRAPHICS_ALLOCATION_CLOSED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242990i32);
pub const ERROR_GRAPHICS_ALLOCATION_CONTENT_LOST: crate::core::HRESULT =
    crate::core::HRESULT(-1071242986i32);
pub const ERROR_GRAPHICS_ALLOCATION_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-1071243002i32);
pub const ERROR_GRAPHICS_CANCEL_VIDPN_TOPOLOGY_AUGMENTATION: crate::core::HRESULT =
    crate::core::HRESULT(-1071242406i32);
pub const ERROR_GRAPHICS_CANNOTCOLORCONVERT: crate::core::HRESULT =
    crate::core::HRESULT(-1071243256i32);
pub const ERROR_GRAPHICS_CANT_ACCESS_ACTIVE_VIDPN: crate::core::HRESULT =
    crate::core::HRESULT(-1071242429i32);
pub const ERROR_GRAPHICS_CANT_EVICT_PINNED_ALLOCATION: crate::core::HRESULT =
    crate::core::HRESULT(-1071242999i32);
pub const ERROR_GRAPHICS_CANT_LOCK_MEMORY: crate::core::HRESULT =
    crate::core::HRESULT(-1071243007i32);
pub const ERROR_GRAPHICS_CANT_RENDER_LOCKED_ALLOCATION: crate::core::HRESULT =
    crate::core::HRESULT(-1071242991i32);
pub const ERROR_GRAPHICS_CHAINLINKS_NOT_ENUMERATED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242190i32);
pub const ERROR_GRAPHICS_CHAINLINKS_NOT_POWERED_ON: crate::core::HRESULT =
    crate::core::HRESULT(-1071242187i32);
pub const ERROR_GRAPHICS_CHAINLINKS_NOT_STARTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242188i32);
pub const ERROR_GRAPHICS_CHILD_DESCRIPTOR_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242239i32);
pub const ERROR_GRAPHICS_CLIENTVIDPN_NOT_SET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242404i32);
pub const ERROR_GRAPHICS_COPP_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071241983i32);
pub const ERROR_GRAPHICS_DATASET_IS_EMPTY: crate::core::HRESULT = crate::core::HRESULT(2499403i32);
pub const ERROR_GRAPHICS_DDCCI_CURRENT_CURRENT_VALUE_GREATER_THAN_MAXIMUM_VALUE:
    crate::core::HRESULT = crate::core::HRESULT(-1071241768i32);
pub const ERROR_GRAPHICS_DDCCI_INVALID_DATA: crate::core::HRESULT =
    crate::core::HRESULT(-1071241851i32);
pub const ERROR_GRAPHICS_DDCCI_INVALID_MESSAGE_CHECKSUM: crate::core::HRESULT =
    crate::core::HRESULT(-1071241845i32);
pub const ERROR_GRAPHICS_DDCCI_INVALID_MESSAGE_COMMAND: crate::core::HRESULT =
    crate::core::HRESULT(-1071241847i32);
pub const ERROR_GRAPHICS_DDCCI_INVALID_MESSAGE_LENGTH: crate::core::HRESULT =
    crate::core::HRESULT(-1071241846i32);
pub const ERROR_GRAPHICS_DDCCI_MONITOR_RETURNED_INVALID_TIMING_STATUS_BYTE: crate::core::HRESULT =
    crate::core::HRESULT(-1071241850i32);
pub const ERROR_GRAPHICS_DDCCI_VCP_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071241852i32);
pub const ERROR_GRAPHICS_DEPENDABLE_CHILD_STATUS: crate::core::HRESULT =
    crate::core::HRESULT(1076241468i32);
pub const ERROR_GRAPHICS_DISPLAY_DEVICE_NOT_ATTACHED_TO_DESKTOP: crate::core::HRESULT =
    crate::core::HRESULT(-1071241758i32);
pub const ERROR_GRAPHICS_DRIVER_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-1071243255i32);
pub const ERROR_GRAPHICS_EMPTY_ADAPTER_MONITOR_MODE_SUPPORT_INTERSECTION: crate::core::HRESULT =
    crate::core::HRESULT(-1071242459i32);
pub const ERROR_GRAPHICS_FREQUENCYRANGE_ALREADY_IN_SET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242465i32);
pub const ERROR_GRAPHICS_FREQUENCYRANGE_NOT_IN_SET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242467i32);
pub const ERROR_GRAPHICS_GAMMA_RAMP_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242424i32);
pub const ERROR_GRAPHICS_GPU_EXCEPTION_ON_DEVICE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242752i32);
pub const ERROR_GRAPHICS_I2C_DEVICE_DOES_NOT_EXIST: crate::core::HRESULT =
    crate::core::HRESULT(-1071241855i32);
pub const ERROR_GRAPHICS_I2C_ERROR_RECEIVING_DATA: crate::core::HRESULT =
    crate::core::HRESULT(-1071241853i32);
pub const ERROR_GRAPHICS_I2C_ERROR_TRANSMITTING_DATA: crate::core::HRESULT =
    crate::core::HRESULT(-1071241854i32);
pub const ERROR_GRAPHICS_I2C_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071241856i32);
pub const ERROR_GRAPHICS_INCOMPATIBLE_PRIVATE_FORMAT: crate::core::HRESULT =
    crate::core::HRESULT(-1071242411i32);
pub const ERROR_GRAPHICS_INCONSISTENT_DEVICE_LINK_STATE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242186i32);
pub const ERROR_GRAPHICS_INDIRECT_DISPLAY_ABANDON_SWAPCHAIN: crate::core::HRESULT =
    crate::core::HRESULT(-1071243246i32);
pub const ERROR_GRAPHICS_INDIRECT_DISPLAY_DEVICE_STOPPED: crate::core::HRESULT =
    crate::core::HRESULT(-1071243245i32);
pub const ERROR_GRAPHICS_INSUFFICIENT_DMA_BUFFER: crate::core::HRESULT =
    crate::core::HRESULT(-1071243263i32);
pub const ERROR_GRAPHICS_INTERNAL_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-1071241753i32);
pub const ERROR_GRAPHICS_INVALID_ACTIVE_REGION: crate::core::HRESULT =
    crate::core::HRESULT(-1071242485i32);
pub const ERROR_GRAPHICS_INVALID_ALLOCATION_HANDLE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242988i32);
pub const ERROR_GRAPHICS_INVALID_ALLOCATION_INSTANCE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242989i32);
pub const ERROR_GRAPHICS_INVALID_ALLOCATION_USAGE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242992i32);
pub const ERROR_GRAPHICS_INVALID_CLIENT_TYPE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242405i32);
pub const ERROR_GRAPHICS_INVALID_COLORBASIS: crate::core::HRESULT =
    crate::core::HRESULT(-1071242434i32);
pub const ERROR_GRAPHICS_INVALID_COPYPROTECTION_TYPE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242417i32);
pub const ERROR_GRAPHICS_INVALID_DISPLAY_ADAPTER: crate::core::HRESULT =
    crate::core::HRESULT(-1071243262i32);
pub const ERROR_GRAPHICS_INVALID_DRIVER_MODEL: crate::core::HRESULT =
    crate::core::HRESULT(-1071243260i32);
pub const ERROR_GRAPHICS_INVALID_FREQUENCY: crate::core::HRESULT =
    crate::core::HRESULT(-1071242486i32);
pub const ERROR_GRAPHICS_INVALID_GAMMA_RAMP: crate::core::HRESULT =
    crate::core::HRESULT(-1071242425i32);
pub const ERROR_GRAPHICS_INVALID_MODE_PRUNING_ALGORITHM: crate::core::HRESULT =
    crate::core::HRESULT(-1071242410i32);
pub const ERROR_GRAPHICS_INVALID_MONITORDESCRIPTOR: crate::core::HRESULT =
    crate::core::HRESULT(-1071242453i32);
pub const ERROR_GRAPHICS_INVALID_MONITORDESCRIPTORSET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242454i32);
pub const ERROR_GRAPHICS_INVALID_MONITOR_CAPABILITY_ORIGIN: crate::core::HRESULT =
    crate::core::HRESULT(-1071242409i32);
pub const ERROR_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242468i32);
pub const ERROR_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGESET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242469i32);
pub const ERROR_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGE_CONSTRAINT: crate::core::HRESULT =
    crate::core::HRESULT(-1071242408i32);
pub const ERROR_GRAPHICS_INVALID_MONITOR_SOURCEMODESET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242463i32);
pub const ERROR_GRAPHICS_INVALID_MONITOR_SOURCE_MODE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242462i32);
pub const ERROR_GRAPHICS_INVALID_PATH_CONTENT_GEOMETRY_TRANSFORMATION: crate::core::HRESULT =
    crate::core::HRESULT(-1071242427i32);
pub const ERROR_GRAPHICS_INVALID_PATH_CONTENT_TYPE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242418i32);
pub const ERROR_GRAPHICS_INVALID_PATH_IMPORTANCE_ORDINAL: crate::core::HRESULT =
    crate::core::HRESULT(-1071242428i32);
pub const ERROR_GRAPHICS_INVALID_PHYSICAL_MONITOR_HANDLE: crate::core::HRESULT =
    crate::core::HRESULT(-1071241844i32);
pub const ERROR_GRAPHICS_INVALID_PIXELFORMAT: crate::core::HRESULT =
    crate::core::HRESULT(-1071242435i32);
pub const ERROR_GRAPHICS_INVALID_PIXELVALUEACCESSMODE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242433i32);
pub const ERROR_GRAPHICS_INVALID_POINTER: crate::core::HRESULT =
    crate::core::HRESULT(-1071241756i32);
pub const ERROR_GRAPHICS_INVALID_PRIMARYSURFACE_SIZE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242438i32);
pub const ERROR_GRAPHICS_INVALID_SCANLINE_ORDERING: crate::core::HRESULT =
    crate::core::HRESULT(-1071242414i32);
pub const ERROR_GRAPHICS_INVALID_STRIDE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242436i32);
pub const ERROR_GRAPHICS_INVALID_TOTAL_REGION: crate::core::HRESULT =
    crate::core::HRESULT(-1071242484i32);
pub const ERROR_GRAPHICS_INVALID_VIDEOPRESENTSOURCESET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242475i32);
pub const ERROR_GRAPHICS_INVALID_VIDEOPRESENTTARGETSET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242474i32);
pub const ERROR_GRAPHICS_INVALID_VIDEO_PRESENT_SOURCE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242492i32);
pub const ERROR_GRAPHICS_INVALID_VIDEO_PRESENT_SOURCE_MODE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242480i32);
pub const ERROR_GRAPHICS_INVALID_VIDEO_PRESENT_TARGET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242491i32);
pub const ERROR_GRAPHICS_INVALID_VIDEO_PRESENT_TARGET_MODE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242479i32);
pub const ERROR_GRAPHICS_INVALID_VIDPN: crate::core::HRESULT = crate::core::HRESULT(-1071242493i32);
pub const ERROR_GRAPHICS_INVALID_VIDPN_PRESENT_PATH: crate::core::HRESULT =
    crate::core::HRESULT(-1071242471i32);
pub const ERROR_GRAPHICS_INVALID_VIDPN_SOURCEMODESET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242488i32);
pub const ERROR_GRAPHICS_INVALID_VIDPN_TARGETMODESET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242487i32);
pub const ERROR_GRAPHICS_INVALID_VIDPN_TARGET_SUBSET_TYPE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242449i32);
pub const ERROR_GRAPHICS_INVALID_VIDPN_TOPOLOGY: crate::core::HRESULT =
    crate::core::HRESULT(-1071242496i32);
pub const ERROR_GRAPHICS_INVALID_VIDPN_TOPOLOGY_RECOMMENDATION_REASON: crate::core::HRESULT =
    crate::core::HRESULT(-1071242419i32);
pub const ERROR_GRAPHICS_INVALID_VISIBLEREGION_SIZE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242437i32);
pub const ERROR_GRAPHICS_LEADLINK_NOT_ENUMERATED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242191i32);
pub const ERROR_GRAPHICS_LEADLINK_START_DEFERRED: crate::core::HRESULT =
    crate::core::HRESULT(1076241463i32);
pub const ERROR_GRAPHICS_MAX_NUM_PATHS_REACHED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242407i32);
pub const ERROR_GRAPHICS_MCA_INTERNAL_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-1071241848i32);
pub const ERROR_GRAPHICS_MCA_INVALID_CAPABILITIES_STRING: crate::core::HRESULT =
    crate::core::HRESULT(-1071241849i32);
pub const ERROR_GRAPHICS_MCA_INVALID_TECHNOLOGY_TYPE_RETURNED: crate::core::HRESULT =
    crate::core::HRESULT(-1071241762i32);
pub const ERROR_GRAPHICS_MCA_INVALID_VCP_VERSION: crate::core::HRESULT =
    crate::core::HRESULT(-1071241767i32);
pub const ERROR_GRAPHICS_MCA_MCCS_VERSION_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-1071241765i32);
pub const ERROR_GRAPHICS_MCA_MONITOR_VIOLATES_MCCS_SPECIFICATION: crate::core::HRESULT =
    crate::core::HRESULT(-1071241766i32);
pub const ERROR_GRAPHICS_MCA_UNSUPPORTED_COLOR_TEMPERATURE: crate::core::HRESULT =
    crate::core::HRESULT(-1071241761i32);
pub const ERROR_GRAPHICS_MCA_UNSUPPORTED_MCCS_VERSION: crate::core::HRESULT =
    crate::core::HRESULT(-1071241764i32);
pub const ERROR_GRAPHICS_MIRRORING_DEVICES_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071241757i32);
pub const ERROR_GRAPHICS_MODE_ALREADY_IN_MODESET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242476i32);
pub const ERROR_GRAPHICS_MODE_ID_MUST_BE_UNIQUE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242460i32);
pub const ERROR_GRAPHICS_MODE_NOT_IN_MODESET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242422i32);
pub const ERROR_GRAPHICS_MODE_NOT_PINNED: crate::core::HRESULT = crate::core::HRESULT(2499335i32);
pub const ERROR_GRAPHICS_MONITORDESCRIPTOR_ALREADY_IN_SET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242451i32);
pub const ERROR_GRAPHICS_MONITORDESCRIPTOR_ID_MUST_BE_UNIQUE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242450i32);
pub const ERROR_GRAPHICS_MONITORDESCRIPTOR_NOT_IN_SET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242452i32);
pub const ERROR_GRAPHICS_MONITOR_COULD_NOT_BE_ASSOCIATED_WITH_ADAPTER: crate::core::HRESULT =
    crate::core::HRESULT(-1071242444i32);
pub const ERROR_GRAPHICS_MONITOR_NOT_CONNECTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242440i32);
pub const ERROR_GRAPHICS_MONITOR_NO_LONGER_EXISTS: crate::core::HRESULT =
    crate::core::HRESULT(-1071241843i32);
pub const ERROR_GRAPHICS_MULTISAMPLING_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242423i32);
pub const ERROR_GRAPHICS_NOT_A_LINKED_ADAPTER: crate::core::HRESULT =
    crate::core::HRESULT(-1071242192i32);
pub const ERROR_GRAPHICS_NOT_EXCLUSIVE_MODE_OWNER: crate::core::HRESULT =
    crate::core::HRESULT(-1071243264i32);
pub const ERROR_GRAPHICS_NOT_POST_DEVICE_DRIVER: crate::core::HRESULT =
    crate::core::HRESULT(-1071242184i32);
pub const ERROR_GRAPHICS_NO_ACTIVE_VIDPN: crate::core::HRESULT =
    crate::core::HRESULT(-1071242442i32);
pub const ERROR_GRAPHICS_NO_AVAILABLE_IMPORTANCE_ORDINALS: crate::core::HRESULT =
    crate::core::HRESULT(-1071242412i32);
pub const ERROR_GRAPHICS_NO_AVAILABLE_VIDPN_TARGET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242445i32);
pub const ERROR_GRAPHICS_NO_DISPLAY_DEVICE_CORRESPONDS_TO_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-1071241759i32);
pub const ERROR_GRAPHICS_NO_DISPLAY_MODE_MANAGEMENT_SUPPORT: crate::core::HRESULT =
    crate::core::HRESULT(-1071242431i32);
pub const ERROR_GRAPHICS_NO_MONITORS_CORRESPOND_TO_DISPLAY_DEVICE: crate::core::HRESULT =
    crate::core::HRESULT(-1071241755i32);
pub const ERROR_GRAPHICS_NO_MORE_ELEMENTS_IN_DATASET: crate::core::HRESULT =
    crate::core::HRESULT(2499404i32);
pub const ERROR_GRAPHICS_NO_PREFERRED_MODE: crate::core::HRESULT = crate::core::HRESULT(2499358i32);
pub const ERROR_GRAPHICS_NO_RECOMMENDED_FUNCTIONAL_VIDPN: crate::core::HRESULT =
    crate::core::HRESULT(-1071242461i32);
pub const ERROR_GRAPHICS_NO_RECOMMENDED_VIDPN_TOPOLOGY: crate::core::HRESULT =
    crate::core::HRESULT(-1071242470i32);
pub const ERROR_GRAPHICS_NO_VIDEO_MEMORY: crate::core::HRESULT =
    crate::core::HRESULT(-1071243008i32);
pub const ERROR_GRAPHICS_NO_VIDPNMGR: crate::core::HRESULT = crate::core::HRESULT(-1071242443i32);
pub const ERROR_GRAPHICS_ONLY_CONSOLE_SESSION_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071241760i32);
pub const ERROR_GRAPHICS_OPM_ALL_HDCP_HARDWARE_ALREADY_IN_USE: crate::core::HRESULT =
    crate::core::HRESULT(-1071241960i32);
pub const ERROR_GRAPHICS_OPM_DRIVER_INTERNAL_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-1071241954i32);
pub const ERROR_GRAPHICS_OPM_HDCP_SRM_NEVER_SET: crate::core::HRESULT =
    crate::core::HRESULT(-1071241962i32);
pub const ERROR_GRAPHICS_OPM_INTERNAL_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-1071241973i32);
pub const ERROR_GRAPHICS_OPM_INVALID_CONFIGURATION_REQUEST: crate::core::HRESULT =
    crate::core::HRESULT(-1071241951i32);
pub const ERROR_GRAPHICS_OPM_INVALID_ENCRYPTED_PARAMETERS: crate::core::HRESULT =
    crate::core::HRESULT(-1071241981i32);
pub const ERROR_GRAPHICS_OPM_INVALID_HANDLE: crate::core::HRESULT =
    crate::core::HRESULT(-1071241972i32);
pub const ERROR_GRAPHICS_OPM_INVALID_INFORMATION_REQUEST: crate::core::HRESULT =
    crate::core::HRESULT(-1071241955i32);
pub const ERROR_GRAPHICS_OPM_INVALID_SRM: crate::core::HRESULT =
    crate::core::HRESULT(-1071241966i32);
pub const ERROR_GRAPHICS_OPM_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071241984i32);
pub const ERROR_GRAPHICS_OPM_NO_VIDEO_OUTPUTS_EXIST: crate::core::HRESULT =
    crate::core::HRESULT(-1071241979i32);
pub const ERROR_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_ACP: crate::core::HRESULT =
    crate::core::HRESULT(-1071241964i32);
pub const ERROR_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_CGMSA: crate::core::HRESULT =
    crate::core::HRESULT(-1071241963i32);
pub const ERROR_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_HDCP: crate::core::HRESULT =
    crate::core::HRESULT(-1071241965i32);
pub const ERROR_GRAPHICS_OPM_RESOLUTION_TOO_HIGH: crate::core::HRESULT =
    crate::core::HRESULT(-1071241961i32);
pub const ERROR_GRAPHICS_OPM_SESSION_TYPE_CHANGE_IN_PROGRESS: crate::core::HRESULT =
    crate::core::HRESULT(-1071241957i32);
pub const ERROR_GRAPHICS_OPM_SIGNALING_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071241952i32);
pub const ERROR_GRAPHICS_OPM_SPANNING_MODE_ENABLED: crate::core::HRESULT =
    crate::core::HRESULT(-1071241969i32);
pub const ERROR_GRAPHICS_OPM_THEATER_MODE_ENABLED: crate::core::HRESULT =
    crate::core::HRESULT(-1071241968i32);
pub const ERROR_GRAPHICS_OPM_VIDEO_OUTPUT_DOES_NOT_HAVE_COPP_SEMANTICS: crate::core::HRESULT =
    crate::core::HRESULT(-1071241956i32);
pub const ERROR_GRAPHICS_OPM_VIDEO_OUTPUT_DOES_NOT_HAVE_OPM_SEMANTICS: crate::core::HRESULT =
    crate::core::HRESULT(-1071241953i32);
pub const ERROR_GRAPHICS_OPM_VIDEO_OUTPUT_NO_LONGER_EXISTS: crate::core::HRESULT =
    crate::core::HRESULT(-1071241958i32);
pub const ERROR_GRAPHICS_PARAMETER_ARRAY_TOO_SMALL: crate::core::HRESULT =
    crate::core::HRESULT(-1071241754i32);
pub const ERROR_GRAPHICS_PARTIAL_DATA_POPULATED: crate::core::HRESULT =
    crate::core::HRESULT(1076240394i32);
pub const ERROR_GRAPHICS_PATH_ALREADY_IN_TOPOLOGY: crate::core::HRESULT =
    crate::core::HRESULT(-1071242477i32);
pub const ERROR_GRAPHICS_PATH_CONTENT_GEOMETRY_TRANSFORMATION_NOT_PINNED: crate::core::HRESULT =
    crate::core::HRESULT(2499409i32);
pub const ERROR_GRAPHICS_PATH_CONTENT_GEOMETRY_TRANSFORMATION_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242426i32);
pub const ERROR_GRAPHICS_PATH_NOT_IN_TOPOLOGY: crate::core::HRESULT =
    crate::core::HRESULT(-1071242457i32);
pub const ERROR_GRAPHICS_PINNED_MODE_MUST_REMAIN_IN_SET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242478i32);
pub const ERROR_GRAPHICS_POLLING_TOO_FREQUENTLY: crate::core::HRESULT =
    crate::core::HRESULT(1076241465i32);
pub const ERROR_GRAPHICS_PRESENT_BUFFER_NOT_BOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1071243248i32);
pub const ERROR_GRAPHICS_PRESENT_DENIED: crate::core::HRESULT =
    crate::core::HRESULT(-1071243257i32);
pub const ERROR_GRAPHICS_PRESENT_INVALID_WINDOW: crate::core::HRESULT =
    crate::core::HRESULT(-1071243249i32);
pub const ERROR_GRAPHICS_PRESENT_MODE_CHANGED: crate::core::HRESULT =
    crate::core::HRESULT(-1071243259i32);
pub const ERROR_GRAPHICS_PRESENT_OCCLUDED: crate::core::HRESULT =
    crate::core::HRESULT(-1071243258i32);
pub const ERROR_GRAPHICS_PRESENT_REDIRECTION_DISABLED: crate::core::HRESULT =
    crate::core::HRESULT(-1071243253i32);
pub const ERROR_GRAPHICS_PRESENT_UNOCCLUDED: crate::core::HRESULT =
    crate::core::HRESULT(-1071243252i32);
pub const ERROR_GRAPHICS_PVP_HFS_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-1071241967i32);
pub const ERROR_GRAPHICS_PVP_INVALID_CERTIFICATE_LENGTH: crate::core::HRESULT =
    crate::core::HRESULT(-1071241970i32);
pub const ERROR_GRAPHICS_RESOURCES_NOT_RELATED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242448i32);
pub const ERROR_GRAPHICS_SESSION_TYPE_CHANGE_IN_PROGRESS: crate::core::HRESULT =
    crate::core::HRESULT(-1071249944i32);
pub const ERROR_GRAPHICS_SKIP_ALLOCATION_PREPARATION: crate::core::HRESULT =
    crate::core::HRESULT(1076240897i32);
pub const ERROR_GRAPHICS_SOURCE_ALREADY_IN_SET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242473i32);
pub const ERROR_GRAPHICS_SOURCE_ID_MUST_BE_UNIQUE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242447i32);
pub const ERROR_GRAPHICS_SOURCE_NOT_IN_TOPOLOGY: crate::core::HRESULT =
    crate::core::HRESULT(-1071242439i32);
pub const ERROR_GRAPHICS_SPECIFIED_CHILD_ALREADY_CONNECTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242240i32);
pub const ERROR_GRAPHICS_STALE_MODESET: crate::core::HRESULT = crate::core::HRESULT(-1071242464i32);
pub const ERROR_GRAPHICS_STALE_VIDPN_TOPOLOGY: crate::core::HRESULT =
    crate::core::HRESULT(-1071242441i32);
pub const ERROR_GRAPHICS_START_DEFERRED: crate::core::HRESULT = crate::core::HRESULT(1076241466i32);
pub const ERROR_GRAPHICS_TARGET_ALREADY_IN_SET: crate::core::HRESULT =
    crate::core::HRESULT(-1071242472i32);
pub const ERROR_GRAPHICS_TARGET_ID_MUST_BE_UNIQUE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242446i32);
pub const ERROR_GRAPHICS_TARGET_NOT_IN_TOPOLOGY: crate::core::HRESULT =
    crate::core::HRESULT(-1071242432i32);
pub const ERROR_GRAPHICS_TOO_MANY_REFERENCES: crate::core::HRESULT =
    crate::core::HRESULT(-1071243005i32);
pub const ERROR_GRAPHICS_TOPOLOGY_CHANGES_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242413i32);
pub const ERROR_GRAPHICS_TRY_AGAIN_LATER: crate::core::HRESULT =
    crate::core::HRESULT(-1071243004i32);
pub const ERROR_GRAPHICS_TRY_AGAIN_NOW: crate::core::HRESULT = crate::core::HRESULT(-1071243003i32);
pub const ERROR_GRAPHICS_UAB_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071241982i32);
pub const ERROR_GRAPHICS_UNASSIGNED_MODESET_ALREADY_EXISTS: crate::core::HRESULT =
    crate::core::HRESULT(-1071242416i32);
pub const ERROR_GRAPHICS_UNKNOWN_CHILD_STATUS: crate::core::HRESULT =
    crate::core::HRESULT(1076241455i32);
pub const ERROR_GRAPHICS_UNSWIZZLING_APERTURE_UNAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1071243001i32);
pub const ERROR_GRAPHICS_UNSWIZZLING_APERTURE_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071243000i32);
pub const ERROR_GRAPHICS_VAIL_FAILED_TO_SEND_COMPOSITION_WINDOW_DPI_MESSAGE: crate::core::HRESULT =
    crate::core::HRESULT(-1071243242i32);
pub const ERROR_GRAPHICS_VAIL_FAILED_TO_SEND_CREATE_SUPERWETINK_MESSAGE: crate::core::HRESULT =
    crate::core::HRESULT(-1071243244i32);
pub const ERROR_GRAPHICS_VAIL_FAILED_TO_SEND_DESTROY_SUPERWETINK_MESSAGE: crate::core::HRESULT =
    crate::core::HRESULT(-1071243243i32);
pub const ERROR_GRAPHICS_VAIL_STATE_CHANGED: crate::core::HRESULT =
    crate::core::HRESULT(-1071243247i32);
pub const ERROR_GRAPHICS_VIDEO_PRESENT_TARGETS_LESS_THAN_SOURCES: crate::core::HRESULT =
    crate::core::HRESULT(-1071242458i32);
pub const ERROR_GRAPHICS_VIDPN_MODALITY_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242490i32);
pub const ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242430i32);
pub const ERROR_GRAPHICS_VIDPN_TOPOLOGY_CURRENTLY_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242494i32);
pub const ERROR_GRAPHICS_VIDPN_TOPOLOGY_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1071242495i32);
pub const ERROR_GRAPHICS_WINDOWDC_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1071243251i32);
pub const ERROR_GRAPHICS_WINDOWLESS_PRESENT_DISABLED: crate::core::HRESULT =
    crate::core::HRESULT(-1071243250i32);
pub const ERROR_GRAPHICS_WRONG_ALLOCATION_DEVICE: crate::core::HRESULT =
    crate::core::HRESULT(-1071242987i32);
pub const ERROR_HUNG_DISPLAY_DRIVER_THREAD: crate::core::HRESULT =
    crate::core::HRESULT(-2144993279i32);
pub const ERROR_IDLE_DISCONNECTED: u32 = 926u32;
pub const ERROR_INTERFACE_ALREADY_EXISTS: u32 = 904u32;
pub const ERROR_INTERFACE_CONFIGURATION: u32 = 912u32;
pub const ERROR_INTERFACE_CONNECTED: u32 = 908u32;
pub const ERROR_INTERFACE_DISABLED: u32 = 916u32;
pub const ERROR_INTERFACE_DISCONNECTED: u32 = 929u32;
pub const ERROR_INTERFACE_HAS_NO_DEVICES: u32 = 925u32;
pub const ERROR_INTERFACE_NOT_CONNECTED: u32 = 906u32;
pub const ERROR_INTERFACE_UNREACHABLE: u32 = 927u32;
pub const ERROR_INVALID_ATTRIBUTE_LENGTH: u32 = 953u32;
pub const ERROR_INVALID_PACKET: u32 = 954u32;
pub const ERROR_INVALID_PACKET_LENGTH_OR_ID: u32 = 952u32;
pub const ERROR_INVALID_RADIUS_RESPONSE: u32 = 939u32;
pub const ERROR_INVALID_SIGNATURE: u32 = 950u32;
pub const ERROR_INVALID_SIGNATURE_LENGTH: u32 = 949u32;
pub const ERROR_IO_PREEMPTED: crate::core::HRESULT = crate::core::HRESULT(-1996423167i32);
pub const ERROR_MAX_CLIENT_INTERFACE_LIMIT: u32 = 935u32;
pub const ERROR_MAX_LAN_INTERFACE_LIMIT: u32 = 933u32;
pub const ERROR_MAX_WAN_INTERFACE_LIMIT: u32 = 934u32;
pub const ERROR_MONITOR_INVALID_DESCRIPTOR_CHECKSUM: crate::core::HRESULT =
    crate::core::HRESULT(-1071247357i32);
pub const ERROR_MONITOR_INVALID_DETAILED_TIMING_BLOCK: crate::core::HRESULT =
    crate::core::HRESULT(-1071247351i32);
pub const ERROR_MONITOR_INVALID_MANUFACTURE_DATE: crate::core::HRESULT =
    crate::core::HRESULT(-1071247350i32);
pub const ERROR_MONITOR_INVALID_SERIAL_NUMBER_MONDSC_BLOCK: crate::core::HRESULT =
    crate::core::HRESULT(-1071247354i32);
pub const ERROR_MONITOR_INVALID_STANDARD_TIMING_BLOCK: crate::core::HRESULT =
    crate::core::HRESULT(-1071247356i32);
pub const ERROR_MONITOR_INVALID_USER_FRIENDLY_MONDSC_BLOCK: crate::core::HRESULT =
    crate::core::HRESULT(-1071247353i32);
pub const ERROR_MONITOR_NO_DESCRIPTOR: crate::core::HRESULT = crate::core::HRESULT(2494465i32);
pub const ERROR_MONITOR_NO_MORE_DESCRIPTOR_DATA: crate::core::HRESULT =
    crate::core::HRESULT(-1071247352i32);
pub const ERROR_MONITOR_UNKNOWN_DESCRIPTOR_FORMAT: crate::core::HRESULT =
    crate::core::HRESULT(2494466i32);
pub const ERROR_MONITOR_WMI_DATABLOCK_REGISTRATION_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-1071247355i32);
pub const ERROR_NOT_A_TIERED_VOLUME: crate::core::HRESULT = crate::core::HRESULT(-2138898423i32);
pub const ERROR_NOT_CLIENT_PORT: u32 = 913u32;
pub const ERROR_NOT_ROUTER_PORT: u32 = 914u32;
pub const ERROR_NO_APPLICABLE_APP_LICENSES_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1058406399i32);
pub const ERROR_NO_AUTH_PROTOCOL_AVAILABLE: u32 = 918u32;
pub const ERROR_NO_INTERFACE_CREDENTIALS_SET: u32 = 909u32;
pub const ERROR_NO_RADIUS_SERVERS: u32 = 938u32;
pub const ERROR_NO_SIGNATURE: u32 = 951u32;
pub const ERROR_NO_SUCH_INTERFACE: u32 = 905u32;
pub const ERROR_PEER_REFUSED_AUTH: u32 = 919u32;
pub const ERROR_PORT_LIMIT_REACHED: u32 = 931u32;
pub const ERROR_PPP_SESSION_TIMEOUT: u32 = 932u32;
pub const ERROR_PROTOCOL_ALREADY_INSTALLED: u32 = 948u32;
pub const ERROR_PROTOCOL_STOP_PENDING: u32 = 907u32;
pub const ERROR_QUIC_ALPN_NEG_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2143223801i32);
pub const ERROR_QUIC_CONNECTION_IDLE: crate::core::HRESULT = crate::core::HRESULT(-2143223803i32);
pub const ERROR_QUIC_CONNECTION_TIMEOUT: crate::core::HRESULT =
    crate::core::HRESULT(-2143223802i32);
pub const ERROR_QUIC_HANDSHAKE_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2143223808i32);
pub const ERROR_QUIC_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2143223805i32);
pub const ERROR_QUIC_PROTOCOL_VIOLATION: crate::core::HRESULT =
    crate::core::HRESULT(-2143223804i32);
pub const ERROR_QUIC_USER_CANCELED: crate::core::HRESULT = crate::core::HRESULT(-2143223806i32);
pub const ERROR_QUIC_VER_NEG_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2143223807i32);
pub const ERROR_REMOTEACCESS_NOT_CONFIGURED: u32 = 956u32;
pub const ERROR_REMOTE_ACCT_DISABLED: u32 = 922u32;
pub const ERROR_REMOTE_AUTHENTICATION_FAILURE: u32 = 924u32;
pub const ERROR_REMOTE_NO_DIALIN_PERMISSION: u32 = 920u32;
pub const ERROR_REMOTE_PASSWD_EXPIRED: u32 = 921u32;
pub const ERROR_REMOTE_RESTRICTED_LOGON_HOURS: u32 = 923u32;
pub const ERROR_ROUTER_CONFIG_INCOMPATIBLE: u32 = 945u32;
pub const ERROR_ROUTER_STOPPED: u32 = 900u32;
pub const ERROR_SECCORE_INVALID_COMMAND: crate::core::HRESULT =
    crate::core::HRESULT(-1058537472i32);
pub const ERROR_SERVICE_IS_PAUSED: u32 = 928u32;
pub const ERROR_SMB_BAD_CLUSTER_DIALECT: crate::core::HRESULT =
    crate::core::HRESULT(-1067646975i32);
pub const ERROR_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP: crate::core::HRESULT =
    crate::core::HRESULT(-1067646976i32);
pub const ERROR_SMB_NO_SIGNING_ALGORITHM_OVERLAP: crate::core::HRESULT =
    crate::core::HRESULT(-1067646974i32);
pub const ERROR_SPACES_ALLOCATION_SIZE_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2132344818i32);
pub const ERROR_SPACES_CACHE_FULL: crate::core::HRESULT = crate::core::HRESULT(-2132344794i32);
pub const ERROR_SPACES_CORRUPT_METADATA: crate::core::HRESULT =
    crate::core::HRESULT(-2132344808i32);
pub const ERROR_SPACES_DRIVE_LOST_DATA: crate::core::HRESULT = crate::core::HRESULT(-2132344801i32);
pub const ERROR_SPACES_DRIVE_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-2132344803i32);
pub const ERROR_SPACES_DRIVE_OPERATIONAL_STATE_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2132344814i32);
pub const ERROR_SPACES_DRIVE_REDUNDANCY_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2132344826i32);
pub const ERROR_SPACES_DRIVE_SECTOR_SIZE_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2132344828i32);
pub const ERROR_SPACES_DRIVE_SPLIT: crate::core::HRESULT = crate::core::HRESULT(-2132344802i32);
pub const ERROR_SPACES_DRT_FULL: crate::core::HRESULT = crate::core::HRESULT(-2132344807i32);
pub const ERROR_SPACES_ENCLOSURE_AWARE_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2132344817i32);
pub const ERROR_SPACES_ENTRY_INCOMPLETE: crate::core::HRESULT =
    crate::core::HRESULT(-2132344813i32);
pub const ERROR_SPACES_ENTRY_INVALID: crate::core::HRESULT = crate::core::HRESULT(-2132344812i32);
pub const ERROR_SPACES_EXTENDED_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2132344820i32);
pub const ERROR_SPACES_FAULT_DOMAIN_TYPE_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2132344831i32);
pub const ERROR_SPACES_FLUSH_METADATA: crate::core::HRESULT = crate::core::HRESULT(-2132344795i32);
pub const ERROR_SPACES_INCONSISTENCY: crate::core::HRESULT = crate::core::HRESULT(-2132344806i32);
pub const ERROR_SPACES_INTERLEAVE_LENGTH_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2132344823i32);
pub const ERROR_SPACES_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2132344830i32);
pub const ERROR_SPACES_LOG_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-2132344805i32);
pub const ERROR_SPACES_MAP_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2132344810i32);
pub const ERROR_SPACES_MARK_DIRTY: crate::core::HRESULT = crate::core::HRESULT(-2132344800i32);
pub const ERROR_SPACES_NOT_ENOUGH_DRIVES: crate::core::HRESULT =
    crate::core::HRESULT(-2132344821i32);
pub const ERROR_SPACES_NO_REDUNDANCY: crate::core::HRESULT = crate::core::HRESULT(-2132344804i32);
pub const ERROR_SPACES_NUMBER_OF_COLUMNS_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2132344822i32);
pub const ERROR_SPACES_NUMBER_OF_DATA_COPIES_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2132344825i32);
pub const ERROR_SPACES_NUMBER_OF_GROUPS_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2132344815i32);
pub const ERROR_SPACES_PARITY_LAYOUT_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2132344824i32);
pub const ERROR_SPACES_POOL_WAS_DELETED: crate::core::HRESULT = crate::core::HRESULT(15138817i32);
pub const ERROR_SPACES_PROVISIONING_TYPE_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2132344819i32);
pub const ERROR_SPACES_RESILIENCY_TYPE_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2132344829i32);
pub const ERROR_SPACES_UNSUPPORTED_VERSION: crate::core::HRESULT =
    crate::core::HRESULT(-2132344809i32);
pub const ERROR_SPACES_UPDATE_COLUMN_STATE: crate::core::HRESULT =
    crate::core::HRESULT(-2132344811i32);
pub const ERROR_SPACES_WRITE_CACHE_SIZE_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2132344816i32);
pub const ERROR_SVHDX_ERROR_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1067647232i32);
pub const ERROR_SVHDX_ERROR_STORED: crate::core::HRESULT = crate::core::HRESULT(-1067712512i32);
pub const ERROR_SVHDX_NO_INITIATOR: crate::core::HRESULT = crate::core::HRESULT(-1067647221i32);
pub const ERROR_SVHDX_RESERVATION_CONFLICT: crate::core::HRESULT =
    crate::core::HRESULT(-1067647225i32);
pub const ERROR_SVHDX_UNIT_ATTENTION_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-1067647231i32);
pub const ERROR_SVHDX_UNIT_ATTENTION_CAPACITY_DATA_CHANGED: crate::core::HRESULT =
    crate::core::HRESULT(-1067647230i32);
pub const ERROR_SVHDX_UNIT_ATTENTION_OPERATING_DEFINITION_CHANGED: crate::core::HRESULT =
    crate::core::HRESULT(-1067647226i32);
pub const ERROR_SVHDX_UNIT_ATTENTION_REGISTRATIONS_PREEMPTED: crate::core::HRESULT =
    crate::core::HRESULT(-1067647227i32);
pub const ERROR_SVHDX_UNIT_ATTENTION_RESERVATIONS_PREEMPTED: crate::core::HRESULT =
    crate::core::HRESULT(-1067647229i32);
pub const ERROR_SVHDX_UNIT_ATTENTION_RESERVATIONS_RELEASED: crate::core::HRESULT =
    crate::core::HRESULT(-1067647228i32);
pub const ERROR_SVHDX_VERSION_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-1067647223i32);
pub const ERROR_SVHDX_WRONG_FILE_TYPE: crate::core::HRESULT = crate::core::HRESULT(-1067647224i32);
pub const ERROR_TIERING_ALREADY_PROCESSING: crate::core::HRESULT =
    crate::core::HRESULT(-2138898426i32);
pub const ERROR_TIERING_CANNOT_PIN_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-2138898425i32);
pub const ERROR_TIERING_FILE_IS_NOT_PINNED: crate::core::HRESULT =
    crate::core::HRESULT(-2138898424i32);
pub const ERROR_TIERING_INVALID_FILE_ID: crate::core::HRESULT =
    crate::core::HRESULT(-2138898428i32);
pub const ERROR_TIERING_NOT_SUPPORTED_ON_VOLUME: crate::core::HRESULT =
    crate::core::HRESULT(-2138898431i32);
pub const ERROR_TIERING_STORAGE_TIER_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2138898429i32);
pub const ERROR_TIERING_VOLUME_DISMOUNT_IN_PROGRESS: crate::core::HRESULT =
    crate::core::HRESULT(-2138898430i32);
pub const ERROR_TIERING_WRONG_CLUSTER_NODE: crate::core::HRESULT =
    crate::core::HRESULT(-2138898427i32);
pub const ERROR_UNKNOWN_PROTOCOL_ID: u32 = 902u32;
pub const ERROR_UPDATE_IN_PROGRESS: u32 = 911u32;
pub const ERROR_USER_LIMIT: u32 = 937u32;
pub const ERROR_VHDSET_BACKING_STORAGE_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1067647220i32);
pub const ERROR_VHD_SHARED: crate::core::HRESULT = crate::core::HRESULT(-1067647222i32);
pub const ERROR_VOLSNAP_ACTIVATION_TIMEOUT: crate::core::HRESULT =
    crate::core::HRESULT(-2138963966i32);
pub const ERROR_VOLSNAP_BOOTFILE_NOT_VALID: crate::core::HRESULT =
    crate::core::HRESULT(-2138963967i32);
pub const ERROR_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT: crate::core::HRESULT =
    crate::core::HRESULT(-2138963965i32);
pub const EVENT_E_ALL_SUBSCRIBERS_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2147220991i32);
pub const EVENT_E_CANT_MODIFY_OR_DELETE_CONFIGURED_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-2147220978i32);
pub const EVENT_E_CANT_MODIFY_OR_DELETE_UNCONFIGURED_OBJECT: crate::core::HRESULT =
    crate::core::HRESULT(-2147220979i32);
pub const EVENT_E_COMPLUS_NOT_INSTALLED: crate::core::HRESULT =
    crate::core::HRESULT(-2147220980i32);
pub const EVENT_E_FIRST: i32 = -2147220992i32;
pub const EVENT_E_INTERNALERROR: crate::core::HRESULT = crate::core::HRESULT(-2147220986i32);
pub const EVENT_E_INTERNALEXCEPTION: crate::core::HRESULT = crate::core::HRESULT(-2147220987i32);
pub const EVENT_E_INVALID_EVENT_CLASS_PARTITION: crate::core::HRESULT =
    crate::core::HRESULT(-2147220977i32);
pub const EVENT_E_INVALID_PER_USER_SID: crate::core::HRESULT = crate::core::HRESULT(-2147220985i32);
pub const EVENT_E_LAST: i32 = -2147220961i32;
pub const EVENT_E_MISSING_EVENTCLASS: crate::core::HRESULT = crate::core::HRESULT(-2147220982i32);
pub const EVENT_E_NOT_ALL_REMOVED: crate::core::HRESULT = crate::core::HRESULT(-2147220981i32);
pub const EVENT_E_PER_USER_SID_NOT_LOGGED_ON: crate::core::HRESULT =
    crate::core::HRESULT(-2147220976i32);
pub const EVENT_E_QUERYFIELD: crate::core::HRESULT = crate::core::HRESULT(-2147220988i32);
pub const EVENT_E_QUERYSYNTAX: crate::core::HRESULT = crate::core::HRESULT(-2147220989i32);
pub const EVENT_E_TOO_MANY_METHODS: crate::core::HRESULT = crate::core::HRESULT(-2147220983i32);
pub const EVENT_E_USER_EXCEPTION: crate::core::HRESULT = crate::core::HRESULT(-2147220984i32);
pub const EVENT_S_FIRST: i32 = 262656i32;
pub const EVENT_S_LAST: i32 = 262687i32;
pub const EVENT_S_NOSUBSCRIBERS: crate::core::HRESULT = crate::core::HRESULT(262658i32);
pub const EVENT_S_SOME_SUBSCRIBERS_FAILED: crate::core::HRESULT = crate::core::HRESULT(262656i32);
pub const EXCEPTION_ACCESS_VIOLATION: NTSTATUS = NTSTATUS(-1073741819i32);
pub const EXCEPTION_ARRAY_BOUNDS_EXCEEDED: NTSTATUS = NTSTATUS(-1073741684i32);
pub const EXCEPTION_BREAKPOINT: NTSTATUS = NTSTATUS(-2147483645i32);
pub const EXCEPTION_DATATYPE_MISALIGNMENT: NTSTATUS = NTSTATUS(-2147483646i32);
pub const EXCEPTION_FLT_DENORMAL_OPERAND: NTSTATUS = NTSTATUS(-1073741683i32);
pub const EXCEPTION_FLT_DIVIDE_BY_ZERO: NTSTATUS = NTSTATUS(-1073741682i32);
pub const EXCEPTION_FLT_INEXACT_RESULT: NTSTATUS = NTSTATUS(-1073741681i32);
pub const EXCEPTION_FLT_INVALID_OPERATION: NTSTATUS = NTSTATUS(-1073741680i32);
pub const EXCEPTION_FLT_OVERFLOW: NTSTATUS = NTSTATUS(-1073741679i32);
pub const EXCEPTION_FLT_STACK_CHECK: NTSTATUS = NTSTATUS(-1073741678i32);
pub const EXCEPTION_FLT_UNDERFLOW: NTSTATUS = NTSTATUS(-1073741677i32);
pub const EXCEPTION_GUARD_PAGE: NTSTATUS = NTSTATUS(-2147483647i32);
pub const EXCEPTION_ILLEGAL_INSTRUCTION: NTSTATUS = NTSTATUS(-1073741795i32);
pub const EXCEPTION_INT_DIVIDE_BY_ZERO: NTSTATUS = NTSTATUS(-1073741676i32);
pub const EXCEPTION_INT_OVERFLOW: NTSTATUS = NTSTATUS(-1073741675i32);
pub const EXCEPTION_INVALID_DISPOSITION: NTSTATUS = NTSTATUS(-1073741786i32);
pub const EXCEPTION_INVALID_HANDLE: NTSTATUS = NTSTATUS(-1073741816i32);
pub const EXCEPTION_IN_PAGE_ERROR: NTSTATUS = NTSTATUS(-1073741818i32);
pub const EXCEPTION_NONCONTINUABLE_EXCEPTION: NTSTATUS = NTSTATUS(-1073741787i32);
pub const EXCEPTION_POSSIBLE_DEADLOCK: NTSTATUS = NTSTATUS(-1073741420i32);
pub const EXCEPTION_PRIV_INSTRUCTION: NTSTATUS = NTSTATUS(-1073741674i32);
pub const EXCEPTION_SINGLE_STEP: NTSTATUS = NTSTATUS(-2147483644i32);
pub const EXCEPTION_STACK_OVERFLOW: NTSTATUS = NTSTATUS(-1073741571i32);
pub const E_ABORT: crate::core::HRESULT = crate::core::HRESULT(-2147467260i32);
pub const E_ACCESSDENIED: crate::core::HRESULT = crate::core::HRESULT(-2147024891i32);
pub const E_APPLICATION_ACTIVATION_EXEC_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-2144927141i32);
pub const E_APPLICATION_ACTIVATION_TIMED_OUT: crate::core::HRESULT =
    crate::core::HRESULT(-2144927142i32);
pub const E_APPLICATION_EXITING: crate::core::HRESULT = crate::core::HRESULT(-2147483622i32);
pub const E_APPLICATION_MANAGER_NOT_RUNNING: crate::core::HRESULT =
    crate::core::HRESULT(-2144927145i32);
pub const E_APPLICATION_NOT_REGISTERED: crate::core::HRESULT = crate::core::HRESULT(-2144927148i32);
pub const E_APPLICATION_TEMPORARY_LICENSE_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-2144927140i32);
pub const E_APPLICATION_TRIAL_LICENSE_EXPIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2144927139i32);
pub const E_APPLICATION_VIEW_EXITING: crate::core::HRESULT = crate::core::HRESULT(-2147483621i32);
pub const E_ASYNC_OPERATION_NOT_STARTED: crate::core::HRESULT =
    crate::core::HRESULT(-2147483623i32);
pub const E_AUDIO_ENGINE_NODE_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2140798975i32);
pub const E_BLUETOOTH_ATT_ATTRIBUTE_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2140864502i32);
pub const E_BLUETOOTH_ATT_ATTRIBUTE_NOT_LONG: crate::core::HRESULT =
    crate::core::HRESULT(-2140864501i32);
pub const E_BLUETOOTH_ATT_INSUFFICIENT_AUTHENTICATION: crate::core::HRESULT =
    crate::core::HRESULT(-2140864507i32);
pub const E_BLUETOOTH_ATT_INSUFFICIENT_AUTHORIZATION: crate::core::HRESULT =
    crate::core::HRESULT(-2140864504i32);
pub const E_BLUETOOTH_ATT_INSUFFICIENT_ENCRYPTION: crate::core::HRESULT =
    crate::core::HRESULT(-2140864497i32);
pub const E_BLUETOOTH_ATT_INSUFFICIENT_ENCRYPTION_KEY_SIZE: crate::core::HRESULT =
    crate::core::HRESULT(-2140864500i32);
pub const E_BLUETOOTH_ATT_INSUFFICIENT_RESOURCES: crate::core::HRESULT =
    crate::core::HRESULT(-2140864495i32);
pub const E_BLUETOOTH_ATT_INVALID_ATTRIBUTE_VALUE_LENGTH: crate::core::HRESULT =
    crate::core::HRESULT(-2140864499i32);
pub const E_BLUETOOTH_ATT_INVALID_HANDLE: crate::core::HRESULT =
    crate::core::HRESULT(-2140864511i32);
pub const E_BLUETOOTH_ATT_INVALID_OFFSET: crate::core::HRESULT =
    crate::core::HRESULT(-2140864505i32);
pub const E_BLUETOOTH_ATT_INVALID_PDU: crate::core::HRESULT = crate::core::HRESULT(-2140864508i32);
pub const E_BLUETOOTH_ATT_PREPARE_QUEUE_FULL: crate::core::HRESULT =
    crate::core::HRESULT(-2140864503i32);
pub const E_BLUETOOTH_ATT_READ_NOT_PERMITTED: crate::core::HRESULT =
    crate::core::HRESULT(-2140864510i32);
pub const E_BLUETOOTH_ATT_REQUEST_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2140864506i32);
pub const E_BLUETOOTH_ATT_UNKNOWN_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-2140860416i32);
pub const E_BLUETOOTH_ATT_UNLIKELY: crate::core::HRESULT = crate::core::HRESULT(-2140864498i32);
pub const E_BLUETOOTH_ATT_UNSUPPORTED_GROUP_TYPE: crate::core::HRESULT =
    crate::core::HRESULT(-2140864496i32);
pub const E_BLUETOOTH_ATT_WRITE_NOT_PERMITTED: crate::core::HRESULT =
    crate::core::HRESULT(-2140864509i32);
pub const E_BOUNDS: crate::core::HRESULT = crate::core::HRESULT(-2147483637i32);
pub const E_CHANGED_STATE: crate::core::HRESULT = crate::core::HRESULT(-2147483636i32);
pub const E_ELEVATED_ACTIVATION_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144927151i32);
pub const E_FAIL: crate::core::HRESULT = crate::core::HRESULT(-2147467259i32);
pub const E_FULL_ADMIN_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2144927149i32);
pub const E_HANDLE: crate::core::HRESULT = crate::core::HRESULT(-2147024890i32);
pub const E_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2140798973i32);
pub const E_HDAUDIO_EMPTY_CONNECTION_LIST: crate::core::HRESULT =
    crate::core::HRESULT(-2140798974i32);
pub const E_HDAUDIO_NO_LOGICAL_DEVICES_CREATED: crate::core::HRESULT =
    crate::core::HRESULT(-2140798972i32);
pub const E_HDAUDIO_NULL_LINKED_LIST_ENTRY: crate::core::HRESULT =
    crate::core::HRESULT(-2140798971i32);
pub const E_ILLEGAL_DELEGATE_ASSIGNMENT: crate::core::HRESULT =
    crate::core::HRESULT(-2147483624i32);
pub const E_ILLEGAL_METHOD_CALL: crate::core::HRESULT = crate::core::HRESULT(-2147483634i32);
pub const E_ILLEGAL_STATE_CHANGE: crate::core::HRESULT = crate::core::HRESULT(-2147483635i32);
pub const E_INVALIDARG: crate::core::HRESULT = crate::core::HRESULT(-2147024809i32);
pub const E_INVALID_PROTOCOL_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-2089418750i32);
pub const E_INVALID_PROTOCOL_OPERATION: crate::core::HRESULT = crate::core::HRESULT(-2089418751i32);
pub const E_MBN_BAD_SIM: crate::core::HRESULT = crate::core::HRESULT(-2141945342i32);
pub const E_MBN_CONTEXT_NOT_ACTIVATED: crate::core::HRESULT = crate::core::HRESULT(-2141945343i32);
pub const E_MBN_DATA_CLASS_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2141945341i32);
pub const E_MBN_DEFAULT_PROFILE_EXIST: crate::core::HRESULT = crate::core::HRESULT(-2141945319i32);
pub const E_MBN_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2141945326i32);
pub const E_MBN_INVALID_ACCESS_STRING: crate::core::HRESULT = crate::core::HRESULT(-2141945340i32);
pub const E_MBN_INVALID_CACHE: crate::core::HRESULT = crate::core::HRESULT(-2141945332i32);
pub const E_MBN_INVALID_PROFILE: crate::core::HRESULT = crate::core::HRESULT(-2141945320i32);
pub const E_MBN_MAX_ACTIVATED_CONTEXTS: crate::core::HRESULT = crate::core::HRESULT(-2141945339i32);
pub const E_MBN_NOT_REGISTERED: crate::core::HRESULT = crate::core::HRESULT(-2141945331i32);
pub const E_MBN_PACKET_SVC_DETACHED: crate::core::HRESULT = crate::core::HRESULT(-2141945338i32);
pub const E_MBN_PIN_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2141945327i32);
pub const E_MBN_PIN_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2141945329i32);
pub const E_MBN_PIN_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2141945328i32);
pub const E_MBN_PROVIDERS_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2141945330i32);
pub const E_MBN_PROVIDER_NOT_VISIBLE: crate::core::HRESULT = crate::core::HRESULT(-2141945337i32);
pub const E_MBN_RADIO_POWER_OFF: crate::core::HRESULT = crate::core::HRESULT(-2141945336i32);
pub const E_MBN_SERVICE_NOT_ACTIVATED: crate::core::HRESULT = crate::core::HRESULT(-2141945335i32);
pub const E_MBN_SIM_NOT_INSERTED: crate::core::HRESULT = crate::core::HRESULT(-2141945334i32);
pub const E_MBN_SMS_ENCODING_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2141945312i32);
pub const E_MBN_SMS_FILTER_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2141945311i32);
pub const E_MBN_SMS_FORMAT_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2141945305i32);
pub const E_MBN_SMS_INVALID_MEMORY_INDEX: crate::core::HRESULT =
    crate::core::HRESULT(-2141945310i32);
pub const E_MBN_SMS_LANG_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2141945309i32);
pub const E_MBN_SMS_MEMORY_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2141945308i32);
pub const E_MBN_SMS_MEMORY_FULL: crate::core::HRESULT = crate::core::HRESULT(-2141945303i32);
pub const E_MBN_SMS_NETWORK_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2141945307i32);
pub const E_MBN_SMS_OPERATION_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2141945304i32);
pub const E_MBN_SMS_UNKNOWN_SMSC_ADDRESS: crate::core::HRESULT =
    crate::core::HRESULT(-2141945306i32);
pub const E_MBN_VOICE_CALL_IN_PROGRESS: crate::core::HRESULT = crate::core::HRESULT(-2141945333i32);
pub const E_MONITOR_RESOLUTION_TOO_LOW: crate::core::HRESULT = crate::core::HRESULT(-2144927152i32);
pub const E_MULTIPLE_EXTENSIONS_FOR_APPLICATION: crate::core::HRESULT =
    crate::core::HRESULT(-2144927147i32);
pub const E_MULTIPLE_PACKAGES_FOR_FAMILY: crate::core::HRESULT =
    crate::core::HRESULT(-2144927146i32);
pub const E_NOINTERFACE: crate::core::HRESULT = crate::core::HRESULT(-2147467262i32);
pub const E_NOTIMPL: crate::core::HRESULT = crate::core::HRESULT(-2147467263i32);
pub const E_OUTOFMEMORY: crate::core::HRESULT = crate::core::HRESULT(-2147024882i32);
pub const E_POINTER: crate::core::HRESULT = crate::core::HRESULT(-2147467261i32);
pub const E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2089418749i32);
pub const E_PROTOCOL_VERSION_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2089418747i32);
pub const E_SKYDRIVE_FILE_NOT_UPLOADED: crate::core::HRESULT = crate::core::HRESULT(-2144927133i32);
pub const E_SKYDRIVE_ROOT_TARGET_CANNOT_INDEX: crate::core::HRESULT =
    crate::core::HRESULT(-2144927134i32);
pub const E_SKYDRIVE_ROOT_TARGET_FILE_SYSTEM_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144927136i32);
pub const E_SKYDRIVE_ROOT_TARGET_OVERLAP: crate::core::HRESULT =
    crate::core::HRESULT(-2144927135i32);
pub const E_SKYDRIVE_ROOT_TARGET_VOLUME_ROOT_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144927131i32);
pub const E_SKYDRIVE_UPDATE_AVAILABILITY_FAIL: crate::core::HRESULT =
    crate::core::HRESULT(-2144927132i32);
pub const E_STRING_NOT_NULL_TERMINATED: crate::core::HRESULT = crate::core::HRESULT(-2147483625i32);
pub const E_SUBPROTOCOL_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2089418748i32);
pub const E_SYNCENGINE_CLIENT_UPDATE_NEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-2013081594i32);
pub const E_SYNCENGINE_FILE_IDENTIFIER_UNKNOWN: crate::core::HRESULT =
    crate::core::HRESULT(-2013085694i32);
pub const E_SYNCENGINE_FILE_SIZE_EXCEEDS_REMAINING_QUOTA: crate::core::HRESULT =
    crate::core::HRESULT(-2013089790i32);
pub const E_SYNCENGINE_FILE_SIZE_OVER_LIMIT: crate::core::HRESULT =
    crate::core::HRESULT(-2013089791i32);
pub const E_SYNCENGINE_FILE_SYNC_PARTNER_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-2013089787i32);
pub const E_SYNCENGINE_FOLDER_INACCESSIBLE: crate::core::HRESULT =
    crate::core::HRESULT(-2013081599i32);
pub const E_SYNCENGINE_FOLDER_IN_REDIRECTION: crate::core::HRESULT =
    crate::core::HRESULT(-2013081589i32);
pub const E_SYNCENGINE_FOLDER_ITEM_COUNT_LIMIT_EXCEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-2013089788i32);
pub const E_SYNCENGINE_PATH_LENGTH_LIMIT_EXCEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-2013081596i32);
pub const E_SYNCENGINE_PROXY_AUTHENTICATION_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2013081593i32);
pub const E_SYNCENGINE_REMOTE_PATH_LENGTH_LIMIT_EXCEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-2013081595i32);
pub const E_SYNCENGINE_REQUEST_BLOCKED_BY_SERVICE: crate::core::HRESULT =
    crate::core::HRESULT(-2013085690i32);
pub const E_SYNCENGINE_REQUEST_BLOCKED_DUE_TO_CLIENT_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-2013085689i32);
pub const E_SYNCENGINE_SERVICE_AUTHENTICATION_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2013085693i32);
pub const E_SYNCENGINE_SERVICE_RETURNED_UNEXPECTED_SIZE: crate::core::HRESULT =
    crate::core::HRESULT(-2013085691i32);
pub const E_SYNCENGINE_STORAGE_SERVICE_BLOCKED: crate::core::HRESULT =
    crate::core::HRESULT(-2013081590i32);
pub const E_SYNCENGINE_STORAGE_SERVICE_PROVISIONING_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2013081592i32);
pub const E_SYNCENGINE_SYNC_PAUSED_BY_SERVICE: crate::core::HRESULT =
    crate::core::HRESULT(-2013089786i32);
pub const E_SYNCENGINE_UNKNOWN_SERVICE_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-2013085692i32);
pub const E_SYNCENGINE_UNSUPPORTED_FILE_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-2013089789i32);
pub const E_SYNCENGINE_UNSUPPORTED_FOLDER_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-2013081598i32);
pub const E_SYNCENGINE_UNSUPPORTED_MARKET: crate::core::HRESULT =
    crate::core::HRESULT(-2013081597i32);
pub const E_SYNCENGINE_UNSUPPORTED_REPARSE_POINT: crate::core::HRESULT =
    crate::core::HRESULT(-2013081591i32);
pub const E_UAC_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2144927150i32);
pub const E_UNEXPECTED: crate::core::HRESULT = crate::core::HRESULT(-2147418113i32);
pub const FACILTIY_MUI_ERROR_CODE: u32 = 11u32;
pub type FARPROC = ::core::option::Option<()>;
pub const FA_E_HOMEGROUP_NOT_AVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2144927198i32);
pub const FA_E_MAX_PERSISTED_ITEMS_REACHED: crate::core::HRESULT =
    crate::core::HRESULT(-2144927200i32);
pub const FDAEMON_E_CHANGEUPDATEFAILED: crate::core::HRESULT = crate::core::HRESULT(-2147215740i32);
pub const FDAEMON_E_FATALERROR: crate::core::HRESULT = crate::core::HRESULT(-2147215742i32);
pub const FDAEMON_E_LOWRESOURCE: crate::core::HRESULT = crate::core::HRESULT(-2147215743i32);
pub const FDAEMON_E_NOWORDLIST: crate::core::HRESULT = crate::core::HRESULT(-2147215737i32);
pub const FDAEMON_E_PARTITIONDELETED: crate::core::HRESULT = crate::core::HRESULT(-2147215741i32);
pub const FDAEMON_E_TOOMANYFILTEREDBLOCKS: crate::core::HRESULT =
    crate::core::HRESULT(-2147215736i32);
pub const FDAEMON_E_WORDLISTCOMMITFAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2147215738i32);
pub const FDAEMON_W_EMPTYWORDLIST: crate::core::HRESULT = crate::core::HRESULT(267909i32);
pub const FDAEMON_W_WORDLISTFULL: crate::core::HRESULT = crate::core::HRESULT(267904i32);
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
impl ::core::marker::Copy for FILETIME {}
impl ::core::clone::Clone for FILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILETIME")
            .field("dwLowDateTime", &self.dwLowDateTime)
            .field("dwHighDateTime", &self.dwHighDateTime)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FILETIME {
    fn eq(&self, other: &Self) -> bool {
        self.dwLowDateTime == other.dwLowDateTime && self.dwHighDateTime == other.dwHighDateTime
    }
}
impl ::core::cmp::Eq for FILETIME {}
impl FromIntoMemory for FILETIME {
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
pub const FILTER_E_ALREADY_OPEN: crate::core::HRESULT = crate::core::HRESULT(-2147215562i32);
pub const FILTER_E_CONTENTINDEXCORRUPT: crate::core::HRESULT = crate::core::HRESULT(-1073473740i32);
pub const FILTER_E_IN_USE: crate::core::HRESULT = crate::core::HRESULT(-2147215560i32);
pub const FILTER_E_NOT_OPEN: crate::core::HRESULT = crate::core::HRESULT(-2147215559i32);
pub const FILTER_E_NO_SUCH_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-2147215557i32);
pub const FILTER_E_OFFLINE: crate::core::HRESULT = crate::core::HRESULT(-2147215555i32);
pub const FILTER_E_PARTIALLY_FILTERED: crate::core::HRESULT = crate::core::HRESULT(-2147215554i32);
pub const FILTER_E_TOO_BIG: crate::core::HRESULT = crate::core::HRESULT(-2147215568i32);
pub const FILTER_E_UNREACHABLE: crate::core::HRESULT = crate::core::HRESULT(-2147215561i32);
pub const FILTER_S_CONTENTSCAN_DELAYED: crate::core::HRESULT = crate::core::HRESULT(268083i32);
pub const FILTER_S_DISK_FULL: crate::core::HRESULT = crate::core::HRESULT(268085i32);
pub const FILTER_S_FULL_CONTENTSCAN_IMMEDIATE: crate::core::HRESULT =
    crate::core::HRESULT(268082i32);
pub const FILTER_S_NO_PROPSETS: crate::core::HRESULT = crate::core::HRESULT(268090i32);
pub const FILTER_S_NO_SECURITY_DESCRIPTOR: crate::core::HRESULT = crate::core::HRESULT(268092i32);
pub const FILTER_S_PARTIAL_CONTENTSCAN_IMMEDIATE: crate::core::HRESULT =
    crate::core::HRESULT(268081i32);
pub struct FLOAT128 {
    pub LowPart: i64,
    pub HighPart: i64,
}
impl ::core::marker::Copy for FLOAT128 {}
impl ::core::clone::Clone for FLOAT128 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLOAT128 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLOAT128")
            .field("LowPart", &self.LowPart)
            .field("HighPart", &self.HighPart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FLOAT128 {
    fn eq(&self, other: &Self) -> bool {
        self.LowPart == other.LowPart && self.HighPart == other.HighPart
    }
}
impl ::core::cmp::Eq for FLOAT128 {}
impl FromIntoMemory for FLOAT128 {
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
pub const FRS_ERR_AUTHENTICATION: i32 = 8008i32;
pub const FRS_ERR_CHILD_TO_PARENT_COMM: i32 = 8011i32;
pub const FRS_ERR_INSUFFICIENT_PRIV: i32 = 8007i32;
pub const FRS_ERR_INTERNAL: i32 = 8005i32;
pub const FRS_ERR_INTERNAL_API: i32 = 8004i32;
pub const FRS_ERR_INVALID_API_SEQUENCE: i32 = 8001i32;
pub const FRS_ERR_INVALID_SERVICE_PARAMETER: i32 = 8017i32;
pub const FRS_ERR_PARENT_AUTHENTICATION: i32 = 8010i32;
pub const FRS_ERR_PARENT_INSUFFICIENT_PRIV: i32 = 8009i32;
pub const FRS_ERR_PARENT_TO_CHILD_COMM: i32 = 8012i32;
pub const FRS_ERR_SERVICE_COMM: i32 = 8006i32;
pub const FRS_ERR_STARTING_SERVICE: i32 = 8002i32;
pub const FRS_ERR_STOPPING_SERVICE: i32 = 8003i32;
pub const FRS_ERR_SYSVOL_DEMOTE: i32 = 8016i32;
pub const FRS_ERR_SYSVOL_IS_BUSY: i32 = 8015i32;
pub const FRS_ERR_SYSVOL_POPULATE: i32 = 8013i32;
pub const FRS_ERR_SYSVOL_POPULATE_TIMEOUT: i32 = 8014i32;
pub const FVE_E_AAD_ENDPOINT_BUSY: crate::core::HRESULT = crate::core::HRESULT(-2144272159i32);
pub const FVE_E_ACTION_NOT_ALLOWED: crate::core::HRESULT = crate::core::HRESULT(-2144272375i32);
pub const FVE_E_ADBACKUP_NOT_ENABLED: crate::core::HRESULT = crate::core::HRESULT(-2144272171i32);
pub const FVE_E_AD_ATTR_NOT_SET: crate::core::HRESULT = crate::core::HRESULT(-2144272370i32);
pub const FVE_E_AD_BACKUP_REQUIRED_POLICY_NOT_SET_FIXED_DRIVE: crate::core::HRESULT =
    crate::core::HRESULT(-2144272165i32);
pub const FVE_E_AD_BACKUP_REQUIRED_POLICY_NOT_SET_OS_DRIVE: crate::core::HRESULT =
    crate::core::HRESULT(-2144272166i32);
pub const FVE_E_AD_BACKUP_REQUIRED_POLICY_NOT_SET_REMOVABLE_DRIVE: crate::core::HRESULT =
    crate::core::HRESULT(-2144272164i32);
pub const FVE_E_AD_GUID_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144272369i32);
pub const FVE_E_AD_INSUFFICIENT_BUFFER: crate::core::HRESULT = crate::core::HRESULT(-2144272358i32);
pub const FVE_E_AD_INVALID_DATASIZE: crate::core::HRESULT = crate::core::HRESULT(-2144272372i32);
pub const FVE_E_AD_INVALID_DATATYPE: crate::core::HRESULT = crate::core::HRESULT(-2144272373i32);
pub const FVE_E_AD_NO_VALUES: crate::core::HRESULT = crate::core::HRESULT(-2144272371i32);
pub const FVE_E_AD_SCHEMA_NOT_INSTALLED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272374i32);
pub const FVE_E_AUTH_INVALID_APPLICATION: crate::core::HRESULT =
    crate::core::HRESULT(-2144272316i32);
pub const FVE_E_AUTH_INVALID_CONFIG: crate::core::HRESULT = crate::core::HRESULT(-2144272315i32);
pub const FVE_E_AUTOUNLOCK_ENABLED: crate::core::HRESULT = crate::core::HRESULT(-2144272343i32);
pub const FVE_E_BAD_DATA: crate::core::HRESULT = crate::core::HRESULT(-2144272362i32);
pub const FVE_E_BAD_INFORMATION: crate::core::HRESULT = crate::core::HRESULT(-2144272368i32);
pub const FVE_E_BAD_PARTITION_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2144272364i32);
pub const FVE_E_BCD_APPLICATIONS_PATH_INCORRECT: crate::core::HRESULT =
    crate::core::HRESULT(-2144272302i32);
pub const FVE_E_BOOTABLE_CDDVD: crate::core::HRESULT = crate::core::HRESULT(-2144272336i32);
pub const FVE_E_BUFFER_TOO_LARGE: crate::core::HRESULT = crate::core::HRESULT(-2144272177i32);
pub const FVE_E_CANNOT_ENCRYPT_NO_KEY: crate::core::HRESULT = crate::core::HRESULT(-2144272338i32);
pub const FVE_E_CANNOT_SET_FVEK_ENCRYPTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272339i32);
pub const FVE_E_CANT_LOCK_AUTOUNLOCK_ENABLED_VOLUME: crate::core::HRESULT =
    crate::core::HRESULT(-2144272233i32);
pub const FVE_E_CLUSTERING_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272354i32);
pub const FVE_E_CONV_READ: crate::core::HRESULT = crate::core::HRESULT(-2144272357i32);
pub const FVE_E_CONV_RECOVERY_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144272248i32);
pub const FVE_E_CONV_WRITE: crate::core::HRESULT = crate::core::HRESULT(-2144272356i32);
pub const FVE_E_DEBUGGER_ENABLED: crate::core::HRESULT = crate::core::HRESULT(-2144272305i32);
pub const FVE_E_DEVICELOCKOUT_COUNTER_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2144272178i32);
pub const FVE_E_DEVICE_LOCKOUT_COUNTER_UNAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2144272179i32);
pub const FVE_E_DEVICE_NOT_JOINED: crate::core::HRESULT = crate::core::HRESULT(-2144272160i32);
pub const FVE_E_DE_DEVICE_LOCKEDOUT: crate::core::HRESULT = crate::core::HRESULT(-2144272182i32);
pub const FVE_E_DE_FIXED_DATA_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272187i32);
pub const FVE_E_DE_HARDWARE_NOT_COMPLIANT: crate::core::HRESULT =
    crate::core::HRESULT(-2144272186i32);
pub const FVE_E_DE_OS_VOLUME_NOT_PROTECTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272183i32);
pub const FVE_E_DE_PREVENTED_FOR_OS: crate::core::HRESULT = crate::core::HRESULT(-2144272175i32);
pub const FVE_E_DE_PROTECTION_NOT_YET_ENABLED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272181i32);
pub const FVE_E_DE_PROTECTION_SUSPENDED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272184i32);
pub const FVE_E_DE_VOLUME_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272173i32);
pub const FVE_E_DE_VOLUME_OPTED_OUT: crate::core::HRESULT = crate::core::HRESULT(-2144272174i32);
pub const FVE_E_DE_WINRE_NOT_CONFIGURED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272185i32);
pub const FVE_E_DRY_RUN_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144272307i32);
pub const FVE_E_DV_NOT_ALLOWED_BY_GP: crate::core::HRESULT = crate::core::HRESULT(-2144272271i32);
pub const FVE_E_DV_NOT_SUPPORTED_ON_FS: crate::core::HRESULT = crate::core::HRESULT(-2144272272i32);
pub const FVE_E_EDRIVE_BAND_ENUMERATION_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272157i32);
pub const FVE_E_EDRIVE_BAND_IN_USE: crate::core::HRESULT = crate::core::HRESULT(-2144272208i32);
pub const FVE_E_EDRIVE_DISALLOWED_BY_GP: crate::core::HRESULT =
    crate::core::HRESULT(-2144272207i32);
pub const FVE_E_EDRIVE_DRY_RUN_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144272196i32);
pub const FVE_E_EDRIVE_DV_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272204i32);
pub const FVE_E_EDRIVE_INCOMPATIBLE_FIRMWARE: crate::core::HRESULT =
    crate::core::HRESULT(-2144272193i32);
pub const FVE_E_EDRIVE_INCOMPATIBLE_VOLUME: crate::core::HRESULT =
    crate::core::HRESULT(-2144272206i32);
pub const FVE_E_EDRIVE_NO_FAILOVER_TO_SW: crate::core::HRESULT =
    crate::core::HRESULT(-2144272209i32);
pub const FVE_E_EFI_ONLY: crate::core::HRESULT = crate::core::HRESULT(-2144272228i32);
pub const FVE_E_ENH_PIN_INVALID: crate::core::HRESULT = crate::core::HRESULT(-2144272231i32);
pub const FVE_E_EOW_NOT_SUPPORTED_IN_VERSION: crate::core::HRESULT =
    crate::core::HRESULT(-2144272172i32);
pub const FVE_E_EXECUTE_REQUEST_SENT_TOO_SOON: crate::core::HRESULT =
    crate::core::HRESULT(-2144272162i32);
pub const FVE_E_FAILED_AUTHENTICATION: crate::core::HRESULT = crate::core::HRESULT(-2144272345i32);
pub const FVE_E_FAILED_SECTOR_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2144272346i32);
pub const FVE_E_FAILED_WRONG_FS: crate::core::HRESULT = crate::core::HRESULT(-2144272365i32);
pub const FVE_E_FIPS_DISABLE_PROTECTION_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272314i32);
pub const FVE_E_FIPS_HASH_KDF_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272232i32);
pub const FVE_E_FIPS_PREVENTS_EXTERNAL_KEY_EXPORT: crate::core::HRESULT =
    crate::core::HRESULT(-2144272328i32);
pub const FVE_E_FIPS_PREVENTS_PASSPHRASE: crate::core::HRESULT =
    crate::core::HRESULT(-2144272276i32);
pub const FVE_E_FIPS_PREVENTS_RECOVERY_PASSWORD: crate::core::HRESULT =
    crate::core::HRESULT(-2144272329i32);
pub const FVE_E_FIPS_RNG_CHECK_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144272330i32);
pub const FVE_E_FIRMWARE_TYPE_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272312i32);
pub const FVE_E_FOREIGN_VOLUME: crate::core::HRESULT = crate::core::HRESULT(-2144272349i32);
pub const FVE_E_FS_MOUNTED: crate::core::HRESULT = crate::core::HRESULT(-2144272309i32);
pub const FVE_E_FS_NOT_EXTENDED: crate::core::HRESULT = crate::core::HRESULT(-2144272313i32);
pub const FVE_E_FULL_ENCRYPTION_NOT_ALLOWED_ON_TP_STORAGE: crate::core::HRESULT =
    crate::core::HRESULT(-2144272219i32);
pub const FVE_E_HIDDEN_VOLUME: crate::core::HRESULT = crate::core::HRESULT(-2144272298i32);
pub const FVE_E_INVALID_BITLOCKER_OID: crate::core::HRESULT = crate::core::HRESULT(-2144272274i32);
pub const FVE_E_INVALID_DATUM_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2144272229i32);
pub const FVE_E_INVALID_KEY_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-2144272332i32);
pub const FVE_E_INVALID_NBP_CERT: crate::core::HRESULT = crate::core::HRESULT(-2144272158i32);
pub const FVE_E_INVALID_NKP_CERT: crate::core::HRESULT = crate::core::HRESULT(-2144272225i32);
pub const FVE_E_INVALID_PASSWORD_FORMAT: crate::core::HRESULT =
    crate::core::HRESULT(-2144272331i32);
pub const FVE_E_INVALID_PIN_CHARS: crate::core::HRESULT = crate::core::HRESULT(-2144272230i32);
pub const FVE_E_INVALID_PIN_CHARS_DETAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272180i32);
pub const FVE_E_INVALID_PROTECTOR_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2144272326i32);
pub const FVE_E_INVALID_STARTUP_OPTIONS: crate::core::HRESULT =
    crate::core::HRESULT(-2144272293i32);
pub const FVE_E_KEYFILE_INVALID: crate::core::HRESULT = crate::core::HRESULT(-2144272323i32);
pub const FVE_E_KEYFILE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144272324i32);
pub const FVE_E_KEYFILE_NO_VMK: crate::core::HRESULT = crate::core::HRESULT(-2144272322i32);
pub const FVE_E_KEY_LENGTH_NOT_SUPPORTED_BY_EDRIVE: crate::core::HRESULT =
    crate::core::HRESULT(-2144272217i32);
pub const FVE_E_KEY_PROTECTOR_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272279i32);
pub const FVE_E_KEY_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2144272355i32);
pub const FVE_E_KEY_ROTATION_NOT_ENABLED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272161i32);
pub const FVE_E_KEY_ROTATION_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272163i32);
pub const FVE_E_LIVEID_ACCOUNT_BLOCKED: crate::core::HRESULT = crate::core::HRESULT(-2144272189i32);
pub const FVE_E_LIVEID_ACCOUNT_SUSPENDED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272190i32);
pub const FVE_E_LOCKED_VOLUME: crate::core::HRESULT = crate::core::HRESULT(-2144272384i32);
pub const FVE_E_MOR_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144272299i32);
pub const FVE_E_MULTIPLE_NKP_CERTS: crate::core::HRESULT = crate::core::HRESULT(-2144272227i32);
pub const FVE_E_NON_BITLOCKER_KU: crate::core::HRESULT = crate::core::HRESULT(-2144272237i32);
pub const FVE_E_NON_BITLOCKER_OID: crate::core::HRESULT = crate::core::HRESULT(-2144272251i32);
pub const FVE_E_NOT_ACTIVATED: crate::core::HRESULT = crate::core::HRESULT(-2144272376i32);
pub const FVE_E_NOT_ALLOWED_IN_SAFE_MODE: crate::core::HRESULT =
    crate::core::HRESULT(-2144272320i32);
pub const FVE_E_NOT_ALLOWED_IN_VERSION: crate::core::HRESULT = crate::core::HRESULT(-2144272301i32);
pub const FVE_E_NOT_ALLOWED_ON_CLUSTER: crate::core::HRESULT = crate::core::HRESULT(-2144272210i32);
pub const FVE_E_NOT_ALLOWED_ON_CSV_STACK: crate::core::HRESULT =
    crate::core::HRESULT(-2144272211i32);
pub const FVE_E_NOT_ALLOWED_TO_UPGRADE_WHILE_CONVERTING: crate::core::HRESULT =
    crate::core::HRESULT(-2144272205i32);
pub const FVE_E_NOT_DATA_VOLUME: crate::core::HRESULT = crate::core::HRESULT(-2144272359i32);
pub const FVE_E_NOT_DECRYPTED: crate::core::HRESULT = crate::core::HRESULT(-2144272327i32);
pub const FVE_E_NOT_DE_VOLUME: crate::core::HRESULT = crate::core::HRESULT(-2144272169i32);
pub const FVE_E_NOT_ENCRYPTED: crate::core::HRESULT = crate::core::HRESULT(-2144272383i32);
pub const FVE_E_NOT_ON_STACK: crate::core::HRESULT = crate::core::HRESULT(-2144272310i32);
pub const FVE_E_NOT_OS_VOLUME: crate::core::HRESULT = crate::core::HRESULT(-2144272344i32);
pub const FVE_E_NOT_PROVISIONED_ON_ALL_VOLUMES: crate::core::HRESULT =
    crate::core::HRESULT(-2144272188i32);
pub const FVE_E_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2144272363i32);
pub const FVE_E_NO_AUTOUNLOCK_MASTER_KEY: crate::core::HRESULT =
    crate::core::HRESULT(-2144272300i32);
pub const FVE_E_NO_BOOTMGR_METRIC: crate::core::HRESULT = crate::core::HRESULT(-2144272379i32);
pub const FVE_E_NO_BOOTSECTOR_METRIC: crate::core::HRESULT = crate::core::HRESULT(-2144272380i32);
pub const FVE_E_NO_EXISTING_PASSPHRASE: crate::core::HRESULT = crate::core::HRESULT(-2144272216i32);
pub const FVE_E_NO_EXISTING_PIN: crate::core::HRESULT = crate::core::HRESULT(-2144272224i32);
pub const FVE_E_NO_FEATURE_LICENSE: crate::core::HRESULT = crate::core::HRESULT(-2144272294i32);
pub const FVE_E_NO_LICENSE: crate::core::HRESULT = crate::core::HRESULT(-2144272311i32);
pub const FVE_E_NO_MBR_METRIC: crate::core::HRESULT = crate::core::HRESULT(-2144272381i32);
pub const FVE_E_NO_PASSPHRASE_WITH_TPM: crate::core::HRESULT = crate::core::HRESULT(-2144272213i32);
pub const FVE_E_NO_PREBOOT_KEYBOARD_DETECTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272203i32);
pub const FVE_E_NO_PREBOOT_KEYBOARD_OR_WINRE_DETECTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272202i32);
pub const FVE_E_NO_PROTECTORS_TO_TEST: crate::core::HRESULT = crate::core::HRESULT(-2144272325i32);
pub const FVE_E_NO_SUCH_CAPABILITY_ON_TARGET: crate::core::HRESULT =
    crate::core::HRESULT(-2144272176i32);
pub const FVE_E_NO_TPM_BIOS: crate::core::HRESULT = crate::core::HRESULT(-2144272382i32);
pub const FVE_E_NO_TPM_WITH_PASSPHRASE: crate::core::HRESULT = crate::core::HRESULT(-2144272212i32);
pub const FVE_E_OPERATION_NOT_SUPPORTED_ON_VISTA_VOLUME: crate::core::HRESULT =
    crate::core::HRESULT(-2144272234i32);
pub const FVE_E_OSV_KSR_NOT_ALLOWED: crate::core::HRESULT = crate::core::HRESULT(-2144272167i32);
pub const FVE_E_OS_NOT_PROTECTED: crate::core::HRESULT = crate::core::HRESULT(-2144272352i32);
pub const FVE_E_OS_VOLUME_PASSPHRASE_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272275i32);
pub const FVE_E_OVERLAPPED_UPDATE: crate::core::HRESULT = crate::core::HRESULT(-2144272348i32);
pub const FVE_E_PASSPHRASE_PROTECTOR_CHANGE_BY_STD_USER_DISALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272191i32);
pub const FVE_E_PASSPHRASE_TOO_LONG: crate::core::HRESULT = crate::core::HRESULT(-2144272214i32);
pub const FVE_E_PIN_INVALID: crate::core::HRESULT = crate::core::HRESULT(-2144272317i32);
pub const FVE_E_PIN_PROTECTOR_CHANGE_BY_STD_USER_DISALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272222i32);
pub const FVE_E_POLICY_CONFLICT_FDV_RK_OFF_AUK_ON: crate::core::HRESULT =
    crate::core::HRESULT(-2144272253i32);
pub const FVE_E_POLICY_CONFLICT_FDV_RP_OFF_ADB_ON: crate::core::HRESULT =
    crate::core::HRESULT(-2144272239i32);
pub const FVE_E_POLICY_CONFLICT_OSV_RP_OFF_ADB_ON: crate::core::HRESULT =
    crate::core::HRESULT(-2144272240i32);
pub const FVE_E_POLICY_CONFLICT_RDV_RK_OFF_AUK_ON: crate::core::HRESULT =
    crate::core::HRESULT(-2144272252i32);
pub const FVE_E_POLICY_CONFLICT_RDV_RP_OFF_ADB_ON: crate::core::HRESULT =
    crate::core::HRESULT(-2144272238i32);
pub const FVE_E_POLICY_CONFLICT_RO_AND_STARTUP_KEY_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272249i32);
pub const FVE_E_POLICY_INVALID_ENHANCED_BCD_SETTINGS: crate::core::HRESULT =
    crate::core::HRESULT(-2144272194i32);
pub const FVE_E_POLICY_INVALID_PASSPHRASE_LENGTH: crate::core::HRESULT =
    crate::core::HRESULT(-2144272256i32);
pub const FVE_E_POLICY_INVALID_PIN_LENGTH: crate::core::HRESULT =
    crate::core::HRESULT(-2144272280i32);
pub const FVE_E_POLICY_ON_RDV_EXCLUSION_LIST: crate::core::HRESULT =
    crate::core::HRESULT(-2144272156i32);
pub const FVE_E_POLICY_PASSPHRASE_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272278i32);
pub const FVE_E_POLICY_PASSPHRASE_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272277i32);
pub const FVE_E_POLICY_PASSPHRASE_REQUIRES_ASCII: crate::core::HRESULT =
    crate::core::HRESULT(-2144272220i32);
pub const FVE_E_POLICY_PASSPHRASE_TOO_SIMPLE: crate::core::HRESULT =
    crate::core::HRESULT(-2144272255i32);
pub const FVE_E_POLICY_PASSWORD_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272340i32);
pub const FVE_E_POLICY_PROHIBITS_SELFSIGNED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272250i32);
pub const FVE_E_POLICY_RECOVERY_KEY_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272290i32);
pub const FVE_E_POLICY_RECOVERY_KEY_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272289i32);
pub const FVE_E_POLICY_RECOVERY_PASSWORD_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272292i32);
pub const FVE_E_POLICY_RECOVERY_PASSWORD_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272291i32);
pub const FVE_E_POLICY_REQUIRES_RECOVERY_PASSWORD_ON_TOUCH_DEVICE: crate::core::HRESULT =
    crate::core::HRESULT(-2144272200i32);
pub const FVE_E_POLICY_REQUIRES_STARTUP_PIN_ON_TOUCH_DEVICE: crate::core::HRESULT =
    crate::core::HRESULT(-2144272201i32);
pub const FVE_E_POLICY_STARTUP_KEY_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272286i32);
pub const FVE_E_POLICY_STARTUP_KEY_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272285i32);
pub const FVE_E_POLICY_STARTUP_PIN_KEY_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272284i32);
pub const FVE_E_POLICY_STARTUP_PIN_KEY_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272283i32);
pub const FVE_E_POLICY_STARTUP_PIN_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272288i32);
pub const FVE_E_POLICY_STARTUP_PIN_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272287i32);
pub const FVE_E_POLICY_STARTUP_TPM_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272282i32);
pub const FVE_E_POLICY_STARTUP_TPM_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272281i32);
pub const FVE_E_POLICY_USER_CERTIFICATE_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272270i32);
pub const FVE_E_POLICY_USER_CERTIFICATE_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272269i32);
pub const FVE_E_POLICY_USER_CERT_MUST_BE_HW: crate::core::HRESULT =
    crate::core::HRESULT(-2144272268i32);
pub const FVE_E_POLICY_USER_CONFIGURE_FDV_AUTOUNLOCK_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272267i32);
pub const FVE_E_POLICY_USER_CONFIGURE_RDV_AUTOUNLOCK_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272266i32);
pub const FVE_E_POLICY_USER_CONFIGURE_RDV_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272265i32);
pub const FVE_E_POLICY_USER_DISABLE_RDV_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272263i32);
pub const FVE_E_POLICY_USER_ENABLE_RDV_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272264i32);
pub const FVE_E_PREDICTED_TPM_PROTECTOR_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272155i32);
pub const FVE_E_PRIVATEKEY_AUTH_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144272236i32);
pub const FVE_E_PROTECTION_CANNOT_BE_DISABLED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272168i32);
pub const FVE_E_PROTECTION_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2144272351i32);
pub const FVE_E_PROTECTOR_CHANGE_MAX_PASSPHRASE_CHANGE_ATTEMPTS_REACHED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272192i32);
pub const FVE_E_PROTECTOR_CHANGE_MAX_PIN_CHANGE_ATTEMPTS_REACHED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272221i32);
pub const FVE_E_PROTECTOR_CHANGE_PASSPHRASE_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2144272215i32);
pub const FVE_E_PROTECTOR_CHANGE_PIN_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2144272223i32);
pub const FVE_E_PROTECTOR_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2144272335i32);
pub const FVE_E_PROTECTOR_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144272333i32);
pub const FVE_E_PUBKEY_NOT_ALLOWED: crate::core::HRESULT = crate::core::HRESULT(-2144272296i32);
pub const FVE_E_RAW_ACCESS: crate::core::HRESULT = crate::core::HRESULT(-2144272304i32);
pub const FVE_E_RAW_BLOCKED: crate::core::HRESULT = crate::core::HRESULT(-2144272303i32);
pub const FVE_E_REBOOT_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2144272306i32);
pub const FVE_E_RECOVERY_KEY_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2144272350i32);
pub const FVE_E_RECOVERY_PARTITION: crate::core::HRESULT = crate::core::HRESULT(-2144272254i32);
pub const FVE_E_RELATIVE_PATH: crate::core::HRESULT = crate::core::HRESULT(-2144272334i32);
pub const FVE_E_REMOVAL_OF_DRA_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144272235i32);
pub const FVE_E_REMOVAL_OF_NKP_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144272226i32);
pub const FVE_E_SECUREBOOT_CONFIGURATION_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2144272197i32);
pub const FVE_E_SECUREBOOT_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2144272198i32);
pub const FVE_E_SECURE_KEY_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2144272377i32);
pub const FVE_E_SETUP_TPM_CALLBACK_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272154i32);
pub const FVE_E_SHADOW_COPY_PRESENT: crate::core::HRESULT = crate::core::HRESULT(-2144272195i32);
pub const FVE_E_SYSTEM_VOLUME: crate::core::HRESULT = crate::core::HRESULT(-2144272366i32);
pub const FVE_E_TOKEN_NOT_IMPERSONATED: crate::core::HRESULT = crate::core::HRESULT(-2144272308i32);
pub const FVE_E_TOO_SMALL: crate::core::HRESULT = crate::core::HRESULT(-2144272367i32);
pub const FVE_E_TPM_CONTEXT_SETUP_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144272153i32);
pub const FVE_E_TPM_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2144272321i32);
pub const FVE_E_TPM_INVALID_PCR: crate::core::HRESULT = crate::core::HRESULT(-2144272319i32);
pub const FVE_E_TPM_NOT_OWNED: crate::core::HRESULT = crate::core::HRESULT(-2144272360i32);
pub const FVE_E_TPM_NO_VMK: crate::core::HRESULT = crate::core::HRESULT(-2144272318i32);
pub const FVE_E_TPM_SRK_AUTH_NOT_ZERO: crate::core::HRESULT = crate::core::HRESULT(-2144272347i32);
pub const FVE_E_TRANSIENT_STATE: crate::core::HRESULT = crate::core::HRESULT(-2144272297i32);
pub const FVE_E_VIRTUALIZED_SPACE_TOO_BIG: crate::core::HRESULT =
    crate::core::HRESULT(-2144272247i32);
pub const FVE_E_VOLUME_BOUND_ALREADY: crate::core::HRESULT = crate::core::HRESULT(-2144272353i32);
pub const FVE_E_VOLUME_EXTEND_PREVENTS_EOW_DECRYPT: crate::core::HRESULT =
    crate::core::HRESULT(-2144272170i32);
pub const FVE_E_VOLUME_HANDLE_OPEN: crate::core::HRESULT = crate::core::HRESULT(-2144272295i32);
pub const FVE_E_VOLUME_NOT_BOUND: crate::core::HRESULT = crate::core::HRESULT(-2144272361i32);
pub const FVE_E_VOLUME_TOO_SMALL: crate::core::HRESULT = crate::core::HRESULT(-2144272273i32);
pub const FVE_E_WIPE_CANCEL_NOT_APPLICABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2144272199i32);
pub const FVE_E_WIPE_NOT_ALLOWED_ON_TP_STORAGE: crate::core::HRESULT =
    crate::core::HRESULT(-2144272218i32);
pub const FVE_E_WRONG_BOOTMGR: crate::core::HRESULT = crate::core::HRESULT(-2144272378i32);
pub const FVE_E_WRONG_BOOTSECTOR: crate::core::HRESULT = crate::core::HRESULT(-2144272342i32);
pub const FVE_E_WRONG_SYSTEM_FS: crate::core::HRESULT = crate::core::HRESULT(-2144272341i32);
pub const FWP_E_ACTION_INCOMPATIBLE_WITH_LAYER: crate::core::HRESULT =
    crate::core::HRESULT(-2144206804i32);
pub const FWP_E_ACTION_INCOMPATIBLE_WITH_SUBLAYER: crate::core::HRESULT =
    crate::core::HRESULT(-2144206803i32);
pub const FWP_E_ALREADY_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2144206839i32);
pub const FWP_E_BUILTIN_OBJECT: crate::core::HRESULT = crate::core::HRESULT(-2144206825i32);
pub const FWP_E_CALLOUT_NOTIFICATION_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2144206793i32);
pub const FWP_E_CALLOUT_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144206847i32);
pub const FWP_E_CONDITION_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144206846i32);
pub const FWP_E_CONNECTIONS_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2144206783i32);
pub const FWP_E_CONTEXT_INCOMPATIBLE_WITH_CALLOUT: crate::core::HRESULT =
    crate::core::HRESULT(-2144206801i32);
pub const FWP_E_CONTEXT_INCOMPATIBLE_WITH_LAYER: crate::core::HRESULT =
    crate::core::HRESULT(-2144206802i32);
pub const FWP_E_DROP_NOICMP: crate::core::HRESULT = crate::core::HRESULT(-2144206588i32);
pub const FWP_E_DUPLICATE_AUTH_METHOD: crate::core::HRESULT = crate::core::HRESULT(-2144206788i32);
pub const FWP_E_DUPLICATE_CONDITION: crate::core::HRESULT = crate::core::HRESULT(-2144206806i32);
pub const FWP_E_DUPLICATE_KEYMOD: crate::core::HRESULT = crate::core::HRESULT(-2144206805i32);
pub const FWP_E_DYNAMIC_SESSION_IN_PROGRESS: crate::core::HRESULT =
    crate::core::HRESULT(-2144206837i32);
pub const FWP_E_EM_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2144206798i32);
pub const FWP_E_FILTER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144206845i32);
pub const FWP_E_IKEEXT_NOT_RUNNING: crate::core::HRESULT = crate::core::HRESULT(-2144206780i32);
pub const FWP_E_INCOMPATIBLE_AUTH_METHOD: crate::core::HRESULT =
    crate::core::HRESULT(-2144206800i32);
pub const FWP_E_INCOMPATIBLE_CIPHER_TRANSFORM: crate::core::HRESULT =
    crate::core::HRESULT(-2144206790i32);
pub const FWP_E_INCOMPATIBLE_DH_GROUP: crate::core::HRESULT = crate::core::HRESULT(-2144206799i32);
pub const FWP_E_INCOMPATIBLE_LAYER: crate::core::HRESULT = crate::core::HRESULT(-2144206828i32);
pub const FWP_E_INCOMPATIBLE_SA_STATE: crate::core::HRESULT = crate::core::HRESULT(-2144206821i32);
pub const FWP_E_INCOMPATIBLE_TXN: crate::core::HRESULT = crate::core::HRESULT(-2144206831i32);
pub const FWP_E_INVALID_ACTION_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2144206812i32);
pub const FWP_E_INVALID_AUTH_TRANSFORM: crate::core::HRESULT = crate::core::HRESULT(-2144206792i32);
pub const FWP_E_INVALID_CIPHER_TRANSFORM: crate::core::HRESULT =
    crate::core::HRESULT(-2144206791i32);
pub const FWP_E_INVALID_DNS_NAME: crate::core::HRESULT = crate::core::HRESULT(-2144206782i32);
pub const FWP_E_INVALID_ENUMERATOR: crate::core::HRESULT = crate::core::HRESULT(-2144206819i32);
pub const FWP_E_INVALID_FLAGS: crate::core::HRESULT = crate::core::HRESULT(-2144206818i32);
pub const FWP_E_INVALID_INTERVAL: crate::core::HRESULT = crate::core::HRESULT(-2144206815i32);
pub const FWP_E_INVALID_NET_MASK: crate::core::HRESULT = crate::core::HRESULT(-2144206817i32);
pub const FWP_E_INVALID_PARAMETER: crate::core::HRESULT = crate::core::HRESULT(-2144206795i32);
pub const FWP_E_INVALID_RANGE: crate::core::HRESULT = crate::core::HRESULT(-2144206816i32);
pub const FWP_E_INVALID_TRANSFORM_COMBINATION: crate::core::HRESULT =
    crate::core::HRESULT(-2144206789i32);
pub const FWP_E_INVALID_TUNNEL_ENDPOINT: crate::core::HRESULT =
    crate::core::HRESULT(-2144206787i32);
pub const FWP_E_INVALID_WEIGHT: crate::core::HRESULT = crate::core::HRESULT(-2144206811i32);
pub const FWP_E_IN_USE: crate::core::HRESULT = crate::core::HRESULT(-2144206838i32);
pub const FWP_E_KEY_DICTATION_INVALID_KEYING_MATERIAL: crate::core::HRESULT =
    crate::core::HRESULT(-2144206784i32);
pub const FWP_E_KEY_DICTATOR_ALREADY_REGISTERED: crate::core::HRESULT =
    crate::core::HRESULT(-2144206785i32);
pub const FWP_E_KM_CLIENTS_ONLY: crate::core::HRESULT = crate::core::HRESULT(-2144206827i32);
pub const FWP_E_L2_DRIVER_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-2144206786i32);
pub const FWP_E_LAYER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144206844i32);
pub const FWP_E_LIFETIME_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2144206826i32);
pub const FWP_E_MATCH_TYPE_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2144206810i32);
pub const FWP_E_NET_EVENTS_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2144206829i32);
pub const FWP_E_NEVER_MATCH: crate::core::HRESULT = crate::core::HRESULT(-2144206797i32);
pub const FWP_E_NOTIFICATION_DROPPED: crate::core::HRESULT = crate::core::HRESULT(-2144206823i32);
pub const FWP_E_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144206840i32);
pub const FWP_E_NO_TXN_IN_PROGRESS: crate::core::HRESULT = crate::core::HRESULT(-2144206835i32);
pub const FWP_E_NULL_DISPLAY_NAME: crate::core::HRESULT = crate::core::HRESULT(-2144206813i32);
pub const FWP_E_NULL_POINTER: crate::core::HRESULT = crate::core::HRESULT(-2144206820i32);
pub const FWP_E_OUT_OF_BOUNDS: crate::core::HRESULT = crate::core::HRESULT(-2144206808i32);
pub const FWP_E_PROVIDER_CONTEXT_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2144206796i32);
pub const FWP_E_PROVIDER_CONTEXT_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2144206842i32);
pub const FWP_E_PROVIDER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144206843i32);
pub const FWP_E_RESERVED: crate::core::HRESULT = crate::core::HRESULT(-2144206807i32);
pub const FWP_E_SESSION_ABORTED: crate::core::HRESULT = crate::core::HRESULT(-2144206832i32);
pub const FWP_E_STILL_ON: crate::core::HRESULT = crate::core::HRESULT(-2144206781i32);
pub const FWP_E_SUBLAYER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144206841i32);
pub const FWP_E_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2144206830i32);
pub const FWP_E_TOO_MANY_CALLOUTS: crate::core::HRESULT = crate::core::HRESULT(-2144206824i32);
pub const FWP_E_TOO_MANY_SUBLAYERS: crate::core::HRESULT = crate::core::HRESULT(-2144206794i32);
pub const FWP_E_TRAFFIC_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2144206822i32);
pub const FWP_E_TXN_ABORTED: crate::core::HRESULT = crate::core::HRESULT(-2144206833i32);
pub const FWP_E_TXN_IN_PROGRESS: crate::core::HRESULT = crate::core::HRESULT(-2144206834i32);
pub const FWP_E_TYPE_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2144206809i32);
pub const FWP_E_WRONG_SESSION: crate::core::HRESULT = crate::core::HRESULT(-2144206836i32);
pub const FWP_E_ZERO_LENGTH_ARRAY: crate::core::HRESULT = crate::core::HRESULT(-2144206814i32);
pub const GCN_E_DEFAULTNAMESPACE_EXISTS: crate::core::HRESULT =
    crate::core::HRESULT(-2143616983i32);
pub const GCN_E_MODULE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2143616991i32);
pub const GCN_E_NETADAPTER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2143616986i32);
pub const GCN_E_NETADAPTER_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2143616987i32);
pub const GCN_E_NETCOMPARTMENT_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2143616985i32);
pub const GCN_E_NETINTERFACE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2143616984i32);
pub const GCN_E_NO_REQUEST_HANDLERS: crate::core::HRESULT = crate::core::HRESULT(-2143616990i32);
pub const GCN_E_REQUEST_UNSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2143616989i32);
pub const GCN_E_RUNTIMEKEYS_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2143616988i32);
pub struct HANDLE(pub PtrRepr);
impl ::core::default::Default for HANDLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::clone::Clone for HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HANDLE {}
impl ::core::cmp::PartialEq for HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HANDLE {}
impl ::core::fmt::Debug for HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HANDLE").field(&self.0).finish()
    }
}
impl FromIntoMemory for HANDLE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrRepr>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HANDLE_FLAGS(pub u32);
pub const HANDLE_FLAG_INHERIT: HANDLE_FLAGS = HANDLE_FLAGS(1u32);
pub const HANDLE_FLAG_PROTECT_FROM_CLOSE: HANDLE_FLAGS = HANDLE_FLAGS(2u32);
impl ::core::marker::Copy for HANDLE_FLAGS {}
impl ::core::clone::Clone for HANDLE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HANDLE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HANDLE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HANDLE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HANDLE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HANDLE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HANDLE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HANDLE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HANDLE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for HANDLE_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<u32>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HANDLE_PTR(pub PtrRepr);
impl HANDLE_PTR {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HANDLE_PTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HANDLE_PTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HANDLE_PTR {}
impl ::core::fmt::Debug for HANDLE_PTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HANDLE_PTR").field(&self.0).finish()
    }
}
impl FromIntoMemory for HANDLE_PTR {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrRepr>()
    }
}
pub const HCN_E_ADAPTER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2143617018i32);
pub const HCN_E_ADDR_INVALID_OR_RESERVED: crate::core::HRESULT =
    crate::core::HRESULT(-2143616977i32);
pub const HCN_E_DEGRADED_OPERATION: crate::core::HRESULT = crate::core::HRESULT(-2143617001i32);
pub const HCN_E_ENDPOINT_ALREADY_ATTACHED: crate::core::HRESULT =
    crate::core::HRESULT(-2143617004i32);
pub const HCN_E_ENDPOINT_NAMESPACE_ALREADY_EXISTS: crate::core::HRESULT =
    crate::core::HRESULT(-2143616981i32);
pub const HCN_E_ENDPOINT_NOT_ATTACHED: crate::core::HRESULT = crate::core::HRESULT(-2143616972i32);
pub const HCN_E_ENDPOINT_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2143617022i32);
pub const HCN_E_ENDPOINT_NOT_LOCAL: crate::core::HRESULT = crate::core::HRESULT(-2143616971i32);
pub const HCN_E_ENDPOINT_SHARING_DISABLED: crate::core::HRESULT =
    crate::core::HRESULT(-2143616995i32);
pub const HCN_E_ENTITY_HAS_REFERENCES: crate::core::HRESULT = crate::core::HRESULT(-2143616980i32);
pub const HCN_E_GUID_CONVERSION_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-2143616999i32);
pub const HCN_E_ICS_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2143616982i32);
pub const HCN_E_INVALID_ENDPOINT: crate::core::HRESULT = crate::core::HRESULT(-2143617012i32);
pub const HCN_E_INVALID_INTERNAL_PORT: crate::core::HRESULT = crate::core::HRESULT(-2143616979i32);
pub const HCN_E_INVALID_IP: crate::core::HRESULT = crate::core::HRESULT(-2143616994i32);
pub const HCN_E_INVALID_IP_SUBNET: crate::core::HRESULT = crate::core::HRESULT(-2143616973i32);
pub const HCN_E_INVALID_JSON: crate::core::HRESULT = crate::core::HRESULT(-2143616997i32);
pub const HCN_E_INVALID_JSON_REFERENCE: crate::core::HRESULT = crate::core::HRESULT(-2143616996i32);
pub const HCN_E_INVALID_NETWORK: crate::core::HRESULT = crate::core::HRESULT(-2143617014i32);
pub const HCN_E_INVALID_NETWORK_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2143617013i32);
pub const HCN_E_INVALID_POLICY: crate::core::HRESULT = crate::core::HRESULT(-2143617011i32);
pub const HCN_E_INVALID_POLICY_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2143617010i32);
pub const HCN_E_INVALID_PREFIX: crate::core::HRESULT = crate::core::HRESULT(-2143616976i32);
pub const HCN_E_INVALID_REMOTE_ENDPOINT_OPERATION: crate::core::HRESULT =
    crate::core::HRESULT(-2143617009i32);
pub const HCN_E_INVALID_SUBNET: crate::core::HRESULT = crate::core::HRESULT(-2143616974i32);
pub const HCN_E_LAYER_ALREADY_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2143617007i32);
pub const HCN_E_LAYER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2143617021i32);
pub const HCN_E_MANAGER_STOPPED: crate::core::HRESULT = crate::core::HRESULT(-2143616992i32);
pub const HCN_E_MAPPING_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2143617002i32);
pub const HCN_E_NAMESPACE_ATTACH_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2143616978i32);
pub const HCN_E_NETWORK_ALREADY_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2143617008i32);
pub const HCN_E_NETWORK_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2143617023i32);
pub const HCN_E_OBJECT_USED_AFTER_UNLOAD: crate::core::HRESULT =
    crate::core::HRESULT(-2143616975i32);
pub const HCN_E_POLICY_ALREADY_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2143617006i32);
pub const HCN_E_POLICY_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2143617016i32);
pub const HCN_E_PORT_ALREADY_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2143617005i32);
pub const HCN_E_PORT_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2143617017i32);
pub const HCN_E_REGKEY_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2143616998i32);
pub const HCN_E_REQUEST_UNSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2143617003i32);
pub const HCN_E_SHARED_SWITCH_MODIFICATION: crate::core::HRESULT =
    crate::core::HRESULT(-2143617000i32);
pub const HCN_E_SUBNET_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2143617019i32);
pub const HCN_E_SWITCH_EXTENSION_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2143616993i32);
pub const HCN_E_SWITCH_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2143617020i32);
pub const HCN_E_VFP_NOT_ALLOWED: crate::core::HRESULT = crate::core::HRESULT(-2143616969i32);
pub const HCN_E_VFP_PORTSETTING_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2143617015i32);
pub const HCN_INTERFACEPARAMETERS_ALREADY_APPLIED: crate::core::HRESULT =
    crate::core::HRESULT(-2143616970i32);
pub const HCS_E_ACCESS_DENIED: crate::core::HRESULT = crate::core::HRESULT(-2143878885i32);
pub const HCS_E_CONNECTION_CLOSED: crate::core::HRESULT = crate::core::HRESULT(-2143878902i32);
pub const HCS_E_CONNECTION_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2143878903i32);
pub const HCS_E_CONNECT_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2143878904i32);
pub const HCS_E_GUEST_CRITICAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2143878884i32);
pub const HCS_E_HYPERV_NOT_INSTALLED: crate::core::HRESULT = crate::core::HRESULT(-2143878910i32);
pub const HCS_E_IMAGE_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2143878911i32);
pub const HCS_E_INVALID_JSON: crate::core::HRESULT = crate::core::HRESULT(-2143878899i32);
pub const HCS_E_INVALID_LAYER: crate::core::HRESULT = crate::core::HRESULT(-2143878894i32);
pub const HCS_E_INVALID_STATE: crate::core::HRESULT = crate::core::HRESULT(-2143878907i32);
pub const HCS_E_OPERATION_ALREADY_STARTED: crate::core::HRESULT =
    crate::core::HRESULT(-2143878890i32);
pub const HCS_E_OPERATION_NOT_STARTED: crate::core::HRESULT = crate::core::HRESULT(-2143878891i32);
pub const HCS_E_OPERATION_PENDING: crate::core::HRESULT = crate::core::HRESULT(-2143878889i32);
pub const HCS_E_OPERATION_RESULT_ALLOCATION_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2143878886i32);
pub const HCS_E_OPERATION_SYSTEM_CALLBACK_ALREADY_SET: crate::core::HRESULT =
    crate::core::HRESULT(-2143878887i32);
pub const HCS_E_OPERATION_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2143878888i32);
pub const HCS_E_PROCESS_ALREADY_STOPPED: crate::core::HRESULT =
    crate::core::HRESULT(-2143878881i32);
pub const HCS_E_PROCESS_INFO_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2143878883i32);
pub const HCS_E_PROTOCOL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2143878895i32);
pub const HCS_E_SERVICE_DISCONNECT: crate::core::HRESULT = crate::core::HRESULT(-2143878882i32);
pub const HCS_E_SERVICE_NOT_AVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2143878892i32);
pub const HCS_E_SYSTEM_ALREADY_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2143878897i32);
pub const HCS_E_SYSTEM_ALREADY_STOPPED: crate::core::HRESULT = crate::core::HRESULT(-2143878896i32);
pub const HCS_E_SYSTEM_NOT_CONFIGURED_FOR_OPERATION: crate::core::HRESULT =
    crate::core::HRESULT(-2143878880i32);
pub const HCS_E_SYSTEM_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2143878898i32);
pub const HCS_E_TERMINATED: crate::core::HRESULT = crate::core::HRESULT(-2143878905i32);
pub const HCS_E_TERMINATED_DURING_START: crate::core::HRESULT =
    crate::core::HRESULT(-2143878912i32);
pub const HCS_E_UNEXPECTED_EXIT: crate::core::HRESULT = crate::core::HRESULT(-2143878906i32);
pub const HCS_E_UNKNOWN_MESSAGE: crate::core::HRESULT = crate::core::HRESULT(-2143878901i32);
pub const HCS_E_UNSUPPORTED_PROTOCOL_VERSION: crate::core::HRESULT =
    crate::core::HRESULT(-2143878900i32);
pub const HCS_E_WINDOWS_INSIDER_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2143878893i32);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HINSTANCE(pub PtrDiffRepr);
impl HINSTANCE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HINSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HINSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HINSTANCE {}
impl ::core::fmt::Debug for HINSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HINSTANCE").field(&self.0).finish()
    }
}
impl FromIntoMemory for HINSTANCE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrDiffRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrDiffRepr>()
    }
}
pub struct HLSURF__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HLSURF__ {}
impl ::core::clone::Clone for HLSURF__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HLSURF__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HLSURF__")
            .field("unused", &self.unused)
            .finish()
    }
}
impl ::core::cmp::PartialEq for HLSURF__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HLSURF__ {}
impl FromIntoMemory for HLSURF__ {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HRSRC(pub PtrDiffRepr);
impl HRSRC {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HRSRC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HRSRC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HRSRC {}
impl ::core::fmt::Debug for HRSRC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HRSRC").field(&self.0).finish()
    }
}
impl FromIntoMemory for HRSRC {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrDiffRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrDiffRepr>()
    }
}
pub struct HSPRITE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HSPRITE__ {}
impl ::core::clone::Clone for HSPRITE__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HSPRITE__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSPRITE__")
            .field("unused", &self.unused)
            .finish()
    }
}
impl ::core::cmp::PartialEq for HSPRITE__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HSPRITE__ {}
impl FromIntoMemory for HSPRITE__ {
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
pub const HSP_BASE_ERROR_MASK: crate::core::HRESULT = crate::core::HRESULT(-2128019200i32);
pub const HSP_BASE_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2128018945i32);
pub const HSP_BS_ERROR_MASK: crate::core::HRESULT = crate::core::HRESULT(-2128080896i32);
pub const HSP_BS_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2128080641i32);
pub const HSP_DRV_ERROR_MASK: crate::core::HRESULT = crate::core::HRESULT(-2128019456i32);
pub const HSP_DRV_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2128019201i32);
pub const HSP_E_ERROR_MASK: crate::core::HRESULT = crate::core::HRESULT(-2128084992i32);
pub const HSP_E_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2128080897i32);
pub const HSP_KSP_ALGORITHM_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2128018935i32);
pub const HSP_KSP_BUFFER_TOO_SMALL: crate::core::HRESULT = crate::core::HRESULT(-2128018939i32);
pub const HSP_KSP_DEVICE_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-2128018943i32);
pub const HSP_KSP_ERROR_MASK: crate::core::HRESULT = crate::core::HRESULT(-2128018944i32);
pub const HSP_KSP_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2128018689i32);
pub const HSP_KSP_INVALID_DATA: crate::core::HRESULT = crate::core::HRESULT(-2128018937i32);
pub const HSP_KSP_INVALID_FLAGS: crate::core::HRESULT = crate::core::HRESULT(-2128018936i32);
pub const HSP_KSP_INVALID_KEY_HANDLE: crate::core::HRESULT = crate::core::HRESULT(-2128018941i32);
pub const HSP_KSP_INVALID_KEY_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2128018932i32);
pub const HSP_KSP_INVALID_PARAMETER: crate::core::HRESULT = crate::core::HRESULT(-2128018940i32);
pub const HSP_KSP_INVALID_PROVIDER_HANDLE: crate::core::HRESULT =
    crate::core::HRESULT(-2128018942i32);
pub const HSP_KSP_KEY_ALREADY_FINALIZED: crate::core::HRESULT =
    crate::core::HRESULT(-2128018934i32);
pub const HSP_KSP_KEY_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2128018923i32);
pub const HSP_KSP_KEY_LOAD_FAIL: crate::core::HRESULT = crate::core::HRESULT(-2128018921i32);
pub const HSP_KSP_KEY_MISSING: crate::core::HRESULT = crate::core::HRESULT(-2128018922i32);
pub const HSP_KSP_KEY_NOT_FINALIZED: crate::core::HRESULT = crate::core::HRESULT(-2128018933i32);
pub const HSP_KSP_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2128018938i32);
pub const HSP_KSP_NO_MEMORY: crate::core::HRESULT = crate::core::HRESULT(-2128018928i32);
pub const HSP_KSP_NO_MORE_ITEMS: crate::core::HRESULT = crate::core::HRESULT(-2128018920i32);
pub const HSP_KSP_PARAMETER_NOT_SET: crate::core::HRESULT = crate::core::HRESULT(-2128018927i32);
pub struct HSTR__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HSTR__ {}
impl ::core::clone::Clone for HSTR__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HSTR__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSTR__")
            .field("unused", &self.unused)
            .finish()
    }
}
impl ::core::cmp::PartialEq for HSTR__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HSTR__ {}
impl FromIntoMemory for HSTR__ {
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
pub const HTTP_E_STATUS_AMBIGUOUS: crate::core::HRESULT = crate::core::HRESULT(-2145844948i32);
pub const HTTP_E_STATUS_BAD_GATEWAY: crate::core::HRESULT = crate::core::HRESULT(-2145844746i32);
pub const HTTP_E_STATUS_BAD_METHOD: crate::core::HRESULT = crate::core::HRESULT(-2145844843i32);
pub const HTTP_E_STATUS_BAD_REQUEST: crate::core::HRESULT = crate::core::HRESULT(-2145844848i32);
pub const HTTP_E_STATUS_CONFLICT: crate::core::HRESULT = crate::core::HRESULT(-2145844839i32);
pub const HTTP_E_STATUS_DENIED: crate::core::HRESULT = crate::core::HRESULT(-2145844847i32);
pub const HTTP_E_STATUS_EXPECTATION_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2145844831i32);
pub const HTTP_E_STATUS_FORBIDDEN: crate::core::HRESULT = crate::core::HRESULT(-2145844845i32);
pub const HTTP_E_STATUS_GATEWAY_TIMEOUT: crate::core::HRESULT =
    crate::core::HRESULT(-2145844744i32);
pub const HTTP_E_STATUS_GONE: crate::core::HRESULT = crate::core::HRESULT(-2145844838i32);
pub const HTTP_E_STATUS_LENGTH_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2145844837i32);
pub const HTTP_E_STATUS_MOVED: crate::core::HRESULT = crate::core::HRESULT(-2145844947i32);
pub const HTTP_E_STATUS_NONE_ACCEPTABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2145844842i32);
pub const HTTP_E_STATUS_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2145844844i32);
pub const HTTP_E_STATUS_NOT_MODIFIED: crate::core::HRESULT = crate::core::HRESULT(-2145844944i32);
pub const HTTP_E_STATUS_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2145844747i32);
pub const HTTP_E_STATUS_PAYMENT_REQ: crate::core::HRESULT = crate::core::HRESULT(-2145844846i32);
pub const HTTP_E_STATUS_PRECOND_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2145844836i32);
pub const HTTP_E_STATUS_PROXY_AUTH_REQ: crate::core::HRESULT = crate::core::HRESULT(-2145844841i32);
pub const HTTP_E_STATUS_RANGE_NOT_SATISFIABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2145844832i32);
pub const HTTP_E_STATUS_REDIRECT: crate::core::HRESULT = crate::core::HRESULT(-2145844946i32);
pub const HTTP_E_STATUS_REDIRECT_KEEP_VERB: crate::core::HRESULT =
    crate::core::HRESULT(-2145844941i32);
pub const HTTP_E_STATUS_REDIRECT_METHOD: crate::core::HRESULT =
    crate::core::HRESULT(-2145844945i32);
pub const HTTP_E_STATUS_REQUEST_TIMEOUT: crate::core::HRESULT =
    crate::core::HRESULT(-2145844840i32);
pub const HTTP_E_STATUS_REQUEST_TOO_LARGE: crate::core::HRESULT =
    crate::core::HRESULT(-2145844835i32);
pub const HTTP_E_STATUS_SERVER_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2145844748i32);
pub const HTTP_E_STATUS_SERVICE_UNAVAIL: crate::core::HRESULT =
    crate::core::HRESULT(-2145844745i32);
pub const HTTP_E_STATUS_UNEXPECTED: crate::core::HRESULT = crate::core::HRESULT(-2145845247i32);
pub const HTTP_E_STATUS_UNEXPECTED_CLIENT_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-2145845244i32);
pub const HTTP_E_STATUS_UNEXPECTED_REDIRECTION: crate::core::HRESULT =
    crate::core::HRESULT(-2145845245i32);
pub const HTTP_E_STATUS_UNEXPECTED_SERVER_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-2145845243i32);
pub const HTTP_E_STATUS_UNSUPPORTED_MEDIA: crate::core::HRESULT =
    crate::core::HRESULT(-2145844833i32);
pub const HTTP_E_STATUS_URI_TOO_LONG: crate::core::HRESULT = crate::core::HRESULT(-2145844834i32);
pub const HTTP_E_STATUS_USE_PROXY: crate::core::HRESULT = crate::core::HRESULT(-2145844943i32);
pub const HTTP_E_STATUS_VERSION_NOT_SUP: crate::core::HRESULT =
    crate::core::HRESULT(-2145844743i32);
pub struct HUMPD__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HUMPD__ {}
impl ::core::clone::Clone for HUMPD__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HUMPD__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HUMPD__")
            .field("unused", &self.unused)
            .finish()
    }
}
impl ::core::cmp::PartialEq for HUMPD__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HUMPD__ {}
impl FromIntoMemory for HUMPD__ {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HWND(pub PtrDiffRepr);
impl HWND {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HWND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWND {}
impl ::core::fmt::Debug for HWND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWND").field(&self.0).finish()
    }
}
impl FromIntoMemory for HWND {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrDiffRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrDiffRepr>()
    }
}
pub const INPLACE_E_FIRST: i32 = -2147221088i32;
pub const INPLACE_E_LAST: i32 = -2147221073i32;
pub const INPLACE_E_NOTOOLSPACE: crate::core::HRESULT = crate::core::HRESULT(-2147221087i32);
pub const INPLACE_E_NOTUNDOABLE: crate::core::HRESULT = crate::core::HRESULT(-2147221088i32);
pub const INPLACE_S_FIRST: i32 = 262560i32;
pub const INPLACE_S_LAST: i32 = 262575i32;
pub const INPLACE_S_TRUNCATED: crate::core::HRESULT = crate::core::HRESULT(262560i32);
pub const INPUT_E_DEVICE_INFO: crate::core::HRESULT = crate::core::HRESULT(-2143289338i32);
pub const INPUT_E_DEVICE_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-2143289336i32);
pub const INPUT_E_FRAME: crate::core::HRESULT = crate::core::HRESULT(-2143289340i32);
pub const INPUT_E_HISTORY: crate::core::HRESULT = crate::core::HRESULT(-2143289339i32);
pub const INPUT_E_MULTIMODAL: crate::core::HRESULT = crate::core::HRESULT(-2143289342i32);
pub const INPUT_E_OUT_OF_ORDER: crate::core::HRESULT = crate::core::HRESULT(-2143289344i32);
pub const INPUT_E_PACKET: crate::core::HRESULT = crate::core::HRESULT(-2143289341i32);
pub const INPUT_E_REENTRANCY: crate::core::HRESULT = crate::core::HRESULT(-2143289343i32);
pub const INPUT_E_TRANSFORM: crate::core::HRESULT = crate::core::HRESULT(-2143289337i32);
pub const INVALID_HANDLE_VALUE: HANDLE = HANDLE(-1i32 as _);
pub const IORING_E_COMPLETION_QUEUE_TOO_BIG: crate::core::HRESULT =
    crate::core::HRESULT(-2142896123i32);
pub const IORING_E_CORRUPT: crate::core::HRESULT = crate::core::HRESULT(-2142896121i32);
pub const IORING_E_REQUIRED_FLAG_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2142896127i32);
pub const IORING_E_SUBMISSION_QUEUE_FULL: crate::core::HRESULT =
    crate::core::HRESULT(-2142896126i32);
pub const IORING_E_SUBMISSION_QUEUE_TOO_BIG: crate::core::HRESULT =
    crate::core::HRESULT(-2142896124i32);
pub const IORING_E_SUBMIT_IN_PROGRESS: crate::core::HRESULT = crate::core::HRESULT(-2142896122i32);
pub const IORING_E_VERSION_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2142896125i32);
pub const JSCRIPT_E_CANTEXECUTE: crate::core::HRESULT = crate::core::HRESULT(-1996357631i32);
pub const LANGUAGE_E_DATABASE_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2147215484i32);
pub const LANGUAGE_S_LARGE_WORD: crate::core::HRESULT = crate::core::HRESULT(268161i32);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LPARAM(pub PtrDiffRepr);
impl LPARAM {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for LPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for LPARAM {}
impl ::core::fmt::Debug for LPARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LPARAM").field(&self.0).finish()
    }
}
impl FromIntoMemory for LPARAM {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrDiffRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrDiffRepr>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LRESULT(pub PtrDiffRepr);
impl LRESULT {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LRESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for LRESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for LRESULT {}
impl ::core::fmt::Debug for LRESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LRESULT").field(&self.0).finish()
    }
}
impl FromIntoMemory for LRESULT {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrDiffRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrDiffRepr>()
    }
}
pub struct LUID {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl ::core::marker::Copy for LUID {}
impl ::core::clone::Clone for LUID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LUID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LUID")
            .field("LowPart", &self.LowPart)
            .field("HighPart", &self.HighPart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LUID {
    fn eq(&self, other: &Self) -> bool {
        self.LowPart == other.LowPart && self.HighPart == other.HighPart
    }
}
impl ::core::cmp::Eq for LUID {}
impl FromIntoMemory for LUID {
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
pub const MARSHAL_E_FIRST: i32 = -2147221216i32;
pub const MARSHAL_E_LAST: i32 = -2147221201i32;
pub const MARSHAL_S_FIRST: i32 = 262432i32;
pub const MARSHAL_S_LAST: i32 = 262447i32;
pub const MAX_PATH: u32 = 260u32;
pub const MEM_E_INVALID_LINK: crate::core::HRESULT = crate::core::HRESULT(-2146959344i32);
pub const MEM_E_INVALID_ROOT: crate::core::HRESULT = crate::core::HRESULT(-2146959351i32);
pub const MEM_E_INVALID_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2146959343i32);
pub const MENROLL_S_ENROLLMENT_SUSPENDED: crate::core::HRESULT = crate::core::HRESULT(1572881i32);
pub const MILAVERR_INSUFFICIENTVIDEORESOURCES: crate::core::HRESULT =
    crate::core::HRESULT(-2003303160i32);
pub const MILAVERR_INVALIDWMPVERSION: crate::core::HRESULT = crate::core::HRESULT(-2003303161i32);
pub const MILAVERR_MEDIAPLAYERCLOSED: crate::core::HRESULT = crate::core::HRESULT(-2003303155i32);
pub const MILAVERR_MODULENOTLOADED: crate::core::HRESULT = crate::core::HRESULT(-2003303163i32);
pub const MILAVERR_NOCLOCK: crate::core::HRESULT = crate::core::HRESULT(-2003303168i32);
pub const MILAVERR_NOMEDIATYPE: crate::core::HRESULT = crate::core::HRESULT(-2003303167i32);
pub const MILAVERR_NOREADYFRAMES: crate::core::HRESULT = crate::core::HRESULT(-2003303164i32);
pub const MILAVERR_NOVIDEOMIXER: crate::core::HRESULT = crate::core::HRESULT(-2003303166i32);
pub const MILAVERR_NOVIDEOPRESENTER: crate::core::HRESULT = crate::core::HRESULT(-2003303165i32);
pub const MILAVERR_REQUESTEDTEXTURETOOBIG: crate::core::HRESULT =
    crate::core::HRESULT(-2003303158i32);
pub const MILAVERR_SEEKFAILED: crate::core::HRESULT = crate::core::HRESULT(-2003303157i32);
pub const MILAVERR_UNEXPECTEDWMPFAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-2003303156i32);
pub const MILAVERR_UNKNOWNHARDWAREERROR: crate::core::HRESULT =
    crate::core::HRESULT(-2003303154i32);
pub const MILAVERR_VIDEOACCELERATIONNOTAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2003303159i32);
pub const MILAVERR_WMPFACTORYNOTREGISTERED: crate::core::HRESULT =
    crate::core::HRESULT(-2003303162i32);
pub const MILEFFECTSERR_ALREADYATTACHEDTOLISTENER: crate::core::HRESULT =
    crate::core::HRESULT(-2003302888i32);
pub const MILEFFECTSERR_CONNECTORNOTASSOCIATEDWITHEFFECT: crate::core::HRESULT =
    crate::core::HRESULT(-2003302894i32);
pub const MILEFFECTSERR_CONNECTORNOTCONNECTED: crate::core::HRESULT =
    crate::core::HRESULT(-2003302895i32);
pub const MILEFFECTSERR_CYCLEDETECTED: crate::core::HRESULT = crate::core::HRESULT(-2003302892i32);
pub const MILEFFECTSERR_EFFECTALREADYINAGRAPH: crate::core::HRESULT =
    crate::core::HRESULT(-2003302890i32);
pub const MILEFFECTSERR_EFFECTHASNOCHILDREN: crate::core::HRESULT =
    crate::core::HRESULT(-2003302889i32);
pub const MILEFFECTSERR_EFFECTINMORETHANONEGRAPH: crate::core::HRESULT =
    crate::core::HRESULT(-2003302891i32);
pub const MILEFFECTSERR_EFFECTNOTPARTOFGROUP: crate::core::HRESULT =
    crate::core::HRESULT(-2003302897i32);
pub const MILEFFECTSERR_EMPTYBOUNDS: crate::core::HRESULT = crate::core::HRESULT(-2003302886i32);
pub const MILEFFECTSERR_NOINPUTSOURCEATTACHED: crate::core::HRESULT =
    crate::core::HRESULT(-2003302896i32);
pub const MILEFFECTSERR_NOTAFFINETRANSFORM: crate::core::HRESULT =
    crate::core::HRESULT(-2003302887i32);
pub const MILEFFECTSERR_OUTPUTSIZETOOLARGE: crate::core::HRESULT =
    crate::core::HRESULT(-2003302885i32);
pub const MILEFFECTSERR_RESERVED: crate::core::HRESULT = crate::core::HRESULT(-2003302893i32);
pub const MILEFFECTSERR_UNKNOWNPROPERTY: crate::core::HRESULT =
    crate::core::HRESULT(-2003302898i32);
pub const MILERR_ADAPTER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2003304290i32);
pub const MILERR_ALREADYLOCKED: crate::core::HRESULT = crate::core::HRESULT(-2003304314i32);
pub const MILERR_ALREADY_INITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2003304305i32);
pub const MILERR_BADNUMBER: crate::core::HRESULT = crate::core::HRESULT(-2003304438i32);
pub const MILERR_COLORSPACE_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2003304289i32);
pub const MILERR_DEVICECANNOTRENDERTEXT: crate::core::HRESULT =
    crate::core::HRESULT(-2003304312i32);
pub const MILERR_DISPLAYFORMATNOTSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2003304316i32);
pub const MILERR_DISPLAYID_ACCESS_DENIED: crate::core::HRESULT =
    crate::core::HRESULT(-2003304287i32);
pub const MILERR_DISPLAYSTATEINVALID: crate::core::HRESULT = crate::core::HRESULT(-2003304442i32);
pub const MILERR_DXGI_ENUMERATION_OUT_OF_SYNC: crate::core::HRESULT =
    crate::core::HRESULT(-2003304291i32);
pub const MILERR_GENERIC_IGNORE: crate::core::HRESULT = crate::core::HRESULT(-2003304309i32);
pub const MILERR_GLYPHBITMAPMISSED: crate::core::HRESULT = crate::core::HRESULT(-2003304311i32);
pub const MILERR_INSUFFICIENTBUFFER: crate::core::HRESULT = crate::core::HRESULT(-2003304446i32);
pub const MILERR_INTERNALERROR: crate::core::HRESULT = crate::core::HRESULT(-2003304320i32);
pub const MILERR_INVALIDCALL: crate::core::HRESULT = crate::core::HRESULT(-2003304315i32);
pub const MILERR_MALFORMEDGLYPHCACHE: crate::core::HRESULT = crate::core::HRESULT(-2003304310i32);
pub const MILERR_MALFORMED_GUIDELINE_DATA: crate::core::HRESULT =
    crate::core::HRESULT(-2003304308i32);
pub const MILERR_MAX_TEXTURE_SIZE_EXCEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-2003304294i32);
pub const MILERR_MISMATCHED_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2003304304i32);
pub const MILERR_MROW_READLOCK_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2003304297i32);
pub const MILERR_MROW_UPDATE_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2003304296i32);
pub const MILERR_NEED_RECREATE_AND_PRESENT: crate::core::HRESULT =
    crate::core::HRESULT(-2003304306i32);
pub const MILERR_NONINVERTIBLEMATRIX: crate::core::HRESULT = crate::core::HRESULT(-2003304441i32);
pub const MILERR_NOTLOCKED: crate::core::HRESULT = crate::core::HRESULT(-2003304313i32);
pub const MILERR_NOT_QUEUING_PRESENTS: crate::core::HRESULT = crate::core::HRESULT(-2003304300i32);
pub const MILERR_NO_HARDWARE_DEVICE: crate::core::HRESULT = crate::core::HRESULT(-2003304307i32);
pub const MILERR_NO_REDIRECTION_SURFACE_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2003304303i32);
pub const MILERR_NO_REDIRECTION_SURFACE_RETRY_LATER: crate::core::HRESULT =
    crate::core::HRESULT(-2003304299i32);
pub const MILERR_OBJECTBUSY: crate::core::HRESULT = crate::core::HRESULT(-2003304447i32);
pub const MILERR_PREFILTER_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2003304288i32);
pub const MILERR_QPC_TIME_WENT_BACKWARD: crate::core::HRESULT =
    crate::core::HRESULT(-2003304293i32);
pub const MILERR_QUEUED_PRESENT_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2003304301i32);
pub const MILERR_REMOTING_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2003304302i32);
pub const MILERR_SCANNER_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2003304444i32);
pub const MILERR_SCREENACCESSDENIED: crate::core::HRESULT = crate::core::HRESULT(-2003304443i32);
pub const MILERR_SHADER_COMPILE_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2003304295i32);
pub const MILERR_TERMINATED: crate::core::HRESULT = crate::core::HRESULT(-2003304439i32);
pub const MILERR_TOOMANYSHADERELEMNTS: crate::core::HRESULT = crate::core::HRESULT(-2003304298i32);
pub const MILERR_WIN32ERROR: crate::core::HRESULT = crate::core::HRESULT(-2003304445i32);
pub const MILERR_ZEROVECTOR: crate::core::HRESULT = crate::core::HRESULT(-2003304440i32);
pub const MK_E_CANTOPENFILE: crate::core::HRESULT = crate::core::HRESULT(-2147221014i32);
pub const MK_E_CONNECTMANUALLY: crate::core::HRESULT = crate::core::HRESULT(-2147221024i32);
pub const MK_E_ENUMERATION_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2147221009i32);
pub const MK_E_EXCEEDEDDEADLINE: crate::core::HRESULT = crate::core::HRESULT(-2147221023i32);
pub const MK_E_FIRST: i32 = -2147221024i32;
pub const MK_E_INTERMEDIATEINTERFACENOTSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2147221017i32);
pub const MK_E_INVALIDEXTENSION: crate::core::HRESULT = crate::core::HRESULT(-2147221018i32);
pub const MK_E_LAST: i32 = -2147221009i32;
pub const MK_E_MUSTBOTHERUSER: crate::core::HRESULT = crate::core::HRESULT(-2147221013i32);
pub const MK_E_NEEDGENERIC: crate::core::HRESULT = crate::core::HRESULT(-2147221022i32);
pub const MK_E_NOINVERSE: crate::core::HRESULT = crate::core::HRESULT(-2147221012i32);
pub const MK_E_NOOBJECT: crate::core::HRESULT = crate::core::HRESULT(-2147221019i32);
pub const MK_E_NOPREFIX: crate::core::HRESULT = crate::core::HRESULT(-2147221010i32);
pub const MK_E_NOSTORAGE: crate::core::HRESULT = crate::core::HRESULT(-2147221011i32);
pub const MK_E_NOTBINDABLE: crate::core::HRESULT = crate::core::HRESULT(-2147221016i32);
pub const MK_E_NOTBOUND: crate::core::HRESULT = crate::core::HRESULT(-2147221015i32);
pub const MK_E_NO_NORMALIZED: crate::core::HRESULT = crate::core::HRESULT(-2146959353i32);
pub const MK_E_SYNTAX: crate::core::HRESULT = crate::core::HRESULT(-2147221020i32);
pub const MK_E_UNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2147221021i32);
pub const MK_S_FIRST: i32 = 262624i32;
pub const MK_S_HIM: crate::core::HRESULT = crate::core::HRESULT(262629i32);
pub const MK_S_LAST: i32 = 262639i32;
pub const MK_S_ME: crate::core::HRESULT = crate::core::HRESULT(262628i32);
pub const MK_S_MONIKERALREADYREGISTERED: crate::core::HRESULT = crate::core::HRESULT(262631i32);
pub const MK_S_REDUCED_TO_SELF: crate::core::HRESULT = crate::core::HRESULT(262626i32);
pub const MK_S_US: crate::core::HRESULT = crate::core::HRESULT(262630i32);
pub const MSDTC_E_DUPLICATE_RESOURCE: crate::core::HRESULT = crate::core::HRESULT(-2146367743i32);
pub const MSSIPOTF_E_BADVERSION: crate::core::HRESULT = crate::core::HRESULT(-2146865131i32);
pub const MSSIPOTF_E_BAD_FIRST_TABLE_PLACEMENT: crate::core::HRESULT =
    crate::core::HRESULT(-2146865144i32);
pub const MSSIPOTF_E_BAD_MAGICNUMBER: crate::core::HRESULT = crate::core::HRESULT(-2146865148i32);
pub const MSSIPOTF_E_BAD_OFFSET_TABLE: crate::core::HRESULT = crate::core::HRESULT(-2146865147i32);
pub const MSSIPOTF_E_CANTGETOBJECT: crate::core::HRESULT = crate::core::HRESULT(-2146865150i32);
pub const MSSIPOTF_E_CRYPT: crate::core::HRESULT = crate::core::HRESULT(-2146865132i32);
pub const MSSIPOTF_E_DSIG_STRUCTURE: crate::core::HRESULT = crate::core::HRESULT(-2146865130i32);
pub const MSSIPOTF_E_FAILED_HINTS_CHECK: crate::core::HRESULT =
    crate::core::HRESULT(-2146865135i32);
pub const MSSIPOTF_E_FAILED_POLICY: crate::core::HRESULT = crate::core::HRESULT(-2146865136i32);
pub const MSSIPOTF_E_FILE: crate::core::HRESULT = crate::core::HRESULT(-2146865133i32);
pub const MSSIPOTF_E_FILETOOSMALL: crate::core::HRESULT = crate::core::HRESULT(-2146865141i32);
pub const MSSIPOTF_E_FILE_CHECKSUM: crate::core::HRESULT = crate::core::HRESULT(-2146865139i32);
pub const MSSIPOTF_E_NOHEADTABLE: crate::core::HRESULT = crate::core::HRESULT(-2146865149i32);
pub const MSSIPOTF_E_NOT_OPENTYPE: crate::core::HRESULT = crate::core::HRESULT(-2146865134i32);
pub const MSSIPOTF_E_OUTOFMEMRANGE: crate::core::HRESULT = crate::core::HRESULT(-2146865151i32);
pub const MSSIPOTF_E_PCONST_CHECK: crate::core::HRESULT = crate::core::HRESULT(-2146865129i32);
pub const MSSIPOTF_E_STRUCTURE: crate::core::HRESULT = crate::core::HRESULT(-2146865128i32);
pub const MSSIPOTF_E_TABLES_OVERLAP: crate::core::HRESULT = crate::core::HRESULT(-2146865143i32);
pub const MSSIPOTF_E_TABLE_CHECKSUM: crate::core::HRESULT = crate::core::HRESULT(-2146865140i32);
pub const MSSIPOTF_E_TABLE_LONGWORD: crate::core::HRESULT = crate::core::HRESULT(-2146865145i32);
pub const MSSIPOTF_E_TABLE_PADBYTES: crate::core::HRESULT = crate::core::HRESULT(-2146865142i32);
pub const MSSIPOTF_E_TABLE_TAGORDER: crate::core::HRESULT = crate::core::HRESULT(-2146865146i32);
pub const NAP_E_CONFLICTING_ID: crate::core::HRESULT = crate::core::HRESULT(-2144927741i32);
pub const NAP_E_ENTITY_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2144927730i32);
pub const NAP_E_ID_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144927734i32);
pub const NAP_E_INVALID_PACKET: crate::core::HRESULT = crate::core::HRESULT(-2144927743i32);
pub const NAP_E_MAXSIZE_TOO_SMALL: crate::core::HRESULT = crate::core::HRESULT(-2144927733i32);
pub const NAP_E_MISMATCHED_ID: crate::core::HRESULT = crate::core::HRESULT(-2144927736i32);
pub const NAP_E_MISSING_SOH: crate::core::HRESULT = crate::core::HRESULT(-2144927742i32);
pub const NAP_E_NETSH_GROUPPOLICY_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-2144927729i32);
pub const NAP_E_NOT_INITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2144927737i32);
pub const NAP_E_NOT_PENDING: crate::core::HRESULT = crate::core::HRESULT(-2144927735i32);
pub const NAP_E_NOT_REGISTERED: crate::core::HRESULT = crate::core::HRESULT(-2144927738i32);
pub const NAP_E_NO_CACHED_SOH: crate::core::HRESULT = crate::core::HRESULT(-2144927740i32);
pub const NAP_E_SERVICE_NOT_RUNNING: crate::core::HRESULT = crate::core::HRESULT(-2144927732i32);
pub const NAP_E_SHV_CONFIG_EXISTED: crate::core::HRESULT = crate::core::HRESULT(-2144927727i32);
pub const NAP_E_SHV_CONFIG_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144927726i32);
pub const NAP_E_SHV_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2144927725i32);
pub const NAP_E_STILL_BOUND: crate::core::HRESULT = crate::core::HRESULT(-2144927739i32);
pub const NAP_E_TOO_MANY_CALLS: crate::core::HRESULT = crate::core::HRESULT(-2144927728i32);
pub const NAP_S_CERT_ALREADY_PRESENT: crate::core::HRESULT = crate::core::HRESULT(2555917i32);
pub type NEARPROC = ::core::option::Option<()>;
pub const NOERROR: u32 = 0u32;
pub const NOT_AN_ERROR1: crate::core::HRESULT = crate::core::HRESULT(529920i32);
pub const NTDDI_MAXVER: u32 = 2560u32;
pub const NTE_AUTHENTICATION_IGNORED: crate::core::HRESULT = crate::core::HRESULT(-2146893775i32);
pub const NTE_BAD_ALGID: crate::core::HRESULT = crate::core::HRESULT(-2146893816i32);
pub const NTE_BAD_DATA: crate::core::HRESULT = crate::core::HRESULT(-2146893819i32);
pub const NTE_BAD_FLAGS: crate::core::HRESULT = crate::core::HRESULT(-2146893815i32);
pub const NTE_BAD_HASH: crate::core::HRESULT = crate::core::HRESULT(-2146893822i32);
pub const NTE_BAD_HASH_STATE: crate::core::HRESULT = crate::core::HRESULT(-2146893812i32);
pub const NTE_BAD_KEY: crate::core::HRESULT = crate::core::HRESULT(-2146893821i32);
pub const NTE_BAD_KEYSET: crate::core::HRESULT = crate::core::HRESULT(-2146893802i32);
pub const NTE_BAD_KEYSET_PARAM: crate::core::HRESULT = crate::core::HRESULT(-2146893793i32);
pub const NTE_BAD_KEY_STATE: crate::core::HRESULT = crate::core::HRESULT(-2146893813i32);
pub const NTE_BAD_LEN: crate::core::HRESULT = crate::core::HRESULT(-2146893820i32);
pub const NTE_BAD_PROVIDER: crate::core::HRESULT = crate::core::HRESULT(-2146893805i32);
pub const NTE_BAD_PROV_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2146893804i32);
pub const NTE_BAD_PUBLIC_KEY: crate::core::HRESULT = crate::core::HRESULT(-2146893803i32);
pub const NTE_BAD_SIGNATURE: crate::core::HRESULT = crate::core::HRESULT(-2146893818i32);
pub const NTE_BAD_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2146893814i32);
pub const NTE_BAD_UID: crate::core::HRESULT = crate::core::HRESULT(-2146893823i32);
pub const NTE_BAD_VER: crate::core::HRESULT = crate::core::HRESULT(-2146893817i32);
pub const NTE_BUFFERS_OVERLAP: crate::core::HRESULT = crate::core::HRESULT(-2146893781i32);
pub const NTE_BUFFER_TOO_SMALL: crate::core::HRESULT = crate::core::HRESULT(-2146893784i32);
pub const NTE_DECRYPTION_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2146893780i32);
pub const NTE_DEVICE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2146893771i32);
pub const NTE_DEVICE_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-2146893776i32);
pub const NTE_DOUBLE_ENCRYPT: crate::core::HRESULT = crate::core::HRESULT(-2146893806i32);
pub const NTE_ENCRYPTION_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2146893772i32);
pub const NTE_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2146893809i32);
pub const NTE_FAIL: crate::core::HRESULT = crate::core::HRESULT(-2146893792i32);
pub const NTE_FIXEDPARAMETER: crate::core::HRESULT = crate::core::HRESULT(-2146893787i32);
pub const NTE_HMAC_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2146893777i32);
pub const NTE_INCORRECT_PASSWORD: crate::core::HRESULT = crate::core::HRESULT(-2146893773i32);
pub const NTE_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146893779i32);
pub const NTE_INVALID_HANDLE: crate::core::HRESULT = crate::core::HRESULT(-2146893786i32);
pub const NTE_INVALID_PARAMETER: crate::core::HRESULT = crate::core::HRESULT(-2146893785i32);
pub const NTE_KEYSET_ENTRY_BAD: crate::core::HRESULT = crate::core::HRESULT(-2146893798i32);
pub const NTE_KEYSET_NOT_DEF: crate::core::HRESULT = crate::core::HRESULT(-2146893799i32);
pub const NTE_NOT_ACTIVE_CONSOLE: crate::core::HRESULT = crate::core::HRESULT(-2146893768i32);
pub const NTE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2146893807i32);
pub const NTE_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2146893783i32);
pub const NTE_NO_KEY: crate::core::HRESULT = crate::core::HRESULT(-2146893811i32);
pub const NTE_NO_MEMORY: crate::core::HRESULT = crate::core::HRESULT(-2146893810i32);
pub const NTE_NO_MORE_ITEMS: crate::core::HRESULT = crate::core::HRESULT(-2146893782i32);
pub const NTE_OP_OK: u32 = 0u32;
pub const NTE_PASSWORD_CHANGE_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2146893769i32);
pub const NTE_PERM: crate::core::HRESULT = crate::core::HRESULT(-2146893808i32);
pub const NTE_PROVIDER_DLL_FAIL: crate::core::HRESULT = crate::core::HRESULT(-2146893795i32);
pub const NTE_PROV_DLL_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2146893794i32);
pub const NTE_PROV_TYPE_ENTRY_BAD: crate::core::HRESULT = crate::core::HRESULT(-2146893800i32);
pub const NTE_PROV_TYPE_NOT_DEF: crate::core::HRESULT = crate::core::HRESULT(-2146893801i32);
pub const NTE_PROV_TYPE_NO_MATCH: crate::core::HRESULT = crate::core::HRESULT(-2146893797i32);
pub const NTE_SIGNATURE_FILE_BAD: crate::core::HRESULT = crate::core::HRESULT(-2146893796i32);
pub const NTE_SILENT_CONTEXT: crate::core::HRESULT = crate::core::HRESULT(-2146893790i32);
pub const NTE_SYS_ERR: crate::core::HRESULT = crate::core::HRESULT(-2146893791i32);
pub const NTE_TEMPORARY_PROFILE: crate::core::HRESULT = crate::core::HRESULT(-2146893788i32);
pub const NTE_TOKEN_KEYSET_STORAGE_FULL: crate::core::HRESULT =
    crate::core::HRESULT(-2146893789i32);
pub const NTE_UI_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2146893778i32);
pub const NTE_USER_CANCELLED: crate::core::HRESULT = crate::core::HRESULT(-2146893770i32);
pub const NTE_VALIDATION_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2146893774i32);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NTSTATUS(pub i32);
impl NTSTATUS {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NTSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NTSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NTSTATUS {}
impl ::core::fmt::Debug for NTSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NTSTATUS").field(&self.0).finish()
    }
}
impl FromIntoMemory for NTSTATUS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<i32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<i32>()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NTSTATUS_FACILITY_CODE(pub u32);
pub const FACILITY_DEBUGGER: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(1u32);
pub const FACILITY_RPC_RUNTIME: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(2u32);
pub const FACILITY_RPC_STUBS: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(3u32);
pub const FACILITY_IO_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(4u32);
pub const FACILITY_CODCLASS_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(6u32);
pub const FACILITY_NTWIN32: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(7u32);
pub const FACILITY_NTCERT: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(8u32);
pub const FACILITY_NTSSPI: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(9u32);
pub const FACILITY_TERMINAL_SERVER: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(10u32);
pub const FACILITY_USB_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(16u32);
pub const FACILITY_HID_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(17u32);
pub const FACILITY_FIREWIRE_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(18u32);
pub const FACILITY_CLUSTER_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(19u32);
pub const FACILITY_ACPI_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(20u32);
pub const FACILITY_SXS_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(21u32);
pub const FACILITY_TRANSACTION: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(25u32);
pub const FACILITY_COMMONLOG: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(26u32);
pub const FACILITY_VIDEO: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(27u32);
pub const FACILITY_FILTER_MANAGER: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(28u32);
pub const FACILITY_MONITOR: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(29u32);
pub const FACILITY_GRAPHICS_KERNEL: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(30u32);
pub const FACILITY_DRIVER_FRAMEWORK: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(32u32);
pub const FACILITY_FVE_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(33u32);
pub const FACILITY_FWP_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(34u32);
pub const FACILITY_NDIS_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(35u32);
pub const FACILITY_QUIC_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(36u32);
pub const FACILITY_TPM: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(41u32);
pub const FACILITY_RTPM: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(42u32);
pub const FACILITY_HYPERVISOR: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(53u32);
pub const FACILITY_IPSEC: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(54u32);
pub const FACILITY_VIRTUALIZATION: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(55u32);
pub const FACILITY_VOLMGR: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(56u32);
pub const FACILITY_BCD_ERROR_CODE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(57u32);
pub const FACILITY_WIN32K_NTUSER: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(62u32);
pub const FACILITY_WIN32K_NTGDI: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(63u32);
pub const FACILITY_RESUME_KEY_FILTER: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(64u32);
pub const FACILITY_RDBSS: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(65u32);
pub const FACILITY_BTH_ATT: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(66u32);
pub const FACILITY_SECUREBOOT: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(67u32);
pub const FACILITY_AUDIO_KERNEL: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(68u32);
pub const FACILITY_VSM: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(69u32);
pub const FACILITY_NT_IORING: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(70u32);
pub const FACILITY_VOLSNAP: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(80u32);
pub const FACILITY_SDBUS: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(81u32);
pub const FACILITY_SHARED_VHDX: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(92u32);
pub const FACILITY_SMB: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(93u32);
pub const FACILITY_XVS: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(94u32);
pub const FACILITY_INTERIX: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(153u32);
pub const FACILITY_SPACES: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(231u32);
pub const FACILITY_SECURITY_CORE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(232u32);
pub const FACILITY_SYSTEM_INTEGRITY: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(233u32);
pub const FACILITY_LICENSING: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(234u32);
pub const FACILITY_PLATFORM_MANIFEST: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(235u32);
pub const FACILITY_APP_EXEC: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(236u32);
pub const FACILITY_MAXIMUM_VALUE: NTSTATUS_FACILITY_CODE = NTSTATUS_FACILITY_CODE(237u32);
impl ::core::marker::Copy for NTSTATUS_FACILITY_CODE {}
impl ::core::clone::Clone for NTSTATUS_FACILITY_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NTSTATUS_FACILITY_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NTSTATUS_FACILITY_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NTSTATUS_FACILITY_CODE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NTSTATUS_FACILITY_CODE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<u32>()
    }
}
pub const OLEOBJ_E_FIRST: i32 = -2147221120i32;
pub const OLEOBJ_E_INVALIDVERB: crate::core::HRESULT = crate::core::HRESULT(-2147221119i32);
pub const OLEOBJ_E_LAST: i32 = -2147221105i32;
pub const OLEOBJ_E_NOVERBS: crate::core::HRESULT = crate::core::HRESULT(-2147221120i32);
pub const OLEOBJ_S_CANNOT_DOVERB_NOW: crate::core::HRESULT = crate::core::HRESULT(262529i32);
pub const OLEOBJ_S_FIRST: i32 = 262528i32;
pub const OLEOBJ_S_INVALIDHWND: crate::core::HRESULT = crate::core::HRESULT(262530i32);
pub const OLEOBJ_S_INVALIDVERB: crate::core::HRESULT = crate::core::HRESULT(262528i32);
pub const OLEOBJ_S_LAST: i32 = 262543i32;
pub const OLE_E_ADVF: crate::core::HRESULT = crate::core::HRESULT(-2147221503i32);
pub const OLE_E_ADVISENOTSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2147221501i32);
pub const OLE_E_BLANK: crate::core::HRESULT = crate::core::HRESULT(-2147221497i32);
pub const OLE_E_CANTCONVERT: crate::core::HRESULT = crate::core::HRESULT(-2147221487i32);
pub const OLE_E_CANT_BINDTOSOURCE: crate::core::HRESULT = crate::core::HRESULT(-2147221494i32);
pub const OLE_E_CANT_GETMONIKER: crate::core::HRESULT = crate::core::HRESULT(-2147221495i32);
pub const OLE_E_CLASSDIFF: crate::core::HRESULT = crate::core::HRESULT(-2147221496i32);
pub const OLE_E_ENUM_NOMORE: crate::core::HRESULT = crate::core::HRESULT(-2147221502i32);
pub const OLE_E_FIRST: crate::core::HRESULT = crate::core::HRESULT(-2147221504i32);
pub const OLE_E_INVALIDHWND: crate::core::HRESULT = crate::core::HRESULT(-2147221489i32);
pub const OLE_E_INVALIDRECT: crate::core::HRESULT = crate::core::HRESULT(-2147221491i32);
pub const OLE_E_LAST: crate::core::HRESULT = crate::core::HRESULT(-2147221249i32);
pub const OLE_E_NOCACHE: crate::core::HRESULT = crate::core::HRESULT(-2147221498i32);
pub const OLE_E_NOCONNECTION: crate::core::HRESULT = crate::core::HRESULT(-2147221500i32);
pub const OLE_E_NOSTORAGE: crate::core::HRESULT = crate::core::HRESULT(-2147221486i32);
pub const OLE_E_NOTRUNNING: crate::core::HRESULT = crate::core::HRESULT(-2147221499i32);
pub const OLE_E_NOT_INPLACEACTIVE: crate::core::HRESULT = crate::core::HRESULT(-2147221488i32);
pub const OLE_E_OLEVERB: crate::core::HRESULT = crate::core::HRESULT(-2147221504i32);
pub const OLE_E_PROMPTSAVECANCELLED: crate::core::HRESULT = crate::core::HRESULT(-2147221492i32);
pub const OLE_E_STATIC: crate::core::HRESULT = crate::core::HRESULT(-2147221493i32);
pub const OLE_E_WRONGCOMPOBJ: crate::core::HRESULT = crate::core::HRESULT(-2147221490i32);
pub const OLE_S_FIRST: crate::core::HRESULT = crate::core::HRESULT(262144i32);
pub const OLE_S_LAST: crate::core::HRESULT = crate::core::HRESULT(262399i32);
pub const OLE_S_MAC_CLIPFORMAT: crate::core::HRESULT = crate::core::HRESULT(262146i32);
pub const OLE_S_STATIC: crate::core::HRESULT = crate::core::HRESULT(262145i32);
pub const OLE_S_USEREG: crate::core::HRESULT = crate::core::HRESULT(262144i32);
pub const ONL_CONNECTION_COUNT_LIMIT: crate::core::HRESULT = crate::core::HRESULT(-2138701811i32);
pub const ONL_E_ACCESS_DENIED_BY_TOU: crate::core::HRESULT = crate::core::HRESULT(-2138701822i32);
pub const ONL_E_ACCOUNT_LOCKED: crate::core::HRESULT = crate::core::HRESULT(-2138701817i32);
pub const ONL_E_ACCOUNT_SUSPENDED_ABUSE: crate::core::HRESULT =
    crate::core::HRESULT(-2138701813i32);
pub const ONL_E_ACCOUNT_SUSPENDED_COMPROIMISE: crate::core::HRESULT =
    crate::core::HRESULT(-2138701814i32);
pub const ONL_E_ACCOUNT_UPDATE_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2138701819i32);
pub const ONL_E_ACTION_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2138701812i32);
pub const ONL_E_CONNECTED_ACCOUNT_CAN_NOT_SIGNOUT: crate::core::HRESULT =
    crate::core::HRESULT(-2138701810i32);
pub const ONL_E_EMAIL_VERIFICATION_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2138701815i32);
pub const ONL_E_FORCESIGNIN: crate::core::HRESULT = crate::core::HRESULT(-2138701818i32);
pub const ONL_E_INVALID_APPLICATION: crate::core::HRESULT = crate::core::HRESULT(-2138701821i32);
pub const ONL_E_INVALID_AUTHENTICATION_TARGET: crate::core::HRESULT =
    crate::core::HRESULT(-2138701823i32);
pub const ONL_E_PARENTAL_CONSENT_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2138701816i32);
pub const ONL_E_PASSWORD_UPDATE_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2138701820i32);
pub const ONL_E_REQUEST_THROTTLED: crate::core::HRESULT = crate::core::HRESULT(-2138701808i32);
pub const ONL_E_USER_AUTHENTICATION_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2138701809i32);
pub const OR_INVALID_OID: i32 = 1911i32;
pub const OR_INVALID_OXID: i32 = 1910i32;
pub const OR_INVALID_SET: i32 = 1912i32;
pub const OSS_ACCESS_SERIALIZATION_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-2146881517i32);
pub const OSS_API_DLL_NOT_LINKED: crate::core::HRESULT = crate::core::HRESULT(-2146881495i32);
pub const OSS_BAD_ARG: crate::core::HRESULT = crate::core::HRESULT(-2146881530i32);
pub const OSS_BAD_ENCRULES: crate::core::HRESULT = crate::core::HRESULT(-2146881514i32);
pub const OSS_BAD_PTR: crate::core::HRESULT = crate::core::HRESULT(-2146881525i32);
pub const OSS_BAD_TABLE: crate::core::HRESULT = crate::core::HRESULT(-2146881521i32);
pub const OSS_BAD_TIME: crate::core::HRESULT = crate::core::HRESULT(-2146881524i32);
pub const OSS_BAD_VERSION: crate::core::HRESULT = crate::core::HRESULT(-2146881529i32);
pub const OSS_BERDER_DLL_NOT_LINKED: crate::core::HRESULT = crate::core::HRESULT(-2146881494i32);
pub const OSS_CANT_CLOSE_TRACE_FILE: crate::core::HRESULT = crate::core::HRESULT(-2146881490i32);
pub const OSS_CANT_OPEN_TRACE_FILE: crate::core::HRESULT = crate::core::HRESULT(-2146881509i32);
pub const OSS_CANT_OPEN_TRACE_WINDOW: crate::core::HRESULT = crate::core::HRESULT(-2146881512i32);
pub const OSS_COMPARATOR_CODE_NOT_LINKED: crate::core::HRESULT =
    crate::core::HRESULT(-2146881499i32);
pub const OSS_COMPARATOR_DLL_NOT_LINKED: crate::core::HRESULT =
    crate::core::HRESULT(-2146881500i32);
pub const OSS_CONSTRAINT_DLL_NOT_LINKED: crate::core::HRESULT =
    crate::core::HRESULT(-2146881501i32);
pub const OSS_CONSTRAINT_VIOLATED: crate::core::HRESULT = crate::core::HRESULT(-2146881519i32);
pub const OSS_COPIER_DLL_NOT_LINKED: crate::core::HRESULT = crate::core::HRESULT(-2146881502i32);
pub const OSS_DATA_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146881531i32);
pub const OSS_FATAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146881518i32);
pub const OSS_INDEFINITE_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2146881523i32);
pub const OSS_LIMITED: crate::core::HRESULT = crate::core::HRESULT(-2146881526i32);
pub const OSS_MEM_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146881522i32);
pub const OSS_MEM_MGR_DLL_NOT_LINKED: crate::core::HRESULT = crate::core::HRESULT(-2146881498i32);
pub const OSS_MORE_BUF: crate::core::HRESULT = crate::core::HRESULT(-2146881535i32);
pub const OSS_MORE_INPUT: crate::core::HRESULT = crate::core::HRESULT(-2146881532i32);
pub const OSS_MUTEX_NOT_CREATED: crate::core::HRESULT = crate::core::HRESULT(-2146881491i32);
pub const OSS_NEGATIVE_UINTEGER: crate::core::HRESULT = crate::core::HRESULT(-2146881534i32);
pub const OSS_NULL_FCN: crate::core::HRESULT = crate::core::HRESULT(-2146881515i32);
pub const OSS_NULL_TBL: crate::core::HRESULT = crate::core::HRESULT(-2146881516i32);
pub const OSS_OID_DLL_NOT_LINKED: crate::core::HRESULT = crate::core::HRESULT(-2146881510i32);
pub const OSS_OPEN_TYPE_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146881492i32);
pub const OSS_OUT_MEMORY: crate::core::HRESULT = crate::core::HRESULT(-2146881528i32);
pub const OSS_OUT_OF_RANGE: crate::core::HRESULT = crate::core::HRESULT(-2146881503i32);
pub const OSS_PDU_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2146881527i32);
pub const OSS_PDU_RANGE: crate::core::HRESULT = crate::core::HRESULT(-2146881533i32);
pub const OSS_PDV_CODE_NOT_LINKED: crate::core::HRESULT = crate::core::HRESULT(-2146881496i32);
pub const OSS_PDV_DLL_NOT_LINKED: crate::core::HRESULT = crate::core::HRESULT(-2146881497i32);
pub const OSS_PER_DLL_NOT_LINKED: crate::core::HRESULT = crate::core::HRESULT(-2146881493i32);
pub const OSS_REAL_CODE_NOT_LINKED: crate::core::HRESULT = crate::core::HRESULT(-2146881504i32);
pub const OSS_REAL_DLL_NOT_LINKED: crate::core::HRESULT = crate::core::HRESULT(-2146881505i32);
pub const OSS_TABLE_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2146881507i32);
pub const OSS_TOO_LONG: crate::core::HRESULT = crate::core::HRESULT(-2146881520i32);
pub const OSS_TRACE_FILE_ALREADY_OPEN: crate::core::HRESULT = crate::core::HRESULT(-2146881508i32);
pub const OSS_TYPE_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2146881506i32);
pub const OSS_UNAVAIL_ENCRULES: crate::core::HRESULT = crate::core::HRESULT(-2146881513i32);
pub const OSS_UNIMPLEMENTED: crate::core::HRESULT = crate::core::HRESULT(-2146881511i32);
pub type PAPCFUNC = ::core::option::Option<()>;
pub const PEERDIST_ERROR_ALREADY_COMPLETED: i32 = 4060i32;
pub const PEERDIST_ERROR_ALREADY_EXISTS: i32 = 4058i32;
pub const PEERDIST_ERROR_ALREADY_INITIALIZED: i32 = 4055i32;
pub const PEERDIST_ERROR_CANNOT_PARSE_CONTENTINFO: i32 = 4051i32;
pub const PEERDIST_ERROR_CONTENTINFO_VERSION_UNSUPPORTED: i32 = 4050i32;
pub const PEERDIST_ERROR_INVALIDATED: i32 = 4057i32;
pub const PEERDIST_ERROR_INVALID_CONFIGURATION: i32 = 4063i32;
pub const PEERDIST_ERROR_MISSING_DATA: i32 = 4052i32;
pub const PEERDIST_ERROR_NOT_INITIALIZED: i32 = 4054i32;
pub const PEERDIST_ERROR_NOT_LICENSED: i32 = 4064i32;
pub const PEERDIST_ERROR_NO_MORE: i32 = 4053i32;
pub const PEERDIST_ERROR_OPERATION_NOTFOUND: i32 = 4059i32;
pub const PEERDIST_ERROR_OUT_OF_BOUNDS: i32 = 4061i32;
pub const PEERDIST_ERROR_SERVICE_UNAVAILABLE: i32 = 4065i32;
pub const PEERDIST_ERROR_SHUTDOWN_IN_PROGRESS: i32 = 4056i32;
pub const PEERDIST_ERROR_TRUST_FAILURE: i32 = 4066i32;
pub const PEERDIST_ERROR_VERSION_UNSUPPORTED: i32 = 4062i32;
pub const PEER_E_ALREADY_LISTENING: crate::core::HRESULT = crate::core::HRESULT(-2140995321i32);
pub const PEER_E_CANNOT_CONVERT_PEER_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-2140979199i32);
pub const PEER_E_CANNOT_START_SERVICE: crate::core::HRESULT = crate::core::HRESULT(-2140995581i32);
pub const PEER_E_CERT_STORE_CORRUPTED: crate::core::HRESULT = crate::core::HRESULT(-2140993535i32);
pub const PEER_E_CHAIN_TOO_LONG: crate::core::HRESULT = crate::core::HRESULT(-2140993789i32);
pub const PEER_E_CIRCULAR_CHAIN_DETECTED: crate::core::HRESULT =
    crate::core::HRESULT(-2140993786i32);
pub const PEER_E_CLASSIFIER_TOO_LONG: crate::core::HRESULT = crate::core::HRESULT(-2140995071i32);
pub const PEER_E_CLOUD_NAME_AMBIGUOUS: crate::core::HRESULT = crate::core::HRESULT(-2140991483i32);
pub const PEER_E_CONNECTION_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2140995319i32);
pub const PEER_E_CONNECTION_NOT_AUTHENTICATED: crate::core::HRESULT =
    crate::core::HRESULT(-2140995318i32);
pub const PEER_E_CONNECTION_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2140995325i32);
pub const PEER_E_CONNECTION_REFUSED: crate::core::HRESULT = crate::core::HRESULT(-2140995317i32);
pub const PEER_E_CONNECT_SELF: crate::core::HRESULT = crate::core::HRESULT(-2140995322i32);
pub const PEER_E_CONTACT_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2140971007i32);
pub const PEER_E_DATABASE_ACCESSDENIED: crate::core::HRESULT = crate::core::HRESULT(-2140994814i32);
pub const PEER_E_DATABASE_ALREADY_PRESENT: crate::core::HRESULT =
    crate::core::HRESULT(-2140994811i32);
pub const PEER_E_DATABASE_NOT_PRESENT: crate::core::HRESULT = crate::core::HRESULT(-2140994810i32);
pub const PEER_E_DBINITIALIZATION_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2140994813i32);
pub const PEER_E_DBNAME_CHANGED: crate::core::HRESULT = crate::core::HRESULT(-2140995567i32);
pub const PEER_E_DEFERRED_VALIDATION: crate::core::HRESULT = crate::core::HRESULT(-2140987344i32);
pub const PEER_E_DUPLICATE_GRAPH: crate::core::HRESULT = crate::core::HRESULT(-2140995566i32);
pub const PEER_E_EVENT_HANDLE_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2140994303i32);
pub const PEER_E_FW_BLOCKED_BY_POLICY: crate::core::HRESULT = crate::core::HRESULT(-2140966903i32);
pub const PEER_E_FW_BLOCKED_BY_SHIELDS_UP: crate::core::HRESULT =
    crate::core::HRESULT(-2140966902i32);
pub const PEER_E_FW_DECLINED: crate::core::HRESULT = crate::core::HRESULT(-2140966901i32);
pub const PEER_E_FW_EXCEPTION_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2140966904i32);
pub const PEER_E_GRAPH_IN_USE: crate::core::HRESULT = crate::core::HRESULT(-2140995563i32);
pub const PEER_E_GRAPH_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-2140995565i32);
pub const PEER_E_GRAPH_SHUTTING_DOWN: crate::core::HRESULT = crate::core::HRESULT(-2140995564i32);
pub const PEER_E_GROUPS_EXIST: crate::core::HRESULT = crate::core::HRESULT(-2140995068i32);
pub const PEER_E_GROUP_IN_USE: crate::core::HRESULT = crate::core::HRESULT(-2140987246i32);
pub const PEER_E_GROUP_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-2140987247i32);
pub const PEER_E_IDENTITY_DELETED: crate::core::HRESULT = crate::core::HRESULT(-2140987232i32);
pub const PEER_E_IDENTITY_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2140994559i32);
pub const PEER_E_INVALID_ADDRESS: crate::core::HRESULT = crate::core::HRESULT(-2140966905i32);
pub const PEER_E_INVALID_ATTRIBUTES: crate::core::HRESULT = crate::core::HRESULT(-2140994046i32);
pub const PEER_E_INVALID_CLASSIFIER: crate::core::HRESULT = crate::core::HRESULT(-2140987296i32);
pub const PEER_E_INVALID_CLASSIFIER_PROPERTY: crate::core::HRESULT =
    crate::core::HRESULT(-2140987278i32);
pub const PEER_E_INVALID_CREDENTIAL: crate::core::HRESULT = crate::core::HRESULT(-2140987262i32);
pub const PEER_E_INVALID_CREDENTIAL_INFO: crate::core::HRESULT =
    crate::core::HRESULT(-2140987263i32);
pub const PEER_E_INVALID_DATABASE: crate::core::HRESULT = crate::core::HRESULT(-2140995562i32);
pub const PEER_E_INVALID_FRIENDLY_NAME: crate::core::HRESULT = crate::core::HRESULT(-2140987280i32);
pub const PEER_E_INVALID_GRAPH: crate::core::HRESULT = crate::core::HRESULT(-2140995568i32);
pub const PEER_E_INVALID_GROUP: crate::core::HRESULT = crate::core::HRESULT(-2140987245i32);
pub const PEER_E_INVALID_GROUP_PROPERTIES: crate::core::HRESULT =
    crate::core::HRESULT(-2140987328i32);
pub const PEER_E_INVALID_PEER_HOST_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-2140979198i32);
pub const PEER_E_INVALID_PEER_NAME: crate::core::HRESULT = crate::core::HRESULT(-2140987312i32);
pub const PEER_E_INVALID_RECORD: crate::core::HRESULT = crate::core::HRESULT(-2140987376i32);
pub const PEER_E_INVALID_RECORD_EXPIRATION: crate::core::HRESULT =
    crate::core::HRESULT(-2140987264i32);
pub const PEER_E_INVALID_RECORD_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2140987261i32);
pub const PEER_E_INVALID_ROLE_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-2140987279i32);
pub const PEER_E_INVALID_SEARCH: crate::core::HRESULT = crate::core::HRESULT(-2140994047i32);
pub const PEER_E_INVALID_TIME_PERIOD: crate::core::HRESULT = crate::core::HRESULT(-2140993787i32);
pub const PEER_E_INVITATION_NOT_TRUSTED: crate::core::HRESULT =
    crate::core::HRESULT(-2140993791i32);
pub const PEER_E_INVITE_CANCELLED: crate::core::HRESULT = crate::core::HRESULT(-2140966912i32);
pub const PEER_E_INVITE_RESPONSE_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2140966911i32);
pub const PEER_E_IPV6_NOT_INSTALLED: crate::core::HRESULT = crate::core::HRESULT(-2140995583i32);
pub const PEER_E_MAX_RECORD_SIZE_EXCEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-2140994812i32);
pub const PEER_E_NODE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2140995320i32);
pub const PEER_E_NOT_AUTHORIZED: crate::core::HRESULT = crate::core::HRESULT(-2140987360i32);
pub const PEER_E_NOT_INITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2140995582i32);
pub const PEER_E_NOT_LICENSED: crate::core::HRESULT = crate::core::HRESULT(-2140995580i32);
pub const PEER_E_NOT_SIGNED_IN: crate::core::HRESULT = crate::core::HRESULT(-2140966909i32);
pub const PEER_E_NO_CLOUD: crate::core::HRESULT = crate::core::HRESULT(-2140991487i32);
pub const PEER_E_NO_KEY_ACCESS: crate::core::HRESULT = crate::core::HRESULT(-2140995069i32);
pub const PEER_E_NO_MEMBERS_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2140987244i32);
pub const PEER_E_NO_MEMBER_CONNECTIONS: crate::core::HRESULT = crate::core::HRESULT(-2140987243i32);
pub const PEER_E_NO_MORE: crate::core::HRESULT = crate::core::HRESULT(-2140979197i32);
pub const PEER_E_PASSWORD_DOES_NOT_MEET_POLICY: crate::core::HRESULT =
    crate::core::HRESULT(-2140987359i32);
pub const PEER_E_PNRP_DUPLICATE_PEER_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-2140979195i32);
pub const PEER_E_PRIVACY_DECLINED: crate::core::HRESULT = crate::core::HRESULT(-2140966908i32);
pub const PEER_E_RECORD_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2140994815i32);
pub const PEER_E_SERVICE_NOT_AVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2140987231i32);
pub const PEER_E_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2140966907i32);
pub const PEER_E_TOO_MANY_ATTRIBUTES: crate::core::HRESULT = crate::core::HRESULT(-2140995561i32);
pub const PEER_E_TOO_MANY_IDENTITIES: crate::core::HRESULT = crate::core::HRESULT(-2140995070i32);
pub const PEER_E_UNABLE_TO_LISTEN: crate::core::HRESULT = crate::core::HRESULT(-2140987242i32);
pub const PEER_E_UNSUPPORTED_VERSION: crate::core::HRESULT = crate::core::HRESULT(-2140987248i32);
pub const PEER_S_ALREADY_A_MEMBER: crate::core::HRESULT = crate::core::HRESULT(6488070i32);
pub const PEER_S_ALREADY_CONNECTED: crate::core::HRESULT = crate::core::HRESULT(6496256i32);
pub const PEER_S_GRAPH_DATA_CREATED: crate::core::HRESULT = crate::core::HRESULT(6488065i32);
pub const PEER_S_NO_CONNECTIVITY: crate::core::HRESULT = crate::core::HRESULT(6488069i32);
pub const PEER_S_NO_EVENT_DATA: crate::core::HRESULT = crate::core::HRESULT(6488066i32);
pub const PEER_S_SUBSCRIPTION_EXISTS: crate::core::HRESULT = crate::core::HRESULT(6512640i32);
pub const PERSIST_E_NOTSELFSIZING: crate::core::HRESULT = crate::core::HRESULT(-2146762741i32);
pub const PERSIST_E_SIZEDEFINITE: crate::core::HRESULT = crate::core::HRESULT(-2146762743i32);
pub const PERSIST_E_SIZEINDEFINITE: crate::core::HRESULT = crate::core::HRESULT(-2146762742i32);
pub const PLA_E_CABAPI_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2144337645i32);
pub const PLA_E_CONFLICT_INCL_EXCL_API: crate::core::HRESULT = crate::core::HRESULT(-2144337659i32);
pub const PLA_E_CREDENTIALS_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2144337661i32);
pub const PLA_E_DCS_ALREADY_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2144337737i32);
pub const PLA_E_DCS_IN_USE: crate::core::HRESULT = crate::core::HRESULT(-2144337750i32);
pub const PLA_E_DCS_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144337918i32);
pub const PLA_E_DCS_NOT_RUNNING: crate::core::HRESULT = crate::core::HRESULT(-2144337660i32);
pub const PLA_E_DCS_SINGLETON_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2144337662i32);
pub const PLA_E_DCS_START_WAIT_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2144337654i32);
pub const PLA_E_DC_ALREADY_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2144337655i32);
pub const PLA_E_DC_START_WAIT_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2144337653i32);
pub const PLA_E_EXE_ALREADY_CONFIGURED: crate::core::HRESULT = crate::core::HRESULT(-2144337657i32);
pub const PLA_E_EXE_FULL_PATH_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2144337650i32);
pub const PLA_E_EXE_PATH_NOT_VALID: crate::core::HRESULT = crate::core::HRESULT(-2144337656i32);
pub const PLA_E_INVALID_SESSION_NAME: crate::core::HRESULT = crate::core::HRESULT(-2144337649i32);
pub const PLA_E_NETWORK_EXE_NOT_VALID: crate::core::HRESULT = crate::core::HRESULT(-2144337658i32);
pub const PLA_E_NO_DUPLICATES: crate::core::HRESULT = crate::core::HRESULT(-2144337651i32);
pub const PLA_E_NO_MIN_DISK: crate::core::HRESULT = crate::core::HRESULT(-2144337808i32);
pub const PLA_E_PLA_CHANNEL_NOT_ENABLED: crate::core::HRESULT =
    crate::core::HRESULT(-2144337648i32);
pub const PLA_E_PROPERTY_CONFLICT: crate::core::HRESULT = crate::core::HRESULT(-2144337663i32);
pub const PLA_E_REPORT_WAIT_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2144337652i32);
pub const PLA_E_RULES_MANAGER_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144337646i32);
pub const PLA_E_TASKSCHED_CHANNEL_NOT_ENABLED: crate::core::HRESULT =
    crate::core::HRESULT(-2144337647i32);
pub const PLA_E_TOO_MANY_FOLDERS: crate::core::HRESULT = crate::core::HRESULT(-2144337851i32);
pub const PLA_S_PROPERTY_IGNORED: crate::core::HRESULT = crate::core::HRESULT(3145984i32);
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
impl ::core::marker::Copy for POINT {}
impl ::core::clone::Clone for POINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINT")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
impl ::core::cmp::PartialEq for POINT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for POINT {}
impl FromIntoMemory for POINT {
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
pub struct POINTL {
    pub x: i32,
    pub y: i32,
}
impl ::core::marker::Copy for POINTL {}
impl ::core::clone::Clone for POINTL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for POINTL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTL")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
impl ::core::cmp::PartialEq for POINTL {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for POINTL {}
impl FromIntoMemory for POINTL {
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
pub struct POINTS {
    pub x: i16,
    pub y: i16,
}
impl ::core::marker::Copy for POINTS {}
impl ::core::clone::Clone for POINTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for POINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTS")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
impl ::core::cmp::PartialEq for POINTS {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for POINTS {}
impl FromIntoMemory for POINTS {
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
pub const PRESENTATION_ERROR_LOST: crate::core::HRESULT = crate::core::HRESULT(-2004811775i32);
pub type PROC = ::core::option::Option<()>;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PSID(pub PtrDiffRepr);
impl PSID {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PSID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PSID {}
impl ::core::fmt::Debug for PSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSID").field(&self.0).finish()
    }
}
impl FromIntoMemory for PSID {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrDiffRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrDiffRepr>()
    }
}
pub const PSINK_E_INDEX_ONLY: crate::core::HRESULT = crate::core::HRESULT(-2147215471i32);
pub const PSINK_E_LARGE_ATTACHMENT: crate::core::HRESULT = crate::core::HRESULT(-2147215470i32);
pub const PSINK_E_QUERY_ONLY: crate::core::HRESULT = crate::core::HRESULT(-2147215472i32);
pub const PSINK_S_LARGE_WORD: crate::core::HRESULT = crate::core::HRESULT(268179i32);
pub const QPARSE_E_EXPECTING_BRACE: crate::core::HRESULT = crate::core::HRESULT(-2147215770i32);
pub const QPARSE_E_EXPECTING_COMMA: crate::core::HRESULT = crate::core::HRESULT(-2147215759i32);
pub const QPARSE_E_EXPECTING_CURRENCY: crate::core::HRESULT = crate::core::HRESULT(-2147215772i32);
pub const QPARSE_E_EXPECTING_DATE: crate::core::HRESULT = crate::core::HRESULT(-2147215773i32);
pub const QPARSE_E_EXPECTING_EOS: crate::core::HRESULT = crate::core::HRESULT(-2147215760i32);
pub const QPARSE_E_EXPECTING_GUID: crate::core::HRESULT = crate::core::HRESULT(-2147215771i32);
pub const QPARSE_E_EXPECTING_INTEGER: crate::core::HRESULT = crate::core::HRESULT(-2147215775i32);
pub const QPARSE_E_EXPECTING_PAREN: crate::core::HRESULT = crate::core::HRESULT(-2147215769i32);
pub const QPARSE_E_EXPECTING_PHRASE: crate::core::HRESULT = crate::core::HRESULT(-2147215766i32);
pub const QPARSE_E_EXPECTING_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-2147215768i32);
pub const QPARSE_E_EXPECTING_REAL: crate::core::HRESULT = crate::core::HRESULT(-2147215774i32);
pub const QPARSE_E_EXPECTING_REGEX: crate::core::HRESULT = crate::core::HRESULT(-2147215764i32);
pub const QPARSE_E_EXPECTING_REGEX_PROPERTY: crate::core::HRESULT =
    crate::core::HRESULT(-2147215763i32);
pub const QPARSE_E_INVALID_GROUPING: crate::core::HRESULT = crate::core::HRESULT(-2147215753i32);
pub const QPARSE_E_INVALID_LITERAL: crate::core::HRESULT = crate::core::HRESULT(-2147215762i32);
pub const QPARSE_E_INVALID_QUERY: crate::core::HRESULT = crate::core::HRESULT(-2147215750i32);
pub const QPARSE_E_INVALID_RANKMETHOD: crate::core::HRESULT = crate::core::HRESULT(-2147215749i32);
pub const QPARSE_E_INVALID_SORT_ORDER: crate::core::HRESULT = crate::core::HRESULT(-2147215755i32);
pub const QPARSE_E_NOT_YET_IMPLEMENTED: crate::core::HRESULT = crate::core::HRESULT(-2147215767i32);
pub const QPARSE_E_NO_SUCH_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-2147215761i32);
pub const QPARSE_E_NO_SUCH_SORT_PROPERTY: crate::core::HRESULT =
    crate::core::HRESULT(-2147215756i32);
pub const QPARSE_E_UNEXPECTED_EOS: crate::core::HRESULT = crate::core::HRESULT(-2147215758i32);
pub const QPARSE_E_UNEXPECTED_NOT: crate::core::HRESULT = crate::core::HRESULT(-2147215776i32);
pub const QPARSE_E_UNSUPPORTED_PROPERTY_TYPE: crate::core::HRESULT =
    crate::core::HRESULT(-2147215765i32);
pub const QPARSE_E_WEIGHT_OUT_OF_RANGE: crate::core::HRESULT = crate::core::HRESULT(-2147215757i32);
pub const QPLIST_E_BAD_GUID: crate::core::HRESULT = crate::core::HRESULT(-2147215783i32);
pub const QPLIST_E_BYREF_USED_WITHOUT_PTRTYPE: crate::core::HRESULT =
    crate::core::HRESULT(-2147215778i32);
pub const QPLIST_E_CANT_OPEN_FILE: crate::core::HRESULT = crate::core::HRESULT(-2147215791i32);
pub const QPLIST_E_CANT_SET_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-2147215781i32);
pub const QPLIST_E_DUPLICATE: crate::core::HRESULT = crate::core::HRESULT(-2147215780i32);
pub const QPLIST_E_EXPECTING_CLOSE_PAREN: crate::core::HRESULT =
    crate::core::HRESULT(-2147215785i32);
pub const QPLIST_E_EXPECTING_GUID: crate::core::HRESULT = crate::core::HRESULT(-2147215784i32);
pub const QPLIST_E_EXPECTING_INTEGER: crate::core::HRESULT = crate::core::HRESULT(-2147215786i32);
pub const QPLIST_E_EXPECTING_NAME: crate::core::HRESULT = crate::core::HRESULT(-2147215789i32);
pub const QPLIST_E_EXPECTING_PROP_SPEC: crate::core::HRESULT = crate::core::HRESULT(-2147215782i32);
pub const QPLIST_E_EXPECTING_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2147215788i32);
pub const QPLIST_E_READ_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2147215790i32);
pub const QPLIST_E_UNRECOGNIZED_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2147215787i32);
pub const QPLIST_E_VECTORBYREF_USED_ALONE: crate::core::HRESULT =
    crate::core::HRESULT(-2147215779i32);
pub const QPLIST_S_DUPLICATE: crate::core::HRESULT = crate::core::HRESULT(267897i32);
pub const QUERY_E_ALLNOISE: crate::core::HRESULT = crate::core::HRESULT(-2147215867i32);
pub const QUERY_E_DIR_ON_REMOVABLE_DRIVE: crate::core::HRESULT =
    crate::core::HRESULT(-2147215861i32);
pub const QUERY_E_DUPLICATE_OUTPUT_COLUMN: crate::core::HRESULT =
    crate::core::HRESULT(-2147215864i32);
pub const QUERY_E_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2147215872i32);
pub const QUERY_E_INVALIDCATEGORIZE: crate::core::HRESULT = crate::core::HRESULT(-2147215868i32);
pub const QUERY_E_INVALIDQUERY: crate::core::HRESULT = crate::core::HRESULT(-2147215871i32);
pub const QUERY_E_INVALIDRESTRICTION: crate::core::HRESULT = crate::core::HRESULT(-2147215870i32);
pub const QUERY_E_INVALIDSORT: crate::core::HRESULT = crate::core::HRESULT(-2147215869i32);
pub const QUERY_E_INVALID_DIRECTORY: crate::core::HRESULT = crate::core::HRESULT(-2147215862i32);
pub const QUERY_E_INVALID_OUTPUT_COLUMN: crate::core::HRESULT =
    crate::core::HRESULT(-2147215863i32);
pub const QUERY_E_TIMEDOUT: crate::core::HRESULT = crate::core::HRESULT(-2147215865i32);
pub const QUERY_E_TOOCOMPLEX: crate::core::HRESULT = crate::core::HRESULT(-2147215866i32);
pub const QUERY_S_NO_QUERY: crate::core::HRESULT = crate::core::HRESULT(-2147215860i32);
pub const QUTIL_E_CANT_CONVERT_VROOT: crate::core::HRESULT = crate::core::HRESULT(-2147215754i32);
pub const QUTIL_E_INVALID_CODEPAGE: crate::core::HRESULT = crate::core::HRESULT(-1073473928i32);
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl ::core::marker::Copy for RECT {}
impl ::core::clone::Clone for RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECT")
            .field("left", &self.left)
            .field("top", &self.top)
            .field("right", &self.right)
            .field("bottom", &self.bottom)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RECT {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left
            && self.top == other.top
            && self.right == other.right
            && self.bottom == other.bottom
    }
}
impl ::core::cmp::Eq for RECT {}
impl FromIntoMemory for RECT {
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
pub struct RECTL {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl ::core::marker::Copy for RECTL {}
impl ::core::clone::Clone for RECTL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RECTL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECTL")
            .field("left", &self.left)
            .field("top", &self.top)
            .field("right", &self.right)
            .field("bottom", &self.bottom)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RECTL {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left
            && self.top == other.top
            && self.right == other.right
            && self.bottom == other.bottom
    }
}
impl ::core::cmp::Eq for RECTL {}
impl FromIntoMemory for RECTL {
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
pub const REGDB_E_BADTHREADINGMODEL: crate::core::HRESULT = crate::core::HRESULT(-2147221162i32);
pub const REGDB_E_CLASSNOTREG: crate::core::HRESULT = crate::core::HRESULT(-2147221164i32);
pub const REGDB_E_FIRST: i32 = -2147221168i32;
pub const REGDB_E_IIDNOTREG: crate::core::HRESULT = crate::core::HRESULT(-2147221163i32);
pub const REGDB_E_INVALIDVALUE: crate::core::HRESULT = crate::core::HRESULT(-2147221165i32);
pub const REGDB_E_KEYMISSING: crate::core::HRESULT = crate::core::HRESULT(-2147221166i32);
pub const REGDB_E_LAST: i32 = -2147221153i32;
pub const REGDB_E_PACKAGEPOLICYVIOLATION: crate::core::HRESULT =
    crate::core::HRESULT(-2147221161i32);
pub const REGDB_E_READREGDB: crate::core::HRESULT = crate::core::HRESULT(-2147221168i32);
pub const REGDB_E_WRITEREGDB: crate::core::HRESULT = crate::core::HRESULT(-2147221167i32);
pub const REGDB_S_FIRST: i32 = 262480i32;
pub const REGDB_S_LAST: i32 = 262495i32;
pub const ROUTEBASE: u32 = 900u32;
pub const ROUTEBASEEND: u32 = 957u32;
pub const RO_E_BLOCKED_CROSS_ASTA_CALL: crate::core::HRESULT = crate::core::HRESULT(-2147483617i32);
pub const RO_E_CANNOT_ACTIVATE_FULL_TRUST_SERVER: crate::core::HRESULT =
    crate::core::HRESULT(-2147483616i32);
pub const RO_E_CANNOT_ACTIVATE_UNIVERSAL_APPLICATION_SERVER: crate::core::HRESULT =
    crate::core::HRESULT(-2147483615i32);
pub const RO_E_CHANGE_NOTIFICATION_IN_PROGRESS: crate::core::HRESULT =
    crate::core::HRESULT(-2147483627i32);
pub const RO_E_CLOSED: crate::core::HRESULT = crate::core::HRESULT(-2147483629i32);
pub const RO_E_COMMITTED: crate::core::HRESULT = crate::core::HRESULT(-2147483618i32);
pub const RO_E_ERROR_STRING_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2147483626i32);
pub const RO_E_EXCLUSIVE_WRITE: crate::core::HRESULT = crate::core::HRESULT(-2147483628i32);
pub const RO_E_INVALID_METADATA_FILE: crate::core::HRESULT = crate::core::HRESULT(-2147483630i32);
pub const RO_E_METADATA_INVALID_TYPE_FORMAT: crate::core::HRESULT =
    crate::core::HRESULT(-2147483631i32);
pub const RO_E_METADATA_NAME_IS_NAMESPACE: crate::core::HRESULT =
    crate::core::HRESULT(-2147483632i32);
pub const RO_E_METADATA_NAME_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2147483633i32);
pub const RO_E_MUST_BE_AGILE: crate::core::HRESULT = crate::core::HRESULT(-2147483620i32);
pub const RO_E_UNSUPPORTED_FROM_MTA: crate::core::HRESULT = crate::core::HRESULT(-2147483619i32);
pub const RPC_E_ACCESS_DENIED: crate::core::HRESULT = crate::core::HRESULT(-2147417829i32);
pub const RPC_E_ATTEMPTED_MULTITHREAD: crate::core::HRESULT = crate::core::HRESULT(-2147417854i32);
pub const RPC_E_CALL_CANCELED: crate::core::HRESULT = crate::core::HRESULT(-2147418110i32);
pub const RPC_E_CALL_COMPLETE: crate::core::HRESULT = crate::core::HRESULT(-2147417833i32);
pub const RPC_E_CALL_REJECTED: crate::core::HRESULT = crate::core::HRESULT(-2147418111i32);
pub const RPC_E_CANTCALLOUT_AGAIN: crate::core::HRESULT = crate::core::HRESULT(-2147418095i32);
pub const RPC_E_CANTCALLOUT_INASYNCCALL: crate::core::HRESULT =
    crate::core::HRESULT(-2147418108i32);
pub const RPC_E_CANTCALLOUT_INEXTERNALCALL: crate::core::HRESULT =
    crate::core::HRESULT(-2147418107i32);
pub const RPC_E_CANTCALLOUT_ININPUTSYNCCALL: crate::core::HRESULT =
    crate::core::HRESULT(-2147417843i32);
pub const RPC_E_CANTPOST_INSENDCALL: crate::core::HRESULT = crate::core::HRESULT(-2147418109i32);
pub const RPC_E_CANTTRANSMIT_CALL: crate::core::HRESULT = crate::core::HRESULT(-2147418102i32);
pub const RPC_E_CHANGED_MODE: crate::core::HRESULT = crate::core::HRESULT(-2147417850i32);
pub const RPC_E_CLIENT_CANTMARSHAL_DATA: crate::core::HRESULT =
    crate::core::HRESULT(-2147418101i32);
pub const RPC_E_CLIENT_CANTUNMARSHAL_DATA: crate::core::HRESULT =
    crate::core::HRESULT(-2147418100i32);
pub const RPC_E_CLIENT_DIED: crate::core::HRESULT = crate::core::HRESULT(-2147418104i32);
pub const RPC_E_CONNECTION_TERMINATED: crate::core::HRESULT = crate::core::HRESULT(-2147418106i32);
pub const RPC_E_DISCONNECTED: crate::core::HRESULT = crate::core::HRESULT(-2147417848i32);
pub const RPC_E_FAULT: crate::core::HRESULT = crate::core::HRESULT(-2147417852i32);
pub const RPC_E_FULLSIC_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2147417823i32);
pub const RPC_E_INVALIDMETHOD: crate::core::HRESULT = crate::core::HRESULT(-2147417849i32);
pub const RPC_E_INVALID_CALLDATA: crate::core::HRESULT = crate::core::HRESULT(-2147417844i32);
pub const RPC_E_INVALID_DATA: crate::core::HRESULT = crate::core::HRESULT(-2147418097i32);
pub const RPC_E_INVALID_DATAPACKET: crate::core::HRESULT = crate::core::HRESULT(-2147418103i32);
pub const RPC_E_INVALID_EXTENSION: crate::core::HRESULT = crate::core::HRESULT(-2147417838i32);
pub const RPC_E_INVALID_HEADER: crate::core::HRESULT = crate::core::HRESULT(-2147417839i32);
pub const RPC_E_INVALID_IPID: crate::core::HRESULT = crate::core::HRESULT(-2147417837i32);
pub const RPC_E_INVALID_OBJECT: crate::core::HRESULT = crate::core::HRESULT(-2147417836i32);
pub const RPC_E_INVALID_OBJREF: crate::core::HRESULT = crate::core::HRESULT(-2147417827i32);
pub const RPC_E_INVALID_PARAMETER: crate::core::HRESULT = crate::core::HRESULT(-2147418096i32);
pub const RPC_E_INVALID_STD_NAME: crate::core::HRESULT = crate::core::HRESULT(-2147417822i32);
pub const RPC_E_NOT_REGISTERED: crate::core::HRESULT = crate::core::HRESULT(-2147417853i32);
pub const RPC_E_NO_CONTEXT: crate::core::HRESULT = crate::core::HRESULT(-2147417826i32);
pub const RPC_E_NO_GOOD_SECURITY_PACKAGES: crate::core::HRESULT =
    crate::core::HRESULT(-2147417830i32);
pub const RPC_E_NO_SYNC: crate::core::HRESULT = crate::core::HRESULT(-2147417824i32);
pub const RPC_E_OUT_OF_RESOURCES: crate::core::HRESULT = crate::core::HRESULT(-2147417855i32);
pub const RPC_E_REMOTE_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2147417828i32);
pub const RPC_E_RETRY: crate::core::HRESULT = crate::core::HRESULT(-2147417847i32);
pub const RPC_E_SERVERCALL_REJECTED: crate::core::HRESULT = crate::core::HRESULT(-2147417845i32);
pub const RPC_E_SERVERCALL_RETRYLATER: crate::core::HRESULT = crate::core::HRESULT(-2147417846i32);
pub const RPC_E_SERVERFAULT: crate::core::HRESULT = crate::core::HRESULT(-2147417851i32);
pub const RPC_E_SERVER_CANTMARSHAL_DATA: crate::core::HRESULT =
    crate::core::HRESULT(-2147418099i32);
pub const RPC_E_SERVER_CANTUNMARSHAL_DATA: crate::core::HRESULT =
    crate::core::HRESULT(-2147418098i32);
pub const RPC_E_SERVER_DIED: crate::core::HRESULT = crate::core::HRESULT(-2147418105i32);
pub const RPC_E_SERVER_DIED_DNE: crate::core::HRESULT = crate::core::HRESULT(-2147418094i32);
pub const RPC_E_SYS_CALL_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2147417856i32);
pub const RPC_E_THREAD_NOT_INIT: crate::core::HRESULT = crate::core::HRESULT(-2147417841i32);
pub const RPC_E_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2147417825i32);
pub const RPC_E_TOO_LATE: crate::core::HRESULT = crate::core::HRESULT(-2147417831i32);
pub const RPC_E_UNEXPECTED: crate::core::HRESULT = crate::core::HRESULT(-2147352577i32);
pub const RPC_E_UNSECURE_CALL: crate::core::HRESULT = crate::core::HRESULT(-2147417832i32);
pub const RPC_E_VERSION_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2147417840i32);
pub const RPC_E_WRONG_THREAD: crate::core::HRESULT = crate::core::HRESULT(-2147417842i32);
pub const RPC_NT_ADDRESS_ERROR: NTSTATUS = NTSTATUS(-1073610683i32);
pub const RPC_NT_ALREADY_LISTENING: NTSTATUS = NTSTATUS(-1073610738i32);
pub const RPC_NT_ALREADY_REGISTERED: NTSTATUS = NTSTATUS(-1073610740i32);
pub const RPC_NT_BAD_STUB_DATA: NTSTATUS = NTSTATUS(-1073545204i32);
pub const RPC_NT_BINDING_HAS_NO_AUTH: NTSTATUS = NTSTATUS(-1073610705i32);
pub const RPC_NT_BINDING_INCOMPLETE: NTSTATUS = NTSTATUS(-1073610671i32);
pub const RPC_NT_BYTE_COUNT_TOO_SMALL: NTSTATUS = NTSTATUS(-1073545205i32);
pub const RPC_NT_CALL_CANCELLED: NTSTATUS = NTSTATUS(-1073610672i32);
pub const RPC_NT_CALL_FAILED: NTSTATUS = NTSTATUS(-1073610725i32);
pub const RPC_NT_CALL_FAILED_DNE: NTSTATUS = NTSTATUS(-1073610724i32);
pub const RPC_NT_CALL_IN_PROGRESS: NTSTATUS = NTSTATUS(-1073610679i32);
pub const RPC_NT_CANNOT_SUPPORT: NTSTATUS = NTSTATUS(-1073610687i32);
pub const RPC_NT_CANT_CREATE_ENDPOINT: NTSTATUS = NTSTATUS(-1073610731i32);
pub const RPC_NT_COMM_FAILURE: NTSTATUS = NTSTATUS(-1073610670i32);
pub const RPC_NT_COOKIE_AUTH_FAILED: NTSTATUS = NTSTATUS(-1073610651i32);
pub const RPC_NT_DUPLICATE_ENDPOINT: NTSTATUS = NTSTATUS(-1073610711i32);
pub const RPC_NT_ENTRY_ALREADY_EXISTS: NTSTATUS = NTSTATUS(-1073610691i32);
pub const RPC_NT_ENTRY_NOT_FOUND: NTSTATUS = NTSTATUS(-1073610690i32);
pub const RPC_NT_ENUM_VALUE_OUT_OF_RANGE: NTSTATUS = NTSTATUS(-1073545206i32);
pub const RPC_NT_FP_DIV_ZERO: NTSTATUS = NTSTATUS(-1073610682i32);
pub const RPC_NT_FP_OVERFLOW: NTSTATUS = NTSTATUS(-1073610680i32);
pub const RPC_NT_FP_UNDERFLOW: NTSTATUS = NTSTATUS(-1073610681i32);
pub const RPC_NT_GROUP_MEMBER_NOT_FOUND: NTSTATUS = NTSTATUS(-1073610677i32);
pub const RPC_NT_INCOMPLETE_NAME: NTSTATUS = NTSTATUS(-1073610696i32);
pub const RPC_NT_INTERFACE_NOT_FOUND: NTSTATUS = NTSTATUS(-1073610692i32);
pub const RPC_NT_INTERNAL_ERROR: NTSTATUS = NTSTATUS(-1073610685i32);
pub const RPC_NT_INVALID_ASYNC_CALL: NTSTATUS = NTSTATUS(-1073610653i32);
pub const RPC_NT_INVALID_ASYNC_HANDLE: NTSTATUS = NTSTATUS(-1073610654i32);
pub const RPC_NT_INVALID_AUTH_IDENTITY: NTSTATUS = NTSTATUS(-1073610702i32);
pub const RPC_NT_INVALID_BINDING: NTSTATUS = NTSTATUS(-1073610749i32);
pub const RPC_NT_INVALID_BOUND: NTSTATUS = NTSTATUS(-1073610717i32);
pub const RPC_NT_INVALID_ENDPOINT_FORMAT: NTSTATUS = NTSTATUS(-1073610745i32);
pub const RPC_NT_INVALID_ES_ACTION: NTSTATUS = NTSTATUS(-1073545127i32);
pub const RPC_NT_INVALID_NAF_ID: NTSTATUS = NTSTATUS(-1073610688i32);
pub const RPC_NT_INVALID_NAME_SYNTAX: NTSTATUS = NTSTATUS(-1073610715i32);
pub const RPC_NT_INVALID_NETWORK_OPTIONS: NTSTATUS = NTSTATUS(-1073610727i32);
pub const RPC_NT_INVALID_NET_ADDR: NTSTATUS = NTSTATUS(-1073610744i32);
pub const RPC_NT_INVALID_OBJECT: NTSTATUS = NTSTATUS(-1073610675i32);
pub const RPC_NT_INVALID_PIPE_OBJECT: NTSTATUS = NTSTATUS(-1073545124i32);
pub const RPC_NT_INVALID_PIPE_OPERATION: NTSTATUS = NTSTATUS(-1073545123i32);
pub const RPC_NT_INVALID_RPC_PROTSEQ: NTSTATUS = NTSTATUS(-1073610747i32);
pub const RPC_NT_INVALID_STRING_BINDING: NTSTATUS = NTSTATUS(-1073610751i32);
pub const RPC_NT_INVALID_STRING_UUID: NTSTATUS = NTSTATUS(-1073610746i32);
pub const RPC_NT_INVALID_TAG: NTSTATUS = NTSTATUS(-1073610718i32);
pub const RPC_NT_INVALID_TIMEOUT: NTSTATUS = NTSTATUS(-1073610742i32);
pub const RPC_NT_INVALID_VERS_OPTION: NTSTATUS = NTSTATUS(-1073610695i32);
pub const RPC_NT_MAX_CALLS_TOO_SMALL: NTSTATUS = NTSTATUS(-1073610709i32);
pub const RPC_NT_NAME_SERVICE_UNAVAILABLE: NTSTATUS = NTSTATUS(-1073610689i32);
pub const RPC_NT_NOTHING_TO_EXPORT: NTSTATUS = NTSTATUS(-1073610697i32);
pub const RPC_NT_NOT_ALL_OBJS_UNEXPORTED: NTSTATUS = NTSTATUS(-1073610693i32);
pub const RPC_NT_NOT_CANCELLED: NTSTATUS = NTSTATUS(-1073610664i32);
pub const RPC_NT_NOT_LISTENING: NTSTATUS = NTSTATUS(-1073610736i32);
pub const RPC_NT_NOT_RPC_ERROR: NTSTATUS = NTSTATUS(-1073610667i32);
pub const RPC_NT_NO_BINDINGS: NTSTATUS = NTSTATUS(-1073610733i32);
pub const RPC_NT_NO_CALL_ACTIVE: NTSTATUS = NTSTATUS(-1073610726i32);
pub const RPC_NT_NO_CONTEXT_AVAILABLE: NTSTATUS = NTSTATUS(-1073610686i32);
pub const RPC_NT_NO_ENDPOINT_FOUND: NTSTATUS = NTSTATUS(-1073610743i32);
pub const RPC_NT_NO_ENTRY_NAME: NTSTATUS = NTSTATUS(-1073610716i32);
pub const RPC_NT_NO_INTERFACES: NTSTATUS = NTSTATUS(-1073610673i32);
pub const RPC_NT_NO_MORE_BINDINGS: NTSTATUS = NTSTATUS(-1073610678i32);
pub const RPC_NT_NO_MORE_ENTRIES: NTSTATUS = NTSTATUS(-1073545215i32);
pub const RPC_NT_NO_MORE_MEMBERS: NTSTATUS = NTSTATUS(-1073610694i32);
pub const RPC_NT_NO_PRINC_NAME: NTSTATUS = NTSTATUS(-1073610668i32);
pub const RPC_NT_NO_PROTSEQS: NTSTATUS = NTSTATUS(-1073610732i32);
pub const RPC_NT_NO_PROTSEQS_REGISTERED: NTSTATUS = NTSTATUS(-1073610737i32);
pub const RPC_NT_NULL_REF_POINTER: NTSTATUS = NTSTATUS(-1073545207i32);
pub const RPC_NT_OBJECT_NOT_FOUND: NTSTATUS = NTSTATUS(-1073610741i32);
pub const RPC_NT_OUT_OF_RESOURCES: NTSTATUS = NTSTATUS(-1073610730i32);
pub const RPC_NT_PIPE_CLOSED: NTSTATUS = NTSTATUS(-1073545121i32);
pub const RPC_NT_PIPE_DISCIPLINE_ERROR: NTSTATUS = NTSTATUS(-1073545120i32);
pub const RPC_NT_PIPE_EMPTY: NTSTATUS = NTSTATUS(-1073545119i32);
pub const RPC_NT_PROCNUM_OUT_OF_RANGE: NTSTATUS = NTSTATUS(-1073610706i32);
pub const RPC_NT_PROTOCOL_ERROR: NTSTATUS = NTSTATUS(-1073610723i32);
pub const RPC_NT_PROTSEQ_NOT_FOUND: NTSTATUS = NTSTATUS(-1073610707i32);
pub const RPC_NT_PROTSEQ_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073610748i32);
pub const RPC_NT_PROXY_ACCESS_DENIED: NTSTATUS = NTSTATUS(-1073610652i32);
pub const RPC_NT_SEC_PKG_ERROR: NTSTATUS = NTSTATUS(-1073610665i32);
pub const RPC_NT_SEND_INCOMPLETE: NTSTATUS = NTSTATUS(1073873071i32);
pub const RPC_NT_SERVER_TOO_BUSY: NTSTATUS = NTSTATUS(-1073610728i32);
pub const RPC_NT_SERVER_UNAVAILABLE: NTSTATUS = NTSTATUS(-1073610729i32);
pub const RPC_NT_SS_CANNOT_GET_CALL_HANDLE: NTSTATUS = NTSTATUS(-1073545208i32);
pub const RPC_NT_SS_CHAR_TRANS_OPEN_FAIL: NTSTATUS = NTSTATUS(-1073545214i32);
pub const RPC_NT_SS_CHAR_TRANS_SHORT_FILE: NTSTATUS = NTSTATUS(-1073545213i32);
pub const RPC_NT_SS_CONTEXT_DAMAGED: NTSTATUS = NTSTATUS(-1073545210i32);
pub const RPC_NT_SS_CONTEXT_MISMATCH: NTSTATUS = NTSTATUS(-1073545211i32);
pub const RPC_NT_SS_HANDLES_MISMATCH: NTSTATUS = NTSTATUS(-1073545209i32);
pub const RPC_NT_SS_IN_NULL_CONTEXT: NTSTATUS = NTSTATUS(-1073545212i32);
pub const RPC_NT_STRING_TOO_LONG: NTSTATUS = NTSTATUS(-1073610708i32);
pub const RPC_NT_TYPE_ALREADY_REGISTERED: NTSTATUS = NTSTATUS(-1073610739i32);
pub const RPC_NT_UNKNOWN_AUTHN_LEVEL: NTSTATUS = NTSTATUS(-1073610703i32);
pub const RPC_NT_UNKNOWN_AUTHN_SERVICE: NTSTATUS = NTSTATUS(-1073610704i32);
pub const RPC_NT_UNKNOWN_AUTHN_TYPE: NTSTATUS = NTSTATUS(-1073610710i32);
pub const RPC_NT_UNKNOWN_AUTHZ_SERVICE: NTSTATUS = NTSTATUS(-1073610701i32);
pub const RPC_NT_UNKNOWN_IF: NTSTATUS = NTSTATUS(-1073610734i32);
pub const RPC_NT_UNKNOWN_MGR_TYPE: NTSTATUS = NTSTATUS(-1073610735i32);
pub const RPC_NT_UNSUPPORTED_AUTHN_LEVEL: NTSTATUS = NTSTATUS(-1073610669i32);
pub const RPC_NT_UNSUPPORTED_NAME_SYNTAX: NTSTATUS = NTSTATUS(-1073610714i32);
pub const RPC_NT_UNSUPPORTED_TRANS_SYN: NTSTATUS = NTSTATUS(-1073610721i32);
pub const RPC_NT_UNSUPPORTED_TYPE: NTSTATUS = NTSTATUS(-1073610719i32);
pub const RPC_NT_UUID_LOCAL_ONLY: NTSTATUS = NTSTATUS(1073872982i32);
pub const RPC_NT_UUID_NO_ADDRESS: NTSTATUS = NTSTATUS(-1073610712i32);
pub const RPC_NT_WRONG_ES_VERSION: NTSTATUS = NTSTATUS(-1073545126i32);
pub const RPC_NT_WRONG_KIND_OF_BINDING: NTSTATUS = NTSTATUS(-1073610750i32);
pub const RPC_NT_WRONG_PIPE_VERSION: NTSTATUS = NTSTATUS(-1073545122i32);
pub const RPC_NT_WRONG_STUB_VERSION: NTSTATUS = NTSTATUS(-1073545125i32);
pub const RPC_NT_ZERO_DIVIDE: NTSTATUS = NTSTATUS(-1073610684i32);
pub const RPC_S_CALLPENDING: crate::core::HRESULT = crate::core::HRESULT(-2147417835i32);
pub const RPC_S_WAITONTIMER: crate::core::HRESULT = crate::core::HRESULT(-2147417834i32);
pub const RPC_X_BAD_STUB_DATA: i32 = 1783i32;
pub const RPC_X_BYTE_COUNT_TOO_SMALL: i32 = 1782i32;
pub const RPC_X_ENUM_VALUE_OUT_OF_RANGE: i32 = 1781i32;
pub const RPC_X_INVALID_ES_ACTION: i32 = 1827i32;
pub const RPC_X_INVALID_PIPE_OBJECT: i32 = 1830i32;
pub const RPC_X_NO_MORE_ENTRIES: i32 = 1772i32;
pub const RPC_X_NULL_REF_POINTER: i32 = 1780i32;
pub const RPC_X_PIPE_CLOSED: i32 = 1916i32;
pub const RPC_X_PIPE_DISCIPLINE_ERROR: i32 = 1917i32;
pub const RPC_X_PIPE_EMPTY: i32 = 1918i32;
pub const RPC_X_SS_CANNOT_GET_CALL_HANDLE: i32 = 1779i32;
pub const RPC_X_SS_CHAR_TRANS_OPEN_FAIL: i32 = 1773i32;
pub const RPC_X_SS_CHAR_TRANS_SHORT_FILE: i32 = 1774i32;
pub const RPC_X_SS_CONTEXT_DAMAGED: i32 = 1777i32;
pub const RPC_X_SS_HANDLES_MISMATCH: i32 = 1778i32;
pub const RPC_X_SS_IN_NULL_CONTEXT: i32 = 1775i32;
pub const RPC_X_WRONG_ES_VERSION: i32 = 1828i32;
pub const RPC_X_WRONG_PIPE_ORDER: i32 = 1831i32;
pub const RPC_X_WRONG_PIPE_VERSION: i32 = 1832i32;
pub const RPC_X_WRONG_STUB_VERSION: i32 = 1829i32;
pub const SCARD_E_BAD_SEEK: crate::core::HRESULT = crate::core::HRESULT(-2146435031i32);
pub const SCARD_E_CANCELLED: crate::core::HRESULT = crate::core::HRESULT(-2146435070i32);
pub const SCARD_E_CANT_DISPOSE: crate::core::HRESULT = crate::core::HRESULT(-2146435058i32);
pub const SCARD_E_CARD_UNSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2146435044i32);
pub const SCARD_E_CERTIFICATE_UNAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2146435027i32);
pub const SCARD_E_COMM_DATA_LOST: crate::core::HRESULT = crate::core::HRESULT(-2146435025i32);
pub const SCARD_E_DIR_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2146435037i32);
pub const SCARD_E_DUPLICATE_READER: crate::core::HRESULT = crate::core::HRESULT(-2146435045i32);
pub const SCARD_E_FILE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2146435036i32);
pub const SCARD_E_ICC_CREATEORDER: crate::core::HRESULT = crate::core::HRESULT(-2146435039i32);
pub const SCARD_E_ICC_INSTALLATION: crate::core::HRESULT = crate::core::HRESULT(-2146435040i32);
pub const SCARD_E_INSUFFICIENT_BUFFER: crate::core::HRESULT = crate::core::HRESULT(-2146435064i32);
pub const SCARD_E_INVALID_ATR: crate::core::HRESULT = crate::core::HRESULT(-2146435051i32);
pub const SCARD_E_INVALID_CHV: crate::core::HRESULT = crate::core::HRESULT(-2146435030i32);
pub const SCARD_E_INVALID_HANDLE: crate::core::HRESULT = crate::core::HRESULT(-2146435069i32);
pub const SCARD_E_INVALID_PARAMETER: crate::core::HRESULT = crate::core::HRESULT(-2146435068i32);
pub const SCARD_E_INVALID_TARGET: crate::core::HRESULT = crate::core::HRESULT(-2146435067i32);
pub const SCARD_E_INVALID_VALUE: crate::core::HRESULT = crate::core::HRESULT(-2146435055i32);
pub const SCARD_E_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-2146435056i32);
pub const SCARD_E_NOT_TRANSACTED: crate::core::HRESULT = crate::core::HRESULT(-2146435050i32);
pub const SCARD_E_NO_ACCESS: crate::core::HRESULT = crate::core::HRESULT(-2146435033i32);
pub const SCARD_E_NO_DIR: crate::core::HRESULT = crate::core::HRESULT(-2146435035i32);
pub const SCARD_E_NO_FILE: crate::core::HRESULT = crate::core::HRESULT(-2146435034i32);
pub const SCARD_E_NO_KEY_CONTAINER: crate::core::HRESULT = crate::core::HRESULT(-2146435024i32);
pub const SCARD_E_NO_MEMORY: crate::core::HRESULT = crate::core::HRESULT(-2146435066i32);
pub const SCARD_E_NO_PIN_CACHE: crate::core::HRESULT = crate::core::HRESULT(-2146435021i32);
pub const SCARD_E_NO_READERS_AVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2146435026i32);
pub const SCARD_E_NO_SERVICE: crate::core::HRESULT = crate::core::HRESULT(-2146435043i32);
pub const SCARD_E_NO_SMARTCARD: crate::core::HRESULT = crate::core::HRESULT(-2146435060i32);
pub const SCARD_E_NO_SUCH_CERTIFICATE: crate::core::HRESULT = crate::core::HRESULT(-2146435028i32);
pub const SCARD_E_PCI_TOO_SMALL: crate::core::HRESULT = crate::core::HRESULT(-2146435047i32);
pub const SCARD_E_PIN_CACHE_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-2146435022i32);
pub const SCARD_E_PROTO_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2146435057i32);
pub const SCARD_E_READER_UNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2146435049i32);
pub const SCARD_E_READER_UNSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2146435046i32);
pub const SCARD_E_READ_ONLY_CARD: crate::core::HRESULT = crate::core::HRESULT(-2146435020i32);
pub const SCARD_E_SERVER_TOO_BUSY: crate::core::HRESULT = crate::core::HRESULT(-2146435023i32);
pub const SCARD_E_SERVICE_STOPPED: crate::core::HRESULT = crate::core::HRESULT(-2146435042i32);
pub const SCARD_E_SHARING_VIOLATION: crate::core::HRESULT = crate::core::HRESULT(-2146435061i32);
pub const SCARD_E_SYSTEM_CANCELLED: crate::core::HRESULT = crate::core::HRESULT(-2146435054i32);
pub const SCARD_E_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2146435062i32);
pub const SCARD_E_UNEXPECTED: crate::core::HRESULT = crate::core::HRESULT(-2146435041i32);
pub const SCARD_E_UNKNOWN_CARD: crate::core::HRESULT = crate::core::HRESULT(-2146435059i32);
pub const SCARD_E_UNKNOWN_READER: crate::core::HRESULT = crate::core::HRESULT(-2146435063i32);
pub const SCARD_E_UNKNOWN_RES_MNG: crate::core::HRESULT = crate::core::HRESULT(-2146435029i32);
pub const SCARD_E_UNSUPPORTED_FEATURE: crate::core::HRESULT = crate::core::HRESULT(-2146435038i32);
pub const SCARD_E_WRITE_TOO_MANY: crate::core::HRESULT = crate::core::HRESULT(-2146435032i32);
pub const SCARD_F_COMM_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146435053i32);
pub const SCARD_F_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146435071i32);
pub const SCARD_F_UNKNOWN_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146435052i32);
pub const SCARD_F_WAITED_TOO_LONG: crate::core::HRESULT = crate::core::HRESULT(-2146435065i32);
pub const SCARD_P_SHUTDOWN: crate::core::HRESULT = crate::core::HRESULT(-2146435048i32);
pub const SCARD_W_CACHE_ITEM_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2146434960i32);
pub const SCARD_W_CACHE_ITEM_STALE: crate::core::HRESULT = crate::core::HRESULT(-2146434959i32);
pub const SCARD_W_CACHE_ITEM_TOO_BIG: crate::core::HRESULT = crate::core::HRESULT(-2146434958i32);
pub const SCARD_W_CANCELLED_BY_USER: crate::core::HRESULT = crate::core::HRESULT(-2146434962i32);
pub const SCARD_W_CARD_NOT_AUTHENTICATED: crate::core::HRESULT =
    crate::core::HRESULT(-2146434961i32);
pub const SCARD_W_CHV_BLOCKED: crate::core::HRESULT = crate::core::HRESULT(-2146434964i32);
pub const SCARD_W_EOF: crate::core::HRESULT = crate::core::HRESULT(-2146434963i32);
pub const SCARD_W_REMOVED_CARD: crate::core::HRESULT = crate::core::HRESULT(-2146434967i32);
pub const SCARD_W_RESET_CARD: crate::core::HRESULT = crate::core::HRESULT(-2146434968i32);
pub const SCARD_W_SECURITY_VIOLATION: crate::core::HRESULT = crate::core::HRESULT(-2146434966i32);
pub const SCARD_W_UNPOWERED_CARD: crate::core::HRESULT = crate::core::HRESULT(-2146434969i32);
pub const SCARD_W_UNRESPONSIVE_CARD: crate::core::HRESULT = crate::core::HRESULT(-2146434970i32);
pub const SCARD_W_UNSUPPORTED_CARD: crate::core::HRESULT = crate::core::HRESULT(-2146434971i32);
pub const SCARD_W_WRONG_CHV: crate::core::HRESULT = crate::core::HRESULT(-2146434965i32);
pub const SCHED_E_ACCOUNT_DBASE_CORRUPT: crate::core::HRESULT =
    crate::core::HRESULT(-2147216623i32);
pub const SCHED_E_ACCOUNT_INFORMATION_NOT_SET: crate::core::HRESULT =
    crate::core::HRESULT(-2147216625i32);
pub const SCHED_E_ACCOUNT_NAME_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2147216624i32);
pub const SCHED_E_ALREADY_RUNNING: crate::core::HRESULT = crate::core::HRESULT(-2147216609i32);
pub const SCHED_E_CANNOT_OPEN_TASK: crate::core::HRESULT = crate::core::HRESULT(-2147216627i32);
pub const SCHED_E_DEPRECATED_FEATURE_USED: crate::core::HRESULT =
    crate::core::HRESULT(-2147216592i32);
pub const SCHED_E_INVALIDVALUE: crate::core::HRESULT = crate::core::HRESULT(-2147216616i32);
pub const SCHED_E_INVALID_TASK: crate::core::HRESULT = crate::core::HRESULT(-2147216626i32);
pub const SCHED_E_INVALID_TASK_HASH: crate::core::HRESULT = crate::core::HRESULT(-2147216607i32);
pub const SCHED_E_MALFORMEDXML: crate::core::HRESULT = crate::core::HRESULT(-2147216614i32);
pub const SCHED_E_MISSINGNODE: crate::core::HRESULT = crate::core::HRESULT(-2147216615i32);
pub const SCHED_E_NAMESPACE: crate::core::HRESULT = crate::core::HRESULT(-2147216617i32);
pub const SCHED_E_NO_SECURITY_SERVICES: crate::core::HRESULT = crate::core::HRESULT(-2147216622i32);
pub const SCHED_E_PAST_END_BOUNDARY: crate::core::HRESULT = crate::core::HRESULT(-2147216610i32);
pub const SCHED_E_SERVICE_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2147216606i32);
pub const SCHED_E_SERVICE_NOT_INSTALLED: crate::core::HRESULT =
    crate::core::HRESULT(-2147216628i32);
pub const SCHED_E_SERVICE_NOT_LOCALSYSTEM: i32 = 6200i32;
pub const SCHED_E_SERVICE_NOT_RUNNING: crate::core::HRESULT = crate::core::HRESULT(-2147216619i32);
pub const SCHED_E_SERVICE_TOO_BUSY: crate::core::HRESULT = crate::core::HRESULT(-2147216605i32);
pub const SCHED_E_START_ON_DEMAND: crate::core::HRESULT = crate::core::HRESULT(-2147216600i32);
pub const SCHED_E_TASK_ATTEMPTED: crate::core::HRESULT = crate::core::HRESULT(-2147216604i32);
pub const SCHED_E_TASK_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2147216602i32);
pub const SCHED_E_TASK_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-2147216630i32);
pub const SCHED_E_TASK_NOT_RUNNING: crate::core::HRESULT = crate::core::HRESULT(-2147216629i32);
pub const SCHED_E_TASK_NOT_UBPM_COMPAT: crate::core::HRESULT = crate::core::HRESULT(-2147216599i32);
pub const SCHED_E_TASK_NOT_V1_COMPAT: crate::core::HRESULT = crate::core::HRESULT(-2147216601i32);
pub const SCHED_E_TOO_MANY_NODES: crate::core::HRESULT = crate::core::HRESULT(-2147216611i32);
pub const SCHED_E_TRIGGER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2147216631i32);
pub const SCHED_E_UNEXPECTEDNODE: crate::core::HRESULT = crate::core::HRESULT(-2147216618i32);
pub const SCHED_E_UNKNOWN_OBJECT_VERSION: crate::core::HRESULT =
    crate::core::HRESULT(-2147216621i32);
pub const SCHED_E_UNSUPPORTED_ACCOUNT_OPTION: crate::core::HRESULT =
    crate::core::HRESULT(-2147216620i32);
pub const SCHED_E_USER_NOT_LOGGED_ON: crate::core::HRESULT = crate::core::HRESULT(-2147216608i32);
pub const SCHED_S_BATCH_LOGON_PROBLEM: crate::core::HRESULT = crate::core::HRESULT(267036i32);
pub const SCHED_S_EVENT_TRIGGER: crate::core::HRESULT = crate::core::HRESULT(267016i32);
pub const SCHED_S_SOME_TRIGGERS_FAILED: crate::core::HRESULT = crate::core::HRESULT(267035i32);
pub const SCHED_S_TASK_DISABLED: crate::core::HRESULT = crate::core::HRESULT(267010i32);
pub const SCHED_S_TASK_HAS_NOT_RUN: crate::core::HRESULT = crate::core::HRESULT(267011i32);
pub const SCHED_S_TASK_NOT_SCHEDULED: crate::core::HRESULT = crate::core::HRESULT(267013i32);
pub const SCHED_S_TASK_NO_MORE_RUNS: crate::core::HRESULT = crate::core::HRESULT(267012i32);
pub const SCHED_S_TASK_NO_VALID_TRIGGERS: crate::core::HRESULT = crate::core::HRESULT(267015i32);
pub const SCHED_S_TASK_QUEUED: crate::core::HRESULT = crate::core::HRESULT(267045i32);
pub const SCHED_S_TASK_READY: crate::core::HRESULT = crate::core::HRESULT(267008i32);
pub const SCHED_S_TASK_RUNNING: crate::core::HRESULT = crate::core::HRESULT(267009i32);
pub const SCHED_S_TASK_TERMINATED: crate::core::HRESULT = crate::core::HRESULT(267014i32);
pub const SDIAG_E_CANCELLED: i32 = -2143551232i32;
pub const SDIAG_E_CANNOTRUN: i32 = -2143551224i32;
pub const SDIAG_E_DISABLED: i32 = -2143551226i32;
pub const SDIAG_E_MANAGEDHOST: i32 = -2143551229i32;
pub const SDIAG_E_NOVERIFIER: i32 = -2143551228i32;
pub const SDIAG_E_POWERSHELL: i32 = -2143551230i32;
pub const SDIAG_E_RESOURCE: i32 = -2143551222i32;
pub const SDIAG_E_ROOTCAUSE: i32 = -2143551221i32;
pub const SDIAG_E_SCRIPT: i32 = -2143551231i32;
pub const SDIAG_E_TRUST: i32 = -2143551225i32;
pub const SDIAG_E_VERSION: i32 = -2143551223i32;
pub const SDIAG_S_CANNOTRUN: i32 = 3932421i32;
pub const SEARCH_E_NOMONIKER: crate::core::HRESULT = crate::core::HRESULT(-2147215711i32);
pub const SEARCH_E_NOREGION: crate::core::HRESULT = crate::core::HRESULT(-2147215710i32);
pub const SEARCH_S_NOMOREHITS: crate::core::HRESULT = crate::core::HRESULT(267936i32);
pub const SEC_E_ALGORITHM_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2146893007i32);
pub const SEC_E_APPLICATION_PROTOCOL_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2146892953i32);
pub const SEC_E_BAD_BINDINGS: crate::core::HRESULT = crate::core::HRESULT(-2146892986i32);
pub const SEC_E_BAD_PKGID: crate::core::HRESULT = crate::core::HRESULT(-2146893034i32);
pub const SEC_E_BUFFER_TOO_SMALL: crate::core::HRESULT = crate::core::HRESULT(-2146893023i32);
pub const SEC_E_CANNOT_INSTALL: crate::core::HRESULT = crate::core::HRESULT(-2146893049i32);
pub const SEC_E_CANNOT_PACK: crate::core::HRESULT = crate::core::HRESULT(-2146893047i32);
pub const SEC_E_CERT_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-2146893016i32);
pub const SEC_E_CERT_UNKNOWN: crate::core::HRESULT = crate::core::HRESULT(-2146893017i32);
pub const SEC_E_CERT_WRONG_USAGE: crate::core::HRESULT = crate::core::HRESULT(-2146892983i32);
pub const SEC_E_CONTEXT_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-2146893033i32);
pub const SEC_E_CROSSREALM_DELEGATION_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-2146892969i32);
pub const SEC_E_CRYPTO_SYSTEM_INVALID: crate::core::HRESULT = crate::core::HRESULT(-2146893001i32);
pub const SEC_E_DECRYPT_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2146893008i32);
pub const SEC_E_DELEGATION_POLICY: crate::core::HRESULT = crate::core::HRESULT(-2146892962i32);
pub const SEC_E_DELEGATION_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2146892987i32);
pub const SEC_E_DOWNGRADE_DETECTED: crate::core::HRESULT = crate::core::HRESULT(-2146892976i32);
pub const SEC_E_ENCRYPT_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2146893015i32);
pub const SEC_E_EXT_BUFFER_TOO_SMALL: crate::core::HRESULT = crate::core::HRESULT(-2146892950i32);
pub const SEC_E_ILLEGAL_MESSAGE: crate::core::HRESULT = crate::core::HRESULT(-2146893018i32);
pub const SEC_E_INCOMPLETE_CREDENTIALS: crate::core::HRESULT = crate::core::HRESULT(-2146893024i32);
pub const SEC_E_INCOMPLETE_MESSAGE: crate::core::HRESULT = crate::core::HRESULT(-2146893032i32);
pub const SEC_E_INSUFFICIENT_BUFFERS: crate::core::HRESULT = crate::core::HRESULT(-2146892949i32);
pub const SEC_E_INSUFFICIENT_MEMORY: crate::core::HRESULT = crate::core::HRESULT(-2146893056i32);
pub const SEC_E_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146893052i32);
pub const SEC_E_INVALID_HANDLE: crate::core::HRESULT = crate::core::HRESULT(-2146893055i32);
pub const SEC_E_INVALID_PARAMETER: crate::core::HRESULT = crate::core::HRESULT(-2146892963i32);
pub const SEC_E_INVALID_TOKEN: crate::core::HRESULT = crate::core::HRESULT(-2146893048i32);
pub const SEC_E_INVALID_UPN_NAME: crate::core::HRESULT = crate::core::HRESULT(-2146892951i32);
pub const SEC_E_ISSUING_CA_UNTRUSTED: crate::core::HRESULT = crate::core::HRESULT(-2146892974i32);
pub const SEC_E_ISSUING_CA_UNTRUSTED_KDC: crate::core::HRESULT =
    crate::core::HRESULT(-2146892967i32);
pub const SEC_E_KDC_CERT_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-2146892966i32);
pub const SEC_E_KDC_CERT_REVOKED: crate::core::HRESULT = crate::core::HRESULT(-2146892965i32);
pub const SEC_E_KDC_INVALID_REQUEST: crate::core::HRESULT = crate::core::HRESULT(-2146892992i32);
pub const SEC_E_KDC_UNABLE_TO_REFER: crate::core::HRESULT = crate::core::HRESULT(-2146892991i32);
pub const SEC_E_KDC_UNKNOWN_ETYPE: crate::core::HRESULT = crate::core::HRESULT(-2146892990i32);
pub const SEC_E_LOGON_DENIED: crate::core::HRESULT = crate::core::HRESULT(-2146893044i32);
pub const SEC_E_MAX_REFERRALS_EXCEEDED: crate::core::HRESULT = crate::core::HRESULT(-2146893000i32);
pub const SEC_E_MESSAGE_ALTERED: crate::core::HRESULT = crate::core::HRESULT(-2146893041i32);
pub const SEC_E_MULTIPLE_ACCOUNTS: crate::core::HRESULT = crate::core::HRESULT(-2146892985i32);
pub const SEC_E_MUST_BE_KDC: crate::core::HRESULT = crate::core::HRESULT(-2146892999i32);
pub const SEC_E_MUTUAL_AUTH_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2146892957i32);
pub const SEC_E_NOT_OWNER: crate::core::HRESULT = crate::core::HRESULT(-2146893050i32);
pub const SEC_E_NOT_SUPPORTED: i32 = -2146893054i32;
pub const SEC_E_NO_AUTHENTICATING_AUTHORITY: crate::core::HRESULT =
    crate::core::HRESULT(-2146893039i32);
pub const SEC_E_NO_CONTEXT: crate::core::HRESULT = crate::core::HRESULT(-2146892959i32);
pub const SEC_E_NO_CREDENTIALS: crate::core::HRESULT = crate::core::HRESULT(-2146893042i32);
pub const SEC_E_NO_IMPERSONATION: crate::core::HRESULT = crate::core::HRESULT(-2146893045i32);
pub const SEC_E_NO_IP_ADDRESSES: crate::core::HRESULT = crate::core::HRESULT(-2146893003i32);
pub const SEC_E_NO_KERB_KEY: crate::core::HRESULT = crate::core::HRESULT(-2146892984i32);
pub const SEC_E_NO_PA_DATA: crate::core::HRESULT = crate::core::HRESULT(-2146892996i32);
pub const SEC_E_NO_S4U_PROT_SUPPORT: crate::core::HRESULT = crate::core::HRESULT(-2146892970i32);
pub const SEC_E_NO_SPM: i32 = -2146893052i32;
pub const SEC_E_NO_TGT_REPLY: crate::core::HRESULT = crate::core::HRESULT(-2146893004i32);
pub const SEC_E_OK: crate::core::HRESULT = crate::core::HRESULT(0i32);
pub const SEC_E_ONLY_HTTPS_ALLOWED: crate::core::HRESULT = crate::core::HRESULT(-2146892955i32);
pub const SEC_E_OUT_OF_SEQUENCE: crate::core::HRESULT = crate::core::HRESULT(-2146893040i32);
pub const SEC_E_PKINIT_CLIENT_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2146892972i32);
pub const SEC_E_PKINIT_NAME_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2146892995i32);
pub const SEC_E_PKU2U_CERT_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2146892958i32);
pub const SEC_E_POLICY_NLTM_ONLY: crate::core::HRESULT = crate::core::HRESULT(-2146892961i32);
pub const SEC_E_QOP_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2146893046i32);
pub const SEC_E_REVOCATION_OFFLINE_C: crate::core::HRESULT = crate::core::HRESULT(-2146892973i32);
pub const SEC_E_REVOCATION_OFFLINE_KDC: crate::core::HRESULT = crate::core::HRESULT(-2146892968i32);
pub const SEC_E_SECPKG_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2146893051i32);
pub const SEC_E_SECURITY_QOS_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2146893006i32);
pub const SEC_E_SHUTDOWN_IN_PROGRESS: crate::core::HRESULT = crate::core::HRESULT(-2146892993i32);
pub const SEC_E_SMARTCARD_CERT_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-2146892971i32);
pub const SEC_E_SMARTCARD_CERT_REVOKED: crate::core::HRESULT = crate::core::HRESULT(-2146892975i32);
pub const SEC_E_SMARTCARD_LOGON_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2146892994i32);
pub const SEC_E_STRONG_CRYPTO_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2146892998i32);
pub const SEC_E_TARGET_UNKNOWN: crate::core::HRESULT = crate::core::HRESULT(-2146893053i32);
pub const SEC_E_TIME_SKEW: crate::core::HRESULT = crate::core::HRESULT(-2146893020i32);
pub const SEC_E_TOO_MANY_PRINCIPALS: crate::core::HRESULT = crate::core::HRESULT(-2146892997i32);
pub const SEC_E_UNFINISHED_CONTEXT_DELETED: crate::core::HRESULT =
    crate::core::HRESULT(-2146893005i32);
pub const SEC_E_UNKNOWN_CREDENTIALS: crate::core::HRESULT = crate::core::HRESULT(-2146893043i32);
pub const SEC_E_UNSUPPORTED_FUNCTION: crate::core::HRESULT = crate::core::HRESULT(-2146893054i32);
pub const SEC_E_UNSUPPORTED_PREAUTH: crate::core::HRESULT = crate::core::HRESULT(-2146892989i32);
pub const SEC_E_UNTRUSTED_ROOT: crate::core::HRESULT = crate::core::HRESULT(-2146893019i32);
pub const SEC_E_WRONG_CREDENTIAL_HANDLE: crate::core::HRESULT =
    crate::core::HRESULT(-2146893002i32);
pub const SEC_E_WRONG_PRINCIPAL: crate::core::HRESULT = crate::core::HRESULT(-2146893022i32);
pub const SEC_I_ASYNC_CALL_PENDING: crate::core::HRESULT = crate::core::HRESULT(590696i32);
pub const SEC_I_COMPLETE_AND_CONTINUE: crate::core::HRESULT = crate::core::HRESULT(590612i32);
pub const SEC_I_COMPLETE_NEEDED: crate::core::HRESULT = crate::core::HRESULT(590611i32);
pub const SEC_I_CONTEXT_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(590615i32);
pub const SEC_I_CONTINUE_NEEDED: crate::core::HRESULT = crate::core::HRESULT(590610i32);
pub const SEC_I_CONTINUE_NEEDED_MESSAGE_OK: crate::core::HRESULT = crate::core::HRESULT(590694i32);
pub const SEC_I_GENERIC_EXTENSION_RECEIVED: crate::core::HRESULT = crate::core::HRESULT(590614i32);
pub const SEC_I_INCOMPLETE_CREDENTIALS: crate::core::HRESULT = crate::core::HRESULT(590624i32);
pub const SEC_I_LOCAL_LOGON: crate::core::HRESULT = crate::core::HRESULT(590613i32);
pub const SEC_I_MESSAGE_FRAGMENT: crate::core::HRESULT = crate::core::HRESULT(590692i32);
pub const SEC_I_NO_LSA_CONTEXT: crate::core::HRESULT = crate::core::HRESULT(590627i32);
pub const SEC_I_NO_RENEGOTIATION: crate::core::HRESULT = crate::core::HRESULT(590688i32);
pub const SEC_I_RENEGOTIATE: crate::core::HRESULT = crate::core::HRESULT(590625i32);
pub const SEC_I_SIGNATURE_NEEDED: crate::core::HRESULT = crate::core::HRESULT(590684i32);
pub const SEVERITY_ERROR: u32 = 1u32;
pub const SEVERITY_SUCCESS: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SHANDLE_PTR(pub PtrDiffRepr);
impl SHANDLE_PTR {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHANDLE_PTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for SHANDLE_PTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for SHANDLE_PTR {}
impl ::core::fmt::Debug for SHANDLE_PTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHANDLE_PTR").field(&self.0).finish()
    }
}
impl FromIntoMemory for SHANDLE_PTR {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrDiffRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrDiffRepr>()
    }
}
pub struct SIZE {
    pub cx: i32,
    pub cy: i32,
}
impl ::core::marker::Copy for SIZE {}
impl ::core::clone::Clone for SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SIZE")
            .field("cx", &self.cx)
            .field("cy", &self.cy)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.cx == other.cx && self.cy == other.cy
    }
}
impl ::core::cmp::Eq for SIZE {}
impl FromIntoMemory for SIZE {
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
pub const SPAPI_E_AUTHENTICODE_DISALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2146500032i32);
pub const SPAPI_E_AUTHENTICODE_PUBLISHER_NOT_TRUSTED: crate::core::HRESULT =
    crate::core::HRESULT(-2146500029i32);
pub const SPAPI_E_AUTHENTICODE_TRUSTED_PUBLISHER: crate::core::HRESULT =
    crate::core::HRESULT(-2146500031i32);
pub const SPAPI_E_AUTHENTICODE_TRUST_NOT_ESTABLISHED: crate::core::HRESULT =
    crate::core::HRESULT(-2146500030i32);
pub const SPAPI_E_BAD_INTERFACE_INSTALLSECT: crate::core::HRESULT =
    crate::core::HRESULT(-2146500067i32);
pub const SPAPI_E_BAD_SECTION_NAME_LINE: crate::core::HRESULT =
    crate::core::HRESULT(-2146500607i32);
pub const SPAPI_E_BAD_SERVICE_INSTALLSECT: crate::core::HRESULT =
    crate::core::HRESULT(-2146500073i32);
pub const SPAPI_E_CANT_LOAD_CLASS_ICON: crate::core::HRESULT = crate::core::HRESULT(-2146500084i32);
pub const SPAPI_E_CANT_REMOVE_DEVINST: crate::core::HRESULT = crate::core::HRESULT(-2146500046i32);
pub const SPAPI_E_CLASS_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2146500095i32);
pub const SPAPI_E_DEVICE_INSTALLER_NOT_READY: crate::core::HRESULT =
    crate::core::HRESULT(-2146500026i32);
pub const SPAPI_E_DEVICE_INSTALL_BLOCKED: crate::core::HRESULT =
    crate::core::HRESULT(-2146500024i32);
pub const SPAPI_E_DEVICE_INTERFACE_ACTIVE: crate::core::HRESULT =
    crate::core::HRESULT(-2146500069i32);
pub const SPAPI_E_DEVICE_INTERFACE_REMOVED: crate::core::HRESULT =
    crate::core::HRESULT(-2146500068i32);
pub const SPAPI_E_DEVINFO_DATA_LOCKED: crate::core::HRESULT = crate::core::HRESULT(-2146500077i32);
pub const SPAPI_E_DEVINFO_LIST_LOCKED: crate::core::HRESULT = crate::core::HRESULT(-2146500078i32);
pub const SPAPI_E_DEVINFO_NOT_REGISTERED: crate::core::HRESULT =
    crate::core::HRESULT(-2146500088i32);
pub const SPAPI_E_DEVINSTALL_QUEUE_NONNATIVE: crate::core::HRESULT =
    crate::core::HRESULT(-2146500048i32);
pub const SPAPI_E_DEVINST_ALREADY_EXISTS: crate::core::HRESULT =
    crate::core::HRESULT(-2146500089i32);
pub const SPAPI_E_DI_BAD_PATH: crate::core::HRESULT = crate::core::HRESULT(-2146500076i32);
pub const SPAPI_E_DI_DONT_INSTALL: crate::core::HRESULT = crate::core::HRESULT(-2146500053i32);
pub const SPAPI_E_DI_DO_DEFAULT: crate::core::HRESULT = crate::core::HRESULT(-2146500082i32);
pub const SPAPI_E_DI_FUNCTION_OBSOLETE: crate::core::HRESULT = crate::core::HRESULT(-2146500034i32);
pub const SPAPI_E_DI_NOFILECOPY: crate::core::HRESULT = crate::core::HRESULT(-2146500081i32);
pub const SPAPI_E_DI_POSTPROCESSING_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2146500058i32);
pub const SPAPI_E_DRIVER_INSTALL_BLOCKED: crate::core::HRESULT =
    crate::core::HRESULT(-2146500023i32);
pub const SPAPI_E_DRIVER_NONNATIVE: crate::core::HRESULT = crate::core::HRESULT(-2146500044i32);
pub const SPAPI_E_DRIVER_STORE_ADD_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2146500025i32);
pub const SPAPI_E_DRIVER_STORE_DELETE_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2146500020i32);
pub const SPAPI_E_DUPLICATE_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2146500094i32);
pub const SPAPI_E_ERROR_NOT_INSTALLED: crate::core::HRESULT = crate::core::HRESULT(-2146496512i32);
pub const SPAPI_E_EXPECTED_SECTION_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-2146500608i32);
pub const SPAPI_E_FILEQUEUE_LOCKED: crate::core::HRESULT = crate::core::HRESULT(-2146500074i32);
pub const SPAPI_E_FILE_HASH_NOT_IN_CATALOG: crate::core::HRESULT =
    crate::core::HRESULT(-2146500021i32);
pub const SPAPI_E_GENERAL_SYNTAX: crate::core::HRESULT = crate::core::HRESULT(-2146500605i32);
pub const SPAPI_E_INCORRECTLY_COPIED_INF: crate::core::HRESULT =
    crate::core::HRESULT(-2146500041i32);
pub const SPAPI_E_INF_IN_USE_BY_DEVICES: crate::core::HRESULT =
    crate::core::HRESULT(-2146500035i32);
pub const SPAPI_E_INVALID_CLASS: crate::core::HRESULT = crate::core::HRESULT(-2146500090i32);
pub const SPAPI_E_INVALID_CLASS_INSTALLER: crate::core::HRESULT =
    crate::core::HRESULT(-2146500083i32);
pub const SPAPI_E_INVALID_COINSTALLER: crate::core::HRESULT = crate::core::HRESULT(-2146500057i32);
pub const SPAPI_E_INVALID_DEVINST_NAME: crate::core::HRESULT = crate::core::HRESULT(-2146500091i32);
pub const SPAPI_E_INVALID_FILTER_DRIVER: crate::core::HRESULT =
    crate::core::HRESULT(-2146500052i32);
pub const SPAPI_E_INVALID_HWPROFILE: crate::core::HRESULT = crate::core::HRESULT(-2146500080i32);
pub const SPAPI_E_INVALID_INF_LOGCONFIG: crate::core::HRESULT =
    crate::core::HRESULT(-2146500054i32);
pub const SPAPI_E_INVALID_MACHINENAME: crate::core::HRESULT = crate::core::HRESULT(-2146500064i32);
pub const SPAPI_E_INVALID_PROPPAGE_PROVIDER: crate::core::HRESULT =
    crate::core::HRESULT(-2146500060i32);
pub const SPAPI_E_INVALID_REFERENCE_STRING: crate::core::HRESULT =
    crate::core::HRESULT(-2146500065i32);
pub const SPAPI_E_INVALID_REG_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-2146500087i32);
pub const SPAPI_E_INVALID_TARGET: crate::core::HRESULT = crate::core::HRESULT(-2146500045i32);
pub const SPAPI_E_IN_WOW64: crate::core::HRESULT = crate::core::HRESULT(-2146500043i32);
pub const SPAPI_E_KEY_DOES_NOT_EXIST: crate::core::HRESULT = crate::core::HRESULT(-2146500092i32);
pub const SPAPI_E_LINE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2146500350i32);
pub const SPAPI_E_MACHINE_UNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2146500062i32);
pub const SPAPI_E_NON_WINDOWS_DRIVER: crate::core::HRESULT = crate::core::HRESULT(-2146500050i32);
pub const SPAPI_E_NON_WINDOWS_NT_DRIVER: crate::core::HRESULT =
    crate::core::HRESULT(-2146500051i32);
pub const SPAPI_E_NOT_AN_INSTALLED_OEM_INF: crate::core::HRESULT =
    crate::core::HRESULT(-2146500036i32);
pub const SPAPI_E_NOT_DISABLEABLE: crate::core::HRESULT = crate::core::HRESULT(-2146500047i32);
pub const SPAPI_E_NO_ASSOCIATED_CLASS: crate::core::HRESULT = crate::core::HRESULT(-2146500096i32);
pub const SPAPI_E_NO_ASSOCIATED_SERVICE: crate::core::HRESULT =
    crate::core::HRESULT(-2146500071i32);
pub const SPAPI_E_NO_AUTHENTICODE_CATALOG: crate::core::HRESULT =
    crate::core::HRESULT(-2146500033i32);
pub const SPAPI_E_NO_BACKUP: crate::core::HRESULT = crate::core::HRESULT(-2146500349i32);
pub const SPAPI_E_NO_CATALOG_FOR_OEM_INF: crate::core::HRESULT =
    crate::core::HRESULT(-2146500049i32);
pub const SPAPI_E_NO_CLASSINSTALL_PARAMS: crate::core::HRESULT =
    crate::core::HRESULT(-2146500075i32);
pub const SPAPI_E_NO_CLASS_DRIVER_LIST: crate::core::HRESULT = crate::core::HRESULT(-2146500072i32);
pub const SPAPI_E_NO_COMPAT_DRIVERS: crate::core::HRESULT = crate::core::HRESULT(-2146500056i32);
pub const SPAPI_E_NO_CONFIGMGR_SERVICES: crate::core::HRESULT =
    crate::core::HRESULT(-2146500061i32);
pub const SPAPI_E_NO_DEFAULT_DEVICE_INTERFACE: crate::core::HRESULT =
    crate::core::HRESULT(-2146500070i32);
pub const SPAPI_E_NO_DEVICE_ICON: crate::core::HRESULT = crate::core::HRESULT(-2146500055i32);
pub const SPAPI_E_NO_DEVICE_SELECTED: crate::core::HRESULT = crate::core::HRESULT(-2146500079i32);
pub const SPAPI_E_NO_DRIVER_SELECTED: crate::core::HRESULT = crate::core::HRESULT(-2146500093i32);
pub const SPAPI_E_NO_INF: crate::core::HRESULT = crate::core::HRESULT(-2146500086i32);
pub const SPAPI_E_NO_SUCH_DEVICE_INTERFACE: crate::core::HRESULT =
    crate::core::HRESULT(-2146500059i32);
pub const SPAPI_E_NO_SUCH_DEVINST: crate::core::HRESULT = crate::core::HRESULT(-2146500085i32);
pub const SPAPI_E_NO_SUCH_INTERFACE_CLASS: crate::core::HRESULT =
    crate::core::HRESULT(-2146500066i32);
pub const SPAPI_E_ONLY_VALIDATE_VIA_AUTHENTICODE: crate::core::HRESULT =
    crate::core::HRESULT(-2146500027i32);
pub const SPAPI_E_PNP_REGISTRY_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146500038i32);
pub const SPAPI_E_REMOTE_COMM_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2146500063i32);
pub const SPAPI_E_REMOTE_REQUEST_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2146500037i32);
pub const SPAPI_E_SCE_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2146500040i32);
pub const SPAPI_E_SECTION_NAME_TOO_LONG: crate::core::HRESULT =
    crate::core::HRESULT(-2146500606i32);
pub const SPAPI_E_SECTION_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2146500351i32);
pub const SPAPI_E_SET_SYSTEM_RESTORE_POINT: crate::core::HRESULT =
    crate::core::HRESULT(-2146500042i32);
pub const SPAPI_E_SIGNATURE_OSATTRIBUTE_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2146500028i32);
pub const SPAPI_E_UNKNOWN_EXCEPTION: crate::core::HRESULT = crate::core::HRESULT(-2146500039i32);
pub const SPAPI_E_UNRECOVERABLE_STACK_OVERFLOW: crate::core::HRESULT =
    crate::core::HRESULT(-2146499840i32);
pub const SPAPI_E_WRONG_INF_STYLE: crate::core::HRESULT = crate::core::HRESULT(-2146500352i32);
pub const SPAPI_E_WRONG_INF_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2146500022i32);
pub const SQLITE_E_ABORT: crate::core::HRESULT = crate::core::HRESULT(-2018574332i32);
pub const SQLITE_E_ABORT_ROLLBACK: crate::core::HRESULT = crate::core::HRESULT(-2018573820i32);
pub const SQLITE_E_AUTH: crate::core::HRESULT = crate::core::HRESULT(-2018574313i32);
pub const SQLITE_E_BUSY: crate::core::HRESULT = crate::core::HRESULT(-2018574331i32);
pub const SQLITE_E_BUSY_RECOVERY: crate::core::HRESULT = crate::core::HRESULT(-2018574075i32);
pub const SQLITE_E_BUSY_SNAPSHOT: crate::core::HRESULT = crate::core::HRESULT(-2018573819i32);
pub const SQLITE_E_CANTOPEN: crate::core::HRESULT = crate::core::HRESULT(-2018574322i32);
pub const SQLITE_E_CANTOPEN_CONVPATH: crate::core::HRESULT = crate::core::HRESULT(-2018573298i32);
pub const SQLITE_E_CANTOPEN_FULLPATH: crate::core::HRESULT = crate::core::HRESULT(-2018573554i32);
pub const SQLITE_E_CANTOPEN_ISDIR: crate::core::HRESULT = crate::core::HRESULT(-2018573810i32);
pub const SQLITE_E_CANTOPEN_NOTEMPDIR: crate::core::HRESULT = crate::core::HRESULT(-2018574066i32);
pub const SQLITE_E_CONSTRAINT: crate::core::HRESULT = crate::core::HRESULT(-2018574317i32);
pub const SQLITE_E_CONSTRAINT_CHECK: crate::core::HRESULT = crate::core::HRESULT(-2018574061i32);
pub const SQLITE_E_CONSTRAINT_COMMITHOOK: crate::core::HRESULT =
    crate::core::HRESULT(-2018573805i32);
pub const SQLITE_E_CONSTRAINT_FOREIGNKEY: crate::core::HRESULT =
    crate::core::HRESULT(-2018573549i32);
pub const SQLITE_E_CONSTRAINT_FUNCTION: crate::core::HRESULT = crate::core::HRESULT(-2018573293i32);
pub const SQLITE_E_CONSTRAINT_NOTNULL: crate::core::HRESULT = crate::core::HRESULT(-2018573037i32);
pub const SQLITE_E_CONSTRAINT_PRIMARYKEY: crate::core::HRESULT =
    crate::core::HRESULT(-2018572781i32);
pub const SQLITE_E_CONSTRAINT_ROWID: crate::core::HRESULT = crate::core::HRESULT(-2018571757i32);
pub const SQLITE_E_CONSTRAINT_TRIGGER: crate::core::HRESULT = crate::core::HRESULT(-2018572525i32);
pub const SQLITE_E_CONSTRAINT_UNIQUE: crate::core::HRESULT = crate::core::HRESULT(-2018572269i32);
pub const SQLITE_E_CONSTRAINT_VTAB: crate::core::HRESULT = crate::core::HRESULT(-2018572013i32);
pub const SQLITE_E_CORRUPT: crate::core::HRESULT = crate::core::HRESULT(-2018574325i32);
pub const SQLITE_E_CORRUPT_VTAB: crate::core::HRESULT = crate::core::HRESULT(-2018574069i32);
pub const SQLITE_E_DONE: crate::core::HRESULT = crate::core::HRESULT(-2018574235i32);
pub const SQLITE_E_EMPTY: crate::core::HRESULT = crate::core::HRESULT(-2018574320i32);
pub const SQLITE_E_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2018574335i32);
pub const SQLITE_E_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-2018574312i32);
pub const SQLITE_E_FULL: crate::core::HRESULT = crate::core::HRESULT(-2018574323i32);
pub const SQLITE_E_INTERNAL: crate::core::HRESULT = crate::core::HRESULT(-2018574334i32);
pub const SQLITE_E_INTERRUPT: crate::core::HRESULT = crate::core::HRESULT(-2018574327i32);
pub const SQLITE_E_IOERR: crate::core::HRESULT = crate::core::HRESULT(-2018574326i32);
pub const SQLITE_E_IOERR_ACCESS: crate::core::HRESULT = crate::core::HRESULT(-2018570998i32);
pub const SQLITE_E_IOERR_AUTH: crate::core::HRESULT = crate::core::HRESULT(-2018567677i32);
pub const SQLITE_E_IOERR_BLOCKED: crate::core::HRESULT = crate::core::HRESULT(-2018571510i32);
pub const SQLITE_E_IOERR_CHECKRESERVEDLOCK: crate::core::HRESULT =
    crate::core::HRESULT(-2018570742i32);
pub const SQLITE_E_IOERR_CLOSE: crate::core::HRESULT = crate::core::HRESULT(-2018570230i32);
pub const SQLITE_E_IOERR_CONVPATH: crate::core::HRESULT = crate::core::HRESULT(-2018567670i32);
pub const SQLITE_E_IOERR_DELETE: crate::core::HRESULT = crate::core::HRESULT(-2018571766i32);
pub const SQLITE_E_IOERR_DELETE_NOENT: crate::core::HRESULT = crate::core::HRESULT(-2018568438i32);
pub const SQLITE_E_IOERR_DIR_CLOSE: crate::core::HRESULT = crate::core::HRESULT(-2018569974i32);
pub const SQLITE_E_IOERR_DIR_FSYNC: crate::core::HRESULT = crate::core::HRESULT(-2018573046i32);
pub const SQLITE_E_IOERR_FSTAT: crate::core::HRESULT = crate::core::HRESULT(-2018572534i32);
pub const SQLITE_E_IOERR_FSYNC: crate::core::HRESULT = crate::core::HRESULT(-2018573302i32);
pub const SQLITE_E_IOERR_GETTEMPPATH: crate::core::HRESULT = crate::core::HRESULT(-2018567926i32);
pub const SQLITE_E_IOERR_LOCK: crate::core::HRESULT = crate::core::HRESULT(-2018570486i32);
pub const SQLITE_E_IOERR_MMAP: crate::core::HRESULT = crate::core::HRESULT(-2018568182i32);
pub const SQLITE_E_IOERR_NOMEM: crate::core::HRESULT = crate::core::HRESULT(-2018571254i32);
pub const SQLITE_E_IOERR_RDLOCK: crate::core::HRESULT = crate::core::HRESULT(-2018572022i32);
pub const SQLITE_E_IOERR_READ: crate::core::HRESULT = crate::core::HRESULT(-2018574070i32);
pub const SQLITE_E_IOERR_SEEK: crate::core::HRESULT = crate::core::HRESULT(-2018568694i32);
pub const SQLITE_E_IOERR_SHMLOCK: crate::core::HRESULT = crate::core::HRESULT(-2018569206i32);
pub const SQLITE_E_IOERR_SHMMAP: crate::core::HRESULT = crate::core::HRESULT(-2018568950i32);
pub const SQLITE_E_IOERR_SHMOPEN: crate::core::HRESULT = crate::core::HRESULT(-2018569718i32);
pub const SQLITE_E_IOERR_SHMSIZE: crate::core::HRESULT = crate::core::HRESULT(-2018569462i32);
pub const SQLITE_E_IOERR_SHORT_READ: crate::core::HRESULT = crate::core::HRESULT(-2018573814i32);
pub const SQLITE_E_IOERR_TRUNCATE: crate::core::HRESULT = crate::core::HRESULT(-2018572790i32);
pub const SQLITE_E_IOERR_UNLOCK: crate::core::HRESULT = crate::core::HRESULT(-2018572278i32);
pub const SQLITE_E_IOERR_VNODE: crate::core::HRESULT = crate::core::HRESULT(-2018567678i32);
pub const SQLITE_E_IOERR_WRITE: crate::core::HRESULT = crate::core::HRESULT(-2018573558i32);
pub const SQLITE_E_LOCKED: crate::core::HRESULT = crate::core::HRESULT(-2018574330i32);
pub const SQLITE_E_LOCKED_SHAREDCACHE: crate::core::HRESULT = crate::core::HRESULT(-2018574074i32);
pub const SQLITE_E_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2018574316i32);
pub const SQLITE_E_MISUSE: crate::core::HRESULT = crate::core::HRESULT(-2018574315i32);
pub const SQLITE_E_NOLFS: crate::core::HRESULT = crate::core::HRESULT(-2018574314i32);
pub const SQLITE_E_NOMEM: crate::core::HRESULT = crate::core::HRESULT(-2018574329i32);
pub const SQLITE_E_NOTADB: crate::core::HRESULT = crate::core::HRESULT(-2018574310i32);
pub const SQLITE_E_NOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2018574324i32);
pub const SQLITE_E_NOTICE: crate::core::HRESULT = crate::core::HRESULT(-2018574309i32);
pub const SQLITE_E_NOTICE_RECOVER_ROLLBACK: crate::core::HRESULT =
    crate::core::HRESULT(-2018573797i32);
pub const SQLITE_E_NOTICE_RECOVER_WAL: crate::core::HRESULT = crate::core::HRESULT(-2018574053i32);
pub const SQLITE_E_PERM: crate::core::HRESULT = crate::core::HRESULT(-2018574333i32);
pub const SQLITE_E_PROTOCOL: crate::core::HRESULT = crate::core::HRESULT(-2018574321i32);
pub const SQLITE_E_RANGE: crate::core::HRESULT = crate::core::HRESULT(-2018574311i32);
pub const SQLITE_E_READONLY: crate::core::HRESULT = crate::core::HRESULT(-2018574328i32);
pub const SQLITE_E_READONLY_CANTLOCK: crate::core::HRESULT = crate::core::HRESULT(-2018573816i32);
pub const SQLITE_E_READONLY_DBMOVED: crate::core::HRESULT = crate::core::HRESULT(-2018573304i32);
pub const SQLITE_E_READONLY_RECOVERY: crate::core::HRESULT = crate::core::HRESULT(-2018574072i32);
pub const SQLITE_E_READONLY_ROLLBACK: crate::core::HRESULT = crate::core::HRESULT(-2018573560i32);
pub const SQLITE_E_ROW: crate::core::HRESULT = crate::core::HRESULT(-2018574236i32);
pub const SQLITE_E_SCHEMA: crate::core::HRESULT = crate::core::HRESULT(-2018574319i32);
pub const SQLITE_E_TOOBIG: crate::core::HRESULT = crate::core::HRESULT(-2018574318i32);
pub const SQLITE_E_WARNING: crate::core::HRESULT = crate::core::HRESULT(-2018574308i32);
pub const SQLITE_E_WARNING_AUTOINDEX: crate::core::HRESULT = crate::core::HRESULT(-2018574052i32);
pub const STATEREPOSITORY_ERROR_CACHE_CORRUPTED: crate::core::HRESULT =
    crate::core::HRESULT(-2140733422i32);
pub const STATEREPOSITORY_ERROR_DICTIONARY_CORRUPTED: crate::core::HRESULT =
    crate::core::HRESULT(-2140733435i32);
pub const STATEREPOSITORY_E_BLOCKED: crate::core::HRESULT = crate::core::HRESULT(-2140733434i32);
pub const STATEREPOSITORY_E_BUSY_RECOVERY_RETRY: crate::core::HRESULT =
    crate::core::HRESULT(-2140733432i32);
pub const STATEREPOSITORY_E_BUSY_RECOVERY_TIMEOUT_EXCEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-2140733427i32);
pub const STATEREPOSITORY_E_BUSY_RETRY: crate::core::HRESULT = crate::core::HRESULT(-2140733433i32);
pub const STATEREPOSITORY_E_BUSY_TIMEOUT_EXCEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-2140733428i32);
pub const STATEREPOSITORY_E_CACHE_NOT_INIITALIZED: crate::core::HRESULT =
    crate::core::HRESULT(-2140733419i32);
pub const STATEREPOSITORY_E_CONCURRENCY_LOCKING_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-2140733439i32);
pub const STATEREPOSITORY_E_CONFIGURATION_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2140733437i32);
pub const STATEREPOSITORY_E_DEPENDENCY_NOT_RESOLVED: crate::core::HRESULT =
    crate::core::HRESULT(-2140733418i32);
pub const STATEREPOSITORY_E_LOCKED_RETRY: crate::core::HRESULT =
    crate::core::HRESULT(-2140733431i32);
pub const STATEREPOSITORY_E_LOCKED_SHAREDCACHE_RETRY: crate::core::HRESULT =
    crate::core::HRESULT(-2140733430i32);
pub const STATEREPOSITORY_E_LOCKED_SHAREDCACHE_TIMEOUT_EXCEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-2140733425i32);
pub const STATEREPOSITORY_E_LOCKED_TIMEOUT_EXCEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-2140733426i32);
pub const STATEREPOSITORY_E_SERVICE_STOP_IN_PROGRESS: crate::core::HRESULT =
    crate::core::HRESULT(-2140733424i32);
pub const STATEREPOSITORY_E_STATEMENT_INPROGRESS: crate::core::HRESULT =
    crate::core::HRESULT(-2140733438i32);
pub const STATEREPOSITORY_E_TRANSACTION_REQUIRED: crate::core::HRESULT =
    crate::core::HRESULT(-2140733429i32);
pub const STATEREPOSITORY_E_UNKNOWN_SCHEMA_VERSION: crate::core::HRESULT =
    crate::core::HRESULT(-2140733436i32);
pub const STATEREPOSITORY_TRANSACTION_CALLER_ID_CHANGED: crate::core::HRESULT =
    crate::core::HRESULT(6750227i32);
pub const STATEREPOSITORY_TRANSACTION_IN_PROGRESS: crate::core::HRESULT =
    crate::core::HRESULT(-2140733420i32);
pub const STATEREPOSTORY_E_NESTED_TRANSACTION_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2140733423i32);
pub const STATUS_ABANDONED: NTSTATUS = NTSTATUS(128i32);
pub const STATUS_ABANDONED_WAIT_0: NTSTATUS = NTSTATUS(128i32);
pub const STATUS_ABANDONED_WAIT_63: NTSTATUS = NTSTATUS(191i32);
pub const STATUS_ABANDON_HIBERFILE: NTSTATUS = NTSTATUS(1073741875i32);
pub const STATUS_ABIOS_INVALID_COMMAND: NTSTATUS = NTSTATUS(-1073741549i32);
pub const STATUS_ABIOS_INVALID_LID: NTSTATUS = NTSTATUS(-1073741548i32);
pub const STATUS_ABIOS_INVALID_SELECTOR: NTSTATUS = NTSTATUS(-1073741546i32);
pub const STATUS_ABIOS_LID_ALREADY_OWNED: NTSTATUS = NTSTATUS(-1073741551i32);
pub const STATUS_ABIOS_LID_NOT_EXIST: NTSTATUS = NTSTATUS(-1073741552i32);
pub const STATUS_ABIOS_NOT_LID_OWNER: NTSTATUS = NTSTATUS(-1073741550i32);
pub const STATUS_ABIOS_NOT_PRESENT: NTSTATUS = NTSTATUS(-1073741553i32);
pub const STATUS_ABIOS_SELECTOR_NOT_AVAILABLE: NTSTATUS = NTSTATUS(-1073741547i32);
pub const STATUS_ACCESS_AUDIT_BY_POLICY: NTSTATUS = NTSTATUS(1073741874i32);
pub const STATUS_ACCESS_DISABLED_BY_POLICY_DEFAULT: NTSTATUS = NTSTATUS(-1073740959i32);
pub const STATUS_ACCESS_DISABLED_BY_POLICY_OTHER: NTSTATUS = NTSTATUS(-1073740956i32);
pub const STATUS_ACCESS_DISABLED_BY_POLICY_PATH: NTSTATUS = NTSTATUS(-1073740958i32);
pub const STATUS_ACCESS_DISABLED_BY_POLICY_PUBLISHER: NTSTATUS = NTSTATUS(-1073740957i32);
pub const STATUS_ACCESS_DISABLED_NO_SAFER_UI_BY_POLICY: NTSTATUS = NTSTATUS(-1073740942i32);
pub const STATUS_ACCESS_VIOLATION: NTSTATUS = NTSTATUS(-1073741819i32);
pub const STATUS_ACPI_ACQUIRE_GLOBAL_LOCK: NTSTATUS = NTSTATUS(-1072431086i32);
pub const STATUS_ACPI_ADDRESS_NOT_MAPPED: NTSTATUS = NTSTATUS(-1072431092i32);
pub const STATUS_ACPI_ALREADY_INITIALIZED: NTSTATUS = NTSTATUS(-1072431085i32);
pub const STATUS_ACPI_ASSERT_FAILED: NTSTATUS = NTSTATUS(-1072431101i32);
pub const STATUS_ACPI_FATAL: NTSTATUS = NTSTATUS(-1072431098i32);
pub const STATUS_ACPI_HANDLER_COLLISION: NTSTATUS = NTSTATUS(-1072431090i32);
pub const STATUS_ACPI_INCORRECT_ARGUMENT_COUNT: NTSTATUS = NTSTATUS(-1072431093i32);
pub const STATUS_ACPI_INVALID_ACCESS_SIZE: NTSTATUS = NTSTATUS(-1072431087i32);
pub const STATUS_ACPI_INVALID_ARGTYPE: NTSTATUS = NTSTATUS(-1072431096i32);
pub const STATUS_ACPI_INVALID_ARGUMENT: NTSTATUS = NTSTATUS(-1072431099i32);
pub const STATUS_ACPI_INVALID_DATA: NTSTATUS = NTSTATUS(-1072431089i32);
pub const STATUS_ACPI_INVALID_EVENTTYPE: NTSTATUS = NTSTATUS(-1072431091i32);
pub const STATUS_ACPI_INVALID_INDEX: NTSTATUS = NTSTATUS(-1072431100i32);
pub const STATUS_ACPI_INVALID_MUTEX_LEVEL: NTSTATUS = NTSTATUS(-1072431083i32);
pub const STATUS_ACPI_INVALID_OBJTYPE: NTSTATUS = NTSTATUS(-1072431095i32);
pub const STATUS_ACPI_INVALID_OPCODE: NTSTATUS = NTSTATUS(-1072431103i32);
pub const STATUS_ACPI_INVALID_REGION: NTSTATUS = NTSTATUS(-1072431088i32);
pub const STATUS_ACPI_INVALID_SUPERNAME: NTSTATUS = NTSTATUS(-1072431097i32);
pub const STATUS_ACPI_INVALID_TABLE: NTSTATUS = NTSTATUS(-1072431079i32);
pub const STATUS_ACPI_INVALID_TARGETTYPE: NTSTATUS = NTSTATUS(-1072431094i32);
pub const STATUS_ACPI_MUTEX_NOT_OWNED: NTSTATUS = NTSTATUS(-1072431082i32);
pub const STATUS_ACPI_MUTEX_NOT_OWNER: NTSTATUS = NTSTATUS(-1072431081i32);
pub const STATUS_ACPI_NOT_INITIALIZED: NTSTATUS = NTSTATUS(-1072431084i32);
pub const STATUS_ACPI_POWER_REQUEST_FAILED: NTSTATUS = NTSTATUS(-1072431071i32);
pub const STATUS_ACPI_REG_HANDLER_FAILED: NTSTATUS = NTSTATUS(-1072431072i32);
pub const STATUS_ACPI_RS_ACCESS: NTSTATUS = NTSTATUS(-1072431080i32);
pub const STATUS_ACPI_STACK_OVERFLOW: NTSTATUS = NTSTATUS(-1072431102i32);
pub const STATUS_ADAPTER_HARDWARE_ERROR: NTSTATUS = NTSTATUS(-1073741630i32);
pub const STATUS_ADDRESS_ALREADY_ASSOCIATED: NTSTATUS = NTSTATUS(-1073741256i32);
pub const STATUS_ADDRESS_ALREADY_EXISTS: NTSTATUS = NTSTATUS(-1073741302i32);
pub const STATUS_ADDRESS_CLOSED: NTSTATUS = NTSTATUS(-1073741301i32);
pub const STATUS_ADDRESS_NOT_ASSOCIATED: NTSTATUS = NTSTATUS(-1073741255i32);
pub const STATUS_ADMINLESS_ACCESS_DENIED: NTSTATUS = NTSTATUS(-1073700348i32);
pub const STATUS_ADVANCED_INSTALLER_FAILED: NTSTATUS = NTSTATUS(-1072365536i32);
pub const STATUS_AGENTS_EXHAUSTED: NTSTATUS = NTSTATUS(-1073741691i32);
pub const STATUS_ALERTED: NTSTATUS = NTSTATUS(257i32);
pub const STATUS_ALIAS_EXISTS: NTSTATUS = NTSTATUS(-1073741484i32);
pub const STATUS_ALLOCATE_BUCKET: NTSTATUS = NTSTATUS(-1073741265i32);
pub const STATUS_ALLOTTED_SPACE_EXCEEDED: NTSTATUS = NTSTATUS(-1073741671i32);
pub const STATUS_ALL_SIDS_FILTERED: NTSTATUS = NTSTATUS(-1073740962i32);
pub const STATUS_ALL_USER_TRUST_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(-1073740798i32);
pub const STATUS_ALPC_CHECK_COMPLETION_LIST: NTSTATUS = NTSTATUS(1073741872i32);
pub const STATUS_ALREADY_COMMITTED: NTSTATUS = NTSTATUS(-1073741791i32);
pub const STATUS_ALREADY_COMPLETE: NTSTATUS = NTSTATUS(255i32);
pub const STATUS_ALREADY_DISCONNECTED: NTSTATUS = NTSTATUS(-2147483611i32);
pub const STATUS_ALREADY_HAS_STREAM_ID: NTSTATUS = NTSTATUS(-1073740530i32);
pub const STATUS_ALREADY_INITIALIZED: NTSTATUS = NTSTATUS(-1073740528i32);
pub const STATUS_ALREADY_REGISTERED: NTSTATUS = NTSTATUS(-1073740008i32);
pub const STATUS_ALREADY_WIN32: NTSTATUS = NTSTATUS(1073741851i32);
pub const STATUS_AMBIGUOUS_SYSTEM_DEVICE: NTSTATUS = NTSTATUS(-1073740719i32);
pub const STATUS_APC_RETURNED_WHILE_IMPERSONATING: NTSTATUS = NTSTATUS(-1073740015i32);
pub const STATUS_APISET_NOT_HOSTED: NTSTATUS = NTSTATUS(-1073740671i32);
pub const STATUS_APISET_NOT_PRESENT: NTSTATUS = NTSTATUS(-1073740670i32);
pub const STATUS_APPEXEC_APP_COMPAT_BLOCK: NTSTATUS = NTSTATUS(-1058275320i32);
pub const STATUS_APPEXEC_CALLER_WAIT_TIMEOUT: NTSTATUS = NTSTATUS(-1058275319i32);
pub const STATUS_APPEXEC_CALLER_WAIT_TIMEOUT_LICENSING: NTSTATUS = NTSTATUS(-1058275317i32);
pub const STATUS_APPEXEC_CALLER_WAIT_TIMEOUT_RESOURCES: NTSTATUS = NTSTATUS(-1058275316i32);
pub const STATUS_APPEXEC_CALLER_WAIT_TIMEOUT_TERMINATION: NTSTATUS = NTSTATUS(-1058275318i32);
pub const STATUS_APPEXEC_CONDITION_NOT_SATISFIED: NTSTATUS = NTSTATUS(-1058275328i32);
pub const STATUS_APPEXEC_HANDLE_INVALIDATED: NTSTATUS = NTSTATUS(-1058275327i32);
pub const STATUS_APPEXEC_HOST_ID_MISMATCH: NTSTATUS = NTSTATUS(-1058275322i32);
pub const STATUS_APPEXEC_INVALID_HOST_GENERATION: NTSTATUS = NTSTATUS(-1058275326i32);
pub const STATUS_APPEXEC_INVALID_HOST_STATE: NTSTATUS = NTSTATUS(-1058275324i32);
pub const STATUS_APPEXEC_NO_DONOR: NTSTATUS = NTSTATUS(-1058275323i32);
pub const STATUS_APPEXEC_UNEXPECTED_PROCESS_REGISTRATION: NTSTATUS = NTSTATUS(-1058275325i32);
pub const STATUS_APPEXEC_UNKNOWN_USER: NTSTATUS = NTSTATUS(-1058275321i32);
pub const STATUS_APPHELP_BLOCK: NTSTATUS = NTSTATUS(-1073740963i32);
pub const STATUS_APPX_FILE_NOT_ENCRYPTED: NTSTATUS = NTSTATUS(-1073740634i32);
pub const STATUS_APPX_INTEGRITY_FAILURE_CLR_NGEN: NTSTATUS = NTSTATUS(-1073740673i32);
pub const STATUS_APP_DATA_CORRUPT: NTSTATUS = NTSTATUS(-1073700221i32);
pub const STATUS_APP_DATA_EXPIRED: NTSTATUS = NTSTATUS(-1073700222i32);
pub const STATUS_APP_DATA_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(-1073700220i32);
pub const STATUS_APP_DATA_NOT_FOUND: NTSTATUS = NTSTATUS(-1073700223i32);
pub const STATUS_APP_DATA_REBOOT_REQUIRED: NTSTATUS = NTSTATUS(-1073700219i32);
pub const STATUS_APP_INIT_FAILURE: NTSTATUS = NTSTATUS(-1073741499i32);
pub const STATUS_ARBITRATION_UNHANDLED: NTSTATUS = NTSTATUS(1073741862i32);
pub const STATUS_ARRAY_BOUNDS_EXCEEDED: NTSTATUS = NTSTATUS(-1073741684i32);
pub const STATUS_ASSERTION_FAILURE: NTSTATUS = NTSTATUS(-1073740768i32);
pub const STATUS_ATTACHED_EXECUTABLE_MEMORY_WRITE: NTSTATUS = NTSTATUS(-1073739995i32);
pub const STATUS_ATTRIBUTE_NOT_PRESENT: NTSTATUS = NTSTATUS(-1073740532i32);
pub const STATUS_AUDIO_ENGINE_NODE_NOT_FOUND: NTSTATUS = NTSTATUS(-1069285375i32);
pub const STATUS_AUDITING_DISABLED: NTSTATUS = NTSTATUS(-1073740970i32);
pub const STATUS_AUDIT_FAILED: NTSTATUS = NTSTATUS(-1073741244i32);
pub const STATUS_AUTHIP_FAILURE: NTSTATUS = NTSTATUS(-1073700730i32);
pub const STATUS_AUTH_TAG_MISMATCH: NTSTATUS = NTSTATUS(-1073700862i32);
pub const STATUS_BACKUP_CONTROLLER: NTSTATUS = NTSTATUS(-1073741433i32);
pub const STATUS_BAD_BINDINGS: NTSTATUS = NTSTATUS(-1073740965i32);
pub const STATUS_BAD_CLUSTERS: NTSTATUS = NTSTATUS(-1073739771i32);
pub const STATUS_BAD_COMPRESSION_BUFFER: NTSTATUS = NTSTATUS(-1073741246i32);
pub const STATUS_BAD_CURRENT_DIRECTORY: NTSTATUS = NTSTATUS(1073741831i32);
pub const STATUS_BAD_DATA: NTSTATUS = NTSTATUS(-1073739509i32);
pub const STATUS_BAD_DESCRIPTOR_FORMAT: NTSTATUS = NTSTATUS(-1073741593i32);
pub const STATUS_BAD_DEVICE_TYPE: NTSTATUS = NTSTATUS(-1073741621i32);
pub const STATUS_BAD_DLL_ENTRYPOINT: NTSTATUS = NTSTATUS(-1073741231i32);
pub const STATUS_BAD_FILE_TYPE: NTSTATUS = NTSTATUS(-1073739517i32);
pub const STATUS_BAD_FUNCTION_TABLE: NTSTATUS = NTSTATUS(-1073741569i32);
pub const STATUS_BAD_IMPERSONATION_LEVEL: NTSTATUS = NTSTATUS(-1073741659i32);
pub const STATUS_BAD_INHERITANCE_ACL: NTSTATUS = NTSTATUS(-1073741699i32);
pub const STATUS_BAD_INITIAL_PC: NTSTATUS = NTSTATUS(-1073741814i32);
pub const STATUS_BAD_INITIAL_STACK: NTSTATUS = NTSTATUS(-1073741815i32);
pub const STATUS_BAD_KEY: NTSTATUS = NTSTATUS(-1073739510i32);
pub const STATUS_BAD_LOGON_SESSION_STATE: NTSTATUS = NTSTATUS(-1073741564i32);
pub const STATUS_BAD_MASTER_BOOT_RECORD: NTSTATUS = NTSTATUS(-1073741655i32);
pub const STATUS_BAD_MCFG_TABLE: NTSTATUS = NTSTATUS(-1073739512i32);
pub const STATUS_BAD_NETWORK_NAME: NTSTATUS = NTSTATUS(-1073741620i32);
pub const STATUS_BAD_NETWORK_PATH: NTSTATUS = NTSTATUS(-1073741634i32);
pub const STATUS_BAD_REMOTE_ADAPTER: NTSTATUS = NTSTATUS(-1073741627i32);
pub const STATUS_BAD_SERVICE_ENTRYPOINT: NTSTATUS = NTSTATUS(-1073741230i32);
pub const STATUS_BAD_STACK: NTSTATUS = NTSTATUS(-1073741784i32);
pub const STATUS_BAD_TOKEN_TYPE: NTSTATUS = NTSTATUS(-1073741656i32);
pub const STATUS_BAD_VALIDATION_CLASS: NTSTATUS = NTSTATUS(-1073741657i32);
pub const STATUS_BAD_WORKING_SET_LIMIT: NTSTATUS = NTSTATUS(-1073741748i32);
pub const STATUS_BCD_NOT_ALL_ENTRIES_IMPORTED: NTSTATUS = NTSTATUS(-2143748095i32);
pub const STATUS_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED: NTSTATUS = NTSTATUS(-2143748093i32);
pub const STATUS_BCD_TOO_MANY_ELEMENTS: NTSTATUS = NTSTATUS(-1070006270i32);
pub const STATUS_BEGINNING_OF_MEDIA: NTSTATUS = NTSTATUS(-2147483617i32);
pub const STATUS_BEYOND_VDL: NTSTATUS = NTSTATUS(-1073740750i32);
pub const STATUS_BIOS_FAILED_TO_CONNECT_INTERRUPT: NTSTATUS = NTSTATUS(-1073741458i32);
pub const STATUS_BIZRULES_NOT_ENABLED: NTSTATUS = NTSTATUS(1073741876i32);
pub const STATUS_BLOCKED_BY_PARENTAL_CONTROLS: NTSTATUS = NTSTATUS(-1073740664i32);
pub const STATUS_BLOCK_TOO_MANY_REFERENCES: NTSTATUS = NTSTATUS(-1073740660i32);
pub const STATUS_BREAKPOINT: NTSTATUS = NTSTATUS(-2147483645i32);
pub const STATUS_BTH_ATT_ATTRIBUTE_NOT_FOUND: NTSTATUS = NTSTATUS(-1069416438i32);
pub const STATUS_BTH_ATT_ATTRIBUTE_NOT_LONG: NTSTATUS = NTSTATUS(-1069416437i32);
pub const STATUS_BTH_ATT_INSUFFICIENT_AUTHENTICATION: NTSTATUS = NTSTATUS(-1069416443i32);
pub const STATUS_BTH_ATT_INSUFFICIENT_AUTHORIZATION: NTSTATUS = NTSTATUS(-1069416440i32);
pub const STATUS_BTH_ATT_INSUFFICIENT_ENCRYPTION: NTSTATUS = NTSTATUS(-1069416433i32);
pub const STATUS_BTH_ATT_INSUFFICIENT_ENCRYPTION_KEY_SIZE: NTSTATUS = NTSTATUS(-1069416436i32);
pub const STATUS_BTH_ATT_INSUFFICIENT_RESOURCES: NTSTATUS = NTSTATUS(-1069416431i32);
pub const STATUS_BTH_ATT_INVALID_ATTRIBUTE_VALUE_LENGTH: NTSTATUS = NTSTATUS(-1069416435i32);
pub const STATUS_BTH_ATT_INVALID_HANDLE: NTSTATUS = NTSTATUS(-1069416447i32);
pub const STATUS_BTH_ATT_INVALID_OFFSET: NTSTATUS = NTSTATUS(-1069416441i32);
pub const STATUS_BTH_ATT_INVALID_PDU: NTSTATUS = NTSTATUS(-1069416444i32);
pub const STATUS_BTH_ATT_PREPARE_QUEUE_FULL: NTSTATUS = NTSTATUS(-1069416439i32);
pub const STATUS_BTH_ATT_READ_NOT_PERMITTED: NTSTATUS = NTSTATUS(-1069416446i32);
pub const STATUS_BTH_ATT_REQUEST_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1069416442i32);
pub const STATUS_BTH_ATT_UNKNOWN_ERROR: NTSTATUS = NTSTATUS(-1069412352i32);
pub const STATUS_BTH_ATT_UNLIKELY: NTSTATUS = NTSTATUS(-1069416434i32);
pub const STATUS_BTH_ATT_UNSUPPORTED_GROUP_TYPE: NTSTATUS = NTSTATUS(-1069416432i32);
pub const STATUS_BTH_ATT_WRITE_NOT_PERMITTED: NTSTATUS = NTSTATUS(-1069416445i32);
pub const STATUS_BUFFER_ALL_ZEROS: NTSTATUS = NTSTATUS(279i32);
pub const STATUS_BUFFER_OVERFLOW: NTSTATUS = NTSTATUS(-2147483643i32);
pub const STATUS_BUFFER_TOO_SMALL: NTSTATUS = NTSTATUS(-1073741789i32);
pub const STATUS_BUS_RESET: NTSTATUS = NTSTATUS(-2147483619i32);
pub const STATUS_BYPASSIO_FLT_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073740590i32);
pub const STATUS_CACHE_PAGE_LOCKED: NTSTATUS = NTSTATUS(277i32);
pub const STATUS_CALLBACK_BYPASS: NTSTATUS = NTSTATUS(-1073740541i32);
pub const STATUS_CALLBACK_INVOKE_INLINE: NTSTATUS = NTSTATUS(-1073740661i32);
pub const STATUS_CALLBACK_POP_STACK: NTSTATUS = NTSTATUS(-1073740765i32);
pub const STATUS_CALLBACK_RETURNED_LANG: NTSTATUS = NTSTATUS(-1073740001i32);
pub const STATUS_CALLBACK_RETURNED_LDR_LOCK: NTSTATUS = NTSTATUS(-1073740002i32);
pub const STATUS_CALLBACK_RETURNED_PRI_BACK: NTSTATUS = NTSTATUS(-1073740000i32);
pub const STATUS_CALLBACK_RETURNED_THREAD_AFFINITY: NTSTATUS = NTSTATUS(-1073739999i32);
pub const STATUS_CALLBACK_RETURNED_THREAD_PRIORITY: NTSTATUS = NTSTATUS(-1073740005i32);
pub const STATUS_CALLBACK_RETURNED_TRANSACTION: NTSTATUS = NTSTATUS(-1073740003i32);
pub const STATUS_CALLBACK_RETURNED_WHILE_IMPERSONATING: NTSTATUS = NTSTATUS(-1073740016i32);
pub const STATUS_CANCELLED: NTSTATUS = NTSTATUS(-1073741536i32);
pub const STATUS_CANNOT_ABORT_TRANSACTIONS: NTSTATUS = NTSTATUS(-1072103347i32);
pub const STATUS_CANNOT_ACCEPT_TRANSACTED_WORK: NTSTATUS = NTSTATUS(-1072103348i32);
pub const STATUS_CANNOT_BREAK_OPLOCK: NTSTATUS = NTSTATUS(-1073739511i32);
pub const STATUS_CANNOT_DELETE: NTSTATUS = NTSTATUS(-1073741535i32);
pub const STATUS_CANNOT_EXECUTE_FILE_IN_TRANSACTION: NTSTATUS = NTSTATUS(-1072103356i32);
pub const STATUS_CANNOT_GRANT_REQUESTED_OPLOCK: NTSTATUS = NTSTATUS(-2147483602i32);
pub const STATUS_CANNOT_IMPERSONATE: NTSTATUS = NTSTATUS(-1073741555i32);
pub const STATUS_CANNOT_LOAD_REGISTRY_FILE: NTSTATUS = NTSTATUS(-1073741288i32);
pub const STATUS_CANNOT_MAKE: NTSTATUS = NTSTATUS(-1073741078i32);
pub const STATUS_CANNOT_SWITCH_RUNLEVEL: NTSTATUS = NTSTATUS(-1073700543i32);
pub const STATUS_CANT_ACCESS_DOMAIN_INFO: NTSTATUS = NTSTATUS(-1073741606i32);
pub const STATUS_CANT_BREAK_TRANSACTIONAL_DEPENDENCY: NTSTATUS = NTSTATUS(-1072103369i32);
pub const STATUS_CANT_CLEAR_ENCRYPTION_FLAG: NTSTATUS = NTSTATUS(-1073740616i32);
pub const STATUS_CANT_CREATE_MORE_STREAM_MINIVERSIONS: NTSTATUS = NTSTATUS(-1072103386i32);
pub const STATUS_CANT_CROSS_RM_BOUNDARY: NTSTATUS = NTSTATUS(-1072103368i32);
pub const STATUS_CANT_DISABLE_MANDATORY: NTSTATUS = NTSTATUS(-1073741731i32);
pub const STATUS_CANT_ENABLE_DENY_ONLY: NTSTATUS = NTSTATUS(-1073741133i32);
pub const STATUS_CANT_OPEN_ANONYMOUS: NTSTATUS = NTSTATUS(-1073741658i32);
pub const STATUS_CANT_OPEN_MINIVERSION_WITH_MODIFY_INTENT: NTSTATUS = NTSTATUS(-1072103387i32);
pub const STATUS_CANT_RECOVER_WITH_HANDLE_OPEN: NTSTATUS = NTSTATUS(-2145845199i32);
pub const STATUS_CANT_TERMINATE_SELF: NTSTATUS = NTSTATUS(-1073741605i32);
pub const STATUS_CANT_WAIT: NTSTATUS = NTSTATUS(-1073741608i32);
pub const STATUS_CARDBUS_NOT_SUPPORTED: NTSTATUS = NTSTATUS(1073741863i32);
pub const STATUS_CASE_DIFFERING_NAMES_IN_DIR: NTSTATUS = NTSTATUS(-1073740621i32);
pub const STATUS_CASE_SENSITIVE_PATH: NTSTATUS = NTSTATUS(-1073740614i32);
pub const STATUS_CC_NEEDS_CALLBACK_SECTION_DRAIN: NTSTATUS = NTSTATUS(-1073700856i32);
pub const STATUS_CERTIFICATE_MAPPING_NOT_UNIQUE: NTSTATUS = NTSTATUS(-1073740012i32);
pub const STATUS_CERTIFICATE_VALIDATION_PREFERENCE_CONFLICT: NTSTATUS = NTSTATUS(-1073741387i32);
pub const STATUS_CHECKING_FILE_SYSTEM: NTSTATUS = NTSTATUS(1073741844i32);
pub const STATUS_CHECKOUT_REQUIRED: NTSTATUS = NTSTATUS(-1073739518i32);
pub const STATUS_CHILD_MUST_BE_VOLATILE: NTSTATUS = NTSTATUS(-1073741439i32);
pub const STATUS_CHILD_PROCESS_BLOCKED: NTSTATUS = NTSTATUS(-1073740643i32);
pub const STATUS_CIMFS_IMAGE_CORRUPT: NTSTATUS = NTSTATUS(-1073692671i32);
pub const STATUS_CIMFS_IMAGE_VERSION_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073692670i32);
pub const STATUS_CLEANER_CARTRIDGE_INSTALLED: NTSTATUS = NTSTATUS(-2147483609i32);
pub const STATUS_CLIENT_SERVER_PARAMETERS_INVALID: NTSTATUS = NTSTATUS(-1073741277i32);
pub const STATUS_CLIP_DEVICE_LICENSE_MISSING: NTSTATUS = NTSTATUS(-1058406397i32);
pub const STATUS_CLIP_KEYHOLDER_LICENSE_MISSING_OR_INVALID: NTSTATUS = NTSTATUS(-1058406395i32);
pub const STATUS_CLIP_LICENSE_DEVICE_ID_MISMATCH: NTSTATUS = NTSTATUS(-1058406390i32);
pub const STATUS_CLIP_LICENSE_EXPIRED: NTSTATUS = NTSTATUS(-1058406394i32);
pub const STATUS_CLIP_LICENSE_HARDWARE_ID_OUT_OF_TOLERANCE: NTSTATUS = NTSTATUS(-1058406391i32);
pub const STATUS_CLIP_LICENSE_INVALID_SIGNATURE: NTSTATUS = NTSTATUS(-1058406396i32);
pub const STATUS_CLIP_LICENSE_NOT_FOUND: NTSTATUS = NTSTATUS(-1058406398i32);
pub const STATUS_CLIP_LICENSE_NOT_SIGNED: NTSTATUS = NTSTATUS(-1058406392i32);
pub const STATUS_CLIP_LICENSE_SIGNED_BY_UNKNOWN_SOURCE: NTSTATUS = NTSTATUS(-1058406393i32);
pub const STATUS_CLOUD_FILE_ACCESS_DENIED: NTSTATUS = NTSTATUS(-1073688808i32);
pub const STATUS_CLOUD_FILE_ALREADY_CONNECTED: NTSTATUS = NTSTATUS(-1073688823i32);
pub const STATUS_CLOUD_FILE_AUTHENTICATION_FAILED: NTSTATUS = NTSTATUS(-1073688817i32);
pub const STATUS_CLOUD_FILE_CONNECTED_PROVIDER_ONLY: NTSTATUS = NTSTATUS(-1073688819i32);
pub const STATUS_CLOUD_FILE_DEHYDRATION_DISALLOWED: NTSTATUS = NTSTATUS(-1073688800i32);
pub const STATUS_CLOUD_FILE_INCOMPATIBLE_HARDLINKS: NTSTATUS = NTSTATUS(-1073688807i32);
pub const STATUS_CLOUD_FILE_INSUFFICIENT_RESOURCES: NTSTATUS = NTSTATUS(-1073688816i32);
pub const STATUS_CLOUD_FILE_INVALID_REQUEST: NTSTATUS = NTSTATUS(-1073688821i32);
pub const STATUS_CLOUD_FILE_IN_USE: NTSTATUS = NTSTATUS(-1073688812i32);
pub const STATUS_CLOUD_FILE_METADATA_CORRUPT: NTSTATUS = NTSTATUS(-1073688830i32);
pub const STATUS_CLOUD_FILE_METADATA_TOO_LARGE: NTSTATUS = NTSTATUS(-1073688829i32);
pub const STATUS_CLOUD_FILE_NETWORK_UNAVAILABLE: NTSTATUS = NTSTATUS(-1073688815i32);
pub const STATUS_CLOUD_FILE_NOT_IN_SYNC: NTSTATUS = NTSTATUS(-1073688824i32);
pub const STATUS_CLOUD_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073688822i32);
pub const STATUS_CLOUD_FILE_NOT_UNDER_SYNC_ROOT: NTSTATUS = NTSTATUS(-1073688813i32);
pub const STATUS_CLOUD_FILE_PINNED: NTSTATUS = NTSTATUS(-1073688811i32);
pub const STATUS_CLOUD_FILE_PROPERTY_BLOB_CHECKSUM_MISMATCH: NTSTATUS = NTSTATUS(-2147430656i32);
pub const STATUS_CLOUD_FILE_PROPERTY_BLOB_TOO_LARGE: NTSTATUS = NTSTATUS(-2147430652i32);
pub const STATUS_CLOUD_FILE_PROPERTY_CORRUPT: NTSTATUS = NTSTATUS(-1073688809i32);
pub const STATUS_CLOUD_FILE_PROPERTY_LOCK_CONFLICT: NTSTATUS = NTSTATUS(-1073688806i32);
pub const STATUS_CLOUD_FILE_PROPERTY_VERSION_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073688826i32);
pub const STATUS_CLOUD_FILE_PROVIDER_NOT_RUNNING: NTSTATUS = NTSTATUS(-1073688831i32);
pub const STATUS_CLOUD_FILE_PROVIDER_TERMINATED: NTSTATUS = NTSTATUS(-1073688803i32);
pub const STATUS_CLOUD_FILE_READ_ONLY_VOLUME: NTSTATUS = NTSTATUS(-1073688820i32);
pub const STATUS_CLOUD_FILE_REQUEST_ABORTED: NTSTATUS = NTSTATUS(-1073688810i32);
pub const STATUS_CLOUD_FILE_REQUEST_CANCELED: NTSTATUS = NTSTATUS(-1073688805i32);
pub const STATUS_CLOUD_FILE_REQUEST_TIMEOUT: NTSTATUS = NTSTATUS(-1073688801i32);
pub const STATUS_CLOUD_FILE_SYNC_ROOT_METADATA_CORRUPT: NTSTATUS = NTSTATUS(-1073688832i32);
pub const STATUS_CLOUD_FILE_TOO_MANY_PROPERTY_BLOBS: NTSTATUS = NTSTATUS(-2147430651i32);
pub const STATUS_CLOUD_FILE_UNSUCCESSFUL: NTSTATUS = NTSTATUS(-1073688814i32);
pub const STATUS_CLOUD_FILE_VALIDATION_FAILED: NTSTATUS = NTSTATUS(-1073688818i32);
pub const STATUS_CLUSTER_CAM_TICKET_REPLAY_DETECTED: NTSTATUS = NTSTATUS(-1072496591i32);
pub const STATUS_CLUSTER_CSV_AUTO_PAUSE_ERROR: NTSTATUS = NTSTATUS(-1072496607i32);
pub const STATUS_CLUSTER_CSV_INVALID_HANDLE: NTSTATUS = NTSTATUS(-1072496599i32);
pub const STATUS_CLUSTER_CSV_NOT_REDIRECTED: NTSTATUS = NTSTATUS(-1072496605i32);
pub const STATUS_CLUSTER_CSV_NO_SNAPSHOTS: NTSTATUS = NTSTATUS(-1072496601i32);
pub const STATUS_CLUSTER_CSV_READ_OPLOCK_BREAK_IN_PROGRESS: NTSTATUS = NTSTATUS(-1072496608i32);
pub const STATUS_CLUSTER_CSV_REDIRECTED: NTSTATUS = NTSTATUS(-1072496606i32);
pub const STATUS_CLUSTER_CSV_SNAPSHOT_CREATION_IN_PROGRESS: NTSTATUS = NTSTATUS(-1072496603i32);
pub const STATUS_CLUSTER_CSV_SUPPORTED_ONLY_ON_COORDINATOR: NTSTATUS = NTSTATUS(-1072496592i32);
pub const STATUS_CLUSTER_CSV_VOLUME_DRAINING: NTSTATUS = NTSTATUS(-1072496604i32);
pub const STATUS_CLUSTER_CSV_VOLUME_DRAINING_SUCCEEDED_DOWNLEVEL: NTSTATUS =
    NTSTATUS(-1072496602i32);
pub const STATUS_CLUSTER_CSV_VOLUME_NOT_LOCAL: NTSTATUS = NTSTATUS(-1072496615i32);
pub const STATUS_CLUSTER_INVALID_NETWORK: NTSTATUS = NTSTATUS(-1072496624i32);
pub const STATUS_CLUSTER_INVALID_NETWORK_PROVIDER: NTSTATUS = NTSTATUS(-1072496629i32);
pub const STATUS_CLUSTER_INVALID_NODE: NTSTATUS = NTSTATUS(-1072496639i32);
pub const STATUS_CLUSTER_INVALID_REQUEST: NTSTATUS = NTSTATUS(-1072496630i32);
pub const STATUS_CLUSTER_JOIN_IN_PROGRESS: NTSTATUS = NTSTATUS(-1072496637i32);
pub const STATUS_CLUSTER_JOIN_NOT_IN_PROGRESS: NTSTATUS = NTSTATUS(-1072496625i32);
pub const STATUS_CLUSTER_LOCAL_NODE_NOT_FOUND: NTSTATUS = NTSTATUS(-1072496635i32);
pub const STATUS_CLUSTER_NETINTERFACE_EXISTS: NTSTATUS = NTSTATUS(-1072496632i32);
pub const STATUS_CLUSTER_NETINTERFACE_NOT_FOUND: NTSTATUS = NTSTATUS(-1072496631i32);
pub const STATUS_CLUSTER_NETWORK_ALREADY_OFFLINE: NTSTATUS = NTSTATUS(-2146238460i32);
pub const STATUS_CLUSTER_NETWORK_ALREADY_ONLINE: NTSTATUS = NTSTATUS(-2146238461i32);
pub const STATUS_CLUSTER_NETWORK_EXISTS: NTSTATUS = NTSTATUS(-1072496634i32);
pub const STATUS_CLUSTER_NETWORK_NOT_FOUND: NTSTATUS = NTSTATUS(-1072496633i32);
pub const STATUS_CLUSTER_NETWORK_NOT_INTERNAL: NTSTATUS = NTSTATUS(-1072496618i32);
pub const STATUS_CLUSTER_NODE_ALREADY_DOWN: NTSTATUS = NTSTATUS(-2146238462i32);
pub const STATUS_CLUSTER_NODE_ALREADY_MEMBER: NTSTATUS = NTSTATUS(-2146238459i32);
pub const STATUS_CLUSTER_NODE_ALREADY_UP: NTSTATUS = NTSTATUS(-2146238463i32);
pub const STATUS_CLUSTER_NODE_DOWN: NTSTATUS = NTSTATUS(-1072496628i32);
pub const STATUS_CLUSTER_NODE_EXISTS: NTSTATUS = NTSTATUS(-1072496638i32);
pub const STATUS_CLUSTER_NODE_NOT_FOUND: NTSTATUS = NTSTATUS(-1072496636i32);
pub const STATUS_CLUSTER_NODE_NOT_MEMBER: NTSTATUS = NTSTATUS(-1072496626i32);
pub const STATUS_CLUSTER_NODE_NOT_PAUSED: NTSTATUS = NTSTATUS(-1072496620i32);
pub const STATUS_CLUSTER_NODE_PAUSED: NTSTATUS = NTSTATUS(-1072496621i32);
pub const STATUS_CLUSTER_NODE_UNREACHABLE: NTSTATUS = NTSTATUS(-1072496627i32);
pub const STATUS_CLUSTER_NODE_UP: NTSTATUS = NTSTATUS(-1072496622i32);
pub const STATUS_CLUSTER_NON_CSV_PATH: NTSTATUS = NTSTATUS(-1072496616i32);
pub const STATUS_CLUSTER_NO_NET_ADAPTERS: NTSTATUS = NTSTATUS(-1072496623i32);
pub const STATUS_CLUSTER_NO_SECURITY_CONTEXT: NTSTATUS = NTSTATUS(-1072496619i32);
pub const STATUS_CLUSTER_POISONED: NTSTATUS = NTSTATUS(-1072496617i32);
pub const STATUS_COMMITMENT_LIMIT: NTSTATUS = NTSTATUS(-1073741523i32);
pub const STATUS_COMMITMENT_MINIMUM: NTSTATUS = NTSTATUS(-1073741112i32);
pub const STATUS_COMPRESSED_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073740677i32);
pub const STATUS_COMPRESSION_DISABLED: NTSTATUS = NTSTATUS(-1073740762i32);
pub const STATUS_COMPRESSION_NOT_ALLOWED_IN_TRANSACTION: NTSTATUS = NTSTATUS(-1072103338i32);
pub const STATUS_COMPRESSION_NOT_BENEFICIAL: NTSTATUS = NTSTATUS(-1073740689i32);
pub const STATUS_CONFLICTING_ADDRESSES: NTSTATUS = NTSTATUS(-1073741800i32);
pub const STATUS_CONNECTION_ABORTED: NTSTATUS = NTSTATUS(-1073741247i32);
pub const STATUS_CONNECTION_ACTIVE: NTSTATUS = NTSTATUS(-1073741253i32);
pub const STATUS_CONNECTION_COUNT_LIMIT: NTSTATUS = NTSTATUS(-1073741242i32);
pub const STATUS_CONNECTION_DISCONNECTED: NTSTATUS = NTSTATUS(-1073741300i32);
pub const STATUS_CONNECTION_INVALID: NTSTATUS = NTSTATUS(-1073741254i32);
pub const STATUS_CONNECTION_IN_USE: NTSTATUS = NTSTATUS(-1073741560i32);
pub const STATUS_CONNECTION_REFUSED: NTSTATUS = NTSTATUS(-1073741258i32);
pub const STATUS_CONNECTION_RESET: NTSTATUS = NTSTATUS(-1073741299i32);
pub const STATUS_CONTAINER_ASSIGNED: NTSTATUS = NTSTATUS(-1073740536i32);
pub const STATUS_CONTENT_BLOCKED: NTSTATUS = NTSTATUS(-1073739772i32);
pub const STATUS_CONTEXT_MISMATCH: NTSTATUS = NTSTATUS(-1073740007i32);
pub const STATUS_CONTEXT_STOWED_EXCEPTION: NTSTATUS = NTSTATUS(-1073741188i32);
pub const STATUS_CONTROL_C_EXIT: NTSTATUS = NTSTATUS(-1073741510i32);
pub const STATUS_CONTROL_STACK_VIOLATION: NTSTATUS = NTSTATUS(-1073741390i32);
pub const STATUS_CONVERT_TO_LARGE: NTSTATUS = NTSTATUS(-1073741268i32);
pub const STATUS_COPY_PROTECTION_FAILURE: NTSTATUS = NTSTATUS(-1073741051i32);
pub const STATUS_CORRUPT_LOG_CLEARED: NTSTATUS = NTSTATUS(-1073739763i32);
pub const STATUS_CORRUPT_LOG_CORRUPTED: NTSTATUS = NTSTATUS(-1073739766i32);
pub const STATUS_CORRUPT_LOG_DELETED_FULL: NTSTATUS = NTSTATUS(-1073739764i32);
pub const STATUS_CORRUPT_LOG_OVERFULL: NTSTATUS = NTSTATUS(-1073739767i32);
pub const STATUS_CORRUPT_LOG_UNAVAILABLE: NTSTATUS = NTSTATUS(-1073739765i32);
pub const STATUS_CORRUPT_LOG_UPLEVEL_RECORDS: NTSTATUS = NTSTATUS(-1073739759i32);
pub const STATUS_CORRUPT_SYSTEM_FILE: NTSTATUS = NTSTATUS(-1073741116i32);
pub const STATUS_COULD_NOT_INTERPRET: NTSTATUS = NTSTATUS(-1073741639i32);
pub const STATUS_COULD_NOT_RESIZE_LOG: NTSTATUS = NTSTATUS(-2145845239i32);
pub const STATUS_CPU_SET_INVALID: NTSTATUS = NTSTATUS(-1073741393i32);
pub const STATUS_CRASH_DUMP: NTSTATUS = NTSTATUS(278i32);
pub const STATUS_CRC_ERROR: NTSTATUS = NTSTATUS(-1073741761i32);
pub const STATUS_CRED_REQUIRES_CONFIRMATION: NTSTATUS = NTSTATUS(-1073740736i32);
pub const STATUS_CRM_PROTOCOL_ALREADY_EXISTS: NTSTATUS = NTSTATUS(-1072103409i32);
pub const STATUS_CRM_PROTOCOL_NOT_FOUND: NTSTATUS = NTSTATUS(-1072103407i32);
pub const STATUS_CROSSREALM_DELEGATION_FAILURE: NTSTATUS = NTSTATUS(-1073740789i32);
pub const STATUS_CROSS_PARTITION_VIOLATION: NTSTATUS = NTSTATUS(-1073740277i32);
pub const STATUS_CRYPTO_SYSTEM_INVALID: NTSTATUS = NTSTATUS(-1073741069i32);
pub const STATUS_CSS_AUTHENTICATION_FAILURE: NTSTATUS = NTSTATUS(-1073741050i32);
pub const STATUS_CSS_KEY_NOT_ESTABLISHED: NTSTATUS = NTSTATUS(-1073741048i32);
pub const STATUS_CSS_KEY_NOT_PRESENT: NTSTATUS = NTSTATUS(-1073741049i32);
pub const STATUS_CSS_REGION_MISMATCH: NTSTATUS = NTSTATUS(-1073741046i32);
pub const STATUS_CSS_RESETS_EXHAUSTED: NTSTATUS = NTSTATUS(-1073741045i32);
pub const STATUS_CSS_SCRAMBLED_SECTOR: NTSTATUS = NTSTATUS(-1073741047i32);
pub const STATUS_CSV_IO_PAUSE_TIMEOUT: NTSTATUS = NTSTATUS(-1072496600i32);
pub const STATUS_CS_ENCRYPTION_EXISTING_ENCRYPTED_FILE: NTSTATUS = NTSTATUS(-1073740733i32);
pub const STATUS_CS_ENCRYPTION_FILE_NOT_CSE: NTSTATUS = NTSTATUS(-1073740731i32);
pub const STATUS_CS_ENCRYPTION_INVALID_SERVER_RESPONSE: NTSTATUS = NTSTATUS(-1073740735i32);
pub const STATUS_CS_ENCRYPTION_NEW_ENCRYPTED_FILE: NTSTATUS = NTSTATUS(-1073740732i32);
pub const STATUS_CS_ENCRYPTION_UNSUPPORTED_SERVER: NTSTATUS = NTSTATUS(-1073740734i32);
pub const STATUS_CTLOG_INCONSISTENT_TRACKING_FILE: NTSTATUS = NTSTATUS(-1069940700i32);
pub const STATUS_CTLOG_INVALID_TRACKING_STATE: NTSTATUS = NTSTATUS(-1069940701i32);
pub const STATUS_CTLOG_LOGFILE_SIZE_EXCEEDED_MAXSIZE: NTSTATUS = NTSTATUS(-1069940703i32);
pub const STATUS_CTLOG_TRACKING_NOT_INITIALIZED: NTSTATUS = NTSTATUS(-1069940704i32);
pub const STATUS_CTLOG_VHD_CHANGED_OFFLINE: NTSTATUS = NTSTATUS(-1069940702i32);
pub const STATUS_CTL_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073741737i32);
pub const STATUS_CTX_BAD_VIDEO_MODE: NTSTATUS = NTSTATUS(-1073086440i32);
pub const STATUS_CTX_CDM_CONNECT: NTSTATUS = NTSTATUS(1074397188i32);
pub const STATUS_CTX_CDM_DISCONNECT: NTSTATUS = NTSTATUS(1074397189i32);
pub const STATUS_CTX_CLIENT_LICENSE_IN_USE: NTSTATUS = NTSTATUS(-1073086412i32);
pub const STATUS_CTX_CLIENT_LICENSE_NOT_SET: NTSTATUS = NTSTATUS(-1073086413i32);
pub const STATUS_CTX_CLIENT_QUERY_TIMEOUT: NTSTATUS = NTSTATUS(-1073086426i32);
pub const STATUS_CTX_CLOSE_PENDING: NTSTATUS = NTSTATUS(-1073086458i32);
pub const STATUS_CTX_CONSOLE_CONNECT: NTSTATUS = NTSTATUS(-1073086424i32);
pub const STATUS_CTX_CONSOLE_DISCONNECT: NTSTATUS = NTSTATUS(-1073086425i32);
pub const STATUS_CTX_GRAPHICS_INVALID: NTSTATUS = NTSTATUS(-1073086430i32);
pub const STATUS_CTX_INVALID_MODEMNAME: NTSTATUS = NTSTATUS(-1073086455i32);
pub const STATUS_CTX_INVALID_PD: NTSTATUS = NTSTATUS(-1073086462i32);
pub const STATUS_CTX_INVALID_WD: NTSTATUS = NTSTATUS(-1073086418i32);
pub const STATUS_CTX_LICENSE_CLIENT_INVALID: NTSTATUS = NTSTATUS(-1073086446i32);
pub const STATUS_CTX_LICENSE_EXPIRED: NTSTATUS = NTSTATUS(-1073086444i32);
pub const STATUS_CTX_LICENSE_NOT_AVAILABLE: NTSTATUS = NTSTATUS(-1073086445i32);
pub const STATUS_CTX_LOGON_DISABLED: NTSTATUS = NTSTATUS(-1073086409i32);
pub const STATUS_CTX_MODEM_INF_NOT_FOUND: NTSTATUS = NTSTATUS(-1073086456i32);
pub const STATUS_CTX_MODEM_RESPONSE_BUSY: NTSTATUS = NTSTATUS(-1073086450i32);
pub const STATUS_CTX_MODEM_RESPONSE_NO_CARRIER: NTSTATUS = NTSTATUS(-1073086452i32);
pub const STATUS_CTX_MODEM_RESPONSE_NO_DIALTONE: NTSTATUS = NTSTATUS(-1073086451i32);
pub const STATUS_CTX_MODEM_RESPONSE_TIMEOUT: NTSTATUS = NTSTATUS(-1073086453i32);
pub const STATUS_CTX_MODEM_RESPONSE_VOICE: NTSTATUS = NTSTATUS(-1073086449i32);
pub const STATUS_CTX_NOT_CONSOLE: NTSTATUS = NTSTATUS(-1073086428i32);
pub const STATUS_CTX_NO_OUTBUF: NTSTATUS = NTSTATUS(-1073086457i32);
pub const STATUS_CTX_PD_NOT_FOUND: NTSTATUS = NTSTATUS(-1073086461i32);
pub const STATUS_CTX_RESPONSE_ERROR: NTSTATUS = NTSTATUS(-1073086454i32);
pub const STATUS_CTX_SECURITY_LAYER_ERROR: NTSTATUS = NTSTATUS(-1073086408i32);
pub const STATUS_CTX_SHADOW_DENIED: NTSTATUS = NTSTATUS(-1073086422i32);
pub const STATUS_CTX_SHADOW_DISABLED: NTSTATUS = NTSTATUS(-1073086415i32);
pub const STATUS_CTX_SHADOW_ENDED_BY_MODE_CHANGE: NTSTATUS = NTSTATUS(-1073086411i32);
pub const STATUS_CTX_SHADOW_INVALID: NTSTATUS = NTSTATUS(-1073086416i32);
pub const STATUS_CTX_SHADOW_NOT_RUNNING: NTSTATUS = NTSTATUS(-1073086410i32);
pub const STATUS_CTX_TD_ERROR: NTSTATUS = NTSTATUS(-1073086448i32);
pub const STATUS_CTX_WD_NOT_FOUND: NTSTATUS = NTSTATUS(-1073086417i32);
pub const STATUS_CTX_WINSTATION_ACCESS_DENIED: NTSTATUS = NTSTATUS(-1073086421i32);
pub const STATUS_CTX_WINSTATION_BUSY: NTSTATUS = NTSTATUS(-1073086441i32);
pub const STATUS_CTX_WINSTATION_NAME_COLLISION: NTSTATUS = NTSTATUS(-1073086442i32);
pub const STATUS_CTX_WINSTATION_NAME_INVALID: NTSTATUS = NTSTATUS(-1073086463i32);
pub const STATUS_CTX_WINSTATION_NOT_FOUND: NTSTATUS = NTSTATUS(-1073086443i32);
pub const STATUS_CURRENT_DOMAIN_NOT_ALLOWED: NTSTATUS = NTSTATUS(-1073741079i32);
pub const STATUS_CURRENT_TRANSACTION_NOT_VALID: NTSTATUS = NTSTATUS(-1072103400i32);
pub const STATUS_DATATYPE_MISALIGNMENT: NTSTATUS = NTSTATUS(-2147483646i32);
pub const STATUS_DATATYPE_MISALIGNMENT_ERROR: NTSTATUS = NTSTATUS(-1073741115i32);
pub const STATUS_DATA_CHECKSUM_ERROR: NTSTATUS = NTSTATUS(-1073740688i32);
pub const STATUS_DATA_ERROR: NTSTATUS = NTSTATUS(-1073741762i32);
pub const STATUS_DATA_LATE_ERROR: NTSTATUS = NTSTATUS(-1073741763i32);
pub const STATUS_DATA_LOST_REPAIR: NTSTATUS = NTSTATUS(-2147481597i32);
pub const STATUS_DATA_NOT_ACCEPTED: NTSTATUS = NTSTATUS(-1073741285i32);
pub const STATUS_DATA_OVERRUN: NTSTATUS = NTSTATUS(-1073741764i32);
pub const STATUS_DATA_OVERWRITTEN: NTSTATUS = NTSTATUS(304i32);
pub const STATUS_DAX_MAPPING_EXISTS: NTSTATUS = NTSTATUS(-1073740644i32);
pub const STATUS_DEBUGGER_INACTIVE: NTSTATUS = NTSTATUS(-1073740972i32);
pub const STATUS_DEBUG_ATTACH_FAILED: NTSTATUS = NTSTATUS(-1073741287i32);
pub const STATUS_DECRYPTION_FAILED: NTSTATUS = NTSTATUS(-1073741173i32);
pub const STATUS_DELAY_LOAD_FAILED: NTSTATUS = NTSTATUS(-1073740782i32);
pub const STATUS_DELETE_PENDING: NTSTATUS = NTSTATUS(-1073741738i32);
pub const STATUS_DESTINATION_ELEMENT_FULL: NTSTATUS = NTSTATUS(-1073741180i32);
pub const STATUS_DEVICE_ALREADY_ATTACHED: NTSTATUS = NTSTATUS(-1073741768i32);
pub const STATUS_DEVICE_BUSY: NTSTATUS = NTSTATUS(-2147483631i32);
pub const STATUS_DEVICE_CONFIGURATION_ERROR: NTSTATUS = NTSTATUS(-1073741438i32);
pub const STATUS_DEVICE_DATA_ERROR: NTSTATUS = NTSTATUS(-1073741668i32);
pub const STATUS_DEVICE_DOES_NOT_EXIST: NTSTATUS = NTSTATUS(-1073741632i32);
pub const STATUS_DEVICE_DOOR_OPEN: NTSTATUS = NTSTATUS(-2147482999i32);
pub const STATUS_DEVICE_ENUMERATION_ERROR: NTSTATUS = NTSTATUS(-1073740954i32);
pub const STATUS_DEVICE_FEATURE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073740701i32);
pub const STATUS_DEVICE_HARDWARE_ERROR: NTSTATUS = NTSTATUS(-1073740669i32);
pub const STATUS_DEVICE_HINT_NAME_BUFFER_TOO_SMALL: NTSTATUS = NTSTATUS(-1073740650i32);
pub const STATUS_DEVICE_HUNG: NTSTATUS = NTSTATUS(-1073740537i32);
pub const STATUS_DEVICE_INSUFFICIENT_RESOURCES: NTSTATUS = NTSTATUS(-1073740696i32);
pub const STATUS_DEVICE_IN_MAINTENANCE: NTSTATUS = NTSTATUS(-1073740647i32);
pub const STATUS_DEVICE_NOT_CONNECTED: NTSTATUS = NTSTATUS(-1073741667i32);
pub const STATUS_DEVICE_NOT_PARTITIONED: NTSTATUS = NTSTATUS(-1073741452i32);
pub const STATUS_DEVICE_NOT_READY: NTSTATUS = NTSTATUS(-1073741661i32);
pub const STATUS_DEVICE_OFF_LINE: NTSTATUS = NTSTATUS(-2147483632i32);
pub const STATUS_DEVICE_PAPER_EMPTY: NTSTATUS = NTSTATUS(-2147483634i32);
pub const STATUS_DEVICE_POWERED_OFF: NTSTATUS = NTSTATUS(-2147483633i32);
pub const STATUS_DEVICE_POWER_CYCLE_REQUIRED: NTSTATUS = NTSTATUS(-2147483599i32);
pub const STATUS_DEVICE_POWER_FAILURE: NTSTATUS = NTSTATUS(-1073741666i32);
pub const STATUS_DEVICE_PROTOCOL_ERROR: NTSTATUS = NTSTATUS(-1073741434i32);
pub const STATUS_DEVICE_REMOVED: NTSTATUS = NTSTATUS(-1073741130i32);
pub const STATUS_DEVICE_REQUIRES_CLEANING: NTSTATUS = NTSTATUS(-2147483000i32);
pub const STATUS_DEVICE_RESET_REQUIRED: NTSTATUS = NTSTATUS(-2147483210i32);
pub const STATUS_DEVICE_SUPPORT_IN_PROGRESS: NTSTATUS = NTSTATUS(-2147483600i32);
pub const STATUS_DEVICE_UNREACHABLE: NTSTATUS = NTSTATUS(-1073740700i32);
pub const STATUS_DEVICE_UNRESPONSIVE: NTSTATUS = NTSTATUS(-1073740534i32);
pub const STATUS_DFS_EXIT_PATH_FOUND: NTSTATUS = NTSTATUS(-1073741669i32);
pub const STATUS_DFS_UNAVAILABLE: NTSTATUS = NTSTATUS(-1073741203i32);
pub const STATUS_DIF_BINDING_API_NOT_FOUND: NTSTATUS = NTSTATUS(-1073738625i32);
pub const STATUS_DIF_IOCALLBACK_NOT_REPLACED: NTSTATUS = NTSTATUS(-1073738634i32);
pub const STATUS_DIF_LIVEDUMP_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(-1073738633i32);
pub const STATUS_DIF_VOLATILE_DRIVER_HOTPATCHED: NTSTATUS = NTSTATUS(-1073738631i32);
pub const STATUS_DIF_VOLATILE_DRIVER_IS_NOT_RUNNING: NTSTATUS = NTSTATUS(-1073738629i32);
pub const STATUS_DIF_VOLATILE_INVALID_INFO: NTSTATUS = NTSTATUS(-1073738630i32);
pub const STATUS_DIF_VOLATILE_NOT_ALLOWED: NTSTATUS = NTSTATUS(-1073738626i32);
pub const STATUS_DIF_VOLATILE_PLUGIN_CHANGE_NOT_ALLOWED: NTSTATUS = NTSTATUS(-1073738627i32);
pub const STATUS_DIF_VOLATILE_PLUGIN_IS_NOT_RUNNING: NTSTATUS = NTSTATUS(-1073738628i32);
pub const STATUS_DIF_VOLATILE_SECTION_NOT_LOCKED: NTSTATUS = NTSTATUS(-1073738632i32);
pub const STATUS_DIRECTORY_IS_A_REPARSE_POINT: NTSTATUS = NTSTATUS(-1073741183i32);
pub const STATUS_DIRECTORY_NOT_EMPTY: NTSTATUS = NTSTATUS(-1073741567i32);
pub const STATUS_DIRECTORY_NOT_RM: NTSTATUS = NTSTATUS(-1072103416i32);
pub const STATUS_DIRECTORY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073740676i32);
pub const STATUS_DIRECTORY_SERVICE_REQUIRED: NTSTATUS = NTSTATUS(-1073741135i32);
pub const STATUS_DISK_CORRUPT_ERROR: NTSTATUS = NTSTATUS(-1073741774i32);
pub const STATUS_DISK_FULL: NTSTATUS = NTSTATUS(-1073741697i32);
pub const STATUS_DISK_OPERATION_FAILED: NTSTATUS = NTSTATUS(-1073741462i32);
pub const STATUS_DISK_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(-1073739774i32);
pub const STATUS_DISK_RECALIBRATE_FAILED: NTSTATUS = NTSTATUS(-1073741463i32);
pub const STATUS_DISK_REPAIR_DISABLED: NTSTATUS = NTSTATUS(-1073739776i32);
pub const STATUS_DISK_REPAIR_REDIRECTED: NTSTATUS = NTSTATUS(1073743879i32);
pub const STATUS_DISK_REPAIR_UNSUCCESSFUL: NTSTATUS = NTSTATUS(-1073739768i32);
pub const STATUS_DISK_RESET_FAILED: NTSTATUS = NTSTATUS(-1073741461i32);
pub const STATUS_DISK_RESOURCES_EXHAUSTED: NTSTATUS = NTSTATUS(-1073740703i32);
pub const STATUS_DLL_INIT_FAILED: NTSTATUS = NTSTATUS(-1073741502i32);
pub const STATUS_DLL_INIT_FAILED_LOGOFF: NTSTATUS = NTSTATUS(-1073741205i32);
pub const STATUS_DLL_MIGHT_BE_INCOMPATIBLE: NTSTATUS = NTSTATUS(-2147483604i32);
pub const STATUS_DLL_MIGHT_BE_INSECURE: NTSTATUS = NTSTATUS(-2147483605i32);
pub const STATUS_DLL_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741515i32);
pub const STATUS_DOMAIN_CONTROLLER_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741261i32);
pub const STATUS_DOMAIN_CTRLR_CONFIG_ERROR: NTSTATUS = NTSTATUS(-1073741474i32);
pub const STATUS_DOMAIN_EXISTS: NTSTATUS = NTSTATUS(-1073741600i32);
pub const STATUS_DOMAIN_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(-1073741599i32);
pub const STATUS_DOMAIN_TRUST_INCONSISTENT: NTSTATUS = NTSTATUS(-1073741413i32);
pub const STATUS_DRIVERS_LEAKING_LOCKED_PAGES: NTSTATUS = NTSTATUS(1073741869i32);
pub const STATUS_DRIVER_BLOCKED: NTSTATUS = NTSTATUS(-1073740948i32);
pub const STATUS_DRIVER_BLOCKED_CRITICAL: NTSTATUS = NTSTATUS(-1073740949i32);
pub const STATUS_DRIVER_CANCEL_TIMEOUT: NTSTATUS = NTSTATUS(-1073741282i32);
pub const STATUS_DRIVER_DATABASE_ERROR: NTSTATUS = NTSTATUS(-1073740947i32);
pub const STATUS_DRIVER_ENTRYPOINT_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741213i32);
pub const STATUS_DRIVER_FAILED_PRIOR_UNLOAD: NTSTATUS = NTSTATUS(-1073740914i32);
pub const STATUS_DRIVER_FAILED_SLEEP: NTSTATUS = NTSTATUS(-1073741118i32);
pub const STATUS_DRIVER_INTERNAL_ERROR: NTSTATUS = NTSTATUS(-1073741437i32);
pub const STATUS_DRIVER_ORDINAL_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741214i32);
pub const STATUS_DRIVER_PROCESS_TERMINATED: NTSTATUS = NTSTATUS(-1073740720i32);
pub const STATUS_DRIVER_UNABLE_TO_LOAD: NTSTATUS = NTSTATUS(-1073741204i32);
pub const STATUS_DS_ADMIN_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(-1073741119i32);
pub const STATUS_DS_AG_CANT_HAVE_UNIVERSAL_MEMBER: NTSTATUS = NTSTATUS(-1073740968i32);
pub const STATUS_DS_ATTRIBUTE_OR_VALUE_EXISTS: NTSTATUS = NTSTATUS(-1073741148i32);
pub const STATUS_DS_ATTRIBUTE_TYPE_UNDEFINED: NTSTATUS = NTSTATUS(-1073741149i32);
pub const STATUS_DS_BUSY: NTSTATUS = NTSTATUS(-1073741147i32);
pub const STATUS_DS_CANT_MOD_OBJ_CLASS: NTSTATUS = NTSTATUS(-1073741138i32);
pub const STATUS_DS_CANT_MOD_PRIMARYGROUPID: NTSTATUS = NTSTATUS(-1073741104i32);
pub const STATUS_DS_CANT_ON_NON_LEAF: NTSTATUS = NTSTATUS(-1073741140i32);
pub const STATUS_DS_CANT_ON_RDN: NTSTATUS = NTSTATUS(-1073741139i32);
pub const STATUS_DS_CANT_START: NTSTATUS = NTSTATUS(-1073741087i32);
pub const STATUS_DS_CROSS_DOM_MOVE_FAILED: NTSTATUS = NTSTATUS(-1073741137i32);
pub const STATUS_DS_DOMAIN_NAME_EXISTS_IN_FOREST: NTSTATUS = NTSTATUS(-1073740774i32);
pub const STATUS_DS_DOMAIN_RENAME_IN_PROGRESS: NTSTATUS = NTSTATUS(-1073739775i32);
pub const STATUS_DS_DUPLICATE_ID_FOUND: NTSTATUS = NTSTATUS(-1073740795i32);
pub const STATUS_DS_FLAT_NAME_EXISTS_IN_FOREST: NTSTATUS = NTSTATUS(-1073740773i32);
pub const STATUS_DS_GC_NOT_AVAILABLE: NTSTATUS = NTSTATUS(-1073741136i32);
pub const STATUS_DS_GC_REQUIRED: NTSTATUS = NTSTATUS(-1073741084i32);
pub const STATUS_DS_GLOBAL_CANT_HAVE_CROSSDOMAIN_MEMBER: NTSTATUS = NTSTATUS(-1073741094i32);
pub const STATUS_DS_GLOBAL_CANT_HAVE_LOCAL_MEMBER: NTSTATUS = NTSTATUS(-1073741097i32);
pub const STATUS_DS_GLOBAL_CANT_HAVE_UNIVERSAL_MEMBER: NTSTATUS = NTSTATUS(-1073741096i32);
pub const STATUS_DS_GROUP_CONVERSION_ERROR: NTSTATUS = NTSTATUS(-1073740794i32);
pub const STATUS_DS_HAVE_PRIMARY_MEMBERS: NTSTATUS = NTSTATUS(-1073741092i32);
pub const STATUS_DS_INCORRECT_ROLE_OWNER: NTSTATUS = NTSTATUS(-1073741143i32);
pub const STATUS_DS_INIT_FAILURE: NTSTATUS = NTSTATUS(-1073741086i32);
pub const STATUS_DS_INIT_FAILURE_CONSOLE: NTSTATUS = NTSTATUS(-1073741076i32);
pub const STATUS_DS_INVALID_ATTRIBUTE_SYNTAX: NTSTATUS = NTSTATUS(-1073741150i32);
pub const STATUS_DS_INVALID_GROUP_TYPE: NTSTATUS = NTSTATUS(-1073741100i32);
pub const STATUS_DS_LOCAL_CANT_HAVE_CROSSDOMAIN_LOCAL_MEMBER: NTSTATUS = NTSTATUS(-1073741093i32);
pub const STATUS_DS_LOCAL_MEMBER_OF_LOCAL_ONLY: NTSTATUS = NTSTATUS(-1073741083i32);
pub const STATUS_DS_MACHINE_ACCOUNT_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(-1073741081i32);
pub const STATUS_DS_MEMBERSHIP_EVALUATED_LOCALLY: NTSTATUS = NTSTATUS(289i32);
pub const STATUS_DS_NAME_NOT_UNIQUE: NTSTATUS = NTSTATUS(-1073740796i32);
pub const STATUS_DS_NO_ATTRIBUTE_OR_VALUE: NTSTATUS = NTSTATUS(-1073741151i32);
pub const STATUS_DS_NO_FPO_IN_UNIVERSAL_GROUPS: NTSTATUS = NTSTATUS(-1073741082i32);
pub const STATUS_DS_NO_MORE_RIDS: NTSTATUS = NTSTATUS(-1073741144i32);
pub const STATUS_DS_NO_NEST_GLOBALGROUP_IN_MIXEDDOMAIN: NTSTATUS = NTSTATUS(-1073741099i32);
pub const STATUS_DS_NO_NEST_LOCALGROUP_IN_MIXEDDOMAIN: NTSTATUS = NTSTATUS(-1073741098i32);
pub const STATUS_DS_NO_RIDS_ALLOCATED: NTSTATUS = NTSTATUS(-1073741145i32);
pub const STATUS_DS_OBJ_CLASS_VIOLATION: NTSTATUS = NTSTATUS(-1073741141i32);
pub const STATUS_DS_OID_MAPPED_GROUP_CANT_HAVE_MEMBERS: NTSTATUS = NTSTATUS(-1073700729i32);
pub const STATUS_DS_OID_NOT_FOUND: NTSTATUS = NTSTATUS(-1073700728i32);
pub const STATUS_DS_RIDMGR_DISABLED: NTSTATUS = NTSTATUS(-1073741126i32);
pub const STATUS_DS_RIDMGR_INIT_ERROR: NTSTATUS = NTSTATUS(-1073741142i32);
pub const STATUS_DS_SAM_INIT_FAILURE: NTSTATUS = NTSTATUS(-1073741109i32);
pub const STATUS_DS_SAM_INIT_FAILURE_CONSOLE: NTSTATUS = NTSTATUS(-1073741075i32);
pub const STATUS_DS_SENSITIVE_GROUP_VIOLATION: NTSTATUS = NTSTATUS(-1073741107i32);
pub const STATUS_DS_SHUTTING_DOWN: NTSTATUS = NTSTATUS(1073742704i32);
pub const STATUS_DS_SRC_SID_EXISTS_IN_FOREST: NTSTATUS = NTSTATUS(-1073740775i32);
pub const STATUS_DS_UNAVAILABLE: NTSTATUS = NTSTATUS(-1073741146i32);
pub const STATUS_DS_UNIVERSAL_CANT_HAVE_LOCAL_MEMBER: NTSTATUS = NTSTATUS(-1073741095i32);
pub const STATUS_DS_VERSION_CHECK_FAILURE: NTSTATUS = NTSTATUS(-1073740971i32);
pub const STATUS_DUPLICATE_NAME: NTSTATUS = NTSTATUS(-1073741635i32);
pub const STATUS_DUPLICATE_OBJECTID: NTSTATUS = NTSTATUS(-1073741270i32);
pub const STATUS_DUPLICATE_PRIVILEGES: NTSTATUS = NTSTATUS(-1073741402i32);
pub const STATUS_DYNAMIC_CODE_BLOCKED: NTSTATUS = NTSTATUS(-1073740284i32);
pub const STATUS_EAS_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073741745i32);
pub const STATUS_EA_CORRUPT_ERROR: NTSTATUS = NTSTATUS(-1073741741i32);
pub const STATUS_EA_LIST_INCONSISTENT: NTSTATUS = NTSTATUS(-2147483628i32);
pub const STATUS_EA_TOO_LARGE: NTSTATUS = NTSTATUS(-1073741744i32);
pub const STATUS_EFS_ALG_BLOB_TOO_BIG: NTSTATUS = NTSTATUS(-1073740974i32);
pub const STATUS_EFS_NOT_ALLOWED_IN_TRANSACTION: NTSTATUS = NTSTATUS(-1072103362i32);
pub const STATUS_ELEVATION_REQUIRED: NTSTATUS = NTSTATUS(-1073740756i32);
pub const STATUS_EMULATION_BREAKPOINT: NTSTATUS = NTSTATUS(1073741880i32);
pub const STATUS_EMULATION_SYSCALL: NTSTATUS = NTSTATUS(1073741881i32);
pub const STATUS_ENCLAVE_FAILURE: NTSTATUS = NTSTATUS(-1073740657i32);
pub const STATUS_ENCLAVE_IS_TERMINATING: NTSTATUS = NTSTATUS(-1073740526i32);
pub const STATUS_ENCLAVE_NOT_TERMINATED: NTSTATUS = NTSTATUS(-1073740527i32);
pub const STATUS_ENCLAVE_VIOLATION: NTSTATUS = NTSTATUS(-1073740638i32);
pub const STATUS_ENCOUNTERED_WRITE_IN_PROGRESS: NTSTATUS = NTSTATUS(-1073740749i32);
pub const STATUS_ENCRYPTED_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073740605i32);
pub const STATUS_ENCRYPTED_IO_NOT_POSSIBLE: NTSTATUS = NTSTATUS(-1073739760i32);
pub const STATUS_ENCRYPTING_METADATA_DISALLOWED: NTSTATUS = NTSTATUS(-1073740617i32);
pub const STATUS_ENCRYPTION_DISABLED: NTSTATUS = NTSTATUS(-1073740618i32);
pub const STATUS_ENCRYPTION_FAILED: NTSTATUS = NTSTATUS(-1073741174i32);
pub const STATUS_END_OF_FILE: NTSTATUS = NTSTATUS(-1073741807i32);
pub const STATUS_END_OF_MEDIA: NTSTATUS = NTSTATUS(-2147483618i32);
pub const STATUS_ENLISTMENT_NOT_FOUND: NTSTATUS = NTSTATUS(-1072103344i32);
pub const STATUS_ENLISTMENT_NOT_SUPERIOR: NTSTATUS = NTSTATUS(-1072103373i32);
pub const STATUS_ENTRYPOINT_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741511i32);
pub const STATUS_EOF_ON_GHOSTED_RANGE: NTSTATUS = NTSTATUS(-1073700857i32);
pub const STATUS_EOM_OVERFLOW: NTSTATUS = NTSTATUS(-1073741449i32);
pub const STATUS_ERROR_PROCESS_NOT_IN_JOB: NTSTATUS = NTSTATUS(-1073741394i32);
pub const STATUS_EVALUATION_EXPIRATION: NTSTATUS = NTSTATUS(-1073741208i32);
pub const STATUS_EVENTLOG_CANT_START: NTSTATUS = NTSTATUS(-1073741425i32);
pub const STATUS_EVENTLOG_FILE_CHANGED: NTSTATUS = NTSTATUS(-1073741417i32);
pub const STATUS_EVENTLOG_FILE_CORRUPT: NTSTATUS = NTSTATUS(-1073741426i32);
pub const STATUS_EVENT_DONE: NTSTATUS = NTSTATUS(1073741842i32);
pub const STATUS_EVENT_PENDING: NTSTATUS = NTSTATUS(1073741843i32);
pub const STATUS_EXECUTABLE_MEMORY_WRITE: NTSTATUS = NTSTATUS(-1073739997i32);
pub const STATUS_EXPIRED_HANDLE: NTSTATUS = NTSTATUS(-1072103328i32);
pub const STATUS_EXTERNAL_BACKING_PROVIDER_UNKNOWN: NTSTATUS = NTSTATUS(-1073740690i32);
pub const STATUS_EXTERNAL_SYSKEY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073740639i32);
pub const STATUS_EXTRANEOUS_INFORMATION: NTSTATUS = NTSTATUS(-2147483625i32);
pub const STATUS_FAILED_DRIVER_ENTRY: NTSTATUS = NTSTATUS(-1073740955i32);
pub const STATUS_FAILED_STACK_SWITCH: NTSTATUS = NTSTATUS(-1073740941i32);
pub const STATUS_FAIL_CHECK: NTSTATUS = NTSTATUS(-1073741271i32);
pub const STATUS_FAIL_FAST_EXCEPTION: NTSTATUS = NTSTATUS(-1073740286i32);
pub const STATUS_FASTPATH_REJECTED: NTSTATUS = NTSTATUS(-1073700844i32);
pub const STATUS_FATAL_APP_EXIT: NTSTATUS = NTSTATUS(1073741845i32);
pub const STATUS_FATAL_MEMORY_EXHAUSTION: NTSTATUS = NTSTATUS(-1073741395i32);
pub const STATUS_FATAL_USER_CALLBACK_EXCEPTION: NTSTATUS = NTSTATUS(-1073740771i32);
pub const STATUS_FILEMARK_DETECTED: NTSTATUS = NTSTATUS(-2147483621i32);
pub const STATUS_FILES_OPEN: NTSTATUS = NTSTATUS(-1073741561i32);
pub const STATUS_FILE_CHECKED_OUT: NTSTATUS = NTSTATUS(-1073739519i32);
pub const STATUS_FILE_CLOSED: NTSTATUS = NTSTATUS(-1073741528i32);
pub const STATUS_FILE_CORRUPT_ERROR: NTSTATUS = NTSTATUS(-1073741566i32);
pub const STATUS_FILE_DELETED: NTSTATUS = NTSTATUS(-1073741533i32);
pub const STATUS_FILE_ENCRYPTED: NTSTATUS = NTSTATUS(-1073741165i32);
pub const STATUS_FILE_FORCED_CLOSED: NTSTATUS = NTSTATUS(-1073741642i32);
pub const STATUS_FILE_HANDLE_REVOKED: NTSTATUS = NTSTATUS(-1073739504i32);
pub const STATUS_FILE_IDENTITY_NOT_PERSISTENT: NTSTATUS = NTSTATUS(-1072103370i32);
pub const STATUS_FILE_INVALID: NTSTATUS = NTSTATUS(-1073741672i32);
pub const STATUS_FILE_IS_A_DIRECTORY: NTSTATUS = NTSTATUS(-1073741638i32);
pub const STATUS_FILE_IS_OFFLINE: NTSTATUS = NTSTATUS(-1073741209i32);
pub const STATUS_FILE_LOCKED_WITH_ONLY_READERS: NTSTATUS = NTSTATUS(298i32);
pub const STATUS_FILE_LOCKED_WITH_WRITERS: NTSTATUS = NTSTATUS(299i32);
pub const STATUS_FILE_LOCK_CONFLICT: NTSTATUS = NTSTATUS(-1073741740i32);
pub const STATUS_FILE_METADATA_OPTIMIZATION_IN_PROGRESS: NTSTATUS = NTSTATUS(-1073741397i32);
pub const STATUS_FILE_NOT_AVAILABLE: NTSTATUS = NTSTATUS(-1073740697i32);
pub const STATUS_FILE_NOT_ENCRYPTED: NTSTATUS = NTSTATUS(-1073741167i32);
pub const STATUS_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073740620i32);
pub const STATUS_FILE_PROTECTED_UNDER_DPL: NTSTATUS = NTSTATUS(-1073740637i32);
pub const STATUS_FILE_RENAMED: NTSTATUS = NTSTATUS(-1073741611i32);
pub const STATUS_FILE_SNAP_INVALID_PARAMETER: NTSTATUS = NTSTATUS(-1073679099i32);
pub const STATUS_FILE_SNAP_IN_PROGRESS: NTSTATUS = NTSTATUS(-1073679104i32);
pub const STATUS_FILE_SNAP_IO_NOT_COORDINATED: NTSTATUS = NTSTATUS(-1073679101i32);
pub const STATUS_FILE_SNAP_MODIFY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073679102i32);
pub const STATUS_FILE_SNAP_UNEXPECTED_ERROR: NTSTATUS = NTSTATUS(-1073679100i32);
pub const STATUS_FILE_SNAP_USER_SECTION_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073679103i32);
pub const STATUS_FILE_SYSTEM_LIMITATION: NTSTATUS = NTSTATUS(-1073740761i32);
pub const STATUS_FILE_SYSTEM_VIRTUALIZATION_BUSY: NTSTATUS = NTSTATUS(-1073689085i32);
pub const STATUS_FILE_SYSTEM_VIRTUALIZATION_INVALID_OPERATION: NTSTATUS = NTSTATUS(-1073689083i32);
pub const STATUS_FILE_SYSTEM_VIRTUALIZATION_METADATA_CORRUPT: NTSTATUS = NTSTATUS(-1073689086i32);
pub const STATUS_FILE_SYSTEM_VIRTUALIZATION_PROVIDER_UNKNOWN: NTSTATUS = NTSTATUS(-1073689084i32);
pub const STATUS_FILE_SYSTEM_VIRTUALIZATION_UNAVAILABLE: NTSTATUS = NTSTATUS(-1073689087i32);
pub const STATUS_FILE_TOO_LARGE: NTSTATUS = NTSTATUS(-1073739516i32);
pub const STATUS_FIRMWARE_IMAGE_INVALID: NTSTATUS = NTSTATUS(-1073740667i32);
pub const STATUS_FIRMWARE_SLOT_INVALID: NTSTATUS = NTSTATUS(-1073740668i32);
pub const STATUS_FIRMWARE_UPDATED: NTSTATUS = NTSTATUS(1073741868i32);
pub const STATUS_FLOATED_SECTION: NTSTATUS = NTSTATUS(-1072103349i32);
pub const STATUS_FLOAT_DENORMAL_OPERAND: NTSTATUS = NTSTATUS(-1073741683i32);
pub const STATUS_FLOAT_DIVIDE_BY_ZERO: NTSTATUS = NTSTATUS(-1073741682i32);
pub const STATUS_FLOAT_INEXACT_RESULT: NTSTATUS = NTSTATUS(-1073741681i32);
pub const STATUS_FLOAT_INVALID_OPERATION: NTSTATUS = NTSTATUS(-1073741680i32);
pub const STATUS_FLOAT_MULTIPLE_FAULTS: NTSTATUS = NTSTATUS(-1073741132i32);
pub const STATUS_FLOAT_MULTIPLE_TRAPS: NTSTATUS = NTSTATUS(-1073741131i32);
pub const STATUS_FLOAT_OVERFLOW: NTSTATUS = NTSTATUS(-1073741679i32);
pub const STATUS_FLOAT_STACK_CHECK: NTSTATUS = NTSTATUS(-1073741678i32);
pub const STATUS_FLOAT_UNDERFLOW: NTSTATUS = NTSTATUS(-1073741677i32);
pub const STATUS_FLOPPY_BAD_REGISTERS: NTSTATUS = NTSTATUS(-1073741464i32);
pub const STATUS_FLOPPY_ID_MARK_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741467i32);
pub const STATUS_FLOPPY_UNKNOWN_ERROR: NTSTATUS = NTSTATUS(-1073741465i32);
pub const STATUS_FLOPPY_VOLUME: NTSTATUS = NTSTATUS(-1073741468i32);
pub const STATUS_FLOPPY_WRONG_CYLINDER: NTSTATUS = NTSTATUS(-1073741466i32);
pub const STATUS_FLT_ALREADY_ENLISTED: NTSTATUS = NTSTATUS(-1071906789i32);
pub const STATUS_FLT_BUFFER_TOO_SMALL: NTSTATUS = NTSTATUS(-2145648639i32);
pub const STATUS_FLT_CBDQ_DISABLED: NTSTATUS = NTSTATUS(-1071906802i32);
pub const STATUS_FLT_CONTEXT_ALLOCATION_NOT_FOUND: NTSTATUS = NTSTATUS(-1071906794i32);
pub const STATUS_FLT_CONTEXT_ALREADY_DEFINED: NTSTATUS = NTSTATUS(-1071906814i32);
pub const STATUS_FLT_CONTEXT_ALREADY_LINKED: NTSTATUS = NTSTATUS(-1071906788i32);
pub const STATUS_FLT_DELETING_OBJECT: NTSTATUS = NTSTATUS(-1071906805i32);
pub const STATUS_FLT_DISALLOW_FAST_IO: NTSTATUS = NTSTATUS(-1071906812i32);
pub const STATUS_FLT_DISALLOW_FSFILTER_IO: i32 = -1071906812i32;
pub const STATUS_FLT_DO_NOT_ATTACH: NTSTATUS = NTSTATUS(-1071906801i32);
pub const STATUS_FLT_DO_NOT_DETACH: NTSTATUS = NTSTATUS(-1071906800i32);
pub const STATUS_FLT_DUPLICATE_ENTRY: NTSTATUS = NTSTATUS(-1071906803i32);
pub const STATUS_FLT_FILTER_NOT_FOUND: NTSTATUS = NTSTATUS(-1071906797i32);
pub const STATUS_FLT_FILTER_NOT_READY: NTSTATUS = NTSTATUS(-1071906808i32);
pub const STATUS_FLT_INSTANCE_ALTITUDE_COLLISION: NTSTATUS = NTSTATUS(-1071906799i32);
pub const STATUS_FLT_INSTANCE_NAME_COLLISION: NTSTATUS = NTSTATUS(-1071906798i32);
pub const STATUS_FLT_INSTANCE_NOT_FOUND: NTSTATUS = NTSTATUS(-1071906795i32);
pub const STATUS_FLT_INTERNAL_ERROR: NTSTATUS = NTSTATUS(-1071906806i32);
pub const STATUS_FLT_INVALID_ASYNCHRONOUS_REQUEST: NTSTATUS = NTSTATUS(-1071906813i32);
pub const STATUS_FLT_INVALID_CONTEXT_REGISTRATION: NTSTATUS = NTSTATUS(-1071906793i32);
pub const STATUS_FLT_INVALID_NAME_REQUEST: NTSTATUS = NTSTATUS(-1071906811i32);
pub const STATUS_FLT_IO_COMPLETE: NTSTATUS = NTSTATUS(1835009i32);
pub const STATUS_FLT_MUST_BE_NONPAGED_POOL: NTSTATUS = NTSTATUS(-1071906804i32);
pub const STATUS_FLT_NAME_CACHE_MISS: NTSTATUS = NTSTATUS(-1071906792i32);
pub const STATUS_FLT_NOT_INITIALIZED: NTSTATUS = NTSTATUS(-1071906809i32);
pub const STATUS_FLT_NOT_SAFE_TO_POST_OPERATION: NTSTATUS = NTSTATUS(-1071906810i32);
pub const STATUS_FLT_NO_DEVICE_OBJECT: NTSTATUS = NTSTATUS(-1071906791i32);
pub const STATUS_FLT_NO_HANDLER_DEFINED: NTSTATUS = NTSTATUS(-1071906815i32);
pub const STATUS_FLT_NO_WAITER_FOR_REPLY: NTSTATUS = NTSTATUS(-1071906784i32);
pub const STATUS_FLT_POST_OPERATION_CLEANUP: NTSTATUS = NTSTATUS(-1071906807i32);
pub const STATUS_FLT_REGISTRATION_BUSY: NTSTATUS = NTSTATUS(-1071906781i32);
pub const STATUS_FLT_VOLUME_ALREADY_MOUNTED: NTSTATUS = NTSTATUS(-1071906790i32);
pub const STATUS_FLT_VOLUME_NOT_FOUND: NTSTATUS = NTSTATUS(-1071906796i32);
pub const STATUS_FLT_WCOS_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071906780i32);
pub const STATUS_FORMS_AUTH_REQUIRED: NTSTATUS = NTSTATUS(-1073739515i32);
pub const STATUS_FOUND_OUT_OF_SCOPE: NTSTATUS = NTSTATUS(-1073741266i32);
pub const STATUS_FREE_SPACE_TOO_FRAGMENTED: NTSTATUS = NTSTATUS(-1073740645i32);
pub const STATUS_FREE_VM_NOT_AT_BASE: NTSTATUS = NTSTATUS(-1073741665i32);
pub const STATUS_FSFILTER_OP_COMPLETED_SUCCESSFULLY: NTSTATUS = NTSTATUS(294i32);
pub const STATUS_FS_DRIVER_REQUIRED: NTSTATUS = NTSTATUS(-1073741412i32);
pub const STATUS_FT_DI_SCAN_REQUIRED: NTSTATUS = NTSTATUS(-1073740692i32);
pub const STATUS_FT_MISSING_MEMBER: NTSTATUS = NTSTATUS(-1073741473i32);
pub const STATUS_FT_ORPHANING: NTSTATUS = NTSTATUS(-1073741459i32);
pub const STATUS_FT_READ_FAILURE: NTSTATUS = NTSTATUS(-1073740629i32);
pub const STATUS_FT_READ_FROM_COPY: NTSTATUS = NTSTATUS(1073741877i32);
pub const STATUS_FT_READ_FROM_COPY_FAILURE: NTSTATUS = NTSTATUS(-1073740609i32);
pub const STATUS_FT_READ_RECOVERY_FROM_BACKUP: NTSTATUS = NTSTATUS(1073741834i32);
pub const STATUS_FT_WRITE_FAILURE: NTSTATUS = NTSTATUS(-1073740693i32);
pub const STATUS_FT_WRITE_RECOVERY: NTSTATUS = NTSTATUS(1073741835i32);
pub const STATUS_FULLSCREEN_MODE: NTSTATUS = NTSTATUS(-1073741479i32);
pub const STATUS_FVE_ACTION_NOT_ALLOWED: NTSTATUS = NTSTATUS(-1071579127i32);
pub const STATUS_FVE_AUTH_INVALID_APPLICATION: NTSTATUS = NTSTATUS(-1071579109i32);
pub const STATUS_FVE_AUTH_INVALID_CONFIG: NTSTATUS = NTSTATUS(-1071579108i32);
pub const STATUS_FVE_BAD_DATA: NTSTATUS = NTSTATUS(-1071579126i32);
pub const STATUS_FVE_BAD_INFORMATION: NTSTATUS = NTSTATUS(-1071579134i32);
pub const STATUS_FVE_BAD_METADATA_POINTER: NTSTATUS = NTSTATUS(-1071579105i32);
pub const STATUS_FVE_BAD_PARTITION_SIZE: NTSTATUS = NTSTATUS(-1071579131i32);
pub const STATUS_FVE_CONV_READ_ERROR: NTSTATUS = NTSTATUS(-1071579123i32);
pub const STATUS_FVE_CONV_RECOVERY_FAILED: NTSTATUS = NTSTATUS(-1071579096i32);
pub const STATUS_FVE_CONV_WRITE_ERROR: NTSTATUS = NTSTATUS(-1071579122i32);
pub const STATUS_FVE_DEBUGGER_ENABLED: NTSTATUS = NTSTATUS(-1071579107i32);
pub const STATUS_FVE_DEVICE_LOCKEDOUT: NTSTATUS = NTSTATUS(-1071579077i32);
pub const STATUS_FVE_DRY_RUN_FAILED: NTSTATUS = NTSTATUS(-1071579106i32);
pub const STATUS_FVE_EDRIVE_BAND_ENUMERATION_FAILED: NTSTATUS = NTSTATUS(-1071579071i32);
pub const STATUS_FVE_EDRIVE_DRY_RUN_FAILED: NTSTATUS = NTSTATUS(-1071579080i32);
pub const STATUS_FVE_ENH_PIN_INVALID: NTSTATUS = NTSTATUS(-1071579087i32);
pub const STATUS_FVE_FAILED_AUTHENTICATION: NTSTATUS = NTSTATUS(-1071579119i32);
pub const STATUS_FVE_FAILED_SECTOR_SIZE: NTSTATUS = NTSTATUS(-1071579120i32);
pub const STATUS_FVE_FAILED_WRONG_FS: NTSTATUS = NTSTATUS(-1071579132i32);
pub const STATUS_FVE_FS_MOUNTED: NTSTATUS = NTSTATUS(-1071579129i32);
pub const STATUS_FVE_FS_NOT_EXTENDED: NTSTATUS = NTSTATUS(-1071579130i32);
pub const STATUS_FVE_FULL_ENCRYPTION_NOT_ALLOWED_ON_TP_STORAGE: NTSTATUS = NTSTATUS(-1071579086i32);
pub const STATUS_FVE_INVALID_DATUM_TYPE: NTSTATUS = NTSTATUS(-1071579094i32);
pub const STATUS_FVE_KEYFILE_INVALID: NTSTATUS = NTSTATUS(-1071579116i32);
pub const STATUS_FVE_KEYFILE_NOT_FOUND: NTSTATUS = NTSTATUS(-1071579117i32);
pub const STATUS_FVE_KEYFILE_NO_VMK: NTSTATUS = NTSTATUS(-1071579115i32);
pub const STATUS_FVE_LOCKED_VOLUME: NTSTATUS = NTSTATUS(-1071579136i32);
pub const STATUS_FVE_MOR_FAILED: NTSTATUS = NTSTATUS(-1071579099i32);
pub const STATUS_FVE_NOT_ALLOWED_ON_CLUSTER: NTSTATUS = NTSTATUS(-1071579083i32);
pub const STATUS_FVE_NOT_ALLOWED_ON_CSV_STACK: NTSTATUS = NTSTATUS(-1071579084i32);
pub const STATUS_FVE_NOT_ALLOWED_TO_UPGRADE_WHILE_CONVERTING: NTSTATUS = NTSTATUS(-1071579082i32);
pub const STATUS_FVE_NOT_DATA_VOLUME: NTSTATUS = NTSTATUS(-1071579124i32);
pub const STATUS_FVE_NOT_DE_VOLUME: NTSTATUS = NTSTATUS(-1071579075i32);
pub const STATUS_FVE_NOT_ENCRYPTED: NTSTATUS = NTSTATUS(-1071579135i32);
pub const STATUS_FVE_NOT_OS_VOLUME: NTSTATUS = NTSTATUS(-1071579118i32);
pub const STATUS_FVE_NO_AUTOUNLOCK_MASTER_KEY: NTSTATUS = NTSTATUS(-1071579100i32);
pub const STATUS_FVE_NO_FEATURE_LICENSE: NTSTATUS = NTSTATUS(-1071579098i32);
pub const STATUS_FVE_NO_LICENSE: NTSTATUS = NTSTATUS(-1071579128i32);
pub const STATUS_FVE_OLD_METADATA_COPY: NTSTATUS = NTSTATUS(-1071579104i32);
pub const STATUS_FVE_OSV_KSR_NOT_ALLOWED: NTSTATUS = NTSTATUS(-1071579072i32);
pub const STATUS_FVE_OVERLAPPED_UPDATE: NTSTATUS = NTSTATUS(-1071579121i32);
pub const STATUS_FVE_PARTIAL_METADATA: NTSTATUS = NTSTATUS(-2145320959i32);
pub const STATUS_FVE_PIN_INVALID: NTSTATUS = NTSTATUS(-1071579110i32);
pub const STATUS_FVE_POLICY_USER_DISABLE_RDV_NOT_ALLOWED: NTSTATUS = NTSTATUS(-1071579097i32);
pub const STATUS_FVE_PROTECTION_CANNOT_BE_DISABLED: NTSTATUS = NTSTATUS(-1071579073i32);
pub const STATUS_FVE_PROTECTION_DISABLED: NTSTATUS = NTSTATUS(-1071579074i32);
pub const STATUS_FVE_RAW_ACCESS: NTSTATUS = NTSTATUS(-1071579102i32);
pub const STATUS_FVE_RAW_BLOCKED: NTSTATUS = NTSTATUS(-1071579101i32);
pub const STATUS_FVE_REBOOT_REQUIRED: NTSTATUS = NTSTATUS(-1071579103i32);
pub const STATUS_FVE_SECUREBOOT_CONFIG_CHANGE: NTSTATUS = NTSTATUS(-1071579078i32);
pub const STATUS_FVE_SECUREBOOT_DISABLED: NTSTATUS = NTSTATUS(-1071579079i32);
pub const STATUS_FVE_TOO_SMALL: NTSTATUS = NTSTATUS(-1071579133i32);
pub const STATUS_FVE_TPM_DISABLED: NTSTATUS = NTSTATUS(-1071579114i32);
pub const STATUS_FVE_TPM_INVALID_PCR: NTSTATUS = NTSTATUS(-1071579112i32);
pub const STATUS_FVE_TPM_NO_VMK: NTSTATUS = NTSTATUS(-1071579111i32);
pub const STATUS_FVE_TPM_SRK_AUTH_NOT_ZERO: NTSTATUS = NTSTATUS(-1071579113i32);
pub const STATUS_FVE_TRANSIENT_STATE: NTSTATUS = NTSTATUS(-2145320958i32);
pub const STATUS_FVE_VIRTUALIZED_SPACE_TOO_BIG: NTSTATUS = NTSTATUS(-1071579095i32);
pub const STATUS_FVE_VOLUME_EXTEND_PREVENTS_EOW_DECRYPT: NTSTATUS = NTSTATUS(-1071579076i32);
pub const STATUS_FVE_VOLUME_NOT_BOUND: NTSTATUS = NTSTATUS(-1071579125i32);
pub const STATUS_FVE_VOLUME_TOO_SMALL: NTSTATUS = NTSTATUS(-1071579088i32);
pub const STATUS_FVE_WIPE_CANCEL_NOT_APPLICABLE: NTSTATUS = NTSTATUS(-1071579081i32);
pub const STATUS_FVE_WIPE_NOT_ALLOWED_ON_TP_STORAGE: NTSTATUS = NTSTATUS(-1071579085i32);
pub const STATUS_FWP_ACTION_INCOMPATIBLE_WITH_LAYER: NTSTATUS = NTSTATUS(-1071513556i32);
pub const STATUS_FWP_ACTION_INCOMPATIBLE_WITH_SUBLAYER: NTSTATUS = NTSTATUS(-1071513555i32);
pub const STATUS_FWP_ALREADY_EXISTS: NTSTATUS = NTSTATUS(-1071513591i32);
pub const STATUS_FWP_BUILTIN_OBJECT: NTSTATUS = NTSTATUS(-1071513577i32);
pub const STATUS_FWP_CALLOUT_NOTIFICATION_FAILED: NTSTATUS = NTSTATUS(-1071513545i32);
pub const STATUS_FWP_CALLOUT_NOT_FOUND: NTSTATUS = NTSTATUS(-1071513599i32);
pub const STATUS_FWP_CANNOT_PEND: NTSTATUS = NTSTATUS(-1071513341i32);
pub const STATUS_FWP_CONDITION_NOT_FOUND: NTSTATUS = NTSTATUS(-1071513598i32);
pub const STATUS_FWP_CONNECTIONS_DISABLED: NTSTATUS = NTSTATUS(-1071513535i32);
pub const STATUS_FWP_CONTEXT_INCOMPATIBLE_WITH_CALLOUT: NTSTATUS = NTSTATUS(-1071513553i32);
pub const STATUS_FWP_CONTEXT_INCOMPATIBLE_WITH_LAYER: NTSTATUS = NTSTATUS(-1071513554i32);
pub const STATUS_FWP_DROP_NOICMP: NTSTATUS = NTSTATUS(-1071513340i32);
pub const STATUS_FWP_DUPLICATE_AUTH_METHOD: NTSTATUS = NTSTATUS(-1071513540i32);
pub const STATUS_FWP_DUPLICATE_CONDITION: NTSTATUS = NTSTATUS(-1071513558i32);
pub const STATUS_FWP_DUPLICATE_KEYMOD: NTSTATUS = NTSTATUS(-1071513557i32);
pub const STATUS_FWP_DYNAMIC_SESSION_IN_PROGRESS: NTSTATUS = NTSTATUS(-1071513589i32);
pub const STATUS_FWP_EM_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071513550i32);
pub const STATUS_FWP_FILTER_NOT_FOUND: NTSTATUS = NTSTATUS(-1071513597i32);
pub const STATUS_FWP_IKEEXT_NOT_RUNNING: NTSTATUS = NTSTATUS(-1071513532i32);
pub const STATUS_FWP_INCOMPATIBLE_AUTH_METHOD: NTSTATUS = NTSTATUS(-1071513552i32);
pub const STATUS_FWP_INCOMPATIBLE_CIPHER_TRANSFORM: NTSTATUS = NTSTATUS(-1071513542i32);
pub const STATUS_FWP_INCOMPATIBLE_DH_GROUP: NTSTATUS = NTSTATUS(-1071513551i32);
pub const STATUS_FWP_INCOMPATIBLE_LAYER: NTSTATUS = NTSTATUS(-1071513580i32);
pub const STATUS_FWP_INCOMPATIBLE_SA_STATE: NTSTATUS = NTSTATUS(-1071513573i32);
pub const STATUS_FWP_INCOMPATIBLE_TXN: NTSTATUS = NTSTATUS(-1071513583i32);
pub const STATUS_FWP_INJECT_HANDLE_CLOSING: NTSTATUS = NTSTATUS(-1071513343i32);
pub const STATUS_FWP_INJECT_HANDLE_STALE: NTSTATUS = NTSTATUS(-1071513342i32);
pub const STATUS_FWP_INVALID_ACTION_TYPE: NTSTATUS = NTSTATUS(-1071513564i32);
pub const STATUS_FWP_INVALID_AUTH_TRANSFORM: NTSTATUS = NTSTATUS(-1071513544i32);
pub const STATUS_FWP_INVALID_CIPHER_TRANSFORM: NTSTATUS = NTSTATUS(-1071513543i32);
pub const STATUS_FWP_INVALID_DNS_NAME: NTSTATUS = NTSTATUS(-1071513534i32);
pub const STATUS_FWP_INVALID_ENUMERATOR: NTSTATUS = NTSTATUS(-1071513571i32);
pub const STATUS_FWP_INVALID_FLAGS: NTSTATUS = NTSTATUS(-1071513570i32);
pub const STATUS_FWP_INVALID_INTERVAL: NTSTATUS = NTSTATUS(-1071513567i32);
pub const STATUS_FWP_INVALID_NET_MASK: NTSTATUS = NTSTATUS(-1071513569i32);
pub const STATUS_FWP_INVALID_PARAMETER: NTSTATUS = NTSTATUS(-1071513547i32);
pub const STATUS_FWP_INVALID_RANGE: NTSTATUS = NTSTATUS(-1071513568i32);
pub const STATUS_FWP_INVALID_TRANSFORM_COMBINATION: NTSTATUS = NTSTATUS(-1071513541i32);
pub const STATUS_FWP_INVALID_TUNNEL_ENDPOINT: NTSTATUS = NTSTATUS(-1071513539i32);
pub const STATUS_FWP_INVALID_WEIGHT: NTSTATUS = NTSTATUS(-1071513563i32);
pub const STATUS_FWP_IN_USE: NTSTATUS = NTSTATUS(-1071513590i32);
pub const STATUS_FWP_KEY_DICTATION_INVALID_KEYING_MATERIAL: NTSTATUS = NTSTATUS(-1071513536i32);
pub const STATUS_FWP_KEY_DICTATOR_ALREADY_REGISTERED: NTSTATUS = NTSTATUS(-1071513537i32);
pub const STATUS_FWP_KM_CLIENTS_ONLY: NTSTATUS = NTSTATUS(-1071513579i32);
pub const STATUS_FWP_L2_DRIVER_NOT_READY: NTSTATUS = NTSTATUS(-1071513538i32);
pub const STATUS_FWP_LAYER_NOT_FOUND: NTSTATUS = NTSTATUS(-1071513596i32);
pub const STATUS_FWP_LIFETIME_MISMATCH: NTSTATUS = NTSTATUS(-1071513578i32);
pub const STATUS_FWP_MATCH_TYPE_MISMATCH: NTSTATUS = NTSTATUS(-1071513562i32);
pub const STATUS_FWP_NET_EVENTS_DISABLED: NTSTATUS = NTSTATUS(-1071513581i32);
pub const STATUS_FWP_NEVER_MATCH: NTSTATUS = NTSTATUS(-1071513549i32);
pub const STATUS_FWP_NOTIFICATION_DROPPED: NTSTATUS = NTSTATUS(-1071513575i32);
pub const STATUS_FWP_NOT_FOUND: NTSTATUS = NTSTATUS(-1071513592i32);
pub const STATUS_FWP_NO_TXN_IN_PROGRESS: NTSTATUS = NTSTATUS(-1071513587i32);
pub const STATUS_FWP_NULL_DISPLAY_NAME: NTSTATUS = NTSTATUS(-1071513565i32);
pub const STATUS_FWP_NULL_POINTER: NTSTATUS = NTSTATUS(-1071513572i32);
pub const STATUS_FWP_OUT_OF_BOUNDS: NTSTATUS = NTSTATUS(-1071513560i32);
pub const STATUS_FWP_PROVIDER_CONTEXT_MISMATCH: NTSTATUS = NTSTATUS(-1071513548i32);
pub const STATUS_FWP_PROVIDER_CONTEXT_NOT_FOUND: NTSTATUS = NTSTATUS(-1071513594i32);
pub const STATUS_FWP_PROVIDER_NOT_FOUND: NTSTATUS = NTSTATUS(-1071513595i32);
pub const STATUS_FWP_RESERVED: NTSTATUS = NTSTATUS(-1071513559i32);
pub const STATUS_FWP_SESSION_ABORTED: NTSTATUS = NTSTATUS(-1071513584i32);
pub const STATUS_FWP_STILL_ON: NTSTATUS = NTSTATUS(-1071513533i32);
pub const STATUS_FWP_SUBLAYER_NOT_FOUND: NTSTATUS = NTSTATUS(-1071513593i32);
pub const STATUS_FWP_TCPIP_NOT_READY: NTSTATUS = NTSTATUS(-1071513344i32);
pub const STATUS_FWP_TIMEOUT: NTSTATUS = NTSTATUS(-1071513582i32);
pub const STATUS_FWP_TOO_MANY_CALLOUTS: NTSTATUS = NTSTATUS(-1071513576i32);
pub const STATUS_FWP_TOO_MANY_SUBLAYERS: NTSTATUS = NTSTATUS(-1071513546i32);
pub const STATUS_FWP_TRAFFIC_MISMATCH: NTSTATUS = NTSTATUS(-1071513574i32);
pub const STATUS_FWP_TXN_ABORTED: NTSTATUS = NTSTATUS(-1071513585i32);
pub const STATUS_FWP_TXN_IN_PROGRESS: NTSTATUS = NTSTATUS(-1071513586i32);
pub const STATUS_FWP_TYPE_MISMATCH: NTSTATUS = NTSTATUS(-1071513561i32);
pub const STATUS_FWP_WRONG_SESSION: NTSTATUS = NTSTATUS(-1071513588i32);
pub const STATUS_FWP_ZERO_LENGTH_ARRAY: NTSTATUS = NTSTATUS(-1071513566i32);
pub const STATUS_GDI_HANDLE_LEAK: NTSTATUS = NTSTATUS(-2143354879i32);
pub const STATUS_GENERIC_COMMAND_FAILED: NTSTATUS = NTSTATUS(-1072365530i32);
pub const STATUS_GENERIC_NOT_MAPPED: NTSTATUS = NTSTATUS(-1073741594i32);
pub const STATUS_GHOSTED: NTSTATUS = NTSTATUS(303i32);
pub const STATUS_GPIO_CLIENT_INFORMATION_INVALID: NTSTATUS = NTSTATUS(-1073700574i32);
pub const STATUS_GPIO_INCOMPATIBLE_CONNECT_MODE: NTSTATUS = NTSTATUS(-1073700570i32);
pub const STATUS_GPIO_INTERRUPT_ALREADY_UNMASKED: NTSTATUS = NTSTATUS(-2147442393i32);
pub const STATUS_GPIO_INVALID_REGISTRATION_PACKET: NTSTATUS = NTSTATUS(-1073700572i32);
pub const STATUS_GPIO_OPERATION_DENIED: NTSTATUS = NTSTATUS(-1073700571i32);
pub const STATUS_GPIO_VERSION_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073700573i32);
pub const STATUS_GRACEFUL_DISCONNECT: NTSTATUS = NTSTATUS(-1073741257i32);
pub const STATUS_GRAPHICS_ADAPTER_ACCESS_NOT_EXCLUDED: NTSTATUS = NTSTATUS(-1071774661i32);
pub const STATUS_GRAPHICS_ADAPTER_CHAIN_NOT_READY: NTSTATUS = NTSTATUS(-1071774669i32);
pub const STATUS_GRAPHICS_ADAPTER_MUST_HAVE_AT_LEAST_ONE_SOURCE: NTSTATUS =
    NTSTATUS(-1071774936i32);
pub const STATUS_GRAPHICS_ADAPTER_MUST_HAVE_AT_LEAST_ONE_TARGET: NTSTATUS =
    NTSTATUS(-1071774935i32);
pub const STATUS_GRAPHICS_ADAPTER_WAS_RESET: NTSTATUS = NTSTATUS(-1071775741i32);
pub const STATUS_GRAPHICS_ALLOCATION_BUSY: NTSTATUS = NTSTATUS(-1071775486i32);
pub const STATUS_GRAPHICS_ALLOCATION_CLOSED: NTSTATUS = NTSTATUS(-1071775470i32);
pub const STATUS_GRAPHICS_ALLOCATION_CONTENT_LOST: NTSTATUS = NTSTATUS(-1071775466i32);
pub const STATUS_GRAPHICS_ALLOCATION_INVALID: NTSTATUS = NTSTATUS(-1071775482i32);
pub const STATUS_GRAPHICS_CANCEL_VIDPN_TOPOLOGY_AUGMENTATION: NTSTATUS = NTSTATUS(-1071774886i32);
pub const STATUS_GRAPHICS_CANNOTCOLORCONVERT: NTSTATUS = NTSTATUS(-1071775736i32);
pub const STATUS_GRAPHICS_CANT_ACCESS_ACTIVE_VIDPN: NTSTATUS = NTSTATUS(-1071774909i32);
pub const STATUS_GRAPHICS_CANT_EVICT_PINNED_ALLOCATION: NTSTATUS = NTSTATUS(-1071775479i32);
pub const STATUS_GRAPHICS_CANT_LOCK_MEMORY: NTSTATUS = NTSTATUS(-1071775487i32);
pub const STATUS_GRAPHICS_CANT_RENDER_LOCKED_ALLOCATION: NTSTATUS = NTSTATUS(-1071775471i32);
pub const STATUS_GRAPHICS_CHAINLINKS_NOT_ENUMERATED: NTSTATUS = NTSTATUS(-1071774670i32);
pub const STATUS_GRAPHICS_CHAINLINKS_NOT_POWERED_ON: NTSTATUS = NTSTATUS(-1071774667i32);
pub const STATUS_GRAPHICS_CHAINLINKS_NOT_STARTED: NTSTATUS = NTSTATUS(-1071774668i32);
pub const STATUS_GRAPHICS_CHILD_DESCRIPTOR_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071774719i32);
pub const STATUS_GRAPHICS_CLIENTVIDPN_NOT_SET: NTSTATUS = NTSTATUS(-1071774884i32);
pub const STATUS_GRAPHICS_COPP_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071774463i32);
pub const STATUS_GRAPHICS_DATASET_IS_EMPTY: NTSTATUS = NTSTATUS(1075708747i32);
pub const STATUS_GRAPHICS_DDCCI_INVALID_CAPABILITIES_STRING: NTSTATUS = NTSTATUS(-1071774329i32);
pub const STATUS_GRAPHICS_DDCCI_INVALID_DATA: NTSTATUS = NTSTATUS(-1071774331i32);
pub const STATUS_GRAPHICS_DDCCI_INVALID_MESSAGE_CHECKSUM: NTSTATUS = NTSTATUS(-1071774325i32);
pub const STATUS_GRAPHICS_DDCCI_INVALID_MESSAGE_COMMAND: NTSTATUS = NTSTATUS(-1071774327i32);
pub const STATUS_GRAPHICS_DDCCI_INVALID_MESSAGE_LENGTH: NTSTATUS = NTSTATUS(-1071774326i32);
pub const STATUS_GRAPHICS_DDCCI_MONITOR_RETURNED_INVALID_TIMING_STATUS_BYTE: NTSTATUS =
    NTSTATUS(-1071774330i32);
pub const STATUS_GRAPHICS_DDCCI_VCP_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071774332i32);
pub const STATUS_GRAPHICS_DEPENDABLE_CHILD_STATUS: NTSTATUS = NTSTATUS(1075708988i32);
pub const STATUS_GRAPHICS_DISPLAY_DEVICE_NOT_ATTACHED_TO_DESKTOP: NTSTATUS =
    NTSTATUS(-1071774238i32);
pub const STATUS_GRAPHICS_DRIVER_MISMATCH: NTSTATUS = NTSTATUS(-1071775735i32);
pub const STATUS_GRAPHICS_EMPTY_ADAPTER_MONITOR_MODE_SUPPORT_INTERSECTION: NTSTATUS =
    NTSTATUS(-1071774939i32);
pub const STATUS_GRAPHICS_FREQUENCYRANGE_ALREADY_IN_SET: NTSTATUS = NTSTATUS(-1071774945i32);
pub const STATUS_GRAPHICS_FREQUENCYRANGE_NOT_IN_SET: NTSTATUS = NTSTATUS(-1071774947i32);
pub const STATUS_GRAPHICS_GAMMA_RAMP_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071774904i32);
pub const STATUS_GRAPHICS_GPU_EXCEPTION_ON_DEVICE: NTSTATUS = NTSTATUS(-1071775232i32);
pub const STATUS_GRAPHICS_I2C_DEVICE_DOES_NOT_EXIST: NTSTATUS = NTSTATUS(-1071774335i32);
pub const STATUS_GRAPHICS_I2C_ERROR_RECEIVING_DATA: NTSTATUS = NTSTATUS(-1071774333i32);
pub const STATUS_GRAPHICS_I2C_ERROR_TRANSMITTING_DATA: NTSTATUS = NTSTATUS(-1071774334i32);
pub const STATUS_GRAPHICS_I2C_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071774336i32);
pub const STATUS_GRAPHICS_INCOMPATIBLE_PRIVATE_FORMAT: NTSTATUS = NTSTATUS(-1071774891i32);
pub const STATUS_GRAPHICS_INCONSISTENT_DEVICE_LINK_STATE: NTSTATUS = NTSTATUS(-1071774666i32);
pub const STATUS_GRAPHICS_INDIRECT_DISPLAY_ABANDON_SWAPCHAIN: NTSTATUS = NTSTATUS(-1071775726i32);
pub const STATUS_GRAPHICS_INDIRECT_DISPLAY_DEVICE_STOPPED: NTSTATUS = NTSTATUS(-1071775725i32);
pub const STATUS_GRAPHICS_INSUFFICIENT_DMA_BUFFER: NTSTATUS = NTSTATUS(-1071775743i32);
pub const STATUS_GRAPHICS_INTERNAL_ERROR: NTSTATUS = NTSTATUS(-1071774233i32);
pub const STATUS_GRAPHICS_INVALID_ACTIVE_REGION: NTSTATUS = NTSTATUS(-1071774965i32);
pub const STATUS_GRAPHICS_INVALID_ALLOCATION_HANDLE: NTSTATUS = NTSTATUS(-1071775468i32);
pub const STATUS_GRAPHICS_INVALID_ALLOCATION_INSTANCE: NTSTATUS = NTSTATUS(-1071775469i32);
pub const STATUS_GRAPHICS_INVALID_ALLOCATION_USAGE: NTSTATUS = NTSTATUS(-1071775472i32);
pub const STATUS_GRAPHICS_INVALID_CLIENT_TYPE: NTSTATUS = NTSTATUS(-1071774885i32);
pub const STATUS_GRAPHICS_INVALID_COLORBASIS: NTSTATUS = NTSTATUS(-1071774914i32);
pub const STATUS_GRAPHICS_INVALID_COPYPROTECTION_TYPE: NTSTATUS = NTSTATUS(-1071774897i32);
pub const STATUS_GRAPHICS_INVALID_DISPLAY_ADAPTER: NTSTATUS = NTSTATUS(-1071775742i32);
pub const STATUS_GRAPHICS_INVALID_DRIVER_MODEL: NTSTATUS = NTSTATUS(-1071775740i32);
pub const STATUS_GRAPHICS_INVALID_FREQUENCY: NTSTATUS = NTSTATUS(-1071774966i32);
pub const STATUS_GRAPHICS_INVALID_GAMMA_RAMP: NTSTATUS = NTSTATUS(-1071774905i32);
pub const STATUS_GRAPHICS_INVALID_MODE_PRUNING_ALGORITHM: NTSTATUS = NTSTATUS(-1071774890i32);
pub const STATUS_GRAPHICS_INVALID_MONITORDESCRIPTOR: NTSTATUS = NTSTATUS(-1071774933i32);
pub const STATUS_GRAPHICS_INVALID_MONITORDESCRIPTORSET: NTSTATUS = NTSTATUS(-1071774934i32);
pub const STATUS_GRAPHICS_INVALID_MONITOR_CAPABILITY_ORIGIN: NTSTATUS = NTSTATUS(-1071774889i32);
pub const STATUS_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGE: NTSTATUS = NTSTATUS(-1071774948i32);
pub const STATUS_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGESET: NTSTATUS = NTSTATUS(-1071774949i32);
pub const STATUS_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGE_CONSTRAINT: NTSTATUS =
    NTSTATUS(-1071774888i32);
pub const STATUS_GRAPHICS_INVALID_MONITOR_SOURCEMODESET: NTSTATUS = NTSTATUS(-1071774943i32);
pub const STATUS_GRAPHICS_INVALID_MONITOR_SOURCE_MODE: NTSTATUS = NTSTATUS(-1071774942i32);
pub const STATUS_GRAPHICS_INVALID_PATH_CONTENT_GEOMETRY_TRANSFORMATION: NTSTATUS =
    NTSTATUS(-1071774907i32);
pub const STATUS_GRAPHICS_INVALID_PATH_CONTENT_TYPE: NTSTATUS = NTSTATUS(-1071774898i32);
pub const STATUS_GRAPHICS_INVALID_PATH_IMPORTANCE_ORDINAL: NTSTATUS = NTSTATUS(-1071774908i32);
pub const STATUS_GRAPHICS_INVALID_PHYSICAL_MONITOR_HANDLE: NTSTATUS = NTSTATUS(-1071774324i32);
pub const STATUS_GRAPHICS_INVALID_PIXELFORMAT: NTSTATUS = NTSTATUS(-1071774915i32);
pub const STATUS_GRAPHICS_INVALID_PIXELVALUEACCESSMODE: NTSTATUS = NTSTATUS(-1071774913i32);
pub const STATUS_GRAPHICS_INVALID_POINTER: NTSTATUS = NTSTATUS(-1071774236i32);
pub const STATUS_GRAPHICS_INVALID_PRIMARYSURFACE_SIZE: NTSTATUS = NTSTATUS(-1071774918i32);
pub const STATUS_GRAPHICS_INVALID_SCANLINE_ORDERING: NTSTATUS = NTSTATUS(-1071774894i32);
pub const STATUS_GRAPHICS_INVALID_STRIDE: NTSTATUS = NTSTATUS(-1071774916i32);
pub const STATUS_GRAPHICS_INVALID_TOTAL_REGION: NTSTATUS = NTSTATUS(-1071774964i32);
pub const STATUS_GRAPHICS_INVALID_VIDEOPRESENTSOURCESET: NTSTATUS = NTSTATUS(-1071774955i32);
pub const STATUS_GRAPHICS_INVALID_VIDEOPRESENTTARGETSET: NTSTATUS = NTSTATUS(-1071774954i32);
pub const STATUS_GRAPHICS_INVALID_VIDEO_PRESENT_SOURCE: NTSTATUS = NTSTATUS(-1071774972i32);
pub const STATUS_GRAPHICS_INVALID_VIDEO_PRESENT_SOURCE_MODE: NTSTATUS = NTSTATUS(-1071774960i32);
pub const STATUS_GRAPHICS_INVALID_VIDEO_PRESENT_TARGET: NTSTATUS = NTSTATUS(-1071774971i32);
pub const STATUS_GRAPHICS_INVALID_VIDEO_PRESENT_TARGET_MODE: NTSTATUS = NTSTATUS(-1071774959i32);
pub const STATUS_GRAPHICS_INVALID_VIDPN: NTSTATUS = NTSTATUS(-1071774973i32);
pub const STATUS_GRAPHICS_INVALID_VIDPN_PRESENT_PATH: NTSTATUS = NTSTATUS(-1071774951i32);
pub const STATUS_GRAPHICS_INVALID_VIDPN_SOURCEMODESET: NTSTATUS = NTSTATUS(-1071774968i32);
pub const STATUS_GRAPHICS_INVALID_VIDPN_TARGETMODESET: NTSTATUS = NTSTATUS(-1071774967i32);
pub const STATUS_GRAPHICS_INVALID_VIDPN_TARGET_SUBSET_TYPE: NTSTATUS = NTSTATUS(-1071774929i32);
pub const STATUS_GRAPHICS_INVALID_VIDPN_TOPOLOGY: NTSTATUS = NTSTATUS(-1071774976i32);
pub const STATUS_GRAPHICS_INVALID_VIDPN_TOPOLOGY_RECOMMENDATION_REASON: NTSTATUS =
    NTSTATUS(-1071774899i32);
pub const STATUS_GRAPHICS_INVALID_VISIBLEREGION_SIZE: NTSTATUS = NTSTATUS(-1071774917i32);
pub const STATUS_GRAPHICS_LEADLINK_NOT_ENUMERATED: NTSTATUS = NTSTATUS(-1071774671i32);
pub const STATUS_GRAPHICS_LEADLINK_START_DEFERRED: NTSTATUS = NTSTATUS(1075708983i32);
pub const STATUS_GRAPHICS_MAX_NUM_PATHS_REACHED: NTSTATUS = NTSTATUS(-1071774887i32);
pub const STATUS_GRAPHICS_MCA_INTERNAL_ERROR: NTSTATUS = NTSTATUS(-1071774328i32);
pub const STATUS_GRAPHICS_MIRRORING_DEVICES_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071774237i32);
pub const STATUS_GRAPHICS_MODE_ALREADY_IN_MODESET: NTSTATUS = NTSTATUS(-1071774956i32);
pub const STATUS_GRAPHICS_MODE_ID_MUST_BE_UNIQUE: NTSTATUS = NTSTATUS(-1071774940i32);
pub const STATUS_GRAPHICS_MODE_NOT_IN_MODESET: NTSTATUS = NTSTATUS(-1071774902i32);
pub const STATUS_GRAPHICS_MODE_NOT_PINNED: NTSTATUS = NTSTATUS(1075708679i32);
pub const STATUS_GRAPHICS_MONITORDESCRIPTOR_ALREADY_IN_SET: NTSTATUS = NTSTATUS(-1071774931i32);
pub const STATUS_GRAPHICS_MONITORDESCRIPTOR_ID_MUST_BE_UNIQUE: NTSTATUS = NTSTATUS(-1071774930i32);
pub const STATUS_GRAPHICS_MONITORDESCRIPTOR_NOT_IN_SET: NTSTATUS = NTSTATUS(-1071774932i32);
pub const STATUS_GRAPHICS_MONITOR_COULD_NOT_BE_ASSOCIATED_WITH_ADAPTER: NTSTATUS =
    NTSTATUS(-1071774924i32);
pub const STATUS_GRAPHICS_MONITOR_NOT_CONNECTED: NTSTATUS = NTSTATUS(-1071774920i32);
pub const STATUS_GRAPHICS_MONITOR_NO_LONGER_EXISTS: NTSTATUS = NTSTATUS(-1071774323i32);
pub const STATUS_GRAPHICS_MULTISAMPLING_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071774903i32);
pub const STATUS_GRAPHICS_NOT_A_LINKED_ADAPTER: NTSTATUS = NTSTATUS(-1071774672i32);
pub const STATUS_GRAPHICS_NOT_EXCLUSIVE_MODE_OWNER: NTSTATUS = NTSTATUS(-1071775744i32);
pub const STATUS_GRAPHICS_NOT_POST_DEVICE_DRIVER: NTSTATUS = NTSTATUS(-1071774664i32);
pub const STATUS_GRAPHICS_NO_ACTIVE_VIDPN: NTSTATUS = NTSTATUS(-1071774922i32);
pub const STATUS_GRAPHICS_NO_AVAILABLE_IMPORTANCE_ORDINALS: NTSTATUS = NTSTATUS(-1071774892i32);
pub const STATUS_GRAPHICS_NO_AVAILABLE_VIDPN_TARGET: NTSTATUS = NTSTATUS(-1071774925i32);
pub const STATUS_GRAPHICS_NO_DISPLAY_DEVICE_CORRESPONDS_TO_NAME: NTSTATUS =
    NTSTATUS(-1071774239i32);
pub const STATUS_GRAPHICS_NO_DISPLAY_MODE_MANAGEMENT_SUPPORT: NTSTATUS = NTSTATUS(-1071774911i32);
pub const STATUS_GRAPHICS_NO_MONITORS_CORRESPOND_TO_DISPLAY_DEVICE: NTSTATUS =
    NTSTATUS(-1071774235i32);
pub const STATUS_GRAPHICS_NO_MORE_ELEMENTS_IN_DATASET: NTSTATUS = NTSTATUS(1075708748i32);
pub const STATUS_GRAPHICS_NO_PREFERRED_MODE: NTSTATUS = NTSTATUS(1075708702i32);
pub const STATUS_GRAPHICS_NO_RECOMMENDED_FUNCTIONAL_VIDPN: NTSTATUS = NTSTATUS(-1071774941i32);
pub const STATUS_GRAPHICS_NO_RECOMMENDED_VIDPN_TOPOLOGY: NTSTATUS = NTSTATUS(-1071774950i32);
pub const STATUS_GRAPHICS_NO_VIDEO_MEMORY: NTSTATUS = NTSTATUS(-1071775488i32);
pub const STATUS_GRAPHICS_NO_VIDPNMGR: NTSTATUS = NTSTATUS(-1071774923i32);
pub const STATUS_GRAPHICS_ONLY_CONSOLE_SESSION_SUPPORTED: NTSTATUS = NTSTATUS(-1071774240i32);
pub const STATUS_GRAPHICS_OPM_ALL_HDCP_HARDWARE_ALREADY_IN_USE: NTSTATUS = NTSTATUS(-1071774440i32);
pub const STATUS_GRAPHICS_OPM_DRIVER_INTERNAL_ERROR: NTSTATUS = NTSTATUS(-1071774434i32);
pub const STATUS_GRAPHICS_OPM_HDCP_SRM_NEVER_SET: NTSTATUS = NTSTATUS(-1071774442i32);
pub const STATUS_GRAPHICS_OPM_INTERNAL_ERROR: NTSTATUS = NTSTATUS(-1071774453i32);
pub const STATUS_GRAPHICS_OPM_INVALID_CONFIGURATION_REQUEST: NTSTATUS = NTSTATUS(-1071774431i32);
pub const STATUS_GRAPHICS_OPM_INVALID_ENCRYPTED_PARAMETERS: NTSTATUS = NTSTATUS(-1071774461i32);
pub const STATUS_GRAPHICS_OPM_INVALID_HANDLE: NTSTATUS = NTSTATUS(-1071774452i32);
pub const STATUS_GRAPHICS_OPM_INVALID_INFORMATION_REQUEST: NTSTATUS = NTSTATUS(-1071774435i32);
pub const STATUS_GRAPHICS_OPM_INVALID_SRM: NTSTATUS = NTSTATUS(-1071774446i32);
pub const STATUS_GRAPHICS_OPM_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071774464i32);
pub const STATUS_GRAPHICS_OPM_NO_PROTECTED_OUTPUTS_EXIST: NTSTATUS = NTSTATUS(-1071774459i32);
pub const STATUS_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_ACP: NTSTATUS = NTSTATUS(-1071774444i32);
pub const STATUS_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_CGMSA: NTSTATUS = NTSTATUS(-1071774443i32);
pub const STATUS_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_HDCP: NTSTATUS = NTSTATUS(-1071774445i32);
pub const STATUS_GRAPHICS_OPM_PROTECTED_OUTPUT_DOES_NOT_HAVE_COPP_SEMANTICS: NTSTATUS =
    NTSTATUS(-1071774436i32);
pub const STATUS_GRAPHICS_OPM_PROTECTED_OUTPUT_DOES_NOT_HAVE_OPM_SEMANTICS: NTSTATUS =
    NTSTATUS(-1071774433i32);
pub const STATUS_GRAPHICS_OPM_PROTECTED_OUTPUT_NO_LONGER_EXISTS: NTSTATUS =
    NTSTATUS(-1071774438i32);
pub const STATUS_GRAPHICS_OPM_RESOLUTION_TOO_HIGH: NTSTATUS = NTSTATUS(-1071774441i32);
pub const STATUS_GRAPHICS_OPM_SIGNALING_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071774432i32);
pub const STATUS_GRAPHICS_OPM_SPANNING_MODE_ENABLED: NTSTATUS = NTSTATUS(-1071774449i32);
pub const STATUS_GRAPHICS_OPM_THEATER_MODE_ENABLED: NTSTATUS = NTSTATUS(-1071774448i32);
pub const STATUS_GRAPHICS_PARAMETER_ARRAY_TOO_SMALL: NTSTATUS = NTSTATUS(-1071774234i32);
pub const STATUS_GRAPHICS_PARTIAL_DATA_POPULATED: NTSTATUS = NTSTATUS(1075707914i32);
pub const STATUS_GRAPHICS_PATH_ALREADY_IN_TOPOLOGY: NTSTATUS = NTSTATUS(-1071774957i32);
pub const STATUS_GRAPHICS_PATH_CONTENT_GEOMETRY_TRANSFORMATION_NOT_PINNED: NTSTATUS =
    NTSTATUS(1075708753i32);
pub const STATUS_GRAPHICS_PATH_CONTENT_GEOMETRY_TRANSFORMATION_NOT_SUPPORTED: NTSTATUS =
    NTSTATUS(-1071774906i32);
pub const STATUS_GRAPHICS_PATH_NOT_IN_TOPOLOGY: NTSTATUS = NTSTATUS(-1071774937i32);
pub const STATUS_GRAPHICS_PINNED_MODE_MUST_REMAIN_IN_SET: NTSTATUS = NTSTATUS(-1071774958i32);
pub const STATUS_GRAPHICS_POLLING_TOO_FREQUENTLY: NTSTATUS = NTSTATUS(1075708985i32);
pub const STATUS_GRAPHICS_PRESENT_BUFFER_NOT_BOUND: NTSTATUS = NTSTATUS(-1071775728i32);
pub const STATUS_GRAPHICS_PRESENT_DENIED: NTSTATUS = NTSTATUS(-1071775737i32);
pub const STATUS_GRAPHICS_PRESENT_INVALID_WINDOW: NTSTATUS = NTSTATUS(-1071775729i32);
pub const STATUS_GRAPHICS_PRESENT_MODE_CHANGED: NTSTATUS = NTSTATUS(-1071775739i32);
pub const STATUS_GRAPHICS_PRESENT_OCCLUDED: NTSTATUS = NTSTATUS(-1071775738i32);
pub const STATUS_GRAPHICS_PRESENT_REDIRECTION_DISABLED: NTSTATUS = NTSTATUS(-1071775733i32);
pub const STATUS_GRAPHICS_PRESENT_UNOCCLUDED: NTSTATUS = NTSTATUS(-1071775732i32);
pub const STATUS_GRAPHICS_PVP_HFS_FAILED: NTSTATUS = NTSTATUS(-1071774447i32);
pub const STATUS_GRAPHICS_PVP_INVALID_CERTIFICATE_LENGTH: NTSTATUS = NTSTATUS(-1071774450i32);
pub const STATUS_GRAPHICS_RESOURCES_NOT_RELATED: NTSTATUS = NTSTATUS(-1071774928i32);
pub const STATUS_GRAPHICS_SESSION_TYPE_CHANGE_IN_PROGRESS: NTSTATUS = NTSTATUS(-1071774232i32);
pub const STATUS_GRAPHICS_SKIP_ALLOCATION_PREPARATION: NTSTATUS = NTSTATUS(1075708417i32);
pub const STATUS_GRAPHICS_SOURCE_ALREADY_IN_SET: NTSTATUS = NTSTATUS(-1071774953i32);
pub const STATUS_GRAPHICS_SOURCE_ID_MUST_BE_UNIQUE: NTSTATUS = NTSTATUS(-1071774927i32);
pub const STATUS_GRAPHICS_SOURCE_NOT_IN_TOPOLOGY: NTSTATUS = NTSTATUS(-1071774919i32);
pub const STATUS_GRAPHICS_SPECIFIED_CHILD_ALREADY_CONNECTED: NTSTATUS = NTSTATUS(-1071774720i32);
pub const STATUS_GRAPHICS_STALE_MODESET: NTSTATUS = NTSTATUS(-1071774944i32);
pub const STATUS_GRAPHICS_STALE_VIDPN_TOPOLOGY: NTSTATUS = NTSTATUS(-1071774921i32);
pub const STATUS_GRAPHICS_START_DEFERRED: NTSTATUS = NTSTATUS(1075708986i32);
pub const STATUS_GRAPHICS_TARGET_ALREADY_IN_SET: NTSTATUS = NTSTATUS(-1071774952i32);
pub const STATUS_GRAPHICS_TARGET_ID_MUST_BE_UNIQUE: NTSTATUS = NTSTATUS(-1071774926i32);
pub const STATUS_GRAPHICS_TARGET_NOT_IN_TOPOLOGY: NTSTATUS = NTSTATUS(-1071774912i32);
pub const STATUS_GRAPHICS_TOO_MANY_REFERENCES: NTSTATUS = NTSTATUS(-1071775485i32);
pub const STATUS_GRAPHICS_TOPOLOGY_CHANGES_NOT_ALLOWED: NTSTATUS = NTSTATUS(-1071774893i32);
pub const STATUS_GRAPHICS_TRY_AGAIN_LATER: NTSTATUS = NTSTATUS(-1071775484i32);
pub const STATUS_GRAPHICS_TRY_AGAIN_NOW: NTSTATUS = NTSTATUS(-1071775483i32);
pub const STATUS_GRAPHICS_UAB_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071774462i32);
pub const STATUS_GRAPHICS_UNASSIGNED_MODESET_ALREADY_EXISTS: NTSTATUS = NTSTATUS(-1071774896i32);
pub const STATUS_GRAPHICS_UNKNOWN_CHILD_STATUS: NTSTATUS = NTSTATUS(1075708975i32);
pub const STATUS_GRAPHICS_UNSWIZZLING_APERTURE_UNAVAILABLE: NTSTATUS = NTSTATUS(-1071775481i32);
pub const STATUS_GRAPHICS_UNSWIZZLING_APERTURE_UNSUPPORTED: NTSTATUS = NTSTATUS(-1071775480i32);
pub const STATUS_GRAPHICS_VAIL_STATE_CHANGED: NTSTATUS = NTSTATUS(-1071775727i32);
pub const STATUS_GRAPHICS_VIDEO_PRESENT_TARGETS_LESS_THAN_SOURCES: NTSTATUS =
    NTSTATUS(-1071774938i32);
pub const STATUS_GRAPHICS_VIDPN_MODALITY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071774970i32);
pub const STATUS_GRAPHICS_VIDPN_SOURCE_IN_USE: NTSTATUS = NTSTATUS(-1071774910i32);
pub const STATUS_GRAPHICS_VIDPN_TOPOLOGY_CURRENTLY_NOT_SUPPORTED: NTSTATUS =
    NTSTATUS(-1071774974i32);
pub const STATUS_GRAPHICS_VIDPN_TOPOLOGY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071774975i32);
pub const STATUS_GRAPHICS_WINDOWDC_NOT_AVAILABLE: NTSTATUS = NTSTATUS(-1071775731i32);
pub const STATUS_GRAPHICS_WINDOWLESS_PRESENT_DISABLED: NTSTATUS = NTSTATUS(-1071775730i32);
pub const STATUS_GRAPHICS_WRONG_ALLOCATION_DEVICE: NTSTATUS = NTSTATUS(-1071775467i32);
pub const STATUS_GROUP_EXISTS: NTSTATUS = NTSTATUS(-1073741723i32);
pub const STATUS_GUARD_PAGE_VIOLATION: NTSTATUS = NTSTATUS(-2147483647i32);
pub const STATUS_GUIDS_EXHAUSTED: NTSTATUS = NTSTATUS(-1073741693i32);
pub const STATUS_GUID_SUBSTITUTION_MADE: NTSTATUS = NTSTATUS(-2147483636i32);
pub const STATUS_HANDLES_CLOSED: NTSTATUS = NTSTATUS(-2147483638i32);
pub const STATUS_HANDLE_NOT_CLOSABLE: NTSTATUS = NTSTATUS(-1073741259i32);
pub const STATUS_HANDLE_NO_LONGER_VALID: NTSTATUS = NTSTATUS(-1072103384i32);
pub const STATUS_HANDLE_REVOKED: NTSTATUS = NTSTATUS(-1073700858i32);
pub const STATUS_HARDWARE_MEMORY_ERROR: NTSTATUS = NTSTATUS(-1073740023i32);
pub const STATUS_HASH_NOT_PRESENT: NTSTATUS = NTSTATUS(-1073700607i32);
pub const STATUS_HASH_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073700608i32);
pub const STATUS_HAS_SYSTEM_CRITICAL_FILES: NTSTATUS = NTSTATUS(-1073740611i32);
pub const STATUS_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1069285373i32);
pub const STATUS_HDAUDIO_EMPTY_CONNECTION_LIST: NTSTATUS = NTSTATUS(-1069285374i32);
pub const STATUS_HDAUDIO_NO_LOGICAL_DEVICES_CREATED: NTSTATUS = NTSTATUS(-1069285372i32);
pub const STATUS_HDAUDIO_NULL_LINKED_LIST_ENTRY: NTSTATUS = NTSTATUS(-1069285371i32);
pub const STATUS_HEAP_CORRUPTION: NTSTATUS = NTSTATUS(-1073740940i32);
pub const STATUS_HEURISTIC_DAMAGE_POSSIBLE: NTSTATUS = NTSTATUS(1075380225i32);
pub const STATUS_HIBERNATED: NTSTATUS = NTSTATUS(1073741866i32);
pub const STATUS_HIBERNATION_FAILURE: NTSTATUS = NTSTATUS(-1073740783i32);
pub const STATUS_HIVE_UNLOADED: NTSTATUS = NTSTATUS(-1073740763i32);
pub const STATUS_HMAC_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073700863i32);
pub const STATUS_HOPLIMIT_EXCEEDED: NTSTATUS = NTSTATUS(-1073700846i32);
pub const STATUS_HOST_DOWN: NTSTATUS = NTSTATUS(-1073740976i32);
pub const STATUS_HOST_UNREACHABLE: NTSTATUS = NTSTATUS(-1073741251i32);
pub const STATUS_HUNG_DISPLAY_DRIVER_THREAD: NTSTATUS = NTSTATUS(-1073740779i32);
pub const STATUS_HV_ACCESS_DENIED: NTSTATUS = NTSTATUS(-1070268410i32);
pub const STATUS_HV_ACKNOWLEDGED: NTSTATUS = NTSTATUS(-1070268394i32);
pub const STATUS_HV_CALL_PENDING: NTSTATUS = NTSTATUS(-1070268295i32);
pub const STATUS_HV_CPUID_FEATURE_VALIDATION_ERROR: NTSTATUS = NTSTATUS(-1070268356i32);
pub const STATUS_HV_CPUID_XSAVE_FEATURE_VALIDATION_ERROR: NTSTATUS = NTSTATUS(-1070268355i32);
pub const STATUS_HV_DEVICE_NOT_IN_DOMAIN: NTSTATUS = NTSTATUS(-1070268298i32);
pub const STATUS_HV_EVENT_BUFFER_ALREADY_FREED: NTSTATUS = NTSTATUS(-1070268300i32);
pub const STATUS_HV_FEATURE_UNAVAILABLE: NTSTATUS = NTSTATUS(-1070268386i32);
pub const STATUS_HV_INACTIVE: NTSTATUS = NTSTATUS(-1070268388i32);
pub const STATUS_HV_INSUFFICIENT_BUFFER: NTSTATUS = NTSTATUS(-1070268365i32);
pub const STATUS_HV_INSUFFICIENT_BUFFERS: NTSTATUS = NTSTATUS(-1070268397i32);
pub const STATUS_HV_INSUFFICIENT_CONTIGUOUS_MEMORY: NTSTATUS = NTSTATUS(-1070268299i32);
pub const STATUS_HV_INSUFFICIENT_DEVICE_DOMAINS: NTSTATUS = NTSTATUS(-1070268360i32);
pub const STATUS_HV_INSUFFICIENT_MEMORY: NTSTATUS = NTSTATUS(-1070268405i32);
pub const STATUS_HV_INSUFFICIENT_ROOT_MEMORY: NTSTATUS = NTSTATUS(-1070268301i32);
pub const STATUS_HV_INVALID_ALIGNMENT: NTSTATUS = NTSTATUS(-1070268412i32);
pub const STATUS_HV_INVALID_CONNECTION_ID: NTSTATUS = NTSTATUS(-1070268398i32);
pub const STATUS_HV_INVALID_CPU_GROUP_ID: NTSTATUS = NTSTATUS(-1070268305i32);
pub const STATUS_HV_INVALID_CPU_GROUP_STATE: NTSTATUS = NTSTATUS(-1070268304i32);
pub const STATUS_HV_INVALID_DEVICE_ID: NTSTATUS = NTSTATUS(-1070268329i32);
pub const STATUS_HV_INVALID_DEVICE_STATE: NTSTATUS = NTSTATUS(-1070268328i32);
pub const STATUS_HV_INVALID_HYPERCALL_CODE: NTSTATUS = NTSTATUS(-1070268414i32);
pub const STATUS_HV_INVALID_HYPERCALL_INPUT: NTSTATUS = NTSTATUS(-1070268413i32);
pub const STATUS_HV_INVALID_LP_INDEX: NTSTATUS = NTSTATUS(-1070268351i32);
pub const STATUS_HV_INVALID_PARAMETER: NTSTATUS = NTSTATUS(-1070268411i32);
pub const STATUS_HV_INVALID_PARTITION_ID: NTSTATUS = NTSTATUS(-1070268403i32);
pub const STATUS_HV_INVALID_PARTITION_STATE: NTSTATUS = NTSTATUS(-1070268409i32);
pub const STATUS_HV_INVALID_PORT_ID: NTSTATUS = NTSTATUS(-1070268399i32);
pub const STATUS_HV_INVALID_PROXIMITY_DOMAIN_INFO: NTSTATUS = NTSTATUS(-1070268390i32);
pub const STATUS_HV_INVALID_REGISTER_VALUE: NTSTATUS = NTSTATUS(-1070268336i32);
pub const STATUS_HV_INVALID_SAVE_RESTORE_STATE: NTSTATUS = NTSTATUS(-1070268393i32);
pub const STATUS_HV_INVALID_SYNIC_STATE: NTSTATUS = NTSTATUS(-1070268392i32);
pub const STATUS_HV_INVALID_VP_INDEX: NTSTATUS = NTSTATUS(-1070268402i32);
pub const STATUS_HV_INVALID_VP_STATE: NTSTATUS = NTSTATUS(-1070268395i32);
pub const STATUS_HV_INVALID_VTL_STATE: NTSTATUS = NTSTATUS(-1070268335i32);
pub const STATUS_HV_MSR_ACCESS_FAILED: NTSTATUS = NTSTATUS(-1070268288i32);
pub const STATUS_HV_NESTED_VM_EXIT: NTSTATUS = NTSTATUS(-1070268297i32);
pub const STATUS_HV_NOT_ACKNOWLEDGED: NTSTATUS = NTSTATUS(-1070268396i32);
pub const STATUS_HV_NOT_ALLOWED_WITH_NESTED_VIRT_ACTIVE: NTSTATUS = NTSTATUS(-1070268302i32);
pub const STATUS_HV_NOT_PRESENT: NTSTATUS = NTSTATUS(-1070264320i32);
pub const STATUS_HV_NO_DATA: NTSTATUS = NTSTATUS(-1070268389i32);
pub const STATUS_HV_NO_RESOURCES: NTSTATUS = NTSTATUS(-1070268387i32);
pub const STATUS_HV_NX_NOT_DETECTED: NTSTATUS = NTSTATUS(-1070268331i32);
pub const STATUS_HV_OBJECT_IN_USE: NTSTATUS = NTSTATUS(-1070268391i32);
pub const STATUS_HV_OPERATION_DENIED: NTSTATUS = NTSTATUS(-1070268408i32);
pub const STATUS_HV_OPERATION_FAILED: NTSTATUS = NTSTATUS(-1070268303i32);
pub const STATUS_HV_PAGE_REQUEST_INVALID: NTSTATUS = NTSTATUS(-1070268320i32);
pub const STATUS_HV_PARTITION_TOO_DEEP: NTSTATUS = NTSTATUS(-1070268404i32);
pub const STATUS_HV_PENDING_PAGE_REQUESTS: NTSTATUS = NTSTATUS(3473497i32);
pub const STATUS_HV_PROCESSOR_STARTUP_TIMEOUT: NTSTATUS = NTSTATUS(-1070268354i32);
pub const STATUS_HV_PROPERTY_VALUE_OUT_OF_RANGE: NTSTATUS = NTSTATUS(-1070268406i32);
pub const STATUS_HV_SMX_ENABLED: NTSTATUS = NTSTATUS(-1070268353i32);
pub const STATUS_HV_UNKNOWN_PROPERTY: NTSTATUS = NTSTATUS(-1070268407i32);
pub const STATUS_ILLEGAL_CHARACTER: NTSTATUS = NTSTATUS(-1073741471i32);
pub const STATUS_ILLEGAL_DLL_RELOCATION: NTSTATUS = NTSTATUS(-1073741207i32);
pub const STATUS_ILLEGAL_ELEMENT_ADDRESS: NTSTATUS = NTSTATUS(-1073741179i32);
pub const STATUS_ILLEGAL_FLOAT_CONTEXT: NTSTATUS = NTSTATUS(-1073741494i32);
pub const STATUS_ILLEGAL_FUNCTION: NTSTATUS = NTSTATUS(-1073741649i32);
pub const STATUS_ILLEGAL_INSTRUCTION: NTSTATUS = NTSTATUS(-1073741795i32);
pub const STATUS_ILL_FORMED_PASSWORD: NTSTATUS = NTSTATUS(-1073741717i32);
pub const STATUS_ILL_FORMED_SERVICE_ENTRY: NTSTATUS = NTSTATUS(-1073741472i32);
pub const STATUS_IMAGE_ALREADY_LOADED: NTSTATUS = NTSTATUS(-1073741554i32);
pub const STATUS_IMAGE_ALREADY_LOADED_AS_DLL: NTSTATUS = NTSTATUS(-1073741411i32);
pub const STATUS_IMAGE_AT_DIFFERENT_BASE: NTSTATUS = NTSTATUS(1073741878i32);
pub const STATUS_IMAGE_CERT_EXPIRED: NTSTATUS = NTSTATUS(-1073740283i32);
pub const STATUS_IMAGE_CERT_REVOKED: NTSTATUS = NTSTATUS(-1073740285i32);
pub const STATUS_IMAGE_CHECKSUM_MISMATCH: NTSTATUS = NTSTATUS(-1073741279i32);
pub const STATUS_IMAGE_LOADED_AS_PATCH_IMAGE: NTSTATUS = NTSTATUS(-1073740608i32);
pub const STATUS_IMAGE_MACHINE_TYPE_MISMATCH: NTSTATUS = NTSTATUS(1073741838i32);
pub const STATUS_IMAGE_MACHINE_TYPE_MISMATCH_EXE: NTSTATUS = NTSTATUS(1073741859i32);
pub const STATUS_IMAGE_MP_UP_MISMATCH: NTSTATUS = NTSTATUS(-1073741239i32);
pub const STATUS_IMAGE_NOT_AT_BASE: NTSTATUS = NTSTATUS(1073741827i32);
pub const STATUS_IMAGE_SUBSYSTEM_NOT_PRESENT: NTSTATUS = NTSTATUS(-1073741405i32);
pub const STATUS_IMPLEMENTATION_LIMIT: NTSTATUS = NTSTATUS(-1073740757i32);
pub const STATUS_INCOMPATIBLE_DRIVER_BLOCKED: NTSTATUS = NTSTATUS(-1073740764i32);
pub const STATUS_INCOMPATIBLE_FILE_MAP: NTSTATUS = NTSTATUS(-1073741747i32);
pub const STATUS_INCOMPATIBLE_WITH_GLOBAL_SHORT_NAME_REGISTRY_SETTING: NTSTATUS =
    NTSTATUS(-1073741410i32);
pub const STATUS_INCORRECT_ACCOUNT_TYPE: NTSTATUS = NTSTATUS(-1073700727i32);
pub const STATUS_INDEX_OUT_OF_BOUNDS: NTSTATUS = NTSTATUS(-1073740591i32);
pub const STATUS_INDOUBT_TRANSACTIONS_EXIST: NTSTATUS = NTSTATUS(-1072103366i32);
pub const STATUS_INFO_LENGTH_MISMATCH: NTSTATUS = NTSTATUS(-1073741820i32);
pub const STATUS_INSTANCE_NOT_AVAILABLE: NTSTATUS = NTSTATUS(-1073741653i32);
pub const STATUS_INSTRUCTION_MISALIGNMENT: NTSTATUS = NTSTATUS(-1073741654i32);
pub const STATUS_INSUFFICIENT_LOGON_INFO: NTSTATUS = NTSTATUS(-1073741232i32);
pub const STATUS_INSUFFICIENT_NVRAM_RESOURCES: NTSTATUS = NTSTATUS(-1073740716i32);
pub const STATUS_INSUFFICIENT_POWER: NTSTATUS = NTSTATUS(-1073741090i32);
pub const STATUS_INSUFFICIENT_RESOURCES: NTSTATUS = NTSTATUS(-1073741670i32);
pub const STATUS_INSUFFICIENT_RESOURCE_FOR_SPECIFIED_SHARED_SECTION_SIZE: NTSTATUS =
    NTSTATUS(-1073740778i32);
pub const STATUS_INSUFFICIENT_VIRTUAL_ADDR_RESOURCES: NTSTATUS = NTSTATUS(-1073740606i32);
pub const STATUS_INSUFF_SERVER_RESOURCES: NTSTATUS = NTSTATUS(-1073741307i32);
pub const STATUS_INTEGER_DIVIDE_BY_ZERO: NTSTATUS = NTSTATUS(-1073741676i32);
pub const STATUS_INTEGER_OVERFLOW: NTSTATUS = NTSTATUS(-1073741675i32);
pub const STATUS_INTERMIXED_KERNEL_EA_OPERATION: NTSTATUS = NTSTATUS(-1073740687i32);
pub const STATUS_INTERNAL_DB_CORRUPTION: NTSTATUS = NTSTATUS(-1073741596i32);
pub const STATUS_INTERNAL_DB_ERROR: NTSTATUS = NTSTATUS(-1073741480i32);
pub const STATUS_INTERNAL_ERROR: NTSTATUS = NTSTATUS(-1073741595i32);
pub const STATUS_INTERRUPTED: NTSTATUS = NTSTATUS(-1073740523i32);
pub const STATUS_INTERRUPT_STILL_CONNECTED: NTSTATUS = NTSTATUS(296i32);
pub const STATUS_INTERRUPT_VECTOR_ALREADY_CONNECTED: NTSTATUS = NTSTATUS(295i32);
pub const STATUS_INVALID_ACCOUNT_NAME: NTSTATUS = NTSTATUS(-1073741726i32);
pub const STATUS_INVALID_ACE_CONDITION: NTSTATUS = NTSTATUS(-1073741406i32);
pub const STATUS_INVALID_ACL: NTSTATUS = NTSTATUS(-1073741705i32);
pub const STATUS_INVALID_ADDRESS: NTSTATUS = NTSTATUS(-1073741503i32);
pub const STATUS_INVALID_ADDRESS_COMPONENT: NTSTATUS = NTSTATUS(-1073741305i32);
pub const STATUS_INVALID_ADDRESS_WILDCARD: NTSTATUS = NTSTATUS(-1073741304i32);
pub const STATUS_INVALID_BLOCK_LENGTH: NTSTATUS = NTSTATUS(-1073741453i32);
pub const STATUS_INVALID_BUFFER_SIZE: NTSTATUS = NTSTATUS(-1073741306i32);
pub const STATUS_INVALID_CAP: NTSTATUS = NTSTATUS(-1073740539i32);
pub const STATUS_INVALID_CID: NTSTATUS = NTSTATUS(-1073741813i32);
pub const STATUS_INVALID_COMPUTER_NAME: NTSTATUS = NTSTATUS(-1073741534i32);
pub const STATUS_INVALID_CONNECTION: NTSTATUS = NTSTATUS(-1073741504i32);
pub const STATUS_INVALID_CRUNTIME_PARAMETER: NTSTATUS = NTSTATUS(-1073740777i32);
pub const STATUS_INVALID_DEVICE_OBJECT_PARAMETER: NTSTATUS = NTSTATUS(-1073740951i32);
pub const STATUS_INVALID_DEVICE_REQUEST: NTSTATUS = NTSTATUS(-1073741808i32);
pub const STATUS_INVALID_DEVICE_STATE: NTSTATUS = NTSTATUS(-1073741436i32);
pub const STATUS_INVALID_DISPOSITION: NTSTATUS = NTSTATUS(-1073741786i32);
pub const STATUS_INVALID_DOMAIN_ROLE: NTSTATUS = NTSTATUS(-1073741602i32);
pub const STATUS_INVALID_DOMAIN_STATE: NTSTATUS = NTSTATUS(-1073741603i32);
pub const STATUS_INVALID_EA_FLAG: NTSTATUS = NTSTATUS(-2147483627i32);
pub const STATUS_INVALID_EA_NAME: NTSTATUS = NTSTATUS(-2147483629i32);
pub const STATUS_INVALID_EXCEPTION_HANDLER: NTSTATUS = NTSTATUS(-1073741403i32);
pub const STATUS_INVALID_FIELD_IN_PARAMETER_LIST: NTSTATUS = NTSTATUS(-1073740683i32);
pub const STATUS_INVALID_FILE_FOR_SECTION: NTSTATUS = NTSTATUS(-1073741792i32);
pub const STATUS_INVALID_GROUP_ATTRIBUTES: NTSTATUS = NTSTATUS(-1073741660i32);
pub const STATUS_INVALID_HANDLE: NTSTATUS = NTSTATUS(-1073741816i32);
pub const STATUS_INVALID_HW_PROFILE: NTSTATUS = NTSTATUS(-1073741216i32);
pub const STATUS_INVALID_IDN_NORMALIZATION: NTSTATUS = NTSTATUS(-1073740010i32);
pub const STATUS_INVALID_ID_AUTHORITY: NTSTATUS = NTSTATUS(-1073741692i32);
pub const STATUS_INVALID_IMAGE_FORMAT: NTSTATUS = NTSTATUS(-1073741701i32);
pub const STATUS_INVALID_IMAGE_HASH: NTSTATUS = NTSTATUS(-1073740760i32);
pub const STATUS_INVALID_IMAGE_LE_FORMAT: NTSTATUS = NTSTATUS(-1073741522i32);
pub const STATUS_INVALID_IMAGE_NE_FORMAT: NTSTATUS = NTSTATUS(-1073741541i32);
pub const STATUS_INVALID_IMAGE_NOT_MZ: NTSTATUS = NTSTATUS(-1073741521i32);
pub const STATUS_INVALID_IMAGE_PROTECT: NTSTATUS = NTSTATUS(-1073741520i32);
pub const STATUS_INVALID_IMAGE_WIN_16: NTSTATUS = NTSTATUS(-1073741519i32);
pub const STATUS_INVALID_IMAGE_WIN_32: NTSTATUS = NTSTATUS(-1073740967i32);
pub const STATUS_INVALID_IMAGE_WIN_64: NTSTATUS = NTSTATUS(-1073740966i32);
pub const STATUS_INVALID_IMPORT_OF_NON_DLL: NTSTATUS = NTSTATUS(-1073740945i32);
pub const STATUS_INVALID_INFO_CLASS: NTSTATUS = NTSTATUS(-1073741821i32);
pub const STATUS_INVALID_INITIATOR_TARGET_PATH: NTSTATUS = NTSTATUS(-1073740681i32);
pub const STATUS_INVALID_KERNEL_INFO_VERSION: NTSTATUS = NTSTATUS(-1073700860i32);
pub const STATUS_INVALID_LABEL: NTSTATUS = NTSTATUS(-1073740730i32);
pub const STATUS_INVALID_LDT_DESCRIPTOR: NTSTATUS = NTSTATUS(-1073741542i32);
pub const STATUS_INVALID_LDT_OFFSET: NTSTATUS = NTSTATUS(-1073741543i32);
pub const STATUS_INVALID_LDT_SIZE: NTSTATUS = NTSTATUS(-1073741544i32);
pub const STATUS_INVALID_LEVEL: NTSTATUS = NTSTATUS(-1073741496i32);
pub const STATUS_INVALID_LOCK_RANGE: NTSTATUS = NTSTATUS(-1073741407i32);
pub const STATUS_INVALID_LOCK_SEQUENCE: NTSTATUS = NTSTATUS(-1073741794i32);
pub const STATUS_INVALID_LOGON_HOURS: NTSTATUS = NTSTATUS(-1073741713i32);
pub const STATUS_INVALID_LOGON_TYPE: NTSTATUS = NTSTATUS(-1073741557i32);
pub const STATUS_INVALID_MEMBER: NTSTATUS = NTSTATUS(-1073741445i32);
pub const STATUS_INVALID_MESSAGE: NTSTATUS = NTSTATUS(-1073740030i32);
pub const STATUS_INVALID_NETWORK_RESPONSE: NTSTATUS = NTSTATUS(-1073741629i32);
pub const STATUS_INVALID_OFFSET_ALIGNMENT: NTSTATUS = NTSTATUS(-1073740684i32);
pub const STATUS_INVALID_OPLOCK_PROTOCOL: NTSTATUS = NTSTATUS(-1073741597i32);
pub const STATUS_INVALID_OWNER: NTSTATUS = NTSTATUS(-1073741734i32);
pub const STATUS_INVALID_PACKAGE_SID_LENGTH: NTSTATUS = NTSTATUS(-1073700350i32);
pub const STATUS_INVALID_PAGE_PROTECTION: NTSTATUS = NTSTATUS(-1073741755i32);
pub const STATUS_INVALID_PARAMETER: NTSTATUS = NTSTATUS(-1073741811i32);
pub const STATUS_INVALID_PARAMETER_1: NTSTATUS = NTSTATUS(-1073741585i32);
pub const STATUS_INVALID_PARAMETER_10: NTSTATUS = NTSTATUS(-1073741576i32);
pub const STATUS_INVALID_PARAMETER_11: NTSTATUS = NTSTATUS(-1073741575i32);
pub const STATUS_INVALID_PARAMETER_12: NTSTATUS = NTSTATUS(-1073741574i32);
pub const STATUS_INVALID_PARAMETER_2: NTSTATUS = NTSTATUS(-1073741584i32);
pub const STATUS_INVALID_PARAMETER_3: NTSTATUS = NTSTATUS(-1073741583i32);
pub const STATUS_INVALID_PARAMETER_4: NTSTATUS = NTSTATUS(-1073741582i32);
pub const STATUS_INVALID_PARAMETER_5: NTSTATUS = NTSTATUS(-1073741581i32);
pub const STATUS_INVALID_PARAMETER_6: NTSTATUS = NTSTATUS(-1073741580i32);
pub const STATUS_INVALID_PARAMETER_7: NTSTATUS = NTSTATUS(-1073741579i32);
pub const STATUS_INVALID_PARAMETER_8: NTSTATUS = NTSTATUS(-1073741578i32);
pub const STATUS_INVALID_PARAMETER_9: NTSTATUS = NTSTATUS(-1073741577i32);
pub const STATUS_INVALID_PARAMETER_MIX: NTSTATUS = NTSTATUS(-1073741776i32);
pub const STATUS_INVALID_PEP_INFO_VERSION: NTSTATUS = NTSTATUS(-1073700859i32);
pub const STATUS_INVALID_PIPE_STATE: NTSTATUS = NTSTATUS(-1073741651i32);
pub const STATUS_INVALID_PLUGPLAY_DEVICE_PATH: NTSTATUS = NTSTATUS(-1073741215i32);
pub const STATUS_INVALID_PORT_ATTRIBUTES: NTSTATUS = NTSTATUS(-1073741778i32);
pub const STATUS_INVALID_PORT_HANDLE: NTSTATUS = NTSTATUS(-1073741758i32);
pub const STATUS_INVALID_PRIMARY_GROUP: NTSTATUS = NTSTATUS(-1073741733i32);
pub const STATUS_INVALID_QUOTA_LOWER: NTSTATUS = NTSTATUS(-1073741775i32);
pub const STATUS_INVALID_READ_MODE: NTSTATUS = NTSTATUS(-1073741644i32);
pub const STATUS_INVALID_RUNLEVEL_SETTING: NTSTATUS = NTSTATUS(-1073700542i32);
pub const STATUS_INVALID_SECURITY_DESCR: NTSTATUS = NTSTATUS(-1073741703i32);
pub const STATUS_INVALID_SERVER_STATE: NTSTATUS = NTSTATUS(-1073741604i32);
pub const STATUS_INVALID_SESSION: NTSTATUS = NTSTATUS(-1073740715i32);
pub const STATUS_INVALID_SID: NTSTATUS = NTSTATUS(-1073741704i32);
pub const STATUS_INVALID_SIGNATURE: NTSTATUS = NTSTATUS(-1073700864i32);
pub const STATUS_INVALID_STATE_TRANSITION: NTSTATUS = NTSTATUS(-1073700861i32);
pub const STATUS_INVALID_SUB_AUTHORITY: NTSTATUS = NTSTATUS(-1073741706i32);
pub const STATUS_INVALID_SYSTEM_SERVICE: NTSTATUS = NTSTATUS(-1073741796i32);
pub const STATUS_INVALID_TASK_INDEX: NTSTATUS = NTSTATUS(-1073740543i32);
pub const STATUS_INVALID_TASK_NAME: NTSTATUS = NTSTATUS(-1073740544i32);
pub const STATUS_INVALID_THREAD: NTSTATUS = NTSTATUS(-1073740004i32);
pub const STATUS_INVALID_TOKEN: NTSTATUS = NTSTATUS(-1073740699i32);
pub const STATUS_INVALID_TRANSACTION: NTSTATUS = NTSTATUS(-1072103422i32);
pub const STATUS_INVALID_UNWIND_TARGET: NTSTATUS = NTSTATUS(-1073741783i32);
pub const STATUS_INVALID_USER_BUFFER: NTSTATUS = NTSTATUS(-1073741592i32);
pub const STATUS_INVALID_USER_PRINCIPAL_NAME: NTSTATUS = NTSTATUS(-1073740772i32);
pub const STATUS_INVALID_VARIANT: NTSTATUS = NTSTATUS(-1073741262i32);
pub const STATUS_INVALID_VIEW_SIZE: NTSTATUS = NTSTATUS(-1073741793i32);
pub const STATUS_INVALID_VOLUME_LABEL: NTSTATUS = NTSTATUS(-1073741690i32);
pub const STATUS_INVALID_WEIGHT: NTSTATUS = NTSTATUS(-1073740712i32);
pub const STATUS_INVALID_WORKSTATION: NTSTATUS = NTSTATUS(-1073741712i32);
pub const STATUS_IN_PAGE_ERROR: NTSTATUS = NTSTATUS(-1073741818i32);
pub const STATUS_IORING_COMPLETION_QUEUE_TOO_BIG: NTSTATUS = NTSTATUS(-1069154299i32);
pub const STATUS_IORING_CORRUPT: NTSTATUS = NTSTATUS(-1069154297i32);
pub const STATUS_IORING_REQUIRED_FLAG_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1069154303i32);
pub const STATUS_IORING_SUBMISSION_QUEUE_FULL: NTSTATUS = NTSTATUS(-1069154302i32);
pub const STATUS_IORING_SUBMISSION_QUEUE_TOO_BIG: NTSTATUS = NTSTATUS(-1069154300i32);
pub const STATUS_IORING_SUBMIT_IN_PROGRESS: NTSTATUS = NTSTATUS(-1069154298i32);
pub const STATUS_IORING_VERSION_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1069154301i32);
pub const STATUS_IO_DEVICE_ERROR: NTSTATUS = NTSTATUS(-1073741435i32);
pub const STATUS_IO_DEVICE_INVALID_DATA: NTSTATUS = NTSTATUS(-1073741392i32);
pub const STATUS_IO_OPERATION_TIMEOUT: NTSTATUS = NTSTATUS(-1073740675i32);
pub const STATUS_IO_PREEMPTED: NTSTATUS = NTSTATUS(-1068433407i32);
pub const STATUS_IO_PRIVILEGE_FAILED: NTSTATUS = NTSTATUS(-1073741513i32);
pub const STATUS_IO_REISSUE_AS_CACHED: NTSTATUS = NTSTATUS(-1073479623i32);
pub const STATUS_IO_REPARSE_DATA_INVALID: NTSTATUS = NTSTATUS(-1073741192i32);
pub const STATUS_IO_REPARSE_TAG_INVALID: NTSTATUS = NTSTATUS(-1073741194i32);
pub const STATUS_IO_REPARSE_TAG_MISMATCH: NTSTATUS = NTSTATUS(-1073741193i32);
pub const STATUS_IO_REPARSE_TAG_NOT_HANDLED: NTSTATUS = NTSTATUS(-1073741191i32);
pub const STATUS_IO_TIMEOUT: NTSTATUS = NTSTATUS(-1073741643i32);
pub const STATUS_IO_UNALIGNED_WRITE: NTSTATUS = NTSTATUS(-1073741391i32);
pub const STATUS_IPSEC_AUTH_FIREWALL_DROP: NTSTATUS = NTSTATUS(-1070202872i32);
pub const STATUS_IPSEC_BAD_SPI: NTSTATUS = NTSTATUS(-1070202879i32);
pub const STATUS_IPSEC_CLEAR_TEXT_DROP: NTSTATUS = NTSTATUS(-1070202873i32);
pub const STATUS_IPSEC_DOSP_BLOCK: NTSTATUS = NTSTATUS(-1070170112i32);
pub const STATUS_IPSEC_DOSP_INVALID_PACKET: NTSTATUS = NTSTATUS(-1070170110i32);
pub const STATUS_IPSEC_DOSP_KEYMOD_NOT_ALLOWED: NTSTATUS = NTSTATUS(-1070170107i32);
pub const STATUS_IPSEC_DOSP_MAX_ENTRIES: NTSTATUS = NTSTATUS(-1070170108i32);
pub const STATUS_IPSEC_DOSP_MAX_PER_IP_RATELIMIT_QUEUES: NTSTATUS = NTSTATUS(-1070170106i32);
pub const STATUS_IPSEC_DOSP_RECEIVED_MULTICAST: NTSTATUS = NTSTATUS(-1070170111i32);
pub const STATUS_IPSEC_DOSP_STATE_LOOKUP_FAILED: NTSTATUS = NTSTATUS(-1070170109i32);
pub const STATUS_IPSEC_INTEGRITY_CHECK_FAILED: NTSTATUS = NTSTATUS(-1070202874i32);
pub const STATUS_IPSEC_INVALID_PACKET: NTSTATUS = NTSTATUS(-1070202875i32);
pub const STATUS_IPSEC_QUEUE_OVERFLOW: NTSTATUS = NTSTATUS(-1073700848i32);
pub const STATUS_IPSEC_REPLAY_CHECK_FAILED: NTSTATUS = NTSTATUS(-1070202876i32);
pub const STATUS_IPSEC_SA_LIFETIME_EXPIRED: NTSTATUS = NTSTATUS(-1070202878i32);
pub const STATUS_IPSEC_THROTTLE_DROP: NTSTATUS = NTSTATUS(-1070202871i32);
pub const STATUS_IPSEC_WRONG_SA: NTSTATUS = NTSTATUS(-1070202877i32);
pub const STATUS_IP_ADDRESS_CONFLICT1: NTSTATUS = NTSTATUS(-1073741228i32);
pub const STATUS_IP_ADDRESS_CONFLICT2: NTSTATUS = NTSTATUS(-1073741227i32);
pub const STATUS_ISSUING_CA_UNTRUSTED: NTSTATUS = NTSTATUS(-1073740918i32);
pub const STATUS_ISSUING_CA_UNTRUSTED_KDC: NTSTATUS = NTSTATUS(-1073740787i32);
pub const STATUS_JOB_NOT_EMPTY: NTSTATUS = NTSTATUS(-1073740529i32);
pub const STATUS_JOB_NO_CONTAINER: NTSTATUS = NTSTATUS(-1073740535i32);
pub const STATUS_JOURNAL_DELETE_IN_PROGRESS: NTSTATUS = NTSTATUS(-1073741129i32);
pub const STATUS_JOURNAL_ENTRY_DELETED: NTSTATUS = NTSTATUS(-1073741105i32);
pub const STATUS_JOURNAL_NOT_ACTIVE: NTSTATUS = NTSTATUS(-1073741128i32);
pub const STATUS_KDC_CERT_EXPIRED: NTSTATUS = NTSTATUS(-1073740786i32);
pub const STATUS_KDC_CERT_REVOKED: NTSTATUS = NTSTATUS(-1073740785i32);
pub const STATUS_KDC_INVALID_REQUEST: NTSTATUS = NTSTATUS(-1073741061i32);
pub const STATUS_KDC_UNABLE_TO_REFER: NTSTATUS = NTSTATUS(-1073741060i32);
pub const STATUS_KDC_UNKNOWN_ETYPE: NTSTATUS = NTSTATUS(-1073741059i32);
pub const STATUS_KERNEL_APC: NTSTATUS = NTSTATUS(256i32);
pub const STATUS_KERNEL_EXECUTABLE_MEMORY_WRITE: NTSTATUS = NTSTATUS(-1073739996i32);
pub const STATUS_KEY_DELETED: NTSTATUS = NTSTATUS(-1073741444i32);
pub const STATUS_KEY_HAS_CHILDREN: NTSTATUS = NTSTATUS(-1073741440i32);
pub const STATUS_LAST_ADMIN: NTSTATUS = NTSTATUS(-1073741719i32);
pub const STATUS_LICENSE_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(-1073741223i32);
pub const STATUS_LICENSE_VIOLATION: NTSTATUS = NTSTATUS(-1073741206i32);
pub const STATUS_LINK_FAILED: NTSTATUS = NTSTATUS(-1073741506i32);
pub const STATUS_LINK_TIMEOUT: NTSTATUS = NTSTATUS(-1073741505i32);
pub const STATUS_LM_CROSS_ENCRYPTION_REQUIRED: NTSTATUS = NTSTATUS(-1073741441i32);
pub const STATUS_LOCAL_DISCONNECT: NTSTATUS = NTSTATUS(-1073741509i32);
pub const STATUS_LOCAL_USER_SESSION_KEY: NTSTATUS = NTSTATUS(1073741830i32);
pub const STATUS_LOCK_NOT_GRANTED: NTSTATUS = NTSTATUS(-1073741739i32);
pub const STATUS_LOGIN_TIME_RESTRICTION: NTSTATUS = NTSTATUS(-1073741241i32);
pub const STATUS_LOGIN_WKSTA_RESTRICTION: NTSTATUS = NTSTATUS(-1073741240i32);
pub const STATUS_LOGON_NOT_GRANTED: NTSTATUS = NTSTATUS(-1073741483i32);
pub const STATUS_LOGON_SERVER_CONFLICT: NTSTATUS = NTSTATUS(-1073741518i32);
pub const STATUS_LOGON_SESSION_COLLISION: NTSTATUS = NTSTATUS(-1073741563i32);
pub const STATUS_LOGON_SESSION_EXISTS: NTSTATUS = NTSTATUS(-1073741586i32);
pub const STATUS_LOG_APPENDED_FLUSH_FAILED: NTSTATUS = NTSTATUS(-1072037841i32);
pub const STATUS_LOG_ARCHIVE_IN_PROGRESS: NTSTATUS = NTSTATUS(-1072037855i32);
pub const STATUS_LOG_ARCHIVE_NOT_IN_PROGRESS: NTSTATUS = NTSTATUS(-1072037856i32);
pub const STATUS_LOG_BLOCKS_EXHAUSTED: NTSTATUS = NTSTATUS(-1072037882i32);
pub const STATUS_LOG_BLOCK_INCOMPLETE: NTSTATUS = NTSTATUS(-1072037884i32);
pub const STATUS_LOG_BLOCK_INVALID: NTSTATUS = NTSTATUS(-1072037878i32);
pub const STATUS_LOG_BLOCK_VERSION: NTSTATUS = NTSTATUS(-1072037879i32);
pub const STATUS_LOG_CANT_DELETE: NTSTATUS = NTSTATUS(-1072037871i32);
pub const STATUS_LOG_CLIENT_ALREADY_REGISTERED: NTSTATUS = NTSTATUS(-1072037852i32);
pub const STATUS_LOG_CLIENT_NOT_REGISTERED: NTSTATUS = NTSTATUS(-1072037851i32);
pub const STATUS_LOG_CONTAINER_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(-1072037870i32);
pub const STATUS_LOG_CONTAINER_OPEN_FAILED: NTSTATUS = NTSTATUS(-1072037847i32);
pub const STATUS_LOG_CONTAINER_READ_FAILED: NTSTATUS = NTSTATUS(-1072037849i32);
pub const STATUS_LOG_CONTAINER_STATE_INVALID: NTSTATUS = NTSTATUS(-1072037846i32);
pub const STATUS_LOG_CONTAINER_WRITE_FAILED: NTSTATUS = NTSTATUS(-1072037848i32);
pub const STATUS_LOG_CORRUPTION_DETECTED: NTSTATUS = NTSTATUS(-1072103376i32);
pub const STATUS_LOG_DEDICATED: NTSTATUS = NTSTATUS(-1072037857i32);
pub const STATUS_LOG_EPHEMERAL: NTSTATUS = NTSTATUS(-1072037854i32);
pub const STATUS_LOG_FILE_FULL: NTSTATUS = NTSTATUS(-1073741432i32);
pub const STATUS_LOG_FULL: NTSTATUS = NTSTATUS(-1072037859i32);
pub const STATUS_LOG_FULL_HANDLER_IN_PROGRESS: NTSTATUS = NTSTATUS(-1072037850i32);
pub const STATUS_LOG_GROWTH_FAILED: NTSTATUS = NTSTATUS(-1072103399i32);
pub const STATUS_LOG_HARD_ERROR: NTSTATUS = NTSTATUS(1073741850i32);
pub const STATUS_LOG_INCONSISTENT_SECURITY: NTSTATUS = NTSTATUS(-1072037842i32);
pub const STATUS_LOG_INVALID_RANGE: NTSTATUS = NTSTATUS(-1072037883i32);
pub const STATUS_LOG_METADATA_CORRUPT: NTSTATUS = NTSTATUS(-1072037875i32);
pub const STATUS_LOG_METADATA_FLUSH_FAILED: NTSTATUS = NTSTATUS(-1072037843i32);
pub const STATUS_LOG_METADATA_INCONSISTENT: NTSTATUS = NTSTATUS(-1072037873i32);
pub const STATUS_LOG_METADATA_INVALID: NTSTATUS = NTSTATUS(-1072037874i32);
pub const STATUS_LOG_MULTIPLEXED: NTSTATUS = NTSTATUS(-1072037858i32);
pub const STATUS_LOG_NOT_ENOUGH_CONTAINERS: NTSTATUS = NTSTATUS(-1072037853i32);
pub const STATUS_LOG_NO_RESTART: NTSTATUS = NTSTATUS(1075445772i32);
pub const STATUS_LOG_PINNED: NTSTATUS = NTSTATUS(-1072037844i32);
pub const STATUS_LOG_PINNED_ARCHIVE_TAIL: NTSTATUS = NTSTATUS(-1072037864i32);
pub const STATUS_LOG_PINNED_RESERVATION: NTSTATUS = NTSTATUS(-1072037840i32);
pub const STATUS_LOG_POLICY_ALREADY_INSTALLED: NTSTATUS = NTSTATUS(-1072037868i32);
pub const STATUS_LOG_POLICY_CONFLICT: NTSTATUS = NTSTATUS(-1072037865i32);
pub const STATUS_LOG_POLICY_INVALID: NTSTATUS = NTSTATUS(-1072037866i32);
pub const STATUS_LOG_POLICY_NOT_INSTALLED: NTSTATUS = NTSTATUS(-1072037867i32);
pub const STATUS_LOG_READ_CONTEXT_INVALID: NTSTATUS = NTSTATUS(-1072037881i32);
pub const STATUS_LOG_READ_MODE_INVALID: NTSTATUS = NTSTATUS(-1072037877i32);
pub const STATUS_LOG_RECORDS_RESERVED_INVALID: NTSTATUS = NTSTATUS(-1072037862i32);
pub const STATUS_LOG_RECORD_NONEXISTENT: NTSTATUS = NTSTATUS(-1072037863i32);
pub const STATUS_LOG_RESERVATION_INVALID: NTSTATUS = NTSTATUS(-1072037872i32);
pub const STATUS_LOG_RESIZE_INVALID_SIZE: NTSTATUS = NTSTATUS(-1072103413i32);
pub const STATUS_LOG_RESTART_INVALID: NTSTATUS = NTSTATUS(-1072037880i32);
pub const STATUS_LOG_SECTOR_INVALID: NTSTATUS = NTSTATUS(-1072037887i32);
pub const STATUS_LOG_SECTOR_PARITY_INVALID: NTSTATUS = NTSTATUS(-1072037886i32);
pub const STATUS_LOG_SECTOR_REMAPPED: NTSTATUS = NTSTATUS(-1072037885i32);
pub const STATUS_LOG_SPACE_RESERVED_INVALID: NTSTATUS = NTSTATUS(-1072037861i32);
pub const STATUS_LOG_START_OF_LOG: NTSTATUS = NTSTATUS(-1072037869i32);
pub const STATUS_LOG_STATE_INVALID: NTSTATUS = NTSTATUS(-1072037845i32);
pub const STATUS_LOG_TAIL_INVALID: NTSTATUS = NTSTATUS(-1072037860i32);
pub const STATUS_LONGJUMP: NTSTATUS = NTSTATUS(-2147483610i32);
pub const STATUS_LOST_MODE_LOGON_RESTRICTION: NTSTATUS = NTSTATUS(-1073741043i32);
pub const STATUS_LOST_WRITEBEHIND_DATA: NTSTATUS = NTSTATUS(-1073741278i32);
pub const STATUS_LOST_WRITEBEHIND_DATA_LOCAL_DISK_ERROR: NTSTATUS = NTSTATUS(-1073700734i32);
pub const STATUS_LOST_WRITEBEHIND_DATA_NETWORK_DISCONNECTED: NTSTATUS = NTSTATUS(-1073700736i32);
pub const STATUS_LOST_WRITEBEHIND_DATA_NETWORK_SERVER_ERROR: NTSTATUS = NTSTATUS(-1073700735i32);
pub const STATUS_LPAC_ACCESS_DENIED: NTSTATUS = NTSTATUS(-1073700349i32);
pub const STATUS_LPC_HANDLE_COUNT_EXCEEDED: NTSTATUS = NTSTATUS(-1073739998i32);
pub const STATUS_LPC_INVALID_CONNECTION_USAGE: NTSTATUS = NTSTATUS(-1073740026i32);
pub const STATUS_LPC_RECEIVE_BUFFER_EXPECTED: NTSTATUS = NTSTATUS(-1073740027i32);
pub const STATUS_LPC_REPLY_LOST: NTSTATUS = NTSTATUS(-1073741229i32);
pub const STATUS_LPC_REQUESTS_NOT_ALLOWED: NTSTATUS = NTSTATUS(-1073740025i32);
pub const STATUS_LUIDS_EXHAUSTED: NTSTATUS = NTSTATUS(-1073741707i32);
pub const STATUS_MAGAZINE_NOT_PRESENT: NTSTATUS = NTSTATUS(-1073741178i32);
pub const STATUS_MAPPED_ALIGNMENT: NTSTATUS = NTSTATUS(-1073741280i32);
pub const STATUS_MAPPED_FILE_SIZE_ZERO: NTSTATUS = NTSTATUS(-1073741538i32);
pub const STATUS_MARKED_TO_DISALLOW_WRITES: NTSTATUS = NTSTATUS(-1073740659i32);
pub const STATUS_MARSHALL_OVERFLOW: NTSTATUS = NTSTATUS(-1073741263i32);
pub const STATUS_MAX_REFERRALS_EXCEEDED: NTSTATUS = NTSTATUS(-1073741068i32);
pub const STATUS_MCA_EXCEPTION: NTSTATUS = NTSTATUS(-1073740013i32);
pub const STATUS_MCA_OCCURED: NTSTATUS = NTSTATUS(-1073740950i32);
pub const STATUS_MEDIA_CHANGED: NTSTATUS = NTSTATUS(-2147483620i32);
pub const STATUS_MEDIA_CHECK: NTSTATUS = NTSTATUS(-2147483616i32);
pub const STATUS_MEDIA_WRITE_PROTECTED: NTSTATUS = NTSTATUS(-1073741662i32);
pub const STATUS_MEMBERS_PRIMARY_GROUP: NTSTATUS = NTSTATUS(-1073741529i32);
pub const STATUS_MEMBER_IN_ALIAS: NTSTATUS = NTSTATUS(-1073741485i32);
pub const STATUS_MEMBER_IN_GROUP: NTSTATUS = NTSTATUS(-1073741721i32);
pub const STATUS_MEMBER_NOT_IN_ALIAS: NTSTATUS = NTSTATUS(-1073741486i32);
pub const STATUS_MEMBER_NOT_IN_GROUP: NTSTATUS = NTSTATUS(-1073741720i32);
pub const STATUS_MEMORY_NOT_ALLOCATED: NTSTATUS = NTSTATUS(-1073741664i32);
pub const STATUS_MESSAGE_LOST: NTSTATUS = NTSTATUS(-1073740031i32);
pub const STATUS_MESSAGE_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741559i32);
pub const STATUS_MESSAGE_RETRIEVED: NTSTATUS = NTSTATUS(1073741870i32);
pub const STATUS_MFT_TOO_FRAGMENTED: NTSTATUS = NTSTATUS(-1073741052i32);
pub const STATUS_MINIVERSION_INACCESSIBLE_FROM_SPECIFIED_TRANSACTION: NTSTATUS =
    NTSTATUS(-1072103388i32);
pub const STATUS_MISSING_SYSTEMFILE: NTSTATUS = NTSTATUS(-1073741501i32);
pub const STATUS_MONITOR_INVALID_DESCRIPTOR_CHECKSUM: NTSTATUS = NTSTATUS(-1071841277i32);
pub const STATUS_MONITOR_INVALID_DETAILED_TIMING_BLOCK: NTSTATUS = NTSTATUS(-1071841271i32);
pub const STATUS_MONITOR_INVALID_MANUFACTURE_DATE: NTSTATUS = NTSTATUS(-1071841270i32);
pub const STATUS_MONITOR_INVALID_SERIAL_NUMBER_MONDSC_BLOCK: NTSTATUS = NTSTATUS(-1071841274i32);
pub const STATUS_MONITOR_INVALID_STANDARD_TIMING_BLOCK: NTSTATUS = NTSTATUS(-1071841276i32);
pub const STATUS_MONITOR_INVALID_USER_FRIENDLY_MONDSC_BLOCK: NTSTATUS = NTSTATUS(-1071841273i32);
pub const STATUS_MONITOR_NO_DESCRIPTOR: NTSTATUS = NTSTATUS(-1071841279i32);
pub const STATUS_MONITOR_NO_MORE_DESCRIPTOR_DATA: NTSTATUS = NTSTATUS(-1071841272i32);
pub const STATUS_MONITOR_UNKNOWN_DESCRIPTOR_FORMAT: NTSTATUS = NTSTATUS(-1071841278i32);
pub const STATUS_MONITOR_WMI_DATABLOCK_REGISTRATION_FAILED: NTSTATUS = NTSTATUS(-1071841275i32);
pub const STATUS_MORE_ENTRIES: NTSTATUS = NTSTATUS(261i32);
pub const STATUS_MORE_PROCESSING_REQUIRED: NTSTATUS = NTSTATUS(-1073741802i32);
pub const STATUS_MOUNT_POINT_NOT_RESOLVED: NTSTATUS = NTSTATUS(-1073740952i32);
pub const STATUS_MP_PROCESSOR_MISMATCH: NTSTATUS = NTSTATUS(1073741865i32);
pub const STATUS_MUI_FILE_NOT_FOUND: NTSTATUS = NTSTATUS(-1073020927i32);
pub const STATUS_MUI_FILE_NOT_LOADED: NTSTATUS = NTSTATUS(-1073020922i32);
pub const STATUS_MUI_INVALID_FILE: NTSTATUS = NTSTATUS(-1073020926i32);
pub const STATUS_MUI_INVALID_LOCALE_NAME: NTSTATUS = NTSTATUS(-1073020924i32);
pub const STATUS_MUI_INVALID_RC_CONFIG: NTSTATUS = NTSTATUS(-1073020925i32);
pub const STATUS_MUI_INVALID_ULTIMATEFALLBACK_NAME: NTSTATUS = NTSTATUS(-1073020923i32);
pub const STATUS_MULTIPLE_FAULT_VIOLATION: NTSTATUS = NTSTATUS(-1073741080i32);
pub const STATUS_MUST_BE_KDC: NTSTATUS = NTSTATUS(-1073741067i32);
pub const STATUS_MUTANT_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(-1073741423i32);
pub const STATUS_MUTANT_NOT_OWNED: NTSTATUS = NTSTATUS(-1073741754i32);
pub const STATUS_MUTUAL_AUTHENTICATION_FAILED: NTSTATUS = NTSTATUS(-1073741117i32);
pub const STATUS_NAME_TOO_LONG: NTSTATUS = NTSTATUS(-1073741562i32);
pub const STATUS_NDIS_ADAPTER_NOT_FOUND: NTSTATUS = NTSTATUS(-1071448058i32);
pub const STATUS_NDIS_ADAPTER_NOT_READY: NTSTATUS = NTSTATUS(-1071448047i32);
pub const STATUS_NDIS_ADAPTER_REMOVED: NTSTATUS = NTSTATUS(-1071448040i32);
pub const STATUS_NDIS_ALREADY_MAPPED: NTSTATUS = NTSTATUS(-1071448035i32);
pub const STATUS_NDIS_BAD_CHARACTERISTICS: NTSTATUS = NTSTATUS(-1071448059i32);
pub const STATUS_NDIS_BAD_VERSION: NTSTATUS = NTSTATUS(-1071448060i32);
pub const STATUS_NDIS_BUFFER_TOO_SHORT: NTSTATUS = NTSTATUS(-1071448042i32);
pub const STATUS_NDIS_CLOSING: NTSTATUS = NTSTATUS(-1071448062i32);
pub const STATUS_NDIS_DEVICE_FAILED: NTSTATUS = NTSTATUS(-1071448056i32);
pub const STATUS_NDIS_DOT11_AP_BAND_CURRENTLY_NOT_AVAILABLE: NTSTATUS = NTSTATUS(-1071439866i32);
pub const STATUS_NDIS_DOT11_AP_BAND_NOT_ALLOWED: NTSTATUS = NTSTATUS(-1071439864i32);
pub const STATUS_NDIS_DOT11_AP_CHANNEL_CURRENTLY_NOT_AVAILABLE: NTSTATUS = NTSTATUS(-1071439867i32);
pub const STATUS_NDIS_DOT11_AP_CHANNEL_NOT_ALLOWED: NTSTATUS = NTSTATUS(-1071439865i32);
pub const STATUS_NDIS_DOT11_AUTO_CONFIG_ENABLED: NTSTATUS = NTSTATUS(-1071439872i32);
pub const STATUS_NDIS_DOT11_MEDIA_IN_USE: NTSTATUS = NTSTATUS(-1071439871i32);
pub const STATUS_NDIS_DOT11_POWER_STATE_INVALID: NTSTATUS = NTSTATUS(-1071439870i32);
pub const STATUS_NDIS_ERROR_READING_FILE: NTSTATUS = NTSTATUS(-1071448036i32);
pub const STATUS_NDIS_FILE_NOT_FOUND: NTSTATUS = NTSTATUS(-1071448037i32);
pub const STATUS_NDIS_GROUP_ADDRESS_IN_USE: NTSTATUS = NTSTATUS(-1071448038i32);
pub const STATUS_NDIS_INDICATION_REQUIRED: NTSTATUS = NTSTATUS(1076035585i32);
pub const STATUS_NDIS_INTERFACE_NOT_FOUND: NTSTATUS = NTSTATUS(-1071448021i32);
pub const STATUS_NDIS_INVALID_ADDRESS: NTSTATUS = NTSTATUS(-1071448030i32);
pub const STATUS_NDIS_INVALID_DATA: NTSTATUS = NTSTATUS(-1071448043i32);
pub const STATUS_NDIS_INVALID_DEVICE_REQUEST: NTSTATUS = NTSTATUS(-1071448048i32);
pub const STATUS_NDIS_INVALID_LENGTH: NTSTATUS = NTSTATUS(-1071448044i32);
pub const STATUS_NDIS_INVALID_OID: NTSTATUS = NTSTATUS(-1071448041i32);
pub const STATUS_NDIS_INVALID_PACKET: NTSTATUS = NTSTATUS(-1071448049i32);
pub const STATUS_NDIS_INVALID_PORT: NTSTATUS = NTSTATUS(-1071448019i32);
pub const STATUS_NDIS_INVALID_PORT_STATE: NTSTATUS = NTSTATUS(-1071448018i32);
pub const STATUS_NDIS_LOW_POWER_STATE: NTSTATUS = NTSTATUS(-1071448017i32);
pub const STATUS_NDIS_MEDIA_DISCONNECTED: NTSTATUS = NTSTATUS(-1071448033i32);
pub const STATUS_NDIS_MULTICAST_EXISTS: NTSTATUS = NTSTATUS(-1071448054i32);
pub const STATUS_NDIS_MULTICAST_FULL: NTSTATUS = NTSTATUS(-1071448055i32);
pub const STATUS_NDIS_MULTICAST_NOT_FOUND: NTSTATUS = NTSTATUS(-1071448053i32);
pub const STATUS_NDIS_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071447877i32);
pub const STATUS_NDIS_NO_QUEUES: NTSTATUS = NTSTATUS(-1071448015i32);
pub const STATUS_NDIS_OFFLOAD_CONNECTION_REJECTED: NTSTATUS = NTSTATUS(-1071443950i32);
pub const STATUS_NDIS_OFFLOAD_PATH_REJECTED: NTSTATUS = NTSTATUS(-1071443949i32);
pub const STATUS_NDIS_OFFLOAD_POLICY: NTSTATUS = NTSTATUS(-1071443953i32);
pub const STATUS_NDIS_OPEN_FAILED: NTSTATUS = NTSTATUS(-1071448057i32);
pub const STATUS_NDIS_PAUSED: NTSTATUS = NTSTATUS(-1071448022i32);
pub const STATUS_NDIS_PM_PROTOCOL_OFFLOAD_LIST_FULL: NTSTATUS = NTSTATUS(-1071439868i32);
pub const STATUS_NDIS_PM_WOL_PATTERN_LIST_FULL: NTSTATUS = NTSTATUS(-1071439869i32);
pub const STATUS_NDIS_REINIT_REQUIRED: NTSTATUS = NTSTATUS(-1071448016i32);
pub const STATUS_NDIS_REQUEST_ABORTED: NTSTATUS = NTSTATUS(-1071448052i32);
pub const STATUS_NDIS_RESET_IN_PROGRESS: NTSTATUS = NTSTATUS(-1071448051i32);
pub const STATUS_NDIS_RESOURCE_CONFLICT: NTSTATUS = NTSTATUS(-1071448034i32);
pub const STATUS_NDIS_UNSUPPORTED_MEDIA: NTSTATUS = NTSTATUS(-1071448039i32);
pub const STATUS_NDIS_UNSUPPORTED_REVISION: NTSTATUS = NTSTATUS(-1071448020i32);
pub const STATUS_ND_QUEUE_OVERFLOW: NTSTATUS = NTSTATUS(-1073700847i32);
pub const STATUS_NEEDS_REGISTRATION: NTSTATUS = NTSTATUS(-1073740663i32);
pub const STATUS_NEEDS_REMEDIATION: NTSTATUS = NTSTATUS(-1073740702i32);
pub const STATUS_NETLOGON_NOT_STARTED: NTSTATUS = NTSTATUS(-1073741422i32);
pub const STATUS_NETWORK_ACCESS_DENIED: NTSTATUS = NTSTATUS(-1073741622i32);
pub const STATUS_NETWORK_ACCESS_DENIED_EDP: NTSTATUS = NTSTATUS(-1073740658i32);
pub const STATUS_NETWORK_BUSY: NTSTATUS = NTSTATUS(-1073741633i32);
pub const STATUS_NETWORK_CREDENTIAL_CONFLICT: NTSTATUS = NTSTATUS(-1073741419i32);
pub const STATUS_NETWORK_NAME_DELETED: NTSTATUS = NTSTATUS(-1073741623i32);
pub const STATUS_NETWORK_OPEN_RESTRICTION: NTSTATUS = NTSTATUS(-1073741311i32);
pub const STATUS_NETWORK_SESSION_EXPIRED: NTSTATUS = NTSTATUS(-1073740964i32);
pub const STATUS_NETWORK_UNREACHABLE: NTSTATUS = NTSTATUS(-1073741252i32);
pub const STATUS_NET_WRITE_FAULT: NTSTATUS = NTSTATUS(-1073741614i32);
pub const STATUS_NOINTERFACE: NTSTATUS = NTSTATUS(-1073741127i32);
pub const STATUS_NOLOGON_INTERDOMAIN_TRUST_ACCOUNT: NTSTATUS = NTSTATUS(-1073741416i32);
pub const STATUS_NOLOGON_SERVER_TRUST_ACCOUNT: NTSTATUS = NTSTATUS(-1073741414i32);
pub const STATUS_NOLOGON_WORKSTATION_TRUST_ACCOUNT: NTSTATUS = NTSTATUS(-1073741415i32);
pub const STATUS_NONCONTINUABLE_EXCEPTION: NTSTATUS = NTSTATUS(-1073741787i32);
pub const STATUS_NONEXISTENT_EA_ENTRY: NTSTATUS = NTSTATUS(-1073741743i32);
pub const STATUS_NONEXISTENT_SECTOR: NTSTATUS = NTSTATUS(-1073741803i32);
pub const STATUS_NONE_MAPPED: NTSTATUS = NTSTATUS(-1073741709i32);
pub const STATUS_NOTHING_TO_TERMINATE: NTSTATUS = NTSTATUS(290i32);
pub const STATUS_NOTIFICATION_GUID_ALREADY_DEFINED: NTSTATUS = NTSTATUS(-1073741404i32);
pub const STATUS_NOTIFY_CLEANUP: NTSTATUS = NTSTATUS(267i32);
pub const STATUS_NOTIFY_ENUM_DIR: NTSTATUS = NTSTATUS(268i32);
pub const STATUS_NOT_ALLOWED_ON_SYSTEM_FILE: NTSTATUS = NTSTATUS(-1073741401i32);
pub const STATUS_NOT_ALL_ASSIGNED: NTSTATUS = NTSTATUS(262i32);
pub const STATUS_NOT_APPCONTAINER: NTSTATUS = NTSTATUS(-1073700352i32);
pub const STATUS_NOT_A_CLOUD_FILE: NTSTATUS = NTSTATUS(-1073688825i32);
pub const STATUS_NOT_A_CLOUD_SYNC_ROOT: NTSTATUS = NTSTATUS(-1073688802i32);
pub const STATUS_NOT_A_DAX_VOLUME: NTSTATUS = NTSTATUS(-1073740623i32);
pub const STATUS_NOT_A_DIRECTORY: NTSTATUS = NTSTATUS(-1073741565i32);
pub const STATUS_NOT_A_REPARSE_POINT: NTSTATUS = NTSTATUS(-1073741195i32);
pub const STATUS_NOT_A_TIERED_VOLUME: NTSTATUS = NTSTATUS(-1073740531i32);
pub const STATUS_NOT_CAPABLE: NTSTATUS = NTSTATUS(-1073740759i32);
pub const STATUS_NOT_CLIENT_SESSION: NTSTATUS = NTSTATUS(-1073741289i32);
pub const STATUS_NOT_COMMITTED: NTSTATUS = NTSTATUS(-1073741779i32);
pub const STATUS_NOT_DAX_MAPPABLE: NTSTATUS = NTSTATUS(-1073740622i32);
pub const STATUS_NOT_EXPORT_FORMAT: NTSTATUS = NTSTATUS(-1073741166i32);
pub const STATUS_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741275i32);
pub const STATUS_NOT_GUI_PROCESS: NTSTATUS = NTSTATUS(-1073740538i32);
pub const STATUS_NOT_IMPLEMENTED: NTSTATUS = NTSTATUS(-1073741822i32);
pub const STATUS_NOT_LOCKED: NTSTATUS = NTSTATUS(-1073741782i32);
pub const STATUS_NOT_LOGON_PROCESS: NTSTATUS = NTSTATUS(-1073741587i32);
pub const STATUS_NOT_MAPPED_DATA: NTSTATUS = NTSTATUS(-1073741688i32);
pub const STATUS_NOT_MAPPED_VIEW: NTSTATUS = NTSTATUS(-1073741799i32);
pub const STATUS_NOT_READ_FROM_COPY: NTSTATUS = NTSTATUS(-1073740694i32);
pub const STATUS_NOT_REDUNDANT_STORAGE: NTSTATUS = NTSTATUS(-1073740679i32);
pub const STATUS_NOT_REGISTRY_FILE: NTSTATUS = NTSTATUS(-1073741476i32);
pub const STATUS_NOT_SAFE_MODE_DRIVER: NTSTATUS = NTSTATUS(-1073740961i32);
pub const STATUS_NOT_SAME_DEVICE: NTSTATUS = NTSTATUS(-1073741612i32);
pub const STATUS_NOT_SAME_OBJECT: NTSTATUS = NTSTATUS(-1073741396i32);
pub const STATUS_NOT_SERVER_SESSION: NTSTATUS = NTSTATUS(-1073741290i32);
pub const STATUS_NOT_SNAPSHOT_VOLUME: NTSTATUS = NTSTATUS(-1072103353i32);
pub const STATUS_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073741637i32);
pub const STATUS_NOT_SUPPORTED_IN_APPCONTAINER: NTSTATUS = NTSTATUS(-1073700351i32);
pub const STATUS_NOT_SUPPORTED_ON_DAX: NTSTATUS = NTSTATUS(-1073740646i32);
pub const STATUS_NOT_SUPPORTED_ON_SBS: NTSTATUS = NTSTATUS(-1073741056i32);
pub const STATUS_NOT_SUPPORTED_WITH_AUDITING: NTSTATUS = NTSTATUS(-1073740595i32);
pub const STATUS_NOT_SUPPORTED_WITH_BTT: NTSTATUS = NTSTATUS(-1073740619i32);
pub const STATUS_NOT_SUPPORTED_WITH_BYPASSIO: NTSTATUS = NTSTATUS(-1073740601i32);
pub const STATUS_NOT_SUPPORTED_WITH_COMPRESSION: NTSTATUS = NTSTATUS(-1073740598i32);
pub const STATUS_NOT_SUPPORTED_WITH_DEDUPLICATION: NTSTATUS = NTSTATUS(-1073740596i32);
pub const STATUS_NOT_SUPPORTED_WITH_ENCRYPTION: NTSTATUS = NTSTATUS(-1073740599i32);
pub const STATUS_NOT_SUPPORTED_WITH_MONITORING: NTSTATUS = NTSTATUS(-1073740594i32);
pub const STATUS_NOT_SUPPORTED_WITH_REPLICATION: NTSTATUS = NTSTATUS(-1073740597i32);
pub const STATUS_NOT_SUPPORTED_WITH_SNAPSHOT: NTSTATUS = NTSTATUS(-1073740593i32);
pub const STATUS_NOT_SUPPORTED_WITH_VIRTUALIZATION: NTSTATUS = NTSTATUS(-1073740592i32);
pub const STATUS_NOT_TINY_STREAM: NTSTATUS = NTSTATUS(-1073741274i32);
pub const STATUS_NO_ACE_CONDITION: NTSTATUS = NTSTATUS(-2147483601i32);
pub const STATUS_NO_APPLICABLE_APP_LICENSES_FOUND: NTSTATUS = NTSTATUS(-1058406399i32);
pub const STATUS_NO_APPLICATION_PACKAGE: NTSTATUS = NTSTATUS(-1073741398i32);
pub const STATUS_NO_BROWSER_SERVERS_FOUND: NTSTATUS = NTSTATUS(-1073741284i32);
pub const STATUS_NO_BYPASSIO_DRIVER_SUPPORT: NTSTATUS = NTSTATUS(-1073740600i32);
pub const STATUS_NO_CALLBACK_ACTIVE: NTSTATUS = NTSTATUS(-1073741224i32);
pub const STATUS_NO_DATA_DETECTED: NTSTATUS = NTSTATUS(-2147483614i32);
pub const STATUS_NO_EAS_ON_FILE: NTSTATUS = NTSTATUS(-1073741742i32);
pub const STATUS_NO_EFS: NTSTATUS = NTSTATUS(-1073741170i32);
pub const STATUS_NO_EVENT_PAIR: NTSTATUS = NTSTATUS(-1073741490i32);
pub const STATUS_NO_GUID_TRANSLATION: NTSTATUS = NTSTATUS(-1073741556i32);
pub const STATUS_NO_IMPERSONATION_TOKEN: NTSTATUS = NTSTATUS(-1073741732i32);
pub const STATUS_NO_INHERITANCE: NTSTATUS = NTSTATUS(-2147483637i32);
pub const STATUS_NO_IP_ADDRESSES: NTSTATUS = NTSTATUS(-1073741071i32);
pub const STATUS_NO_KERB_KEY: NTSTATUS = NTSTATUS(-1073741022i32);
pub const STATUS_NO_KEY: NTSTATUS = NTSTATUS(-1073739508i32);
pub const STATUS_NO_LDT: NTSTATUS = NTSTATUS(-1073741545i32);
pub const STATUS_NO_LINK_TRACKING_IN_TRANSACTION: NTSTATUS = NTSTATUS(-1072103335i32);
pub const STATUS_NO_LOGON_SERVERS: NTSTATUS = NTSTATUS(-1073741730i32);
pub const STATUS_NO_LOG_SPACE: NTSTATUS = NTSTATUS(-1073741443i32);
pub const STATUS_NO_MATCH: NTSTATUS = NTSTATUS(-1073741198i32);
pub const STATUS_NO_MEDIA: NTSTATUS = NTSTATUS(-1073741448i32);
pub const STATUS_NO_MEDIA_IN_DEVICE: NTSTATUS = NTSTATUS(-1073741805i32);
pub const STATUS_NO_MEMORY: NTSTATUS = NTSTATUS(-1073741801i32);
pub const STATUS_NO_MORE_EAS: NTSTATUS = NTSTATUS(-2147483630i32);
pub const STATUS_NO_MORE_ENTRIES: NTSTATUS = NTSTATUS(-2147483622i32);
pub const STATUS_NO_MORE_FILES: NTSTATUS = NTSTATUS(-2147483642i32);
pub const STATUS_NO_MORE_MATCHES: NTSTATUS = NTSTATUS(-1073741197i32);
pub const STATUS_NO_PAGEFILE: NTSTATUS = NTSTATUS(-1073741497i32);
pub const STATUS_NO_PA_DATA: NTSTATUS = NTSTATUS(-1073741064i32);
pub const STATUS_NO_PHYSICALLY_ALIGNED_FREE_SPACE_FOUND: NTSTATUS = NTSTATUS(-1073740635i32);
pub const STATUS_NO_QUOTAS_FOR_ACCOUNT: NTSTATUS = NTSTATUS(269i32);
pub const STATUS_NO_RANGES_PROCESSED: NTSTATUS = NTSTATUS(-1073740704i32);
pub const STATUS_NO_RECOVERY_POLICY: NTSTATUS = NTSTATUS(-1073741171i32);
pub const STATUS_NO_S4U_PROT_SUPPORT: NTSTATUS = NTSTATUS(-1073740790i32);
pub const STATUS_NO_SAVEPOINT_WITH_OPEN_FILES: NTSTATUS = NTSTATUS(-1072103352i32);
pub const STATUS_NO_SECRETS: NTSTATUS = NTSTATUS(-1073740943i32);
pub const STATUS_NO_SECURITY_CONTEXT: NTSTATUS = NTSTATUS(-1073740755i32);
pub const STATUS_NO_SECURITY_ON_OBJECT: NTSTATUS = NTSTATUS(-1073741609i32);
pub const STATUS_NO_SPOOL_SPACE: NTSTATUS = NTSTATUS(-1073741625i32);
pub const STATUS_NO_SUCH_ALIAS: NTSTATUS = NTSTATUS(-1073741487i32);
pub const STATUS_NO_SUCH_DEVICE: NTSTATUS = NTSTATUS(-1073741810i32);
pub const STATUS_NO_SUCH_DOMAIN: NTSTATUS = NTSTATUS(-1073741601i32);
pub const STATUS_NO_SUCH_FILE: NTSTATUS = NTSTATUS(-1073741809i32);
pub const STATUS_NO_SUCH_GROUP: NTSTATUS = NTSTATUS(-1073741722i32);
pub const STATUS_NO_SUCH_MEMBER: NTSTATUS = NTSTATUS(-1073741446i32);
pub const STATUS_NO_SUCH_PACKAGE: NTSTATUS = NTSTATUS(-1073741570i32);
pub const STATUS_NO_SUCH_PRIVILEGE: NTSTATUS = NTSTATUS(-1073741728i32);
pub const STATUS_NO_TGT_REPLY: NTSTATUS = NTSTATUS(-1073741073i32);
pub const STATUS_NO_TOKEN: NTSTATUS = NTSTATUS(-1073741700i32);
pub const STATUS_NO_TRACKING_SERVICE: NTSTATUS = NTSTATUS(-1073741153i32);
pub const STATUS_NO_TRUST_LSA_SECRET: NTSTATUS = NTSTATUS(-1073741430i32);
pub const STATUS_NO_TRUST_SAM_ACCOUNT: NTSTATUS = NTSTATUS(-1073741429i32);
pub const STATUS_NO_TXF_METADATA: NTSTATUS = NTSTATUS(-2145845207i32);
pub const STATUS_NO_UNICODE_TRANSLATION: NTSTATUS = NTSTATUS(-1073740009i32);
pub const STATUS_NO_USER_KEYS: NTSTATUS = NTSTATUS(-1073741168i32);
pub const STATUS_NO_USER_SESSION_KEY: NTSTATUS = NTSTATUS(-1073741310i32);
pub const STATUS_NO_WORK_DONE: NTSTATUS = NTSTATUS(-2147483598i32);
pub const STATUS_NO_YIELD_PERFORMED: NTSTATUS = NTSTATUS(1073741860i32);
pub const STATUS_NTLM_BLOCKED: NTSTATUS = NTSTATUS(-1073740776i32);
pub const STATUS_NT_CROSS_ENCRYPTION_REQUIRED: NTSTATUS = NTSTATUS(-1073741475i32);
pub const STATUS_NULL_LM_PASSWORD: NTSTATUS = NTSTATUS(1073741837i32);
pub const STATUS_OBJECTID_EXISTS: NTSTATUS = NTSTATUS(-1073741269i32);
pub const STATUS_OBJECTID_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741072i32);
pub const STATUS_OBJECT_IS_IMMUTABLE: NTSTATUS = NTSTATUS(-1073740610i32);
pub const STATUS_OBJECT_NAME_COLLISION: NTSTATUS = NTSTATUS(-1073741771i32);
pub const STATUS_OBJECT_NAME_EXISTS: NTSTATUS = NTSTATUS(1073741824i32);
pub const STATUS_OBJECT_NAME_INVALID: NTSTATUS = NTSTATUS(-1073741773i32);
pub const STATUS_OBJECT_NAME_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741772i32);
pub const STATUS_OBJECT_NOT_EXTERNALLY_BACKED: NTSTATUS = NTSTATUS(-1073740691i32);
pub const STATUS_OBJECT_NO_LONGER_EXISTS: NTSTATUS = NTSTATUS(-1072103391i32);
pub const STATUS_OBJECT_PATH_INVALID: NTSTATUS = NTSTATUS(-1073741767i32);
pub const STATUS_OBJECT_PATH_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741766i32);
pub const STATUS_OBJECT_PATH_SYNTAX_BAD: NTSTATUS = NTSTATUS(-1073741765i32);
pub const STATUS_OBJECT_TYPE_MISMATCH: NTSTATUS = NTSTATUS(-1073741788i32);
pub const STATUS_OFFLOAD_READ_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073700189i32);
pub const STATUS_OFFLOAD_READ_FLT_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073700191i32);
pub const STATUS_OFFLOAD_WRITE_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073700188i32);
pub const STATUS_OFFLOAD_WRITE_FLT_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073700190i32);
pub const STATUS_ONLY_IF_CONNECTED: NTSTATUS = NTSTATUS(-1073741108i32);
pub const STATUS_OPEN_FAILED: NTSTATUS = NTSTATUS(-1073741514i32);
pub const STATUS_OPERATION_IN_PROGRESS: NTSTATUS = NTSTATUS(-1073740682i32);
pub const STATUS_OPERATION_NOT_SUPPORTED_IN_TRANSACTION: NTSTATUS = NTSTATUS(-1072103334i32);
pub const STATUS_OPLOCK_BREAK_IN_PROGRESS: NTSTATUS = NTSTATUS(264i32);
pub const STATUS_OPLOCK_HANDLE_CLOSED: NTSTATUS = NTSTATUS(534i32);
pub const STATUS_OPLOCK_NOT_GRANTED: NTSTATUS = NTSTATUS(-1073741598i32);
pub const STATUS_OPLOCK_SWITCHED_TO_NEW_HANDLE: NTSTATUS = NTSTATUS(533i32);
pub const STATUS_ORDINAL_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741512i32);
pub const STATUS_ORPHAN_NAME_EXHAUSTED: NTSTATUS = NTSTATUS(-1073739762i32);
pub const STATUS_PACKAGE_NOT_AVAILABLE: NTSTATUS = NTSTATUS(-1073740649i32);
pub const STATUS_PACKAGE_UPDATING: NTSTATUS = NTSTATUS(-1073740695i32);
pub const STATUS_PAGEFILE_CREATE_FAILED: NTSTATUS = NTSTATUS(-1073741498i32);
pub const STATUS_PAGEFILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073740603i32);
pub const STATUS_PAGEFILE_QUOTA: NTSTATUS = NTSTATUS(-1073741817i32);
pub const STATUS_PAGEFILE_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(-1073741524i32);
pub const STATUS_PAGE_FAULT_COPY_ON_WRITE: NTSTATUS = NTSTATUS(274i32);
pub const STATUS_PAGE_FAULT_DEMAND_ZERO: NTSTATUS = NTSTATUS(273i32);
pub const STATUS_PAGE_FAULT_GUARD_PAGE: NTSTATUS = NTSTATUS(275i32);
pub const STATUS_PAGE_FAULT_PAGING_FILE: NTSTATUS = NTSTATUS(276i32);
pub const STATUS_PAGE_FAULT_TRANSITION: NTSTATUS = NTSTATUS(272i32);
pub const STATUS_PARAMETER_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(-1073740784i32);
pub const STATUS_PARITY_ERROR: NTSTATUS = NTSTATUS(-1073741781i32);
pub const STATUS_PARTIAL_COPY: NTSTATUS = NTSTATUS(-2147483635i32);
pub const STATUS_PARTITION_FAILURE: NTSTATUS = NTSTATUS(-1073741454i32);
pub const STATUS_PARTITION_TERMINATING: NTSTATUS = NTSTATUS(-1073740640i32);
pub const STATUS_PASSWORD_CHANGE_REQUIRED: NTSTATUS = NTSTATUS(-1073741044i32);
pub const STATUS_PASSWORD_RESTRICTION: NTSTATUS = NTSTATUS(-1073741716i32);
pub const STATUS_PATCH_CONFLICT: NTSTATUS = NTSTATUS(-1073740628i32);
pub const STATUS_PATCH_DEFERRED: NTSTATUS = NTSTATUS(1073741879i32);
pub const STATUS_PATH_NOT_COVERED: NTSTATUS = NTSTATUS(-1073741225i32);
pub const STATUS_PCP_ATTESTATION_CHALLENGE_NOT_SET: NTSTATUS = NTSTATUS(-1071046638i32);
pub const STATUS_PCP_AUTHENTICATION_FAILED: NTSTATUS = NTSTATUS(-1071046648i32);
pub const STATUS_PCP_AUTHENTICATION_IGNORED: NTSTATUS = NTSTATUS(-1071046647i32);
pub const STATUS_PCP_BUFFER_LENGTH_MISMATCH: NTSTATUS = NTSTATUS(-1071046626i32);
pub const STATUS_PCP_BUFFER_TOO_SMALL: NTSTATUS = NTSTATUS(-1071046650i32);
pub const STATUS_PCP_CLAIM_TYPE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071046628i32);
pub const STATUS_PCP_DEVICE_NOT_FOUND: NTSTATUS = NTSTATUS(-1071046643i32);
pub const STATUS_PCP_DEVICE_NOT_READY: NTSTATUS = NTSTATUS(-1071046655i32);
pub const STATUS_PCP_ERROR_MASK: NTSTATUS = NTSTATUS(-1071046656i32);
pub const STATUS_PCP_FLAG_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071046652i32);
pub const STATUS_PCP_IFX_RSA_KEY_CREATION_BLOCKED: NTSTATUS = NTSTATUS(-1071046625i32);
pub const STATUS_PCP_INTERNAL_ERROR: NTSTATUS = NTSTATUS(-1071046649i32);
pub const STATUS_PCP_INVALID_HANDLE: NTSTATUS = NTSTATUS(-1071046654i32);
pub const STATUS_PCP_INVALID_PARAMETER: NTSTATUS = NTSTATUS(-1071046653i32);
pub const STATUS_PCP_KEY_ALREADY_FINALIZED: NTSTATUS = NTSTATUS(-1071046636i32);
pub const STATUS_PCP_KEY_HANDLE_INVALIDATED: NTSTATUS = NTSTATUS(-1071046622i32);
pub const STATUS_PCP_KEY_NOT_AIK: NTSTATUS = NTSTATUS(-1071046631i32);
pub const STATUS_PCP_KEY_NOT_AUTHENTICATED: NTSTATUS = NTSTATUS(-1071046632i32);
pub const STATUS_PCP_KEY_NOT_FINALIZED: NTSTATUS = NTSTATUS(-1071046639i32);
pub const STATUS_PCP_KEY_NOT_LOADED: NTSTATUS = NTSTATUS(-1071046641i32);
pub const STATUS_PCP_KEY_NOT_SIGNING_KEY: NTSTATUS = NTSTATUS(-1071046630i32);
pub const STATUS_PCP_KEY_USAGE_POLICY_INVALID: NTSTATUS = NTSTATUS(-1071046634i32);
pub const STATUS_PCP_KEY_USAGE_POLICY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071046635i32);
pub const STATUS_PCP_LOCKED_OUT: NTSTATUS = NTSTATUS(-1071046629i32);
pub const STATUS_PCP_NOT_PCR_BOUND: NTSTATUS = NTSTATUS(-1071046637i32);
pub const STATUS_PCP_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071046651i32);
pub const STATUS_PCP_NO_KEY_CERTIFICATION: NTSTATUS = NTSTATUS(-1071046640i32);
pub const STATUS_PCP_POLICY_NOT_FOUND: NTSTATUS = NTSTATUS(-1071046646i32);
pub const STATUS_PCP_PROFILE_NOT_FOUND: NTSTATUS = NTSTATUS(-1071046645i32);
pub const STATUS_PCP_RAW_POLICY_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071046623i32);
pub const STATUS_PCP_SOFT_KEY_ERROR: NTSTATUS = NTSTATUS(-1071046633i32);
pub const STATUS_PCP_TICKET_MISSING: NTSTATUS = NTSTATUS(-1071046624i32);
pub const STATUS_PCP_TPM_VERSION_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1071046627i32);
pub const STATUS_PCP_UNSUPPORTED_PSS_SALT: NTSTATUS = NTSTATUS(1076437027i32);
pub const STATUS_PCP_VALIDATION_FAILED: NTSTATUS = NTSTATUS(-1071046644i32);
pub const STATUS_PCP_WRONG_PARENT: NTSTATUS = NTSTATUS(-1071046642i32);
pub const STATUS_PENDING: NTSTATUS = NTSTATUS(259i32);
pub const STATUS_PER_USER_TRUST_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(-1073740799i32);
pub const STATUS_PIPE_BROKEN: NTSTATUS = NTSTATUS(-1073741493i32);
pub const STATUS_PIPE_BUSY: NTSTATUS = NTSTATUS(-1073741650i32);
pub const STATUS_PIPE_CLOSING: NTSTATUS = NTSTATUS(-1073741647i32);
pub const STATUS_PIPE_CONNECTED: NTSTATUS = NTSTATUS(-1073741646i32);
pub const STATUS_PIPE_DISCONNECTED: NTSTATUS = NTSTATUS(-1073741648i32);
pub const STATUS_PIPE_EMPTY: NTSTATUS = NTSTATUS(-1073741607i32);
pub const STATUS_PIPE_LISTENING: NTSTATUS = NTSTATUS(-1073741645i32);
pub const STATUS_PIPE_NOT_AVAILABLE: NTSTATUS = NTSTATUS(-1073741652i32);
pub const STATUS_PKINIT_CLIENT_FAILURE: NTSTATUS = NTSTATUS(-1073740916i32);
pub const STATUS_PKINIT_FAILURE: NTSTATUS = NTSTATUS(-1073741024i32);
pub const STATUS_PKINIT_NAME_MISMATCH: NTSTATUS = NTSTATUS(-1073741063i32);
pub const STATUS_PKU2U_CERT_FAILURE: NTSTATUS = NTSTATUS(-1073740753i32);
pub const STATUS_PLATFORM_MANIFEST_BINARY_ID_NOT_FOUND: NTSTATUS = NTSTATUS(-1058340859i32);
pub const STATUS_PLATFORM_MANIFEST_CATALOG_NOT_AUTHORIZED: NTSTATUS = NTSTATUS(-1058340860i32);
pub const STATUS_PLATFORM_MANIFEST_FILE_NOT_AUTHORIZED: NTSTATUS = NTSTATUS(-1058340861i32);
pub const STATUS_PLATFORM_MANIFEST_INVALID: NTSTATUS = NTSTATUS(-1058340862i32);
pub const STATUS_PLATFORM_MANIFEST_NOT_ACTIVE: NTSTATUS = NTSTATUS(-1058340858i32);
pub const STATUS_PLATFORM_MANIFEST_NOT_AUTHORIZED: NTSTATUS = NTSTATUS(-1058340863i32);
pub const STATUS_PLATFORM_MANIFEST_NOT_SIGNED: NTSTATUS = NTSTATUS(-1058340857i32);
pub const STATUS_PLUGPLAY_NO_DEVICE: NTSTATUS = NTSTATUS(-1073741218i32);
pub const STATUS_PLUGPLAY_QUERY_VETOED: NTSTATUS = NTSTATUS(-2147483608i32);
pub const STATUS_PNP_BAD_MPS_TABLE: NTSTATUS = NTSTATUS(-1073479627i32);
pub const STATUS_PNP_DEVICE_CONFIGURATION_PENDING: NTSTATUS = NTSTATUS(-1073740651i32);
pub const STATUS_PNP_DRIVER_CONFIGURATION_INCOMPLETE: NTSTATUS = NTSTATUS(-1073740653i32);
pub const STATUS_PNP_DRIVER_CONFIGURATION_NOT_FOUND: NTSTATUS = NTSTATUS(-1073740654i32);
pub const STATUS_PNP_DRIVER_PACKAGE_NOT_FOUND: NTSTATUS = NTSTATUS(-1073740655i32);
pub const STATUS_PNP_FUNCTION_DRIVER_REQUIRED: NTSTATUS = NTSTATUS(-1073740652i32);
pub const STATUS_PNP_INVALID_ID: NTSTATUS = NTSTATUS(-1073479624i32);
pub const STATUS_PNP_IRQ_TRANSLATION_FAILED: NTSTATUS = NTSTATUS(-1073479625i32);
pub const STATUS_PNP_NO_COMPAT_DRIVERS: NTSTATUS = NTSTATUS(-1073740656i32);
pub const STATUS_PNP_REBOOT_REQUIRED: NTSTATUS = NTSTATUS(-1073741102i32);
pub const STATUS_PNP_RESTART_ENUMERATION: NTSTATUS = NTSTATUS(-1073741106i32);
pub const STATUS_PNP_TRANSLATION_FAILED: NTSTATUS = NTSTATUS(-1073479626i32);
pub const STATUS_POLICY_OBJECT_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741158i32);
pub const STATUS_POLICY_ONLY_IN_DS: NTSTATUS = NTSTATUS(-1073741157i32);
pub const STATUS_PORT_ALREADY_HAS_COMPLETION_LIST: NTSTATUS = NTSTATUS(-1073740006i32);
pub const STATUS_PORT_ALREADY_SET: NTSTATUS = NTSTATUS(-1073741752i32);
pub const STATUS_PORT_CLOSED: NTSTATUS = NTSTATUS(-1073740032i32);
pub const STATUS_PORT_CONNECTION_REFUSED: NTSTATUS = NTSTATUS(-1073741759i32);
pub const STATUS_PORT_DISCONNECTED: NTSTATUS = NTSTATUS(-1073741769i32);
pub const STATUS_PORT_DO_NOT_DISTURB: NTSTATUS = NTSTATUS(-1073741770i32);
pub const STATUS_PORT_MESSAGE_TOO_LONG: NTSTATUS = NTSTATUS(-1073741777i32);
pub const STATUS_PORT_NOT_SET: NTSTATUS = NTSTATUS(-1073740973i32);
pub const STATUS_PORT_UNREACHABLE: NTSTATUS = NTSTATUS(-1073741249i32);
pub const STATUS_POSSIBLE_DEADLOCK: NTSTATUS = NTSTATUS(-1073741420i32);
pub const STATUS_POWER_STATE_INVALID: NTSTATUS = NTSTATUS(-1073741101i32);
pub const STATUS_PREDEFINED_HANDLE: NTSTATUS = NTSTATUS(1073741846i32);
pub const STATUS_PRENT4_MACHINE_ACCOUNT: NTSTATUS = NTSTATUS(-1073740969i32);
pub const STATUS_PRIMARY_TRANSPORT_CONNECT_FAILED: NTSTATUS = NTSTATUS(270i32);
pub const STATUS_PRINT_CANCELLED: NTSTATUS = NTSTATUS(-1073741624i32);
pub const STATUS_PRINT_QUEUE_FULL: NTSTATUS = NTSTATUS(-1073741626i32);
pub const STATUS_PRIVILEGED_INSTRUCTION: NTSTATUS = NTSTATUS(-1073741674i32);
pub const STATUS_PRIVILEGE_NOT_HELD: NTSTATUS = NTSTATUS(-1073741727i32);
pub const STATUS_PROACTIVE_SCAN_IN_PROGRESS: NTSTATUS = NTSTATUS(-1073739761i32);
pub const STATUS_PROCEDURE_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741702i32);
pub const STATUS_PROCESS_CLONED: NTSTATUS = NTSTATUS(297i32);
pub const STATUS_PROCESS_IN_JOB: NTSTATUS = NTSTATUS(292i32);
pub const STATUS_PROCESS_IS_PROTECTED: NTSTATUS = NTSTATUS(-1073740014i32);
pub const STATUS_PROCESS_IS_TERMINATING: NTSTATUS = NTSTATUS(-1073741558i32);
pub const STATUS_PROCESS_NOT_IN_JOB: NTSTATUS = NTSTATUS(291i32);
pub const STATUS_PROFILING_AT_LIMIT: NTSTATUS = NTSTATUS(-1073741613i32);
pub const STATUS_PROFILING_NOT_STARTED: NTSTATUS = NTSTATUS(-1073741641i32);
pub const STATUS_PROFILING_NOT_STOPPED: NTSTATUS = NTSTATUS(-1073741640i32);
pub const STATUS_PROPSET_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741264i32);
pub const STATUS_PROTOCOL_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073700845i32);
pub const STATUS_PROTOCOL_UNREACHABLE: NTSTATUS = NTSTATUS(-1073741250i32);
pub const STATUS_PTE_CHANGED: NTSTATUS = NTSTATUS(-1073740748i32);
pub const STATUS_PURGE_FAILED: NTSTATUS = NTSTATUS(-1073740747i32);
pub const STATUS_PWD_HISTORY_CONFLICT: NTSTATUS = NTSTATUS(-1073741220i32);
pub const STATUS_PWD_TOO_LONG: NTSTATUS = NTSTATUS(-1073741190i32);
pub const STATUS_PWD_TOO_RECENT: NTSTATUS = NTSTATUS(-1073741221i32);
pub const STATUS_PWD_TOO_SHORT: NTSTATUS = NTSTATUS(-1073741222i32);
pub const STATUS_QUERY_STORAGE_ERROR: NTSTATUS = NTSTATUS(-2143682559i32);
pub const STATUS_QUIC_ALPN_NEG_FAILURE: NTSTATUS = NTSTATUS(-1071382521i32);
pub const STATUS_QUIC_CONNECTION_IDLE: NTSTATUS = NTSTATUS(-1071382523i32);
pub const STATUS_QUIC_CONNECTION_TIMEOUT: NTSTATUS = NTSTATUS(-1071382522i32);
pub const STATUS_QUIC_HANDSHAKE_FAILURE: NTSTATUS = NTSTATUS(-1071382528i32);
pub const STATUS_QUIC_INTERNAL_ERROR: NTSTATUS = NTSTATUS(-1071382525i32);
pub const STATUS_QUIC_PROTOCOL_VIOLATION: NTSTATUS = NTSTATUS(-1071382524i32);
pub const STATUS_QUIC_USER_CANCELED: NTSTATUS = NTSTATUS(-1071382526i32);
pub const STATUS_QUIC_VER_NEG_FAILURE: NTSTATUS = NTSTATUS(-1071382527i32);
pub const STATUS_QUOTA_ACTIVITY: NTSTATUS = NTSTATUS(-1073740662i32);
pub const STATUS_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(-1073741756i32);
pub const STATUS_QUOTA_LIST_INCONSISTENT: NTSTATUS = NTSTATUS(-1073741210i32);
pub const STATUS_QUOTA_NOT_ENABLED: NTSTATUS = NTSTATUS(-1073741399i32);
pub const STATUS_RANGE_LIST_CONFLICT: NTSTATUS = NTSTATUS(-1073741182i32);
pub const STATUS_RANGE_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741172i32);
pub const STATUS_RANGE_NOT_LOCKED: NTSTATUS = NTSTATUS(-1073741698i32);
pub const STATUS_RDBSS_CONTINUE_OPERATION: NTSTATUS = NTSTATUS(-1069481982i32);
pub const STATUS_RDBSS_POST_OPERATION: NTSTATUS = NTSTATUS(-1069481981i32);
pub const STATUS_RDBSS_RESTART_OPERATION: NTSTATUS = NTSTATUS(-1069481983i32);
pub const STATUS_RDBSS_RETRY_LOOKUP: NTSTATUS = NTSTATUS(-1069481980i32);
pub const STATUS_RDP_PROTOCOL_ERROR: NTSTATUS = NTSTATUS(-1073086414i32);
pub const STATUS_RECEIVE_EXPEDITED: NTSTATUS = NTSTATUS(1073741840i32);
pub const STATUS_RECEIVE_PARTIAL: NTSTATUS = NTSTATUS(1073741839i32);
pub const STATUS_RECEIVE_PARTIAL_EXPEDITED: NTSTATUS = NTSTATUS(1073741841i32);
pub const STATUS_RECOVERABLE_BUGCHECK: NTSTATUS = NTSTATUS(-2147483596i32);
pub const STATUS_RECOVERY_FAILURE: NTSTATUS = NTSTATUS(-1073741273i32);
pub const STATUS_RECOVERY_NOT_NEEDED: NTSTATUS = NTSTATUS(1075380276i32);
pub const STATUS_RECURSIVE_DISPATCH: NTSTATUS = NTSTATUS(-1073740028i32);
pub const STATUS_REDIRECTOR_HAS_OPEN_HANDLES: NTSTATUS = NTSTATUS(-2147483613i32);
pub const STATUS_REDIRECTOR_NOT_STARTED: NTSTATUS = NTSTATUS(-1073741573i32);
pub const STATUS_REDIRECTOR_PAUSED: NTSTATUS = NTSTATUS(-1073741615i32);
pub const STATUS_REDIRECTOR_STARTED: NTSTATUS = NTSTATUS(-1073741572i32);
pub const STATUS_REGISTRY_CORRUPT: NTSTATUS = NTSTATUS(-1073741492i32);
pub const STATUS_REGISTRY_HIVE_RECOVERED: NTSTATUS = NTSTATUS(-2147483606i32);
pub const STATUS_REGISTRY_IO_FAILED: NTSTATUS = NTSTATUS(-1073741491i32);
pub const STATUS_REGISTRY_QUOTA_LIMIT: NTSTATUS = NTSTATUS(-1073741226i32);
pub const STATUS_REGISTRY_RECOVERED: NTSTATUS = NTSTATUS(1073741833i32);
pub const STATUS_REG_NAT_CONSUMPTION: NTSTATUS = NTSTATUS(-1073741111i32);
pub const STATUS_REINITIALIZATION_NEEDED: NTSTATUS = NTSTATUS(-1073741177i32);
pub const STATUS_REMOTE_DISCONNECT: NTSTATUS = NTSTATUS(-1073741508i32);
pub const STATUS_REMOTE_FILE_VERSION_MISMATCH: NTSTATUS = NTSTATUS(-1072103412i32);
pub const STATUS_REMOTE_NOT_LISTENING: NTSTATUS = NTSTATUS(-1073741636i32);
pub const STATUS_REMOTE_RESOURCES: NTSTATUS = NTSTATUS(-1073741507i32);
pub const STATUS_REMOTE_SESSION_LIMIT: NTSTATUS = NTSTATUS(-1073741418i32);
pub const STATUS_REMOTE_STORAGE_MEDIA_ERROR: NTSTATUS = NTSTATUS(-1073741154i32);
pub const STATUS_REMOTE_STORAGE_NOT_ACTIVE: NTSTATUS = NTSTATUS(-1073741155i32);
pub const STATUS_REPAIR_NEEDED: NTSTATUS = NTSTATUS(-1073741400i32);
pub const STATUS_REPARSE: NTSTATUS = NTSTATUS(260i32);
pub const STATUS_REPARSE_ATTRIBUTE_CONFLICT: NTSTATUS = NTSTATUS(-1073741134i32);
pub const STATUS_REPARSE_GLOBAL: NTSTATUS = NTSTATUS(872i32);
pub const STATUS_REPARSE_OBJECT: NTSTATUS = NTSTATUS(280i32);
pub const STATUS_REPARSE_POINT_ENCOUNTERED: NTSTATUS = NTSTATUS(-1073740533i32);
pub const STATUS_REPARSE_POINT_NOT_RESOLVED: NTSTATUS = NTSTATUS(-1073741184i32);
pub const STATUS_REPLY_MESSAGE_MISMATCH: NTSTATUS = NTSTATUS(-1073741281i32);
pub const STATUS_REQUEST_ABORTED: NTSTATUS = NTSTATUS(-1073741248i32);
pub const STATUS_REQUEST_CANCELED: NTSTATUS = NTSTATUS(-1073740029i32);
pub const STATUS_REQUEST_NOT_ACCEPTED: NTSTATUS = NTSTATUS(-1073741616i32);
pub const STATUS_REQUEST_OUT_OF_SEQUENCE: NTSTATUS = NTSTATUS(-1073740758i32);
pub const STATUS_REQUEST_PAUSED: NTSTATUS = NTSTATUS(-1073740711i32);
pub const STATUS_RESIDENT_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073740678i32);
pub const STATUS_RESOURCEMANAGER_NOT_FOUND: NTSTATUS = NTSTATUS(-1072103345i32);
pub const STATUS_RESOURCEMANAGER_READ_ONLY: NTSTATUS = NTSTATUS(514i32);
pub const STATUS_RESOURCE_DATA_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741687i32);
pub const STATUS_RESOURCE_ENUM_USER_STOP: NTSTATUS = NTSTATUS(-1073020921i32);
pub const STATUS_RESOURCE_IN_USE: NTSTATUS = NTSTATUS(-1073740024i32);
pub const STATUS_RESOURCE_LANG_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741308i32);
pub const STATUS_RESOURCE_NAME_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741685i32);
pub const STATUS_RESOURCE_NOT_OWNED: NTSTATUS = NTSTATUS(-1073741212i32);
pub const STATUS_RESOURCE_REQUIREMENTS_CHANGED: NTSTATUS = NTSTATUS(281i32);
pub const STATUS_RESOURCE_TYPE_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741686i32);
pub const STATUS_RESTART_BOOT_APPLICATION: NTSTATUS = NTSTATUS(-1073740717i32);
pub const STATUS_RESUME_HIBERNATION: NTSTATUS = NTSTATUS(1073741867i32);
pub const STATUS_RETRY: NTSTATUS = NTSTATUS(-1073741267i32);
pub const STATUS_RETURN_ADDRESS_HIJACK_ATTEMPT: NTSTATUS = NTSTATUS(-2147483597i32);
pub const STATUS_REVISION_MISMATCH: NTSTATUS = NTSTATUS(-1073741735i32);
pub const STATUS_REVOCATION_OFFLINE_C: NTSTATUS = NTSTATUS(-1073740917i32);
pub const STATUS_REVOCATION_OFFLINE_KDC: NTSTATUS = NTSTATUS(-1073740788i32);
pub const STATUS_RING_NEWLY_EMPTY: NTSTATUS = NTSTATUS(531i32);
pub const STATUS_RING_PREVIOUSLY_ABOVE_QUOTA: NTSTATUS = NTSTATUS(530i32);
pub const STATUS_RING_PREVIOUSLY_EMPTY: NTSTATUS = NTSTATUS(528i32);
pub const STATUS_RING_PREVIOUSLY_FULL: NTSTATUS = NTSTATUS(529i32);
pub const STATUS_RING_SIGNAL_OPPOSITE_ENDPOINT: NTSTATUS = NTSTATUS(532i32);
pub const STATUS_RKF_ACTIVE_KEY: NTSTATUS = NTSTATUS(-1069547514i32);
pub const STATUS_RKF_BLOB_FULL: NTSTATUS = NTSTATUS(-1069547517i32);
pub const STATUS_RKF_DUPLICATE_KEY: NTSTATUS = NTSTATUS(-1069547518i32);
pub const STATUS_RKF_FILE_BLOCKED: NTSTATUS = NTSTATUS(-1069547515i32);
pub const STATUS_RKF_KEY_NOT_FOUND: NTSTATUS = NTSTATUS(-1069547519i32);
pub const STATUS_RKF_STORE_FULL: NTSTATUS = NTSTATUS(-1069547516i32);
pub const STATUS_RM_ALREADY_STARTED: NTSTATUS = NTSTATUS(1075380277i32);
pub const STATUS_RM_CANNOT_BE_FROZEN_FOR_SNAPSHOT: NTSTATUS = NTSTATUS(-1072103331i32);
pub const STATUS_RM_DISCONNECTED: NTSTATUS = NTSTATUS(-1072103374i32);
pub const STATUS_RM_METADATA_CORRUPT: NTSTATUS = NTSTATUS(-1072103418i32);
pub const STATUS_RM_NOT_ACTIVE: NTSTATUS = NTSTATUS(-1072103419i32);
pub const STATUS_ROLLBACK_TIMER_EXPIRED: NTSTATUS = NTSTATUS(-1072103364i32);
pub const STATUS_RTPM_CONTEXT_COMPLETE: NTSTATUS = NTSTATUS(2699265i32);
pub const STATUS_RTPM_CONTEXT_CONTINUE: NTSTATUS = NTSTATUS(2699264i32);
pub const STATUS_RTPM_INVALID_CONTEXT: NTSTATUS = NTSTATUS(-1071042556i32);
pub const STATUS_RTPM_NO_RESULT: NTSTATUS = NTSTATUS(-1071042558i32);
pub const STATUS_RTPM_PCR_READ_INCOMPLETE: NTSTATUS = NTSTATUS(-1071042557i32);
pub const STATUS_RTPM_UNSUPPORTED_CMD: NTSTATUS = NTSTATUS(-1071042555i32);
pub const STATUS_RUNLEVEL_SWITCH_AGENT_TIMEOUT: NTSTATUS = NTSTATUS(-1073700539i32);
pub const STATUS_RUNLEVEL_SWITCH_IN_PROGRESS: NTSTATUS = NTSTATUS(-1073700538i32);
pub const STATUS_RUNLEVEL_SWITCH_TIMEOUT: NTSTATUS = NTSTATUS(-1073700541i32);
pub const STATUS_RWRAW_ENCRYPTED_FILE_NOT_ENCRYPTED: NTSTATUS = NTSTATUS(-1073740633i32);
pub const STATUS_RWRAW_ENCRYPTED_INVALID_EDATAINFO_FILEOFFSET: NTSTATUS = NTSTATUS(-1073740632i32);
pub const STATUS_RWRAW_ENCRYPTED_INVALID_EDATAINFO_FILERANGE: NTSTATUS = NTSTATUS(-1073740631i32);
pub const STATUS_RWRAW_ENCRYPTED_INVALID_EDATAINFO_PARAMETER: NTSTATUS = NTSTATUS(-1073740630i32);
pub const STATUS_RXACT_COMMITTED: NTSTATUS = NTSTATUS(266i32);
pub const STATUS_RXACT_COMMIT_FAILURE: NTSTATUS = NTSTATUS(-1073741539i32);
pub const STATUS_RXACT_COMMIT_NECESSARY: NTSTATUS = NTSTATUS(-2147483624i32);
pub const STATUS_RXACT_INVALID_STATE: NTSTATUS = NTSTATUS(-1073741540i32);
pub const STATUS_RXACT_STATE_CREATED: NTSTATUS = NTSTATUS(1073741828i32);
pub const STATUS_SAM_INIT_FAILURE: NTSTATUS = NTSTATUS(-1073741085i32);
pub const STATUS_SAM_NEED_BOOTKEY_FLOPPY: NTSTATUS = NTSTATUS(-1073741088i32);
pub const STATUS_SAM_NEED_BOOTKEY_PASSWORD: NTSTATUS = NTSTATUS(-1073741089i32);
pub const STATUS_SCRUB_DATA_DISABLED: NTSTATUS = NTSTATUS(-1073740680i32);
pub const STATUS_SECCORE_INVALID_COMMAND: NTSTATUS = NTSTATUS(-1058537472i32);
pub const STATUS_SECONDARY_IC_PROVIDER_NOT_REGISTERED: NTSTATUS = NTSTATUS(-1073700575i32);
pub const STATUS_SECRET_TOO_LONG: NTSTATUS = NTSTATUS(-1073741481i32);
pub const STATUS_SECTION_DIRECT_MAP_ONLY: NTSTATUS = NTSTATUS(-1073739503i32);
pub const STATUS_SECTION_NOT_EXTENDED: NTSTATUS = NTSTATUS(-1073741689i32);
pub const STATUS_SECTION_NOT_IMAGE: NTSTATUS = NTSTATUS(-1073741751i32);
pub const STATUS_SECTION_PROTECTION: NTSTATUS = NTSTATUS(-1073741746i32);
pub const STATUS_SECTION_TOO_BIG: NTSTATUS = NTSTATUS(-1073741760i32);
pub const STATUS_SECUREBOOT_FILE_REPLACED: NTSTATUS = NTSTATUS(-1069350905i32);
pub const STATUS_SECUREBOOT_INVALID_POLICY: NTSTATUS = NTSTATUS(-1069350909i32);
pub const STATUS_SECUREBOOT_NOT_BASE_POLICY: NTSTATUS = NTSTATUS(-1069350897i32);
pub const STATUS_SECUREBOOT_NOT_ENABLED: NTSTATUS = NTSTATUS(-2143092730i32);
pub const STATUS_SECUREBOOT_NOT_SUPPLEMENTAL_POLICY: NTSTATUS = NTSTATUS(-1069350896i32);
pub const STATUS_SECUREBOOT_PLATFORM_ID_MISMATCH: NTSTATUS = NTSTATUS(-1069350901i32);
pub const STATUS_SECUREBOOT_POLICY_MISSING_ANTIROLLBACKVERSION: NTSTATUS = NTSTATUS(-1069350902i32);
pub const STATUS_SECUREBOOT_POLICY_NOT_AUTHORIZED: NTSTATUS = NTSTATUS(-1069350904i32);
pub const STATUS_SECUREBOOT_POLICY_NOT_SIGNED: NTSTATUS = NTSTATUS(-1069350907i32);
pub const STATUS_SECUREBOOT_POLICY_PUBLISHER_NOT_FOUND: NTSTATUS = NTSTATUS(-1069350908i32);
pub const STATUS_SECUREBOOT_POLICY_ROLLBACK_DETECTED: NTSTATUS = NTSTATUS(-1069350900i32);
pub const STATUS_SECUREBOOT_POLICY_UNKNOWN: NTSTATUS = NTSTATUS(-1069350903i32);
pub const STATUS_SECUREBOOT_POLICY_UPGRADE_MISMATCH: NTSTATUS = NTSTATUS(-1069350899i32);
pub const STATUS_SECUREBOOT_POLICY_VIOLATION: NTSTATUS = NTSTATUS(-1069350910i32);
pub const STATUS_SECUREBOOT_REQUIRED_POLICY_FILE_MISSING: NTSTATUS = NTSTATUS(-1069350898i32);
pub const STATUS_SECUREBOOT_ROLLBACK_DETECTED: NTSTATUS = NTSTATUS(-1069350911i32);
pub const STATUS_SECURITY_STREAM_IS_INCONSISTENT: NTSTATUS = NTSTATUS(-1073741408i32);
pub const STATUS_SEGMENT_NOTIFICATION: NTSTATUS = NTSTATUS(1073741829i32);
pub const STATUS_SEMAPHORE_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(-1073741753i32);
pub const STATUS_SERIAL_COUNTER_TIMEOUT: NTSTATUS = NTSTATUS(1073741836i32);
pub const STATUS_SERIAL_MORE_WRITES: NTSTATUS = NTSTATUS(1073741832i32);
pub const STATUS_SERIAL_NO_DEVICE_INITED: NTSTATUS = NTSTATUS(-1073741488i32);
pub const STATUS_SERVER_DISABLED: NTSTATUS = NTSTATUS(-1073741696i32);
pub const STATUS_SERVER_HAS_OPEN_HANDLES: NTSTATUS = NTSTATUS(-2147483612i32);
pub const STATUS_SERVER_NOT_DISABLED: NTSTATUS = NTSTATUS(-1073741695i32);
pub const STATUS_SERVER_SHUTDOWN_IN_PROGRESS: NTSTATUS = NTSTATUS(-1073741057i32);
pub const STATUS_SERVER_SID_MISMATCH: NTSTATUS = NTSTATUS(-1073741152i32);
pub const STATUS_SERVER_TRANSPORT_CONFLICT: NTSTATUS = NTSTATUS(-1073741388i32);
pub const STATUS_SERVER_UNAVAILABLE: NTSTATUS = NTSTATUS(-1073740698i32);
pub const STATUS_SERVICES_FAILED_AUTOSTART: NTSTATUS = NTSTATUS(1073783108i32);
pub const STATUS_SERVICE_NOTIFICATION: NTSTATUS = NTSTATUS(1073741848i32);
pub const STATUS_SESSION_KEY_TOO_SHORT: NTSTATUS = NTSTATUS(-1073740521i32);
pub const STATUS_SETMARK_DETECTED: NTSTATUS = NTSTATUS(-2147483615i32);
pub const STATUS_SET_CONTEXT_DENIED: NTSTATUS = NTSTATUS(-1073740278i32);
pub const STATUS_SEVERITY_COERROR: u32 = 2u32;
pub const STATUS_SEVERITY_COFAIL: u32 = 3u32;
pub const STATUS_SHARED_IRQ_BUSY: NTSTATUS = NTSTATUS(-1073741460i32);
pub const STATUS_SHARED_POLICY: NTSTATUS = NTSTATUS(-1073741159i32);
pub const STATUS_SHARE_UNAVAILABLE: NTSTATUS = NTSTATUS(-1073740672i32);
pub const STATUS_SHARING_PAUSED: NTSTATUS = NTSTATUS(-1073741617i32);
pub const STATUS_SHARING_VIOLATION: NTSTATUS = NTSTATUS(-1073741757i32);
pub const STATUS_SHORT_NAMES_NOT_ENABLED_ON_VOLUME: NTSTATUS = NTSTATUS(-1073741409i32);
pub const STATUS_SHUTDOWN_IN_PROGRESS: NTSTATUS = NTSTATUS(-1073741058i32);
pub const STATUS_SINGLE_STEP: NTSTATUS = NTSTATUS(-2147483644i32);
pub const STATUS_SMARTCARD_CARD_BLOCKED: NTSTATUS = NTSTATUS(-1073740927i32);
pub const STATUS_SMARTCARD_CARD_NOT_AUTHENTICATED: NTSTATUS = NTSTATUS(-1073740926i32);
pub const STATUS_SMARTCARD_CERT_EXPIRED: NTSTATUS = NTSTATUS(-1073740915i32);
pub const STATUS_SMARTCARD_CERT_REVOKED: NTSTATUS = NTSTATUS(-1073740919i32);
pub const STATUS_SMARTCARD_IO_ERROR: NTSTATUS = NTSTATUS(-1073740921i32);
pub const STATUS_SMARTCARD_LOGON_REQUIRED: NTSTATUS = NTSTATUS(-1073741062i32);
pub const STATUS_SMARTCARD_NO_CARD: NTSTATUS = NTSTATUS(-1073740925i32);
pub const STATUS_SMARTCARD_NO_CERTIFICATE: NTSTATUS = NTSTATUS(-1073740923i32);
pub const STATUS_SMARTCARD_NO_KEYSET: NTSTATUS = NTSTATUS(-1073740922i32);
pub const STATUS_SMARTCARD_NO_KEY_CONTAINER: NTSTATUS = NTSTATUS(-1073740924i32);
pub const STATUS_SMARTCARD_SILENT_CONTEXT: NTSTATUS = NTSTATUS(-1073740913i32);
pub const STATUS_SMARTCARD_SUBSYSTEM_FAILURE: NTSTATUS = NTSTATUS(-1073741023i32);
pub const STATUS_SMARTCARD_WRONG_PIN: NTSTATUS = NTSTATUS(-1073740928i32);
pub const STATUS_SMB1_NOT_AVAILABLE: NTSTATUS = NTSTATUS(-1073740525i32);
pub const STATUS_SMB_BAD_CLUSTER_DIALECT: NTSTATUS = NTSTATUS(-1067646975i32);
pub const STATUS_SMB_GUEST_LOGON_BLOCKED: NTSTATUS = NTSTATUS(-1067646974i32);
pub const STATUS_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP: NTSTATUS = NTSTATUS(-1067646976i32);
pub const STATUS_SMB_NO_SIGNING_ALGORITHM_OVERLAP: NTSTATUS = NTSTATUS(-1067646973i32);
pub const STATUS_SMI_PRIMITIVE_INSTALLER_FAILED: NTSTATUS = NTSTATUS(-1072365531i32);
pub const STATUS_SMR_GARBAGE_COLLECTION_REQUIRED: NTSTATUS = NTSTATUS(-1073740524i32);
pub const STATUS_SOME_NOT_MAPPED: NTSTATUS = NTSTATUS(263i32);
pub const STATUS_SOURCE_ELEMENT_EMPTY: NTSTATUS = NTSTATUS(-1073741181i32);
pub const STATUS_SPACES_ALLOCATION_SIZE_INVALID: NTSTATUS = NTSTATUS(-1058602994i32);
pub const STATUS_SPACES_CACHE_FULL: NTSTATUS = NTSTATUS(-1058602970i32);
pub const STATUS_SPACES_COMPLETE: NTSTATUS = NTSTATUS(15138818i32);
pub const STATUS_SPACES_CORRUPT_METADATA: NTSTATUS = NTSTATUS(-1058602986i32);
pub const STATUS_SPACES_DRIVE_LOST_DATA: NTSTATUS = NTSTATUS(-1058602979i32);
pub const STATUS_SPACES_DRIVE_NOT_READY: NTSTATUS = NTSTATUS(-1058602981i32);
pub const STATUS_SPACES_DRIVE_OPERATIONAL_STATE_INVALID: NTSTATUS = NTSTATUS(-1058602990i32);
pub const STATUS_SPACES_DRIVE_REDUNDANCY_INVALID: NTSTATUS = NTSTATUS(-1058603002i32);
pub const STATUS_SPACES_DRIVE_SECTOR_SIZE_INVALID: NTSTATUS = NTSTATUS(-1058603004i32);
pub const STATUS_SPACES_DRIVE_SPLIT: NTSTATUS = NTSTATUS(-1058602980i32);
pub const STATUS_SPACES_DRT_FULL: NTSTATUS = NTSTATUS(-1058602985i32);
pub const STATUS_SPACES_ENCLOSURE_AWARE_INVALID: NTSTATUS = NTSTATUS(-1058602993i32);
pub const STATUS_SPACES_ENTRY_INCOMPLETE: NTSTATUS = NTSTATUS(-1058602978i32);
pub const STATUS_SPACES_ENTRY_INVALID: NTSTATUS = NTSTATUS(-1058602977i32);
pub const STATUS_SPACES_EXTENDED_ERROR: NTSTATUS = NTSTATUS(-1058602996i32);
pub const STATUS_SPACES_FAULT_DOMAIN_TYPE_INVALID: NTSTATUS = NTSTATUS(-1058603007i32);
pub const STATUS_SPACES_FLUSH_METADATA: NTSTATUS = NTSTATUS(-1058602971i32);
pub const STATUS_SPACES_INCONSISTENCY: NTSTATUS = NTSTATUS(-1058602984i32);
pub const STATUS_SPACES_INTERLEAVE_LENGTH_INVALID: NTSTATUS = NTSTATUS(-1058602999i32);
pub const STATUS_SPACES_LOG_NOT_READY: NTSTATUS = NTSTATUS(-1058602983i32);
pub const STATUS_SPACES_MAP_REQUIRED: NTSTATUS = NTSTATUS(-1058602988i32);
pub const STATUS_SPACES_MARK_DIRTY: NTSTATUS = NTSTATUS(-1058602976i32);
pub const STATUS_SPACES_NOT_ENOUGH_DRIVES: NTSTATUS = NTSTATUS(-1058602997i32);
pub const STATUS_SPACES_NO_REDUNDANCY: NTSTATUS = NTSTATUS(-1058602982i32);
pub const STATUS_SPACES_NUMBER_OF_COLUMNS_INVALID: NTSTATUS = NTSTATUS(-1058602998i32);
pub const STATUS_SPACES_NUMBER_OF_DATA_COPIES_INVALID: NTSTATUS = NTSTATUS(-1058603001i32);
pub const STATUS_SPACES_NUMBER_OF_GROUPS_INVALID: NTSTATUS = NTSTATUS(-1058602991i32);
pub const STATUS_SPACES_PAUSE: NTSTATUS = NTSTATUS(15138817i32);
pub const STATUS_SPACES_PD_INVALID_DATA: NTSTATUS = NTSTATUS(-1058602972i32);
pub const STATUS_SPACES_PD_LENGTH_MISMATCH: NTSTATUS = NTSTATUS(-1058602974i32);
pub const STATUS_SPACES_PD_NOT_FOUND: NTSTATUS = NTSTATUS(-1058602975i32);
pub const STATUS_SPACES_PD_UNSUPPORTED_VERSION: NTSTATUS = NTSTATUS(-1058602973i32);
pub const STATUS_SPACES_PROVISIONING_TYPE_INVALID: NTSTATUS = NTSTATUS(-1058602995i32);
pub const STATUS_SPACES_REDIRECT: NTSTATUS = NTSTATUS(15138819i32);
pub const STATUS_SPACES_REPAIRED: NTSTATUS = NTSTATUS(15138816i32);
pub const STATUS_SPACES_RESILIENCY_TYPE_INVALID: NTSTATUS = NTSTATUS(-1058603005i32);
pub const STATUS_SPACES_UNSUPPORTED_VERSION: NTSTATUS = NTSTATUS(-1058602987i32);
pub const STATUS_SPACES_UPDATE_COLUMN_STATE: NTSTATUS = NTSTATUS(-1058602989i32);
pub const STATUS_SPACES_WRITE_CACHE_SIZE_INVALID: NTSTATUS = NTSTATUS(-1058602992i32);
pub const STATUS_SPARSE_FILE_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073740604i32);
pub const STATUS_SPARSE_NOT_ALLOWED_IN_TRANSACTION: NTSTATUS = NTSTATUS(-1072103351i32);
pub const STATUS_SPECIAL_ACCOUNT: NTSTATUS = NTSTATUS(-1073741532i32);
pub const STATUS_SPECIAL_GROUP: NTSTATUS = NTSTATUS(-1073741531i32);
pub const STATUS_SPECIAL_USER: NTSTATUS = NTSTATUS(-1073741530i32);
pub const STATUS_STACK_BUFFER_OVERRUN: NTSTATUS = NTSTATUS(-1073740791i32);
pub const STATUS_STACK_OVERFLOW: NTSTATUS = NTSTATUS(-1073741571i32);
pub const STATUS_STACK_OVERFLOW_READ: NTSTATUS = NTSTATUS(-1073741272i32);
pub const STATUS_STOPPED_ON_SYMLINK: NTSTATUS = NTSTATUS(-2147483603i32);
pub const STATUS_STORAGE_LOST_DATA_PERSISTENCE: NTSTATUS = NTSTATUS(-1073740642i32);
pub const STATUS_STORAGE_RESERVE_ALREADY_EXISTS: NTSTATUS = NTSTATUS(-1073740625i32);
pub const STATUS_STORAGE_RESERVE_DOES_NOT_EXIST: NTSTATUS = NTSTATUS(-1073740626i32);
pub const STATUS_STORAGE_RESERVE_ID_INVALID: NTSTATUS = NTSTATUS(-1073740627i32);
pub const STATUS_STORAGE_RESERVE_NOT_EMPTY: NTSTATUS = NTSTATUS(-1073740624i32);
pub const STATUS_STORAGE_STACK_ACCESS_DENIED: NTSTATUS = NTSTATUS(-1073740607i32);
pub const STATUS_STORAGE_TOPOLOGY_ID_MISMATCH: NTSTATUS = NTSTATUS(-1073740666i32);
pub const STATUS_STOWED_EXCEPTION: NTSTATUS = NTSTATUS(-1073741189i32);
pub const STATUS_STREAM_MINIVERSION_NOT_FOUND: NTSTATUS = NTSTATUS(-1072103390i32);
pub const STATUS_STREAM_MINIVERSION_NOT_VALID: NTSTATUS = NTSTATUS(-1072103389i32);
pub const STATUS_STRICT_CFG_VIOLATION: NTSTATUS = NTSTATUS(-1073740282i32);
pub const STATUS_STRONG_CRYPTO_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073741066i32);
pub const STATUS_SUCCESS: NTSTATUS = NTSTATUS(0i32);
pub const STATUS_SUSPEND_COUNT_EXCEEDED: NTSTATUS = NTSTATUS(-1073741750i32);
pub const STATUS_SVHDX_ERROR_NOT_AVAILABLE: NTSTATUS = NTSTATUS(-1067647232i32);
pub const STATUS_SVHDX_ERROR_STORED: NTSTATUS = NTSTATUS(-1067712512i32);
pub const STATUS_SVHDX_NO_INITIATOR: NTSTATUS = NTSTATUS(-1067647221i32);
pub const STATUS_SVHDX_RESERVATION_CONFLICT: NTSTATUS = NTSTATUS(-1067647225i32);
pub const STATUS_SVHDX_UNIT_ATTENTION_AVAILABLE: NTSTATUS = NTSTATUS(-1067647231i32);
pub const STATUS_SVHDX_UNIT_ATTENTION_CAPACITY_DATA_CHANGED: NTSTATUS = NTSTATUS(-1067647230i32);
pub const STATUS_SVHDX_UNIT_ATTENTION_OPERATING_DEFINITION_CHANGED: NTSTATUS =
    NTSTATUS(-1067647226i32);
pub const STATUS_SVHDX_UNIT_ATTENTION_REGISTRATIONS_PREEMPTED: NTSTATUS = NTSTATUS(-1067647227i32);
pub const STATUS_SVHDX_UNIT_ATTENTION_RESERVATIONS_PREEMPTED: NTSTATUS = NTSTATUS(-1067647229i32);
pub const STATUS_SVHDX_UNIT_ATTENTION_RESERVATIONS_RELEASED: NTSTATUS = NTSTATUS(-1067647228i32);
pub const STATUS_SVHDX_VERSION_MISMATCH: NTSTATUS = NTSTATUS(-1067647223i32);
pub const STATUS_SVHDX_WRONG_FILE_TYPE: NTSTATUS = NTSTATUS(-1067647224i32);
pub const STATUS_SXS_ACTIVATION_CONTEXT_DISABLED: NTSTATUS = NTSTATUS(-1072365561i32);
pub const STATUS_SXS_ASSEMBLY_IS_NOT_A_DEPLOYMENT: NTSTATUS = NTSTATUS(-1072365538i32);
pub const STATUS_SXS_ASSEMBLY_MISSING: NTSTATUS = NTSTATUS(-1072365556i32);
pub const STATUS_SXS_ASSEMBLY_NOT_FOUND: NTSTATUS = NTSTATUS(-1072365564i32);
pub const STATUS_SXS_CANT_GEN_ACTCTX: NTSTATUS = NTSTATUS(-1072365566i32);
pub const STATUS_SXS_COMPONENT_STORE_CORRUPT: NTSTATUS = NTSTATUS(-1072365542i32);
pub const STATUS_SXS_CORRUPTION: NTSTATUS = NTSTATUS(-1072365547i32);
pub const STATUS_SXS_CORRUPT_ACTIVATION_STACK: NTSTATUS = NTSTATUS(-1072365548i32);
pub const STATUS_SXS_EARLY_DEACTIVATION: NTSTATUS = NTSTATUS(-1072365553i32);
pub const STATUS_SXS_FILE_HASH_MISMATCH: NTSTATUS = NTSTATUS(-1072365541i32);
pub const STATUS_SXS_FILE_HASH_MISSING: NTSTATUS = NTSTATUS(-1072365529i32);
pub const STATUS_SXS_FILE_NOT_PART_OF_ASSEMBLY: NTSTATUS = NTSTATUS(-1072365537i32);
pub const STATUS_SXS_IDENTITIES_DIFFERENT: NTSTATUS = NTSTATUS(-1072365539i32);
pub const STATUS_SXS_IDENTITY_DUPLICATE_ATTRIBUTE: NTSTATUS = NTSTATUS(-1072365544i32);
pub const STATUS_SXS_IDENTITY_PARSE_ERROR: NTSTATUS = NTSTATUS(-1072365543i32);
pub const STATUS_SXS_INVALID_ACTCTXDATA_FORMAT: NTSTATUS = NTSTATUS(-1072365565i32);
pub const STATUS_SXS_INVALID_DEACTIVATION: NTSTATUS = NTSTATUS(-1072365552i32);
pub const STATUS_SXS_INVALID_IDENTITY_ATTRIBUTE_NAME: NTSTATUS = NTSTATUS(-1072365545i32);
pub const STATUS_SXS_INVALID_IDENTITY_ATTRIBUTE_VALUE: NTSTATUS = NTSTATUS(-1072365546i32);
pub const STATUS_SXS_KEY_NOT_FOUND: NTSTATUS = NTSTATUS(-1072365560i32);
pub const STATUS_SXS_MANIFEST_FORMAT_ERROR: NTSTATUS = NTSTATUS(-1072365563i32);
pub const STATUS_SXS_MANIFEST_IDENTITY_SAME_BUT_CONTENTS_DIFFERENT: NTSTATUS =
    NTSTATUS(-1072365540i32);
pub const STATUS_SXS_MANIFEST_PARSE_ERROR: NTSTATUS = NTSTATUS(-1072365562i32);
pub const STATUS_SXS_MANIFEST_TOO_BIG: NTSTATUS = NTSTATUS(-1072365534i32);
pub const STATUS_SXS_MULTIPLE_DEACTIVATION: NTSTATUS = NTSTATUS(-1072365551i32);
pub const STATUS_SXS_PROCESS_DEFAULT_ALREADY_SET: NTSTATUS = NTSTATUS(-1072365554i32);
pub const STATUS_SXS_PROCESS_TERMINATION_REQUESTED: NTSTATUS = NTSTATUS(-1072365549i32);
pub const STATUS_SXS_RELEASE_ACTIVATION_CONTEXT: NTSTATUS = NTSTATUS(1075118093i32);
pub const STATUS_SXS_SECTION_NOT_FOUND: NTSTATUS = NTSTATUS(-1072365567i32);
pub const STATUS_SXS_SETTING_NOT_REGISTERED: NTSTATUS = NTSTATUS(-1072365533i32);
pub const STATUS_SXS_SYSTEM_DEFAULT_ACTIVATION_CONTEXT_EMPTY: NTSTATUS = NTSTATUS(-1072365550i32);
pub const STATUS_SXS_THREAD_QUERIES_DISABLED: NTSTATUS = NTSTATUS(-1072365557i32);
pub const STATUS_SXS_TRANSACTION_CLOSURE_INCOMPLETE: NTSTATUS = NTSTATUS(-1072365532i32);
pub const STATUS_SXS_VERSION_CONFLICT: NTSTATUS = NTSTATUS(-1072365559i32);
pub const STATUS_SXS_WRONG_SECTION_TYPE: NTSTATUS = NTSTATUS(-1072365558i32);
pub const STATUS_SYMLINK_CLASS_DISABLED: NTSTATUS = NTSTATUS(-1073740011i32);
pub const STATUS_SYNCHRONIZATION_REQUIRED: NTSTATUS = NTSTATUS(-1073741516i32);
pub const STATUS_SYSTEM_DEVICE_NOT_FOUND: NTSTATUS = NTSTATUS(-1073740718i32);
pub const STATUS_SYSTEM_HIVE_TOO_LARGE: NTSTATUS = NTSTATUS(-1073740946i32);
pub const STATUS_SYSTEM_IMAGE_BAD_SIGNATURE: NTSTATUS = NTSTATUS(-1073741103i32);
pub const STATUS_SYSTEM_INTEGRITY_INVALID_POLICY: NTSTATUS = NTSTATUS(-1058471933i32);
pub const STATUS_SYSTEM_INTEGRITY_POLICY_NOT_SIGNED: NTSTATUS = NTSTATUS(-1058471932i32);
pub const STATUS_SYSTEM_INTEGRITY_POLICY_VIOLATION: NTSTATUS = NTSTATUS(-1058471934i32);
pub const STATUS_SYSTEM_INTEGRITY_REPUTATION_DANGEROUS_EXT: NTSTATUS = NTSTATUS(-1058471927i32);
pub const STATUS_SYSTEM_INTEGRITY_REPUTATION_MALICIOUS: NTSTATUS = NTSTATUS(-1058471929i32);
pub const STATUS_SYSTEM_INTEGRITY_REPUTATION_OFFLINE: NTSTATUS = NTSTATUS(-1058471926i32);
pub const STATUS_SYSTEM_INTEGRITY_REPUTATION_PUA: NTSTATUS = NTSTATUS(-1058471928i32);
pub const STATUS_SYSTEM_INTEGRITY_ROLLBACK_DETECTED: NTSTATUS = NTSTATUS(-1058471935i32);
pub const STATUS_SYSTEM_INTEGRITY_SUPPLEMENTAL_POLICY_NOT_AUTHORIZED: NTSTATUS =
    NTSTATUS(-1058471930i32);
pub const STATUS_SYSTEM_INTEGRITY_TOO_MANY_POLICIES: NTSTATUS = NTSTATUS(-1058471931i32);
pub const STATUS_SYSTEM_NEEDS_REMEDIATION: NTSTATUS = NTSTATUS(-1073740674i32);
pub const STATUS_SYSTEM_POWERSTATE_COMPLEX_TRANSITION: NTSTATUS = NTSTATUS(1073741873i32);
pub const STATUS_SYSTEM_POWERSTATE_TRANSITION: NTSTATUS = NTSTATUS(1073741871i32);
pub const STATUS_SYSTEM_PROCESS_TERMINATED: NTSTATUS = NTSTATUS(-1073741286i32);
pub const STATUS_SYSTEM_SHUTDOWN: NTSTATUS = NTSTATUS(-1073741077i32);
pub const STATUS_THREADPOOL_FREE_LIBRARY_ON_COMPLETION_FAILED: NTSTATUS = NTSTATUS(-1073740018i32);
pub const STATUS_THREADPOOL_HANDLE_EXCEPTION: NTSTATUS = NTSTATUS(-1073740022i32);
pub const STATUS_THREADPOOL_RELEASED_DURING_OPERATION: NTSTATUS = NTSTATUS(-1073740017i32);
pub const STATUS_THREADPOOL_RELEASE_MUTEX_ON_COMPLETION_FAILED: NTSTATUS = NTSTATUS(-1073740019i32);
pub const STATUS_THREADPOOL_RELEASE_SEMAPHORE_ON_COMPLETION_FAILED: NTSTATUS =
    NTSTATUS(-1073740020i32);
pub const STATUS_THREADPOOL_SET_EVENT_ON_COMPLETION_FAILED: NTSTATUS = NTSTATUS(-1073740021i32);
pub const STATUS_THREAD_ALREADY_IN_SESSION: NTSTATUS = NTSTATUS(-1073740714i32);
pub const STATUS_THREAD_ALREADY_IN_TASK: NTSTATUS = NTSTATUS(-1073740542i32);
pub const STATUS_THREAD_IS_TERMINATING: NTSTATUS = NTSTATUS(-1073741749i32);
pub const STATUS_THREAD_NOT_IN_PROCESS: NTSTATUS = NTSTATUS(-1073741526i32);
pub const STATUS_THREAD_NOT_IN_SESSION: NTSTATUS = NTSTATUS(-1073740713i32);
pub const STATUS_THREAD_NOT_RUNNING: NTSTATUS = NTSTATUS(-1073740522i32);
pub const STATUS_THREAD_WAS_SUSPENDED: NTSTATUS = NTSTATUS(1073741825i32);
pub const STATUS_TIMEOUT: NTSTATUS = NTSTATUS(258i32);
pub const STATUS_TIMER_NOT_CANCELED: NTSTATUS = NTSTATUS(-1073741812i32);
pub const STATUS_TIMER_RESOLUTION_NOT_SET: NTSTATUS = NTSTATUS(-1073741243i32);
pub const STATUS_TIMER_RESUME_IGNORED: NTSTATUS = NTSTATUS(1073741861i32);
pub const STATUS_TIME_DIFFERENCE_AT_DC: NTSTATUS = NTSTATUS(-1073741517i32);
pub const STATUS_TM_IDENTITY_MISMATCH: NTSTATUS = NTSTATUS(-1072103350i32);
pub const STATUS_TM_INITIALIZATION_FAILED: NTSTATUS = NTSTATUS(-1072103420i32);
pub const STATUS_TM_VOLATILE: NTSTATUS = NTSTATUS(-1072103365i32);
pub const STATUS_TOKEN_ALREADY_IN_USE: NTSTATUS = NTSTATUS(-1073741525i32);
pub const STATUS_TOO_LATE: NTSTATUS = NTSTATUS(-1073741431i32);
pub const STATUS_TOO_MANY_ADDRESSES: NTSTATUS = NTSTATUS(-1073741303i32);
pub const STATUS_TOO_MANY_COMMANDS: NTSTATUS = NTSTATUS(-1073741631i32);
pub const STATUS_TOO_MANY_CONTEXT_IDS: NTSTATUS = NTSTATUS(-1073741478i32);
pub const STATUS_TOO_MANY_GUIDS_REQUESTED: NTSTATUS = NTSTATUS(-1073741694i32);
pub const STATUS_TOO_MANY_LINKS: NTSTATUS = NTSTATUS(-1073741211i32);
pub const STATUS_TOO_MANY_LUIDS_REQUESTED: NTSTATUS = NTSTATUS(-1073741708i32);
pub const STATUS_TOO_MANY_NAMES: NTSTATUS = NTSTATUS(-1073741619i32);
pub const STATUS_TOO_MANY_NODES: NTSTATUS = NTSTATUS(-1073741298i32);
pub const STATUS_TOO_MANY_OPENED_FILES: NTSTATUS = NTSTATUS(-1073741537i32);
pub const STATUS_TOO_MANY_PAGING_FILES: NTSTATUS = NTSTATUS(-1073741673i32);
pub const STATUS_TOO_MANY_PRINCIPALS: NTSTATUS = NTSTATUS(-1073741065i32);
pub const STATUS_TOO_MANY_SECRETS: NTSTATUS = NTSTATUS(-1073741482i32);
pub const STATUS_TOO_MANY_SEGMENT_DESCRIPTORS: NTSTATUS = NTSTATUS(-1073740685i32);
pub const STATUS_TOO_MANY_SESSIONS: NTSTATUS = NTSTATUS(-1073741618i32);
pub const STATUS_TOO_MANY_SIDS: NTSTATUS = NTSTATUS(-1073741442i32);
pub const STATUS_TOO_MANY_THREADS: NTSTATUS = NTSTATUS(-1073741527i32);
pub const STATUS_TPM_20_E_ASYMMETRIC: NTSTATUS = NTSTATUS(-1071054719i32);
pub const STATUS_TPM_20_E_ATTRIBUTES: NTSTATUS = NTSTATUS(-1071054718i32);
pub const STATUS_TPM_20_E_AUTHSIZE: NTSTATUS = NTSTATUS(-1071054524i32);
pub const STATUS_TPM_20_E_AUTH_CONTEXT: NTSTATUS = NTSTATUS(-1071054523i32);
pub const STATUS_TPM_20_E_AUTH_FAIL: NTSTATUS = NTSTATUS(-1071054706i32);
pub const STATUS_TPM_20_E_AUTH_MISSING: NTSTATUS = NTSTATUS(-1071054555i32);
pub const STATUS_TPM_20_E_AUTH_TYPE: NTSTATUS = NTSTATUS(-1071054556i32);
pub const STATUS_TPM_20_E_AUTH_UNAVAILABLE: NTSTATUS = NTSTATUS(-1071054545i32);
pub const STATUS_TPM_20_E_BAD_AUTH: NTSTATUS = NTSTATUS(-1071054686i32);
pub const STATUS_TPM_20_E_BAD_CONTEXT: NTSTATUS = NTSTATUS(-1071054512i32);
pub const STATUS_TPM_20_E_BINDING: NTSTATUS = NTSTATUS(-1071054683i32);
pub const STATUS_TPM_20_E_COMMAND_CODE: NTSTATUS = NTSTATUS(-1071054525i32);
pub const STATUS_TPM_20_E_COMMAND_SIZE: NTSTATUS = NTSTATUS(-1071054526i32);
pub const STATUS_TPM_20_E_CPHASH: NTSTATUS = NTSTATUS(-1071054511i32);
pub const STATUS_TPM_20_E_CURVE: NTSTATUS = NTSTATUS(-1071054682i32);
pub const STATUS_TPM_20_E_DISABLED: NTSTATUS = NTSTATUS(-1071054560i32);
pub const STATUS_TPM_20_E_ECC_CURVE: NTSTATUS = NTSTATUS(-1071054557i32);
pub const STATUS_TPM_20_E_ECC_POINT: NTSTATUS = NTSTATUS(-1071054681i32);
pub const STATUS_TPM_20_E_EXCLUSIVE: NTSTATUS = NTSTATUS(-1071054559i32);
pub const STATUS_TPM_20_E_EXPIRED: NTSTATUS = NTSTATUS(-1071054685i32);
pub const STATUS_TPM_20_E_FAILURE: NTSTATUS = NTSTATUS(-1071054591i32);
pub const STATUS_TPM_20_E_HANDLE: NTSTATUS = NTSTATUS(-1071054709i32);
pub const STATUS_TPM_20_E_HASH: NTSTATUS = NTSTATUS(-1071054717i32);
pub const STATUS_TPM_20_E_HIERARCHY: NTSTATUS = NTSTATUS(-1071054715i32);
pub const STATUS_TPM_20_E_HMAC: NTSTATUS = NTSTATUS(-1071054567i32);
pub const STATUS_TPM_20_E_INITIALIZE: NTSTATUS = NTSTATUS(-1071054592i32);
pub const STATUS_TPM_20_E_INSUFFICIENT: NTSTATUS = NTSTATUS(-1071054694i32);
pub const STATUS_TPM_20_E_INTEGRITY: NTSTATUS = NTSTATUS(-1071054689i32);
pub const STATUS_TPM_20_E_KDF: NTSTATUS = NTSTATUS(-1071054708i32);
pub const STATUS_TPM_20_E_KEY: NTSTATUS = NTSTATUS(-1071054692i32);
pub const STATUS_TPM_20_E_KEY_SIZE: NTSTATUS = NTSTATUS(-1071054713i32);
pub const STATUS_TPM_20_E_MGF: NTSTATUS = NTSTATUS(-1071054712i32);
pub const STATUS_TPM_20_E_MODE: NTSTATUS = NTSTATUS(-1071054711i32);
pub const STATUS_TPM_20_E_NEEDS_TEST: NTSTATUS = NTSTATUS(-1071054509i32);
pub const STATUS_TPM_20_E_NONCE: NTSTATUS = NTSTATUS(-1071054705i32);
pub const STATUS_TPM_20_E_NO_RESULT: NTSTATUS = NTSTATUS(-1071054508i32);
pub const STATUS_TPM_20_E_NV_AUTHORIZATION: NTSTATUS = NTSTATUS(-1071054519i32);
pub const STATUS_TPM_20_E_NV_DEFINED: NTSTATUS = NTSTATUS(-1071054516i32);
pub const STATUS_TPM_20_E_NV_LOCKED: NTSTATUS = NTSTATUS(-1071054520i32);
pub const STATUS_TPM_20_E_NV_RANGE: NTSTATUS = NTSTATUS(-1071054522i32);
pub const STATUS_TPM_20_E_NV_SIZE: NTSTATUS = NTSTATUS(-1071054521i32);
pub const STATUS_TPM_20_E_NV_SPACE: NTSTATUS = NTSTATUS(-1071054517i32);
pub const STATUS_TPM_20_E_NV_UNINITIALIZED: NTSTATUS = NTSTATUS(-1071054518i32);
pub const STATUS_TPM_20_E_PARENT: NTSTATUS = NTSTATUS(-1071054510i32);
pub const STATUS_TPM_20_E_PCR: NTSTATUS = NTSTATUS(-1071054553i32);
pub const STATUS_TPM_20_E_PCR_CHANGED: NTSTATUS = NTSTATUS(-1071054552i32);
pub const STATUS_TPM_20_E_POLICY: NTSTATUS = NTSTATUS(-1071054554i32);
pub const STATUS_TPM_20_E_POLICY_CC: NTSTATUS = NTSTATUS(-1071054684i32);
pub const STATUS_TPM_20_E_POLICY_FAIL: NTSTATUS = NTSTATUS(-1071054691i32);
pub const STATUS_TPM_20_E_PP: NTSTATUS = NTSTATUS(-1071054704i32);
pub const STATUS_TPM_20_E_PRIVATE: NTSTATUS = NTSTATUS(-1071054581i32);
pub const STATUS_TPM_20_E_RANGE: NTSTATUS = NTSTATUS(-1071054707i32);
pub const STATUS_TPM_20_E_REBOOT: NTSTATUS = NTSTATUS(-1071054544i32);
pub const STATUS_TPM_20_E_RESERVED_BITS: NTSTATUS = NTSTATUS(-1071054687i32);
pub const STATUS_TPM_20_E_SCHEME: NTSTATUS = NTSTATUS(-1071054702i32);
pub const STATUS_TPM_20_E_SELECTOR: NTSTATUS = NTSTATUS(-1071054696i32);
pub const STATUS_TPM_20_E_SENSITIVE: NTSTATUS = NTSTATUS(-1071054507i32);
pub const STATUS_TPM_20_E_SEQUENCE: NTSTATUS = NTSTATUS(-1071054589i32);
pub const STATUS_TPM_20_E_SIGNATURE: NTSTATUS = NTSTATUS(-1071054693i32);
pub const STATUS_TPM_20_E_SIZE: NTSTATUS = NTSTATUS(-1071054699i32);
pub const STATUS_TPM_20_E_SYMMETRIC: NTSTATUS = NTSTATUS(-1071054698i32);
pub const STATUS_TPM_20_E_TAG: NTSTATUS = NTSTATUS(-1071054697i32);
pub const STATUS_TPM_20_E_TICKET: NTSTATUS = NTSTATUS(-1071054688i32);
pub const STATUS_TPM_20_E_TOO_MANY_CONTEXTS: NTSTATUS = NTSTATUS(-1071054546i32);
pub const STATUS_TPM_20_E_TYPE: NTSTATUS = NTSTATUS(-1071054710i32);
pub const STATUS_TPM_20_E_UNBALANCED: NTSTATUS = NTSTATUS(-1071054543i32);
pub const STATUS_TPM_20_E_UPGRADE: NTSTATUS = NTSTATUS(-1071054547i32);
pub const STATUS_TPM_20_E_VALUE: NTSTATUS = NTSTATUS(-1071054716i32);
pub const STATUS_TPM_ACCESS_DENIED: NTSTATUS = NTSTATUS(-1071050748i32);
pub const STATUS_TPM_AREA_LOCKED: NTSTATUS = NTSTATUS(-1071054788i32);
pub const STATUS_TPM_AUDITFAILURE: NTSTATUS = NTSTATUS(-1071054844i32);
pub const STATUS_TPM_AUDITFAIL_SUCCESSFUL: NTSTATUS = NTSTATUS(-1071054799i32);
pub const STATUS_TPM_AUDITFAIL_UNSUCCESSFUL: NTSTATUS = NTSTATUS(-1071054800i32);
pub const STATUS_TPM_AUTH2FAIL: NTSTATUS = NTSTATUS(-1071054819i32);
pub const STATUS_TPM_AUTHFAIL: NTSTATUS = NTSTATUS(-1071054847i32);
pub const STATUS_TPM_AUTH_CONFLICT: NTSTATUS = NTSTATUS(-1071054789i32);
pub const STATUS_TPM_BADCONTEXT: NTSTATUS = NTSTATUS(-1071054758i32);
pub const STATUS_TPM_BADINDEX: NTSTATUS = NTSTATUS(-1071054846i32);
pub const STATUS_TPM_BADTAG: NTSTATUS = NTSTATUS(-1071054818i32);
pub const STATUS_TPM_BAD_ATTRIBUTES: NTSTATUS = NTSTATUS(-1071054782i32);
pub const STATUS_TPM_BAD_COUNTER: NTSTATUS = NTSTATUS(-1071054779i32);
pub const STATUS_TPM_BAD_DATASIZE: NTSTATUS = NTSTATUS(-1071054805i32);
pub const STATUS_TPM_BAD_DELEGATE: NTSTATUS = NTSTATUS(-1071054759i32);
pub const STATUS_TPM_BAD_HANDLE: NTSTATUS = NTSTATUS(-1071054760i32);
pub const STATUS_TPM_BAD_KEY_PROPERTY: NTSTATUS = NTSTATUS(-1071054808i32);
pub const STATUS_TPM_BAD_LOCALITY: NTSTATUS = NTSTATUS(-1071054787i32);
pub const STATUS_TPM_BAD_MIGRATION: NTSTATUS = NTSTATUS(-1071054807i32);
pub const STATUS_TPM_BAD_MODE: NTSTATUS = NTSTATUS(-1071054804i32);
pub const STATUS_TPM_BAD_ORDINAL: NTSTATUS = NTSTATUS(-1071054838i32);
pub const STATUS_TPM_BAD_PARAMETER: NTSTATUS = NTSTATUS(-1071054845i32);
pub const STATUS_TPM_BAD_PARAM_SIZE: NTSTATUS = NTSTATUS(-1071054823i32);
pub const STATUS_TPM_BAD_PRESENCE: NTSTATUS = NTSTATUS(-1071054803i32);
pub const STATUS_TPM_BAD_SCHEME: NTSTATUS = NTSTATUS(-1071054806i32);
pub const STATUS_TPM_BAD_SIGNATURE: NTSTATUS = NTSTATUS(-1071054750i32);
pub const STATUS_TPM_BAD_TYPE: NTSTATUS = NTSTATUS(-1071054796i32);
pub const STATUS_TPM_BAD_VERSION: NTSTATUS = NTSTATUS(-1071054802i32);
pub const STATUS_TPM_CLEAR_DISABLED: NTSTATUS = NTSTATUS(-1071054843i32);
pub const STATUS_TPM_COMMAND_BLOCKED: NTSTATUS = NTSTATUS(-1071053824i32);
pub const STATUS_TPM_COMMAND_CANCELED: NTSTATUS = NTSTATUS(-1071050751i32);
pub const STATUS_TPM_CONTEXT_GAP: NTSTATUS = NTSTATUS(-1071054777i32);
pub const STATUS_TPM_DAA_INPUT_DATA0: NTSTATUS = NTSTATUS(-1071054767i32);
pub const STATUS_TPM_DAA_INPUT_DATA1: NTSTATUS = NTSTATUS(-1071054766i32);
pub const STATUS_TPM_DAA_ISSUER_SETTINGS: NTSTATUS = NTSTATUS(-1071054765i32);
pub const STATUS_TPM_DAA_ISSUER_VALIDITY: NTSTATUS = NTSTATUS(-1071054762i32);
pub const STATUS_TPM_DAA_RESOURCES: NTSTATUS = NTSTATUS(-1071054768i32);
pub const STATUS_TPM_DAA_STAGE: NTSTATUS = NTSTATUS(-1071054763i32);
pub const STATUS_TPM_DAA_TPM_SETTINGS: NTSTATUS = NTSTATUS(-1071054764i32);
pub const STATUS_TPM_DAA_WRONG_W: NTSTATUS = NTSTATUS(-1071054761i32);
pub const STATUS_TPM_DEACTIVATED: NTSTATUS = NTSTATUS(-1071054842i32);
pub const STATUS_TPM_DECRYPT_ERROR: NTSTATUS = NTSTATUS(-1071054815i32);
pub const STATUS_TPM_DEFEND_LOCK_RUNNING: NTSTATUS = NTSTATUS(-1071052797i32);
pub const STATUS_TPM_DELEGATE_ADMIN: NTSTATUS = NTSTATUS(-1071054771i32);
pub const STATUS_TPM_DELEGATE_FAMILY: NTSTATUS = NTSTATUS(-1071054772i32);
pub const STATUS_TPM_DELEGATE_LOCK: NTSTATUS = NTSTATUS(-1071054773i32);
pub const STATUS_TPM_DISABLED: NTSTATUS = NTSTATUS(-1071054841i32);
pub const STATUS_TPM_DISABLED_CMD: NTSTATUS = NTSTATUS(-1071054840i32);
pub const STATUS_TPM_DOING_SELFTEST: NTSTATUS = NTSTATUS(-1071052798i32);
pub const STATUS_TPM_DUPLICATE_VHANDLE: NTSTATUS = NTSTATUS(-1071053822i32);
pub const STATUS_TPM_EMBEDDED_COMMAND_BLOCKED: NTSTATUS = NTSTATUS(-1071053821i32);
pub const STATUS_TPM_EMBEDDED_COMMAND_UNSUPPORTED: NTSTATUS = NTSTATUS(-1071053820i32);
pub const STATUS_TPM_ENCRYPT_ERROR: NTSTATUS = NTSTATUS(-1071054816i32);
pub const STATUS_TPM_ERROR_MASK: NTSTATUS = NTSTATUS(-1071054848i32);
pub const STATUS_TPM_FAIL: NTSTATUS = NTSTATUS(-1071054839i32);
pub const STATUS_TPM_FAILEDSELFTEST: NTSTATUS = NTSTATUS(-1071054820i32);
pub const STATUS_TPM_FAMILYCOUNT: NTSTATUS = NTSTATUS(-1071054784i32);
pub const STATUS_TPM_INAPPROPRIATE_ENC: NTSTATUS = NTSTATUS(-1071054834i32);
pub const STATUS_TPM_INAPPROPRIATE_SIG: NTSTATUS = NTSTATUS(-1071054809i32);
pub const STATUS_TPM_INSTALL_DISABLED: NTSTATUS = NTSTATUS(-1071054837i32);
pub const STATUS_TPM_INSUFFICIENT_BUFFER: NTSTATUS = NTSTATUS(-1071050747i32);
pub const STATUS_TPM_INVALID_AUTHHANDLE: NTSTATUS = NTSTATUS(-1071054814i32);
pub const STATUS_TPM_INVALID_FAMILY: NTSTATUS = NTSTATUS(-1071054793i32);
pub const STATUS_TPM_INVALID_HANDLE: NTSTATUS = NTSTATUS(-1071053823i32);
pub const STATUS_TPM_INVALID_KEYHANDLE: NTSTATUS = NTSTATUS(-1071054836i32);
pub const STATUS_TPM_INVALID_KEYUSAGE: NTSTATUS = NTSTATUS(-1071054812i32);
pub const STATUS_TPM_INVALID_PCR_INFO: NTSTATUS = NTSTATUS(-1071054832i32);
pub const STATUS_TPM_INVALID_POSTINIT: NTSTATUS = NTSTATUS(-1071054810i32);
pub const STATUS_TPM_INVALID_RESOURCE: NTSTATUS = NTSTATUS(-1071054795i32);
pub const STATUS_TPM_INVALID_STRUCTURE: NTSTATUS = NTSTATUS(-1071054781i32);
pub const STATUS_TPM_IOERROR: NTSTATUS = NTSTATUS(-1071054817i32);
pub const STATUS_TPM_KEYNOTFOUND: NTSTATUS = NTSTATUS(-1071054835i32);
pub const STATUS_TPM_KEY_NOTSUPPORTED: NTSTATUS = NTSTATUS(-1071054790i32);
pub const STATUS_TPM_KEY_OWNER_CONTROL: NTSTATUS = NTSTATUS(-1071054780i32);
pub const STATUS_TPM_MAXNVWRITES: NTSTATUS = NTSTATUS(-1071054776i32);
pub const STATUS_TPM_MA_AUTHORITY: NTSTATUS = NTSTATUS(-1071054753i32);
pub const STATUS_TPM_MA_DESTINATION: NTSTATUS = NTSTATUS(-1071054755i32);
pub const STATUS_TPM_MA_SOURCE: NTSTATUS = NTSTATUS(-1071054754i32);
pub const STATUS_TPM_MA_TICKET_SIGNATURE: NTSTATUS = NTSTATUS(-1071054756i32);
pub const STATUS_TPM_MIGRATEFAIL: NTSTATUS = NTSTATUS(-1071054833i32);
pub const STATUS_TPM_NEEDS_SELFTEST: NTSTATUS = NTSTATUS(-1071052799i32);
pub const STATUS_TPM_NOCONTEXTSPACE: NTSTATUS = NTSTATUS(-1071054749i32);
pub const STATUS_TPM_NOOPERATOR: NTSTATUS = NTSTATUS(-1071054775i32);
pub const STATUS_TPM_NOSPACE: NTSTATUS = NTSTATUS(-1071054831i32);
pub const STATUS_TPM_NOSRK: NTSTATUS = NTSTATUS(-1071054830i32);
pub const STATUS_TPM_NOTFIPS: NTSTATUS = NTSTATUS(-1071054794i32);
pub const STATUS_TPM_NOTLOCAL: NTSTATUS = NTSTATUS(-1071054797i32);
pub const STATUS_TPM_NOTRESETABLE: NTSTATUS = NTSTATUS(-1071054798i32);
pub const STATUS_TPM_NOTSEALED_BLOB: NTSTATUS = NTSTATUS(-1071054829i32);
pub const STATUS_TPM_NOT_FOUND: NTSTATUS = NTSTATUS(-1071050749i32);
pub const STATUS_TPM_NOT_FULLWRITE: NTSTATUS = NTSTATUS(-1071054778i32);
pub const STATUS_TPM_NO_ENDORSEMENT: NTSTATUS = NTSTATUS(-1071054813i32);
pub const STATUS_TPM_NO_NV_PERMISSION: NTSTATUS = NTSTATUS(-1071054792i32);
pub const STATUS_TPM_NO_WRAP_TRANSPORT: NTSTATUS = NTSTATUS(-1071054801i32);
pub const STATUS_TPM_OWNER_CONTROL: NTSTATUS = NTSTATUS(-1071054769i32);
pub const STATUS_TPM_OWNER_SET: NTSTATUS = NTSTATUS(-1071054828i32);
pub const STATUS_TPM_PERMANENTEK: NTSTATUS = NTSTATUS(-1071054751i32);
pub const STATUS_TPM_PER_NOWRITE: NTSTATUS = NTSTATUS(-1071054785i32);
pub const STATUS_TPM_PPI_FUNCTION_UNSUPPORTED: NTSTATUS = NTSTATUS(-1071050746i32);
pub const STATUS_TPM_READ_ONLY: NTSTATUS = NTSTATUS(-1071054786i32);
pub const STATUS_TPM_REQUIRES_SIGN: NTSTATUS = NTSTATUS(-1071054791i32);
pub const STATUS_TPM_RESOURCEMISSING: NTSTATUS = NTSTATUS(-1071054774i32);
pub const STATUS_TPM_RESOURCES: NTSTATUS = NTSTATUS(-1071054827i32);
pub const STATUS_TPM_RETRY: NTSTATUS = NTSTATUS(-1071052800i32);
pub const STATUS_TPM_SHA_ERROR: NTSTATUS = NTSTATUS(-1071054821i32);
pub const STATUS_TPM_SHA_THREAD: NTSTATUS = NTSTATUS(-1071054822i32);
pub const STATUS_TPM_SHORTRANDOM: NTSTATUS = NTSTATUS(-1071054826i32);
pub const STATUS_TPM_SIZE: NTSTATUS = NTSTATUS(-1071054825i32);
pub const STATUS_TPM_TOOMANYCONTEXTS: NTSTATUS = NTSTATUS(-1071054757i32);
pub const STATUS_TPM_TOO_MANY_CONTEXTS: NTSTATUS = NTSTATUS(-1071050750i32);
pub const STATUS_TPM_TRANSPORT_NOTEXCLUSIVE: NTSTATUS = NTSTATUS(-1071054770i32);
pub const STATUS_TPM_WRITE_LOCKED: NTSTATUS = NTSTATUS(-1071054783i32);
pub const STATUS_TPM_WRONGPCRVAL: NTSTATUS = NTSTATUS(-1071054824i32);
pub const STATUS_TPM_WRONG_ENTITYTYPE: NTSTATUS = NTSTATUS(-1071054811i32);
pub const STATUS_TPM_ZERO_EXHAUST_ENABLED: NTSTATUS = NTSTATUS(-1071038464i32);
pub const STATUS_TRANSACTED_MAPPING_UNSUPPORTED_REMOTE: NTSTATUS = NTSTATUS(-1072103360i32);
pub const STATUS_TRANSACTIONAL_CONFLICT: NTSTATUS = NTSTATUS(-1072103423i32);
pub const STATUS_TRANSACTIONAL_OPEN_NOT_ALLOWED: NTSTATUS = NTSTATUS(-1072103361i32);
pub const STATUS_TRANSACTIONMANAGER_IDENTITY_MISMATCH: NTSTATUS = NTSTATUS(-1072103332i32);
pub const STATUS_TRANSACTIONMANAGER_NOT_FOUND: NTSTATUS = NTSTATUS(-1072103343i32);
pub const STATUS_TRANSACTIONMANAGER_NOT_ONLINE: NTSTATUS = NTSTATUS(-1072103342i32);
pub const STATUS_TRANSACTIONMANAGER_RECOVERY_NAME_COLLISION: NTSTATUS = NTSTATUS(-1072103341i32);
pub const STATUS_TRANSACTIONS_NOT_FROZEN: NTSTATUS = NTSTATUS(-1072103355i32);
pub const STATUS_TRANSACTIONS_UNSUPPORTED_REMOTE: NTSTATUS = NTSTATUS(-1072103414i32);
pub const STATUS_TRANSACTION_ABORTED: NTSTATUS = NTSTATUS(-1073741297i32);
pub const STATUS_TRANSACTION_ALREADY_ABORTED: NTSTATUS = NTSTATUS(-1072103403i32);
pub const STATUS_TRANSACTION_ALREADY_COMMITTED: NTSTATUS = NTSTATUS(-1072103402i32);
pub const STATUS_TRANSACTION_FREEZE_IN_PROGRESS: NTSTATUS = NTSTATUS(-1072103354i32);
pub const STATUS_TRANSACTION_INTEGRITY_VIOLATED: NTSTATUS = NTSTATUS(-1072103333i32);
pub const STATUS_TRANSACTION_INVALID_ID: NTSTATUS = NTSTATUS(-1073741292i32);
pub const STATUS_TRANSACTION_INVALID_MARSHALL_BUFFER: NTSTATUS = NTSTATUS(-1072103401i32);
pub const STATUS_TRANSACTION_INVALID_TYPE: NTSTATUS = NTSTATUS(-1073741291i32);
pub const STATUS_TRANSACTION_MUST_WRITETHROUGH: NTSTATUS = NTSTATUS(-1072103330i32);
pub const STATUS_TRANSACTION_NOT_ACTIVE: NTSTATUS = NTSTATUS(-1072103421i32);
pub const STATUS_TRANSACTION_NOT_ENLISTED: NTSTATUS = NTSTATUS(-1072103327i32);
pub const STATUS_TRANSACTION_NOT_FOUND: NTSTATUS = NTSTATUS(-1072103346i32);
pub const STATUS_TRANSACTION_NOT_JOINED: NTSTATUS = NTSTATUS(-1072103417i32);
pub const STATUS_TRANSACTION_NOT_REQUESTED: NTSTATUS = NTSTATUS(-1072103404i32);
pub const STATUS_TRANSACTION_NOT_ROOT: NTSTATUS = NTSTATUS(-1072103340i32);
pub const STATUS_TRANSACTION_NO_MATCH: NTSTATUS = NTSTATUS(-1073741294i32);
pub const STATUS_TRANSACTION_NO_RELEASE: NTSTATUS = NTSTATUS(-1073741295i32);
pub const STATUS_TRANSACTION_NO_SUPERIOR: NTSTATUS = NTSTATUS(-1072103329i32);
pub const STATUS_TRANSACTION_OBJECT_EXPIRED: NTSTATUS = NTSTATUS(-1072103339i32);
pub const STATUS_TRANSACTION_PROPAGATION_FAILED: NTSTATUS = NTSTATUS(-1072103408i32);
pub const STATUS_TRANSACTION_RECORD_TOO_LONG: NTSTATUS = NTSTATUS(-1072103336i32);
pub const STATUS_TRANSACTION_REQUEST_NOT_VALID: NTSTATUS = NTSTATUS(-1072103405i32);
pub const STATUS_TRANSACTION_REQUIRED_PROMOTION: NTSTATUS = NTSTATUS(-1072103357i32);
pub const STATUS_TRANSACTION_RESPONDED: NTSTATUS = NTSTATUS(-1073741293i32);
pub const STATUS_TRANSACTION_RESPONSE_NOT_ENLISTED: NTSTATUS = NTSTATUS(-1072103337i32);
pub const STATUS_TRANSACTION_SCOPE_CALLBACKS_NOT_SET: NTSTATUS = NTSTATUS(-2145845182i32);
pub const STATUS_TRANSACTION_SUPERIOR_EXISTS: NTSTATUS = NTSTATUS(-1072103406i32);
pub const STATUS_TRANSACTION_TIMED_OUT: NTSTATUS = NTSTATUS(-1073741296i32);
pub const STATUS_TRANSLATION_COMPLETE: NTSTATUS = NTSTATUS(288i32);
pub const STATUS_TRANSPORT_FULL: NTSTATUS = NTSTATUS(-1073741110i32);
pub const STATUS_TRIGGERED_EXECUTABLE_MEMORY_WRITE: NTSTATUS = NTSTATUS(-1073739994i32);
pub const STATUS_TRIM_READ_ZERO_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073740686i32);
pub const STATUS_TRUSTED_DOMAIN_FAILURE: NTSTATUS = NTSTATUS(-1073741428i32);
pub const STATUS_TRUSTED_RELATIONSHIP_FAILURE: NTSTATUS = NTSTATUS(-1073741427i32);
pub const STATUS_TRUST_FAILURE: NTSTATUS = NTSTATUS(-1073741424i32);
pub const STATUS_TS_INCOMPATIBLE_SESSIONS: NTSTATUS = NTSTATUS(-1073086407i32);
pub const STATUS_TS_VIDEO_SUBSYSTEM_ERROR: NTSTATUS = NTSTATUS(-1073086406i32);
pub const STATUS_TXF_ATTRIBUTE_CORRUPT: NTSTATUS = NTSTATUS(-1072103363i32);
pub const STATUS_TXF_DIR_NOT_EMPTY: NTSTATUS = NTSTATUS(-1072103367i32);
pub const STATUS_TXF_METADATA_ALREADY_PRESENT: NTSTATUS = NTSTATUS(-2145845183i32);
pub const STATUS_UNABLE_TO_DECOMMIT_VM: NTSTATUS = NTSTATUS(-1073741780i32);
pub const STATUS_UNABLE_TO_DELETE_SECTION: NTSTATUS = NTSTATUS(-1073741797i32);
pub const STATUS_UNABLE_TO_FREE_VM: NTSTATUS = NTSTATUS(-1073741798i32);
pub const STATUS_UNABLE_TO_LOCK_MEDIA: NTSTATUS = NTSTATUS(-1073741451i32);
pub const STATUS_UNABLE_TO_UNLOAD_MEDIA: NTSTATUS = NTSTATUS(-1073741450i32);
pub const STATUS_UNDEFINED_CHARACTER: NTSTATUS = NTSTATUS(-1073741469i32);
pub const STATUS_UNDEFINED_SCOPE: NTSTATUS = NTSTATUS(-1073740540i32);
pub const STATUS_UNEXPECTED_IO_ERROR: NTSTATUS = NTSTATUS(-1073741591i32);
pub const STATUS_UNEXPECTED_MM_CREATE_ERR: NTSTATUS = NTSTATUS(-1073741590i32);
pub const STATUS_UNEXPECTED_MM_EXTEND_ERR: NTSTATUS = NTSTATUS(-1073741588i32);
pub const STATUS_UNEXPECTED_MM_MAP_ERROR: NTSTATUS = NTSTATUS(-1073741589i32);
pub const STATUS_UNEXPECTED_NETWORK_ERROR: NTSTATUS = NTSTATUS(-1073741628i32);
pub const STATUS_UNFINISHED_CONTEXT_DELETED: NTSTATUS = NTSTATUS(-1073741074i32);
pub const STATUS_UNHANDLED_EXCEPTION: NTSTATUS = NTSTATUS(-1073741500i32);
pub const STATUS_UNKNOWN_REVISION: NTSTATUS = NTSTATUS(-1073741736i32);
pub const STATUS_UNMAPPABLE_CHARACTER: NTSTATUS = NTSTATUS(-1073741470i32);
pub const STATUS_UNRECOGNIZED_MEDIA: NTSTATUS = NTSTATUS(-1073741804i32);
pub const STATUS_UNRECOGNIZED_VOLUME: NTSTATUS = NTSTATUS(-1073741489i32);
pub const STATUS_UNSATISFIED_DEPENDENCIES: NTSTATUS = NTSTATUS(-1073740615i32);
pub const STATUS_UNSUCCESSFUL: NTSTATUS = NTSTATUS(-1073741823i32);
pub const STATUS_UNSUPPORTED_COMPRESSION: NTSTATUS = NTSTATUS(-1073741217i32);
pub const STATUS_UNSUPPORTED_PAGING_MODE: NTSTATUS = NTSTATUS(-1073740613i32);
pub const STATUS_UNSUPPORTED_PREAUTH: NTSTATUS = NTSTATUS(-1073740975i32);
pub const STATUS_UNTRUSTED_MOUNT_POINT: NTSTATUS = NTSTATUS(-1073740612i32);
pub const STATUS_UNWIND: NTSTATUS = NTSTATUS(-1073741785i32);
pub const STATUS_UNWIND_CONSOLIDATE: NTSTATUS = NTSTATUS(-2147483607i32);
pub const STATUS_USER2USER_REQUIRED: NTSTATUS = NTSTATUS(-1073740792i32);
pub const STATUS_USER_APC: NTSTATUS = NTSTATUS(192i32);
pub const STATUS_USER_DELETE_TRUST_QUOTA_EXCEEDED: NTSTATUS = NTSTATUS(-1073740797i32);
pub const STATUS_USER_EXISTS: NTSTATUS = NTSTATUS(-1073741725i32);
pub const STATUS_USER_MAPPED_FILE: NTSTATUS = NTSTATUS(-1073741245i32);
pub const STATUS_USER_SESSION_DELETED: NTSTATUS = NTSTATUS(-1073741309i32);
pub const STATUS_VALIDATE_CONTINUE: NTSTATUS = NTSTATUS(-1073741199i32);
pub const STATUS_VALID_CATALOG_HASH: NTSTATUS = NTSTATUS(301i32);
pub const STATUS_VALID_IMAGE_HASH: NTSTATUS = NTSTATUS(300i32);
pub const STATUS_VALID_STRONG_CODE_HASH: NTSTATUS = NTSTATUS(302i32);
pub const STATUS_VARIABLE_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741568i32);
pub const STATUS_VDM_DISALLOWED: NTSTATUS = NTSTATUS(-1073740780i32);
pub const STATUS_VDM_HARD_ERROR: NTSTATUS = NTSTATUS(-1073741283i32);
pub const STATUS_VERIFIER_STOP: NTSTATUS = NTSTATUS(-1073740767i32);
pub const STATUS_VERIFY_REQUIRED: NTSTATUS = NTSTATUS(-2147483626i32);
pub const STATUS_VHDSET_BACKING_STORAGE_NOT_FOUND: NTSTATUS = NTSTATUS(-1067647220i32);
pub const STATUS_VHD_ALREADY_AT_OR_BELOW_MINIMUM_VIRTUAL_SIZE: NTSTATUS = NTSTATUS(-1069940685i32);
pub const STATUS_VHD_BITMAP_MISMATCH: NTSTATUS = NTSTATUS(-1069940724i32);
pub const STATUS_VHD_BLOCK_ALLOCATION_FAILURE: NTSTATUS = NTSTATUS(-1069940727i32);
pub const STATUS_VHD_BLOCK_ALLOCATION_TABLE_CORRUPT: NTSTATUS = NTSTATUS(-1069940726i32);
pub const STATUS_VHD_CHANGE_TRACKING_DISABLED: NTSTATUS = NTSTATUS(-1069940694i32);
pub const STATUS_VHD_CHILD_PARENT_ID_MISMATCH: NTSTATUS = NTSTATUS(-1069940722i32);
pub const STATUS_VHD_CHILD_PARENT_SIZE_MISMATCH: NTSTATUS = NTSTATUS(-1069940713i32);
pub const STATUS_VHD_CHILD_PARENT_TIMESTAMP_MISMATCH: NTSTATUS = NTSTATUS(-1069940721i32);
pub const STATUS_VHD_COULD_NOT_COMPUTE_MINIMUM_VIRTUAL_SIZE: NTSTATUS = NTSTATUS(-1069940686i32);
pub const STATUS_VHD_DIFFERENCING_CHAIN_CYCLE_DETECTED: NTSTATUS = NTSTATUS(-1069940712i32);
pub const STATUS_VHD_DIFFERENCING_CHAIN_ERROR_IN_PARENT: NTSTATUS = NTSTATUS(-1069940711i32);
pub const STATUS_VHD_DRIVE_FOOTER_CHECKSUM_MISMATCH: NTSTATUS = NTSTATUS(-1069940734i32);
pub const STATUS_VHD_DRIVE_FOOTER_CORRUPT: NTSTATUS = NTSTATUS(-1069940733i32);
pub const STATUS_VHD_DRIVE_FOOTER_MISSING: NTSTATUS = NTSTATUS(-1069940735i32);
pub const STATUS_VHD_FORMAT_UNKNOWN: NTSTATUS = NTSTATUS(-1069940732i32);
pub const STATUS_VHD_FORMAT_UNSUPPORTED_VERSION: NTSTATUS = NTSTATUS(-1069940731i32);
pub const STATUS_VHD_INVALID_BLOCK_SIZE: NTSTATUS = NTSTATUS(-1069940725i32);
pub const STATUS_VHD_INVALID_CHANGE_TRACKING_ID: NTSTATUS = NTSTATUS(-1069940695i32);
pub const STATUS_VHD_INVALID_FILE_SIZE: NTSTATUS = NTSTATUS(-1069940717i32);
pub const STATUS_VHD_INVALID_SIZE: NTSTATUS = NTSTATUS(-1069940718i32);
pub const STATUS_VHD_INVALID_STATE: NTSTATUS = NTSTATUS(-1069940708i32);
pub const STATUS_VHD_INVALID_TYPE: NTSTATUS = NTSTATUS(-1069940709i32);
pub const STATUS_VHD_METADATA_FULL: NTSTATUS = NTSTATUS(-1069940696i32);
pub const STATUS_VHD_METADATA_READ_FAILURE: NTSTATUS = NTSTATUS(-1069940720i32);
pub const STATUS_VHD_METADATA_WRITE_FAILURE: NTSTATUS = NTSTATUS(-1069940719i32);
pub const STATUS_VHD_MISSING_CHANGE_TRACKING_INFORMATION: NTSTATUS = NTSTATUS(-1069940688i32);
pub const STATUS_VHD_PARENT_VHD_ACCESS_DENIED: NTSTATUS = NTSTATUS(-1069940714i32);
pub const STATUS_VHD_PARENT_VHD_NOT_FOUND: NTSTATUS = NTSTATUS(-1069940723i32);
pub const STATUS_VHD_RESIZE_WOULD_TRUNCATE_DATA: NTSTATUS = NTSTATUS(-1069940687i32);
pub const STATUS_VHD_SHARED: NTSTATUS = NTSTATUS(-1067647222i32);
pub const STATUS_VHD_SPARSE_HEADER_CHECKSUM_MISMATCH: NTSTATUS = NTSTATUS(-1069940730i32);
pub const STATUS_VHD_SPARSE_HEADER_CORRUPT: NTSTATUS = NTSTATUS(-1069940728i32);
pub const STATUS_VHD_SPARSE_HEADER_UNSUPPORTED_VERSION: NTSTATUS = NTSTATUS(-1069940729i32);
pub const STATUS_VIDEO_DRIVER_DEBUG_REPORT_REQUEST: NTSTATUS = NTSTATUS(1075511532i32);
pub const STATUS_VIDEO_HUNG_DISPLAY_DRIVER_THREAD: NTSTATUS = NTSTATUS(-1071972118i32);
pub const STATUS_VIDEO_HUNG_DISPLAY_DRIVER_THREAD_RECOVERED: NTSTATUS = NTSTATUS(-2145713941i32);
pub const STATUS_VID_CHILD_GPA_PAGE_SET_CORRUPTED: NTSTATUS = NTSTATUS(-1070137330i32);
pub const STATUS_VID_DUPLICATE_HANDLER: NTSTATUS = NTSTATUS(-1070137343i32);
pub const STATUS_VID_EXCEEDED_KM_CONTEXT_COUNT_LIMIT: NTSTATUS = NTSTATUS(-1070137314i32);
pub const STATUS_VID_EXCEEDED_MBP_ENTRY_MAP_LIMIT: NTSTATUS = NTSTATUS(-1070137332i32);
pub const STATUS_VID_HANDLER_NOT_PRESENT: NTSTATUS = NTSTATUS(-1070137340i32);
pub const STATUS_VID_INVALID_CHILD_GPA_PAGE_SET: NTSTATUS = NTSTATUS(-1070137310i32);
pub const STATUS_VID_INVALID_GPA_RANGE_HANDLE: NTSTATUS = NTSTATUS(-1070137323i32);
pub const STATUS_VID_INVALID_MEMORY_BLOCK_HANDLE: NTSTATUS = NTSTATUS(-1070137326i32);
pub const STATUS_VID_INVALID_MESSAGE_QUEUE_HANDLE: NTSTATUS = NTSTATUS(-1070137324i32);
pub const STATUS_VID_INVALID_NUMA_NODE_INDEX: NTSTATUS = NTSTATUS(-1070137328i32);
pub const STATUS_VID_INVALID_NUMA_SETTINGS: NTSTATUS = NTSTATUS(-1070137329i32);
pub const STATUS_VID_INVALID_OBJECT_NAME: NTSTATUS = NTSTATUS(-1070137339i32);
pub const STATUS_VID_INVALID_PPM_HANDLE: NTSTATUS = NTSTATUS(-1070137320i32);
pub const STATUS_VID_INVALID_PROCESSOR_STATE: NTSTATUS = NTSTATUS(-1070137315i32);
pub const STATUS_VID_KM_INTERFACE_ALREADY_INITIALIZED: NTSTATUS = NTSTATUS(-1070137313i32);
pub const STATUS_VID_MBPS_ARE_LOCKED: NTSTATUS = NTSTATUS(-1070137319i32);
pub const STATUS_VID_MBP_ALREADY_LOCKED_USING_RESERVED_PAGE: NTSTATUS = NTSTATUS(-1070137307i32);
pub const STATUS_VID_MBP_COUNT_EXCEEDED_LIMIT: NTSTATUS = NTSTATUS(-1070137306i32);
pub const STATUS_VID_MB_PROPERTY_ALREADY_SET_RESET: NTSTATUS = NTSTATUS(-1070137312i32);
pub const STATUS_VID_MB_STILL_REFERENCED: NTSTATUS = NTSTATUS(-1070137331i32);
pub const STATUS_VID_MEMORY_BLOCK_LOCK_COUNT_EXCEEDED: NTSTATUS = NTSTATUS(-1070137321i32);
pub const STATUS_VID_MESSAGE_QUEUE_ALREADY_EXISTS: NTSTATUS = NTSTATUS(-1070137333i32);
pub const STATUS_VID_MESSAGE_QUEUE_CLOSED: NTSTATUS = NTSTATUS(-1070137318i32);
pub const STATUS_VID_MESSAGE_QUEUE_NAME_TOO_LONG: NTSTATUS = NTSTATUS(-1070137337i32);
pub const STATUS_VID_MMIO_RANGE_DESTROYED: NTSTATUS = NTSTATUS(-1070137311i32);
pub const STATUS_VID_NOTIFICATION_QUEUE_ALREADY_ASSOCIATED: NTSTATUS = NTSTATUS(-1070137327i32);
pub const STATUS_VID_NO_MEMORY_BLOCK_NOTIFICATION_QUEUE: NTSTATUS = NTSTATUS(-1070137322i32);
pub const STATUS_VID_PAGE_RANGE_OVERFLOW: NTSTATUS = NTSTATUS(-1070137325i32);
pub const STATUS_VID_PARTITION_ALREADY_EXISTS: NTSTATUS = NTSTATUS(-1070137336i32);
pub const STATUS_VID_PARTITION_DOES_NOT_EXIST: NTSTATUS = NTSTATUS(-1070137335i32);
pub const STATUS_VID_PARTITION_NAME_NOT_FOUND: NTSTATUS = NTSTATUS(-1070137334i32);
pub const STATUS_VID_PARTITION_NAME_TOO_LONG: NTSTATUS = NTSTATUS(-1070137338i32);
pub const STATUS_VID_QUEUE_FULL: NTSTATUS = NTSTATUS(-1070137341i32);
pub const STATUS_VID_REMOTE_NODE_PARENT_GPA_PAGES_USED: NTSTATUS = NTSTATUS(-2143879167i32);
pub const STATUS_VID_RESERVE_PAGE_SET_IS_BEING_USED: NTSTATUS = NTSTATUS(-1070137309i32);
pub const STATUS_VID_RESERVE_PAGE_SET_TOO_SMALL: NTSTATUS = NTSTATUS(-1070137308i32);
pub const STATUS_VID_SAVED_STATE_CORRUPT: NTSTATUS = NTSTATUS(-1070137305i32);
pub const STATUS_VID_SAVED_STATE_INCOMPATIBLE: NTSTATUS = NTSTATUS(-1070137303i32);
pub const STATUS_VID_SAVED_STATE_UNRECOGNIZED_ITEM: NTSTATUS = NTSTATUS(-1070137304i32);
pub const STATUS_VID_STOP_PENDING: NTSTATUS = NTSTATUS(-1070137316i32);
pub const STATUS_VID_TOO_MANY_HANDLERS: NTSTATUS = NTSTATUS(-1070137342i32);
pub const STATUS_VID_VIRTUAL_PROCESSOR_LIMIT_EXCEEDED: NTSTATUS = NTSTATUS(-1070137317i32);
pub const STATUS_VID_VTL_ACCESS_DENIED: NTSTATUS = NTSTATUS(-1070137302i32);
pub const STATUS_VIRTDISK_DISK_ALREADY_OWNED: NTSTATUS = NTSTATUS(-1069940706i32);
pub const STATUS_VIRTDISK_DISK_ONLINE_AND_WRITABLE: NTSTATUS = NTSTATUS(-1069940705i32);
pub const STATUS_VIRTDISK_NOT_VIRTUAL_DISK: NTSTATUS = NTSTATUS(-1069940715i32);
pub const STATUS_VIRTDISK_PROVIDER_NOT_FOUND: NTSTATUS = NTSTATUS(-1069940716i32);
pub const STATUS_VIRTDISK_UNSUPPORTED_DISK_SECTOR_SIZE: NTSTATUS = NTSTATUS(-1069940707i32);
pub const STATUS_VIRTUAL_CIRCUIT_CLOSED: NTSTATUS = NTSTATUS(-1073741610i32);
pub const STATUS_VIRTUAL_DISK_LIMITATION: NTSTATUS = NTSTATUS(-1069940710i32);
pub const STATUS_VIRUS_DELETED: NTSTATUS = NTSTATUS(-1073739513i32);
pub const STATUS_VIRUS_INFECTED: NTSTATUS = NTSTATUS(-1073739514i32);
pub const STATUS_VOLMGR_ALL_DISKS_FAILED: NTSTATUS = NTSTATUS(-1070071767i32);
pub const STATUS_VOLMGR_BAD_BOOT_DISK: NTSTATUS = NTSTATUS(-1070071729i32);
pub const STATUS_VOLMGR_DATABASE_FULL: NTSTATUS = NTSTATUS(-1070071807i32);
pub const STATUS_VOLMGR_DIFFERENT_SECTOR_SIZE: NTSTATUS = NTSTATUS(-1070071730i32);
pub const STATUS_VOLMGR_DISK_CONFIGURATION_CORRUPTED: NTSTATUS = NTSTATUS(-1070071806i32);
pub const STATUS_VOLMGR_DISK_CONFIGURATION_NOT_IN_SYNC: NTSTATUS = NTSTATUS(-1070071805i32);
pub const STATUS_VOLMGR_DISK_CONTAINS_NON_SIMPLE_VOLUME: NTSTATUS = NTSTATUS(-1070071803i32);
pub const STATUS_VOLMGR_DISK_DUPLICATE: NTSTATUS = NTSTATUS(-1070071802i32);
pub const STATUS_VOLMGR_DISK_DYNAMIC: NTSTATUS = NTSTATUS(-1070071801i32);
pub const STATUS_VOLMGR_DISK_ID_INVALID: NTSTATUS = NTSTATUS(-1070071800i32);
pub const STATUS_VOLMGR_DISK_INVALID: NTSTATUS = NTSTATUS(-1070071799i32);
pub const STATUS_VOLMGR_DISK_LAST_VOTER: NTSTATUS = NTSTATUS(-1070071798i32);
pub const STATUS_VOLMGR_DISK_LAYOUT_INVALID: NTSTATUS = NTSTATUS(-1070071797i32);
pub const STATUS_VOLMGR_DISK_LAYOUT_NON_BASIC_BETWEEN_BASIC_PARTITIONS: NTSTATUS =
    NTSTATUS(-1070071796i32);
pub const STATUS_VOLMGR_DISK_LAYOUT_NOT_CYLINDER_ALIGNED: NTSTATUS = NTSTATUS(-1070071795i32);
pub const STATUS_VOLMGR_DISK_LAYOUT_PARTITIONS_TOO_SMALL: NTSTATUS = NTSTATUS(-1070071794i32);
pub const STATUS_VOLMGR_DISK_LAYOUT_PRIMARY_BETWEEN_LOGICAL_PARTITIONS: NTSTATUS =
    NTSTATUS(-1070071793i32);
pub const STATUS_VOLMGR_DISK_LAYOUT_TOO_MANY_PARTITIONS: NTSTATUS = NTSTATUS(-1070071792i32);
pub const STATUS_VOLMGR_DISK_MISSING: NTSTATUS = NTSTATUS(-1070071791i32);
pub const STATUS_VOLMGR_DISK_NOT_EMPTY: NTSTATUS = NTSTATUS(-1070071790i32);
pub const STATUS_VOLMGR_DISK_NOT_ENOUGH_SPACE: NTSTATUS = NTSTATUS(-1070071789i32);
pub const STATUS_VOLMGR_DISK_REVECTORING_FAILED: NTSTATUS = NTSTATUS(-1070071788i32);
pub const STATUS_VOLMGR_DISK_SECTOR_SIZE_INVALID: NTSTATUS = NTSTATUS(-1070071787i32);
pub const STATUS_VOLMGR_DISK_SET_NOT_CONTAINED: NTSTATUS = NTSTATUS(-1070071786i32);
pub const STATUS_VOLMGR_DISK_USED_BY_MULTIPLE_MEMBERS: NTSTATUS = NTSTATUS(-1070071785i32);
pub const STATUS_VOLMGR_DISK_USED_BY_MULTIPLE_PLEXES: NTSTATUS = NTSTATUS(-1070071784i32);
pub const STATUS_VOLMGR_DYNAMIC_DISK_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1070071783i32);
pub const STATUS_VOLMGR_EXTENT_ALREADY_USED: NTSTATUS = NTSTATUS(-1070071782i32);
pub const STATUS_VOLMGR_EXTENT_NOT_CONTIGUOUS: NTSTATUS = NTSTATUS(-1070071781i32);
pub const STATUS_VOLMGR_EXTENT_NOT_IN_PUBLIC_REGION: NTSTATUS = NTSTATUS(-1070071780i32);
pub const STATUS_VOLMGR_EXTENT_NOT_SECTOR_ALIGNED: NTSTATUS = NTSTATUS(-1070071779i32);
pub const STATUS_VOLMGR_EXTENT_OVERLAPS_EBR_PARTITION: NTSTATUS = NTSTATUS(-1070071778i32);
pub const STATUS_VOLMGR_EXTENT_VOLUME_LENGTHS_DO_NOT_MATCH: NTSTATUS = NTSTATUS(-1070071777i32);
pub const STATUS_VOLMGR_FAULT_TOLERANT_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1070071776i32);
pub const STATUS_VOLMGR_INCOMPLETE_DISK_MIGRATION: NTSTATUS = NTSTATUS(-2143813630i32);
pub const STATUS_VOLMGR_INCOMPLETE_REGENERATION: NTSTATUS = NTSTATUS(-2143813631i32);
pub const STATUS_VOLMGR_INTERLEAVE_LENGTH_INVALID: NTSTATUS = NTSTATUS(-1070071775i32);
pub const STATUS_VOLMGR_MAXIMUM_REGISTERED_USERS: NTSTATUS = NTSTATUS(-1070071774i32);
pub const STATUS_VOLMGR_MEMBER_INDEX_DUPLICATE: NTSTATUS = NTSTATUS(-1070071772i32);
pub const STATUS_VOLMGR_MEMBER_INDEX_INVALID: NTSTATUS = NTSTATUS(-1070071771i32);
pub const STATUS_VOLMGR_MEMBER_IN_SYNC: NTSTATUS = NTSTATUS(-1070071773i32);
pub const STATUS_VOLMGR_MEMBER_MISSING: NTSTATUS = NTSTATUS(-1070071770i32);
pub const STATUS_VOLMGR_MEMBER_NOT_DETACHED: NTSTATUS = NTSTATUS(-1070071769i32);
pub const STATUS_VOLMGR_MEMBER_REGENERATING: NTSTATUS = NTSTATUS(-1070071768i32);
pub const STATUS_VOLMGR_MIRROR_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1070071717i32);
pub const STATUS_VOLMGR_NOTIFICATION_RESET: NTSTATUS = NTSTATUS(-1070071764i32);
pub const STATUS_VOLMGR_NOT_PRIMARY_PACK: NTSTATUS = NTSTATUS(-1070071726i32);
pub const STATUS_VOLMGR_NO_REGISTERED_USERS: NTSTATUS = NTSTATUS(-1070071766i32);
pub const STATUS_VOLMGR_NO_SUCH_USER: NTSTATUS = NTSTATUS(-1070071765i32);
pub const STATUS_VOLMGR_NO_VALID_LOG_COPIES: NTSTATUS = NTSTATUS(-1070071720i32);
pub const STATUS_VOLMGR_NUMBER_OF_DISKS_INVALID: NTSTATUS = NTSTATUS(-1070071718i32);
pub const STATUS_VOLMGR_NUMBER_OF_DISKS_IN_MEMBER_INVALID: NTSTATUS = NTSTATUS(-1070071723i32);
pub const STATUS_VOLMGR_NUMBER_OF_DISKS_IN_PLEX_INVALID: NTSTATUS = NTSTATUS(-1070071724i32);
pub const STATUS_VOLMGR_NUMBER_OF_EXTENTS_INVALID: NTSTATUS = NTSTATUS(-1070071731i32);
pub const STATUS_VOLMGR_NUMBER_OF_MEMBERS_INVALID: NTSTATUS = NTSTATUS(-1070071763i32);
pub const STATUS_VOLMGR_NUMBER_OF_PLEXES_INVALID: NTSTATUS = NTSTATUS(-1070071762i32);
pub const STATUS_VOLMGR_PACK_CONFIG_OFFLINE: NTSTATUS = NTSTATUS(-1070071728i32);
pub const STATUS_VOLMGR_PACK_CONFIG_ONLINE: NTSTATUS = NTSTATUS(-1070071727i32);
pub const STATUS_VOLMGR_PACK_CONFIG_UPDATE_FAILED: NTSTATUS = NTSTATUS(-1070071804i32);
pub const STATUS_VOLMGR_PACK_DUPLICATE: NTSTATUS = NTSTATUS(-1070071761i32);
pub const STATUS_VOLMGR_PACK_HAS_QUORUM: NTSTATUS = NTSTATUS(-1070071756i32);
pub const STATUS_VOLMGR_PACK_ID_INVALID: NTSTATUS = NTSTATUS(-1070071760i32);
pub const STATUS_VOLMGR_PACK_INVALID: NTSTATUS = NTSTATUS(-1070071759i32);
pub const STATUS_VOLMGR_PACK_LOG_UPDATE_FAILED: NTSTATUS = NTSTATUS(-1070071725i32);
pub const STATUS_VOLMGR_PACK_NAME_INVALID: NTSTATUS = NTSTATUS(-1070071758i32);
pub const STATUS_VOLMGR_PACK_OFFLINE: NTSTATUS = NTSTATUS(-1070071757i32);
pub const STATUS_VOLMGR_PACK_WITHOUT_QUORUM: NTSTATUS = NTSTATUS(-1070071755i32);
pub const STATUS_VOLMGR_PARTITION_STYLE_INVALID: NTSTATUS = NTSTATUS(-1070071754i32);
pub const STATUS_VOLMGR_PARTITION_UPDATE_FAILED: NTSTATUS = NTSTATUS(-1070071753i32);
pub const STATUS_VOLMGR_PLEX_INDEX_DUPLICATE: NTSTATUS = NTSTATUS(-1070071751i32);
pub const STATUS_VOLMGR_PLEX_INDEX_INVALID: NTSTATUS = NTSTATUS(-1070071750i32);
pub const STATUS_VOLMGR_PLEX_IN_SYNC: NTSTATUS = NTSTATUS(-1070071752i32);
pub const STATUS_VOLMGR_PLEX_LAST_ACTIVE: NTSTATUS = NTSTATUS(-1070071749i32);
pub const STATUS_VOLMGR_PLEX_MISSING: NTSTATUS = NTSTATUS(-1070071748i32);
pub const STATUS_VOLMGR_PLEX_NOT_RAID5: NTSTATUS = NTSTATUS(-1070071745i32);
pub const STATUS_VOLMGR_PLEX_NOT_SIMPLE: NTSTATUS = NTSTATUS(-1070071744i32);
pub const STATUS_VOLMGR_PLEX_NOT_SIMPLE_SPANNED: NTSTATUS = NTSTATUS(-1070071721i32);
pub const STATUS_VOLMGR_PLEX_REGENERATING: NTSTATUS = NTSTATUS(-1070071747i32);
pub const STATUS_VOLMGR_PLEX_TYPE_INVALID: NTSTATUS = NTSTATUS(-1070071746i32);
pub const STATUS_VOLMGR_PRIMARY_PACK_PRESENT: NTSTATUS = NTSTATUS(-1070071719i32);
pub const STATUS_VOLMGR_RAID5_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1070071716i32);
pub const STATUS_VOLMGR_STRUCTURE_SIZE_INVALID: NTSTATUS = NTSTATUS(-1070071743i32);
pub const STATUS_VOLMGR_TOO_MANY_NOTIFICATION_REQUESTS: NTSTATUS = NTSTATUS(-1070071742i32);
pub const STATUS_VOLMGR_TRANSACTION_IN_PROGRESS: NTSTATUS = NTSTATUS(-1070071741i32);
pub const STATUS_VOLMGR_UNEXPECTED_DISK_LAYOUT_CHANGE: NTSTATUS = NTSTATUS(-1070071740i32);
pub const STATUS_VOLMGR_VOLUME_CONTAINS_MISSING_DISK: NTSTATUS = NTSTATUS(-1070071739i32);
pub const STATUS_VOLMGR_VOLUME_ID_INVALID: NTSTATUS = NTSTATUS(-1070071738i32);
pub const STATUS_VOLMGR_VOLUME_LENGTH_INVALID: NTSTATUS = NTSTATUS(-1070071737i32);
pub const STATUS_VOLMGR_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE: NTSTATUS = NTSTATUS(-1070071736i32);
pub const STATUS_VOLMGR_VOLUME_MIRRORED: NTSTATUS = NTSTATUS(-1070071722i32);
pub const STATUS_VOLMGR_VOLUME_NOT_MIRRORED: NTSTATUS = NTSTATUS(-1070071735i32);
pub const STATUS_VOLMGR_VOLUME_NOT_RETAINED: NTSTATUS = NTSTATUS(-1070071734i32);
pub const STATUS_VOLMGR_VOLUME_OFFLINE: NTSTATUS = NTSTATUS(-1070071733i32);
pub const STATUS_VOLMGR_VOLUME_RETAINED: NTSTATUS = NTSTATUS(-1070071732i32);
pub const STATUS_VOLSNAP_ACTIVATION_TIMEOUT: NTSTATUS = NTSTATUS(-1068498940i32);
pub const STATUS_VOLSNAP_BOOTFILE_NOT_VALID: NTSTATUS = NTSTATUS(-1068498941i32);
pub const STATUS_VOLSNAP_HIBERNATE_READY: NTSTATUS = NTSTATUS(293i32);
pub const STATUS_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT: NTSTATUS = NTSTATUS(-1068498939i32);
pub const STATUS_VOLSNAP_PREPARE_HIBERNATE: NTSTATUS = NTSTATUS(-1073740793i32);
pub const STATUS_VOLUME_DIRTY: NTSTATUS = NTSTATUS(-1073739770i32);
pub const STATUS_VOLUME_DISMOUNTED: NTSTATUS = NTSTATUS(-1073741202i32);
pub const STATUS_VOLUME_MOUNTED: NTSTATUS = NTSTATUS(265i32);
pub const STATUS_VOLUME_NOT_CLUSTER_ALIGNED: NTSTATUS = NTSTATUS(-1073740636i32);
pub const STATUS_VOLUME_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073740602i32);
pub const STATUS_VOLUME_NOT_UPGRADED: NTSTATUS = NTSTATUS(-1073741156i32);
pub const STATUS_VOLUME_WRITE_ACCESS_DENIED: NTSTATUS = NTSTATUS(-1073740589i32);
pub const STATUS_VRF_VOLATILE_CFG_AND_IO_ENABLED: NTSTATUS = NTSTATUS(-1073738744i32);
pub const STATUS_VRF_VOLATILE_NMI_REGISTERED: NTSTATUS = NTSTATUS(-1073738738i32);
pub const STATUS_VRF_VOLATILE_NOT_RUNNABLE_SYSTEM: NTSTATUS = NTSTATUS(-1073738741i32);
pub const STATUS_VRF_VOLATILE_NOT_STOPPABLE: NTSTATUS = NTSTATUS(-1073738743i32);
pub const STATUS_VRF_VOLATILE_NOT_SUPPORTED_RULECLASS: NTSTATUS = NTSTATUS(-1073738740i32);
pub const STATUS_VRF_VOLATILE_PROTECTED_DRIVER: NTSTATUS = NTSTATUS(-1073738739i32);
pub const STATUS_VRF_VOLATILE_SAFE_MODE: NTSTATUS = NTSTATUS(-1073738742i32);
pub const STATUS_VRF_VOLATILE_SETTINGS_CONFLICT: NTSTATUS = NTSTATUS(-1073738737i32);
pub const STATUS_VSM_DMA_PROTECTION_NOT_IN_USE: NTSTATUS = NTSTATUS(-1069219839i32);
pub const STATUS_VSM_NOT_INITIALIZED: NTSTATUS = NTSTATUS(-1069219840i32);
pub const STATUS_WAIT_0: NTSTATUS = NTSTATUS(0i32);
pub const STATUS_WAIT_1: NTSTATUS = NTSTATUS(1i32);
pub const STATUS_WAIT_2: NTSTATUS = NTSTATUS(2i32);
pub const STATUS_WAIT_3: NTSTATUS = NTSTATUS(3i32);
pub const STATUS_WAIT_63: NTSTATUS = NTSTATUS(63i32);
pub const STATUS_WAIT_FOR_OPLOCK: NTSTATUS = NTSTATUS(871i32);
pub const STATUS_WAKE_SYSTEM: NTSTATUS = NTSTATUS(1073742484i32);
pub const STATUS_WAKE_SYSTEM_DEBUGGER: NTSTATUS = NTSTATUS(-2147483641i32);
pub const STATUS_WAS_LOCKED: NTSTATUS = NTSTATUS(1073741849i32);
pub const STATUS_WAS_UNLOCKED: NTSTATUS = NTSTATUS(1073741847i32);
pub const STATUS_WEAK_WHFBKEY_BLOCKED: NTSTATUS = NTSTATUS(-1073741389i32);
pub const STATUS_WIM_NOT_BOOTABLE: NTSTATUS = NTSTATUS(-1073740665i32);
pub const STATUS_WMI_ALREADY_DISABLED: NTSTATUS = NTSTATUS(-1073741054i32);
pub const STATUS_WMI_ALREADY_ENABLED: NTSTATUS = NTSTATUS(-1073741053i32);
pub const STATUS_WMI_GUID_DISCONNECTED: NTSTATUS = NTSTATUS(-1073741055i32);
pub const STATUS_WMI_GUID_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741163i32);
pub const STATUS_WMI_INSTANCE_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741162i32);
pub const STATUS_WMI_ITEMID_NOT_FOUND: NTSTATUS = NTSTATUS(-1073741161i32);
pub const STATUS_WMI_NOT_SUPPORTED: NTSTATUS = NTSTATUS(-1073741091i32);
pub const STATUS_WMI_READ_ONLY: NTSTATUS = NTSTATUS(-1073741114i32);
pub const STATUS_WMI_SET_FAILURE: NTSTATUS = NTSTATUS(-1073741113i32);
pub const STATUS_WMI_TRY_AGAIN: NTSTATUS = NTSTATUS(-1073741160i32);
pub const STATUS_WOF_FILE_RESOURCE_TABLE_CORRUPT: NTSTATUS = NTSTATUS(-1073700185i32);
pub const STATUS_WOF_WIM_HEADER_CORRUPT: NTSTATUS = NTSTATUS(-1073700187i32);
pub const STATUS_WOF_WIM_RESOURCE_TABLE_CORRUPT: NTSTATUS = NTSTATUS(-1073700186i32);
pub const STATUS_WORKING_SET_LIMIT_RANGE: NTSTATUS = NTSTATUS(1073741826i32);
pub const STATUS_WORKING_SET_QUOTA: NTSTATUS = NTSTATUS(-1073741663i32);
pub const STATUS_WOW_ASSERTION: NTSTATUS = NTSTATUS(-1073702760i32);
pub const STATUS_WRONG_COMPARTMENT: NTSTATUS = NTSTATUS(-1073700731i32);
pub const STATUS_WRONG_CREDENTIAL_HANDLE: NTSTATUS = NTSTATUS(-1073741070i32);
pub const STATUS_WRONG_EFS: NTSTATUS = NTSTATUS(-1073741169i32);
pub const STATUS_WRONG_PASSWORD_CORE: NTSTATUS = NTSTATUS(-1073741495i32);
pub const STATUS_WRONG_VOLUME: NTSTATUS = NTSTATUS(-1073741806i32);
pub const STATUS_WX86_BREAKPOINT: NTSTATUS = NTSTATUS(1073741855i32);
pub const STATUS_WX86_CONTINUE: NTSTATUS = NTSTATUS(1073741853i32);
pub const STATUS_WX86_CREATEWX86TIB: NTSTATUS = NTSTATUS(1073741864i32);
pub const STATUS_WX86_EXCEPTION_CHAIN: NTSTATUS = NTSTATUS(1073741858i32);
pub const STATUS_WX86_EXCEPTION_CONTINUE: NTSTATUS = NTSTATUS(1073741856i32);
pub const STATUS_WX86_EXCEPTION_LASTCHANCE: NTSTATUS = NTSTATUS(1073741857i32);
pub const STATUS_WX86_FLOAT_STACK_CHECK: NTSTATUS = NTSTATUS(-1073741200i32);
pub const STATUS_WX86_INTERNAL_ERROR: NTSTATUS = NTSTATUS(-1073741201i32);
pub const STATUS_WX86_SINGLE_STEP: NTSTATUS = NTSTATUS(1073741854i32);
pub const STATUS_WX86_UNSIMULATE: NTSTATUS = NTSTATUS(1073741852i32);
pub const STATUS_XMLDSIG_ERROR: NTSTATUS = NTSTATUS(-1073700732i32);
pub const STATUS_XML_ENCODING_MISMATCH: NTSTATUS = NTSTATUS(-1072365535i32);
pub const STATUS_XML_PARSE_ERROR: NTSTATUS = NTSTATUS(-1073700733i32);
pub const STG_E_ABNORMALAPIEXIT: crate::core::HRESULT = crate::core::HRESULT(-2147286790i32);
pub const STG_E_ACCESSDENIED: crate::core::HRESULT = crate::core::HRESULT(-2147287035i32);
pub const STG_E_BADBASEADDRESS: crate::core::HRESULT = crate::core::HRESULT(-2147286768i32);
pub const STG_E_CANTSAVE: crate::core::HRESULT = crate::core::HRESULT(-2147286781i32);
pub const STG_E_CSS_AUTHENTICATION_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-2147286266i32);
pub const STG_E_CSS_KEY_NOT_ESTABLISHED: crate::core::HRESULT =
    crate::core::HRESULT(-2147286264i32);
pub const STG_E_CSS_KEY_NOT_PRESENT: crate::core::HRESULT = crate::core::HRESULT(-2147286265i32);
pub const STG_E_CSS_REGION_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2147286262i32);
pub const STG_E_CSS_SCRAMBLED_SECTOR: crate::core::HRESULT = crate::core::HRESULT(-2147286263i32);
pub const STG_E_DEVICE_UNRESPONSIVE: crate::core::HRESULT = crate::core::HRESULT(-2147286518i32);
pub const STG_E_DISKISWRITEPROTECTED: crate::core::HRESULT = crate::core::HRESULT(-2147287021i32);
pub const STG_E_DOCFILECORRUPT: crate::core::HRESULT = crate::core::HRESULT(-2147286775i32);
pub const STG_E_DOCFILETOOLARGE: crate::core::HRESULT = crate::core::HRESULT(-2147286767i32);
pub const STG_E_EXTANTMARSHALLINGS: crate::core::HRESULT = crate::core::HRESULT(-2147286776i32);
pub const STG_E_FILEALREADYEXISTS: crate::core::HRESULT = crate::core::HRESULT(-2147286960i32);
pub const STG_E_FILENOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147287038i32);
pub const STG_E_FIRMWARE_IMAGE_INVALID: crate::core::HRESULT = crate::core::HRESULT(-2147286519i32);
pub const STG_E_FIRMWARE_SLOT_INVALID: crate::core::HRESULT = crate::core::HRESULT(-2147286520i32);
pub const STG_E_INCOMPLETE: crate::core::HRESULT = crate::core::HRESULT(-2147286527i32);
pub const STG_E_INSUFFICIENTMEMORY: crate::core::HRESULT = crate::core::HRESULT(-2147287032i32);
pub const STG_E_INUSE: crate::core::HRESULT = crate::core::HRESULT(-2147286784i32);
pub const STG_E_INVALIDFLAG: crate::core::HRESULT = crate::core::HRESULT(-2147286785i32);
pub const STG_E_INVALIDFUNCTION: crate::core::HRESULT = crate::core::HRESULT(-2147287039i32);
pub const STG_E_INVALIDHANDLE: crate::core::HRESULT = crate::core::HRESULT(-2147287034i32);
pub const STG_E_INVALIDHEADER: crate::core::HRESULT = crate::core::HRESULT(-2147286789i32);
pub const STG_E_INVALIDNAME: crate::core::HRESULT = crate::core::HRESULT(-2147286788i32);
pub const STG_E_INVALIDPARAMETER: crate::core::HRESULT = crate::core::HRESULT(-2147286953i32);
pub const STG_E_INVALIDPOINTER: crate::core::HRESULT = crate::core::HRESULT(-2147287031i32);
pub const STG_E_LOCKVIOLATION: crate::core::HRESULT = crate::core::HRESULT(-2147287007i32);
pub const STG_E_MEDIUMFULL: crate::core::HRESULT = crate::core::HRESULT(-2147286928i32);
pub const STG_E_NOMOREFILES: crate::core::HRESULT = crate::core::HRESULT(-2147287022i32);
pub const STG_E_NOTCURRENT: crate::core::HRESULT = crate::core::HRESULT(-2147286783i32);
pub const STG_E_NOTFILEBASEDSTORAGE: crate::core::HRESULT = crate::core::HRESULT(-2147286777i32);
pub const STG_E_NOTSIMPLEFORMAT: crate::core::HRESULT = crate::core::HRESULT(-2147286766i32);
pub const STG_E_OLDDLL: crate::core::HRESULT = crate::core::HRESULT(-2147286779i32);
pub const STG_E_OLDFORMAT: crate::core::HRESULT = crate::core::HRESULT(-2147286780i32);
pub const STG_E_PATHNOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147287037i32);
pub const STG_E_PROPSETMISMATCHED: crate::core::HRESULT = crate::core::HRESULT(-2147286800i32);
pub const STG_E_READFAULT: crate::core::HRESULT = crate::core::HRESULT(-2147287010i32);
pub const STG_E_RESETS_EXHAUSTED: crate::core::HRESULT = crate::core::HRESULT(-2147286261i32);
pub const STG_E_REVERTED: crate::core::HRESULT = crate::core::HRESULT(-2147286782i32);
pub const STG_E_SEEKERROR: crate::core::HRESULT = crate::core::HRESULT(-2147287015i32);
pub const STG_E_SHAREREQUIRED: crate::core::HRESULT = crate::core::HRESULT(-2147286778i32);
pub const STG_E_SHAREVIOLATION: crate::core::HRESULT = crate::core::HRESULT(-2147287008i32);
pub const STG_E_STATUS_COPY_PROTECTION_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-2147286267i32);
pub const STG_E_TERMINATED: crate::core::HRESULT = crate::core::HRESULT(-2147286526i32);
pub const STG_E_TOOMANYOPENFILES: crate::core::HRESULT = crate::core::HRESULT(-2147287036i32);
pub const STG_E_UNIMPLEMENTEDFUNCTION: crate::core::HRESULT = crate::core::HRESULT(-2147286786i32);
pub const STG_E_UNKNOWN: crate::core::HRESULT = crate::core::HRESULT(-2147286787i32);
pub const STG_E_WRITEFAULT: crate::core::HRESULT = crate::core::HRESULT(-2147287011i32);
pub const STG_S_BLOCK: crate::core::HRESULT = crate::core::HRESULT(197121i32);
pub const STG_S_CANNOTCONSOLIDATE: crate::core::HRESULT = crate::core::HRESULT(197126i32);
pub const STG_S_CONSOLIDATIONFAILED: crate::core::HRESULT = crate::core::HRESULT(197125i32);
pub const STG_S_CONVERTED: crate::core::HRESULT = crate::core::HRESULT(197120i32);
pub const STG_S_MONITORING: crate::core::HRESULT = crate::core::HRESULT(197123i32);
pub const STG_S_MULTIPLEOPENS: crate::core::HRESULT = crate::core::HRESULT(197124i32);
pub const STG_S_POWER_CYCLE_REQUIRED: crate::core::HRESULT = crate::core::HRESULT(197127i32);
pub const STG_S_RETRYNOW: crate::core::HRESULT = crate::core::HRESULT(197122i32);
pub const STILL_ACTIVE: NTSTATUS = NTSTATUS(259i32);
pub const STORE_ERROR_LICENSE_REVOKED: i32 = 15864i32;
pub const STORE_ERROR_PENDING_COM_TRANSACTION: i32 = 15863i32;
pub const STORE_ERROR_UNLICENSED: i32 = 15861i32;
pub const STORE_ERROR_UNLICENSED_USER: i32 = 15862i32;
pub const STRICT: u32 = 1u32;
pub const SUCCESS: u32 = 0u32;
pub struct SYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
impl ::core::marker::Copy for SYSTEMTIME {}
impl ::core::clone::Clone for SYSTEMTIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEMTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEMTIME")
            .field("wYear", &self.wYear)
            .field("wMonth", &self.wMonth)
            .field("wDayOfWeek", &self.wDayOfWeek)
            .field("wDay", &self.wDay)
            .field("wHour", &self.wHour)
            .field("wMinute", &self.wMinute)
            .field("wSecond", &self.wSecond)
            .field("wMilliseconds", &self.wMilliseconds)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEMTIME {
    fn eq(&self, other: &Self) -> bool {
        self.wYear == other.wYear
            && self.wMonth == other.wMonth
            && self.wDayOfWeek == other.wDayOfWeek
            && self.wDay == other.wDay
            && self.wHour == other.wHour
            && self.wMinute == other.wMinute
            && self.wSecond == other.wSecond
            && self.wMilliseconds == other.wMilliseconds
    }
}
impl ::core::cmp::Eq for SYSTEMTIME {}
impl FromIntoMemory for SYSTEMTIME {
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
pub const S_APPLICATION_ACTIVATION_ERROR_HANDLED_BY_DIALOG: crate::core::HRESULT =
    crate::core::HRESULT(2556505i32);
pub const S_FALSE: crate::core::HRESULT = crate::core::HRESULT(1i32);
pub const S_OK: crate::core::HRESULT = crate::core::HRESULT(0i32);
pub const S_STORE_LAUNCHED_FOR_REMEDIATION: crate::core::HRESULT = crate::core::HRESULT(2556504i32);
pub const TBSIMP_E_BUFFER_TOO_SMALL: crate::core::HRESULT = crate::core::HRESULT(-2144796160i32);
pub const TBSIMP_E_CLEANUP_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144796159i32);
pub const TBSIMP_E_COMMAND_CANCELED: crate::core::HRESULT = crate::core::HRESULT(-2144796149i32);
pub const TBSIMP_E_COMMAND_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144796143i32);
pub const TBSIMP_E_DUPLICATE_VHANDLE: crate::core::HRESULT = crate::core::HRESULT(-2144796154i32);
pub const TBSIMP_E_HASH_BAD_KEY: crate::core::HRESULT = crate::core::HRESULT(-2144796155i32);
pub const TBSIMP_E_HASH_TABLE_FULL: crate::core::HRESULT = crate::core::HRESULT(-2144796138i32);
pub const TBSIMP_E_INVALID_CONTEXT_HANDLE: crate::core::HRESULT =
    crate::core::HRESULT(-2144796158i32);
pub const TBSIMP_E_INVALID_CONTEXT_PARAM: crate::core::HRESULT =
    crate::core::HRESULT(-2144796157i32);
pub const TBSIMP_E_INVALID_OUTPUT_POINTER: crate::core::HRESULT =
    crate::core::HRESULT(-2144796153i32);
pub const TBSIMP_E_INVALID_PARAMETER: crate::core::HRESULT = crate::core::HRESULT(-2144796152i32);
pub const TBSIMP_E_INVALID_RESOURCE: crate::core::HRESULT = crate::core::HRESULT(-2144796140i32);
pub const TBSIMP_E_LIST_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144796146i32);
pub const TBSIMP_E_LIST_NO_MORE_ITEMS: crate::core::HRESULT = crate::core::HRESULT(-2144796147i32);
pub const TBSIMP_E_NOTHING_TO_UNLOAD: crate::core::HRESULT = crate::core::HRESULT(-2144796139i32);
pub const TBSIMP_E_NOT_ENOUGH_SPACE: crate::core::HRESULT = crate::core::HRESULT(-2144796145i32);
pub const TBSIMP_E_NOT_ENOUGH_TPM_CONTEXTS: crate::core::HRESULT =
    crate::core::HRESULT(-2144796144i32);
pub const TBSIMP_E_NO_EVENT_LOG: crate::core::HRESULT = crate::core::HRESULT(-2144796133i32);
pub const TBSIMP_E_OUT_OF_MEMORY: crate::core::HRESULT = crate::core::HRESULT(-2144796148i32);
pub const TBSIMP_E_PPI_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2144796135i32);
pub const TBSIMP_E_RESOURCE_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-2144796141i32);
pub const TBSIMP_E_RPC_INIT_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144796151i32);
pub const TBSIMP_E_SCHEDULER_NOT_RUNNING: crate::core::HRESULT =
    crate::core::HRESULT(-2144796150i32);
pub const TBSIMP_E_TOO_MANY_RESOURCES: crate::core::HRESULT = crate::core::HRESULT(-2144796136i32);
pub const TBSIMP_E_TOO_MANY_TBS_CONTEXTS: crate::core::HRESULT =
    crate::core::HRESULT(-2144796137i32);
pub const TBSIMP_E_TPM_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2144796156i32);
pub const TBSIMP_E_TPM_INCOMPATIBLE: crate::core::HRESULT = crate::core::HRESULT(-2144796134i32);
pub const TBSIMP_E_UNKNOWN_ORDINAL: crate::core::HRESULT = crate::core::HRESULT(-2144796142i32);
pub const TBS_E_ACCESS_DENIED: crate::core::HRESULT = crate::core::HRESULT(-2144845806i32);
pub const TBS_E_BAD_PARAMETER: crate::core::HRESULT = crate::core::HRESULT(-2144845822i32);
pub const TBS_E_BUFFER_TOO_LARGE: crate::core::HRESULT = crate::core::HRESULT(-2144845810i32);
pub const TBS_E_COMMAND_CANCELED: crate::core::HRESULT = crate::core::HRESULT(-2144845811i32);
pub const TBS_E_INSUFFICIENT_BUFFER: crate::core::HRESULT = crate::core::HRESULT(-2144845819i32);
pub const TBS_E_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2144845823i32);
pub const TBS_E_INVALID_CONTEXT: crate::core::HRESULT = crate::core::HRESULT(-2144845820i32);
pub const TBS_E_INVALID_CONTEXT_PARAM: crate::core::HRESULT = crate::core::HRESULT(-2144845817i32);
pub const TBS_E_INVALID_OUTPUT_POINTER: crate::core::HRESULT = crate::core::HRESULT(-2144845821i32);
pub const TBS_E_IOERROR: crate::core::HRESULT = crate::core::HRESULT(-2144845818i32);
pub const TBS_E_NO_EVENT_LOG: crate::core::HRESULT = crate::core::HRESULT(-2144845807i32);
pub const TBS_E_OWNERAUTH_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144845803i32);
pub const TBS_E_PPI_FUNCTION_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144845804i32);
pub const TBS_E_PPI_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2144845812i32);
pub const TBS_E_PROVISIONING_INCOMPLETE: crate::core::HRESULT =
    crate::core::HRESULT(-2144845802i32);
pub const TBS_E_PROVISIONING_NOT_ALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2144845805i32);
pub const TBS_E_SERVICE_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2144845808i32);
pub const TBS_E_SERVICE_NOT_RUNNING: crate::core::HRESULT = crate::core::HRESULT(-2144845816i32);
pub const TBS_E_SERVICE_START_PENDING: crate::core::HRESULT = crate::core::HRESULT(-2144845813i32);
pub const TBS_E_TOO_MANY_RESOURCES: crate::core::HRESULT = crate::core::HRESULT(-2144845814i32);
pub const TBS_E_TOO_MANY_TBS_CONTEXTS: crate::core::HRESULT = crate::core::HRESULT(-2144845815i32);
pub const TBS_E_TPM_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144845809i32);
pub const TPC_E_INITIALIZE_FAIL: crate::core::HRESULT = crate::core::HRESULT(-2147220957i32);
pub const TPC_E_INVALID_CONFIGURATION: crate::core::HRESULT = crate::core::HRESULT(-2147220935i32);
pub const TPC_E_INVALID_DATA_FROM_RECOGNIZER: crate::core::HRESULT =
    crate::core::HRESULT(-2147220934i32);
pub const TPC_E_INVALID_INPUT_RECT: crate::core::HRESULT = crate::core::HRESULT(-2147220967i32);
pub const TPC_E_INVALID_PACKET_DESCRIPTION: crate::core::HRESULT =
    crate::core::HRESULT(-2147220941i32);
pub const TPC_E_INVALID_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-2147220927i32);
pub const TPC_E_INVALID_RIGHTS: crate::core::HRESULT = crate::core::HRESULT(-2147220938i32);
pub const TPC_E_INVALID_STROKE: crate::core::HRESULT = crate::core::HRESULT(-2147220958i32);
pub const TPC_E_NOT_RELEVANT: crate::core::HRESULT = crate::core::HRESULT(-2147220942i32);
pub const TPC_E_NO_DEFAULT_TABLET: crate::core::HRESULT = crate::core::HRESULT(-2147220974i32);
pub const TPC_E_OUT_OF_ORDER_CALL: crate::core::HRESULT = crate::core::HRESULT(-2147220937i32);
pub const TPC_E_QUEUE_FULL: crate::core::HRESULT = crate::core::HRESULT(-2147220936i32);
pub const TPC_E_RECOGNIZER_NOT_REGISTERED: crate::core::HRESULT =
    crate::core::HRESULT(-2147220939i32);
pub const TPC_E_UNKNOWN_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-2147220965i32);
pub const TPC_S_INTERRUPTED: crate::core::HRESULT = crate::core::HRESULT(262739i32);
pub const TPC_S_NO_DATA_TO_PROCESS: crate::core::HRESULT = crate::core::HRESULT(262740i32);
pub const TPC_S_TRUNCATED: crate::core::HRESULT = crate::core::HRESULT(262738i32);
pub const TPMAPI_E_ACCESS_DENIED: crate::core::HRESULT = crate::core::HRESULT(-2144796408i32);
pub const TPMAPI_E_AUTHORIZATION_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2144796407i32);
pub const TPMAPI_E_AUTHORIZATION_REVOKED: crate::core::HRESULT =
    crate::core::HRESULT(-2144796378i32);
pub const TPMAPI_E_AUTHORIZING_KEY_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144796376i32);
pub const TPMAPI_E_BUFFER_TOO_SMALL: crate::core::HRESULT = crate::core::HRESULT(-2144796410i32);
pub const TPMAPI_E_EMPTY_TCG_LOG: crate::core::HRESULT = crate::core::HRESULT(-2144796390i32);
pub const TPMAPI_E_ENCRYPTION_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144796400i32);
pub const TPMAPI_E_ENDORSEMENT_AUTH_NOT_NULL: crate::core::HRESULT =
    crate::core::HRESULT(-2144796379i32);
pub const TPMAPI_E_FIPS_RNG_CHECK_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2144796391i32);
pub const TPMAPI_E_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2144796409i32);
pub const TPMAPI_E_INVALID_AUTHORIZATION_SIGNATURE: crate::core::HRESULT =
    crate::core::HRESULT(-2144796375i32);
pub const TPMAPI_E_INVALID_CONTEXT_HANDLE: crate::core::HRESULT =
    crate::core::HRESULT(-2144796406i32);
pub const TPMAPI_E_INVALID_CONTEXT_PARAMS: crate::core::HRESULT =
    crate::core::HRESULT(-2144796395i32);
pub const TPMAPI_E_INVALID_DELEGATE_BLOB: crate::core::HRESULT =
    crate::core::HRESULT(-2144796396i32);
pub const TPMAPI_E_INVALID_ENCODING: crate::core::HRESULT = crate::core::HRESULT(-2144796402i32);
pub const TPMAPI_E_INVALID_KEY_BLOB: crate::core::HRESULT = crate::core::HRESULT(-2144796394i32);
pub const TPMAPI_E_INVALID_KEY_PARAMS: crate::core::HRESULT = crate::core::HRESULT(-2144796399i32);
pub const TPMAPI_E_INVALID_KEY_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2144796401i32);
pub const TPMAPI_E_INVALID_MIGRATION_AUTHORIZATION_BLOB: crate::core::HRESULT =
    crate::core::HRESULT(-2144796398i32);
pub const TPMAPI_E_INVALID_OUTPUT_POINTER: crate::core::HRESULT =
    crate::core::HRESULT(-2144796413i32);
pub const TPMAPI_E_INVALID_OWNER_AUTH: crate::core::HRESULT = crate::core::HRESULT(-2144796392i32);
pub const TPMAPI_E_INVALID_PARAMETER: crate::core::HRESULT = crate::core::HRESULT(-2144796412i32);
pub const TPMAPI_E_INVALID_PCR_DATA: crate::core::HRESULT = crate::core::HRESULT(-2144796393i32);
pub const TPMAPI_E_INVALID_PCR_INDEX: crate::core::HRESULT = crate::core::HRESULT(-2144796397i32);
pub const TPMAPI_E_INVALID_POLICYAUTH_BLOB_TYPE: crate::core::HRESULT =
    crate::core::HRESULT(-2144796370i32);
pub const TPMAPI_E_INVALID_STATE: crate::core::HRESULT = crate::core::HRESULT(-2144796416i32);
pub const TPMAPI_E_INVALID_TCG_LOG_ENTRY: crate::core::HRESULT =
    crate::core::HRESULT(-2144796389i32);
pub const TPMAPI_E_INVALID_TPM_VERSION: crate::core::HRESULT = crate::core::HRESULT(-2144796371i32);
pub const TPMAPI_E_MALFORMED_AUTHORIZATION_KEY: crate::core::HRESULT =
    crate::core::HRESULT(-2144796377i32);
pub const TPMAPI_E_MALFORMED_AUTHORIZATION_OTHER: crate::core::HRESULT =
    crate::core::HRESULT(-2144796373i32);
pub const TPMAPI_E_MALFORMED_AUTHORIZATION_POLICY: crate::core::HRESULT =
    crate::core::HRESULT(-2144796374i32);
pub const TPMAPI_E_MESSAGE_TOO_LARGE: crate::core::HRESULT = crate::core::HRESULT(-2144796403i32);
pub const TPMAPI_E_NOT_ENOUGH_DATA: crate::core::HRESULT = crate::core::HRESULT(-2144796415i32);
pub const TPMAPI_E_NO_AUTHORIZATION_CHAIN_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2144796382i32);
pub const TPMAPI_E_NV_BITS_NOT_DEFINED: crate::core::HRESULT = crate::core::HRESULT(-2144796385i32);
pub const TPMAPI_E_NV_BITS_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-2144796384i32);
pub const TPMAPI_E_OUT_OF_MEMORY: crate::core::HRESULT = crate::core::HRESULT(-2144796411i32);
pub const TPMAPI_E_OWNER_AUTH_NOT_NULL: crate::core::HRESULT = crate::core::HRESULT(-2144796380i32);
pub const TPMAPI_E_POLICY_DENIES_OPERATION: crate::core::HRESULT =
    crate::core::HRESULT(-2144796386i32);
pub const TPMAPI_E_SEALING_KEY_CHANGED: crate::core::HRESULT = crate::core::HRESULT(-2144796372i32);
pub const TPMAPI_E_SEALING_KEY_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2144796383i32);
pub const TPMAPI_E_SVN_COUNTER_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2144796381i32);
pub const TPMAPI_E_TBS_COMMUNICATION_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-2144796405i32);
pub const TPMAPI_E_TCG_INVALID_DIGEST_ENTRY: crate::core::HRESULT =
    crate::core::HRESULT(-2144796387i32);
pub const TPMAPI_E_TCG_SEPARATOR_ABSENT: crate::core::HRESULT =
    crate::core::HRESULT(-2144796388i32);
pub const TPMAPI_E_TOO_MUCH_DATA: crate::core::HRESULT = crate::core::HRESULT(-2144796414i32);
pub const TPMAPI_E_TPM_COMMAND_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2144796404i32);
pub const TPM_20_E_ASYMMETRIC: crate::core::HRESULT = crate::core::HRESULT(-2144862079i32);
pub const TPM_20_E_ATTRIBUTES: crate::core::HRESULT = crate::core::HRESULT(-2144862078i32);
pub const TPM_20_E_AUTHSIZE: crate::core::HRESULT = crate::core::HRESULT(-2144861884i32);
pub const TPM_20_E_AUTH_CONTEXT: crate::core::HRESULT = crate::core::HRESULT(-2144861883i32);
pub const TPM_20_E_AUTH_FAIL: crate::core::HRESULT = crate::core::HRESULT(-2144862066i32);
pub const TPM_20_E_AUTH_MISSING: crate::core::HRESULT = crate::core::HRESULT(-2144861915i32);
pub const TPM_20_E_AUTH_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2144861916i32);
pub const TPM_20_E_AUTH_UNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2144861905i32);
pub const TPM_20_E_BAD_AUTH: crate::core::HRESULT = crate::core::HRESULT(-2144862046i32);
pub const TPM_20_E_BAD_CONTEXT: crate::core::HRESULT = crate::core::HRESULT(-2144861872i32);
pub const TPM_20_E_BINDING: crate::core::HRESULT = crate::core::HRESULT(-2144862043i32);
pub const TPM_20_E_CANCELED: crate::core::HRESULT = crate::core::HRESULT(-2144859895i32);
pub const TPM_20_E_COMMAND_CODE: crate::core::HRESULT = crate::core::HRESULT(-2144861885i32);
pub const TPM_20_E_COMMAND_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2144861886i32);
pub const TPM_20_E_CONTEXT_GAP: crate::core::HRESULT = crate::core::HRESULT(-2144859903i32);
pub const TPM_20_E_CPHASH: crate::core::HRESULT = crate::core::HRESULT(-2144861871i32);
pub const TPM_20_E_CURVE: crate::core::HRESULT = crate::core::HRESULT(-2144862042i32);
pub const TPM_20_E_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2144861920i32);
pub const TPM_20_E_ECC_CURVE: crate::core::HRESULT = crate::core::HRESULT(-2144861917i32);
pub const TPM_20_E_ECC_POINT: crate::core::HRESULT = crate::core::HRESULT(-2144862041i32);
pub const TPM_20_E_EXCLUSIVE: crate::core::HRESULT = crate::core::HRESULT(-2144861919i32);
pub const TPM_20_E_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-2144862045i32);
pub const TPM_20_E_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2144861951i32);
pub const TPM_20_E_HANDLE: crate::core::HRESULT = crate::core::HRESULT(-2144862069i32);
pub const TPM_20_E_HASH: crate::core::HRESULT = crate::core::HRESULT(-2144862077i32);
pub const TPM_20_E_HIERARCHY: crate::core::HRESULT = crate::core::HRESULT(-2144862075i32);
pub const TPM_20_E_HMAC: crate::core::HRESULT = crate::core::HRESULT(-2144861927i32);
pub const TPM_20_E_INITIALIZE: crate::core::HRESULT = crate::core::HRESULT(-2144861952i32);
pub const TPM_20_E_INSUFFICIENT: crate::core::HRESULT = crate::core::HRESULT(-2144862054i32);
pub const TPM_20_E_INTEGRITY: crate::core::HRESULT = crate::core::HRESULT(-2144862049i32);
pub const TPM_20_E_KDF: crate::core::HRESULT = crate::core::HRESULT(-2144862068i32);
pub const TPM_20_E_KEY: crate::core::HRESULT = crate::core::HRESULT(-2144862052i32);
pub const TPM_20_E_KEY_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2144862073i32);
pub const TPM_20_E_LOCALITY: crate::core::HRESULT = crate::core::HRESULT(-2144859897i32);
pub const TPM_20_E_LOCKOUT: crate::core::HRESULT = crate::core::HRESULT(-2144859871i32);
pub const TPM_20_E_MEMORY: crate::core::HRESULT = crate::core::HRESULT(-2144859900i32);
pub const TPM_20_E_MGF: crate::core::HRESULT = crate::core::HRESULT(-2144862072i32);
pub const TPM_20_E_MODE: crate::core::HRESULT = crate::core::HRESULT(-2144862071i32);
pub const TPM_20_E_NEEDS_TEST: crate::core::HRESULT = crate::core::HRESULT(-2144861869i32);
pub const TPM_20_E_NONCE: crate::core::HRESULT = crate::core::HRESULT(-2144862065i32);
pub const TPM_20_E_NO_RESULT: crate::core::HRESULT = crate::core::HRESULT(-2144861868i32);
pub const TPM_20_E_NV_AUTHORIZATION: crate::core::HRESULT = crate::core::HRESULT(-2144861879i32);
pub const TPM_20_E_NV_DEFINED: crate::core::HRESULT = crate::core::HRESULT(-2144861876i32);
pub const TPM_20_E_NV_LOCKED: crate::core::HRESULT = crate::core::HRESULT(-2144861880i32);
pub const TPM_20_E_NV_RANGE: crate::core::HRESULT = crate::core::HRESULT(-2144861882i32);
pub const TPM_20_E_NV_RATE: crate::core::HRESULT = crate::core::HRESULT(-2144859872i32);
pub const TPM_20_E_NV_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2144861881i32);
pub const TPM_20_E_NV_SPACE: crate::core::HRESULT = crate::core::HRESULT(-2144861877i32);
pub const TPM_20_E_NV_UNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2144859869i32);
pub const TPM_20_E_NV_UNINITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2144861878i32);
pub const TPM_20_E_OBJECT_HANDLES: crate::core::HRESULT = crate::core::HRESULT(-2144859898i32);
pub const TPM_20_E_OBJECT_MEMORY: crate::core::HRESULT = crate::core::HRESULT(-2144859902i32);
pub const TPM_20_E_PARENT: crate::core::HRESULT = crate::core::HRESULT(-2144861870i32);
pub const TPM_20_E_PCR: crate::core::HRESULT = crate::core::HRESULT(-2144861913i32);
pub const TPM_20_E_PCR_CHANGED: crate::core::HRESULT = crate::core::HRESULT(-2144861912i32);
pub const TPM_20_E_POLICY: crate::core::HRESULT = crate::core::HRESULT(-2144861914i32);
pub const TPM_20_E_POLICY_CC: crate::core::HRESULT = crate::core::HRESULT(-2144862044i32);
pub const TPM_20_E_POLICY_FAIL: crate::core::HRESULT = crate::core::HRESULT(-2144862051i32);
pub const TPM_20_E_PP: crate::core::HRESULT = crate::core::HRESULT(-2144862064i32);
pub const TPM_20_E_PRIVATE: crate::core::HRESULT = crate::core::HRESULT(-2144861941i32);
pub const TPM_20_E_RANGE: crate::core::HRESULT = crate::core::HRESULT(-2144862067i32);
pub const TPM_20_E_REBOOT: crate::core::HRESULT = crate::core::HRESULT(-2144861904i32);
pub const TPM_20_E_RESERVED_BITS: crate::core::HRESULT = crate::core::HRESULT(-2144862047i32);
pub const TPM_20_E_RETRY: crate::core::HRESULT = crate::core::HRESULT(-2144859870i32);
pub const TPM_20_E_SCHEME: crate::core::HRESULT = crate::core::HRESULT(-2144862062i32);
pub const TPM_20_E_SELECTOR: crate::core::HRESULT = crate::core::HRESULT(-2144862056i32);
pub const TPM_20_E_SENSITIVE: crate::core::HRESULT = crate::core::HRESULT(-2144861867i32);
pub const TPM_20_E_SEQUENCE: crate::core::HRESULT = crate::core::HRESULT(-2144861949i32);
pub const TPM_20_E_SESSION_HANDLES: crate::core::HRESULT = crate::core::HRESULT(-2144859899i32);
pub const TPM_20_E_SESSION_MEMORY: crate::core::HRESULT = crate::core::HRESULT(-2144859901i32);
pub const TPM_20_E_SIGNATURE: crate::core::HRESULT = crate::core::HRESULT(-2144862053i32);
pub const TPM_20_E_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2144862059i32);
pub const TPM_20_E_SYMMETRIC: crate::core::HRESULT = crate::core::HRESULT(-2144862058i32);
pub const TPM_20_E_TAG: crate::core::HRESULT = crate::core::HRESULT(-2144862057i32);
pub const TPM_20_E_TESTING: crate::core::HRESULT = crate::core::HRESULT(-2144859894i32);
pub const TPM_20_E_TICKET: crate::core::HRESULT = crate::core::HRESULT(-2144862048i32);
pub const TPM_20_E_TOO_MANY_CONTEXTS: crate::core::HRESULT = crate::core::HRESULT(-2144861906i32);
pub const TPM_20_E_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2144862070i32);
pub const TPM_20_E_UNBALANCED: crate::core::HRESULT = crate::core::HRESULT(-2144861903i32);
pub const TPM_20_E_UPGRADE: crate::core::HRESULT = crate::core::HRESULT(-2144861907i32);
pub const TPM_20_E_VALUE: crate::core::HRESULT = crate::core::HRESULT(-2144862076i32);
pub const TPM_20_E_YIELDED: crate::core::HRESULT = crate::core::HRESULT(-2144859896i32);
pub const TPM_E_AREA_LOCKED: crate::core::HRESULT = crate::core::HRESULT(-2144862148i32);
pub const TPM_E_ATTESTATION_CHALLENGE_NOT_SET: crate::core::HRESULT =
    crate::core::HRESULT(-2144795630i32);
pub const TPM_E_AUDITFAILURE: crate::core::HRESULT = crate::core::HRESULT(-2144862204i32);
pub const TPM_E_AUDITFAIL_SUCCESSFUL: crate::core::HRESULT = crate::core::HRESULT(-2144862159i32);
pub const TPM_E_AUDITFAIL_UNSUCCESSFUL: crate::core::HRESULT = crate::core::HRESULT(-2144862160i32);
pub const TPM_E_AUTH2FAIL: crate::core::HRESULT = crate::core::HRESULT(-2144862179i32);
pub const TPM_E_AUTHFAIL: crate::core::HRESULT = crate::core::HRESULT(-2144862207i32);
pub const TPM_E_AUTH_CONFLICT: crate::core::HRESULT = crate::core::HRESULT(-2144862149i32);
pub const TPM_E_BADCONTEXT: crate::core::HRESULT = crate::core::HRESULT(-2144862118i32);
pub const TPM_E_BADINDEX: crate::core::HRESULT = crate::core::HRESULT(-2144862206i32);
pub const TPM_E_BADTAG: crate::core::HRESULT = crate::core::HRESULT(-2144862178i32);
pub const TPM_E_BAD_ATTRIBUTES: crate::core::HRESULT = crate::core::HRESULT(-2144862142i32);
pub const TPM_E_BAD_COUNTER: crate::core::HRESULT = crate::core::HRESULT(-2144862139i32);
pub const TPM_E_BAD_DATASIZE: crate::core::HRESULT = crate::core::HRESULT(-2144862165i32);
pub const TPM_E_BAD_DELEGATE: crate::core::HRESULT = crate::core::HRESULT(-2144862119i32);
pub const TPM_E_BAD_HANDLE: crate::core::HRESULT = crate::core::HRESULT(-2144862120i32);
pub const TPM_E_BAD_KEY_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-2144862168i32);
pub const TPM_E_BAD_LOCALITY: crate::core::HRESULT = crate::core::HRESULT(-2144862147i32);
pub const TPM_E_BAD_MIGRATION: crate::core::HRESULT = crate::core::HRESULT(-2144862167i32);
pub const TPM_E_BAD_MODE: crate::core::HRESULT = crate::core::HRESULT(-2144862164i32);
pub const TPM_E_BAD_ORDINAL: crate::core::HRESULT = crate::core::HRESULT(-2144862198i32);
pub const TPM_E_BAD_PARAMETER: crate::core::HRESULT = crate::core::HRESULT(-2144862205i32);
pub const TPM_E_BAD_PARAM_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2144862183i32);
pub const TPM_E_BAD_PRESENCE: crate::core::HRESULT = crate::core::HRESULT(-2144862163i32);
pub const TPM_E_BAD_SCHEME: crate::core::HRESULT = crate::core::HRESULT(-2144862166i32);
pub const TPM_E_BAD_SIGNATURE: crate::core::HRESULT = crate::core::HRESULT(-2144862110i32);
pub const TPM_E_BAD_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2144862156i32);
pub const TPM_E_BAD_VERSION: crate::core::HRESULT = crate::core::HRESULT(-2144862162i32);
pub const TPM_E_BUFFER_LENGTH_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2144795618i32);
pub const TPM_E_CLAIM_TYPE_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144795620i32);
pub const TPM_E_CLEAR_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2144862203i32);
pub const TPM_E_COMMAND_BLOCKED: crate::core::HRESULT = crate::core::HRESULT(-2144861184i32);
pub const TPM_E_CONTEXT_GAP: crate::core::HRESULT = crate::core::HRESULT(-2144862137i32);
pub const TPM_E_DAA_INPUT_DATA0: crate::core::HRESULT = crate::core::HRESULT(-2144862127i32);
pub const TPM_E_DAA_INPUT_DATA1: crate::core::HRESULT = crate::core::HRESULT(-2144862126i32);
pub const TPM_E_DAA_ISSUER_SETTINGS: crate::core::HRESULT = crate::core::HRESULT(-2144862125i32);
pub const TPM_E_DAA_ISSUER_VALIDITY: crate::core::HRESULT = crate::core::HRESULT(-2144862122i32);
pub const TPM_E_DAA_RESOURCES: crate::core::HRESULT = crate::core::HRESULT(-2144862128i32);
pub const TPM_E_DAA_STAGE: crate::core::HRESULT = crate::core::HRESULT(-2144862123i32);
pub const TPM_E_DAA_TPM_SETTINGS: crate::core::HRESULT = crate::core::HRESULT(-2144862124i32);
pub const TPM_E_DAA_WRONG_W: crate::core::HRESULT = crate::core::HRESULT(-2144862121i32);
pub const TPM_E_DEACTIVATED: crate::core::HRESULT = crate::core::HRESULT(-2144862202i32);
pub const TPM_E_DECRYPT_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2144862175i32);
pub const TPM_E_DEFEND_LOCK_RUNNING: crate::core::HRESULT = crate::core::HRESULT(-2144860157i32);
pub const TPM_E_DELEGATE_ADMIN: crate::core::HRESULT = crate::core::HRESULT(-2144862131i32);
pub const TPM_E_DELEGATE_FAMILY: crate::core::HRESULT = crate::core::HRESULT(-2144862132i32);
pub const TPM_E_DELEGATE_LOCK: crate::core::HRESULT = crate::core::HRESULT(-2144862133i32);
pub const TPM_E_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2144862201i32);
pub const TPM_E_DISABLED_CMD: crate::core::HRESULT = crate::core::HRESULT(-2144862200i32);
pub const TPM_E_DOING_SELFTEST: crate::core::HRESULT = crate::core::HRESULT(-2144860158i32);
pub const TPM_E_DUPLICATE_VHANDLE: crate::core::HRESULT = crate::core::HRESULT(-2144861182i32);
pub const TPM_E_EMBEDDED_COMMAND_BLOCKED: crate::core::HRESULT =
    crate::core::HRESULT(-2144861181i32);
pub const TPM_E_EMBEDDED_COMMAND_UNSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144861180i32);
pub const TPM_E_ENCRYPT_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2144862176i32);
pub const TPM_E_ERROR_MASK: crate::core::HRESULT = crate::core::HRESULT(-2144862208i32);
pub const TPM_E_FAIL: crate::core::HRESULT = crate::core::HRESULT(-2144862199i32);
pub const TPM_E_FAILEDSELFTEST: crate::core::HRESULT = crate::core::HRESULT(-2144862180i32);
pub const TPM_E_FAMILYCOUNT: crate::core::HRESULT = crate::core::HRESULT(-2144862144i32);
pub const TPM_E_INAPPROPRIATE_ENC: crate::core::HRESULT = crate::core::HRESULT(-2144862194i32);
pub const TPM_E_INAPPROPRIATE_SIG: crate::core::HRESULT = crate::core::HRESULT(-2144862169i32);
pub const TPM_E_INSTALL_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2144862197i32);
pub const TPM_E_INVALID_AUTHHANDLE: crate::core::HRESULT = crate::core::HRESULT(-2144862174i32);
pub const TPM_E_INVALID_FAMILY: crate::core::HRESULT = crate::core::HRESULT(-2144862153i32);
pub const TPM_E_INVALID_HANDLE: crate::core::HRESULT = crate::core::HRESULT(-2144861183i32);
pub const TPM_E_INVALID_KEYHANDLE: crate::core::HRESULT = crate::core::HRESULT(-2144862196i32);
pub const TPM_E_INVALID_KEYUSAGE: crate::core::HRESULT = crate::core::HRESULT(-2144862172i32);
pub const TPM_E_INVALID_OWNER_AUTH: crate::core::HRESULT = crate::core::HRESULT(-2144795135i32);
pub const TPM_E_INVALID_PCR_INFO: crate::core::HRESULT = crate::core::HRESULT(-2144862192i32);
pub const TPM_E_INVALID_POSTINIT: crate::core::HRESULT = crate::core::HRESULT(-2144862170i32);
pub const TPM_E_INVALID_RESOURCE: crate::core::HRESULT = crate::core::HRESULT(-2144862155i32);
pub const TPM_E_INVALID_STRUCTURE: crate::core::HRESULT = crate::core::HRESULT(-2144862141i32);
pub const TPM_E_IOERROR: crate::core::HRESULT = crate::core::HRESULT(-2144862177i32);
pub const TPM_E_KEYNOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2144862195i32);
pub const TPM_E_KEY_ALREADY_FINALIZED: crate::core::HRESULT = crate::core::HRESULT(-2144795628i32);
pub const TPM_E_KEY_NOTSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2144862150i32);
pub const TPM_E_KEY_NOT_AUTHENTICATED: crate::core::HRESULT = crate::core::HRESULT(-2144795624i32);
pub const TPM_E_KEY_NOT_FINALIZED: crate::core::HRESULT = crate::core::HRESULT(-2144795631i32);
pub const TPM_E_KEY_NOT_LOADED: crate::core::HRESULT = crate::core::HRESULT(-2144795633i32);
pub const TPM_E_KEY_NOT_SIGNING_KEY: crate::core::HRESULT = crate::core::HRESULT(-2144795622i32);
pub const TPM_E_KEY_OWNER_CONTROL: crate::core::HRESULT = crate::core::HRESULT(-2144862140i32);
pub const TPM_E_KEY_USAGE_POLICY_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2144795626i32);
pub const TPM_E_KEY_USAGE_POLICY_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144795627i32);
pub const TPM_E_LOCKED_OUT: crate::core::HRESULT = crate::core::HRESULT(-2144795621i32);
pub const TPM_E_MAXNVWRITES: crate::core::HRESULT = crate::core::HRESULT(-2144862136i32);
pub const TPM_E_MA_AUTHORITY: crate::core::HRESULT = crate::core::HRESULT(-2144862113i32);
pub const TPM_E_MA_DESTINATION: crate::core::HRESULT = crate::core::HRESULT(-2144862115i32);
pub const TPM_E_MA_SOURCE: crate::core::HRESULT = crate::core::HRESULT(-2144862114i32);
pub const TPM_E_MA_TICKET_SIGNATURE: crate::core::HRESULT = crate::core::HRESULT(-2144862116i32);
pub const TPM_E_MIGRATEFAIL: crate::core::HRESULT = crate::core::HRESULT(-2144862193i32);
pub const TPM_E_NEEDS_SELFTEST: crate::core::HRESULT = crate::core::HRESULT(-2144860159i32);
pub const TPM_E_NOCONTEXTSPACE: crate::core::HRESULT = crate::core::HRESULT(-2144862109i32);
pub const TPM_E_NOOPERATOR: crate::core::HRESULT = crate::core::HRESULT(-2144862135i32);
pub const TPM_E_NOSPACE: crate::core::HRESULT = crate::core::HRESULT(-2144862191i32);
pub const TPM_E_NOSRK: crate::core::HRESULT = crate::core::HRESULT(-2144862190i32);
pub const TPM_E_NOTFIPS: crate::core::HRESULT = crate::core::HRESULT(-2144862154i32);
pub const TPM_E_NOTLOCAL: crate::core::HRESULT = crate::core::HRESULT(-2144862157i32);
pub const TPM_E_NOTRESETABLE: crate::core::HRESULT = crate::core::HRESULT(-2144862158i32);
pub const TPM_E_NOTSEALED_BLOB: crate::core::HRESULT = crate::core::HRESULT(-2144862189i32);
pub const TPM_E_NOT_FULLWRITE: crate::core::HRESULT = crate::core::HRESULT(-2144862138i32);
pub const TPM_E_NOT_PCR_BOUND: crate::core::HRESULT = crate::core::HRESULT(-2144795629i32);
pub const TPM_E_NO_ENDORSEMENT: crate::core::HRESULT = crate::core::HRESULT(-2144862173i32);
pub const TPM_E_NO_KEY_CERTIFICATION: crate::core::HRESULT = crate::core::HRESULT(-2144795632i32);
pub const TPM_E_NO_NV_PERMISSION: crate::core::HRESULT = crate::core::HRESULT(-2144862152i32);
pub const TPM_E_NO_WRAP_TRANSPORT: crate::core::HRESULT = crate::core::HRESULT(-2144862161i32);
pub const TPM_E_OWNER_CONTROL: crate::core::HRESULT = crate::core::HRESULT(-2144862129i32);
pub const TPM_E_OWNER_SET: crate::core::HRESULT = crate::core::HRESULT(-2144862188i32);
pub const TPM_E_PCP_AUTHENTICATION_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2144795640i32);
pub const TPM_E_PCP_AUTHENTICATION_IGNORED: crate::core::HRESULT =
    crate::core::HRESULT(-2144795639i32);
pub const TPM_E_PCP_BUFFER_TOO_SMALL: crate::core::HRESULT = crate::core::HRESULT(-2144795642i32);
pub const TPM_E_PCP_DEVICE_NOT_READY: crate::core::HRESULT = crate::core::HRESULT(-2144795647i32);
pub const TPM_E_PCP_ERROR_MASK: crate::core::HRESULT = crate::core::HRESULT(-2144795648i32);
pub const TPM_E_PCP_FLAG_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2144795644i32);
pub const TPM_E_PCP_IFX_RSA_KEY_CREATION_BLOCKED: crate::core::HRESULT =
    crate::core::HRESULT(-2144795617i32);
pub const TPM_E_PCP_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2144795641i32);
pub const TPM_E_PCP_INVALID_HANDLE: crate::core::HRESULT = crate::core::HRESULT(-2144795646i32);
pub const TPM_E_PCP_INVALID_PARAMETER: crate::core::HRESULT = crate::core::HRESULT(-2144795645i32);
pub const TPM_E_PCP_KEY_HANDLE_INVALIDATED: crate::core::HRESULT =
    crate::core::HRESULT(-2144795614i32);
pub const TPM_E_PCP_KEY_NOT_AIK: crate::core::HRESULT = crate::core::HRESULT(-2144795623i32);
pub const TPM_E_PCP_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2144795643i32);
pub const TPM_E_PCP_PLATFORM_CLAIM_MAY_BE_OUTDATED: crate::core::HRESULT =
    crate::core::HRESULT(1076429860i32);
pub const TPM_E_PCP_PLATFORM_CLAIM_OUTDATED: crate::core::HRESULT =
    crate::core::HRESULT(1076429861i32);
pub const TPM_E_PCP_PLATFORM_CLAIM_REBOOT: crate::core::HRESULT =
    crate::core::HRESULT(1076429862i32);
pub const TPM_E_PCP_POLICY_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144795638i32);
pub const TPM_E_PCP_PROFILE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2144795637i32);
pub const TPM_E_PCP_RAW_POLICY_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144795615i32);
pub const TPM_E_PCP_TICKET_MISSING: crate::core::HRESULT = crate::core::HRESULT(-2144795616i32);
pub const TPM_E_PCP_UNSUPPORTED_PSS_SALT: crate::core::HRESULT =
    crate::core::HRESULT(1076429859i32);
pub const TPM_E_PCP_VALIDATION_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144795636i32);
pub const TPM_E_PCP_WRONG_PARENT: crate::core::HRESULT = crate::core::HRESULT(-2144795634i32);
pub const TPM_E_PERMANENTEK: crate::core::HRESULT = crate::core::HRESULT(-2144862111i32);
pub const TPM_E_PER_NOWRITE: crate::core::HRESULT = crate::core::HRESULT(-2144862145i32);
pub const TPM_E_PPI_ACPI_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2144795904i32);
pub const TPM_E_PPI_BIOS_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2144795902i32);
pub const TPM_E_PPI_BLOCKED_IN_BIOS: crate::core::HRESULT = crate::core::HRESULT(-2144795900i32);
pub const TPM_E_PPI_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2144795901i32);
pub const TPM_E_PPI_USER_ABORT: crate::core::HRESULT = crate::core::HRESULT(-2144795903i32);
pub const TPM_E_PROVISIONING_INCOMPLETE: crate::core::HRESULT =
    crate::core::HRESULT(-2144795136i32);
pub const TPM_E_READ_ONLY: crate::core::HRESULT = crate::core::HRESULT(-2144862146i32);
pub const TPM_E_REQUIRES_SIGN: crate::core::HRESULT = crate::core::HRESULT(-2144862151i32);
pub const TPM_E_RESOURCEMISSING: crate::core::HRESULT = crate::core::HRESULT(-2144862134i32);
pub const TPM_E_RESOURCES: crate::core::HRESULT = crate::core::HRESULT(-2144862187i32);
pub const TPM_E_RETRY: crate::core::HRESULT = crate::core::HRESULT(-2144860160i32);
pub const TPM_E_SHA_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2144862181i32);
pub const TPM_E_SHA_THREAD: crate::core::HRESULT = crate::core::HRESULT(-2144862182i32);
pub const TPM_E_SHORTRANDOM: crate::core::HRESULT = crate::core::HRESULT(-2144862186i32);
pub const TPM_E_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2144862185i32);
pub const TPM_E_SOFT_KEY_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2144795625i32);
pub const TPM_E_TOOMANYCONTEXTS: crate::core::HRESULT = crate::core::HRESULT(-2144862117i32);
pub const TPM_E_TOO_MUCH_DATA: crate::core::HRESULT = crate::core::HRESULT(-2144795134i32);
pub const TPM_E_TRANSPORT_NOTEXCLUSIVE: crate::core::HRESULT = crate::core::HRESULT(-2144862130i32);
pub const TPM_E_VERSION_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2144795619i32);
pub const TPM_E_WRITE_LOCKED: crate::core::HRESULT = crate::core::HRESULT(-2144862143i32);
pub const TPM_E_WRONGPCRVAL: crate::core::HRESULT = crate::core::HRESULT(-2144862184i32);
pub const TPM_E_WRONG_ENTITYTYPE: crate::core::HRESULT = crate::core::HRESULT(-2144862171i32);
pub const TPM_E_ZERO_EXHAUST_ENABLED: crate::core::HRESULT = crate::core::HRESULT(-2144795392i32);
pub const TRUST_E_ACTION_UNKNOWN: crate::core::HRESULT = crate::core::HRESULT(-2146762750i32);
pub const TRUST_E_BAD_DIGEST: crate::core::HRESULT = crate::core::HRESULT(-2146869232i32);
pub const TRUST_E_BASIC_CONSTRAINTS: crate::core::HRESULT = crate::core::HRESULT(-2146869223i32);
pub const TRUST_E_CERT_SIGNATURE: crate::core::HRESULT = crate::core::HRESULT(-2146869244i32);
pub const TRUST_E_COUNTER_SIGNER: crate::core::HRESULT = crate::core::HRESULT(-2146869245i32);
pub const TRUST_E_EXPLICIT_DISTRUST: crate::core::HRESULT = crate::core::HRESULT(-2146762479i32);
pub const TRUST_E_FAIL: crate::core::HRESULT = crate::core::HRESULT(-2146762485i32);
pub const TRUST_E_FINANCIAL_CRITERIA: crate::core::HRESULT = crate::core::HRESULT(-2146869218i32);
pub const TRUST_E_MALFORMED_SIGNATURE: crate::core::HRESULT = crate::core::HRESULT(-2146869231i32);
pub const TRUST_E_NOSIGNATURE: crate::core::HRESULT = crate::core::HRESULT(-2146762496i32);
pub const TRUST_E_NO_SIGNER_CERT: crate::core::HRESULT = crate::core::HRESULT(-2146869246i32);
pub const TRUST_E_PROVIDER_UNKNOWN: crate::core::HRESULT = crate::core::HRESULT(-2146762751i32);
pub const TRUST_E_SUBJECT_FORM_UNKNOWN: crate::core::HRESULT = crate::core::HRESULT(-2146762749i32);
pub const TRUST_E_SUBJECT_NOT_TRUSTED: crate::core::HRESULT = crate::core::HRESULT(-2146762748i32);
pub const TRUST_E_SYSTEM_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2146869247i32);
pub const TRUST_E_TIME_STAMP: crate::core::HRESULT = crate::core::HRESULT(-2146869243i32);
pub const TYPE_E_AMBIGUOUSNAME: crate::core::HRESULT = crate::core::HRESULT(-2147319764i32);
pub const TYPE_E_BADMODULEKIND: crate::core::HRESULT = crate::core::HRESULT(-2147317571i32);
pub const TYPE_E_BUFFERTOOSMALL: crate::core::HRESULT = crate::core::HRESULT(-2147319786i32);
pub const TYPE_E_CANTCREATETMPFILE: crate::core::HRESULT = crate::core::HRESULT(-2147316573i32);
pub const TYPE_E_CANTLOADLIBRARY: crate::core::HRESULT = crate::core::HRESULT(-2147312566i32);
pub const TYPE_E_CIRCULARTYPE: crate::core::HRESULT = crate::core::HRESULT(-2147312508i32);
pub const TYPE_E_DLLFUNCTIONNOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147319761i32);
pub const TYPE_E_DUPLICATEID: crate::core::HRESULT = crate::core::HRESULT(-2147317562i32);
pub const TYPE_E_ELEMENTNOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147319765i32);
pub const TYPE_E_FIELDNOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147319785i32);
pub const TYPE_E_INCONSISTENTPROPFUNCS: crate::core::HRESULT = crate::core::HRESULT(-2147312509i32);
pub const TYPE_E_INVALIDID: crate::core::HRESULT = crate::core::HRESULT(-2147317553i32);
pub const TYPE_E_INVALIDSTATE: crate::core::HRESULT = crate::core::HRESULT(-2147319767i32);
pub const TYPE_E_INVDATAREAD: crate::core::HRESULT = crate::core::HRESULT(-2147319784i32);
pub const TYPE_E_IOERROR: crate::core::HRESULT = crate::core::HRESULT(-2147316574i32);
pub const TYPE_E_LIBNOTREGISTERED: crate::core::HRESULT = crate::core::HRESULT(-2147319779i32);
pub const TYPE_E_NAMECONFLICT: crate::core::HRESULT = crate::core::HRESULT(-2147319763i32);
pub const TYPE_E_OUTOFBOUNDS: crate::core::HRESULT = crate::core::HRESULT(-2147316575i32);
pub const TYPE_E_QUALIFIEDNAMEDISALLOWED: crate::core::HRESULT =
    crate::core::HRESULT(-2147319768i32);
pub const TYPE_E_REGISTRYACCESS: crate::core::HRESULT = crate::core::HRESULT(-2147319780i32);
pub const TYPE_E_SIZETOOBIG: crate::core::HRESULT = crate::core::HRESULT(-2147317563i32);
pub const TYPE_E_TYPEMISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2147316576i32);
pub const TYPE_E_UNDEFINEDTYPE: crate::core::HRESULT = crate::core::HRESULT(-2147319769i32);
pub const TYPE_E_UNKNOWNLCID: crate::core::HRESULT = crate::core::HRESULT(-2147319762i32);
pub const TYPE_E_UNSUPFORMAT: crate::core::HRESULT = crate::core::HRESULT(-2147319783i32);
pub const TYPE_E_WRONGTYPEKIND: crate::core::HRESULT = crate::core::HRESULT(-2147319766i32);
pub const UCEERR_BLOCKSFULL: crate::core::HRESULT = crate::core::HRESULT(-2003303415i32);
pub const UCEERR_CHANNELSYNCABANDONED: crate::core::HRESULT = crate::core::HRESULT(-2003303404i32);
pub const UCEERR_CHANNELSYNCTIMEDOUT: crate::core::HRESULT = crate::core::HRESULT(-2003303405i32);
pub const UCEERR_COMMANDTRANSPORTDENIED: crate::core::HRESULT =
    crate::core::HRESULT(-2003303400i32);
pub const UCEERR_CONNECTIONIDLOOKUPFAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2003303416i32);
pub const UCEERR_CTXSTACKFRSTTARGETNULL: crate::core::HRESULT =
    crate::core::HRESULT(-2003303417i32);
pub const UCEERR_FEEDBACK_UNSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2003303401i32);
pub const UCEERR_GRAPHICSSTREAMALREADYOPEN: crate::core::HRESULT =
    crate::core::HRESULT(-2003303392i32);
pub const UCEERR_GRAPHICSSTREAMUNAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2003303399i32);
pub const UCEERR_HANDLELOOKUPFAILED: crate::core::HRESULT = crate::core::HRESULT(-2003303419i32);
pub const UCEERR_ILLEGALHANDLE: crate::core::HRESULT = crate::core::HRESULT(-2003303420i32);
pub const UCEERR_ILLEGALPACKET: crate::core::HRESULT = crate::core::HRESULT(-2003303422i32);
pub const UCEERR_ILLEGALRECORDTYPE: crate::core::HRESULT = crate::core::HRESULT(-2003303412i32);
pub const UCEERR_INVALIDPACKETHEADER: crate::core::HRESULT = crate::core::HRESULT(-2003303424i32);
pub const UCEERR_MALFORMEDPACKET: crate::core::HRESULT = crate::core::HRESULT(-2003303421i32);
pub const UCEERR_MEMORYFAILURE: crate::core::HRESULT = crate::core::HRESULT(-2003303414i32);
pub const UCEERR_MISSINGBEGINCOMMAND: crate::core::HRESULT = crate::core::HRESULT(-2003303406i32);
pub const UCEERR_MISSINGENDCOMMAND: crate::core::HRESULT = crate::core::HRESULT(-2003303407i32);
pub const UCEERR_NO_MULTIPLE_WORKER_THREADS: crate::core::HRESULT =
    crate::core::HRESULT(-2003303409i32);
pub const UCEERR_OUTOFHANDLES: crate::core::HRESULT = crate::core::HRESULT(-2003303411i32);
pub const UCEERR_PACKETRECORDOUTOFRANGE: crate::core::HRESULT =
    crate::core::HRESULT(-2003303413i32);
pub const UCEERR_PARTITION_ZOMBIED: crate::core::HRESULT = crate::core::HRESULT(-2003303389i32);
pub const UCEERR_REMOTINGNOTSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2003303408i32);
pub const UCEERR_RENDERTHREADFAILURE: crate::core::HRESULT = crate::core::HRESULT(-2003303418i32);
pub const UCEERR_TRANSPORTDISCONNECTED: crate::core::HRESULT = crate::core::HRESULT(-2003303391i32);
pub const UCEERR_TRANSPORTOVERLOADED: crate::core::HRESULT = crate::core::HRESULT(-2003303390i32);
pub const UCEERR_TRANSPORTUNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2003303402i32);
pub const UCEERR_UNCHANGABLE_UPDATE_ATTEMPTED: crate::core::HRESULT =
    crate::core::HRESULT(-2003303410i32);
pub const UCEERR_UNKNOWNPACKET: crate::core::HRESULT = crate::core::HRESULT(-2003303423i32);
pub const UCEERR_UNSUPPORTEDTRANSPORTVERSION: crate::core::HRESULT =
    crate::core::HRESULT(-2003303403i32);
pub const UI_E_AMBIGUOUS_MATCH: crate::core::HRESULT = crate::core::HRESULT(-2144731126i32);
pub const UI_E_BOOLEAN_EXPECTED: crate::core::HRESULT = crate::core::HRESULT(-2144731128i32);
pub const UI_E_CREATE_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2144731135i32);
pub const UI_E_DIFFERENT_OWNER: crate::core::HRESULT = crate::core::HRESULT(-2144731127i32);
pub const UI_E_END_KEYFRAME_NOT_DETERMINED: crate::core::HRESULT =
    crate::core::HRESULT(-2144730876i32);
pub const UI_E_FP_OVERFLOW: crate::core::HRESULT = crate::core::HRESULT(-2144731125i32);
pub const UI_E_ILLEGAL_REENTRANCY: crate::core::HRESULT = crate::core::HRESULT(-2144731133i32);
pub const UI_E_INVALID_DIMENSION: crate::core::HRESULT = crate::core::HRESULT(-2144730869i32);
pub const UI_E_INVALID_OUTPUT: crate::core::HRESULT = crate::core::HRESULT(-2144731129i32);
pub const UI_E_LOOPS_OVERLAP: crate::core::HRESULT = crate::core::HRESULT(-2144730875i32);
pub const UI_E_OBJECT_SEALED: crate::core::HRESULT = crate::core::HRESULT(-2144731132i32);
pub const UI_E_PRIMITIVE_OUT_OF_BOUNDS: crate::core::HRESULT = crate::core::HRESULT(-2144730868i32);
pub const UI_E_SHUTDOWN_CALLED: crate::core::HRESULT = crate::core::HRESULT(-2144731134i32);
pub const UI_E_START_KEYFRAME_AFTER_END: crate::core::HRESULT =
    crate::core::HRESULT(-2144730877i32);
pub const UI_E_STORYBOARD_ACTIVE: crate::core::HRESULT = crate::core::HRESULT(-2144730879i32);
pub const UI_E_STORYBOARD_NOT_PLAYING: crate::core::HRESULT = crate::core::HRESULT(-2144730878i32);
pub const UI_E_TIMER_CLIENT_ALREADY_CONNECTED: crate::core::HRESULT =
    crate::core::HRESULT(-2144730870i32);
pub const UI_E_TIME_BEFORE_LAST_UPDATE: crate::core::HRESULT = crate::core::HRESULT(-2144730871i32);
pub const UI_E_TRANSITION_ALREADY_USED: crate::core::HRESULT = crate::core::HRESULT(-2144730874i32);
pub const UI_E_TRANSITION_ECLIPSED: crate::core::HRESULT = crate::core::HRESULT(-2144730872i32);
pub const UI_E_TRANSITION_NOT_IN_STORYBOARD: crate::core::HRESULT =
    crate::core::HRESULT(-2144730873i32);
pub const UI_E_VALUE_NOT_DETERMINED: crate::core::HRESULT = crate::core::HRESULT(-2144731130i32);
pub const UI_E_VALUE_NOT_SET: crate::core::HRESULT = crate::core::HRESULT(-2144731131i32);
pub const UI_E_WINDOW_CLOSED: crate::core::HRESULT = crate::core::HRESULT(-2144730623i32);
pub const UI_E_WRONG_THREAD: crate::core::HRESULT = crate::core::HRESULT(-2144731124i32);
pub struct UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: crate::core::PWSTR,
}
impl ::core::marker::Copy for UNICODE_STRING {}
impl ::core::clone::Clone for UNICODE_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UNICODE_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNICODE_STRING")
            .field("Length", &self.Length)
            .field("MaximumLength", &self.MaximumLength)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for UNICODE_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.MaximumLength == other.MaximumLength
            && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for UNICODE_STRING {}
impl FromIntoMemory for UNICODE_STRING {
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
pub const UTC_E_ACTION_NOT_SUPPORTED_IN_DESTINATION: crate::core::HRESULT =
    crate::core::HRESULT(-2017128380i32);
pub const UTC_E_AGENT_DIAGNOSTICS_TOO_LARGE: crate::core::HRESULT =
    crate::core::HRESULT(-2017128363i32);
pub const UTC_E_ALTERNATIVE_TRACE_CANNOT_PREEMPT: crate::core::HRESULT =
    crate::core::HRESULT(-2017128446i32);
pub const UTC_E_AOT_NOT_RUNNING: crate::core::HRESULT = crate::core::HRESULT(-2017128445i32);
pub const UTC_E_API_BUSY: crate::core::HRESULT = crate::core::HRESULT(-2017128405i32);
pub const UTC_E_API_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2017128388i32);
pub const UTC_E_API_RESULT_UNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2017128408i32);
pub const UTC_E_BINARY_MISSING: crate::core::HRESULT = crate::core::HRESULT(-2017128396i32);
pub const UTC_E_CANNOT_LOAD_SCENARIO_EDITOR_XML: crate::core::HRESULT =
    crate::core::HRESULT(-2017128417i32);
pub const UTC_E_CERT_REV_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2017128385i32);
pub const UTC_E_CHILD_PROCESS_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2017128419i32);
pub const UTC_E_COMMAND_LINE_NOT_AUTHORIZED: crate::core::HRESULT =
    crate::core::HRESULT(-2017128418i32);
pub const UTC_E_DELAY_TERMINATED: crate::core::HRESULT = crate::core::HRESULT(-2017128411i32);
pub const UTC_E_DEVICE_TICKET_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2017128410i32);
pub const UTC_E_DIAGRULES_SCHEMAVERSION_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2017128438i32);
pub const UTC_E_ESCALATION_ALREADY_RUNNING: crate::core::HRESULT =
    crate::core::HRESULT(-2017128433i32);
pub const UTC_E_ESCALATION_CANCELLED_AT_SHUTDOWN: crate::core::HRESULT =
    crate::core::HRESULT(-2017128358i32);
pub const UTC_E_ESCALATION_DIRECTORY_ALREADY_EXISTS: crate::core::HRESULT =
    crate::core::HRESULT(-2017128401i32);
pub const UTC_E_ESCALATION_NOT_AUTHORIZED: crate::core::HRESULT =
    crate::core::HRESULT(-2017128421i32);
pub const UTC_E_ESCALATION_TIMED_OUT: crate::core::HRESULT = crate::core::HRESULT(-2017128416i32);
pub const UTC_E_EVENTLOG_ENTRY_MALFORMED: crate::core::HRESULT =
    crate::core::HRESULT(-2017128439i32);
pub const UTC_E_EXCLUSIVITY_NOT_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2017128403i32);
pub const UTC_E_EXE_TERMINATED: crate::core::HRESULT = crate::core::HRESULT(-2017128422i32);
pub const UTC_E_FAILED_TO_RECEIVE_AGENT_DIAGNOSTICS: crate::core::HRESULT =
    crate::core::HRESULT(-2017128362i32);
pub const UTC_E_FAILED_TO_RESOLVE_CONTAINER_ID: crate::core::HRESULT =
    crate::core::HRESULT(-2017128394i32);
pub const UTC_E_FAILED_TO_START_NDISCAP: crate::core::HRESULT =
    crate::core::HRESULT(-2017128384i32);
pub const UTC_E_FILTER_FUNCTION_RESTRICTED: crate::core::HRESULT =
    crate::core::HRESULT(-2017128376i32);
pub const UTC_E_FILTER_ILLEGAL_EVAL: crate::core::HRESULT = crate::core::HRESULT(-2017128365i32);
pub const UTC_E_FILTER_INVALID_COMMAND: crate::core::HRESULT = crate::core::HRESULT(-2017128366i32);
pub const UTC_E_FILTER_INVALID_FUNCTION: crate::core::HRESULT =
    crate::core::HRESULT(-2017128368i32);
pub const UTC_E_FILTER_INVALID_FUNCTION_PARAMS: crate::core::HRESULT =
    crate::core::HRESULT(-2017128367i32);
pub const UTC_E_FILTER_INVALID_TYPE: crate::core::HRESULT = crate::core::HRESULT(-2017128378i32);
pub const UTC_E_FILTER_MISSING_ATTRIBUTE: crate::core::HRESULT =
    crate::core::HRESULT(-2017128379i32);
pub const UTC_E_FILTER_VARIABLE_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2017128377i32);
pub const UTC_E_FILTER_VERSION_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2017128375i32);
pub const UTC_E_FORWARDER_ALREADY_DISABLED: crate::core::HRESULT =
    crate::core::HRESULT(-2017128440i32);
pub const UTC_E_FORWARDER_ALREADY_ENABLED: crate::core::HRESULT =
    crate::core::HRESULT(-2017128441i32);
pub const UTC_E_FORWARDER_PRODUCER_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2017128430i32);
pub const UTC_E_GETFILEINFOACTION_FILE_NOT_APPROVED: crate::core::HRESULT =
    crate::core::HRESULT(-2017128357i32);
pub const UTC_E_GETFILE_EXTERNAL_PATH_NOT_APPROVED: crate::core::HRESULT =
    crate::core::HRESULT(-2017128387i32);
pub const UTC_E_GETFILE_FILE_PATH_NOT_APPROVED: crate::core::HRESULT =
    crate::core::HRESULT(-2017128402i32);
pub const UTC_E_INSUFFICIENT_SPACE_TO_START_TRACE: crate::core::HRESULT =
    crate::core::HRESULT(-2017128359i32);
pub const UTC_E_INTENTIONAL_SCRIPT_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-2017128429i32);
pub const UTC_E_INVALID_AGGREGATION_STRUCT: crate::core::HRESULT =
    crate::core::HRESULT(-2017128381i32);
pub const UTC_E_INVALID_CUSTOM_FILTER: crate::core::HRESULT = crate::core::HRESULT(-2017128436i32);
pub const UTC_E_INVALID_FILTER: crate::core::HRESULT = crate::core::HRESULT(-2017128423i32);
pub const UTC_E_KERNELDUMP_LIMIT_REACHED: crate::core::HRESULT =
    crate::core::HRESULT(-2017128383i32);
pub const UTC_E_MISSING_AGGREGATE_EVENT_TAG: crate::core::HRESULT =
    crate::core::HRESULT(-2017128382i32);
pub const UTC_E_MULTIPLE_TIME_TRIGGER_ON_SINGLE_STATE: crate::core::HRESULT =
    crate::core::HRESULT(-2017128397i32);
pub const UTC_E_NO_WER_LOGGER_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2017128427i32);
pub const UTC_E_PERFTRACK_ALREADY_TRACING: crate::core::HRESULT =
    crate::core::HRESULT(-2017128432i32);
pub const UTC_E_REACHED_MAX_ESCALATIONS: crate::core::HRESULT =
    crate::core::HRESULT(-2017128431i32);
pub const UTC_E_REESCALATED_TOO_QUICKLY: crate::core::HRESULT =
    crate::core::HRESULT(-2017128434i32);
pub const UTC_E_RPC_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2017128407i32);
pub const UTC_E_RPC_WAIT_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2017128406i32);
pub const UTC_E_SCENARIODEF_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2017128443i32);
pub const UTC_E_SCENARIODEF_SCHEMAVERSION_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2017128424i32);
pub const UTC_E_SCENARIO_HAS_NO_ACTIONS: crate::core::HRESULT =
    crate::core::HRESULT(-2017128361i32);
pub const UTC_E_SCENARIO_THROTTLED: crate::core::HRESULT = crate::core::HRESULT(-2017128389i32);
pub const UTC_E_SCRIPT_MISSING: crate::core::HRESULT = crate::core::HRESULT(-2017128390i32);
pub const UTC_E_SCRIPT_TERMINATED: crate::core::HRESULT = crate::core::HRESULT(-2017128437i32);
pub const UTC_E_SCRIPT_TYPE_INVALID: crate::core::HRESULT = crate::core::HRESULT(-2017128444i32);
pub const UTC_E_SETREGKEYACTION_TYPE_NOT_APPROVED: crate::core::HRESULT =
    crate::core::HRESULT(-2017128356i32);
pub const UTC_E_SETUP_NOT_AUTHORIZED: crate::core::HRESULT = crate::core::HRESULT(-2017128420i32);
pub const UTC_E_SETUP_TIMED_OUT: crate::core::HRESULT = crate::core::HRESULT(-2017128415i32);
pub const UTC_E_SIF_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2017128412i32);
pub const UTC_E_SQM_INIT_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2017128428i32);
pub const UTC_E_THROTTLED: crate::core::HRESULT = crate::core::HRESULT(-2017128392i32);
pub const UTC_E_TIME_TRIGGER_INVALID_TIME_RANGE: crate::core::HRESULT =
    crate::core::HRESULT(-2017128398i32);
pub const UTC_E_TIME_TRIGGER_ONLY_VALID_ON_SINGLE_TRANSITION: crate::core::HRESULT =
    crate::core::HRESULT(-2017128399i32);
pub const UTC_E_TIME_TRIGGER_ON_START_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2017128400i32);
pub const UTC_E_TOGGLE_TRACE_STARTED: crate::core::HRESULT = crate::core::HRESULT(-2017128447i32);
pub const UTC_E_TRACEPROFILE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2017128442i32);
pub const UTC_E_TRACERS_DONT_EXIST: crate::core::HRESULT = crate::core::HRESULT(-2017128426i32);
pub const UTC_E_TRACE_BUFFER_LIMIT_EXCEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-2017128409i32);
pub const UTC_E_TRACE_MIN_DURATION_REQUIREMENT_NOT_MET: crate::core::HRESULT =
    crate::core::HRESULT(-2017128404i32);
pub const UTC_E_TRACE_NOT_RUNNING: crate::core::HRESULT = crate::core::HRESULT(-2017128435i32);
pub const UTC_E_TRACE_THROTTLED: crate::core::HRESULT = crate::core::HRESULT(-2017128355i32);
pub const UTC_E_TRIGGER_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2017128414i32);
pub const UTC_E_TRIGGER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2017128413i32);
pub const UTC_E_TRY_GET_SCENARIO_TIMEOUT_EXCEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-2017128386i32);
pub const UTC_E_TTTRACER_RETURNED_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-2017128364i32);
pub const UTC_E_TTTRACER_STORAGE_FULL: crate::core::HRESULT = crate::core::HRESULT(-2017128360i32);
pub const UTC_E_UNABLE_TO_RESOLVE_SESSION: crate::core::HRESULT =
    crate::core::HRESULT(-2017128393i32);
pub const UTC_E_UNAPPROVED_SCRIPT: crate::core::HRESULT = crate::core::HRESULT(-2017128391i32);
pub const UTC_E_WINRT_INIT_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2017128425i32);
pub const VIEW_E_DRAW: crate::core::HRESULT = crate::core::HRESULT(-2147221184i32);
pub const VIEW_E_FIRST: i32 = -2147221184i32;
pub const VIEW_E_LAST: i32 = -2147221169i32;
pub const VIEW_S_ALREADY_FROZEN: crate::core::HRESULT = crate::core::HRESULT(262464i32);
pub const VIEW_S_FIRST: i32 = 262464i32;
pub const VIEW_S_LAST: i32 = 262479i32;
pub const VM_SAVED_STATE_DUMP_E_GUEST_MEMORY_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1070136063i32);
pub const VM_SAVED_STATE_DUMP_E_INVALID_VP_STATE: crate::core::HRESULT =
    crate::core::HRESULT(-1070136058i32);
pub const VM_SAVED_STATE_DUMP_E_NESTED_VIRTUALIZATION_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-1070136061i32);
pub const VM_SAVED_STATE_DUMP_E_NO_VP_FOUND_IN_PARTITION_STATE: crate::core::HRESULT =
    crate::core::HRESULT(-1070136062i32);
pub const VM_SAVED_STATE_DUMP_E_PARTITION_STATE_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1070136064i32);
pub const VM_SAVED_STATE_DUMP_E_VA_NOT_MAPPED: crate::core::HRESULT =
    crate::core::HRESULT(-1070136059i32);
pub const VM_SAVED_STATE_DUMP_E_VP_VTL_NOT_ENABLED: crate::core::HRESULT =
    crate::core::HRESULT(-1070136055i32);
pub const VM_SAVED_STATE_DUMP_E_WINDOWS_KERNEL_IMAGE_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-1070136060i32);
pub const WARNING_IPSEC_MM_POLICY_PRUNED: i32 = 13024i32;
pub const WARNING_IPSEC_QM_POLICY_PRUNED: i32 = 13025i32;
pub const WARNING_NO_MD5_MIGRATION: u32 = 946u32;
pub const WBREAK_E_BUFFER_TOO_SMALL: crate::core::HRESULT = crate::core::HRESULT(-2147215485i32);
pub const WBREAK_E_END_OF_TEXT: crate::core::HRESULT = crate::core::HRESULT(-2147215488i32);
pub const WBREAK_E_INIT_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2147215483i32);
pub const WBREAK_E_QUERY_ONLY: crate::core::HRESULT = crate::core::HRESULT(-2147215486i32);
pub const WEB_E_INVALID_JSON_NUMBER: crate::core::HRESULT = crate::core::HRESULT(-2089484280i32);
pub const WEB_E_INVALID_JSON_STRING: crate::core::HRESULT = crate::core::HRESULT(-2089484281i32);
pub const WEB_E_INVALID_XML: crate::core::HRESULT = crate::core::HRESULT(-2089484286i32);
pub const WEB_E_JSON_VALUE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2089484279i32);
pub const WEB_E_MISSING_REQUIRED_ATTRIBUTE: crate::core::HRESULT =
    crate::core::HRESULT(-2089484284i32);
pub const WEB_E_MISSING_REQUIRED_ELEMENT: crate::core::HRESULT =
    crate::core::HRESULT(-2089484285i32);
pub const WEB_E_RESOURCE_TOO_LARGE: crate::core::HRESULT = crate::core::HRESULT(-2089484282i32);
pub const WEB_E_UNEXPECTED_CONTENT: crate::core::HRESULT = crate::core::HRESULT(-2089484283i32);
pub const WEB_E_UNSUPPORTED_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-2089484287i32);
pub const WEP_E_BUFFER_TOO_LARGE: crate::core::HRESULT = crate::core::HRESULT(-2013200375i32);
pub const WEP_E_FIXED_DATA_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2013200382i32);
pub const WEP_E_HARDWARE_NOT_COMPLIANT: crate::core::HRESULT = crate::core::HRESULT(-2013200381i32);
pub const WEP_E_LOCK_NOT_CONFIGURED: crate::core::HRESULT = crate::core::HRESULT(-2013200380i32);
pub const WEP_E_NOT_PROVISIONED_ON_ALL_VOLUMES: crate::core::HRESULT =
    crate::core::HRESULT(-2013200383i32);
pub const WEP_E_NO_LICENSE: crate::core::HRESULT = crate::core::HRESULT(-2013200378i32);
pub const WEP_E_OS_NOT_PROTECTED: crate::core::HRESULT = crate::core::HRESULT(-2013200377i32);
pub const WEP_E_PROTECTION_SUSPENDED: crate::core::HRESULT = crate::core::HRESULT(-2013200379i32);
pub const WEP_E_UNEXPECTED_FAIL: crate::core::HRESULT = crate::core::HRESULT(-2013200376i32);
pub const WER_E_ALREADY_REPORTING: crate::core::HRESULT = crate::core::HRESULT(-2145681404i32);
pub const WER_E_CANCELED: crate::core::HRESULT = crate::core::HRESULT(-2145681407i32);
pub const WER_E_CRASH_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2145681408i32);
pub const WER_E_DUMP_THROTTLED: crate::core::HRESULT = crate::core::HRESULT(-2145681403i32);
pub const WER_E_INSUFFICIENT_CONSENT: crate::core::HRESULT = crate::core::HRESULT(-2145681402i32);
pub const WER_E_NETWORK_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2145681406i32);
pub const WER_E_NOT_INITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2145681405i32);
pub const WER_E_TOO_HEAVY: crate::core::HRESULT = crate::core::HRESULT(-2145681401i32);
pub const WER_S_ASSERT_CONTINUE: crate::core::HRESULT = crate::core::HRESULT(1769482i32);
pub const WER_S_DISABLED: crate::core::HRESULT = crate::core::HRESULT(1769475i32);
pub const WER_S_DISABLED_ARCHIVE: crate::core::HRESULT = crate::core::HRESULT(1769478i32);
pub const WER_S_DISABLED_QUEUE: crate::core::HRESULT = crate::core::HRESULT(1769477i32);
pub const WER_S_IGNORE_ALL_ASSERTS: crate::core::HRESULT = crate::core::HRESULT(1769481i32);
pub const WER_S_IGNORE_ASSERT_INSTANCE: crate::core::HRESULT = crate::core::HRESULT(1769480i32);
pub const WER_S_REPORT_ASYNC: crate::core::HRESULT = crate::core::HRESULT(1769479i32);
pub const WER_S_REPORT_DEBUG: crate::core::HRESULT = crate::core::HRESULT(1769472i32);
pub const WER_S_REPORT_QUEUED: crate::core::HRESULT = crate::core::HRESULT(1769474i32);
pub const WER_S_REPORT_UPLOADED: crate::core::HRESULT = crate::core::HRESULT(1769473i32);
pub const WER_S_REPORT_UPLOADED_CAB: crate::core::HRESULT = crate::core::HRESULT(1769484i32);
pub const WER_S_SUSPENDED_UPLOAD: crate::core::HRESULT = crate::core::HRESULT(1769476i32);
pub const WER_S_THROTTLED: crate::core::HRESULT = crate::core::HRESULT(1769483i32);
pub const WHV_E_GPA_RANGE_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2143878395i32);
pub const WHV_E_INSUFFICIENT_BUFFER: crate::core::HRESULT = crate::core::HRESULT(-2143878399i32);
pub const WHV_E_INVALID_PARTITION_CONFIG: crate::core::HRESULT =
    crate::core::HRESULT(-2143878396i32);
pub const WHV_E_INVALID_VP_REGISTER_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-2143878391i32);
pub const WHV_E_INVALID_VP_STATE: crate::core::HRESULT = crate::core::HRESULT(-2143878392i32);
pub const WHV_E_UNKNOWN_CAPABILITY: crate::core::HRESULT = crate::core::HRESULT(-2143878400i32);
pub const WHV_E_UNKNOWN_PROPERTY: crate::core::HRESULT = crate::core::HRESULT(-2143878398i32);
pub const WHV_E_UNSUPPORTED_HYPERVISOR_CONFIG: crate::core::HRESULT =
    crate::core::HRESULT(-2143878397i32);
pub const WHV_E_UNSUPPORTED_PROCESSOR_CONFIG: crate::core::HRESULT =
    crate::core::HRESULT(-2143878384i32);
pub const WHV_E_VP_ALREADY_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2143878394i32);
pub const WHV_E_VP_DOES_NOT_EXIST: crate::core::HRESULT = crate::core::HRESULT(-2143878393i32);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WIN32_ERROR(pub u32);
pub const NO_ERROR: WIN32_ERROR = WIN32_ERROR(0u32);
pub const WAIT_TIMEOUT: WIN32_ERROR = WIN32_ERROR(258u32);
pub const WAIT_FAILED: WIN32_ERROR = WIN32_ERROR(4294967295u32);
pub const ERROR_SUCCESS: WIN32_ERROR = WIN32_ERROR(0u32);
pub const ERROR_INVALID_FUNCTION: WIN32_ERROR = WIN32_ERROR(1u32);
pub const ERROR_FILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2u32);
pub const ERROR_PATH_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3u32);
pub const ERROR_TOO_MANY_OPEN_FILES: WIN32_ERROR = WIN32_ERROR(4u32);
pub const ERROR_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(5u32);
pub const ERROR_INVALID_HANDLE: WIN32_ERROR = WIN32_ERROR(6u32);
pub const ERROR_ARENA_TRASHED: WIN32_ERROR = WIN32_ERROR(7u32);
pub const ERROR_NOT_ENOUGH_MEMORY: WIN32_ERROR = WIN32_ERROR(8u32);
pub const ERROR_INVALID_BLOCK: WIN32_ERROR = WIN32_ERROR(9u32);
pub const ERROR_BAD_ENVIRONMENT: WIN32_ERROR = WIN32_ERROR(10u32);
pub const ERROR_BAD_FORMAT: WIN32_ERROR = WIN32_ERROR(11u32);
pub const ERROR_INVALID_ACCESS: WIN32_ERROR = WIN32_ERROR(12u32);
pub const ERROR_INVALID_DATA: WIN32_ERROR = WIN32_ERROR(13u32);
pub const ERROR_OUTOFMEMORY: WIN32_ERROR = WIN32_ERROR(14u32);
pub const ERROR_INVALID_DRIVE: WIN32_ERROR = WIN32_ERROR(15u32);
pub const ERROR_CURRENT_DIRECTORY: WIN32_ERROR = WIN32_ERROR(16u32);
pub const ERROR_NOT_SAME_DEVICE: WIN32_ERROR = WIN32_ERROR(17u32);
pub const ERROR_NO_MORE_FILES: WIN32_ERROR = WIN32_ERROR(18u32);
pub const ERROR_WRITE_PROTECT: WIN32_ERROR = WIN32_ERROR(19u32);
pub const ERROR_BAD_UNIT: WIN32_ERROR = WIN32_ERROR(20u32);
pub const ERROR_NOT_READY: WIN32_ERROR = WIN32_ERROR(21u32);
pub const ERROR_BAD_COMMAND: WIN32_ERROR = WIN32_ERROR(22u32);
pub const ERROR_CRC: WIN32_ERROR = WIN32_ERROR(23u32);
pub const ERROR_BAD_LENGTH: WIN32_ERROR = WIN32_ERROR(24u32);
pub const ERROR_SEEK: WIN32_ERROR = WIN32_ERROR(25u32);
pub const ERROR_NOT_DOS_DISK: WIN32_ERROR = WIN32_ERROR(26u32);
pub const ERROR_SECTOR_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(27u32);
pub const ERROR_OUT_OF_PAPER: WIN32_ERROR = WIN32_ERROR(28u32);
pub const ERROR_WRITE_FAULT: WIN32_ERROR = WIN32_ERROR(29u32);
pub const ERROR_READ_FAULT: WIN32_ERROR = WIN32_ERROR(30u32);
pub const ERROR_GEN_FAILURE: WIN32_ERROR = WIN32_ERROR(31u32);
pub const ERROR_SHARING_VIOLATION: WIN32_ERROR = WIN32_ERROR(32u32);
pub const ERROR_LOCK_VIOLATION: WIN32_ERROR = WIN32_ERROR(33u32);
pub const ERROR_WRONG_DISK: WIN32_ERROR = WIN32_ERROR(34u32);
pub const ERROR_SHARING_BUFFER_EXCEEDED: WIN32_ERROR = WIN32_ERROR(36u32);
pub const ERROR_HANDLE_EOF: WIN32_ERROR = WIN32_ERROR(38u32);
pub const ERROR_HANDLE_DISK_FULL: WIN32_ERROR = WIN32_ERROR(39u32);
pub const ERROR_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(50u32);
pub const ERROR_REM_NOT_LIST: WIN32_ERROR = WIN32_ERROR(51u32);
pub const ERROR_DUP_NAME: WIN32_ERROR = WIN32_ERROR(52u32);
pub const ERROR_BAD_NETPATH: WIN32_ERROR = WIN32_ERROR(53u32);
pub const ERROR_NETWORK_BUSY: WIN32_ERROR = WIN32_ERROR(54u32);
pub const ERROR_DEV_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(55u32);
pub const ERROR_TOO_MANY_CMDS: WIN32_ERROR = WIN32_ERROR(56u32);
pub const ERROR_ADAP_HDW_ERR: WIN32_ERROR = WIN32_ERROR(57u32);
pub const ERROR_BAD_NET_RESP: WIN32_ERROR = WIN32_ERROR(58u32);
pub const ERROR_UNEXP_NET_ERR: WIN32_ERROR = WIN32_ERROR(59u32);
pub const ERROR_BAD_REM_ADAP: WIN32_ERROR = WIN32_ERROR(60u32);
pub const ERROR_PRINTQ_FULL: WIN32_ERROR = WIN32_ERROR(61u32);
pub const ERROR_NO_SPOOL_SPACE: WIN32_ERROR = WIN32_ERROR(62u32);
pub const ERROR_PRINT_CANCELLED: WIN32_ERROR = WIN32_ERROR(63u32);
pub const ERROR_NETNAME_DELETED: WIN32_ERROR = WIN32_ERROR(64u32);
pub const ERROR_NETWORK_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(65u32);
pub const ERROR_BAD_DEV_TYPE: WIN32_ERROR = WIN32_ERROR(66u32);
pub const ERROR_BAD_NET_NAME: WIN32_ERROR = WIN32_ERROR(67u32);
pub const ERROR_TOO_MANY_NAMES: WIN32_ERROR = WIN32_ERROR(68u32);
pub const ERROR_TOO_MANY_SESS: WIN32_ERROR = WIN32_ERROR(69u32);
pub const ERROR_SHARING_PAUSED: WIN32_ERROR = WIN32_ERROR(70u32);
pub const ERROR_REQ_NOT_ACCEP: WIN32_ERROR = WIN32_ERROR(71u32);
pub const ERROR_REDIR_PAUSED: WIN32_ERROR = WIN32_ERROR(72u32);
pub const ERROR_FILE_EXISTS: WIN32_ERROR = WIN32_ERROR(80u32);
pub const ERROR_CANNOT_MAKE: WIN32_ERROR = WIN32_ERROR(82u32);
pub const ERROR_FAIL_I24: WIN32_ERROR = WIN32_ERROR(83u32);
pub const ERROR_OUT_OF_STRUCTURES: WIN32_ERROR = WIN32_ERROR(84u32);
pub const ERROR_ALREADY_ASSIGNED: WIN32_ERROR = WIN32_ERROR(85u32);
pub const ERROR_INVALID_PASSWORD: WIN32_ERROR = WIN32_ERROR(86u32);
pub const ERROR_INVALID_PARAMETER: WIN32_ERROR = WIN32_ERROR(87u32);
pub const ERROR_NET_WRITE_FAULT: WIN32_ERROR = WIN32_ERROR(88u32);
pub const ERROR_NO_PROC_SLOTS: WIN32_ERROR = WIN32_ERROR(89u32);
pub const ERROR_TOO_MANY_SEMAPHORES: WIN32_ERROR = WIN32_ERROR(100u32);
pub const ERROR_EXCL_SEM_ALREADY_OWNED: WIN32_ERROR = WIN32_ERROR(101u32);
pub const ERROR_SEM_IS_SET: WIN32_ERROR = WIN32_ERROR(102u32);
pub const ERROR_TOO_MANY_SEM_REQUESTS: WIN32_ERROR = WIN32_ERROR(103u32);
pub const ERROR_INVALID_AT_INTERRUPT_TIME: WIN32_ERROR = WIN32_ERROR(104u32);
pub const ERROR_SEM_OWNER_DIED: WIN32_ERROR = WIN32_ERROR(105u32);
pub const ERROR_SEM_USER_LIMIT: WIN32_ERROR = WIN32_ERROR(106u32);
pub const ERROR_DISK_CHANGE: WIN32_ERROR = WIN32_ERROR(107u32);
pub const ERROR_DRIVE_LOCKED: WIN32_ERROR = WIN32_ERROR(108u32);
pub const ERROR_BROKEN_PIPE: WIN32_ERROR = WIN32_ERROR(109u32);
pub const ERROR_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(110u32);
pub const ERROR_BUFFER_OVERFLOW: WIN32_ERROR = WIN32_ERROR(111u32);
pub const ERROR_DISK_FULL: WIN32_ERROR = WIN32_ERROR(112u32);
pub const ERROR_NO_MORE_SEARCH_HANDLES: WIN32_ERROR = WIN32_ERROR(113u32);
pub const ERROR_INVALID_TARGET_HANDLE: WIN32_ERROR = WIN32_ERROR(114u32);
pub const ERROR_INVALID_CATEGORY: WIN32_ERROR = WIN32_ERROR(117u32);
pub const ERROR_INVALID_VERIFY_SWITCH: WIN32_ERROR = WIN32_ERROR(118u32);
pub const ERROR_BAD_DRIVER_LEVEL: WIN32_ERROR = WIN32_ERROR(119u32);
pub const ERROR_CALL_NOT_IMPLEMENTED: WIN32_ERROR = WIN32_ERROR(120u32);
pub const ERROR_SEM_TIMEOUT: WIN32_ERROR = WIN32_ERROR(121u32);
pub const ERROR_INSUFFICIENT_BUFFER: WIN32_ERROR = WIN32_ERROR(122u32);
pub const ERROR_INVALID_NAME: WIN32_ERROR = WIN32_ERROR(123u32);
pub const ERROR_INVALID_LEVEL: WIN32_ERROR = WIN32_ERROR(124u32);
pub const ERROR_NO_VOLUME_LABEL: WIN32_ERROR = WIN32_ERROR(125u32);
pub const ERROR_MOD_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(126u32);
pub const ERROR_PROC_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(127u32);
pub const ERROR_WAIT_NO_CHILDREN: WIN32_ERROR = WIN32_ERROR(128u32);
pub const ERROR_CHILD_NOT_COMPLETE: WIN32_ERROR = WIN32_ERROR(129u32);
pub const ERROR_DIRECT_ACCESS_HANDLE: WIN32_ERROR = WIN32_ERROR(130u32);
pub const ERROR_NEGATIVE_SEEK: WIN32_ERROR = WIN32_ERROR(131u32);
pub const ERROR_SEEK_ON_DEVICE: WIN32_ERROR = WIN32_ERROR(132u32);
pub const ERROR_IS_JOIN_TARGET: WIN32_ERROR = WIN32_ERROR(133u32);
pub const ERROR_IS_JOINED: WIN32_ERROR = WIN32_ERROR(134u32);
pub const ERROR_IS_SUBSTED: WIN32_ERROR = WIN32_ERROR(135u32);
pub const ERROR_NOT_JOINED: WIN32_ERROR = WIN32_ERROR(136u32);
pub const ERROR_NOT_SUBSTED: WIN32_ERROR = WIN32_ERROR(137u32);
pub const ERROR_JOIN_TO_JOIN: WIN32_ERROR = WIN32_ERROR(138u32);
pub const ERROR_SUBST_TO_SUBST: WIN32_ERROR = WIN32_ERROR(139u32);
pub const ERROR_JOIN_TO_SUBST: WIN32_ERROR = WIN32_ERROR(140u32);
pub const ERROR_SUBST_TO_JOIN: WIN32_ERROR = WIN32_ERROR(141u32);
pub const ERROR_BUSY_DRIVE: WIN32_ERROR = WIN32_ERROR(142u32);
pub const ERROR_SAME_DRIVE: WIN32_ERROR = WIN32_ERROR(143u32);
pub const ERROR_DIR_NOT_ROOT: WIN32_ERROR = WIN32_ERROR(144u32);
pub const ERROR_DIR_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(145u32);
pub const ERROR_IS_SUBST_PATH: WIN32_ERROR = WIN32_ERROR(146u32);
pub const ERROR_IS_JOIN_PATH: WIN32_ERROR = WIN32_ERROR(147u32);
pub const ERROR_PATH_BUSY: WIN32_ERROR = WIN32_ERROR(148u32);
pub const ERROR_IS_SUBST_TARGET: WIN32_ERROR = WIN32_ERROR(149u32);
pub const ERROR_SYSTEM_TRACE: WIN32_ERROR = WIN32_ERROR(150u32);
pub const ERROR_INVALID_EVENT_COUNT: WIN32_ERROR = WIN32_ERROR(151u32);
pub const ERROR_TOO_MANY_MUXWAITERS: WIN32_ERROR = WIN32_ERROR(152u32);
pub const ERROR_INVALID_LIST_FORMAT: WIN32_ERROR = WIN32_ERROR(153u32);
pub const ERROR_LABEL_TOO_LONG: WIN32_ERROR = WIN32_ERROR(154u32);
pub const ERROR_TOO_MANY_TCBS: WIN32_ERROR = WIN32_ERROR(155u32);
pub const ERROR_SIGNAL_REFUSED: WIN32_ERROR = WIN32_ERROR(156u32);
pub const ERROR_DISCARDED: WIN32_ERROR = WIN32_ERROR(157u32);
pub const ERROR_NOT_LOCKED: WIN32_ERROR = WIN32_ERROR(158u32);
pub const ERROR_BAD_THREADID_ADDR: WIN32_ERROR = WIN32_ERROR(159u32);
pub const ERROR_BAD_ARGUMENTS: WIN32_ERROR = WIN32_ERROR(160u32);
pub const ERROR_BAD_PATHNAME: WIN32_ERROR = WIN32_ERROR(161u32);
pub const ERROR_SIGNAL_PENDING: WIN32_ERROR = WIN32_ERROR(162u32);
pub const ERROR_MAX_THRDS_REACHED: WIN32_ERROR = WIN32_ERROR(164u32);
pub const ERROR_LOCK_FAILED: WIN32_ERROR = WIN32_ERROR(167u32);
pub const ERROR_BUSY: WIN32_ERROR = WIN32_ERROR(170u32);
pub const ERROR_DEVICE_SUPPORT_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(171u32);
pub const ERROR_CANCEL_VIOLATION: WIN32_ERROR = WIN32_ERROR(173u32);
pub const ERROR_ATOMIC_LOCKS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(174u32);
pub const ERROR_INVALID_SEGMENT_NUMBER: WIN32_ERROR = WIN32_ERROR(180u32);
pub const ERROR_INVALID_ORDINAL: WIN32_ERROR = WIN32_ERROR(182u32);
pub const ERROR_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(183u32);
pub const ERROR_INVALID_FLAG_NUMBER: WIN32_ERROR = WIN32_ERROR(186u32);
pub const ERROR_SEM_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(187u32);
pub const ERROR_INVALID_STARTING_CODESEG: WIN32_ERROR = WIN32_ERROR(188u32);
pub const ERROR_INVALID_STACKSEG: WIN32_ERROR = WIN32_ERROR(189u32);
pub const ERROR_INVALID_MODULETYPE: WIN32_ERROR = WIN32_ERROR(190u32);
pub const ERROR_INVALID_EXE_SIGNATURE: WIN32_ERROR = WIN32_ERROR(191u32);
pub const ERROR_EXE_MARKED_INVALID: WIN32_ERROR = WIN32_ERROR(192u32);
pub const ERROR_BAD_EXE_FORMAT: WIN32_ERROR = WIN32_ERROR(193u32);
pub const ERROR_ITERATED_DATA_EXCEEDS_64k: WIN32_ERROR = WIN32_ERROR(194u32);
pub const ERROR_INVALID_MINALLOCSIZE: WIN32_ERROR = WIN32_ERROR(195u32);
pub const ERROR_DYNLINK_FROM_INVALID_RING: WIN32_ERROR = WIN32_ERROR(196u32);
pub const ERROR_IOPL_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(197u32);
pub const ERROR_INVALID_SEGDPL: WIN32_ERROR = WIN32_ERROR(198u32);
pub const ERROR_AUTODATASEG_EXCEEDS_64k: WIN32_ERROR = WIN32_ERROR(199u32);
pub const ERROR_RING2SEG_MUST_BE_MOVABLE: WIN32_ERROR = WIN32_ERROR(200u32);
pub const ERROR_RELOC_CHAIN_XEEDS_SEGLIM: WIN32_ERROR = WIN32_ERROR(201u32);
pub const ERROR_INFLOOP_IN_RELOC_CHAIN: WIN32_ERROR = WIN32_ERROR(202u32);
pub const ERROR_ENVVAR_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(203u32);
pub const ERROR_NO_SIGNAL_SENT: WIN32_ERROR = WIN32_ERROR(205u32);
pub const ERROR_FILENAME_EXCED_RANGE: WIN32_ERROR = WIN32_ERROR(206u32);
pub const ERROR_RING2_STACK_IN_USE: WIN32_ERROR = WIN32_ERROR(207u32);
pub const ERROR_META_EXPANSION_TOO_LONG: WIN32_ERROR = WIN32_ERROR(208u32);
pub const ERROR_INVALID_SIGNAL_NUMBER: WIN32_ERROR = WIN32_ERROR(209u32);
pub const ERROR_THREAD_1_INACTIVE: WIN32_ERROR = WIN32_ERROR(210u32);
pub const ERROR_LOCKED: WIN32_ERROR = WIN32_ERROR(212u32);
pub const ERROR_TOO_MANY_MODULES: WIN32_ERROR = WIN32_ERROR(214u32);
pub const ERROR_NESTING_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(215u32);
pub const ERROR_EXE_MACHINE_TYPE_MISMATCH: WIN32_ERROR = WIN32_ERROR(216u32);
pub const ERROR_EXE_CANNOT_MODIFY_SIGNED_BINARY: WIN32_ERROR = WIN32_ERROR(217u32);
pub const ERROR_EXE_CANNOT_MODIFY_STRONG_SIGNED_BINARY: WIN32_ERROR = WIN32_ERROR(218u32);
pub const ERROR_FILE_CHECKED_OUT: WIN32_ERROR = WIN32_ERROR(220u32);
pub const ERROR_CHECKOUT_REQUIRED: WIN32_ERROR = WIN32_ERROR(221u32);
pub const ERROR_BAD_FILE_TYPE: WIN32_ERROR = WIN32_ERROR(222u32);
pub const ERROR_FILE_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(223u32);
pub const ERROR_FORMS_AUTH_REQUIRED: WIN32_ERROR = WIN32_ERROR(224u32);
pub const ERROR_VIRUS_INFECTED: WIN32_ERROR = WIN32_ERROR(225u32);
pub const ERROR_VIRUS_DELETED: WIN32_ERROR = WIN32_ERROR(226u32);
pub const ERROR_PIPE_LOCAL: WIN32_ERROR = WIN32_ERROR(229u32);
pub const ERROR_BAD_PIPE: WIN32_ERROR = WIN32_ERROR(230u32);
pub const ERROR_PIPE_BUSY: WIN32_ERROR = WIN32_ERROR(231u32);
pub const ERROR_NO_DATA: WIN32_ERROR = WIN32_ERROR(232u32);
pub const ERROR_PIPE_NOT_CONNECTED: WIN32_ERROR = WIN32_ERROR(233u32);
pub const ERROR_MORE_DATA: WIN32_ERROR = WIN32_ERROR(234u32);
pub const ERROR_NO_WORK_DONE: WIN32_ERROR = WIN32_ERROR(235u32);
pub const ERROR_VC_DISCONNECTED: WIN32_ERROR = WIN32_ERROR(240u32);
pub const ERROR_INVALID_EA_NAME: WIN32_ERROR = WIN32_ERROR(254u32);
pub const ERROR_EA_LIST_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(255u32);
pub const ERROR_NO_MORE_ITEMS: WIN32_ERROR = WIN32_ERROR(259u32);
pub const ERROR_CANNOT_COPY: WIN32_ERROR = WIN32_ERROR(266u32);
pub const ERROR_DIRECTORY: WIN32_ERROR = WIN32_ERROR(267u32);
pub const ERROR_EAS_DIDNT_FIT: WIN32_ERROR = WIN32_ERROR(275u32);
pub const ERROR_EA_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(276u32);
pub const ERROR_EA_TABLE_FULL: WIN32_ERROR = WIN32_ERROR(277u32);
pub const ERROR_INVALID_EA_HANDLE: WIN32_ERROR = WIN32_ERROR(278u32);
pub const ERROR_EAS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(282u32);
pub const ERROR_NOT_OWNER: WIN32_ERROR = WIN32_ERROR(288u32);
pub const ERROR_TOO_MANY_POSTS: WIN32_ERROR = WIN32_ERROR(298u32);
pub const ERROR_PARTIAL_COPY: WIN32_ERROR = WIN32_ERROR(299u32);
pub const ERROR_OPLOCK_NOT_GRANTED: WIN32_ERROR = WIN32_ERROR(300u32);
pub const ERROR_INVALID_OPLOCK_PROTOCOL: WIN32_ERROR = WIN32_ERROR(301u32);
pub const ERROR_DISK_TOO_FRAGMENTED: WIN32_ERROR = WIN32_ERROR(302u32);
pub const ERROR_DELETE_PENDING: WIN32_ERROR = WIN32_ERROR(303u32);
pub const ERROR_INCOMPATIBLE_WITH_GLOBAL_SHORT_NAME_REGISTRY_SETTING: WIN32_ERROR =
    WIN32_ERROR(304u32);
pub const ERROR_SHORT_NAMES_NOT_ENABLED_ON_VOLUME: WIN32_ERROR = WIN32_ERROR(305u32);
pub const ERROR_SECURITY_STREAM_IS_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(306u32);
pub const ERROR_INVALID_LOCK_RANGE: WIN32_ERROR = WIN32_ERROR(307u32);
pub const ERROR_IMAGE_SUBSYSTEM_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(308u32);
pub const ERROR_NOTIFICATION_GUID_ALREADY_DEFINED: WIN32_ERROR = WIN32_ERROR(309u32);
pub const ERROR_INVALID_EXCEPTION_HANDLER: WIN32_ERROR = WIN32_ERROR(310u32);
pub const ERROR_DUPLICATE_PRIVILEGES: WIN32_ERROR = WIN32_ERROR(311u32);
pub const ERROR_NO_RANGES_PROCESSED: WIN32_ERROR = WIN32_ERROR(312u32);
pub const ERROR_NOT_ALLOWED_ON_SYSTEM_FILE: WIN32_ERROR = WIN32_ERROR(313u32);
pub const ERROR_DISK_RESOURCES_EXHAUSTED: WIN32_ERROR = WIN32_ERROR(314u32);
pub const ERROR_INVALID_TOKEN: WIN32_ERROR = WIN32_ERROR(315u32);
pub const ERROR_DEVICE_FEATURE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(316u32);
pub const ERROR_MR_MID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(317u32);
pub const ERROR_SCOPE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(318u32);
pub const ERROR_UNDEFINED_SCOPE: WIN32_ERROR = WIN32_ERROR(319u32);
pub const ERROR_INVALID_CAP: WIN32_ERROR = WIN32_ERROR(320u32);
pub const ERROR_DEVICE_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(321u32);
pub const ERROR_DEVICE_NO_RESOURCES: WIN32_ERROR = WIN32_ERROR(322u32);
pub const ERROR_DATA_CHECKSUM_ERROR: WIN32_ERROR = WIN32_ERROR(323u32);
pub const ERROR_INTERMIXED_KERNEL_EA_OPERATION: WIN32_ERROR = WIN32_ERROR(324u32);
pub const ERROR_FILE_LEVEL_TRIM_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(326u32);
pub const ERROR_OFFSET_ALIGNMENT_VIOLATION: WIN32_ERROR = WIN32_ERROR(327u32);
pub const ERROR_INVALID_FIELD_IN_PARAMETER_LIST: WIN32_ERROR = WIN32_ERROR(328u32);
pub const ERROR_OPERATION_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(329u32);
pub const ERROR_BAD_DEVICE_PATH: WIN32_ERROR = WIN32_ERROR(330u32);
pub const ERROR_TOO_MANY_DESCRIPTORS: WIN32_ERROR = WIN32_ERROR(331u32);
pub const ERROR_SCRUB_DATA_DISABLED: WIN32_ERROR = WIN32_ERROR(332u32);
pub const ERROR_NOT_REDUNDANT_STORAGE: WIN32_ERROR = WIN32_ERROR(333u32);
pub const ERROR_RESIDENT_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(334u32);
pub const ERROR_COMPRESSED_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(335u32);
pub const ERROR_DIRECTORY_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(336u32);
pub const ERROR_NOT_READ_FROM_COPY: WIN32_ERROR = WIN32_ERROR(337u32);
pub const ERROR_FT_WRITE_FAILURE: WIN32_ERROR = WIN32_ERROR(338u32);
pub const ERROR_FT_DI_SCAN_REQUIRED: WIN32_ERROR = WIN32_ERROR(339u32);
pub const ERROR_INVALID_KERNEL_INFO_VERSION: WIN32_ERROR = WIN32_ERROR(340u32);
pub const ERROR_INVALID_PEP_INFO_VERSION: WIN32_ERROR = WIN32_ERROR(341u32);
pub const ERROR_OBJECT_NOT_EXTERNALLY_BACKED: WIN32_ERROR = WIN32_ERROR(342u32);
pub const ERROR_EXTERNAL_BACKING_PROVIDER_UNKNOWN: WIN32_ERROR = WIN32_ERROR(343u32);
pub const ERROR_COMPRESSION_NOT_BENEFICIAL: WIN32_ERROR = WIN32_ERROR(344u32);
pub const ERROR_STORAGE_TOPOLOGY_ID_MISMATCH: WIN32_ERROR = WIN32_ERROR(345u32);
pub const ERROR_BLOCKED_BY_PARENTAL_CONTROLS: WIN32_ERROR = WIN32_ERROR(346u32);
pub const ERROR_BLOCK_TOO_MANY_REFERENCES: WIN32_ERROR = WIN32_ERROR(347u32);
pub const ERROR_MARKED_TO_DISALLOW_WRITES: WIN32_ERROR = WIN32_ERROR(348u32);
pub const ERROR_ENCLAVE_FAILURE: WIN32_ERROR = WIN32_ERROR(349u32);
pub const ERROR_FAIL_NOACTION_REBOOT: WIN32_ERROR = WIN32_ERROR(350u32);
pub const ERROR_FAIL_SHUTDOWN: WIN32_ERROR = WIN32_ERROR(351u32);
pub const ERROR_FAIL_RESTART: WIN32_ERROR = WIN32_ERROR(352u32);
pub const ERROR_MAX_SESSIONS_REACHED: WIN32_ERROR = WIN32_ERROR(353u32);
pub const ERROR_NETWORK_ACCESS_DENIED_EDP: WIN32_ERROR = WIN32_ERROR(354u32);
pub const ERROR_DEVICE_HINT_NAME_BUFFER_TOO_SMALL: WIN32_ERROR = WIN32_ERROR(355u32);
pub const ERROR_EDP_POLICY_DENIES_OPERATION: WIN32_ERROR = WIN32_ERROR(356u32);
pub const ERROR_EDP_DPL_POLICY_CANT_BE_SATISFIED: WIN32_ERROR = WIN32_ERROR(357u32);
pub const ERROR_CLOUD_FILE_SYNC_ROOT_METADATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(358u32);
pub const ERROR_DEVICE_IN_MAINTENANCE: WIN32_ERROR = WIN32_ERROR(359u32);
pub const ERROR_NOT_SUPPORTED_ON_DAX: WIN32_ERROR = WIN32_ERROR(360u32);
pub const ERROR_DAX_MAPPING_EXISTS: WIN32_ERROR = WIN32_ERROR(361u32);
pub const ERROR_CLOUD_FILE_PROVIDER_NOT_RUNNING: WIN32_ERROR = WIN32_ERROR(362u32);
pub const ERROR_CLOUD_FILE_METADATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(363u32);
pub const ERROR_CLOUD_FILE_METADATA_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(364u32);
pub const ERROR_CLOUD_FILE_PROPERTY_BLOB_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(365u32);
pub const ERROR_CLOUD_FILE_PROPERTY_BLOB_CHECKSUM_MISMATCH: WIN32_ERROR = WIN32_ERROR(366u32);
pub const ERROR_CHILD_PROCESS_BLOCKED: WIN32_ERROR = WIN32_ERROR(367u32);
pub const ERROR_STORAGE_LOST_DATA_PERSISTENCE: WIN32_ERROR = WIN32_ERROR(368u32);
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(369u32);
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_METADATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(370u32);
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_BUSY: WIN32_ERROR = WIN32_ERROR(371u32);
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_PROVIDER_UNKNOWN: WIN32_ERROR = WIN32_ERROR(372u32);
pub const ERROR_GDI_HANDLE_LEAK: WIN32_ERROR = WIN32_ERROR(373u32);
pub const ERROR_CLOUD_FILE_TOO_MANY_PROPERTY_BLOBS: WIN32_ERROR = WIN32_ERROR(374u32);
pub const ERROR_CLOUD_FILE_PROPERTY_VERSION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(375u32);
pub const ERROR_NOT_A_CLOUD_FILE: WIN32_ERROR = WIN32_ERROR(376u32);
pub const ERROR_CLOUD_FILE_NOT_IN_SYNC: WIN32_ERROR = WIN32_ERROR(377u32);
pub const ERROR_CLOUD_FILE_ALREADY_CONNECTED: WIN32_ERROR = WIN32_ERROR(378u32);
pub const ERROR_CLOUD_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(379u32);
pub const ERROR_CLOUD_FILE_INVALID_REQUEST: WIN32_ERROR = WIN32_ERROR(380u32);
pub const ERROR_CLOUD_FILE_READ_ONLY_VOLUME: WIN32_ERROR = WIN32_ERROR(381u32);
pub const ERROR_CLOUD_FILE_CONNECTED_PROVIDER_ONLY: WIN32_ERROR = WIN32_ERROR(382u32);
pub const ERROR_CLOUD_FILE_VALIDATION_FAILED: WIN32_ERROR = WIN32_ERROR(383u32);
pub const ERROR_SMB1_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(384u32);
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_INVALID_OPERATION: WIN32_ERROR = WIN32_ERROR(385u32);
pub const ERROR_CLOUD_FILE_AUTHENTICATION_FAILED: WIN32_ERROR = WIN32_ERROR(386u32);
pub const ERROR_CLOUD_FILE_INSUFFICIENT_RESOURCES: WIN32_ERROR = WIN32_ERROR(387u32);
pub const ERROR_CLOUD_FILE_NETWORK_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(388u32);
pub const ERROR_CLOUD_FILE_UNSUCCESSFUL: WIN32_ERROR = WIN32_ERROR(389u32);
pub const ERROR_CLOUD_FILE_NOT_UNDER_SYNC_ROOT: WIN32_ERROR = WIN32_ERROR(390u32);
pub const ERROR_CLOUD_FILE_IN_USE: WIN32_ERROR = WIN32_ERROR(391u32);
pub const ERROR_CLOUD_FILE_PINNED: WIN32_ERROR = WIN32_ERROR(392u32);
pub const ERROR_CLOUD_FILE_REQUEST_ABORTED: WIN32_ERROR = WIN32_ERROR(393u32);
pub const ERROR_CLOUD_FILE_PROPERTY_CORRUPT: WIN32_ERROR = WIN32_ERROR(394u32);
pub const ERROR_CLOUD_FILE_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(395u32);
pub const ERROR_CLOUD_FILE_INCOMPATIBLE_HARDLINKS: WIN32_ERROR = WIN32_ERROR(396u32);
pub const ERROR_CLOUD_FILE_PROPERTY_LOCK_CONFLICT: WIN32_ERROR = WIN32_ERROR(397u32);
pub const ERROR_CLOUD_FILE_REQUEST_CANCELED: WIN32_ERROR = WIN32_ERROR(398u32);
pub const ERROR_EXTERNAL_SYSKEY_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(399u32);
pub const ERROR_THREAD_MODE_ALREADY_BACKGROUND: WIN32_ERROR = WIN32_ERROR(400u32);
pub const ERROR_THREAD_MODE_NOT_BACKGROUND: WIN32_ERROR = WIN32_ERROR(401u32);
pub const ERROR_PROCESS_MODE_ALREADY_BACKGROUND: WIN32_ERROR = WIN32_ERROR(402u32);
pub const ERROR_PROCESS_MODE_NOT_BACKGROUND: WIN32_ERROR = WIN32_ERROR(403u32);
pub const ERROR_CLOUD_FILE_PROVIDER_TERMINATED: WIN32_ERROR = WIN32_ERROR(404u32);
pub const ERROR_NOT_A_CLOUD_SYNC_ROOT: WIN32_ERROR = WIN32_ERROR(405u32);
pub const ERROR_FILE_PROTECTED_UNDER_DPL: WIN32_ERROR = WIN32_ERROR(406u32);
pub const ERROR_VOLUME_NOT_CLUSTER_ALIGNED: WIN32_ERROR = WIN32_ERROR(407u32);
pub const ERROR_NO_PHYSICALLY_ALIGNED_FREE_SPACE_FOUND: WIN32_ERROR = WIN32_ERROR(408u32);
pub const ERROR_APPX_FILE_NOT_ENCRYPTED: WIN32_ERROR = WIN32_ERROR(409u32);
pub const ERROR_RWRAW_ENCRYPTED_FILE_NOT_ENCRYPTED: WIN32_ERROR = WIN32_ERROR(410u32);
pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_FILEOFFSET: WIN32_ERROR = WIN32_ERROR(411u32);
pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_FILERANGE: WIN32_ERROR = WIN32_ERROR(412u32);
pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_PARAMETER: WIN32_ERROR = WIN32_ERROR(413u32);
pub const ERROR_LINUX_SUBSYSTEM_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(414u32);
pub const ERROR_FT_READ_FAILURE: WIN32_ERROR = WIN32_ERROR(415u32);
pub const ERROR_STORAGE_RESERVE_ID_INVALID: WIN32_ERROR = WIN32_ERROR(416u32);
pub const ERROR_STORAGE_RESERVE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(417u32);
pub const ERROR_STORAGE_RESERVE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(418u32);
pub const ERROR_STORAGE_RESERVE_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(419u32);
pub const ERROR_NOT_A_DAX_VOLUME: WIN32_ERROR = WIN32_ERROR(420u32);
pub const ERROR_NOT_DAX_MAPPABLE: WIN32_ERROR = WIN32_ERROR(421u32);
pub const ERROR_TIME_SENSITIVE_THREAD: WIN32_ERROR = WIN32_ERROR(422u32);
pub const ERROR_DPL_NOT_SUPPORTED_FOR_USER: WIN32_ERROR = WIN32_ERROR(423u32);
pub const ERROR_CASE_DIFFERING_NAMES_IN_DIR: WIN32_ERROR = WIN32_ERROR(424u32);
pub const ERROR_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(425u32);
pub const ERROR_CLOUD_FILE_REQUEST_TIMEOUT: WIN32_ERROR = WIN32_ERROR(426u32);
pub const ERROR_NO_TASK_QUEUE: WIN32_ERROR = WIN32_ERROR(427u32);
pub const ERROR_SRC_SRV_DLL_LOAD_FAILED: WIN32_ERROR = WIN32_ERROR(428u32);
pub const ERROR_NOT_SUPPORTED_WITH_BTT: WIN32_ERROR = WIN32_ERROR(429u32);
pub const ERROR_ENCRYPTION_DISABLED: WIN32_ERROR = WIN32_ERROR(430u32);
pub const ERROR_ENCRYPTING_METADATA_DISALLOWED: WIN32_ERROR = WIN32_ERROR(431u32);
pub const ERROR_CANT_CLEAR_ENCRYPTION_FLAG: WIN32_ERROR = WIN32_ERROR(432u32);
pub const ERROR_NO_SUCH_DEVICE: WIN32_ERROR = WIN32_ERROR(433u32);
pub const ERROR_CLOUD_FILE_DEHYDRATION_DISALLOWED: WIN32_ERROR = WIN32_ERROR(434u32);
pub const ERROR_FILE_SNAP_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(435u32);
pub const ERROR_FILE_SNAP_USER_SECTION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(436u32);
pub const ERROR_FILE_SNAP_MODIFY_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(437u32);
pub const ERROR_FILE_SNAP_IO_NOT_COORDINATED: WIN32_ERROR = WIN32_ERROR(438u32);
pub const ERROR_FILE_SNAP_UNEXPECTED_ERROR: WIN32_ERROR = WIN32_ERROR(439u32);
pub const ERROR_FILE_SNAP_INVALID_PARAMETER: WIN32_ERROR = WIN32_ERROR(440u32);
pub const ERROR_UNSATISFIED_DEPENDENCIES: WIN32_ERROR = WIN32_ERROR(441u32);
pub const ERROR_CASE_SENSITIVE_PATH: WIN32_ERROR = WIN32_ERROR(442u32);
pub const ERROR_UNEXPECTED_NTCACHEMANAGER_ERROR: WIN32_ERROR = WIN32_ERROR(443u32);
pub const ERROR_LINUX_SUBSYSTEM_UPDATE_REQUIRED: WIN32_ERROR = WIN32_ERROR(444u32);
pub const ERROR_DLP_POLICY_WARNS_AGAINST_OPERATION: WIN32_ERROR = WIN32_ERROR(445u32);
pub const ERROR_DLP_POLICY_DENIES_OPERATION: WIN32_ERROR = WIN32_ERROR(446u32);
pub const ERROR_SECURITY_DENIES_OPERATION: WIN32_ERROR = WIN32_ERROR(447u32);
pub const ERROR_UNTRUSTED_MOUNT_POINT: WIN32_ERROR = WIN32_ERROR(448u32);
pub const ERROR_DLP_POLICY_SILENTLY_FAIL: WIN32_ERROR = WIN32_ERROR(449u32);
pub const ERROR_CAPAUTHZ_NOT_DEVUNLOCKED: WIN32_ERROR = WIN32_ERROR(450u32);
pub const ERROR_CAPAUTHZ_CHANGE_TYPE: WIN32_ERROR = WIN32_ERROR(451u32);
pub const ERROR_CAPAUTHZ_NOT_PROVISIONED: WIN32_ERROR = WIN32_ERROR(452u32);
pub const ERROR_CAPAUTHZ_NOT_AUTHORIZED: WIN32_ERROR = WIN32_ERROR(453u32);
pub const ERROR_CAPAUTHZ_NO_POLICY: WIN32_ERROR = WIN32_ERROR(454u32);
pub const ERROR_CAPAUTHZ_DB_CORRUPTED: WIN32_ERROR = WIN32_ERROR(455u32);
pub const ERROR_CAPAUTHZ_SCCD_INVALID_CATALOG: WIN32_ERROR = WIN32_ERROR(456u32);
pub const ERROR_CAPAUTHZ_SCCD_NO_AUTH_ENTITY: WIN32_ERROR = WIN32_ERROR(457u32);
pub const ERROR_CAPAUTHZ_SCCD_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(458u32);
pub const ERROR_CAPAUTHZ_SCCD_DEV_MODE_REQUIRED: WIN32_ERROR = WIN32_ERROR(459u32);
pub const ERROR_CAPAUTHZ_SCCD_NO_CAPABILITY_MATCH: WIN32_ERROR = WIN32_ERROR(460u32);
pub const ERROR_CIMFS_IMAGE_CORRUPT: WIN32_ERROR = WIN32_ERROR(470u32);
pub const ERROR_CIMFS_IMAGE_VERSION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(471u32);
pub const ERROR_STORAGE_STACK_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(472u32);
pub const ERROR_INSUFFICIENT_VIRTUAL_ADDR_RESOURCES: WIN32_ERROR = WIN32_ERROR(473u32);
pub const ERROR_INDEX_OUT_OF_BOUNDS: WIN32_ERROR = WIN32_ERROR(474u32);
pub const ERROR_PNP_QUERY_REMOVE_DEVICE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(480u32);
pub const ERROR_PNP_QUERY_REMOVE_RELATED_DEVICE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(481u32);
pub const ERROR_PNP_QUERY_REMOVE_UNRELATED_DEVICE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(482u32);
pub const ERROR_DEVICE_HARDWARE_ERROR: WIN32_ERROR = WIN32_ERROR(483u32);
pub const ERROR_INVALID_ADDRESS: WIN32_ERROR = WIN32_ERROR(487u32);
pub const ERROR_HAS_SYSTEM_CRITICAL_FILES: WIN32_ERROR = WIN32_ERROR(488u32);
pub const ERROR_ENCRYPTED_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(489u32);
pub const ERROR_SPARSE_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(490u32);
pub const ERROR_PAGEFILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(491u32);
pub const ERROR_VOLUME_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(492u32);
pub const ERROR_NOT_SUPPORTED_WITH_BYPASSIO: WIN32_ERROR = WIN32_ERROR(493u32);
pub const ERROR_NO_BYPASSIO_DRIVER_SUPPORT: WIN32_ERROR = WIN32_ERROR(494u32);
pub const ERROR_NOT_SUPPORTED_WITH_ENCRYPTION: WIN32_ERROR = WIN32_ERROR(495u32);
pub const ERROR_NOT_SUPPORTED_WITH_COMPRESSION: WIN32_ERROR = WIN32_ERROR(496u32);
pub const ERROR_NOT_SUPPORTED_WITH_REPLICATION: WIN32_ERROR = WIN32_ERROR(497u32);
pub const ERROR_NOT_SUPPORTED_WITH_DEDUPLICATION: WIN32_ERROR = WIN32_ERROR(498u32);
pub const ERROR_NOT_SUPPORTED_WITH_AUDITING: WIN32_ERROR = WIN32_ERROR(499u32);
pub const ERROR_USER_PROFILE_LOAD: WIN32_ERROR = WIN32_ERROR(500u32);
pub const ERROR_SESSION_KEY_TOO_SHORT: WIN32_ERROR = WIN32_ERROR(501u32);
pub const ERROR_ACCESS_DENIED_APPDATA: WIN32_ERROR = WIN32_ERROR(502u32);
pub const ERROR_NOT_SUPPORTED_WITH_MONITORING: WIN32_ERROR = WIN32_ERROR(503u32);
pub const ERROR_NOT_SUPPORTED_WITH_SNAPSHOT: WIN32_ERROR = WIN32_ERROR(504u32);
pub const ERROR_NOT_SUPPORTED_WITH_VIRTUALIZATION: WIN32_ERROR = WIN32_ERROR(505u32);
pub const ERROR_BYPASSIO_FLT_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(506u32);
pub const ERROR_DEVICE_RESET_REQUIRED: WIN32_ERROR = WIN32_ERROR(507u32);
pub const ERROR_VOLUME_WRITE_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(508u32);
pub const ERROR_ARITHMETIC_OVERFLOW: WIN32_ERROR = WIN32_ERROR(534u32);
pub const ERROR_PIPE_CONNECTED: WIN32_ERROR = WIN32_ERROR(535u32);
pub const ERROR_PIPE_LISTENING: WIN32_ERROR = WIN32_ERROR(536u32);
pub const ERROR_VERIFIER_STOP: WIN32_ERROR = WIN32_ERROR(537u32);
pub const ERROR_ABIOS_ERROR: WIN32_ERROR = WIN32_ERROR(538u32);
pub const ERROR_WX86_WARNING: WIN32_ERROR = WIN32_ERROR(539u32);
pub const ERROR_WX86_ERROR: WIN32_ERROR = WIN32_ERROR(540u32);
pub const ERROR_TIMER_NOT_CANCELED: WIN32_ERROR = WIN32_ERROR(541u32);
pub const ERROR_UNWIND: WIN32_ERROR = WIN32_ERROR(542u32);
pub const ERROR_BAD_STACK: WIN32_ERROR = WIN32_ERROR(543u32);
pub const ERROR_INVALID_UNWIND_TARGET: WIN32_ERROR = WIN32_ERROR(544u32);
pub const ERROR_INVALID_PORT_ATTRIBUTES: WIN32_ERROR = WIN32_ERROR(545u32);
pub const ERROR_PORT_MESSAGE_TOO_LONG: WIN32_ERROR = WIN32_ERROR(546u32);
pub const ERROR_INVALID_QUOTA_LOWER: WIN32_ERROR = WIN32_ERROR(547u32);
pub const ERROR_DEVICE_ALREADY_ATTACHED: WIN32_ERROR = WIN32_ERROR(548u32);
pub const ERROR_INSTRUCTION_MISALIGNMENT: WIN32_ERROR = WIN32_ERROR(549u32);
pub const ERROR_PROFILING_NOT_STARTED: WIN32_ERROR = WIN32_ERROR(550u32);
pub const ERROR_PROFILING_NOT_STOPPED: WIN32_ERROR = WIN32_ERROR(551u32);
pub const ERROR_COULD_NOT_INTERPRET: WIN32_ERROR = WIN32_ERROR(552u32);
pub const ERROR_PROFILING_AT_LIMIT: WIN32_ERROR = WIN32_ERROR(553u32);
pub const ERROR_CANT_WAIT: WIN32_ERROR = WIN32_ERROR(554u32);
pub const ERROR_CANT_TERMINATE_SELF: WIN32_ERROR = WIN32_ERROR(555u32);
pub const ERROR_UNEXPECTED_MM_CREATE_ERR: WIN32_ERROR = WIN32_ERROR(556u32);
pub const ERROR_UNEXPECTED_MM_MAP_ERROR: WIN32_ERROR = WIN32_ERROR(557u32);
pub const ERROR_UNEXPECTED_MM_EXTEND_ERR: WIN32_ERROR = WIN32_ERROR(558u32);
pub const ERROR_BAD_FUNCTION_TABLE: WIN32_ERROR = WIN32_ERROR(559u32);
pub const ERROR_NO_GUID_TRANSLATION: WIN32_ERROR = WIN32_ERROR(560u32);
pub const ERROR_INVALID_LDT_SIZE: WIN32_ERROR = WIN32_ERROR(561u32);
pub const ERROR_INVALID_LDT_OFFSET: WIN32_ERROR = WIN32_ERROR(563u32);
pub const ERROR_INVALID_LDT_DESCRIPTOR: WIN32_ERROR = WIN32_ERROR(564u32);
pub const ERROR_TOO_MANY_THREADS: WIN32_ERROR = WIN32_ERROR(565u32);
pub const ERROR_THREAD_NOT_IN_PROCESS: WIN32_ERROR = WIN32_ERROR(566u32);
pub const ERROR_PAGEFILE_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(567u32);
pub const ERROR_LOGON_SERVER_CONFLICT: WIN32_ERROR = WIN32_ERROR(568u32);
pub const ERROR_SYNCHRONIZATION_REQUIRED: WIN32_ERROR = WIN32_ERROR(569u32);
pub const ERROR_NET_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(570u32);
pub const ERROR_IO_PRIVILEGE_FAILED: WIN32_ERROR = WIN32_ERROR(571u32);
pub const ERROR_CONTROL_C_EXIT: WIN32_ERROR = WIN32_ERROR(572u32);
pub const ERROR_MISSING_SYSTEMFILE: WIN32_ERROR = WIN32_ERROR(573u32);
pub const ERROR_UNHANDLED_EXCEPTION: WIN32_ERROR = WIN32_ERROR(574u32);
pub const ERROR_APP_INIT_FAILURE: WIN32_ERROR = WIN32_ERROR(575u32);
pub const ERROR_PAGEFILE_CREATE_FAILED: WIN32_ERROR = WIN32_ERROR(576u32);
pub const ERROR_INVALID_IMAGE_HASH: WIN32_ERROR = WIN32_ERROR(577u32);
pub const ERROR_NO_PAGEFILE: WIN32_ERROR = WIN32_ERROR(578u32);
pub const ERROR_ILLEGAL_FLOAT_CONTEXT: WIN32_ERROR = WIN32_ERROR(579u32);
pub const ERROR_NO_EVENT_PAIR: WIN32_ERROR = WIN32_ERROR(580u32);
pub const ERROR_DOMAIN_CTRLR_CONFIG_ERROR: WIN32_ERROR = WIN32_ERROR(581u32);
pub const ERROR_ILLEGAL_CHARACTER: WIN32_ERROR = WIN32_ERROR(582u32);
pub const ERROR_UNDEFINED_CHARACTER: WIN32_ERROR = WIN32_ERROR(583u32);
pub const ERROR_FLOPPY_VOLUME: WIN32_ERROR = WIN32_ERROR(584u32);
pub const ERROR_BIOS_FAILED_TO_CONNECT_INTERRUPT: WIN32_ERROR = WIN32_ERROR(585u32);
pub const ERROR_BACKUP_CONTROLLER: WIN32_ERROR = WIN32_ERROR(586u32);
pub const ERROR_MUTANT_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(587u32);
pub const ERROR_FS_DRIVER_REQUIRED: WIN32_ERROR = WIN32_ERROR(588u32);
pub const ERROR_CANNOT_LOAD_REGISTRY_FILE: WIN32_ERROR = WIN32_ERROR(589u32);
pub const ERROR_DEBUG_ATTACH_FAILED: WIN32_ERROR = WIN32_ERROR(590u32);
pub const ERROR_SYSTEM_PROCESS_TERMINATED: WIN32_ERROR = WIN32_ERROR(591u32);
pub const ERROR_DATA_NOT_ACCEPTED: WIN32_ERROR = WIN32_ERROR(592u32);
pub const ERROR_VDM_HARD_ERROR: WIN32_ERROR = WIN32_ERROR(593u32);
pub const ERROR_DRIVER_CANCEL_TIMEOUT: WIN32_ERROR = WIN32_ERROR(594u32);
pub const ERROR_REPLY_MESSAGE_MISMATCH: WIN32_ERROR = WIN32_ERROR(595u32);
pub const ERROR_LOST_WRITEBEHIND_DATA: WIN32_ERROR = WIN32_ERROR(596u32);
pub const ERROR_CLIENT_SERVER_PARAMETERS_INVALID: WIN32_ERROR = WIN32_ERROR(597u32);
pub const ERROR_NOT_TINY_STREAM: WIN32_ERROR = WIN32_ERROR(598u32);
pub const ERROR_STACK_OVERFLOW_READ: WIN32_ERROR = WIN32_ERROR(599u32);
pub const ERROR_CONVERT_TO_LARGE: WIN32_ERROR = WIN32_ERROR(600u32);
pub const ERROR_FOUND_OUT_OF_SCOPE: WIN32_ERROR = WIN32_ERROR(601u32);
pub const ERROR_ALLOCATE_BUCKET: WIN32_ERROR = WIN32_ERROR(602u32);
pub const ERROR_MARSHALL_OVERFLOW: WIN32_ERROR = WIN32_ERROR(603u32);
pub const ERROR_INVALID_VARIANT: WIN32_ERROR = WIN32_ERROR(604u32);
pub const ERROR_BAD_COMPRESSION_BUFFER: WIN32_ERROR = WIN32_ERROR(605u32);
pub const ERROR_AUDIT_FAILED: WIN32_ERROR = WIN32_ERROR(606u32);
pub const ERROR_TIMER_RESOLUTION_NOT_SET: WIN32_ERROR = WIN32_ERROR(607u32);
pub const ERROR_INSUFFICIENT_LOGON_INFO: WIN32_ERROR = WIN32_ERROR(608u32);
pub const ERROR_BAD_DLL_ENTRYPOINT: WIN32_ERROR = WIN32_ERROR(609u32);
pub const ERROR_BAD_SERVICE_ENTRYPOINT: WIN32_ERROR = WIN32_ERROR(610u32);
pub const ERROR_IP_ADDRESS_CONFLICT1: WIN32_ERROR = WIN32_ERROR(611u32);
pub const ERROR_IP_ADDRESS_CONFLICT2: WIN32_ERROR = WIN32_ERROR(612u32);
pub const ERROR_REGISTRY_QUOTA_LIMIT: WIN32_ERROR = WIN32_ERROR(613u32);
pub const ERROR_NO_CALLBACK_ACTIVE: WIN32_ERROR = WIN32_ERROR(614u32);
pub const ERROR_PWD_TOO_SHORT: WIN32_ERROR = WIN32_ERROR(615u32);
pub const ERROR_PWD_TOO_RECENT: WIN32_ERROR = WIN32_ERROR(616u32);
pub const ERROR_PWD_HISTORY_CONFLICT: WIN32_ERROR = WIN32_ERROR(617u32);
pub const ERROR_UNSUPPORTED_COMPRESSION: WIN32_ERROR = WIN32_ERROR(618u32);
pub const ERROR_INVALID_HW_PROFILE: WIN32_ERROR = WIN32_ERROR(619u32);
pub const ERROR_INVALID_PLUGPLAY_DEVICE_PATH: WIN32_ERROR = WIN32_ERROR(620u32);
pub const ERROR_QUOTA_LIST_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(621u32);
pub const ERROR_EVALUATION_EXPIRATION: WIN32_ERROR = WIN32_ERROR(622u32);
pub const ERROR_ILLEGAL_DLL_RELOCATION: WIN32_ERROR = WIN32_ERROR(623u32);
pub const ERROR_DLL_INIT_FAILED_LOGOFF: WIN32_ERROR = WIN32_ERROR(624u32);
pub const ERROR_VALIDATE_CONTINUE: WIN32_ERROR = WIN32_ERROR(625u32);
pub const ERROR_NO_MORE_MATCHES: WIN32_ERROR = WIN32_ERROR(626u32);
pub const ERROR_RANGE_LIST_CONFLICT: WIN32_ERROR = WIN32_ERROR(627u32);
pub const ERROR_SERVER_SID_MISMATCH: WIN32_ERROR = WIN32_ERROR(628u32);
pub const ERROR_CANT_ENABLE_DENY_ONLY: WIN32_ERROR = WIN32_ERROR(629u32);
pub const ERROR_FLOAT_MULTIPLE_FAULTS: WIN32_ERROR = WIN32_ERROR(630u32);
pub const ERROR_FLOAT_MULTIPLE_TRAPS: WIN32_ERROR = WIN32_ERROR(631u32);
pub const ERROR_NOINTERFACE: WIN32_ERROR = WIN32_ERROR(632u32);
pub const ERROR_DRIVER_FAILED_SLEEP: WIN32_ERROR = WIN32_ERROR(633u32);
pub const ERROR_CORRUPT_SYSTEM_FILE: WIN32_ERROR = WIN32_ERROR(634u32);
pub const ERROR_COMMITMENT_MINIMUM: WIN32_ERROR = WIN32_ERROR(635u32);
pub const ERROR_PNP_RESTART_ENUMERATION: WIN32_ERROR = WIN32_ERROR(636u32);
pub const ERROR_SYSTEM_IMAGE_BAD_SIGNATURE: WIN32_ERROR = WIN32_ERROR(637u32);
pub const ERROR_PNP_REBOOT_REQUIRED: WIN32_ERROR = WIN32_ERROR(638u32);
pub const ERROR_INSUFFICIENT_POWER: WIN32_ERROR = WIN32_ERROR(639u32);
pub const ERROR_MULTIPLE_FAULT_VIOLATION: WIN32_ERROR = WIN32_ERROR(640u32);
pub const ERROR_SYSTEM_SHUTDOWN: WIN32_ERROR = WIN32_ERROR(641u32);
pub const ERROR_PORT_NOT_SET: WIN32_ERROR = WIN32_ERROR(642u32);
pub const ERROR_DS_VERSION_CHECK_FAILURE: WIN32_ERROR = WIN32_ERROR(643u32);
pub const ERROR_RANGE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(644u32);
pub const ERROR_NOT_SAFE_MODE_DRIVER: WIN32_ERROR = WIN32_ERROR(646u32);
pub const ERROR_FAILED_DRIVER_ENTRY: WIN32_ERROR = WIN32_ERROR(647u32);
pub const ERROR_DEVICE_ENUMERATION_ERROR: WIN32_ERROR = WIN32_ERROR(648u32);
pub const ERROR_MOUNT_POINT_NOT_RESOLVED: WIN32_ERROR = WIN32_ERROR(649u32);
pub const ERROR_INVALID_DEVICE_OBJECT_PARAMETER: WIN32_ERROR = WIN32_ERROR(650u32);
pub const ERROR_MCA_OCCURED: WIN32_ERROR = WIN32_ERROR(651u32);
pub const ERROR_DRIVER_DATABASE_ERROR: WIN32_ERROR = WIN32_ERROR(652u32);
pub const ERROR_SYSTEM_HIVE_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(653u32);
pub const ERROR_DRIVER_FAILED_PRIOR_UNLOAD: WIN32_ERROR = WIN32_ERROR(654u32);
pub const ERROR_VOLSNAP_PREPARE_HIBERNATE: WIN32_ERROR = WIN32_ERROR(655u32);
pub const ERROR_HIBERNATION_FAILURE: WIN32_ERROR = WIN32_ERROR(656u32);
pub const ERROR_PWD_TOO_LONG: WIN32_ERROR = WIN32_ERROR(657u32);
pub const ERROR_FILE_SYSTEM_LIMITATION: WIN32_ERROR = WIN32_ERROR(665u32);
pub const ERROR_ASSERTION_FAILURE: WIN32_ERROR = WIN32_ERROR(668u32);
pub const ERROR_ACPI_ERROR: WIN32_ERROR = WIN32_ERROR(669u32);
pub const ERROR_WOW_ASSERTION: WIN32_ERROR = WIN32_ERROR(670u32);
pub const ERROR_PNP_BAD_MPS_TABLE: WIN32_ERROR = WIN32_ERROR(671u32);
pub const ERROR_PNP_TRANSLATION_FAILED: WIN32_ERROR = WIN32_ERROR(672u32);
pub const ERROR_PNP_IRQ_TRANSLATION_FAILED: WIN32_ERROR = WIN32_ERROR(673u32);
pub const ERROR_PNP_INVALID_ID: WIN32_ERROR = WIN32_ERROR(674u32);
pub const ERROR_WAKE_SYSTEM_DEBUGGER: WIN32_ERROR = WIN32_ERROR(675u32);
pub const ERROR_HANDLES_CLOSED: WIN32_ERROR = WIN32_ERROR(676u32);
pub const ERROR_EXTRANEOUS_INFORMATION: WIN32_ERROR = WIN32_ERROR(677u32);
pub const ERROR_RXACT_COMMIT_NECESSARY: WIN32_ERROR = WIN32_ERROR(678u32);
pub const ERROR_MEDIA_CHECK: WIN32_ERROR = WIN32_ERROR(679u32);
pub const ERROR_GUID_SUBSTITUTION_MADE: WIN32_ERROR = WIN32_ERROR(680u32);
pub const ERROR_STOPPED_ON_SYMLINK: WIN32_ERROR = WIN32_ERROR(681u32);
pub const ERROR_LONGJUMP: WIN32_ERROR = WIN32_ERROR(682u32);
pub const ERROR_PLUGPLAY_QUERY_VETOED: WIN32_ERROR = WIN32_ERROR(683u32);
pub const ERROR_UNWIND_CONSOLIDATE: WIN32_ERROR = WIN32_ERROR(684u32);
pub const ERROR_REGISTRY_HIVE_RECOVERED: WIN32_ERROR = WIN32_ERROR(685u32);
pub const ERROR_DLL_MIGHT_BE_INSECURE: WIN32_ERROR = WIN32_ERROR(686u32);
pub const ERROR_DLL_MIGHT_BE_INCOMPATIBLE: WIN32_ERROR = WIN32_ERROR(687u32);
pub const ERROR_DBG_EXCEPTION_NOT_HANDLED: WIN32_ERROR = WIN32_ERROR(688u32);
pub const ERROR_DBG_REPLY_LATER: WIN32_ERROR = WIN32_ERROR(689u32);
pub const ERROR_DBG_UNABLE_TO_PROVIDE_HANDLE: WIN32_ERROR = WIN32_ERROR(690u32);
pub const ERROR_DBG_TERMINATE_THREAD: WIN32_ERROR = WIN32_ERROR(691u32);
pub const ERROR_DBG_TERMINATE_PROCESS: WIN32_ERROR = WIN32_ERROR(692u32);
pub const ERROR_DBG_CONTROL_C: WIN32_ERROR = WIN32_ERROR(693u32);
pub const ERROR_DBG_PRINTEXCEPTION_C: WIN32_ERROR = WIN32_ERROR(694u32);
pub const ERROR_DBG_RIPEXCEPTION: WIN32_ERROR = WIN32_ERROR(695u32);
pub const ERROR_DBG_CONTROL_BREAK: WIN32_ERROR = WIN32_ERROR(696u32);
pub const ERROR_DBG_COMMAND_EXCEPTION: WIN32_ERROR = WIN32_ERROR(697u32);
pub const ERROR_OBJECT_NAME_EXISTS: WIN32_ERROR = WIN32_ERROR(698u32);
pub const ERROR_THREAD_WAS_SUSPENDED: WIN32_ERROR = WIN32_ERROR(699u32);
pub const ERROR_IMAGE_NOT_AT_BASE: WIN32_ERROR = WIN32_ERROR(700u32);
pub const ERROR_RXACT_STATE_CREATED: WIN32_ERROR = WIN32_ERROR(701u32);
pub const ERROR_SEGMENT_NOTIFICATION: WIN32_ERROR = WIN32_ERROR(702u32);
pub const ERROR_BAD_CURRENT_DIRECTORY: WIN32_ERROR = WIN32_ERROR(703u32);
pub const ERROR_FT_READ_RECOVERY_FROM_BACKUP: WIN32_ERROR = WIN32_ERROR(704u32);
pub const ERROR_FT_WRITE_RECOVERY: WIN32_ERROR = WIN32_ERROR(705u32);
pub const ERROR_IMAGE_MACHINE_TYPE_MISMATCH: WIN32_ERROR = WIN32_ERROR(706u32);
pub const ERROR_RECEIVE_PARTIAL: WIN32_ERROR = WIN32_ERROR(707u32);
pub const ERROR_RECEIVE_EXPEDITED: WIN32_ERROR = WIN32_ERROR(708u32);
pub const ERROR_RECEIVE_PARTIAL_EXPEDITED: WIN32_ERROR = WIN32_ERROR(709u32);
pub const ERROR_EVENT_DONE: WIN32_ERROR = WIN32_ERROR(710u32);
pub const ERROR_EVENT_PENDING: WIN32_ERROR = WIN32_ERROR(711u32);
pub const ERROR_CHECKING_FILE_SYSTEM: WIN32_ERROR = WIN32_ERROR(712u32);
pub const ERROR_FATAL_APP_EXIT: WIN32_ERROR = WIN32_ERROR(713u32);
pub const ERROR_PREDEFINED_HANDLE: WIN32_ERROR = WIN32_ERROR(714u32);
pub const ERROR_WAS_UNLOCKED: WIN32_ERROR = WIN32_ERROR(715u32);
pub const ERROR_SERVICE_NOTIFICATION: WIN32_ERROR = WIN32_ERROR(716u32);
pub const ERROR_WAS_LOCKED: WIN32_ERROR = WIN32_ERROR(717u32);
pub const ERROR_LOG_HARD_ERROR: WIN32_ERROR = WIN32_ERROR(718u32);
pub const ERROR_ALREADY_WIN32: WIN32_ERROR = WIN32_ERROR(719u32);
pub const ERROR_IMAGE_MACHINE_TYPE_MISMATCH_EXE: WIN32_ERROR = WIN32_ERROR(720u32);
pub const ERROR_NO_YIELD_PERFORMED: WIN32_ERROR = WIN32_ERROR(721u32);
pub const ERROR_TIMER_RESUME_IGNORED: WIN32_ERROR = WIN32_ERROR(722u32);
pub const ERROR_ARBITRATION_UNHANDLED: WIN32_ERROR = WIN32_ERROR(723u32);
pub const ERROR_CARDBUS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(724u32);
pub const ERROR_MP_PROCESSOR_MISMATCH: WIN32_ERROR = WIN32_ERROR(725u32);
pub const ERROR_HIBERNATED: WIN32_ERROR = WIN32_ERROR(726u32);
pub const ERROR_RESUME_HIBERNATION: WIN32_ERROR = WIN32_ERROR(727u32);
pub const ERROR_FIRMWARE_UPDATED: WIN32_ERROR = WIN32_ERROR(728u32);
pub const ERROR_DRIVERS_LEAKING_LOCKED_PAGES: WIN32_ERROR = WIN32_ERROR(729u32);
pub const ERROR_WAKE_SYSTEM: WIN32_ERROR = WIN32_ERROR(730u32);
pub const ERROR_WAIT_1: WIN32_ERROR = WIN32_ERROR(731u32);
pub const ERROR_WAIT_2: WIN32_ERROR = WIN32_ERROR(732u32);
pub const ERROR_WAIT_3: WIN32_ERROR = WIN32_ERROR(733u32);
pub const ERROR_WAIT_63: WIN32_ERROR = WIN32_ERROR(734u32);
pub const ERROR_ABANDONED_WAIT_0: WIN32_ERROR = WIN32_ERROR(735u32);
pub const ERROR_ABANDONED_WAIT_63: WIN32_ERROR = WIN32_ERROR(736u32);
pub const ERROR_USER_APC: WIN32_ERROR = WIN32_ERROR(737u32);
pub const ERROR_KERNEL_APC: WIN32_ERROR = WIN32_ERROR(738u32);
pub const ERROR_ALERTED: WIN32_ERROR = WIN32_ERROR(739u32);
pub const ERROR_ELEVATION_REQUIRED: WIN32_ERROR = WIN32_ERROR(740u32);
pub const ERROR_REPARSE: WIN32_ERROR = WIN32_ERROR(741u32);
pub const ERROR_OPLOCK_BREAK_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(742u32);
pub const ERROR_VOLUME_MOUNTED: WIN32_ERROR = WIN32_ERROR(743u32);
pub const ERROR_RXACT_COMMITTED: WIN32_ERROR = WIN32_ERROR(744u32);
pub const ERROR_NOTIFY_CLEANUP: WIN32_ERROR = WIN32_ERROR(745u32);
pub const ERROR_PRIMARY_TRANSPORT_CONNECT_FAILED: WIN32_ERROR = WIN32_ERROR(746u32);
pub const ERROR_PAGE_FAULT_TRANSITION: WIN32_ERROR = WIN32_ERROR(747u32);
pub const ERROR_PAGE_FAULT_DEMAND_ZERO: WIN32_ERROR = WIN32_ERROR(748u32);
pub const ERROR_PAGE_FAULT_COPY_ON_WRITE: WIN32_ERROR = WIN32_ERROR(749u32);
pub const ERROR_PAGE_FAULT_GUARD_PAGE: WIN32_ERROR = WIN32_ERROR(750u32);
pub const ERROR_PAGE_FAULT_PAGING_FILE: WIN32_ERROR = WIN32_ERROR(751u32);
pub const ERROR_CACHE_PAGE_LOCKED: WIN32_ERROR = WIN32_ERROR(752u32);
pub const ERROR_CRASH_DUMP: WIN32_ERROR = WIN32_ERROR(753u32);
pub const ERROR_BUFFER_ALL_ZEROS: WIN32_ERROR = WIN32_ERROR(754u32);
pub const ERROR_REPARSE_OBJECT: WIN32_ERROR = WIN32_ERROR(755u32);
pub const ERROR_RESOURCE_REQUIREMENTS_CHANGED: WIN32_ERROR = WIN32_ERROR(756u32);
pub const ERROR_TRANSLATION_COMPLETE: WIN32_ERROR = WIN32_ERROR(757u32);
pub const ERROR_NOTHING_TO_TERMINATE: WIN32_ERROR = WIN32_ERROR(758u32);
pub const ERROR_PROCESS_NOT_IN_JOB: WIN32_ERROR = WIN32_ERROR(759u32);
pub const ERROR_PROCESS_IN_JOB: WIN32_ERROR = WIN32_ERROR(760u32);
pub const ERROR_VOLSNAP_HIBERNATE_READY: WIN32_ERROR = WIN32_ERROR(761u32);
pub const ERROR_FSFILTER_OP_COMPLETED_SUCCESSFULLY: WIN32_ERROR = WIN32_ERROR(762u32);
pub const ERROR_INTERRUPT_VECTOR_ALREADY_CONNECTED: WIN32_ERROR = WIN32_ERROR(763u32);
pub const ERROR_INTERRUPT_STILL_CONNECTED: WIN32_ERROR = WIN32_ERROR(764u32);
pub const ERROR_WAIT_FOR_OPLOCK: WIN32_ERROR = WIN32_ERROR(765u32);
pub const ERROR_DBG_EXCEPTION_HANDLED: WIN32_ERROR = WIN32_ERROR(766u32);
pub const ERROR_DBG_CONTINUE: WIN32_ERROR = WIN32_ERROR(767u32);
pub const ERROR_CALLBACK_POP_STACK: WIN32_ERROR = WIN32_ERROR(768u32);
pub const ERROR_COMPRESSION_DISABLED: WIN32_ERROR = WIN32_ERROR(769u32);
pub const ERROR_CANTFETCHBACKWARDS: WIN32_ERROR = WIN32_ERROR(770u32);
pub const ERROR_CANTSCROLLBACKWARDS: WIN32_ERROR = WIN32_ERROR(771u32);
pub const ERROR_ROWSNOTRELEASED: WIN32_ERROR = WIN32_ERROR(772u32);
pub const ERROR_BAD_ACCESSOR_FLAGS: WIN32_ERROR = WIN32_ERROR(773u32);
pub const ERROR_ERRORS_ENCOUNTERED: WIN32_ERROR = WIN32_ERROR(774u32);
pub const ERROR_NOT_CAPABLE: WIN32_ERROR = WIN32_ERROR(775u32);
pub const ERROR_REQUEST_OUT_OF_SEQUENCE: WIN32_ERROR = WIN32_ERROR(776u32);
pub const ERROR_VERSION_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(777u32);
pub const ERROR_BADSTARTPOSITION: WIN32_ERROR = WIN32_ERROR(778u32);
pub const ERROR_MEMORY_HARDWARE: WIN32_ERROR = WIN32_ERROR(779u32);
pub const ERROR_DISK_REPAIR_DISABLED: WIN32_ERROR = WIN32_ERROR(780u32);
pub const ERROR_INSUFFICIENT_RESOURCE_FOR_SPECIFIED_SHARED_SECTION_SIZE: WIN32_ERROR =
    WIN32_ERROR(781u32);
pub const ERROR_SYSTEM_POWERSTATE_TRANSITION: WIN32_ERROR = WIN32_ERROR(782u32);
pub const ERROR_SYSTEM_POWERSTATE_COMPLEX_TRANSITION: WIN32_ERROR = WIN32_ERROR(783u32);
pub const ERROR_MCA_EXCEPTION: WIN32_ERROR = WIN32_ERROR(784u32);
pub const ERROR_ACCESS_AUDIT_BY_POLICY: WIN32_ERROR = WIN32_ERROR(785u32);
pub const ERROR_ACCESS_DISABLED_NO_SAFER_UI_BY_POLICY: WIN32_ERROR = WIN32_ERROR(786u32);
pub const ERROR_ABANDON_HIBERFILE: WIN32_ERROR = WIN32_ERROR(787u32);
pub const ERROR_LOST_WRITEBEHIND_DATA_NETWORK_DISCONNECTED: WIN32_ERROR = WIN32_ERROR(788u32);
pub const ERROR_LOST_WRITEBEHIND_DATA_NETWORK_SERVER_ERROR: WIN32_ERROR = WIN32_ERROR(789u32);
pub const ERROR_LOST_WRITEBEHIND_DATA_LOCAL_DISK_ERROR: WIN32_ERROR = WIN32_ERROR(790u32);
pub const ERROR_BAD_MCFG_TABLE: WIN32_ERROR = WIN32_ERROR(791u32);
pub const ERROR_DISK_REPAIR_REDIRECTED: WIN32_ERROR = WIN32_ERROR(792u32);
pub const ERROR_DISK_REPAIR_UNSUCCESSFUL: WIN32_ERROR = WIN32_ERROR(793u32);
pub const ERROR_CORRUPT_LOG_OVERFULL: WIN32_ERROR = WIN32_ERROR(794u32);
pub const ERROR_CORRUPT_LOG_CORRUPTED: WIN32_ERROR = WIN32_ERROR(795u32);
pub const ERROR_CORRUPT_LOG_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(796u32);
pub const ERROR_CORRUPT_LOG_DELETED_FULL: WIN32_ERROR = WIN32_ERROR(797u32);
pub const ERROR_CORRUPT_LOG_CLEARED: WIN32_ERROR = WIN32_ERROR(798u32);
pub const ERROR_ORPHAN_NAME_EXHAUSTED: WIN32_ERROR = WIN32_ERROR(799u32);
pub const ERROR_OPLOCK_SWITCHED_TO_NEW_HANDLE: WIN32_ERROR = WIN32_ERROR(800u32);
pub const ERROR_CANNOT_GRANT_REQUESTED_OPLOCK: WIN32_ERROR = WIN32_ERROR(801u32);
pub const ERROR_CANNOT_BREAK_OPLOCK: WIN32_ERROR = WIN32_ERROR(802u32);
pub const ERROR_OPLOCK_HANDLE_CLOSED: WIN32_ERROR = WIN32_ERROR(803u32);
pub const ERROR_NO_ACE_CONDITION: WIN32_ERROR = WIN32_ERROR(804u32);
pub const ERROR_INVALID_ACE_CONDITION: WIN32_ERROR = WIN32_ERROR(805u32);
pub const ERROR_FILE_HANDLE_REVOKED: WIN32_ERROR = WIN32_ERROR(806u32);
pub const ERROR_IMAGE_AT_DIFFERENT_BASE: WIN32_ERROR = WIN32_ERROR(807u32);
pub const ERROR_ENCRYPTED_IO_NOT_POSSIBLE: WIN32_ERROR = WIN32_ERROR(808u32);
pub const ERROR_FILE_METADATA_OPTIMIZATION_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(809u32);
pub const ERROR_QUOTA_ACTIVITY: WIN32_ERROR = WIN32_ERROR(810u32);
pub const ERROR_HANDLE_REVOKED: WIN32_ERROR = WIN32_ERROR(811u32);
pub const ERROR_CALLBACK_INVOKE_INLINE: WIN32_ERROR = WIN32_ERROR(812u32);
pub const ERROR_CPU_SET_INVALID: WIN32_ERROR = WIN32_ERROR(813u32);
pub const ERROR_ENCLAVE_NOT_TERMINATED: WIN32_ERROR = WIN32_ERROR(814u32);
pub const ERROR_ENCLAVE_VIOLATION: WIN32_ERROR = WIN32_ERROR(815u32);
pub const ERROR_SERVER_TRANSPORT_CONFLICT: WIN32_ERROR = WIN32_ERROR(816u32);
pub const ERROR_CERTIFICATE_VALIDATION_PREFERENCE_CONFLICT: WIN32_ERROR = WIN32_ERROR(817u32);
pub const ERROR_FT_READ_FROM_COPY_FAILURE: WIN32_ERROR = WIN32_ERROR(818u32);
pub const ERROR_SECTION_DIRECT_MAP_ONLY: WIN32_ERROR = WIN32_ERROR(819u32);
pub const ERROR_EA_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(994u32);
pub const ERROR_OPERATION_ABORTED: WIN32_ERROR = WIN32_ERROR(995u32);
pub const ERROR_IO_INCOMPLETE: WIN32_ERROR = WIN32_ERROR(996u32);
pub const ERROR_IO_PENDING: WIN32_ERROR = WIN32_ERROR(997u32);
pub const ERROR_NOACCESS: WIN32_ERROR = WIN32_ERROR(998u32);
pub const ERROR_SWAPERROR: WIN32_ERROR = WIN32_ERROR(999u32);
pub const ERROR_STACK_OVERFLOW: WIN32_ERROR = WIN32_ERROR(1001u32);
pub const ERROR_INVALID_MESSAGE: WIN32_ERROR = WIN32_ERROR(1002u32);
pub const ERROR_CAN_NOT_COMPLETE: WIN32_ERROR = WIN32_ERROR(1003u32);
pub const ERROR_INVALID_FLAGS: WIN32_ERROR = WIN32_ERROR(1004u32);
pub const ERROR_UNRECOGNIZED_VOLUME: WIN32_ERROR = WIN32_ERROR(1005u32);
pub const ERROR_FILE_INVALID: WIN32_ERROR = WIN32_ERROR(1006u32);
pub const ERROR_FULLSCREEN_MODE: WIN32_ERROR = WIN32_ERROR(1007u32);
pub const ERROR_NO_TOKEN: WIN32_ERROR = WIN32_ERROR(1008u32);
pub const ERROR_BADDB: WIN32_ERROR = WIN32_ERROR(1009u32);
pub const ERROR_BADKEY: WIN32_ERROR = WIN32_ERROR(1010u32);
pub const ERROR_CANTOPEN: WIN32_ERROR = WIN32_ERROR(1011u32);
pub const ERROR_CANTREAD: WIN32_ERROR = WIN32_ERROR(1012u32);
pub const ERROR_CANTWRITE: WIN32_ERROR = WIN32_ERROR(1013u32);
pub const ERROR_REGISTRY_RECOVERED: WIN32_ERROR = WIN32_ERROR(1014u32);
pub const ERROR_REGISTRY_CORRUPT: WIN32_ERROR = WIN32_ERROR(1015u32);
pub const ERROR_REGISTRY_IO_FAILED: WIN32_ERROR = WIN32_ERROR(1016u32);
pub const ERROR_NOT_REGISTRY_FILE: WIN32_ERROR = WIN32_ERROR(1017u32);
pub const ERROR_KEY_DELETED: WIN32_ERROR = WIN32_ERROR(1018u32);
pub const ERROR_NO_LOG_SPACE: WIN32_ERROR = WIN32_ERROR(1019u32);
pub const ERROR_KEY_HAS_CHILDREN: WIN32_ERROR = WIN32_ERROR(1020u32);
pub const ERROR_CHILD_MUST_BE_VOLATILE: WIN32_ERROR = WIN32_ERROR(1021u32);
pub const ERROR_NOTIFY_ENUM_DIR: WIN32_ERROR = WIN32_ERROR(1022u32);
pub const ERROR_DEPENDENT_SERVICES_RUNNING: WIN32_ERROR = WIN32_ERROR(1051u32);
pub const ERROR_INVALID_SERVICE_CONTROL: WIN32_ERROR = WIN32_ERROR(1052u32);
pub const ERROR_SERVICE_REQUEST_TIMEOUT: WIN32_ERROR = WIN32_ERROR(1053u32);
pub const ERROR_SERVICE_NO_THREAD: WIN32_ERROR = WIN32_ERROR(1054u32);
pub const ERROR_SERVICE_DATABASE_LOCKED: WIN32_ERROR = WIN32_ERROR(1055u32);
pub const ERROR_SERVICE_ALREADY_RUNNING: WIN32_ERROR = WIN32_ERROR(1056u32);
pub const ERROR_INVALID_SERVICE_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1057u32);
pub const ERROR_SERVICE_DISABLED: WIN32_ERROR = WIN32_ERROR(1058u32);
pub const ERROR_CIRCULAR_DEPENDENCY: WIN32_ERROR = WIN32_ERROR(1059u32);
pub const ERROR_SERVICE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(1060u32);
pub const ERROR_SERVICE_CANNOT_ACCEPT_CTRL: WIN32_ERROR = WIN32_ERROR(1061u32);
pub const ERROR_SERVICE_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(1062u32);
pub const ERROR_FAILED_SERVICE_CONTROLLER_CONNECT: WIN32_ERROR = WIN32_ERROR(1063u32);
pub const ERROR_EXCEPTION_IN_SERVICE: WIN32_ERROR = WIN32_ERROR(1064u32);
pub const ERROR_DATABASE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(1065u32);
pub const ERROR_SERVICE_SPECIFIC_ERROR: WIN32_ERROR = WIN32_ERROR(1066u32);
pub const ERROR_PROCESS_ABORTED: WIN32_ERROR = WIN32_ERROR(1067u32);
pub const ERROR_SERVICE_DEPENDENCY_FAIL: WIN32_ERROR = WIN32_ERROR(1068u32);
pub const ERROR_SERVICE_LOGON_FAILED: WIN32_ERROR = WIN32_ERROR(1069u32);
pub const ERROR_SERVICE_START_HANG: WIN32_ERROR = WIN32_ERROR(1070u32);
pub const ERROR_INVALID_SERVICE_LOCK: WIN32_ERROR = WIN32_ERROR(1071u32);
pub const ERROR_SERVICE_MARKED_FOR_DELETE: WIN32_ERROR = WIN32_ERROR(1072u32);
pub const ERROR_SERVICE_EXISTS: WIN32_ERROR = WIN32_ERROR(1073u32);
pub const ERROR_ALREADY_RUNNING_LKG: WIN32_ERROR = WIN32_ERROR(1074u32);
pub const ERROR_SERVICE_DEPENDENCY_DELETED: WIN32_ERROR = WIN32_ERROR(1075u32);
pub const ERROR_BOOT_ALREADY_ACCEPTED: WIN32_ERROR = WIN32_ERROR(1076u32);
pub const ERROR_SERVICE_NEVER_STARTED: WIN32_ERROR = WIN32_ERROR(1077u32);
pub const ERROR_DUPLICATE_SERVICE_NAME: WIN32_ERROR = WIN32_ERROR(1078u32);
pub const ERROR_DIFFERENT_SERVICE_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1079u32);
pub const ERROR_CANNOT_DETECT_DRIVER_FAILURE: WIN32_ERROR = WIN32_ERROR(1080u32);
pub const ERROR_CANNOT_DETECT_PROCESS_ABORT: WIN32_ERROR = WIN32_ERROR(1081u32);
pub const ERROR_NO_RECOVERY_PROGRAM: WIN32_ERROR = WIN32_ERROR(1082u32);
pub const ERROR_SERVICE_NOT_IN_EXE: WIN32_ERROR = WIN32_ERROR(1083u32);
pub const ERROR_NOT_SAFEBOOT_SERVICE: WIN32_ERROR = WIN32_ERROR(1084u32);
pub const ERROR_END_OF_MEDIA: WIN32_ERROR = WIN32_ERROR(1100u32);
pub const ERROR_FILEMARK_DETECTED: WIN32_ERROR = WIN32_ERROR(1101u32);
pub const ERROR_BEGINNING_OF_MEDIA: WIN32_ERROR = WIN32_ERROR(1102u32);
pub const ERROR_SETMARK_DETECTED: WIN32_ERROR = WIN32_ERROR(1103u32);
pub const ERROR_NO_DATA_DETECTED: WIN32_ERROR = WIN32_ERROR(1104u32);
pub const ERROR_PARTITION_FAILURE: WIN32_ERROR = WIN32_ERROR(1105u32);
pub const ERROR_INVALID_BLOCK_LENGTH: WIN32_ERROR = WIN32_ERROR(1106u32);
pub const ERROR_DEVICE_NOT_PARTITIONED: WIN32_ERROR = WIN32_ERROR(1107u32);
pub const ERROR_UNABLE_TO_LOCK_MEDIA: WIN32_ERROR = WIN32_ERROR(1108u32);
pub const ERROR_UNABLE_TO_UNLOAD_MEDIA: WIN32_ERROR = WIN32_ERROR(1109u32);
pub const ERROR_MEDIA_CHANGED: WIN32_ERROR = WIN32_ERROR(1110u32);
pub const ERROR_BUS_RESET: WIN32_ERROR = WIN32_ERROR(1111u32);
pub const ERROR_NO_MEDIA_IN_DRIVE: WIN32_ERROR = WIN32_ERROR(1112u32);
pub const ERROR_NO_UNICODE_TRANSLATION: WIN32_ERROR = WIN32_ERROR(1113u32);
pub const ERROR_DLL_INIT_FAILED: WIN32_ERROR = WIN32_ERROR(1114u32);
pub const ERROR_SHUTDOWN_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(1115u32);
pub const ERROR_NO_SHUTDOWN_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(1116u32);
pub const ERROR_IO_DEVICE: WIN32_ERROR = WIN32_ERROR(1117u32);
pub const ERROR_SERIAL_NO_DEVICE: WIN32_ERROR = WIN32_ERROR(1118u32);
pub const ERROR_IRQ_BUSY: WIN32_ERROR = WIN32_ERROR(1119u32);
pub const ERROR_MORE_WRITES: WIN32_ERROR = WIN32_ERROR(1120u32);
pub const ERROR_COUNTER_TIMEOUT: WIN32_ERROR = WIN32_ERROR(1121u32);
pub const ERROR_FLOPPY_ID_MARK_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1122u32);
pub const ERROR_FLOPPY_WRONG_CYLINDER: WIN32_ERROR = WIN32_ERROR(1123u32);
pub const ERROR_FLOPPY_UNKNOWN_ERROR: WIN32_ERROR = WIN32_ERROR(1124u32);
pub const ERROR_FLOPPY_BAD_REGISTERS: WIN32_ERROR = WIN32_ERROR(1125u32);
pub const ERROR_DISK_RECALIBRATE_FAILED: WIN32_ERROR = WIN32_ERROR(1126u32);
pub const ERROR_DISK_OPERATION_FAILED: WIN32_ERROR = WIN32_ERROR(1127u32);
pub const ERROR_DISK_RESET_FAILED: WIN32_ERROR = WIN32_ERROR(1128u32);
pub const ERROR_EOM_OVERFLOW: WIN32_ERROR = WIN32_ERROR(1129u32);
pub const ERROR_NOT_ENOUGH_SERVER_MEMORY: WIN32_ERROR = WIN32_ERROR(1130u32);
pub const ERROR_POSSIBLE_DEADLOCK: WIN32_ERROR = WIN32_ERROR(1131u32);
pub const ERROR_MAPPED_ALIGNMENT: WIN32_ERROR = WIN32_ERROR(1132u32);
pub const ERROR_SET_POWER_STATE_VETOED: WIN32_ERROR = WIN32_ERROR(1140u32);
pub const ERROR_SET_POWER_STATE_FAILED: WIN32_ERROR = WIN32_ERROR(1141u32);
pub const ERROR_TOO_MANY_LINKS: WIN32_ERROR = WIN32_ERROR(1142u32);
pub const ERROR_OLD_WIN_VERSION: WIN32_ERROR = WIN32_ERROR(1150u32);
pub const ERROR_APP_WRONG_OS: WIN32_ERROR = WIN32_ERROR(1151u32);
pub const ERROR_SINGLE_INSTANCE_APP: WIN32_ERROR = WIN32_ERROR(1152u32);
pub const ERROR_RMODE_APP: WIN32_ERROR = WIN32_ERROR(1153u32);
pub const ERROR_INVALID_DLL: WIN32_ERROR = WIN32_ERROR(1154u32);
pub const ERROR_NO_ASSOCIATION: WIN32_ERROR = WIN32_ERROR(1155u32);
pub const ERROR_DDE_FAIL: WIN32_ERROR = WIN32_ERROR(1156u32);
pub const ERROR_DLL_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1157u32);
pub const ERROR_NO_MORE_USER_HANDLES: WIN32_ERROR = WIN32_ERROR(1158u32);
pub const ERROR_MESSAGE_SYNC_ONLY: WIN32_ERROR = WIN32_ERROR(1159u32);
pub const ERROR_SOURCE_ELEMENT_EMPTY: WIN32_ERROR = WIN32_ERROR(1160u32);
pub const ERROR_DESTINATION_ELEMENT_FULL: WIN32_ERROR = WIN32_ERROR(1161u32);
pub const ERROR_ILLEGAL_ELEMENT_ADDRESS: WIN32_ERROR = WIN32_ERROR(1162u32);
pub const ERROR_MAGAZINE_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(1163u32);
pub const ERROR_DEVICE_REINITIALIZATION_NEEDED: WIN32_ERROR = WIN32_ERROR(1164u32);
pub const ERROR_DEVICE_REQUIRES_CLEANING: WIN32_ERROR = WIN32_ERROR(1165u32);
pub const ERROR_DEVICE_DOOR_OPEN: WIN32_ERROR = WIN32_ERROR(1166u32);
pub const ERROR_DEVICE_NOT_CONNECTED: WIN32_ERROR = WIN32_ERROR(1167u32);
pub const ERROR_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1168u32);
pub const ERROR_NO_MATCH: WIN32_ERROR = WIN32_ERROR(1169u32);
pub const ERROR_SET_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1170u32);
pub const ERROR_POINT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1171u32);
pub const ERROR_NO_TRACKING_SERVICE: WIN32_ERROR = WIN32_ERROR(1172u32);
pub const ERROR_NO_VOLUME_ID: WIN32_ERROR = WIN32_ERROR(1173u32);
pub const ERROR_UNABLE_TO_REMOVE_REPLACED: WIN32_ERROR = WIN32_ERROR(1175u32);
pub const ERROR_UNABLE_TO_MOVE_REPLACEMENT: WIN32_ERROR = WIN32_ERROR(1176u32);
pub const ERROR_UNABLE_TO_MOVE_REPLACEMENT_2: WIN32_ERROR = WIN32_ERROR(1177u32);
pub const ERROR_JOURNAL_DELETE_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(1178u32);
pub const ERROR_JOURNAL_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(1179u32);
pub const ERROR_POTENTIAL_FILE_FOUND: WIN32_ERROR = WIN32_ERROR(1180u32);
pub const ERROR_JOURNAL_ENTRY_DELETED: WIN32_ERROR = WIN32_ERROR(1181u32);
pub const ERROR_PARTITION_TERMINATING: WIN32_ERROR = WIN32_ERROR(1184u32);
pub const ERROR_SHUTDOWN_IS_SCHEDULED: WIN32_ERROR = WIN32_ERROR(1190u32);
pub const ERROR_SHUTDOWN_USERS_LOGGED_ON: WIN32_ERROR = WIN32_ERROR(1191u32);
pub const ERROR_SHUTDOWN_DISKS_NOT_IN_MAINTENANCE_MODE: WIN32_ERROR = WIN32_ERROR(1192u32);
pub const ERROR_BAD_DEVICE: WIN32_ERROR = WIN32_ERROR(1200u32);
pub const ERROR_CONNECTION_UNAVAIL: WIN32_ERROR = WIN32_ERROR(1201u32);
pub const ERROR_DEVICE_ALREADY_REMEMBERED: WIN32_ERROR = WIN32_ERROR(1202u32);
pub const ERROR_NO_NET_OR_BAD_PATH: WIN32_ERROR = WIN32_ERROR(1203u32);
pub const ERROR_BAD_PROVIDER: WIN32_ERROR = WIN32_ERROR(1204u32);
pub const ERROR_CANNOT_OPEN_PROFILE: WIN32_ERROR = WIN32_ERROR(1205u32);
pub const ERROR_BAD_PROFILE: WIN32_ERROR = WIN32_ERROR(1206u32);
pub const ERROR_NOT_CONTAINER: WIN32_ERROR = WIN32_ERROR(1207u32);
pub const ERROR_EXTENDED_ERROR: WIN32_ERROR = WIN32_ERROR(1208u32);
pub const ERROR_INVALID_GROUPNAME: WIN32_ERROR = WIN32_ERROR(1209u32);
pub const ERROR_INVALID_COMPUTERNAME: WIN32_ERROR = WIN32_ERROR(1210u32);
pub const ERROR_INVALID_EVENTNAME: WIN32_ERROR = WIN32_ERROR(1211u32);
pub const ERROR_INVALID_DOMAINNAME: WIN32_ERROR = WIN32_ERROR(1212u32);
pub const ERROR_INVALID_SERVICENAME: WIN32_ERROR = WIN32_ERROR(1213u32);
pub const ERROR_INVALID_NETNAME: WIN32_ERROR = WIN32_ERROR(1214u32);
pub const ERROR_INVALID_SHARENAME: WIN32_ERROR = WIN32_ERROR(1215u32);
pub const ERROR_INVALID_PASSWORDNAME: WIN32_ERROR = WIN32_ERROR(1216u32);
pub const ERROR_INVALID_MESSAGENAME: WIN32_ERROR = WIN32_ERROR(1217u32);
pub const ERROR_INVALID_MESSAGEDEST: WIN32_ERROR = WIN32_ERROR(1218u32);
pub const ERROR_SESSION_CREDENTIAL_CONFLICT: WIN32_ERROR = WIN32_ERROR(1219u32);
pub const ERROR_REMOTE_SESSION_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1220u32);
pub const ERROR_DUP_DOMAINNAME: WIN32_ERROR = WIN32_ERROR(1221u32);
pub const ERROR_NO_NETWORK: WIN32_ERROR = WIN32_ERROR(1222u32);
pub const ERROR_CANCELLED: WIN32_ERROR = WIN32_ERROR(1223u32);
pub const ERROR_USER_MAPPED_FILE: WIN32_ERROR = WIN32_ERROR(1224u32);
pub const ERROR_CONNECTION_REFUSED: WIN32_ERROR = WIN32_ERROR(1225u32);
pub const ERROR_GRACEFUL_DISCONNECT: WIN32_ERROR = WIN32_ERROR(1226u32);
pub const ERROR_ADDRESS_ALREADY_ASSOCIATED: WIN32_ERROR = WIN32_ERROR(1227u32);
pub const ERROR_ADDRESS_NOT_ASSOCIATED: WIN32_ERROR = WIN32_ERROR(1228u32);
pub const ERROR_CONNECTION_INVALID: WIN32_ERROR = WIN32_ERROR(1229u32);
pub const ERROR_CONNECTION_ACTIVE: WIN32_ERROR = WIN32_ERROR(1230u32);
pub const ERROR_NETWORK_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(1231u32);
pub const ERROR_HOST_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(1232u32);
pub const ERROR_PROTOCOL_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(1233u32);
pub const ERROR_PORT_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(1234u32);
pub const ERROR_REQUEST_ABORTED: WIN32_ERROR = WIN32_ERROR(1235u32);
pub const ERROR_CONNECTION_ABORTED: WIN32_ERROR = WIN32_ERROR(1236u32);
pub const ERROR_RETRY: WIN32_ERROR = WIN32_ERROR(1237u32);
pub const ERROR_CONNECTION_COUNT_LIMIT: WIN32_ERROR = WIN32_ERROR(1238u32);
pub const ERROR_LOGIN_TIME_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1239u32);
pub const ERROR_LOGIN_WKSTA_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1240u32);
pub const ERROR_INCORRECT_ADDRESS: WIN32_ERROR = WIN32_ERROR(1241u32);
pub const ERROR_ALREADY_REGISTERED: WIN32_ERROR = WIN32_ERROR(1242u32);
pub const ERROR_SERVICE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1243u32);
pub const ERROR_NOT_AUTHENTICATED: WIN32_ERROR = WIN32_ERROR(1244u32);
pub const ERROR_NOT_LOGGED_ON: WIN32_ERROR = WIN32_ERROR(1245u32);
pub const ERROR_CONTINUE: WIN32_ERROR = WIN32_ERROR(1246u32);
pub const ERROR_ALREADY_INITIALIZED: WIN32_ERROR = WIN32_ERROR(1247u32);
pub const ERROR_NO_MORE_DEVICES: WIN32_ERROR = WIN32_ERROR(1248u32);
pub const ERROR_NO_SUCH_SITE: WIN32_ERROR = WIN32_ERROR(1249u32);
pub const ERROR_DOMAIN_CONTROLLER_EXISTS: WIN32_ERROR = WIN32_ERROR(1250u32);
pub const ERROR_ONLY_IF_CONNECTED: WIN32_ERROR = WIN32_ERROR(1251u32);
pub const ERROR_OVERRIDE_NOCHANGES: WIN32_ERROR = WIN32_ERROR(1252u32);
pub const ERROR_BAD_USER_PROFILE: WIN32_ERROR = WIN32_ERROR(1253u32);
pub const ERROR_NOT_SUPPORTED_ON_SBS: WIN32_ERROR = WIN32_ERROR(1254u32);
pub const ERROR_SERVER_SHUTDOWN_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(1255u32);
pub const ERROR_HOST_DOWN: WIN32_ERROR = WIN32_ERROR(1256u32);
pub const ERROR_NON_ACCOUNT_SID: WIN32_ERROR = WIN32_ERROR(1257u32);
pub const ERROR_NON_DOMAIN_SID: WIN32_ERROR = WIN32_ERROR(1258u32);
pub const ERROR_APPHELP_BLOCK: WIN32_ERROR = WIN32_ERROR(1259u32);
pub const ERROR_ACCESS_DISABLED_BY_POLICY: WIN32_ERROR = WIN32_ERROR(1260u32);
pub const ERROR_REG_NAT_CONSUMPTION: WIN32_ERROR = WIN32_ERROR(1261u32);
pub const ERROR_CSCSHARE_OFFLINE: WIN32_ERROR = WIN32_ERROR(1262u32);
pub const ERROR_PKINIT_FAILURE: WIN32_ERROR = WIN32_ERROR(1263u32);
pub const ERROR_SMARTCARD_SUBSYSTEM_FAILURE: WIN32_ERROR = WIN32_ERROR(1264u32);
pub const ERROR_DOWNGRADE_DETECTED: WIN32_ERROR = WIN32_ERROR(1265u32);
pub const ERROR_MACHINE_LOCKED: WIN32_ERROR = WIN32_ERROR(1271u32);
pub const ERROR_SMB_GUEST_LOGON_BLOCKED: WIN32_ERROR = WIN32_ERROR(1272u32);
pub const ERROR_CALLBACK_SUPPLIED_INVALID_DATA: WIN32_ERROR = WIN32_ERROR(1273u32);
pub const ERROR_SYNC_FOREGROUND_REFRESH_REQUIRED: WIN32_ERROR = WIN32_ERROR(1274u32);
pub const ERROR_DRIVER_BLOCKED: WIN32_ERROR = WIN32_ERROR(1275u32);
pub const ERROR_INVALID_IMPORT_OF_NON_DLL: WIN32_ERROR = WIN32_ERROR(1276u32);
pub const ERROR_ACCESS_DISABLED_WEBBLADE: WIN32_ERROR = WIN32_ERROR(1277u32);
pub const ERROR_ACCESS_DISABLED_WEBBLADE_TAMPER: WIN32_ERROR = WIN32_ERROR(1278u32);
pub const ERROR_RECOVERY_FAILURE: WIN32_ERROR = WIN32_ERROR(1279u32);
pub const ERROR_ALREADY_FIBER: WIN32_ERROR = WIN32_ERROR(1280u32);
pub const ERROR_ALREADY_THREAD: WIN32_ERROR = WIN32_ERROR(1281u32);
pub const ERROR_STACK_BUFFER_OVERRUN: WIN32_ERROR = WIN32_ERROR(1282u32);
pub const ERROR_PARAMETER_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1283u32);
pub const ERROR_DEBUGGER_INACTIVE: WIN32_ERROR = WIN32_ERROR(1284u32);
pub const ERROR_DELAY_LOAD_FAILED: WIN32_ERROR = WIN32_ERROR(1285u32);
pub const ERROR_VDM_DISALLOWED: WIN32_ERROR = WIN32_ERROR(1286u32);
pub const ERROR_UNIDENTIFIED_ERROR: WIN32_ERROR = WIN32_ERROR(1287u32);
pub const ERROR_INVALID_CRUNTIME_PARAMETER: WIN32_ERROR = WIN32_ERROR(1288u32);
pub const ERROR_BEYOND_VDL: WIN32_ERROR = WIN32_ERROR(1289u32);
pub const ERROR_INCOMPATIBLE_SERVICE_SID_TYPE: WIN32_ERROR = WIN32_ERROR(1290u32);
pub const ERROR_DRIVER_PROCESS_TERMINATED: WIN32_ERROR = WIN32_ERROR(1291u32);
pub const ERROR_IMPLEMENTATION_LIMIT: WIN32_ERROR = WIN32_ERROR(1292u32);
pub const ERROR_PROCESS_IS_PROTECTED: WIN32_ERROR = WIN32_ERROR(1293u32);
pub const ERROR_SERVICE_NOTIFY_CLIENT_LAGGING: WIN32_ERROR = WIN32_ERROR(1294u32);
pub const ERROR_DISK_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1295u32);
pub const ERROR_CONTENT_BLOCKED: WIN32_ERROR = WIN32_ERROR(1296u32);
pub const ERROR_INCOMPATIBLE_SERVICE_PRIVILEGE: WIN32_ERROR = WIN32_ERROR(1297u32);
pub const ERROR_APP_HANG: WIN32_ERROR = WIN32_ERROR(1298u32);
pub const ERROR_INVALID_LABEL: WIN32_ERROR = WIN32_ERROR(1299u32);
pub const ERROR_NOT_ALL_ASSIGNED: WIN32_ERROR = WIN32_ERROR(1300u32);
pub const ERROR_SOME_NOT_MAPPED: WIN32_ERROR = WIN32_ERROR(1301u32);
pub const ERROR_NO_QUOTAS_FOR_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1302u32);
pub const ERROR_LOCAL_USER_SESSION_KEY: WIN32_ERROR = WIN32_ERROR(1303u32);
pub const ERROR_NULL_LM_PASSWORD: WIN32_ERROR = WIN32_ERROR(1304u32);
pub const ERROR_UNKNOWN_REVISION: WIN32_ERROR = WIN32_ERROR(1305u32);
pub const ERROR_REVISION_MISMATCH: WIN32_ERROR = WIN32_ERROR(1306u32);
pub const ERROR_INVALID_OWNER: WIN32_ERROR = WIN32_ERROR(1307u32);
pub const ERROR_INVALID_PRIMARY_GROUP: WIN32_ERROR = WIN32_ERROR(1308u32);
pub const ERROR_NO_IMPERSONATION_TOKEN: WIN32_ERROR = WIN32_ERROR(1309u32);
pub const ERROR_CANT_DISABLE_MANDATORY: WIN32_ERROR = WIN32_ERROR(1310u32);
pub const ERROR_NO_LOGON_SERVERS: WIN32_ERROR = WIN32_ERROR(1311u32);
pub const ERROR_NO_SUCH_LOGON_SESSION: WIN32_ERROR = WIN32_ERROR(1312u32);
pub const ERROR_NO_SUCH_PRIVILEGE: WIN32_ERROR = WIN32_ERROR(1313u32);
pub const ERROR_PRIVILEGE_NOT_HELD: WIN32_ERROR = WIN32_ERROR(1314u32);
pub const ERROR_INVALID_ACCOUNT_NAME: WIN32_ERROR = WIN32_ERROR(1315u32);
pub const ERROR_USER_EXISTS: WIN32_ERROR = WIN32_ERROR(1316u32);
pub const ERROR_NO_SUCH_USER: WIN32_ERROR = WIN32_ERROR(1317u32);
pub const ERROR_GROUP_EXISTS: WIN32_ERROR = WIN32_ERROR(1318u32);
pub const ERROR_NO_SUCH_GROUP: WIN32_ERROR = WIN32_ERROR(1319u32);
pub const ERROR_MEMBER_IN_GROUP: WIN32_ERROR = WIN32_ERROR(1320u32);
pub const ERROR_MEMBER_NOT_IN_GROUP: WIN32_ERROR = WIN32_ERROR(1321u32);
pub const ERROR_LAST_ADMIN: WIN32_ERROR = WIN32_ERROR(1322u32);
pub const ERROR_WRONG_PASSWORD: WIN32_ERROR = WIN32_ERROR(1323u32);
pub const ERROR_ILL_FORMED_PASSWORD: WIN32_ERROR = WIN32_ERROR(1324u32);
pub const ERROR_PASSWORD_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1325u32);
pub const ERROR_LOGON_FAILURE: WIN32_ERROR = WIN32_ERROR(1326u32);
pub const ERROR_ACCOUNT_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1327u32);
pub const ERROR_INVALID_LOGON_HOURS: WIN32_ERROR = WIN32_ERROR(1328u32);
pub const ERROR_INVALID_WORKSTATION: WIN32_ERROR = WIN32_ERROR(1329u32);
pub const ERROR_PASSWORD_EXPIRED: WIN32_ERROR = WIN32_ERROR(1330u32);
pub const ERROR_ACCOUNT_DISABLED: WIN32_ERROR = WIN32_ERROR(1331u32);
pub const ERROR_NONE_MAPPED: WIN32_ERROR = WIN32_ERROR(1332u32);
pub const ERROR_TOO_MANY_LUIDS_REQUESTED: WIN32_ERROR = WIN32_ERROR(1333u32);
pub const ERROR_LUIDS_EXHAUSTED: WIN32_ERROR = WIN32_ERROR(1334u32);
pub const ERROR_INVALID_SUB_AUTHORITY: WIN32_ERROR = WIN32_ERROR(1335u32);
pub const ERROR_INVALID_ACL: WIN32_ERROR = WIN32_ERROR(1336u32);
pub const ERROR_INVALID_SID: WIN32_ERROR = WIN32_ERROR(1337u32);
pub const ERROR_INVALID_SECURITY_DESCR: WIN32_ERROR = WIN32_ERROR(1338u32);
pub const ERROR_BAD_INHERITANCE_ACL: WIN32_ERROR = WIN32_ERROR(1340u32);
pub const ERROR_SERVER_DISABLED: WIN32_ERROR = WIN32_ERROR(1341u32);
pub const ERROR_SERVER_NOT_DISABLED: WIN32_ERROR = WIN32_ERROR(1342u32);
pub const ERROR_INVALID_ID_AUTHORITY: WIN32_ERROR = WIN32_ERROR(1343u32);
pub const ERROR_ALLOTTED_SPACE_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1344u32);
pub const ERROR_INVALID_GROUP_ATTRIBUTES: WIN32_ERROR = WIN32_ERROR(1345u32);
pub const ERROR_BAD_IMPERSONATION_LEVEL: WIN32_ERROR = WIN32_ERROR(1346u32);
pub const ERROR_CANT_OPEN_ANONYMOUS: WIN32_ERROR = WIN32_ERROR(1347u32);
pub const ERROR_BAD_VALIDATION_CLASS: WIN32_ERROR = WIN32_ERROR(1348u32);
pub const ERROR_BAD_TOKEN_TYPE: WIN32_ERROR = WIN32_ERROR(1349u32);
pub const ERROR_NO_SECURITY_ON_OBJECT: WIN32_ERROR = WIN32_ERROR(1350u32);
pub const ERROR_CANT_ACCESS_DOMAIN_INFO: WIN32_ERROR = WIN32_ERROR(1351u32);
pub const ERROR_INVALID_SERVER_STATE: WIN32_ERROR = WIN32_ERROR(1352u32);
pub const ERROR_INVALID_DOMAIN_STATE: WIN32_ERROR = WIN32_ERROR(1353u32);
pub const ERROR_INVALID_DOMAIN_ROLE: WIN32_ERROR = WIN32_ERROR(1354u32);
pub const ERROR_NO_SUCH_DOMAIN: WIN32_ERROR = WIN32_ERROR(1355u32);
pub const ERROR_DOMAIN_EXISTS: WIN32_ERROR = WIN32_ERROR(1356u32);
pub const ERROR_DOMAIN_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1357u32);
pub const ERROR_INTERNAL_DB_CORRUPTION: WIN32_ERROR = WIN32_ERROR(1358u32);
pub const ERROR_INTERNAL_ERROR: WIN32_ERROR = WIN32_ERROR(1359u32);
pub const ERROR_GENERIC_NOT_MAPPED: WIN32_ERROR = WIN32_ERROR(1360u32);
pub const ERROR_BAD_DESCRIPTOR_FORMAT: WIN32_ERROR = WIN32_ERROR(1361u32);
pub const ERROR_NOT_LOGON_PROCESS: WIN32_ERROR = WIN32_ERROR(1362u32);
pub const ERROR_LOGON_SESSION_EXISTS: WIN32_ERROR = WIN32_ERROR(1363u32);
pub const ERROR_NO_SUCH_PACKAGE: WIN32_ERROR = WIN32_ERROR(1364u32);
pub const ERROR_BAD_LOGON_SESSION_STATE: WIN32_ERROR = WIN32_ERROR(1365u32);
pub const ERROR_LOGON_SESSION_COLLISION: WIN32_ERROR = WIN32_ERROR(1366u32);
pub const ERROR_INVALID_LOGON_TYPE: WIN32_ERROR = WIN32_ERROR(1367u32);
pub const ERROR_CANNOT_IMPERSONATE: WIN32_ERROR = WIN32_ERROR(1368u32);
pub const ERROR_RXACT_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(1369u32);
pub const ERROR_RXACT_COMMIT_FAILURE: WIN32_ERROR = WIN32_ERROR(1370u32);
pub const ERROR_SPECIAL_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1371u32);
pub const ERROR_SPECIAL_GROUP: WIN32_ERROR = WIN32_ERROR(1372u32);
pub const ERROR_SPECIAL_USER: WIN32_ERROR = WIN32_ERROR(1373u32);
pub const ERROR_MEMBERS_PRIMARY_GROUP: WIN32_ERROR = WIN32_ERROR(1374u32);
pub const ERROR_TOKEN_ALREADY_IN_USE: WIN32_ERROR = WIN32_ERROR(1375u32);
pub const ERROR_NO_SUCH_ALIAS: WIN32_ERROR = WIN32_ERROR(1376u32);
pub const ERROR_MEMBER_NOT_IN_ALIAS: WIN32_ERROR = WIN32_ERROR(1377u32);
pub const ERROR_MEMBER_IN_ALIAS: WIN32_ERROR = WIN32_ERROR(1378u32);
pub const ERROR_ALIAS_EXISTS: WIN32_ERROR = WIN32_ERROR(1379u32);
pub const ERROR_LOGON_NOT_GRANTED: WIN32_ERROR = WIN32_ERROR(1380u32);
pub const ERROR_TOO_MANY_SECRETS: WIN32_ERROR = WIN32_ERROR(1381u32);
pub const ERROR_SECRET_TOO_LONG: WIN32_ERROR = WIN32_ERROR(1382u32);
pub const ERROR_INTERNAL_DB_ERROR: WIN32_ERROR = WIN32_ERROR(1383u32);
pub const ERROR_TOO_MANY_CONTEXT_IDS: WIN32_ERROR = WIN32_ERROR(1384u32);
pub const ERROR_LOGON_TYPE_NOT_GRANTED: WIN32_ERROR = WIN32_ERROR(1385u32);
pub const ERROR_NT_CROSS_ENCRYPTION_REQUIRED: WIN32_ERROR = WIN32_ERROR(1386u32);
pub const ERROR_NO_SUCH_MEMBER: WIN32_ERROR = WIN32_ERROR(1387u32);
pub const ERROR_INVALID_MEMBER: WIN32_ERROR = WIN32_ERROR(1388u32);
pub const ERROR_TOO_MANY_SIDS: WIN32_ERROR = WIN32_ERROR(1389u32);
pub const ERROR_LM_CROSS_ENCRYPTION_REQUIRED: WIN32_ERROR = WIN32_ERROR(1390u32);
pub const ERROR_NO_INHERITANCE: WIN32_ERROR = WIN32_ERROR(1391u32);
pub const ERROR_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(1392u32);
pub const ERROR_DISK_CORRUPT: WIN32_ERROR = WIN32_ERROR(1393u32);
pub const ERROR_NO_USER_SESSION_KEY: WIN32_ERROR = WIN32_ERROR(1394u32);
pub const ERROR_LICENSE_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1395u32);
pub const ERROR_WRONG_TARGET_NAME: WIN32_ERROR = WIN32_ERROR(1396u32);
pub const ERROR_MUTUAL_AUTH_FAILED: WIN32_ERROR = WIN32_ERROR(1397u32);
pub const ERROR_TIME_SKEW: WIN32_ERROR = WIN32_ERROR(1398u32);
pub const ERROR_CURRENT_DOMAIN_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(1399u32);
pub const ERROR_INVALID_WINDOW_HANDLE: WIN32_ERROR = WIN32_ERROR(1400u32);
pub const ERROR_INVALID_MENU_HANDLE: WIN32_ERROR = WIN32_ERROR(1401u32);
pub const ERROR_INVALID_CURSOR_HANDLE: WIN32_ERROR = WIN32_ERROR(1402u32);
pub const ERROR_INVALID_ACCEL_HANDLE: WIN32_ERROR = WIN32_ERROR(1403u32);
pub const ERROR_INVALID_HOOK_HANDLE: WIN32_ERROR = WIN32_ERROR(1404u32);
pub const ERROR_INVALID_DWP_HANDLE: WIN32_ERROR = WIN32_ERROR(1405u32);
pub const ERROR_TLW_WITH_WSCHILD: WIN32_ERROR = WIN32_ERROR(1406u32);
pub const ERROR_CANNOT_FIND_WND_CLASS: WIN32_ERROR = WIN32_ERROR(1407u32);
pub const ERROR_WINDOW_OF_OTHER_THREAD: WIN32_ERROR = WIN32_ERROR(1408u32);
pub const ERROR_HOTKEY_ALREADY_REGISTERED: WIN32_ERROR = WIN32_ERROR(1409u32);
pub const ERROR_CLASS_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(1410u32);
pub const ERROR_CLASS_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(1411u32);
pub const ERROR_CLASS_HAS_WINDOWS: WIN32_ERROR = WIN32_ERROR(1412u32);
pub const ERROR_INVALID_INDEX: WIN32_ERROR = WIN32_ERROR(1413u32);
pub const ERROR_INVALID_ICON_HANDLE: WIN32_ERROR = WIN32_ERROR(1414u32);
pub const ERROR_PRIVATE_DIALOG_INDEX: WIN32_ERROR = WIN32_ERROR(1415u32);
pub const ERROR_LISTBOX_ID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1416u32);
pub const ERROR_NO_WILDCARD_CHARACTERS: WIN32_ERROR = WIN32_ERROR(1417u32);
pub const ERROR_CLIPBOARD_NOT_OPEN: WIN32_ERROR = WIN32_ERROR(1418u32);
pub const ERROR_HOTKEY_NOT_REGISTERED: WIN32_ERROR = WIN32_ERROR(1419u32);
pub const ERROR_WINDOW_NOT_DIALOG: WIN32_ERROR = WIN32_ERROR(1420u32);
pub const ERROR_CONTROL_ID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1421u32);
pub const ERROR_INVALID_COMBOBOX_MESSAGE: WIN32_ERROR = WIN32_ERROR(1422u32);
pub const ERROR_WINDOW_NOT_COMBOBOX: WIN32_ERROR = WIN32_ERROR(1423u32);
pub const ERROR_INVALID_EDIT_HEIGHT: WIN32_ERROR = WIN32_ERROR(1424u32);
pub const ERROR_DC_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1425u32);
pub const ERROR_INVALID_HOOK_FILTER: WIN32_ERROR = WIN32_ERROR(1426u32);
pub const ERROR_INVALID_FILTER_PROC: WIN32_ERROR = WIN32_ERROR(1427u32);
pub const ERROR_HOOK_NEEDS_HMOD: WIN32_ERROR = WIN32_ERROR(1428u32);
pub const ERROR_GLOBAL_ONLY_HOOK: WIN32_ERROR = WIN32_ERROR(1429u32);
pub const ERROR_JOURNAL_HOOK_SET: WIN32_ERROR = WIN32_ERROR(1430u32);
pub const ERROR_HOOK_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(1431u32);
pub const ERROR_INVALID_LB_MESSAGE: WIN32_ERROR = WIN32_ERROR(1432u32);
pub const ERROR_SETCOUNT_ON_BAD_LB: WIN32_ERROR = WIN32_ERROR(1433u32);
pub const ERROR_LB_WITHOUT_TABSTOPS: WIN32_ERROR = WIN32_ERROR(1434u32);
pub const ERROR_DESTROY_OBJECT_OF_OTHER_THREAD: WIN32_ERROR = WIN32_ERROR(1435u32);
pub const ERROR_CHILD_WINDOW_MENU: WIN32_ERROR = WIN32_ERROR(1436u32);
pub const ERROR_NO_SYSTEM_MENU: WIN32_ERROR = WIN32_ERROR(1437u32);
pub const ERROR_INVALID_MSGBOX_STYLE: WIN32_ERROR = WIN32_ERROR(1438u32);
pub const ERROR_INVALID_SPI_VALUE: WIN32_ERROR = WIN32_ERROR(1439u32);
pub const ERROR_SCREEN_ALREADY_LOCKED: WIN32_ERROR = WIN32_ERROR(1440u32);
pub const ERROR_HWNDS_HAVE_DIFF_PARENT: WIN32_ERROR = WIN32_ERROR(1441u32);
pub const ERROR_NOT_CHILD_WINDOW: WIN32_ERROR = WIN32_ERROR(1442u32);
pub const ERROR_INVALID_GW_COMMAND: WIN32_ERROR = WIN32_ERROR(1443u32);
pub const ERROR_INVALID_THREAD_ID: WIN32_ERROR = WIN32_ERROR(1444u32);
pub const ERROR_NON_MDICHILD_WINDOW: WIN32_ERROR = WIN32_ERROR(1445u32);
pub const ERROR_POPUP_ALREADY_ACTIVE: WIN32_ERROR = WIN32_ERROR(1446u32);
pub const ERROR_NO_SCROLLBARS: WIN32_ERROR = WIN32_ERROR(1447u32);
pub const ERROR_INVALID_SCROLLBAR_RANGE: WIN32_ERROR = WIN32_ERROR(1448u32);
pub const ERROR_INVALID_SHOWWIN_COMMAND: WIN32_ERROR = WIN32_ERROR(1449u32);
pub const ERROR_NO_SYSTEM_RESOURCES: WIN32_ERROR = WIN32_ERROR(1450u32);
pub const ERROR_NONPAGED_SYSTEM_RESOURCES: WIN32_ERROR = WIN32_ERROR(1451u32);
pub const ERROR_PAGED_SYSTEM_RESOURCES: WIN32_ERROR = WIN32_ERROR(1452u32);
pub const ERROR_WORKING_SET_QUOTA: WIN32_ERROR = WIN32_ERROR(1453u32);
pub const ERROR_PAGEFILE_QUOTA: WIN32_ERROR = WIN32_ERROR(1454u32);
pub const ERROR_COMMITMENT_LIMIT: WIN32_ERROR = WIN32_ERROR(1455u32);
pub const ERROR_MENU_ITEM_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1456u32);
pub const ERROR_INVALID_KEYBOARD_HANDLE: WIN32_ERROR = WIN32_ERROR(1457u32);
pub const ERROR_HOOK_TYPE_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(1458u32);
pub const ERROR_REQUIRES_INTERACTIVE_WINDOWSTATION: WIN32_ERROR = WIN32_ERROR(1459u32);
pub const ERROR_TIMEOUT: WIN32_ERROR = WIN32_ERROR(1460u32);
pub const ERROR_INVALID_MONITOR_HANDLE: WIN32_ERROR = WIN32_ERROR(1461u32);
pub const ERROR_INCORRECT_SIZE: WIN32_ERROR = WIN32_ERROR(1462u32);
pub const ERROR_SYMLINK_CLASS_DISABLED: WIN32_ERROR = WIN32_ERROR(1463u32);
pub const ERROR_SYMLINK_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(1464u32);
pub const ERROR_XML_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(1465u32);
pub const ERROR_XMLDSIG_ERROR: WIN32_ERROR = WIN32_ERROR(1466u32);
pub const ERROR_RESTART_APPLICATION: WIN32_ERROR = WIN32_ERROR(1467u32);
pub const ERROR_WRONG_COMPARTMENT: WIN32_ERROR = WIN32_ERROR(1468u32);
pub const ERROR_AUTHIP_FAILURE: WIN32_ERROR = WIN32_ERROR(1469u32);
pub const ERROR_NO_NVRAM_RESOURCES: WIN32_ERROR = WIN32_ERROR(1470u32);
pub const ERROR_NOT_GUI_PROCESS: WIN32_ERROR = WIN32_ERROR(1471u32);
pub const ERROR_EVENTLOG_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(1500u32);
pub const ERROR_EVENTLOG_CANT_START: WIN32_ERROR = WIN32_ERROR(1501u32);
pub const ERROR_LOG_FILE_FULL: WIN32_ERROR = WIN32_ERROR(1502u32);
pub const ERROR_EVENTLOG_FILE_CHANGED: WIN32_ERROR = WIN32_ERROR(1503u32);
pub const ERROR_CONTAINER_ASSIGNED: WIN32_ERROR = WIN32_ERROR(1504u32);
pub const ERROR_JOB_NO_CONTAINER: WIN32_ERROR = WIN32_ERROR(1505u32);
pub const ERROR_INVALID_TASK_NAME: WIN32_ERROR = WIN32_ERROR(1550u32);
pub const ERROR_INVALID_TASK_INDEX: WIN32_ERROR = WIN32_ERROR(1551u32);
pub const ERROR_THREAD_ALREADY_IN_TASK: WIN32_ERROR = WIN32_ERROR(1552u32);
pub const ERROR_INSTALL_SERVICE_FAILURE: WIN32_ERROR = WIN32_ERROR(1601u32);
pub const ERROR_INSTALL_USEREXIT: WIN32_ERROR = WIN32_ERROR(1602u32);
pub const ERROR_INSTALL_FAILURE: WIN32_ERROR = WIN32_ERROR(1603u32);
pub const ERROR_INSTALL_SUSPEND: WIN32_ERROR = WIN32_ERROR(1604u32);
pub const ERROR_UNKNOWN_PRODUCT: WIN32_ERROR = WIN32_ERROR(1605u32);
pub const ERROR_UNKNOWN_FEATURE: WIN32_ERROR = WIN32_ERROR(1606u32);
pub const ERROR_UNKNOWN_COMPONENT: WIN32_ERROR = WIN32_ERROR(1607u32);
pub const ERROR_UNKNOWN_PROPERTY: WIN32_ERROR = WIN32_ERROR(1608u32);
pub const ERROR_INVALID_HANDLE_STATE: WIN32_ERROR = WIN32_ERROR(1609u32);
pub const ERROR_BAD_CONFIGURATION: WIN32_ERROR = WIN32_ERROR(1610u32);
pub const ERROR_INDEX_ABSENT: WIN32_ERROR = WIN32_ERROR(1611u32);
pub const ERROR_INSTALL_SOURCE_ABSENT: WIN32_ERROR = WIN32_ERROR(1612u32);
pub const ERROR_INSTALL_PACKAGE_VERSION: WIN32_ERROR = WIN32_ERROR(1613u32);
pub const ERROR_PRODUCT_UNINSTALLED: WIN32_ERROR = WIN32_ERROR(1614u32);
pub const ERROR_BAD_QUERY_SYNTAX: WIN32_ERROR = WIN32_ERROR(1615u32);
pub const ERROR_INVALID_FIELD: WIN32_ERROR = WIN32_ERROR(1616u32);
pub const ERROR_DEVICE_REMOVED: WIN32_ERROR = WIN32_ERROR(1617u32);
pub const ERROR_INSTALL_ALREADY_RUNNING: WIN32_ERROR = WIN32_ERROR(1618u32);
pub const ERROR_INSTALL_PACKAGE_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(1619u32);
pub const ERROR_INSTALL_PACKAGE_INVALID: WIN32_ERROR = WIN32_ERROR(1620u32);
pub const ERROR_INSTALL_UI_FAILURE: WIN32_ERROR = WIN32_ERROR(1621u32);
pub const ERROR_INSTALL_LOG_FAILURE: WIN32_ERROR = WIN32_ERROR(1622u32);
pub const ERROR_INSTALL_LANGUAGE_UNSUPPORTED: WIN32_ERROR = WIN32_ERROR(1623u32);
pub const ERROR_INSTALL_TRANSFORM_FAILURE: WIN32_ERROR = WIN32_ERROR(1624u32);
pub const ERROR_INSTALL_PACKAGE_REJECTED: WIN32_ERROR = WIN32_ERROR(1625u32);
pub const ERROR_FUNCTION_NOT_CALLED: WIN32_ERROR = WIN32_ERROR(1626u32);
pub const ERROR_FUNCTION_FAILED: WIN32_ERROR = WIN32_ERROR(1627u32);
pub const ERROR_INVALID_TABLE: WIN32_ERROR = WIN32_ERROR(1628u32);
pub const ERROR_DATATYPE_MISMATCH: WIN32_ERROR = WIN32_ERROR(1629u32);
pub const ERROR_UNSUPPORTED_TYPE: WIN32_ERROR = WIN32_ERROR(1630u32);
pub const ERROR_CREATE_FAILED: WIN32_ERROR = WIN32_ERROR(1631u32);
pub const ERROR_INSTALL_TEMP_UNWRITABLE: WIN32_ERROR = WIN32_ERROR(1632u32);
pub const ERROR_INSTALL_PLATFORM_UNSUPPORTED: WIN32_ERROR = WIN32_ERROR(1633u32);
pub const ERROR_INSTALL_NOTUSED: WIN32_ERROR = WIN32_ERROR(1634u32);
pub const ERROR_PATCH_PACKAGE_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(1635u32);
pub const ERROR_PATCH_PACKAGE_INVALID: WIN32_ERROR = WIN32_ERROR(1636u32);
pub const ERROR_PATCH_PACKAGE_UNSUPPORTED: WIN32_ERROR = WIN32_ERROR(1637u32);
pub const ERROR_PRODUCT_VERSION: WIN32_ERROR = WIN32_ERROR(1638u32);
pub const ERROR_INVALID_COMMAND_LINE: WIN32_ERROR = WIN32_ERROR(1639u32);
pub const ERROR_INSTALL_REMOTE_DISALLOWED: WIN32_ERROR = WIN32_ERROR(1640u32);
pub const ERROR_SUCCESS_REBOOT_INITIATED: WIN32_ERROR = WIN32_ERROR(1641u32);
pub const ERROR_PATCH_TARGET_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1642u32);
pub const ERROR_PATCH_PACKAGE_REJECTED: WIN32_ERROR = WIN32_ERROR(1643u32);
pub const ERROR_INSTALL_TRANSFORM_REJECTED: WIN32_ERROR = WIN32_ERROR(1644u32);
pub const ERROR_INSTALL_REMOTE_PROHIBITED: WIN32_ERROR = WIN32_ERROR(1645u32);
pub const ERROR_PATCH_REMOVAL_UNSUPPORTED: WIN32_ERROR = WIN32_ERROR(1646u32);
pub const ERROR_UNKNOWN_PATCH: WIN32_ERROR = WIN32_ERROR(1647u32);
pub const ERROR_PATCH_NO_SEQUENCE: WIN32_ERROR = WIN32_ERROR(1648u32);
pub const ERROR_PATCH_REMOVAL_DISALLOWED: WIN32_ERROR = WIN32_ERROR(1649u32);
pub const ERROR_INVALID_PATCH_XML: WIN32_ERROR = WIN32_ERROR(1650u32);
pub const ERROR_PATCH_MANAGED_ADVERTISED_PRODUCT: WIN32_ERROR = WIN32_ERROR(1651u32);
pub const ERROR_INSTALL_SERVICE_SAFEBOOT: WIN32_ERROR = WIN32_ERROR(1652u32);
pub const ERROR_FAIL_FAST_EXCEPTION: WIN32_ERROR = WIN32_ERROR(1653u32);
pub const ERROR_INSTALL_REJECTED: WIN32_ERROR = WIN32_ERROR(1654u32);
pub const ERROR_DYNAMIC_CODE_BLOCKED: WIN32_ERROR = WIN32_ERROR(1655u32);
pub const ERROR_NOT_SAME_OBJECT: WIN32_ERROR = WIN32_ERROR(1656u32);
pub const ERROR_STRICT_CFG_VIOLATION: WIN32_ERROR = WIN32_ERROR(1657u32);
pub const ERROR_SET_CONTEXT_DENIED: WIN32_ERROR = WIN32_ERROR(1660u32);
pub const ERROR_CROSS_PARTITION_VIOLATION: WIN32_ERROR = WIN32_ERROR(1661u32);
pub const ERROR_RETURN_ADDRESS_HIJACK_ATTEMPT: WIN32_ERROR = WIN32_ERROR(1662u32);
pub const ERROR_INVALID_USER_BUFFER: WIN32_ERROR = WIN32_ERROR(1784u32);
pub const ERROR_UNRECOGNIZED_MEDIA: WIN32_ERROR = WIN32_ERROR(1785u32);
pub const ERROR_NO_TRUST_LSA_SECRET: WIN32_ERROR = WIN32_ERROR(1786u32);
pub const ERROR_NO_TRUST_SAM_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1787u32);
pub const ERROR_TRUSTED_DOMAIN_FAILURE: WIN32_ERROR = WIN32_ERROR(1788u32);
pub const ERROR_TRUSTED_RELATIONSHIP_FAILURE: WIN32_ERROR = WIN32_ERROR(1789u32);
pub const ERROR_TRUST_FAILURE: WIN32_ERROR = WIN32_ERROR(1790u32);
pub const ERROR_NETLOGON_NOT_STARTED: WIN32_ERROR = WIN32_ERROR(1792u32);
pub const ERROR_ACCOUNT_EXPIRED: WIN32_ERROR = WIN32_ERROR(1793u32);
pub const ERROR_REDIRECTOR_HAS_OPEN_HANDLES: WIN32_ERROR = WIN32_ERROR(1794u32);
pub const ERROR_PRINTER_DRIVER_ALREADY_INSTALLED: WIN32_ERROR = WIN32_ERROR(1795u32);
pub const ERROR_UNKNOWN_PORT: WIN32_ERROR = WIN32_ERROR(1796u32);
pub const ERROR_UNKNOWN_PRINTER_DRIVER: WIN32_ERROR = WIN32_ERROR(1797u32);
pub const ERROR_UNKNOWN_PRINTPROCESSOR: WIN32_ERROR = WIN32_ERROR(1798u32);
pub const ERROR_INVALID_SEPARATOR_FILE: WIN32_ERROR = WIN32_ERROR(1799u32);
pub const ERROR_INVALID_PRIORITY: WIN32_ERROR = WIN32_ERROR(1800u32);
pub const ERROR_INVALID_PRINTER_NAME: WIN32_ERROR = WIN32_ERROR(1801u32);
pub const ERROR_PRINTER_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(1802u32);
pub const ERROR_INVALID_PRINTER_COMMAND: WIN32_ERROR = WIN32_ERROR(1803u32);
pub const ERROR_INVALID_DATATYPE: WIN32_ERROR = WIN32_ERROR(1804u32);
pub const ERROR_INVALID_ENVIRONMENT: WIN32_ERROR = WIN32_ERROR(1805u32);
pub const ERROR_NOLOGON_INTERDOMAIN_TRUST_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1807u32);
pub const ERROR_NOLOGON_WORKSTATION_TRUST_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1808u32);
pub const ERROR_NOLOGON_SERVER_TRUST_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1809u32);
pub const ERROR_DOMAIN_TRUST_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(1810u32);
pub const ERROR_SERVER_HAS_OPEN_HANDLES: WIN32_ERROR = WIN32_ERROR(1811u32);
pub const ERROR_RESOURCE_DATA_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1812u32);
pub const ERROR_RESOURCE_TYPE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1813u32);
pub const ERROR_RESOURCE_NAME_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1814u32);
pub const ERROR_RESOURCE_LANG_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1815u32);
pub const ERROR_NOT_ENOUGH_QUOTA: WIN32_ERROR = WIN32_ERROR(1816u32);
pub const ERROR_INVALID_TIME: WIN32_ERROR = WIN32_ERROR(1901u32);
pub const ERROR_INVALID_FORM_NAME: WIN32_ERROR = WIN32_ERROR(1902u32);
pub const ERROR_INVALID_FORM_SIZE: WIN32_ERROR = WIN32_ERROR(1903u32);
pub const ERROR_ALREADY_WAITING: WIN32_ERROR = WIN32_ERROR(1904u32);
pub const ERROR_PRINTER_DELETED: WIN32_ERROR = WIN32_ERROR(1905u32);
pub const ERROR_INVALID_PRINTER_STATE: WIN32_ERROR = WIN32_ERROR(1906u32);
pub const ERROR_PASSWORD_MUST_CHANGE: WIN32_ERROR = WIN32_ERROR(1907u32);
pub const ERROR_DOMAIN_CONTROLLER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1908u32);
pub const ERROR_ACCOUNT_LOCKED_OUT: WIN32_ERROR = WIN32_ERROR(1909u32);
pub const ERROR_NO_SITENAME: WIN32_ERROR = WIN32_ERROR(1919u32);
pub const ERROR_CANT_ACCESS_FILE: WIN32_ERROR = WIN32_ERROR(1920u32);
pub const ERROR_CANT_RESOLVE_FILENAME: WIN32_ERROR = WIN32_ERROR(1921u32);
pub const ERROR_KM_DRIVER_BLOCKED: WIN32_ERROR = WIN32_ERROR(1930u32);
pub const ERROR_CONTEXT_EXPIRED: WIN32_ERROR = WIN32_ERROR(1931u32);
pub const ERROR_PER_USER_TRUST_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1932u32);
pub const ERROR_ALL_USER_TRUST_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1933u32);
pub const ERROR_USER_DELETE_TRUST_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1934u32);
pub const ERROR_AUTHENTICATION_FIREWALL_FAILED: WIN32_ERROR = WIN32_ERROR(1935u32);
pub const ERROR_REMOTE_PRINT_CONNECTIONS_BLOCKED: WIN32_ERROR = WIN32_ERROR(1936u32);
pub const ERROR_NTLM_BLOCKED: WIN32_ERROR = WIN32_ERROR(1937u32);
pub const ERROR_PASSWORD_CHANGE_REQUIRED: WIN32_ERROR = WIN32_ERROR(1938u32);
pub const ERROR_LOST_MODE_LOGON_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1939u32);
pub const ERROR_INVALID_PIXEL_FORMAT: WIN32_ERROR = WIN32_ERROR(2000u32);
pub const ERROR_BAD_DRIVER: WIN32_ERROR = WIN32_ERROR(2001u32);
pub const ERROR_INVALID_WINDOW_STYLE: WIN32_ERROR = WIN32_ERROR(2002u32);
pub const ERROR_METAFILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(2003u32);
pub const ERROR_TRANSFORM_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(2004u32);
pub const ERROR_CLIPPING_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(2005u32);
pub const ERROR_INVALID_CMM: WIN32_ERROR = WIN32_ERROR(2010u32);
pub const ERROR_INVALID_PROFILE: WIN32_ERROR = WIN32_ERROR(2011u32);
pub const ERROR_TAG_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2012u32);
pub const ERROR_TAG_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(2013u32);
pub const ERROR_DUPLICATE_TAG: WIN32_ERROR = WIN32_ERROR(2014u32);
pub const ERROR_PROFILE_NOT_ASSOCIATED_WITH_DEVICE: WIN32_ERROR = WIN32_ERROR(2015u32);
pub const ERROR_PROFILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2016u32);
pub const ERROR_INVALID_COLORSPACE: WIN32_ERROR = WIN32_ERROR(2017u32);
pub const ERROR_ICM_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(2018u32);
pub const ERROR_DELETING_ICM_XFORM: WIN32_ERROR = WIN32_ERROR(2019u32);
pub const ERROR_INVALID_TRANSFORM: WIN32_ERROR = WIN32_ERROR(2020u32);
pub const ERROR_COLORSPACE_MISMATCH: WIN32_ERROR = WIN32_ERROR(2021u32);
pub const ERROR_INVALID_COLORINDEX: WIN32_ERROR = WIN32_ERROR(2022u32);
pub const ERROR_PROFILE_DOES_NOT_MATCH_DEVICE: WIN32_ERROR = WIN32_ERROR(2023u32);
pub const ERROR_CONNECTED_OTHER_PASSWORD: WIN32_ERROR = WIN32_ERROR(2108u32);
pub const ERROR_CONNECTED_OTHER_PASSWORD_DEFAULT: WIN32_ERROR = WIN32_ERROR(2109u32);
pub const ERROR_BAD_USERNAME: WIN32_ERROR = WIN32_ERROR(2202u32);
pub const ERROR_NOT_CONNECTED: WIN32_ERROR = WIN32_ERROR(2250u32);
pub const ERROR_OPEN_FILES: WIN32_ERROR = WIN32_ERROR(2401u32);
pub const ERROR_ACTIVE_CONNECTIONS: WIN32_ERROR = WIN32_ERROR(2402u32);
pub const ERROR_DEVICE_IN_USE: WIN32_ERROR = WIN32_ERROR(2404u32);
pub const ERROR_UNKNOWN_PRINT_MONITOR: WIN32_ERROR = WIN32_ERROR(3000u32);
pub const ERROR_PRINTER_DRIVER_IN_USE: WIN32_ERROR = WIN32_ERROR(3001u32);
pub const ERROR_SPOOL_FILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3002u32);
pub const ERROR_SPL_NO_STARTDOC: WIN32_ERROR = WIN32_ERROR(3003u32);
pub const ERROR_SPL_NO_ADDJOB: WIN32_ERROR = WIN32_ERROR(3004u32);
pub const ERROR_PRINT_PROCESSOR_ALREADY_INSTALLED: WIN32_ERROR = WIN32_ERROR(3005u32);
pub const ERROR_PRINT_MONITOR_ALREADY_INSTALLED: WIN32_ERROR = WIN32_ERROR(3006u32);
pub const ERROR_INVALID_PRINT_MONITOR: WIN32_ERROR = WIN32_ERROR(3007u32);
pub const ERROR_PRINT_MONITOR_IN_USE: WIN32_ERROR = WIN32_ERROR(3008u32);
pub const ERROR_PRINTER_HAS_JOBS_QUEUED: WIN32_ERROR = WIN32_ERROR(3009u32);
pub const ERROR_SUCCESS_REBOOT_REQUIRED: WIN32_ERROR = WIN32_ERROR(3010u32);
pub const ERROR_SUCCESS_RESTART_REQUIRED: WIN32_ERROR = WIN32_ERROR(3011u32);
pub const ERROR_PRINTER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3012u32);
pub const ERROR_PRINTER_DRIVER_WARNED: WIN32_ERROR = WIN32_ERROR(3013u32);
pub const ERROR_PRINTER_DRIVER_BLOCKED: WIN32_ERROR = WIN32_ERROR(3014u32);
pub const ERROR_PRINTER_DRIVER_PACKAGE_IN_USE: WIN32_ERROR = WIN32_ERROR(3015u32);
pub const ERROR_CORE_DRIVER_PACKAGE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3016u32);
pub const ERROR_FAIL_REBOOT_REQUIRED: WIN32_ERROR = WIN32_ERROR(3017u32);
pub const ERROR_FAIL_REBOOT_INITIATED: WIN32_ERROR = WIN32_ERROR(3018u32);
pub const ERROR_PRINTER_DRIVER_DOWNLOAD_NEEDED: WIN32_ERROR = WIN32_ERROR(3019u32);
pub const ERROR_PRINT_JOB_RESTART_REQUIRED: WIN32_ERROR = WIN32_ERROR(3020u32);
pub const ERROR_INVALID_PRINTER_DRIVER_MANIFEST: WIN32_ERROR = WIN32_ERROR(3021u32);
pub const ERROR_PRINTER_NOT_SHAREABLE: WIN32_ERROR = WIN32_ERROR(3022u32);
pub const ERROR_REQUEST_PAUSED: WIN32_ERROR = WIN32_ERROR(3050u32);
pub const ERROR_APPEXEC_CONDITION_NOT_SATISFIED: WIN32_ERROR = WIN32_ERROR(3060u32);
pub const ERROR_APPEXEC_HANDLE_INVALIDATED: WIN32_ERROR = WIN32_ERROR(3061u32);
pub const ERROR_APPEXEC_INVALID_HOST_GENERATION: WIN32_ERROR = WIN32_ERROR(3062u32);
pub const ERROR_APPEXEC_UNEXPECTED_PROCESS_REGISTRATION: WIN32_ERROR = WIN32_ERROR(3063u32);
pub const ERROR_APPEXEC_INVALID_HOST_STATE: WIN32_ERROR = WIN32_ERROR(3064u32);
pub const ERROR_APPEXEC_NO_DONOR: WIN32_ERROR = WIN32_ERROR(3065u32);
pub const ERROR_APPEXEC_HOST_ID_MISMATCH: WIN32_ERROR = WIN32_ERROR(3066u32);
pub const ERROR_APPEXEC_UNKNOWN_USER: WIN32_ERROR = WIN32_ERROR(3067u32);
pub const ERROR_APPEXEC_APP_COMPAT_BLOCK: WIN32_ERROR = WIN32_ERROR(3068u32);
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT: WIN32_ERROR = WIN32_ERROR(3069u32);
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT_TERMINATION: WIN32_ERROR = WIN32_ERROR(3070u32);
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT_LICENSING: WIN32_ERROR = WIN32_ERROR(3071u32);
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT_RESOURCES: WIN32_ERROR = WIN32_ERROR(3072u32);
pub const ERROR_VRF_VOLATILE_CFG_AND_IO_ENABLED: WIN32_ERROR = WIN32_ERROR(3080u32);
pub const ERROR_VRF_VOLATILE_NOT_STOPPABLE: WIN32_ERROR = WIN32_ERROR(3081u32);
pub const ERROR_VRF_VOLATILE_SAFE_MODE: WIN32_ERROR = WIN32_ERROR(3082u32);
pub const ERROR_VRF_VOLATILE_NOT_RUNNABLE_SYSTEM: WIN32_ERROR = WIN32_ERROR(3083u32);
pub const ERROR_VRF_VOLATILE_NOT_SUPPORTED_RULECLASS: WIN32_ERROR = WIN32_ERROR(3084u32);
pub const ERROR_VRF_VOLATILE_PROTECTED_DRIVER: WIN32_ERROR = WIN32_ERROR(3085u32);
pub const ERROR_VRF_VOLATILE_NMI_REGISTERED: WIN32_ERROR = WIN32_ERROR(3086u32);
pub const ERROR_VRF_VOLATILE_SETTINGS_CONFLICT: WIN32_ERROR = WIN32_ERROR(3087u32);
pub const ERROR_DIF_IOCALLBACK_NOT_REPLACED: WIN32_ERROR = WIN32_ERROR(3190u32);
pub const ERROR_DIF_LIVEDUMP_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(3191u32);
pub const ERROR_DIF_VOLATILE_SECTION_NOT_LOCKED: WIN32_ERROR = WIN32_ERROR(3192u32);
pub const ERROR_DIF_VOLATILE_DRIVER_HOTPATCHED: WIN32_ERROR = WIN32_ERROR(3193u32);
pub const ERROR_DIF_VOLATILE_INVALID_INFO: WIN32_ERROR = WIN32_ERROR(3194u32);
pub const ERROR_DIF_VOLATILE_DRIVER_IS_NOT_RUNNING: WIN32_ERROR = WIN32_ERROR(3195u32);
pub const ERROR_DIF_VOLATILE_PLUGIN_IS_NOT_RUNNING: WIN32_ERROR = WIN32_ERROR(3196u32);
pub const ERROR_DIF_VOLATILE_PLUGIN_CHANGE_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(3197u32);
pub const ERROR_DIF_VOLATILE_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(3198u32);
pub const ERROR_DIF_BINDING_API_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3199u32);
pub const ERROR_IO_REISSUE_AS_CACHED: WIN32_ERROR = WIN32_ERROR(3950u32);
pub const ERROR_WINS_INTERNAL: WIN32_ERROR = WIN32_ERROR(4000u32);
pub const ERROR_CAN_NOT_DEL_LOCAL_WINS: WIN32_ERROR = WIN32_ERROR(4001u32);
pub const ERROR_STATIC_INIT: WIN32_ERROR = WIN32_ERROR(4002u32);
pub const ERROR_INC_BACKUP: WIN32_ERROR = WIN32_ERROR(4003u32);
pub const ERROR_FULL_BACKUP: WIN32_ERROR = WIN32_ERROR(4004u32);
pub const ERROR_REC_NON_EXISTENT: WIN32_ERROR = WIN32_ERROR(4005u32);
pub const ERROR_RPL_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(4006u32);
pub const ERROR_DHCP_ADDRESS_CONFLICT: WIN32_ERROR = WIN32_ERROR(4100u32);
pub const ERROR_WMI_GUID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4200u32);
pub const ERROR_WMI_INSTANCE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4201u32);
pub const ERROR_WMI_ITEMID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4202u32);
pub const ERROR_WMI_TRY_AGAIN: WIN32_ERROR = WIN32_ERROR(4203u32);
pub const ERROR_WMI_DP_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4204u32);
pub const ERROR_WMI_UNRESOLVED_INSTANCE_REF: WIN32_ERROR = WIN32_ERROR(4205u32);
pub const ERROR_WMI_ALREADY_ENABLED: WIN32_ERROR = WIN32_ERROR(4206u32);
pub const ERROR_WMI_GUID_DISCONNECTED: WIN32_ERROR = WIN32_ERROR(4207u32);
pub const ERROR_WMI_SERVER_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(4208u32);
pub const ERROR_WMI_DP_FAILED: WIN32_ERROR = WIN32_ERROR(4209u32);
pub const ERROR_WMI_INVALID_MOF: WIN32_ERROR = WIN32_ERROR(4210u32);
pub const ERROR_WMI_INVALID_REGINFO: WIN32_ERROR = WIN32_ERROR(4211u32);
pub const ERROR_WMI_ALREADY_DISABLED: WIN32_ERROR = WIN32_ERROR(4212u32);
pub const ERROR_WMI_READ_ONLY: WIN32_ERROR = WIN32_ERROR(4213u32);
pub const ERROR_WMI_SET_FAILURE: WIN32_ERROR = WIN32_ERROR(4214u32);
pub const ERROR_NOT_APPCONTAINER: WIN32_ERROR = WIN32_ERROR(4250u32);
pub const ERROR_APPCONTAINER_REQUIRED: WIN32_ERROR = WIN32_ERROR(4251u32);
pub const ERROR_NOT_SUPPORTED_IN_APPCONTAINER: WIN32_ERROR = WIN32_ERROR(4252u32);
pub const ERROR_INVALID_PACKAGE_SID_LENGTH: WIN32_ERROR = WIN32_ERROR(4253u32);
pub const ERROR_INVALID_MEDIA: WIN32_ERROR = WIN32_ERROR(4300u32);
pub const ERROR_INVALID_LIBRARY: WIN32_ERROR = WIN32_ERROR(4301u32);
pub const ERROR_INVALID_MEDIA_POOL: WIN32_ERROR = WIN32_ERROR(4302u32);
pub const ERROR_DRIVE_MEDIA_MISMATCH: WIN32_ERROR = WIN32_ERROR(4303u32);
pub const ERROR_MEDIA_OFFLINE: WIN32_ERROR = WIN32_ERROR(4304u32);
pub const ERROR_LIBRARY_OFFLINE: WIN32_ERROR = WIN32_ERROR(4305u32);
pub const ERROR_EMPTY: WIN32_ERROR = WIN32_ERROR(4306u32);
pub const ERROR_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(4307u32);
pub const ERROR_MEDIA_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(4308u32);
pub const ERROR_RESOURCE_DISABLED: WIN32_ERROR = WIN32_ERROR(4309u32);
pub const ERROR_INVALID_CLEANER: WIN32_ERROR = WIN32_ERROR(4310u32);
pub const ERROR_UNABLE_TO_CLEAN: WIN32_ERROR = WIN32_ERROR(4311u32);
pub const ERROR_OBJECT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4312u32);
pub const ERROR_DATABASE_FAILURE: WIN32_ERROR = WIN32_ERROR(4313u32);
pub const ERROR_DATABASE_FULL: WIN32_ERROR = WIN32_ERROR(4314u32);
pub const ERROR_MEDIA_INCOMPATIBLE: WIN32_ERROR = WIN32_ERROR(4315u32);
pub const ERROR_RESOURCE_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(4316u32);
pub const ERROR_INVALID_OPERATION: WIN32_ERROR = WIN32_ERROR(4317u32);
pub const ERROR_MEDIA_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(4318u32);
pub const ERROR_DEVICE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(4319u32);
pub const ERROR_REQUEST_REFUSED: WIN32_ERROR = WIN32_ERROR(4320u32);
pub const ERROR_INVALID_DRIVE_OBJECT: WIN32_ERROR = WIN32_ERROR(4321u32);
pub const ERROR_LIBRARY_FULL: WIN32_ERROR = WIN32_ERROR(4322u32);
pub const ERROR_MEDIUM_NOT_ACCESSIBLE: WIN32_ERROR = WIN32_ERROR(4323u32);
pub const ERROR_UNABLE_TO_LOAD_MEDIUM: WIN32_ERROR = WIN32_ERROR(4324u32);
pub const ERROR_UNABLE_TO_INVENTORY_DRIVE: WIN32_ERROR = WIN32_ERROR(4325u32);
pub const ERROR_UNABLE_TO_INVENTORY_SLOT: WIN32_ERROR = WIN32_ERROR(4326u32);
pub const ERROR_UNABLE_TO_INVENTORY_TRANSPORT: WIN32_ERROR = WIN32_ERROR(4327u32);
pub const ERROR_TRANSPORT_FULL: WIN32_ERROR = WIN32_ERROR(4328u32);
pub const ERROR_CONTROLLING_IEPORT: WIN32_ERROR = WIN32_ERROR(4329u32);
pub const ERROR_UNABLE_TO_EJECT_MOUNTED_MEDIA: WIN32_ERROR = WIN32_ERROR(4330u32);
pub const ERROR_CLEANER_SLOT_SET: WIN32_ERROR = WIN32_ERROR(4331u32);
pub const ERROR_CLEANER_SLOT_NOT_SET: WIN32_ERROR = WIN32_ERROR(4332u32);
pub const ERROR_CLEANER_CARTRIDGE_SPENT: WIN32_ERROR = WIN32_ERROR(4333u32);
pub const ERROR_UNEXPECTED_OMID: WIN32_ERROR = WIN32_ERROR(4334u32);
pub const ERROR_CANT_DELETE_LAST_ITEM: WIN32_ERROR = WIN32_ERROR(4335u32);
pub const ERROR_MESSAGE_EXCEEDS_MAX_SIZE: WIN32_ERROR = WIN32_ERROR(4336u32);
pub const ERROR_VOLUME_CONTAINS_SYS_FILES: WIN32_ERROR = WIN32_ERROR(4337u32);
pub const ERROR_INDIGENOUS_TYPE: WIN32_ERROR = WIN32_ERROR(4338u32);
pub const ERROR_NO_SUPPORTING_DRIVES: WIN32_ERROR = WIN32_ERROR(4339u32);
pub const ERROR_CLEANER_CARTRIDGE_INSTALLED: WIN32_ERROR = WIN32_ERROR(4340u32);
pub const ERROR_IEPORT_FULL: WIN32_ERROR = WIN32_ERROR(4341u32);
pub const ERROR_FILE_OFFLINE: WIN32_ERROR = WIN32_ERROR(4350u32);
pub const ERROR_REMOTE_STORAGE_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(4351u32);
pub const ERROR_REMOTE_STORAGE_MEDIA_ERROR: WIN32_ERROR = WIN32_ERROR(4352u32);
pub const ERROR_NOT_A_REPARSE_POINT: WIN32_ERROR = WIN32_ERROR(4390u32);
pub const ERROR_REPARSE_ATTRIBUTE_CONFLICT: WIN32_ERROR = WIN32_ERROR(4391u32);
pub const ERROR_INVALID_REPARSE_DATA: WIN32_ERROR = WIN32_ERROR(4392u32);
pub const ERROR_REPARSE_TAG_INVALID: WIN32_ERROR = WIN32_ERROR(4393u32);
pub const ERROR_REPARSE_TAG_MISMATCH: WIN32_ERROR = WIN32_ERROR(4394u32);
pub const ERROR_REPARSE_POINT_ENCOUNTERED: WIN32_ERROR = WIN32_ERROR(4395u32);
pub const ERROR_APP_DATA_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4400u32);
pub const ERROR_APP_DATA_EXPIRED: WIN32_ERROR = WIN32_ERROR(4401u32);
pub const ERROR_APP_DATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(4402u32);
pub const ERROR_APP_DATA_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(4403u32);
pub const ERROR_APP_DATA_REBOOT_REQUIRED: WIN32_ERROR = WIN32_ERROR(4404u32);
pub const ERROR_SECUREBOOT_ROLLBACK_DETECTED: WIN32_ERROR = WIN32_ERROR(4420u32);
pub const ERROR_SECUREBOOT_POLICY_VIOLATION: WIN32_ERROR = WIN32_ERROR(4421u32);
pub const ERROR_SECUREBOOT_INVALID_POLICY: WIN32_ERROR = WIN32_ERROR(4422u32);
pub const ERROR_SECUREBOOT_POLICY_PUBLISHER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4423u32);
pub const ERROR_SECUREBOOT_POLICY_NOT_SIGNED: WIN32_ERROR = WIN32_ERROR(4424u32);
pub const ERROR_SECUREBOOT_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(4425u32);
pub const ERROR_SECUREBOOT_FILE_REPLACED: WIN32_ERROR = WIN32_ERROR(4426u32);
pub const ERROR_SECUREBOOT_POLICY_NOT_AUTHORIZED: WIN32_ERROR = WIN32_ERROR(4427u32);
pub const ERROR_SECUREBOOT_POLICY_UNKNOWN: WIN32_ERROR = WIN32_ERROR(4428u32);
pub const ERROR_SECUREBOOT_POLICY_MISSING_ANTIROLLBACKVERSION: WIN32_ERROR = WIN32_ERROR(4429u32);
pub const ERROR_SECUREBOOT_PLATFORM_ID_MISMATCH: WIN32_ERROR = WIN32_ERROR(4430u32);
pub const ERROR_SECUREBOOT_POLICY_ROLLBACK_DETECTED: WIN32_ERROR = WIN32_ERROR(4431u32);
pub const ERROR_SECUREBOOT_POLICY_UPGRADE_MISMATCH: WIN32_ERROR = WIN32_ERROR(4432u32);
pub const ERROR_SECUREBOOT_REQUIRED_POLICY_FILE_MISSING: WIN32_ERROR = WIN32_ERROR(4433u32);
pub const ERROR_SECUREBOOT_NOT_BASE_POLICY: WIN32_ERROR = WIN32_ERROR(4434u32);
pub const ERROR_SECUREBOOT_NOT_SUPPLEMENTAL_POLICY: WIN32_ERROR = WIN32_ERROR(4435u32);
pub const ERROR_OFFLOAD_READ_FLT_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(4440u32);
pub const ERROR_OFFLOAD_WRITE_FLT_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(4441u32);
pub const ERROR_OFFLOAD_READ_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(4442u32);
pub const ERROR_OFFLOAD_WRITE_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(4443u32);
pub const ERROR_ALREADY_HAS_STREAM_ID: WIN32_ERROR = WIN32_ERROR(4444u32);
pub const ERROR_SMR_GARBAGE_COLLECTION_REQUIRED: WIN32_ERROR = WIN32_ERROR(4445u32);
pub const ERROR_WOF_WIM_HEADER_CORRUPT: WIN32_ERROR = WIN32_ERROR(4446u32);
pub const ERROR_WOF_WIM_RESOURCE_TABLE_CORRUPT: WIN32_ERROR = WIN32_ERROR(4447u32);
pub const ERROR_WOF_FILE_RESOURCE_TABLE_CORRUPT: WIN32_ERROR = WIN32_ERROR(4448u32);
pub const ERROR_OBJECT_IS_IMMUTABLE: WIN32_ERROR = WIN32_ERROR(4449u32);
pub const ERROR_VOLUME_NOT_SIS_ENABLED: WIN32_ERROR = WIN32_ERROR(4500u32);
pub const ERROR_SYSTEM_INTEGRITY_ROLLBACK_DETECTED: WIN32_ERROR = WIN32_ERROR(4550u32);
pub const ERROR_SYSTEM_INTEGRITY_POLICY_VIOLATION: WIN32_ERROR = WIN32_ERROR(4551u32);
pub const ERROR_SYSTEM_INTEGRITY_INVALID_POLICY: WIN32_ERROR = WIN32_ERROR(4552u32);
pub const ERROR_SYSTEM_INTEGRITY_POLICY_NOT_SIGNED: WIN32_ERROR = WIN32_ERROR(4553u32);
pub const ERROR_SYSTEM_INTEGRITY_TOO_MANY_POLICIES: WIN32_ERROR = WIN32_ERROR(4554u32);
pub const ERROR_SYSTEM_INTEGRITY_SUPPLEMENTAL_POLICY_NOT_AUTHORIZED: WIN32_ERROR =
    WIN32_ERROR(4555u32);
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_MALICIOUS: WIN32_ERROR = WIN32_ERROR(4556u32);
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_PUA: WIN32_ERROR = WIN32_ERROR(4557u32);
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_DANGEROUS_EXT: WIN32_ERROR = WIN32_ERROR(4558u32);
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_OFFLINE: WIN32_ERROR = WIN32_ERROR(4559u32);
pub const ERROR_VSM_NOT_INITIALIZED: WIN32_ERROR = WIN32_ERROR(4560u32);
pub const ERROR_VSM_DMA_PROTECTION_NOT_IN_USE: WIN32_ERROR = WIN32_ERROR(4561u32);
pub const ERROR_PLATFORM_MANIFEST_NOT_AUTHORIZED: WIN32_ERROR = WIN32_ERROR(4570u32);
pub const ERROR_PLATFORM_MANIFEST_INVALID: WIN32_ERROR = WIN32_ERROR(4571u32);
pub const ERROR_PLATFORM_MANIFEST_FILE_NOT_AUTHORIZED: WIN32_ERROR = WIN32_ERROR(4572u32);
pub const ERROR_PLATFORM_MANIFEST_CATALOG_NOT_AUTHORIZED: WIN32_ERROR = WIN32_ERROR(4573u32);
pub const ERROR_PLATFORM_MANIFEST_BINARY_ID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4574u32);
pub const ERROR_PLATFORM_MANIFEST_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(4575u32);
pub const ERROR_PLATFORM_MANIFEST_NOT_SIGNED: WIN32_ERROR = WIN32_ERROR(4576u32);
pub const ERROR_DEPENDENT_RESOURCE_EXISTS: WIN32_ERROR = WIN32_ERROR(5001u32);
pub const ERROR_DEPENDENCY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5002u32);
pub const ERROR_DEPENDENCY_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(5003u32);
pub const ERROR_RESOURCE_NOT_ONLINE: WIN32_ERROR = WIN32_ERROR(5004u32);
pub const ERROR_HOST_NODE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5005u32);
pub const ERROR_RESOURCE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5006u32);
pub const ERROR_RESOURCE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5007u32);
pub const ERROR_SHUTDOWN_CLUSTER: WIN32_ERROR = WIN32_ERROR(5008u32);
pub const ERROR_CANT_EVICT_ACTIVE_NODE: WIN32_ERROR = WIN32_ERROR(5009u32);
pub const ERROR_OBJECT_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(5010u32);
pub const ERROR_OBJECT_IN_LIST: WIN32_ERROR = WIN32_ERROR(5011u32);
pub const ERROR_GROUP_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5012u32);
pub const ERROR_GROUP_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5013u32);
pub const ERROR_GROUP_NOT_ONLINE: WIN32_ERROR = WIN32_ERROR(5014u32);
pub const ERROR_HOST_NODE_NOT_RESOURCE_OWNER: WIN32_ERROR = WIN32_ERROR(5015u32);
pub const ERROR_HOST_NODE_NOT_GROUP_OWNER: WIN32_ERROR = WIN32_ERROR(5016u32);
pub const ERROR_RESMON_CREATE_FAILED: WIN32_ERROR = WIN32_ERROR(5017u32);
pub const ERROR_RESMON_ONLINE_FAILED: WIN32_ERROR = WIN32_ERROR(5018u32);
pub const ERROR_RESOURCE_ONLINE: WIN32_ERROR = WIN32_ERROR(5019u32);
pub const ERROR_QUORUM_RESOURCE: WIN32_ERROR = WIN32_ERROR(5020u32);
pub const ERROR_NOT_QUORUM_CAPABLE: WIN32_ERROR = WIN32_ERROR(5021u32);
pub const ERROR_CLUSTER_SHUTTING_DOWN: WIN32_ERROR = WIN32_ERROR(5022u32);
pub const ERROR_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(5023u32);
pub const ERROR_RESOURCE_PROPERTIES_STORED: WIN32_ERROR = WIN32_ERROR(5024u32);
pub const ERROR_NOT_QUORUM_CLASS: WIN32_ERROR = WIN32_ERROR(5025u32);
pub const ERROR_CORE_RESOURCE: WIN32_ERROR = WIN32_ERROR(5026u32);
pub const ERROR_QUORUM_RESOURCE_ONLINE_FAILED: WIN32_ERROR = WIN32_ERROR(5027u32);
pub const ERROR_QUORUMLOG_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(5028u32);
pub const ERROR_CLUSTERLOG_CORRUPT: WIN32_ERROR = WIN32_ERROR(5029u32);
pub const ERROR_CLUSTERLOG_RECORD_EXCEEDS_MAXSIZE: WIN32_ERROR = WIN32_ERROR(5030u32);
pub const ERROR_CLUSTERLOG_EXCEEDS_MAXSIZE: WIN32_ERROR = WIN32_ERROR(5031u32);
pub const ERROR_CLUSTERLOG_CHKPOINT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5032u32);
pub const ERROR_CLUSTERLOG_NOT_ENOUGH_SPACE: WIN32_ERROR = WIN32_ERROR(5033u32);
pub const ERROR_QUORUM_OWNER_ALIVE: WIN32_ERROR = WIN32_ERROR(5034u32);
pub const ERROR_NETWORK_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5035u32);
pub const ERROR_NODE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5036u32);
pub const ERROR_ALL_NODES_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5037u32);
pub const ERROR_RESOURCE_FAILED: WIN32_ERROR = WIN32_ERROR(5038u32);
pub const ERROR_CLUSTER_INVALID_NODE: WIN32_ERROR = WIN32_ERROR(5039u32);
pub const ERROR_CLUSTER_NODE_EXISTS: WIN32_ERROR = WIN32_ERROR(5040u32);
pub const ERROR_CLUSTER_JOIN_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5041u32);
pub const ERROR_CLUSTER_NODE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5042u32);
pub const ERROR_CLUSTER_LOCAL_NODE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5043u32);
pub const ERROR_CLUSTER_NETWORK_EXISTS: WIN32_ERROR = WIN32_ERROR(5044u32);
pub const ERROR_CLUSTER_NETWORK_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5045u32);
pub const ERROR_CLUSTER_NETINTERFACE_EXISTS: WIN32_ERROR = WIN32_ERROR(5046u32);
pub const ERROR_CLUSTER_NETINTERFACE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5047u32);
pub const ERROR_CLUSTER_INVALID_REQUEST: WIN32_ERROR = WIN32_ERROR(5048u32);
pub const ERROR_CLUSTER_INVALID_NETWORK_PROVIDER: WIN32_ERROR = WIN32_ERROR(5049u32);
pub const ERROR_CLUSTER_NODE_DOWN: WIN32_ERROR = WIN32_ERROR(5050u32);
pub const ERROR_CLUSTER_NODE_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(5051u32);
pub const ERROR_CLUSTER_NODE_NOT_MEMBER: WIN32_ERROR = WIN32_ERROR(5052u32);
pub const ERROR_CLUSTER_JOIN_NOT_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5053u32);
pub const ERROR_CLUSTER_INVALID_NETWORK: WIN32_ERROR = WIN32_ERROR(5054u32);
pub const ERROR_CLUSTER_NODE_UP: WIN32_ERROR = WIN32_ERROR(5056u32);
pub const ERROR_CLUSTER_IPADDR_IN_USE: WIN32_ERROR = WIN32_ERROR(5057u32);
pub const ERROR_CLUSTER_NODE_NOT_PAUSED: WIN32_ERROR = WIN32_ERROR(5058u32);
pub const ERROR_CLUSTER_NO_SECURITY_CONTEXT: WIN32_ERROR = WIN32_ERROR(5059u32);
pub const ERROR_CLUSTER_NETWORK_NOT_INTERNAL: WIN32_ERROR = WIN32_ERROR(5060u32);
pub const ERROR_CLUSTER_NODE_ALREADY_UP: WIN32_ERROR = WIN32_ERROR(5061u32);
pub const ERROR_CLUSTER_NODE_ALREADY_DOWN: WIN32_ERROR = WIN32_ERROR(5062u32);
pub const ERROR_CLUSTER_NETWORK_ALREADY_ONLINE: WIN32_ERROR = WIN32_ERROR(5063u32);
pub const ERROR_CLUSTER_NETWORK_ALREADY_OFFLINE: WIN32_ERROR = WIN32_ERROR(5064u32);
pub const ERROR_CLUSTER_NODE_ALREADY_MEMBER: WIN32_ERROR = WIN32_ERROR(5065u32);
pub const ERROR_CLUSTER_LAST_INTERNAL_NETWORK: WIN32_ERROR = WIN32_ERROR(5066u32);
pub const ERROR_CLUSTER_NETWORK_HAS_DEPENDENTS: WIN32_ERROR = WIN32_ERROR(5067u32);
pub const ERROR_INVALID_OPERATION_ON_QUORUM: WIN32_ERROR = WIN32_ERROR(5068u32);
pub const ERROR_DEPENDENCY_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(5069u32);
pub const ERROR_CLUSTER_NODE_PAUSED: WIN32_ERROR = WIN32_ERROR(5070u32);
pub const ERROR_NODE_CANT_HOST_RESOURCE: WIN32_ERROR = WIN32_ERROR(5071u32);
pub const ERROR_CLUSTER_NODE_NOT_READY: WIN32_ERROR = WIN32_ERROR(5072u32);
pub const ERROR_CLUSTER_NODE_SHUTTING_DOWN: WIN32_ERROR = WIN32_ERROR(5073u32);
pub const ERROR_CLUSTER_JOIN_ABORTED: WIN32_ERROR = WIN32_ERROR(5074u32);
pub const ERROR_CLUSTER_INCOMPATIBLE_VERSIONS: WIN32_ERROR = WIN32_ERROR(5075u32);
pub const ERROR_CLUSTER_MAXNUM_OF_RESOURCES_EXCEEDED: WIN32_ERROR = WIN32_ERROR(5076u32);
pub const ERROR_CLUSTER_SYSTEM_CONFIG_CHANGED: WIN32_ERROR = WIN32_ERROR(5077u32);
pub const ERROR_CLUSTER_RESOURCE_TYPE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5078u32);
pub const ERROR_CLUSTER_RESTYPE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(5079u32);
pub const ERROR_CLUSTER_RESNAME_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5080u32);
pub const ERROR_CLUSTER_NO_RPC_PACKAGES_REGISTERED: WIN32_ERROR = WIN32_ERROR(5081u32);
pub const ERROR_CLUSTER_OWNER_NOT_IN_PREFLIST: WIN32_ERROR = WIN32_ERROR(5082u32);
pub const ERROR_CLUSTER_DATABASE_SEQMISMATCH: WIN32_ERROR = WIN32_ERROR(5083u32);
pub const ERROR_RESMON_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(5084u32);
pub const ERROR_CLUSTER_GUM_NOT_LOCKER: WIN32_ERROR = WIN32_ERROR(5085u32);
pub const ERROR_QUORUM_DISK_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5086u32);
pub const ERROR_DATABASE_BACKUP_CORRUPT: WIN32_ERROR = WIN32_ERROR(5087u32);
pub const ERROR_CLUSTER_NODE_ALREADY_HAS_DFS_ROOT: WIN32_ERROR = WIN32_ERROR(5088u32);
pub const ERROR_RESOURCE_PROPERTY_UNCHANGEABLE: WIN32_ERROR = WIN32_ERROR(5089u32);
pub const ERROR_NO_ADMIN_ACCESS_POINT: WIN32_ERROR = WIN32_ERROR(5090u32);
pub const ERROR_CLUSTER_MEMBERSHIP_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(5890u32);
pub const ERROR_CLUSTER_QUORUMLOG_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5891u32);
pub const ERROR_CLUSTER_MEMBERSHIP_HALT: WIN32_ERROR = WIN32_ERROR(5892u32);
pub const ERROR_CLUSTER_INSTANCE_ID_MISMATCH: WIN32_ERROR = WIN32_ERROR(5893u32);
pub const ERROR_CLUSTER_NETWORK_NOT_FOUND_FOR_IP: WIN32_ERROR = WIN32_ERROR(5894u32);
pub const ERROR_CLUSTER_PROPERTY_DATA_TYPE_MISMATCH: WIN32_ERROR = WIN32_ERROR(5895u32);
pub const ERROR_CLUSTER_EVICT_WITHOUT_CLEANUP: WIN32_ERROR = WIN32_ERROR(5896u32);
pub const ERROR_CLUSTER_PARAMETER_MISMATCH: WIN32_ERROR = WIN32_ERROR(5897u32);
pub const ERROR_NODE_CANNOT_BE_CLUSTERED: WIN32_ERROR = WIN32_ERROR(5898u32);
pub const ERROR_CLUSTER_WRONG_OS_VERSION: WIN32_ERROR = WIN32_ERROR(5899u32);
pub const ERROR_CLUSTER_CANT_CREATE_DUP_CLUSTER_NAME: WIN32_ERROR = WIN32_ERROR(5900u32);
pub const ERROR_CLUSCFG_ALREADY_COMMITTED: WIN32_ERROR = WIN32_ERROR(5901u32);
pub const ERROR_CLUSCFG_ROLLBACK_FAILED: WIN32_ERROR = WIN32_ERROR(5902u32);
pub const ERROR_CLUSCFG_SYSTEM_DISK_DRIVE_LETTER_CONFLICT: WIN32_ERROR = WIN32_ERROR(5903u32);
pub const ERROR_CLUSTER_OLD_VERSION: WIN32_ERROR = WIN32_ERROR(5904u32);
pub const ERROR_CLUSTER_MISMATCHED_COMPUTER_ACCT_NAME: WIN32_ERROR = WIN32_ERROR(5905u32);
pub const ERROR_CLUSTER_NO_NET_ADAPTERS: WIN32_ERROR = WIN32_ERROR(5906u32);
pub const ERROR_CLUSTER_POISONED: WIN32_ERROR = WIN32_ERROR(5907u32);
pub const ERROR_CLUSTER_GROUP_MOVING: WIN32_ERROR = WIN32_ERROR(5908u32);
pub const ERROR_CLUSTER_RESOURCE_TYPE_BUSY: WIN32_ERROR = WIN32_ERROR(5909u32);
pub const ERROR_RESOURCE_CALL_TIMED_OUT: WIN32_ERROR = WIN32_ERROR(5910u32);
pub const ERROR_INVALID_CLUSTER_IPV6_ADDRESS: WIN32_ERROR = WIN32_ERROR(5911u32);
pub const ERROR_CLUSTER_INTERNAL_INVALID_FUNCTION: WIN32_ERROR = WIN32_ERROR(5912u32);
pub const ERROR_CLUSTER_PARAMETER_OUT_OF_BOUNDS: WIN32_ERROR = WIN32_ERROR(5913u32);
pub const ERROR_CLUSTER_PARTIAL_SEND: WIN32_ERROR = WIN32_ERROR(5914u32);
pub const ERROR_CLUSTER_REGISTRY_INVALID_FUNCTION: WIN32_ERROR = WIN32_ERROR(5915u32);
pub const ERROR_CLUSTER_INVALID_STRING_TERMINATION: WIN32_ERROR = WIN32_ERROR(5916u32);
pub const ERROR_CLUSTER_INVALID_STRING_FORMAT: WIN32_ERROR = WIN32_ERROR(5917u32);
pub const ERROR_CLUSTER_DATABASE_TRANSACTION_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5918u32);
pub const ERROR_CLUSTER_DATABASE_TRANSACTION_NOT_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5919u32);
pub const ERROR_CLUSTER_NULL_DATA: WIN32_ERROR = WIN32_ERROR(5920u32);
pub const ERROR_CLUSTER_PARTIAL_READ: WIN32_ERROR = WIN32_ERROR(5921u32);
pub const ERROR_CLUSTER_PARTIAL_WRITE: WIN32_ERROR = WIN32_ERROR(5922u32);
pub const ERROR_CLUSTER_CANT_DESERIALIZE_DATA: WIN32_ERROR = WIN32_ERROR(5923u32);
pub const ERROR_DEPENDENT_RESOURCE_PROPERTY_CONFLICT: WIN32_ERROR = WIN32_ERROR(5924u32);
pub const ERROR_CLUSTER_NO_QUORUM: WIN32_ERROR = WIN32_ERROR(5925u32);
pub const ERROR_CLUSTER_INVALID_IPV6_NETWORK: WIN32_ERROR = WIN32_ERROR(5926u32);
pub const ERROR_CLUSTER_INVALID_IPV6_TUNNEL_NETWORK: WIN32_ERROR = WIN32_ERROR(5927u32);
pub const ERROR_QUORUM_NOT_ALLOWED_IN_THIS_GROUP: WIN32_ERROR = WIN32_ERROR(5928u32);
pub const ERROR_DEPENDENCY_TREE_TOO_COMPLEX: WIN32_ERROR = WIN32_ERROR(5929u32);
pub const ERROR_EXCEPTION_IN_RESOURCE_CALL: WIN32_ERROR = WIN32_ERROR(5930u32);
pub const ERROR_CLUSTER_RHS_FAILED_INITIALIZATION: WIN32_ERROR = WIN32_ERROR(5931u32);
pub const ERROR_CLUSTER_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(5932u32);
pub const ERROR_CLUSTER_RESOURCES_MUST_BE_ONLINE_ON_THE_SAME_NODE: WIN32_ERROR =
    WIN32_ERROR(5933u32);
pub const ERROR_CLUSTER_MAX_NODES_IN_CLUSTER: WIN32_ERROR = WIN32_ERROR(5934u32);
pub const ERROR_CLUSTER_TOO_MANY_NODES: WIN32_ERROR = WIN32_ERROR(5935u32);
pub const ERROR_CLUSTER_OBJECT_ALREADY_USED: WIN32_ERROR = WIN32_ERROR(5936u32);
pub const ERROR_NONCORE_GROUPS_FOUND: WIN32_ERROR = WIN32_ERROR(5937u32);
pub const ERROR_FILE_SHARE_RESOURCE_CONFLICT: WIN32_ERROR = WIN32_ERROR(5938u32);
pub const ERROR_CLUSTER_EVICT_INVALID_REQUEST: WIN32_ERROR = WIN32_ERROR(5939u32);
pub const ERROR_CLUSTER_SINGLETON_RESOURCE: WIN32_ERROR = WIN32_ERROR(5940u32);
pub const ERROR_CLUSTER_GROUP_SINGLETON_RESOURCE: WIN32_ERROR = WIN32_ERROR(5941u32);
pub const ERROR_CLUSTER_RESOURCE_PROVIDER_FAILED: WIN32_ERROR = WIN32_ERROR(5942u32);
pub const ERROR_CLUSTER_RESOURCE_CONFIGURATION_ERROR: WIN32_ERROR = WIN32_ERROR(5943u32);
pub const ERROR_CLUSTER_GROUP_BUSY: WIN32_ERROR = WIN32_ERROR(5944u32);
pub const ERROR_CLUSTER_NOT_SHARED_VOLUME: WIN32_ERROR = WIN32_ERROR(5945u32);
pub const ERROR_CLUSTER_INVALID_SECURITY_DESCRIPTOR: WIN32_ERROR = WIN32_ERROR(5946u32);
pub const ERROR_CLUSTER_SHARED_VOLUMES_IN_USE: WIN32_ERROR = WIN32_ERROR(5947u32);
pub const ERROR_CLUSTER_USE_SHARED_VOLUMES_API: WIN32_ERROR = WIN32_ERROR(5948u32);
pub const ERROR_CLUSTER_BACKUP_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5949u32);
pub const ERROR_NON_CSV_PATH: WIN32_ERROR = WIN32_ERROR(5950u32);
pub const ERROR_CSV_VOLUME_NOT_LOCAL: WIN32_ERROR = WIN32_ERROR(5951u32);
pub const ERROR_CLUSTER_WATCHDOG_TERMINATING: WIN32_ERROR = WIN32_ERROR(5952u32);
pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_INCOMPATIBLE_NODES: WIN32_ERROR = WIN32_ERROR(5953u32);
pub const ERROR_CLUSTER_INVALID_NODE_WEIGHT: WIN32_ERROR = WIN32_ERROR(5954u32);
pub const ERROR_CLUSTER_RESOURCE_VETOED_CALL: WIN32_ERROR = WIN32_ERROR(5955u32);
pub const ERROR_RESMON_SYSTEM_RESOURCES_LACKING: WIN32_ERROR = WIN32_ERROR(5956u32);
pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_NOT_ENOUGH_RESOURCES_ON_DESTINATION: WIN32_ERROR =
    WIN32_ERROR(5957u32);
pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_NOT_ENOUGH_RESOURCES_ON_SOURCE: WIN32_ERROR =
    WIN32_ERROR(5958u32);
pub const ERROR_CLUSTER_GROUP_QUEUED: WIN32_ERROR = WIN32_ERROR(5959u32);
pub const ERROR_CLUSTER_RESOURCE_LOCKED_STATUS: WIN32_ERROR = WIN32_ERROR(5960u32);
pub const ERROR_CLUSTER_SHARED_VOLUME_FAILOVER_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(5961u32);
pub const ERROR_CLUSTER_NODE_DRAIN_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5962u32);
pub const ERROR_CLUSTER_DISK_NOT_CONNECTED: WIN32_ERROR = WIN32_ERROR(5963u32);
pub const ERROR_DISK_NOT_CSV_CAPABLE: WIN32_ERROR = WIN32_ERROR(5964u32);
pub const ERROR_RESOURCE_NOT_IN_AVAILABLE_STORAGE: WIN32_ERROR = WIN32_ERROR(5965u32);
pub const ERROR_CLUSTER_SHARED_VOLUME_REDIRECTED: WIN32_ERROR = WIN32_ERROR(5966u32);
pub const ERROR_CLUSTER_SHARED_VOLUME_NOT_REDIRECTED: WIN32_ERROR = WIN32_ERROR(5967u32);
pub const ERROR_CLUSTER_CANNOT_RETURN_PROPERTIES: WIN32_ERROR = WIN32_ERROR(5968u32);
pub const ERROR_CLUSTER_RESOURCE_CONTAINS_UNSUPPORTED_DIFF_AREA_FOR_SHARED_VOLUMES: WIN32_ERROR =
    WIN32_ERROR(5969u32);
pub const ERROR_CLUSTER_RESOURCE_IS_IN_MAINTENANCE_MODE: WIN32_ERROR = WIN32_ERROR(5970u32);
pub const ERROR_CLUSTER_AFFINITY_CONFLICT: WIN32_ERROR = WIN32_ERROR(5971u32);
pub const ERROR_CLUSTER_RESOURCE_IS_REPLICA_VIRTUAL_MACHINE: WIN32_ERROR = WIN32_ERROR(5972u32);
pub const ERROR_CLUSTER_UPGRADE_INCOMPATIBLE_VERSIONS: WIN32_ERROR = WIN32_ERROR(5973u32);
pub const ERROR_CLUSTER_UPGRADE_FIX_QUORUM_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(5974u32);
pub const ERROR_CLUSTER_UPGRADE_RESTART_REQUIRED: WIN32_ERROR = WIN32_ERROR(5975u32);
pub const ERROR_CLUSTER_UPGRADE_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5976u32);
pub const ERROR_CLUSTER_UPGRADE_INCOMPLETE: WIN32_ERROR = WIN32_ERROR(5977u32);
pub const ERROR_CLUSTER_NODE_IN_GRACE_PERIOD: WIN32_ERROR = WIN32_ERROR(5978u32);
pub const ERROR_CLUSTER_CSV_IO_PAUSE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(5979u32);
pub const ERROR_NODE_NOT_ACTIVE_CLUSTER_MEMBER: WIN32_ERROR = WIN32_ERROR(5980u32);
pub const ERROR_CLUSTER_RESOURCE_NOT_MONITORED: WIN32_ERROR = WIN32_ERROR(5981u32);
pub const ERROR_CLUSTER_RESOURCE_DOES_NOT_SUPPORT_UNMONITORED: WIN32_ERROR = WIN32_ERROR(5982u32);
pub const ERROR_CLUSTER_RESOURCE_IS_REPLICATED: WIN32_ERROR = WIN32_ERROR(5983u32);
pub const ERROR_CLUSTER_NODE_ISOLATED: WIN32_ERROR = WIN32_ERROR(5984u32);
pub const ERROR_CLUSTER_NODE_QUARANTINED: WIN32_ERROR = WIN32_ERROR(5985u32);
pub const ERROR_CLUSTER_DATABASE_UPDATE_CONDITION_FAILED: WIN32_ERROR = WIN32_ERROR(5986u32);
pub const ERROR_CLUSTER_SPACE_DEGRADED: WIN32_ERROR = WIN32_ERROR(5987u32);
pub const ERROR_CLUSTER_TOKEN_DELEGATION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(5988u32);
pub const ERROR_CLUSTER_CSV_INVALID_HANDLE: WIN32_ERROR = WIN32_ERROR(5989u32);
pub const ERROR_CLUSTER_CSV_SUPPORTED_ONLY_ON_COORDINATOR: WIN32_ERROR = WIN32_ERROR(5990u32);
pub const ERROR_GROUPSET_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5991u32);
pub const ERROR_GROUPSET_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5992u32);
pub const ERROR_GROUPSET_CANT_PROVIDE: WIN32_ERROR = WIN32_ERROR(5993u32);
pub const ERROR_CLUSTER_FAULT_DOMAIN_PARENT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5994u32);
pub const ERROR_CLUSTER_FAULT_DOMAIN_INVALID_HIERARCHY: WIN32_ERROR = WIN32_ERROR(5995u32);
pub const ERROR_CLUSTER_FAULT_DOMAIN_FAILED_S2D_VALIDATION: WIN32_ERROR = WIN32_ERROR(5996u32);
pub const ERROR_CLUSTER_FAULT_DOMAIN_S2D_CONNECTIVITY_LOSS: WIN32_ERROR = WIN32_ERROR(5997u32);
pub const ERROR_CLUSTER_INVALID_INFRASTRUCTURE_FILESERVER_NAME: WIN32_ERROR = WIN32_ERROR(5998u32);
pub const ERROR_CLUSTERSET_MANAGEMENT_CLUSTER_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(5999u32);
pub const ERROR_ENCRYPTION_FAILED: WIN32_ERROR = WIN32_ERROR(6000u32);
pub const ERROR_DECRYPTION_FAILED: WIN32_ERROR = WIN32_ERROR(6001u32);
pub const ERROR_FILE_ENCRYPTED: WIN32_ERROR = WIN32_ERROR(6002u32);
pub const ERROR_NO_RECOVERY_POLICY: WIN32_ERROR = WIN32_ERROR(6003u32);
pub const ERROR_NO_EFS: WIN32_ERROR = WIN32_ERROR(6004u32);
pub const ERROR_WRONG_EFS: WIN32_ERROR = WIN32_ERROR(6005u32);
pub const ERROR_NO_USER_KEYS: WIN32_ERROR = WIN32_ERROR(6006u32);
pub const ERROR_FILE_NOT_ENCRYPTED: WIN32_ERROR = WIN32_ERROR(6007u32);
pub const ERROR_NOT_EXPORT_FORMAT: WIN32_ERROR = WIN32_ERROR(6008u32);
pub const ERROR_FILE_READ_ONLY: WIN32_ERROR = WIN32_ERROR(6009u32);
pub const ERROR_DIR_EFS_DISALLOWED: WIN32_ERROR = WIN32_ERROR(6010u32);
pub const ERROR_EFS_SERVER_NOT_TRUSTED: WIN32_ERROR = WIN32_ERROR(6011u32);
pub const ERROR_BAD_RECOVERY_POLICY: WIN32_ERROR = WIN32_ERROR(6012u32);
pub const ERROR_EFS_ALG_BLOB_TOO_BIG: WIN32_ERROR = WIN32_ERROR(6013u32);
pub const ERROR_VOLUME_NOT_SUPPORT_EFS: WIN32_ERROR = WIN32_ERROR(6014u32);
pub const ERROR_EFS_DISABLED: WIN32_ERROR = WIN32_ERROR(6015u32);
pub const ERROR_EFS_VERSION_NOT_SUPPORT: WIN32_ERROR = WIN32_ERROR(6016u32);
pub const ERROR_CS_ENCRYPTION_INVALID_SERVER_RESPONSE: WIN32_ERROR = WIN32_ERROR(6017u32);
pub const ERROR_CS_ENCRYPTION_UNSUPPORTED_SERVER: WIN32_ERROR = WIN32_ERROR(6018u32);
pub const ERROR_CS_ENCRYPTION_EXISTING_ENCRYPTED_FILE: WIN32_ERROR = WIN32_ERROR(6019u32);
pub const ERROR_CS_ENCRYPTION_NEW_ENCRYPTED_FILE: WIN32_ERROR = WIN32_ERROR(6020u32);
pub const ERROR_CS_ENCRYPTION_FILE_NOT_CSE: WIN32_ERROR = WIN32_ERROR(6021u32);
pub const ERROR_ENCRYPTION_POLICY_DENIES_OPERATION: WIN32_ERROR = WIN32_ERROR(6022u32);
pub const ERROR_WIP_ENCRYPTION_FAILED: WIN32_ERROR = WIN32_ERROR(6023u32);
pub const ERROR_NO_BROWSER_SERVERS_FOUND: WIN32_ERROR = WIN32_ERROR(6118u32);
pub const ERROR_CLUSTER_OBJECT_IS_CLUSTER_SET_VM: WIN32_ERROR = WIN32_ERROR(6250u32);
pub const ERROR_LOG_SECTOR_INVALID: WIN32_ERROR = WIN32_ERROR(6600u32);
pub const ERROR_LOG_SECTOR_PARITY_INVALID: WIN32_ERROR = WIN32_ERROR(6601u32);
pub const ERROR_LOG_SECTOR_REMAPPED: WIN32_ERROR = WIN32_ERROR(6602u32);
pub const ERROR_LOG_BLOCK_INCOMPLETE: WIN32_ERROR = WIN32_ERROR(6603u32);
pub const ERROR_LOG_INVALID_RANGE: WIN32_ERROR = WIN32_ERROR(6604u32);
pub const ERROR_LOG_BLOCKS_EXHAUSTED: WIN32_ERROR = WIN32_ERROR(6605u32);
pub const ERROR_LOG_READ_CONTEXT_INVALID: WIN32_ERROR = WIN32_ERROR(6606u32);
pub const ERROR_LOG_RESTART_INVALID: WIN32_ERROR = WIN32_ERROR(6607u32);
pub const ERROR_LOG_BLOCK_VERSION: WIN32_ERROR = WIN32_ERROR(6608u32);
pub const ERROR_LOG_BLOCK_INVALID: WIN32_ERROR = WIN32_ERROR(6609u32);
pub const ERROR_LOG_READ_MODE_INVALID: WIN32_ERROR = WIN32_ERROR(6610u32);
pub const ERROR_LOG_NO_RESTART: WIN32_ERROR = WIN32_ERROR(6611u32);
pub const ERROR_LOG_METADATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(6612u32);
pub const ERROR_LOG_METADATA_INVALID: WIN32_ERROR = WIN32_ERROR(6613u32);
pub const ERROR_LOG_METADATA_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(6614u32);
pub const ERROR_LOG_RESERVATION_INVALID: WIN32_ERROR = WIN32_ERROR(6615u32);
pub const ERROR_LOG_CANT_DELETE: WIN32_ERROR = WIN32_ERROR(6616u32);
pub const ERROR_LOG_CONTAINER_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(6617u32);
pub const ERROR_LOG_START_OF_LOG: WIN32_ERROR = WIN32_ERROR(6618u32);
pub const ERROR_LOG_POLICY_ALREADY_INSTALLED: WIN32_ERROR = WIN32_ERROR(6619u32);
pub const ERROR_LOG_POLICY_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(6620u32);
pub const ERROR_LOG_POLICY_INVALID: WIN32_ERROR = WIN32_ERROR(6621u32);
pub const ERROR_LOG_POLICY_CONFLICT: WIN32_ERROR = WIN32_ERROR(6622u32);
pub const ERROR_LOG_PINNED_ARCHIVE_TAIL: WIN32_ERROR = WIN32_ERROR(6623u32);
pub const ERROR_LOG_RECORD_NONEXISTENT: WIN32_ERROR = WIN32_ERROR(6624u32);
pub const ERROR_LOG_RECORDS_RESERVED_INVALID: WIN32_ERROR = WIN32_ERROR(6625u32);
pub const ERROR_LOG_SPACE_RESERVED_INVALID: WIN32_ERROR = WIN32_ERROR(6626u32);
pub const ERROR_LOG_TAIL_INVALID: WIN32_ERROR = WIN32_ERROR(6627u32);
pub const ERROR_LOG_FULL: WIN32_ERROR = WIN32_ERROR(6628u32);
pub const ERROR_COULD_NOT_RESIZE_LOG: WIN32_ERROR = WIN32_ERROR(6629u32);
pub const ERROR_LOG_MULTIPLEXED: WIN32_ERROR = WIN32_ERROR(6630u32);
pub const ERROR_LOG_DEDICATED: WIN32_ERROR = WIN32_ERROR(6631u32);
pub const ERROR_LOG_ARCHIVE_NOT_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(6632u32);
pub const ERROR_LOG_ARCHIVE_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(6633u32);
pub const ERROR_LOG_EPHEMERAL: WIN32_ERROR = WIN32_ERROR(6634u32);
pub const ERROR_LOG_NOT_ENOUGH_CONTAINERS: WIN32_ERROR = WIN32_ERROR(6635u32);
pub const ERROR_LOG_CLIENT_ALREADY_REGISTERED: WIN32_ERROR = WIN32_ERROR(6636u32);
pub const ERROR_LOG_CLIENT_NOT_REGISTERED: WIN32_ERROR = WIN32_ERROR(6637u32);
pub const ERROR_LOG_FULL_HANDLER_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(6638u32);
pub const ERROR_LOG_CONTAINER_READ_FAILED: WIN32_ERROR = WIN32_ERROR(6639u32);
pub const ERROR_LOG_CONTAINER_WRITE_FAILED: WIN32_ERROR = WIN32_ERROR(6640u32);
pub const ERROR_LOG_CONTAINER_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(6641u32);
pub const ERROR_LOG_CONTAINER_STATE_INVALID: WIN32_ERROR = WIN32_ERROR(6642u32);
pub const ERROR_LOG_STATE_INVALID: WIN32_ERROR = WIN32_ERROR(6643u32);
pub const ERROR_LOG_PINNED: WIN32_ERROR = WIN32_ERROR(6644u32);
pub const ERROR_LOG_METADATA_FLUSH_FAILED: WIN32_ERROR = WIN32_ERROR(6645u32);
pub const ERROR_LOG_INCONSISTENT_SECURITY: WIN32_ERROR = WIN32_ERROR(6646u32);
pub const ERROR_LOG_APPENDED_FLUSH_FAILED: WIN32_ERROR = WIN32_ERROR(6647u32);
pub const ERROR_LOG_PINNED_RESERVATION: WIN32_ERROR = WIN32_ERROR(6648u32);
pub const ERROR_INVALID_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6700u32);
pub const ERROR_TRANSACTION_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(6701u32);
pub const ERROR_TRANSACTION_REQUEST_NOT_VALID: WIN32_ERROR = WIN32_ERROR(6702u32);
pub const ERROR_TRANSACTION_NOT_REQUESTED: WIN32_ERROR = WIN32_ERROR(6703u32);
pub const ERROR_TRANSACTION_ALREADY_ABORTED: WIN32_ERROR = WIN32_ERROR(6704u32);
pub const ERROR_TRANSACTION_ALREADY_COMMITTED: WIN32_ERROR = WIN32_ERROR(6705u32);
pub const ERROR_TM_INITIALIZATION_FAILED: WIN32_ERROR = WIN32_ERROR(6706u32);
pub const ERROR_RESOURCEMANAGER_READ_ONLY: WIN32_ERROR = WIN32_ERROR(6707u32);
pub const ERROR_TRANSACTION_NOT_JOINED: WIN32_ERROR = WIN32_ERROR(6708u32);
pub const ERROR_TRANSACTION_SUPERIOR_EXISTS: WIN32_ERROR = WIN32_ERROR(6709u32);
pub const ERROR_CRM_PROTOCOL_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(6710u32);
pub const ERROR_TRANSACTION_PROPAGATION_FAILED: WIN32_ERROR = WIN32_ERROR(6711u32);
pub const ERROR_CRM_PROTOCOL_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6712u32);
pub const ERROR_TRANSACTION_INVALID_MARSHALL_BUFFER: WIN32_ERROR = WIN32_ERROR(6713u32);
pub const ERROR_CURRENT_TRANSACTION_NOT_VALID: WIN32_ERROR = WIN32_ERROR(6714u32);
pub const ERROR_TRANSACTION_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6715u32);
pub const ERROR_RESOURCEMANAGER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6716u32);
pub const ERROR_ENLISTMENT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6717u32);
pub const ERROR_TRANSACTIONMANAGER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6718u32);
pub const ERROR_TRANSACTIONMANAGER_NOT_ONLINE: WIN32_ERROR = WIN32_ERROR(6719u32);
pub const ERROR_TRANSACTIONMANAGER_RECOVERY_NAME_COLLISION: WIN32_ERROR = WIN32_ERROR(6720u32);
pub const ERROR_TRANSACTION_NOT_ROOT: WIN32_ERROR = WIN32_ERROR(6721u32);
pub const ERROR_TRANSACTION_OBJECT_EXPIRED: WIN32_ERROR = WIN32_ERROR(6722u32);
pub const ERROR_TRANSACTION_RESPONSE_NOT_ENLISTED: WIN32_ERROR = WIN32_ERROR(6723u32);
pub const ERROR_TRANSACTION_RECORD_TOO_LONG: WIN32_ERROR = WIN32_ERROR(6724u32);
pub const ERROR_IMPLICIT_TRANSACTION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(6725u32);
pub const ERROR_TRANSACTION_INTEGRITY_VIOLATED: WIN32_ERROR = WIN32_ERROR(6726u32);
pub const ERROR_TRANSACTIONMANAGER_IDENTITY_MISMATCH: WIN32_ERROR = WIN32_ERROR(6727u32);
pub const ERROR_RM_CANNOT_BE_FROZEN_FOR_SNAPSHOT: WIN32_ERROR = WIN32_ERROR(6728u32);
pub const ERROR_TRANSACTION_MUST_WRITETHROUGH: WIN32_ERROR = WIN32_ERROR(6729u32);
pub const ERROR_TRANSACTION_NO_SUPERIOR: WIN32_ERROR = WIN32_ERROR(6730u32);
pub const ERROR_HEURISTIC_DAMAGE_POSSIBLE: WIN32_ERROR = WIN32_ERROR(6731u32);
pub const ERROR_TRANSACTIONAL_CONFLICT: WIN32_ERROR = WIN32_ERROR(6800u32);
pub const ERROR_RM_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(6801u32);
pub const ERROR_RM_METADATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(6802u32);
pub const ERROR_DIRECTORY_NOT_RM: WIN32_ERROR = WIN32_ERROR(6803u32);
pub const ERROR_TRANSACTIONS_UNSUPPORTED_REMOTE: WIN32_ERROR = WIN32_ERROR(6805u32);
pub const ERROR_LOG_RESIZE_INVALID_SIZE: WIN32_ERROR = WIN32_ERROR(6806u32);
pub const ERROR_OBJECT_NO_LONGER_EXISTS: WIN32_ERROR = WIN32_ERROR(6807u32);
pub const ERROR_STREAM_MINIVERSION_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6808u32);
pub const ERROR_STREAM_MINIVERSION_NOT_VALID: WIN32_ERROR = WIN32_ERROR(6809u32);
pub const ERROR_MINIVERSION_INACCESSIBLE_FROM_SPECIFIED_TRANSACTION: WIN32_ERROR =
    WIN32_ERROR(6810u32);
pub const ERROR_CANT_OPEN_MINIVERSION_WITH_MODIFY_INTENT: WIN32_ERROR = WIN32_ERROR(6811u32);
pub const ERROR_CANT_CREATE_MORE_STREAM_MINIVERSIONS: WIN32_ERROR = WIN32_ERROR(6812u32);
pub const ERROR_REMOTE_FILE_VERSION_MISMATCH: WIN32_ERROR = WIN32_ERROR(6814u32);
pub const ERROR_HANDLE_NO_LONGER_VALID: WIN32_ERROR = WIN32_ERROR(6815u32);
pub const ERROR_NO_TXF_METADATA: WIN32_ERROR = WIN32_ERROR(6816u32);
pub const ERROR_LOG_CORRUPTION_DETECTED: WIN32_ERROR = WIN32_ERROR(6817u32);
pub const ERROR_CANT_RECOVER_WITH_HANDLE_OPEN: WIN32_ERROR = WIN32_ERROR(6818u32);
pub const ERROR_RM_DISCONNECTED: WIN32_ERROR = WIN32_ERROR(6819u32);
pub const ERROR_ENLISTMENT_NOT_SUPERIOR: WIN32_ERROR = WIN32_ERROR(6820u32);
pub const ERROR_RECOVERY_NOT_NEEDED: WIN32_ERROR = WIN32_ERROR(6821u32);
pub const ERROR_RM_ALREADY_STARTED: WIN32_ERROR = WIN32_ERROR(6822u32);
pub const ERROR_FILE_IDENTITY_NOT_PERSISTENT: WIN32_ERROR = WIN32_ERROR(6823u32);
pub const ERROR_CANT_BREAK_TRANSACTIONAL_DEPENDENCY: WIN32_ERROR = WIN32_ERROR(6824u32);
pub const ERROR_CANT_CROSS_RM_BOUNDARY: WIN32_ERROR = WIN32_ERROR(6825u32);
pub const ERROR_TXF_DIR_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(6826u32);
pub const ERROR_INDOUBT_TRANSACTIONS_EXIST: WIN32_ERROR = WIN32_ERROR(6827u32);
pub const ERROR_TM_VOLATILE: WIN32_ERROR = WIN32_ERROR(6828u32);
pub const ERROR_ROLLBACK_TIMER_EXPIRED: WIN32_ERROR = WIN32_ERROR(6829u32);
pub const ERROR_TXF_ATTRIBUTE_CORRUPT: WIN32_ERROR = WIN32_ERROR(6830u32);
pub const ERROR_EFS_NOT_ALLOWED_IN_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6831u32);
pub const ERROR_TRANSACTIONAL_OPEN_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(6832u32);
pub const ERROR_LOG_GROWTH_FAILED: WIN32_ERROR = WIN32_ERROR(6833u32);
pub const ERROR_TRANSACTED_MAPPING_UNSUPPORTED_REMOTE: WIN32_ERROR = WIN32_ERROR(6834u32);
pub const ERROR_TXF_METADATA_ALREADY_PRESENT: WIN32_ERROR = WIN32_ERROR(6835u32);
pub const ERROR_TRANSACTION_SCOPE_CALLBACKS_NOT_SET: WIN32_ERROR = WIN32_ERROR(6836u32);
pub const ERROR_TRANSACTION_REQUIRED_PROMOTION: WIN32_ERROR = WIN32_ERROR(6837u32);
pub const ERROR_CANNOT_EXECUTE_FILE_IN_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6838u32);
pub const ERROR_TRANSACTIONS_NOT_FROZEN: WIN32_ERROR = WIN32_ERROR(6839u32);
pub const ERROR_TRANSACTION_FREEZE_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(6840u32);
pub const ERROR_NOT_SNAPSHOT_VOLUME: WIN32_ERROR = WIN32_ERROR(6841u32);
pub const ERROR_NO_SAVEPOINT_WITH_OPEN_FILES: WIN32_ERROR = WIN32_ERROR(6842u32);
pub const ERROR_DATA_LOST_REPAIR: WIN32_ERROR = WIN32_ERROR(6843u32);
pub const ERROR_SPARSE_NOT_ALLOWED_IN_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6844u32);
pub const ERROR_TM_IDENTITY_MISMATCH: WIN32_ERROR = WIN32_ERROR(6845u32);
pub const ERROR_FLOATED_SECTION: WIN32_ERROR = WIN32_ERROR(6846u32);
pub const ERROR_CANNOT_ACCEPT_TRANSACTED_WORK: WIN32_ERROR = WIN32_ERROR(6847u32);
pub const ERROR_CANNOT_ABORT_TRANSACTIONS: WIN32_ERROR = WIN32_ERROR(6848u32);
pub const ERROR_BAD_CLUSTERS: WIN32_ERROR = WIN32_ERROR(6849u32);
pub const ERROR_COMPRESSION_NOT_ALLOWED_IN_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6850u32);
pub const ERROR_VOLUME_DIRTY: WIN32_ERROR = WIN32_ERROR(6851u32);
pub const ERROR_NO_LINK_TRACKING_IN_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6852u32);
pub const ERROR_OPERATION_NOT_SUPPORTED_IN_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6853u32);
pub const ERROR_EXPIRED_HANDLE: WIN32_ERROR = WIN32_ERROR(6854u32);
pub const ERROR_TRANSACTION_NOT_ENLISTED: WIN32_ERROR = WIN32_ERROR(6855u32);
pub const ERROR_CTX_WINSTATION_NAME_INVALID: WIN32_ERROR = WIN32_ERROR(7001u32);
pub const ERROR_CTX_INVALID_PD: WIN32_ERROR = WIN32_ERROR(7002u32);
pub const ERROR_CTX_PD_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(7003u32);
pub const ERROR_CTX_WD_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(7004u32);
pub const ERROR_CTX_CANNOT_MAKE_EVENTLOG_ENTRY: WIN32_ERROR = WIN32_ERROR(7005u32);
pub const ERROR_CTX_SERVICE_NAME_COLLISION: WIN32_ERROR = WIN32_ERROR(7006u32);
pub const ERROR_CTX_CLOSE_PENDING: WIN32_ERROR = WIN32_ERROR(7007u32);
pub const ERROR_CTX_NO_OUTBUF: WIN32_ERROR = WIN32_ERROR(7008u32);
pub const ERROR_CTX_MODEM_INF_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(7009u32);
pub const ERROR_CTX_INVALID_MODEMNAME: WIN32_ERROR = WIN32_ERROR(7010u32);
pub const ERROR_CTX_MODEM_RESPONSE_ERROR: WIN32_ERROR = WIN32_ERROR(7011u32);
pub const ERROR_CTX_MODEM_RESPONSE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(7012u32);
pub const ERROR_CTX_MODEM_RESPONSE_NO_CARRIER: WIN32_ERROR = WIN32_ERROR(7013u32);
pub const ERROR_CTX_MODEM_RESPONSE_NO_DIALTONE: WIN32_ERROR = WIN32_ERROR(7014u32);
pub const ERROR_CTX_MODEM_RESPONSE_BUSY: WIN32_ERROR = WIN32_ERROR(7015u32);
pub const ERROR_CTX_MODEM_RESPONSE_VOICE: WIN32_ERROR = WIN32_ERROR(7016u32);
pub const ERROR_CTX_TD_ERROR: WIN32_ERROR = WIN32_ERROR(7017u32);
pub const ERROR_CTX_WINSTATION_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(7022u32);
pub const ERROR_CTX_WINSTATION_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(7023u32);
pub const ERROR_CTX_WINSTATION_BUSY: WIN32_ERROR = WIN32_ERROR(7024u32);
pub const ERROR_CTX_BAD_VIDEO_MODE: WIN32_ERROR = WIN32_ERROR(7025u32);
pub const ERROR_CTX_GRAPHICS_INVALID: WIN32_ERROR = WIN32_ERROR(7035u32);
pub const ERROR_CTX_LOGON_DISABLED: WIN32_ERROR = WIN32_ERROR(7037u32);
pub const ERROR_CTX_NOT_CONSOLE: WIN32_ERROR = WIN32_ERROR(7038u32);
pub const ERROR_CTX_CLIENT_QUERY_TIMEOUT: WIN32_ERROR = WIN32_ERROR(7040u32);
pub const ERROR_CTX_CONSOLE_DISCONNECT: WIN32_ERROR = WIN32_ERROR(7041u32);
pub const ERROR_CTX_CONSOLE_CONNECT: WIN32_ERROR = WIN32_ERROR(7042u32);
pub const ERROR_CTX_SHADOW_DENIED: WIN32_ERROR = WIN32_ERROR(7044u32);
pub const ERROR_CTX_WINSTATION_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(7045u32);
pub const ERROR_CTX_INVALID_WD: WIN32_ERROR = WIN32_ERROR(7049u32);
pub const ERROR_CTX_SHADOW_INVALID: WIN32_ERROR = WIN32_ERROR(7050u32);
pub const ERROR_CTX_SHADOW_DISABLED: WIN32_ERROR = WIN32_ERROR(7051u32);
pub const ERROR_CTX_CLIENT_LICENSE_IN_USE: WIN32_ERROR = WIN32_ERROR(7052u32);
pub const ERROR_CTX_CLIENT_LICENSE_NOT_SET: WIN32_ERROR = WIN32_ERROR(7053u32);
pub const ERROR_CTX_LICENSE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(7054u32);
pub const ERROR_CTX_LICENSE_CLIENT_INVALID: WIN32_ERROR = WIN32_ERROR(7055u32);
pub const ERROR_CTX_LICENSE_EXPIRED: WIN32_ERROR = WIN32_ERROR(7056u32);
pub const ERROR_CTX_SHADOW_NOT_RUNNING: WIN32_ERROR = WIN32_ERROR(7057u32);
pub const ERROR_CTX_SHADOW_ENDED_BY_MODE_CHANGE: WIN32_ERROR = WIN32_ERROR(7058u32);
pub const ERROR_ACTIVATION_COUNT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(7059u32);
pub const ERROR_CTX_WINSTATIONS_DISABLED: WIN32_ERROR = WIN32_ERROR(7060u32);
pub const ERROR_CTX_ENCRYPTION_LEVEL_REQUIRED: WIN32_ERROR = WIN32_ERROR(7061u32);
pub const ERROR_CTX_SESSION_IN_USE: WIN32_ERROR = WIN32_ERROR(7062u32);
pub const ERROR_CTX_NO_FORCE_LOGOFF: WIN32_ERROR = WIN32_ERROR(7063u32);
pub const ERROR_CTX_ACCOUNT_RESTRICTION: WIN32_ERROR = WIN32_ERROR(7064u32);
pub const ERROR_RDP_PROTOCOL_ERROR: WIN32_ERROR = WIN32_ERROR(7065u32);
pub const ERROR_CTX_CDM_CONNECT: WIN32_ERROR = WIN32_ERROR(7066u32);
pub const ERROR_CTX_CDM_DISCONNECT: WIN32_ERROR = WIN32_ERROR(7067u32);
pub const ERROR_CTX_SECURITY_LAYER_ERROR: WIN32_ERROR = WIN32_ERROR(7068u32);
pub const ERROR_TS_INCOMPATIBLE_SESSIONS: WIN32_ERROR = WIN32_ERROR(7069u32);
pub const ERROR_TS_VIDEO_SUBSYSTEM_ERROR: WIN32_ERROR = WIN32_ERROR(7070u32);
pub const ERROR_DS_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(8200u32);
pub const ERROR_DS_MEMBERSHIP_EVALUATED_LOCALLY: WIN32_ERROR = WIN32_ERROR(8201u32);
pub const ERROR_DS_NO_ATTRIBUTE_OR_VALUE: WIN32_ERROR = WIN32_ERROR(8202u32);
pub const ERROR_DS_INVALID_ATTRIBUTE_SYNTAX: WIN32_ERROR = WIN32_ERROR(8203u32);
pub const ERROR_DS_ATTRIBUTE_TYPE_UNDEFINED: WIN32_ERROR = WIN32_ERROR(8204u32);
pub const ERROR_DS_ATTRIBUTE_OR_VALUE_EXISTS: WIN32_ERROR = WIN32_ERROR(8205u32);
pub const ERROR_DS_BUSY: WIN32_ERROR = WIN32_ERROR(8206u32);
pub const ERROR_DS_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(8207u32);
pub const ERROR_DS_NO_RIDS_ALLOCATED: WIN32_ERROR = WIN32_ERROR(8208u32);
pub const ERROR_DS_NO_MORE_RIDS: WIN32_ERROR = WIN32_ERROR(8209u32);
pub const ERROR_DS_INCORRECT_ROLE_OWNER: WIN32_ERROR = WIN32_ERROR(8210u32);
pub const ERROR_DS_RIDMGR_INIT_ERROR: WIN32_ERROR = WIN32_ERROR(8211u32);
pub const ERROR_DS_OBJ_CLASS_VIOLATION: WIN32_ERROR = WIN32_ERROR(8212u32);
pub const ERROR_DS_CANT_ON_NON_LEAF: WIN32_ERROR = WIN32_ERROR(8213u32);
pub const ERROR_DS_CANT_ON_RDN: WIN32_ERROR = WIN32_ERROR(8214u32);
pub const ERROR_DS_CANT_MOD_OBJ_CLASS: WIN32_ERROR = WIN32_ERROR(8215u32);
pub const ERROR_DS_CROSS_DOM_MOVE_ERROR: WIN32_ERROR = WIN32_ERROR(8216u32);
pub const ERROR_DS_GC_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(8217u32);
pub const ERROR_SHARED_POLICY: WIN32_ERROR = WIN32_ERROR(8218u32);
pub const ERROR_POLICY_OBJECT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8219u32);
pub const ERROR_POLICY_ONLY_IN_DS: WIN32_ERROR = WIN32_ERROR(8220u32);
pub const ERROR_PROMOTION_ACTIVE: WIN32_ERROR = WIN32_ERROR(8221u32);
pub const ERROR_NO_PROMOTION_ACTIVE: WIN32_ERROR = WIN32_ERROR(8222u32);
pub const ERROR_DS_OPERATIONS_ERROR: WIN32_ERROR = WIN32_ERROR(8224u32);
pub const ERROR_DS_PROTOCOL_ERROR: WIN32_ERROR = WIN32_ERROR(8225u32);
pub const ERROR_DS_TIMELIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8226u32);
pub const ERROR_DS_SIZELIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8227u32);
pub const ERROR_DS_ADMIN_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8228u32);
pub const ERROR_DS_COMPARE_FALSE: WIN32_ERROR = WIN32_ERROR(8229u32);
pub const ERROR_DS_COMPARE_TRUE: WIN32_ERROR = WIN32_ERROR(8230u32);
pub const ERROR_DS_AUTH_METHOD_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(8231u32);
pub const ERROR_DS_STRONG_AUTH_REQUIRED: WIN32_ERROR = WIN32_ERROR(8232u32);
pub const ERROR_DS_INAPPROPRIATE_AUTH: WIN32_ERROR = WIN32_ERROR(8233u32);
pub const ERROR_DS_AUTH_UNKNOWN: WIN32_ERROR = WIN32_ERROR(8234u32);
pub const ERROR_DS_REFERRAL: WIN32_ERROR = WIN32_ERROR(8235u32);
pub const ERROR_DS_UNAVAILABLE_CRIT_EXTENSION: WIN32_ERROR = WIN32_ERROR(8236u32);
pub const ERROR_DS_CONFIDENTIALITY_REQUIRED: WIN32_ERROR = WIN32_ERROR(8237u32);
pub const ERROR_DS_INAPPROPRIATE_MATCHING: WIN32_ERROR = WIN32_ERROR(8238u32);
pub const ERROR_DS_CONSTRAINT_VIOLATION: WIN32_ERROR = WIN32_ERROR(8239u32);
pub const ERROR_DS_NO_SUCH_OBJECT: WIN32_ERROR = WIN32_ERROR(8240u32);
pub const ERROR_DS_ALIAS_PROBLEM: WIN32_ERROR = WIN32_ERROR(8241u32);
pub const ERROR_DS_INVALID_DN_SYNTAX: WIN32_ERROR = WIN32_ERROR(8242u32);
pub const ERROR_DS_IS_LEAF: WIN32_ERROR = WIN32_ERROR(8243u32);
pub const ERROR_DS_ALIAS_DEREF_PROBLEM: WIN32_ERROR = WIN32_ERROR(8244u32);
pub const ERROR_DS_UNWILLING_TO_PERFORM: WIN32_ERROR = WIN32_ERROR(8245u32);
pub const ERROR_DS_LOOP_DETECT: WIN32_ERROR = WIN32_ERROR(8246u32);
pub const ERROR_DS_NAMING_VIOLATION: WIN32_ERROR = WIN32_ERROR(8247u32);
pub const ERROR_DS_OBJECT_RESULTS_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(8248u32);
pub const ERROR_DS_AFFECTS_MULTIPLE_DSAS: WIN32_ERROR = WIN32_ERROR(8249u32);
pub const ERROR_DS_SERVER_DOWN: WIN32_ERROR = WIN32_ERROR(8250u32);
pub const ERROR_DS_LOCAL_ERROR: WIN32_ERROR = WIN32_ERROR(8251u32);
pub const ERROR_DS_ENCODING_ERROR: WIN32_ERROR = WIN32_ERROR(8252u32);
pub const ERROR_DS_DECODING_ERROR: WIN32_ERROR = WIN32_ERROR(8253u32);
pub const ERROR_DS_FILTER_UNKNOWN: WIN32_ERROR = WIN32_ERROR(8254u32);
pub const ERROR_DS_PARAM_ERROR: WIN32_ERROR = WIN32_ERROR(8255u32);
pub const ERROR_DS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(8256u32);
pub const ERROR_DS_NO_RESULTS_RETURNED: WIN32_ERROR = WIN32_ERROR(8257u32);
pub const ERROR_DS_CONTROL_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8258u32);
pub const ERROR_DS_CLIENT_LOOP: WIN32_ERROR = WIN32_ERROR(8259u32);
pub const ERROR_DS_REFERRAL_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8260u32);
pub const ERROR_DS_SORT_CONTROL_MISSING: WIN32_ERROR = WIN32_ERROR(8261u32);
pub const ERROR_DS_OFFSET_RANGE_ERROR: WIN32_ERROR = WIN32_ERROR(8262u32);
pub const ERROR_DS_RIDMGR_DISABLED: WIN32_ERROR = WIN32_ERROR(8263u32);
pub const ERROR_DS_ROOT_MUST_BE_NC: WIN32_ERROR = WIN32_ERROR(8301u32);
pub const ERROR_DS_ADD_REPLICA_INHIBITED: WIN32_ERROR = WIN32_ERROR(8302u32);
pub const ERROR_DS_ATT_NOT_DEF_IN_SCHEMA: WIN32_ERROR = WIN32_ERROR(8303u32);
pub const ERROR_DS_MAX_OBJ_SIZE_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8304u32);
pub const ERROR_DS_OBJ_STRING_NAME_EXISTS: WIN32_ERROR = WIN32_ERROR(8305u32);
pub const ERROR_DS_NO_RDN_DEFINED_IN_SCHEMA: WIN32_ERROR = WIN32_ERROR(8306u32);
pub const ERROR_DS_RDN_DOESNT_MATCH_SCHEMA: WIN32_ERROR = WIN32_ERROR(8307u32);
pub const ERROR_DS_NO_REQUESTED_ATTS_FOUND: WIN32_ERROR = WIN32_ERROR(8308u32);
pub const ERROR_DS_USER_BUFFER_TO_SMALL: WIN32_ERROR = WIN32_ERROR(8309u32);
pub const ERROR_DS_ATT_IS_NOT_ON_OBJ: WIN32_ERROR = WIN32_ERROR(8310u32);
pub const ERROR_DS_ILLEGAL_MOD_OPERATION: WIN32_ERROR = WIN32_ERROR(8311u32);
pub const ERROR_DS_OBJ_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(8312u32);
pub const ERROR_DS_BAD_INSTANCE_TYPE: WIN32_ERROR = WIN32_ERROR(8313u32);
pub const ERROR_DS_MASTERDSA_REQUIRED: WIN32_ERROR = WIN32_ERROR(8314u32);
pub const ERROR_DS_OBJECT_CLASS_REQUIRED: WIN32_ERROR = WIN32_ERROR(8315u32);
pub const ERROR_DS_MISSING_REQUIRED_ATT: WIN32_ERROR = WIN32_ERROR(8316u32);
pub const ERROR_DS_ATT_NOT_DEF_FOR_CLASS: WIN32_ERROR = WIN32_ERROR(8317u32);
pub const ERROR_DS_ATT_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(8318u32);
pub const ERROR_DS_CANT_ADD_ATT_VALUES: WIN32_ERROR = WIN32_ERROR(8320u32);
pub const ERROR_DS_SINGLE_VALUE_CONSTRAINT: WIN32_ERROR = WIN32_ERROR(8321u32);
pub const ERROR_DS_RANGE_CONSTRAINT: WIN32_ERROR = WIN32_ERROR(8322u32);
pub const ERROR_DS_ATT_VAL_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(8323u32);
pub const ERROR_DS_CANT_REM_MISSING_ATT: WIN32_ERROR = WIN32_ERROR(8324u32);
pub const ERROR_DS_CANT_REM_MISSING_ATT_VAL: WIN32_ERROR = WIN32_ERROR(8325u32);
pub const ERROR_DS_ROOT_CANT_BE_SUBREF: WIN32_ERROR = WIN32_ERROR(8326u32);
pub const ERROR_DS_NO_CHAINING: WIN32_ERROR = WIN32_ERROR(8327u32);
pub const ERROR_DS_NO_CHAINED_EVAL: WIN32_ERROR = WIN32_ERROR(8328u32);
pub const ERROR_DS_NO_PARENT_OBJECT: WIN32_ERROR = WIN32_ERROR(8329u32);
pub const ERROR_DS_PARENT_IS_AN_ALIAS: WIN32_ERROR = WIN32_ERROR(8330u32);
pub const ERROR_DS_CANT_MIX_MASTER_AND_REPS: WIN32_ERROR = WIN32_ERROR(8331u32);
pub const ERROR_DS_CHILDREN_EXIST: WIN32_ERROR = WIN32_ERROR(8332u32);
pub const ERROR_DS_OBJ_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8333u32);
pub const ERROR_DS_ALIASED_OBJ_MISSING: WIN32_ERROR = WIN32_ERROR(8334u32);
pub const ERROR_DS_BAD_NAME_SYNTAX: WIN32_ERROR = WIN32_ERROR(8335u32);
pub const ERROR_DS_ALIAS_POINTS_TO_ALIAS: WIN32_ERROR = WIN32_ERROR(8336u32);
pub const ERROR_DS_CANT_DEREF_ALIAS: WIN32_ERROR = WIN32_ERROR(8337u32);
pub const ERROR_DS_OUT_OF_SCOPE: WIN32_ERROR = WIN32_ERROR(8338u32);
pub const ERROR_DS_OBJECT_BEING_REMOVED: WIN32_ERROR = WIN32_ERROR(8339u32);
pub const ERROR_DS_CANT_DELETE_DSA_OBJ: WIN32_ERROR = WIN32_ERROR(8340u32);
pub const ERROR_DS_GENERIC_ERROR: WIN32_ERROR = WIN32_ERROR(8341u32);
pub const ERROR_DS_DSA_MUST_BE_INT_MASTER: WIN32_ERROR = WIN32_ERROR(8342u32);
pub const ERROR_DS_CLASS_NOT_DSA: WIN32_ERROR = WIN32_ERROR(8343u32);
pub const ERROR_DS_INSUFF_ACCESS_RIGHTS: WIN32_ERROR = WIN32_ERROR(8344u32);
pub const ERROR_DS_ILLEGAL_SUPERIOR: WIN32_ERROR = WIN32_ERROR(8345u32);
pub const ERROR_DS_ATTRIBUTE_OWNED_BY_SAM: WIN32_ERROR = WIN32_ERROR(8346u32);
pub const ERROR_DS_NAME_TOO_MANY_PARTS: WIN32_ERROR = WIN32_ERROR(8347u32);
pub const ERROR_DS_NAME_TOO_LONG: WIN32_ERROR = WIN32_ERROR(8348u32);
pub const ERROR_DS_NAME_VALUE_TOO_LONG: WIN32_ERROR = WIN32_ERROR(8349u32);
pub const ERROR_DS_NAME_UNPARSEABLE: WIN32_ERROR = WIN32_ERROR(8350u32);
pub const ERROR_DS_NAME_TYPE_UNKNOWN: WIN32_ERROR = WIN32_ERROR(8351u32);
pub const ERROR_DS_NOT_AN_OBJECT: WIN32_ERROR = WIN32_ERROR(8352u32);
pub const ERROR_DS_SEC_DESC_TOO_SHORT: WIN32_ERROR = WIN32_ERROR(8353u32);
pub const ERROR_DS_SEC_DESC_INVALID: WIN32_ERROR = WIN32_ERROR(8354u32);
pub const ERROR_DS_NO_DELETED_NAME: WIN32_ERROR = WIN32_ERROR(8355u32);
pub const ERROR_DS_SUBREF_MUST_HAVE_PARENT: WIN32_ERROR = WIN32_ERROR(8356u32);
pub const ERROR_DS_NCNAME_MUST_BE_NC: WIN32_ERROR = WIN32_ERROR(8357u32);
pub const ERROR_DS_CANT_ADD_SYSTEM_ONLY: WIN32_ERROR = WIN32_ERROR(8358u32);
pub const ERROR_DS_CLASS_MUST_BE_CONCRETE: WIN32_ERROR = WIN32_ERROR(8359u32);
pub const ERROR_DS_INVALID_DMD: WIN32_ERROR = WIN32_ERROR(8360u32);
pub const ERROR_DS_OBJ_GUID_EXISTS: WIN32_ERROR = WIN32_ERROR(8361u32);
pub const ERROR_DS_NOT_ON_BACKLINK: WIN32_ERROR = WIN32_ERROR(8362u32);
pub const ERROR_DS_NO_CROSSREF_FOR_NC: WIN32_ERROR = WIN32_ERROR(8363u32);
pub const ERROR_DS_SHUTTING_DOWN: WIN32_ERROR = WIN32_ERROR(8364u32);
pub const ERROR_DS_UNKNOWN_OPERATION: WIN32_ERROR = WIN32_ERROR(8365u32);
pub const ERROR_DS_INVALID_ROLE_OWNER: WIN32_ERROR = WIN32_ERROR(8366u32);
pub const ERROR_DS_COULDNT_CONTACT_FSMO: WIN32_ERROR = WIN32_ERROR(8367u32);
pub const ERROR_DS_CROSS_NC_DN_RENAME: WIN32_ERROR = WIN32_ERROR(8368u32);
pub const ERROR_DS_CANT_MOD_SYSTEM_ONLY: WIN32_ERROR = WIN32_ERROR(8369u32);
pub const ERROR_DS_REPLICATOR_ONLY: WIN32_ERROR = WIN32_ERROR(8370u32);
pub const ERROR_DS_OBJ_CLASS_NOT_DEFINED: WIN32_ERROR = WIN32_ERROR(8371u32);
pub const ERROR_DS_OBJ_CLASS_NOT_SUBCLASS: WIN32_ERROR = WIN32_ERROR(8372u32);
pub const ERROR_DS_NAME_REFERENCE_INVALID: WIN32_ERROR = WIN32_ERROR(8373u32);
pub const ERROR_DS_CROSS_REF_EXISTS: WIN32_ERROR = WIN32_ERROR(8374u32);
pub const ERROR_DS_CANT_DEL_MASTER_CROSSREF: WIN32_ERROR = WIN32_ERROR(8375u32);
pub const ERROR_DS_SUBTREE_NOTIFY_NOT_NC_HEAD: WIN32_ERROR = WIN32_ERROR(8376u32);
pub const ERROR_DS_NOTIFY_FILTER_TOO_COMPLEX: WIN32_ERROR = WIN32_ERROR(8377u32);
pub const ERROR_DS_DUP_RDN: WIN32_ERROR = WIN32_ERROR(8378u32);
pub const ERROR_DS_DUP_OID: WIN32_ERROR = WIN32_ERROR(8379u32);
pub const ERROR_DS_DUP_MAPI_ID: WIN32_ERROR = WIN32_ERROR(8380u32);
pub const ERROR_DS_DUP_SCHEMA_ID_GUID: WIN32_ERROR = WIN32_ERROR(8381u32);
pub const ERROR_DS_DUP_LDAP_DISPLAY_NAME: WIN32_ERROR = WIN32_ERROR(8382u32);
pub const ERROR_DS_SEMANTIC_ATT_TEST: WIN32_ERROR = WIN32_ERROR(8383u32);
pub const ERROR_DS_SYNTAX_MISMATCH: WIN32_ERROR = WIN32_ERROR(8384u32);
pub const ERROR_DS_EXISTS_IN_MUST_HAVE: WIN32_ERROR = WIN32_ERROR(8385u32);
pub const ERROR_DS_EXISTS_IN_MAY_HAVE: WIN32_ERROR = WIN32_ERROR(8386u32);
pub const ERROR_DS_NONEXISTENT_MAY_HAVE: WIN32_ERROR = WIN32_ERROR(8387u32);
pub const ERROR_DS_NONEXISTENT_MUST_HAVE: WIN32_ERROR = WIN32_ERROR(8388u32);
pub const ERROR_DS_AUX_CLS_TEST_FAIL: WIN32_ERROR = WIN32_ERROR(8389u32);
pub const ERROR_DS_NONEXISTENT_POSS_SUP: WIN32_ERROR = WIN32_ERROR(8390u32);
pub const ERROR_DS_SUB_CLS_TEST_FAIL: WIN32_ERROR = WIN32_ERROR(8391u32);
pub const ERROR_DS_BAD_RDN_ATT_ID_SYNTAX: WIN32_ERROR = WIN32_ERROR(8392u32);
pub const ERROR_DS_EXISTS_IN_AUX_CLS: WIN32_ERROR = WIN32_ERROR(8393u32);
pub const ERROR_DS_EXISTS_IN_SUB_CLS: WIN32_ERROR = WIN32_ERROR(8394u32);
pub const ERROR_DS_EXISTS_IN_POSS_SUP: WIN32_ERROR = WIN32_ERROR(8395u32);
pub const ERROR_DS_RECALCSCHEMA_FAILED: WIN32_ERROR = WIN32_ERROR(8396u32);
pub const ERROR_DS_TREE_DELETE_NOT_FINISHED: WIN32_ERROR = WIN32_ERROR(8397u32);
pub const ERROR_DS_CANT_DELETE: WIN32_ERROR = WIN32_ERROR(8398u32);
pub const ERROR_DS_ATT_SCHEMA_REQ_ID: WIN32_ERROR = WIN32_ERROR(8399u32);
pub const ERROR_DS_BAD_ATT_SCHEMA_SYNTAX: WIN32_ERROR = WIN32_ERROR(8400u32);
pub const ERROR_DS_CANT_CACHE_ATT: WIN32_ERROR = WIN32_ERROR(8401u32);
pub const ERROR_DS_CANT_CACHE_CLASS: WIN32_ERROR = WIN32_ERROR(8402u32);
pub const ERROR_DS_CANT_REMOVE_ATT_CACHE: WIN32_ERROR = WIN32_ERROR(8403u32);
pub const ERROR_DS_CANT_REMOVE_CLASS_CACHE: WIN32_ERROR = WIN32_ERROR(8404u32);
pub const ERROR_DS_CANT_RETRIEVE_DN: WIN32_ERROR = WIN32_ERROR(8405u32);
pub const ERROR_DS_MISSING_SUPREF: WIN32_ERROR = WIN32_ERROR(8406u32);
pub const ERROR_DS_CANT_RETRIEVE_INSTANCE: WIN32_ERROR = WIN32_ERROR(8407u32);
pub const ERROR_DS_CODE_INCONSISTENCY: WIN32_ERROR = WIN32_ERROR(8408u32);
pub const ERROR_DS_DATABASE_ERROR: WIN32_ERROR = WIN32_ERROR(8409u32);
pub const ERROR_DS_GOVERNSID_MISSING: WIN32_ERROR = WIN32_ERROR(8410u32);
pub const ERROR_DS_MISSING_EXPECTED_ATT: WIN32_ERROR = WIN32_ERROR(8411u32);
pub const ERROR_DS_NCNAME_MISSING_CR_REF: WIN32_ERROR = WIN32_ERROR(8412u32);
pub const ERROR_DS_SECURITY_CHECKING_ERROR: WIN32_ERROR = WIN32_ERROR(8413u32);
pub const ERROR_DS_SCHEMA_NOT_LOADED: WIN32_ERROR = WIN32_ERROR(8414u32);
pub const ERROR_DS_SCHEMA_ALLOC_FAILED: WIN32_ERROR = WIN32_ERROR(8415u32);
pub const ERROR_DS_ATT_SCHEMA_REQ_SYNTAX: WIN32_ERROR = WIN32_ERROR(8416u32);
pub const ERROR_DS_GCVERIFY_ERROR: WIN32_ERROR = WIN32_ERROR(8417u32);
pub const ERROR_DS_DRA_SCHEMA_MISMATCH: WIN32_ERROR = WIN32_ERROR(8418u32);
pub const ERROR_DS_CANT_FIND_DSA_OBJ: WIN32_ERROR = WIN32_ERROR(8419u32);
pub const ERROR_DS_CANT_FIND_EXPECTED_NC: WIN32_ERROR = WIN32_ERROR(8420u32);
pub const ERROR_DS_CANT_FIND_NC_IN_CACHE: WIN32_ERROR = WIN32_ERROR(8421u32);
pub const ERROR_DS_CANT_RETRIEVE_CHILD: WIN32_ERROR = WIN32_ERROR(8422u32);
pub const ERROR_DS_SECURITY_ILLEGAL_MODIFY: WIN32_ERROR = WIN32_ERROR(8423u32);
pub const ERROR_DS_CANT_REPLACE_HIDDEN_REC: WIN32_ERROR = WIN32_ERROR(8424u32);
pub const ERROR_DS_BAD_HIERARCHY_FILE: WIN32_ERROR = WIN32_ERROR(8425u32);
pub const ERROR_DS_BUILD_HIERARCHY_TABLE_FAILED: WIN32_ERROR = WIN32_ERROR(8426u32);
pub const ERROR_DS_CONFIG_PARAM_MISSING: WIN32_ERROR = WIN32_ERROR(8427u32);
pub const ERROR_DS_COUNTING_AB_INDICES_FAILED: WIN32_ERROR = WIN32_ERROR(8428u32);
pub const ERROR_DS_HIERARCHY_TABLE_MALLOC_FAILED: WIN32_ERROR = WIN32_ERROR(8429u32);
pub const ERROR_DS_INTERNAL_FAILURE: WIN32_ERROR = WIN32_ERROR(8430u32);
pub const ERROR_DS_UNKNOWN_ERROR: WIN32_ERROR = WIN32_ERROR(8431u32);
pub const ERROR_DS_ROOT_REQUIRES_CLASS_TOP: WIN32_ERROR = WIN32_ERROR(8432u32);
pub const ERROR_DS_REFUSING_FSMO_ROLES: WIN32_ERROR = WIN32_ERROR(8433u32);
pub const ERROR_DS_MISSING_FSMO_SETTINGS: WIN32_ERROR = WIN32_ERROR(8434u32);
pub const ERROR_DS_UNABLE_TO_SURRENDER_ROLES: WIN32_ERROR = WIN32_ERROR(8435u32);
pub const ERROR_DS_DRA_GENERIC: WIN32_ERROR = WIN32_ERROR(8436u32);
pub const ERROR_DS_DRA_INVALID_PARAMETER: WIN32_ERROR = WIN32_ERROR(8437u32);
pub const ERROR_DS_DRA_BUSY: WIN32_ERROR = WIN32_ERROR(8438u32);
pub const ERROR_DS_DRA_BAD_DN: WIN32_ERROR = WIN32_ERROR(8439u32);
pub const ERROR_DS_DRA_BAD_NC: WIN32_ERROR = WIN32_ERROR(8440u32);
pub const ERROR_DS_DRA_DN_EXISTS: WIN32_ERROR = WIN32_ERROR(8441u32);
pub const ERROR_DS_DRA_INTERNAL_ERROR: WIN32_ERROR = WIN32_ERROR(8442u32);
pub const ERROR_DS_DRA_INCONSISTENT_DIT: WIN32_ERROR = WIN32_ERROR(8443u32);
pub const ERROR_DS_DRA_CONNECTION_FAILED: WIN32_ERROR = WIN32_ERROR(8444u32);
pub const ERROR_DS_DRA_BAD_INSTANCE_TYPE: WIN32_ERROR = WIN32_ERROR(8445u32);
pub const ERROR_DS_DRA_OUT_OF_MEM: WIN32_ERROR = WIN32_ERROR(8446u32);
pub const ERROR_DS_DRA_MAIL_PROBLEM: WIN32_ERROR = WIN32_ERROR(8447u32);
pub const ERROR_DS_DRA_REF_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(8448u32);
pub const ERROR_DS_DRA_REF_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8449u32);
pub const ERROR_DS_DRA_OBJ_IS_REP_SOURCE: WIN32_ERROR = WIN32_ERROR(8450u32);
pub const ERROR_DS_DRA_DB_ERROR: WIN32_ERROR = WIN32_ERROR(8451u32);
pub const ERROR_DS_DRA_NO_REPLICA: WIN32_ERROR = WIN32_ERROR(8452u32);
pub const ERROR_DS_DRA_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(8453u32);
pub const ERROR_DS_DRA_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(8454u32);
pub const ERROR_DS_DRA_RPC_CANCELLED: WIN32_ERROR = WIN32_ERROR(8455u32);
pub const ERROR_DS_DRA_SOURCE_DISABLED: WIN32_ERROR = WIN32_ERROR(8456u32);
pub const ERROR_DS_DRA_SINK_DISABLED: WIN32_ERROR = WIN32_ERROR(8457u32);
pub const ERROR_DS_DRA_NAME_COLLISION: WIN32_ERROR = WIN32_ERROR(8458u32);
pub const ERROR_DS_DRA_SOURCE_REINSTALLED: WIN32_ERROR = WIN32_ERROR(8459u32);
pub const ERROR_DS_DRA_MISSING_PARENT: WIN32_ERROR = WIN32_ERROR(8460u32);
pub const ERROR_DS_DRA_PREEMPTED: WIN32_ERROR = WIN32_ERROR(8461u32);
pub const ERROR_DS_DRA_ABANDON_SYNC: WIN32_ERROR = WIN32_ERROR(8462u32);
pub const ERROR_DS_DRA_SHUTDOWN: WIN32_ERROR = WIN32_ERROR(8463u32);
pub const ERROR_DS_DRA_INCOMPATIBLE_PARTIAL_SET: WIN32_ERROR = WIN32_ERROR(8464u32);
pub const ERROR_DS_DRA_SOURCE_IS_PARTIAL_REPLICA: WIN32_ERROR = WIN32_ERROR(8465u32);
pub const ERROR_DS_DRA_EXTN_CONNECTION_FAILED: WIN32_ERROR = WIN32_ERROR(8466u32);
pub const ERROR_DS_INSTALL_SCHEMA_MISMATCH: WIN32_ERROR = WIN32_ERROR(8467u32);
pub const ERROR_DS_DUP_LINK_ID: WIN32_ERROR = WIN32_ERROR(8468u32);
pub const ERROR_DS_NAME_ERROR_RESOLVING: WIN32_ERROR = WIN32_ERROR(8469u32);
pub const ERROR_DS_NAME_ERROR_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8470u32);
pub const ERROR_DS_NAME_ERROR_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(8471u32);
pub const ERROR_DS_NAME_ERROR_NO_MAPPING: WIN32_ERROR = WIN32_ERROR(8472u32);
pub const ERROR_DS_NAME_ERROR_DOMAIN_ONLY: WIN32_ERROR = WIN32_ERROR(8473u32);
pub const ERROR_DS_NAME_ERROR_NO_SYNTACTICAL_MAPPING: WIN32_ERROR = WIN32_ERROR(8474u32);
pub const ERROR_DS_CONSTRUCTED_ATT_MOD: WIN32_ERROR = WIN32_ERROR(8475u32);
pub const ERROR_DS_WRONG_OM_OBJ_CLASS: WIN32_ERROR = WIN32_ERROR(8476u32);
pub const ERROR_DS_DRA_REPL_PENDING: WIN32_ERROR = WIN32_ERROR(8477u32);
pub const ERROR_DS_DS_REQUIRED: WIN32_ERROR = WIN32_ERROR(8478u32);
pub const ERROR_DS_INVALID_LDAP_DISPLAY_NAME: WIN32_ERROR = WIN32_ERROR(8479u32);
pub const ERROR_DS_NON_BASE_SEARCH: WIN32_ERROR = WIN32_ERROR(8480u32);
pub const ERROR_DS_CANT_RETRIEVE_ATTS: WIN32_ERROR = WIN32_ERROR(8481u32);
pub const ERROR_DS_BACKLINK_WITHOUT_LINK: WIN32_ERROR = WIN32_ERROR(8482u32);
pub const ERROR_DS_EPOCH_MISMATCH: WIN32_ERROR = WIN32_ERROR(8483u32);
pub const ERROR_DS_SRC_NAME_MISMATCH: WIN32_ERROR = WIN32_ERROR(8484u32);
pub const ERROR_DS_SRC_AND_DST_NC_IDENTICAL: WIN32_ERROR = WIN32_ERROR(8485u32);
pub const ERROR_DS_DST_NC_MISMATCH: WIN32_ERROR = WIN32_ERROR(8486u32);
pub const ERROR_DS_NOT_AUTHORITIVE_FOR_DST_NC: WIN32_ERROR = WIN32_ERROR(8487u32);
pub const ERROR_DS_SRC_GUID_MISMATCH: WIN32_ERROR = WIN32_ERROR(8488u32);
pub const ERROR_DS_CANT_MOVE_DELETED_OBJECT: WIN32_ERROR = WIN32_ERROR(8489u32);
pub const ERROR_DS_PDC_OPERATION_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(8490u32);
pub const ERROR_DS_CROSS_DOMAIN_CLEANUP_REQD: WIN32_ERROR = WIN32_ERROR(8491u32);
pub const ERROR_DS_ILLEGAL_XDOM_MOVE_OPERATION: WIN32_ERROR = WIN32_ERROR(8492u32);
pub const ERROR_DS_CANT_WITH_ACCT_GROUP_MEMBERSHPS: WIN32_ERROR = WIN32_ERROR(8493u32);
pub const ERROR_DS_NC_MUST_HAVE_NC_PARENT: WIN32_ERROR = WIN32_ERROR(8494u32);
pub const ERROR_DS_CR_IMPOSSIBLE_TO_VALIDATE: WIN32_ERROR = WIN32_ERROR(8495u32);
pub const ERROR_DS_DST_DOMAIN_NOT_NATIVE: WIN32_ERROR = WIN32_ERROR(8496u32);
pub const ERROR_DS_MISSING_INFRASTRUCTURE_CONTAINER: WIN32_ERROR = WIN32_ERROR(8497u32);
pub const ERROR_DS_CANT_MOVE_ACCOUNT_GROUP: WIN32_ERROR = WIN32_ERROR(8498u32);
pub const ERROR_DS_CANT_MOVE_RESOURCE_GROUP: WIN32_ERROR = WIN32_ERROR(8499u32);
pub const ERROR_DS_INVALID_SEARCH_FLAG: WIN32_ERROR = WIN32_ERROR(8500u32);
pub const ERROR_DS_NO_TREE_DELETE_ABOVE_NC: WIN32_ERROR = WIN32_ERROR(8501u32);
pub const ERROR_DS_COULDNT_LOCK_TREE_FOR_DELETE: WIN32_ERROR = WIN32_ERROR(8502u32);
pub const ERROR_DS_COULDNT_IDENTIFY_OBJECTS_FOR_TREE_DELETE: WIN32_ERROR = WIN32_ERROR(8503u32);
pub const ERROR_DS_SAM_INIT_FAILURE: WIN32_ERROR = WIN32_ERROR(8504u32);
pub const ERROR_DS_SENSITIVE_GROUP_VIOLATION: WIN32_ERROR = WIN32_ERROR(8505u32);
pub const ERROR_DS_CANT_MOD_PRIMARYGROUPID: WIN32_ERROR = WIN32_ERROR(8506u32);
pub const ERROR_DS_ILLEGAL_BASE_SCHEMA_MOD: WIN32_ERROR = WIN32_ERROR(8507u32);
pub const ERROR_DS_NONSAFE_SCHEMA_CHANGE: WIN32_ERROR = WIN32_ERROR(8508u32);
pub const ERROR_DS_SCHEMA_UPDATE_DISALLOWED: WIN32_ERROR = WIN32_ERROR(8509u32);
pub const ERROR_DS_CANT_CREATE_UNDER_SCHEMA: WIN32_ERROR = WIN32_ERROR(8510u32);
pub const ERROR_DS_INSTALL_NO_SRC_SCH_VERSION: WIN32_ERROR = WIN32_ERROR(8511u32);
pub const ERROR_DS_INSTALL_NO_SCH_VERSION_IN_INIFILE: WIN32_ERROR = WIN32_ERROR(8512u32);
pub const ERROR_DS_INVALID_GROUP_TYPE: WIN32_ERROR = WIN32_ERROR(8513u32);
pub const ERROR_DS_NO_NEST_GLOBALGROUP_IN_MIXEDDOMAIN: WIN32_ERROR = WIN32_ERROR(8514u32);
pub const ERROR_DS_NO_NEST_LOCALGROUP_IN_MIXEDDOMAIN: WIN32_ERROR = WIN32_ERROR(8515u32);
pub const ERROR_DS_GLOBAL_CANT_HAVE_LOCAL_MEMBER: WIN32_ERROR = WIN32_ERROR(8516u32);
pub const ERROR_DS_GLOBAL_CANT_HAVE_UNIVERSAL_MEMBER: WIN32_ERROR = WIN32_ERROR(8517u32);
pub const ERROR_DS_UNIVERSAL_CANT_HAVE_LOCAL_MEMBER: WIN32_ERROR = WIN32_ERROR(8518u32);
pub const ERROR_DS_GLOBAL_CANT_HAVE_CROSSDOMAIN_MEMBER: WIN32_ERROR = WIN32_ERROR(8519u32);
pub const ERROR_DS_LOCAL_CANT_HAVE_CROSSDOMAIN_LOCAL_MEMBER: WIN32_ERROR = WIN32_ERROR(8520u32);
pub const ERROR_DS_HAVE_PRIMARY_MEMBERS: WIN32_ERROR = WIN32_ERROR(8521u32);
pub const ERROR_DS_STRING_SD_CONVERSION_FAILED: WIN32_ERROR = WIN32_ERROR(8522u32);
pub const ERROR_DS_NAMING_MASTER_GC: WIN32_ERROR = WIN32_ERROR(8523u32);
pub const ERROR_DS_DNS_LOOKUP_FAILURE: WIN32_ERROR = WIN32_ERROR(8524u32);
pub const ERROR_DS_COULDNT_UPDATE_SPNS: WIN32_ERROR = WIN32_ERROR(8525u32);
pub const ERROR_DS_CANT_RETRIEVE_SD: WIN32_ERROR = WIN32_ERROR(8526u32);
pub const ERROR_DS_KEY_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(8527u32);
pub const ERROR_DS_WRONG_LINKED_ATT_SYNTAX: WIN32_ERROR = WIN32_ERROR(8528u32);
pub const ERROR_DS_SAM_NEED_BOOTKEY_PASSWORD: WIN32_ERROR = WIN32_ERROR(8529u32);
pub const ERROR_DS_SAM_NEED_BOOTKEY_FLOPPY: WIN32_ERROR = WIN32_ERROR(8530u32);
pub const ERROR_DS_CANT_START: WIN32_ERROR = WIN32_ERROR(8531u32);
pub const ERROR_DS_INIT_FAILURE: WIN32_ERROR = WIN32_ERROR(8532u32);
pub const ERROR_DS_NO_PKT_PRIVACY_ON_CONNECTION: WIN32_ERROR = WIN32_ERROR(8533u32);
pub const ERROR_DS_SOURCE_DOMAIN_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8534u32);
pub const ERROR_DS_DESTINATION_DOMAIN_NOT_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8535u32);
pub const ERROR_DS_DESTINATION_AUDITING_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(8536u32);
pub const ERROR_DS_CANT_FIND_DC_FOR_SRC_DOMAIN: WIN32_ERROR = WIN32_ERROR(8537u32);
pub const ERROR_DS_SRC_OBJ_NOT_GROUP_OR_USER: WIN32_ERROR = WIN32_ERROR(8538u32);
pub const ERROR_DS_SRC_SID_EXISTS_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8539u32);
pub const ERROR_DS_SRC_AND_DST_OBJECT_CLASS_MISMATCH: WIN32_ERROR = WIN32_ERROR(8540u32);
pub const ERROR_SAM_INIT_FAILURE: WIN32_ERROR = WIN32_ERROR(8541u32);
pub const ERROR_DS_DRA_SCHEMA_INFO_SHIP: WIN32_ERROR = WIN32_ERROR(8542u32);
pub const ERROR_DS_DRA_SCHEMA_CONFLICT: WIN32_ERROR = WIN32_ERROR(8543u32);
pub const ERROR_DS_DRA_EARLIER_SCHEMA_CONFLICT: WIN32_ERROR = WIN32_ERROR(8544u32);
pub const ERROR_DS_DRA_OBJ_NC_MISMATCH: WIN32_ERROR = WIN32_ERROR(8545u32);
pub const ERROR_DS_NC_STILL_HAS_DSAS: WIN32_ERROR = WIN32_ERROR(8546u32);
pub const ERROR_DS_GC_REQUIRED: WIN32_ERROR = WIN32_ERROR(8547u32);
pub const ERROR_DS_LOCAL_MEMBER_OF_LOCAL_ONLY: WIN32_ERROR = WIN32_ERROR(8548u32);
pub const ERROR_DS_NO_FPO_IN_UNIVERSAL_GROUPS: WIN32_ERROR = WIN32_ERROR(8549u32);
pub const ERROR_DS_CANT_ADD_TO_GC: WIN32_ERROR = WIN32_ERROR(8550u32);
pub const ERROR_DS_NO_CHECKPOINT_WITH_PDC: WIN32_ERROR = WIN32_ERROR(8551u32);
pub const ERROR_DS_SOURCE_AUDITING_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(8552u32);
pub const ERROR_DS_CANT_CREATE_IN_NONDOMAIN_NC: WIN32_ERROR = WIN32_ERROR(8553u32);
pub const ERROR_DS_INVALID_NAME_FOR_SPN: WIN32_ERROR = WIN32_ERROR(8554u32);
pub const ERROR_DS_FILTER_USES_CONTRUCTED_ATTRS: WIN32_ERROR = WIN32_ERROR(8555u32);
pub const ERROR_DS_UNICODEPWD_NOT_IN_QUOTES: WIN32_ERROR = WIN32_ERROR(8556u32);
pub const ERROR_DS_MACHINE_ACCOUNT_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8557u32);
pub const ERROR_DS_MUST_BE_RUN_ON_DST_DC: WIN32_ERROR = WIN32_ERROR(8558u32);
pub const ERROR_DS_SRC_DC_MUST_BE_SP4_OR_GREATER: WIN32_ERROR = WIN32_ERROR(8559u32);
pub const ERROR_DS_CANT_TREE_DELETE_CRITICAL_OBJ: WIN32_ERROR = WIN32_ERROR(8560u32);
pub const ERROR_DS_INIT_FAILURE_CONSOLE: WIN32_ERROR = WIN32_ERROR(8561u32);
pub const ERROR_DS_SAM_INIT_FAILURE_CONSOLE: WIN32_ERROR = WIN32_ERROR(8562u32);
pub const ERROR_DS_FOREST_VERSION_TOO_HIGH: WIN32_ERROR = WIN32_ERROR(8563u32);
pub const ERROR_DS_DOMAIN_VERSION_TOO_HIGH: WIN32_ERROR = WIN32_ERROR(8564u32);
pub const ERROR_DS_FOREST_VERSION_TOO_LOW: WIN32_ERROR = WIN32_ERROR(8565u32);
pub const ERROR_DS_DOMAIN_VERSION_TOO_LOW: WIN32_ERROR = WIN32_ERROR(8566u32);
pub const ERROR_DS_INCOMPATIBLE_VERSION: WIN32_ERROR = WIN32_ERROR(8567u32);
pub const ERROR_DS_LOW_DSA_VERSION: WIN32_ERROR = WIN32_ERROR(8568u32);
pub const ERROR_DS_NO_BEHAVIOR_VERSION_IN_MIXEDDOMAIN: WIN32_ERROR = WIN32_ERROR(8569u32);
pub const ERROR_DS_NOT_SUPPORTED_SORT_ORDER: WIN32_ERROR = WIN32_ERROR(8570u32);
pub const ERROR_DS_NAME_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(8571u32);
pub const ERROR_DS_MACHINE_ACCOUNT_CREATED_PRENT4: WIN32_ERROR = WIN32_ERROR(8572u32);
pub const ERROR_DS_OUT_OF_VERSION_STORE: WIN32_ERROR = WIN32_ERROR(8573u32);
pub const ERROR_DS_INCOMPATIBLE_CONTROLS_USED: WIN32_ERROR = WIN32_ERROR(8574u32);
pub const ERROR_DS_NO_REF_DOMAIN: WIN32_ERROR = WIN32_ERROR(8575u32);
pub const ERROR_DS_RESERVED_LINK_ID: WIN32_ERROR = WIN32_ERROR(8576u32);
pub const ERROR_DS_LINK_ID_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(8577u32);
pub const ERROR_DS_AG_CANT_HAVE_UNIVERSAL_MEMBER: WIN32_ERROR = WIN32_ERROR(8578u32);
pub const ERROR_DS_MODIFYDN_DISALLOWED_BY_INSTANCE_TYPE: WIN32_ERROR = WIN32_ERROR(8579u32);
pub const ERROR_DS_NO_OBJECT_MOVE_IN_SCHEMA_NC: WIN32_ERROR = WIN32_ERROR(8580u32);
pub const ERROR_DS_MODIFYDN_DISALLOWED_BY_FLAG: WIN32_ERROR = WIN32_ERROR(8581u32);
pub const ERROR_DS_MODIFYDN_WRONG_GRANDPARENT: WIN32_ERROR = WIN32_ERROR(8582u32);
pub const ERROR_DS_NAME_ERROR_TRUST_REFERRAL: WIN32_ERROR = WIN32_ERROR(8583u32);
pub const ERROR_NOT_SUPPORTED_ON_STANDARD_SERVER: WIN32_ERROR = WIN32_ERROR(8584u32);
pub const ERROR_DS_CANT_ACCESS_REMOTE_PART_OF_AD: WIN32_ERROR = WIN32_ERROR(8585u32);
pub const ERROR_DS_CR_IMPOSSIBLE_TO_VALIDATE_V2: WIN32_ERROR = WIN32_ERROR(8586u32);
pub const ERROR_DS_THREAD_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8587u32);
pub const ERROR_DS_NOT_CLOSEST: WIN32_ERROR = WIN32_ERROR(8588u32);
pub const ERROR_DS_CANT_DERIVE_SPN_WITHOUT_SERVER_REF: WIN32_ERROR = WIN32_ERROR(8589u32);
pub const ERROR_DS_SINGLE_USER_MODE_FAILED: WIN32_ERROR = WIN32_ERROR(8590u32);
pub const ERROR_DS_NTDSCRIPT_SYNTAX_ERROR: WIN32_ERROR = WIN32_ERROR(8591u32);
pub const ERROR_DS_NTDSCRIPT_PROCESS_ERROR: WIN32_ERROR = WIN32_ERROR(8592u32);
pub const ERROR_DS_DIFFERENT_REPL_EPOCHS: WIN32_ERROR = WIN32_ERROR(8593u32);
pub const ERROR_DS_DRS_EXTENSIONS_CHANGED: WIN32_ERROR = WIN32_ERROR(8594u32);
pub const ERROR_DS_REPLICA_SET_CHANGE_NOT_ALLOWED_ON_DISABLED_CR: WIN32_ERROR =
    WIN32_ERROR(8595u32);
pub const ERROR_DS_NO_MSDS_INTID: WIN32_ERROR = WIN32_ERROR(8596u32);
pub const ERROR_DS_DUP_MSDS_INTID: WIN32_ERROR = WIN32_ERROR(8597u32);
pub const ERROR_DS_EXISTS_IN_RDNATTID: WIN32_ERROR = WIN32_ERROR(8598u32);
pub const ERROR_DS_AUTHORIZATION_FAILED: WIN32_ERROR = WIN32_ERROR(8599u32);
pub const ERROR_DS_INVALID_SCRIPT: WIN32_ERROR = WIN32_ERROR(8600u32);
pub const ERROR_DS_REMOTE_CROSSREF_OP_FAILED: WIN32_ERROR = WIN32_ERROR(8601u32);
pub const ERROR_DS_CROSS_REF_BUSY: WIN32_ERROR = WIN32_ERROR(8602u32);
pub const ERROR_DS_CANT_DERIVE_SPN_FOR_DELETED_DOMAIN: WIN32_ERROR = WIN32_ERROR(8603u32);
pub const ERROR_DS_CANT_DEMOTE_WITH_WRITEABLE_NC: WIN32_ERROR = WIN32_ERROR(8604u32);
pub const ERROR_DS_DUPLICATE_ID_FOUND: WIN32_ERROR = WIN32_ERROR(8605u32);
pub const ERROR_DS_INSUFFICIENT_ATTR_TO_CREATE_OBJECT: WIN32_ERROR = WIN32_ERROR(8606u32);
pub const ERROR_DS_GROUP_CONVERSION_ERROR: WIN32_ERROR = WIN32_ERROR(8607u32);
pub const ERROR_DS_CANT_MOVE_APP_BASIC_GROUP: WIN32_ERROR = WIN32_ERROR(8608u32);
pub const ERROR_DS_CANT_MOVE_APP_QUERY_GROUP: WIN32_ERROR = WIN32_ERROR(8609u32);
pub const ERROR_DS_ROLE_NOT_VERIFIED: WIN32_ERROR = WIN32_ERROR(8610u32);
pub const ERROR_DS_WKO_CONTAINER_CANNOT_BE_SPECIAL: WIN32_ERROR = WIN32_ERROR(8611u32);
pub const ERROR_DS_DOMAIN_RENAME_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(8612u32);
pub const ERROR_DS_EXISTING_AD_CHILD_NC: WIN32_ERROR = WIN32_ERROR(8613u32);
pub const ERROR_DS_REPL_LIFETIME_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8614u32);
pub const ERROR_DS_DISALLOWED_IN_SYSTEM_CONTAINER: WIN32_ERROR = WIN32_ERROR(8615u32);
pub const ERROR_DS_LDAP_SEND_QUEUE_FULL: WIN32_ERROR = WIN32_ERROR(8616u32);
pub const ERROR_DS_DRA_OUT_SCHEDULE_WINDOW: WIN32_ERROR = WIN32_ERROR(8617u32);
pub const ERROR_DS_POLICY_NOT_KNOWN: WIN32_ERROR = WIN32_ERROR(8618u32);
pub const ERROR_NO_SITE_SETTINGS_OBJECT: WIN32_ERROR = WIN32_ERROR(8619u32);
pub const ERROR_NO_SECRETS: WIN32_ERROR = WIN32_ERROR(8620u32);
pub const ERROR_NO_WRITABLE_DC_FOUND: WIN32_ERROR = WIN32_ERROR(8621u32);
pub const ERROR_DS_NO_SERVER_OBJECT: WIN32_ERROR = WIN32_ERROR(8622u32);
pub const ERROR_DS_NO_NTDSA_OBJECT: WIN32_ERROR = WIN32_ERROR(8623u32);
pub const ERROR_DS_NON_ASQ_SEARCH: WIN32_ERROR = WIN32_ERROR(8624u32);
pub const ERROR_DS_AUDIT_FAILURE: WIN32_ERROR = WIN32_ERROR(8625u32);
pub const ERROR_DS_INVALID_SEARCH_FLAG_SUBTREE: WIN32_ERROR = WIN32_ERROR(8626u32);
pub const ERROR_DS_INVALID_SEARCH_FLAG_TUPLE: WIN32_ERROR = WIN32_ERROR(8627u32);
pub const ERROR_DS_HIERARCHY_TABLE_TOO_DEEP: WIN32_ERROR = WIN32_ERROR(8628u32);
pub const ERROR_DS_DRA_CORRUPT_UTD_VECTOR: WIN32_ERROR = WIN32_ERROR(8629u32);
pub const ERROR_DS_DRA_SECRETS_DENIED: WIN32_ERROR = WIN32_ERROR(8630u32);
pub const ERROR_DS_RESERVED_MAPI_ID: WIN32_ERROR = WIN32_ERROR(8631u32);
pub const ERROR_DS_MAPI_ID_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(8632u32);
pub const ERROR_DS_DRA_MISSING_KRBTGT_SECRET: WIN32_ERROR = WIN32_ERROR(8633u32);
pub const ERROR_DS_DOMAIN_NAME_EXISTS_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8634u32);
pub const ERROR_DS_FLAT_NAME_EXISTS_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8635u32);
pub const ERROR_INVALID_USER_PRINCIPAL_NAME: WIN32_ERROR = WIN32_ERROR(8636u32);
pub const ERROR_DS_OID_MAPPED_GROUP_CANT_HAVE_MEMBERS: WIN32_ERROR = WIN32_ERROR(8637u32);
pub const ERROR_DS_OID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8638u32);
pub const ERROR_DS_DRA_RECYCLED_TARGET: WIN32_ERROR = WIN32_ERROR(8639u32);
pub const ERROR_DS_DISALLOWED_NC_REDIRECT: WIN32_ERROR = WIN32_ERROR(8640u32);
pub const ERROR_DS_HIGH_ADLDS_FFL: WIN32_ERROR = WIN32_ERROR(8641u32);
pub const ERROR_DS_HIGH_DSA_VERSION: WIN32_ERROR = WIN32_ERROR(8642u32);
pub const ERROR_DS_LOW_ADLDS_FFL: WIN32_ERROR = WIN32_ERROR(8643u32);
pub const ERROR_DOMAIN_SID_SAME_AS_LOCAL_WORKSTATION: WIN32_ERROR = WIN32_ERROR(8644u32);
pub const ERROR_DS_UNDELETE_SAM_VALIDATION_FAILED: WIN32_ERROR = WIN32_ERROR(8645u32);
pub const ERROR_INCORRECT_ACCOUNT_TYPE: WIN32_ERROR = WIN32_ERROR(8646u32);
pub const ERROR_DS_SPN_VALUE_NOT_UNIQUE_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8647u32);
pub const ERROR_DS_UPN_VALUE_NOT_UNIQUE_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8648u32);
pub const ERROR_DS_MISSING_FOREST_TRUST: WIN32_ERROR = WIN32_ERROR(8649u32);
pub const ERROR_DS_VALUE_KEY_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(8650u32);
pub const ERROR_WEAK_WHFBKEY_BLOCKED: WIN32_ERROR = WIN32_ERROR(8651u32);
pub const DNS_ERROR_RESPONSE_CODES_BASE: WIN32_ERROR = WIN32_ERROR(9000u32);
pub const DNS_ERROR_RCODE_NO_ERROR: WIN32_ERROR = WIN32_ERROR(0u32);
pub const DNS_ERROR_MASK: WIN32_ERROR = WIN32_ERROR(9000u32);
pub const DNS_ERROR_RCODE_FORMAT_ERROR: WIN32_ERROR = WIN32_ERROR(9001u32);
pub const DNS_ERROR_RCODE_SERVER_FAILURE: WIN32_ERROR = WIN32_ERROR(9002u32);
pub const DNS_ERROR_RCODE_NAME_ERROR: WIN32_ERROR = WIN32_ERROR(9003u32);
pub const DNS_ERROR_RCODE_NOT_IMPLEMENTED: WIN32_ERROR = WIN32_ERROR(9004u32);
pub const DNS_ERROR_RCODE_REFUSED: WIN32_ERROR = WIN32_ERROR(9005u32);
pub const DNS_ERROR_RCODE_YXDOMAIN: WIN32_ERROR = WIN32_ERROR(9006u32);
pub const DNS_ERROR_RCODE_YXRRSET: WIN32_ERROR = WIN32_ERROR(9007u32);
pub const DNS_ERROR_RCODE_NXRRSET: WIN32_ERROR = WIN32_ERROR(9008u32);
pub const DNS_ERROR_RCODE_NOTAUTH: WIN32_ERROR = WIN32_ERROR(9009u32);
pub const DNS_ERROR_RCODE_NOTZONE: WIN32_ERROR = WIN32_ERROR(9010u32);
pub const DNS_ERROR_RCODE_BADSIG: WIN32_ERROR = WIN32_ERROR(9016u32);
pub const DNS_ERROR_RCODE_BADKEY: WIN32_ERROR = WIN32_ERROR(9017u32);
pub const DNS_ERROR_RCODE_BADTIME: WIN32_ERROR = WIN32_ERROR(9018u32);
pub const DNS_ERROR_RCODE_LAST: WIN32_ERROR = WIN32_ERROR(9018u32);
pub const DNS_ERROR_DNSSEC_BASE: WIN32_ERROR = WIN32_ERROR(9100u32);
pub const DNS_ERROR_KEYMASTER_REQUIRED: WIN32_ERROR = WIN32_ERROR(9101u32);
pub const DNS_ERROR_NOT_ALLOWED_ON_SIGNED_ZONE: WIN32_ERROR = WIN32_ERROR(9102u32);
pub const DNS_ERROR_NSEC3_INCOMPATIBLE_WITH_RSA_SHA1: WIN32_ERROR = WIN32_ERROR(9103u32);
pub const DNS_ERROR_NOT_ENOUGH_SIGNING_KEY_DESCRIPTORS: WIN32_ERROR = WIN32_ERROR(9104u32);
pub const DNS_ERROR_UNSUPPORTED_ALGORITHM: WIN32_ERROR = WIN32_ERROR(9105u32);
pub const DNS_ERROR_INVALID_KEY_SIZE: WIN32_ERROR = WIN32_ERROR(9106u32);
pub const DNS_ERROR_SIGNING_KEY_NOT_ACCESSIBLE: WIN32_ERROR = WIN32_ERROR(9107u32);
pub const DNS_ERROR_KSP_DOES_NOT_SUPPORT_PROTECTION: WIN32_ERROR = WIN32_ERROR(9108u32);
pub const DNS_ERROR_UNEXPECTED_DATA_PROTECTION_ERROR: WIN32_ERROR = WIN32_ERROR(9109u32);
pub const DNS_ERROR_UNEXPECTED_CNG_ERROR: WIN32_ERROR = WIN32_ERROR(9110u32);
pub const DNS_ERROR_UNKNOWN_SIGNING_PARAMETER_VERSION: WIN32_ERROR = WIN32_ERROR(9111u32);
pub const DNS_ERROR_KSP_NOT_ACCESSIBLE: WIN32_ERROR = WIN32_ERROR(9112u32);
pub const DNS_ERROR_TOO_MANY_SKDS: WIN32_ERROR = WIN32_ERROR(9113u32);
pub const DNS_ERROR_INVALID_ROLLOVER_PERIOD: WIN32_ERROR = WIN32_ERROR(9114u32);
pub const DNS_ERROR_INVALID_INITIAL_ROLLOVER_OFFSET: WIN32_ERROR = WIN32_ERROR(9115u32);
pub const DNS_ERROR_ROLLOVER_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(9116u32);
pub const DNS_ERROR_STANDBY_KEY_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(9117u32);
pub const DNS_ERROR_NOT_ALLOWED_ON_ZSK: WIN32_ERROR = WIN32_ERROR(9118u32);
pub const DNS_ERROR_NOT_ALLOWED_ON_ACTIVE_SKD: WIN32_ERROR = WIN32_ERROR(9119u32);
pub const DNS_ERROR_ROLLOVER_ALREADY_QUEUED: WIN32_ERROR = WIN32_ERROR(9120u32);
pub const DNS_ERROR_NOT_ALLOWED_ON_UNSIGNED_ZONE: WIN32_ERROR = WIN32_ERROR(9121u32);
pub const DNS_ERROR_BAD_KEYMASTER: WIN32_ERROR = WIN32_ERROR(9122u32);
pub const DNS_ERROR_INVALID_SIGNATURE_VALIDITY_PERIOD: WIN32_ERROR = WIN32_ERROR(9123u32);
pub const DNS_ERROR_INVALID_NSEC3_ITERATION_COUNT: WIN32_ERROR = WIN32_ERROR(9124u32);
pub const DNS_ERROR_DNSSEC_IS_DISABLED: WIN32_ERROR = WIN32_ERROR(9125u32);
pub const DNS_ERROR_INVALID_XML: WIN32_ERROR = WIN32_ERROR(9126u32);
pub const DNS_ERROR_NO_VALID_TRUST_ANCHORS: WIN32_ERROR = WIN32_ERROR(9127u32);
pub const DNS_ERROR_ROLLOVER_NOT_POKEABLE: WIN32_ERROR = WIN32_ERROR(9128u32);
pub const DNS_ERROR_NSEC3_NAME_COLLISION: WIN32_ERROR = WIN32_ERROR(9129u32);
pub const DNS_ERROR_NSEC_INCOMPATIBLE_WITH_NSEC3_RSA_SHA1: WIN32_ERROR = WIN32_ERROR(9130u32);
pub const DNS_ERROR_PACKET_FMT_BASE: WIN32_ERROR = WIN32_ERROR(9500u32);
pub const DNS_ERROR_BAD_PACKET: WIN32_ERROR = WIN32_ERROR(9502u32);
pub const DNS_ERROR_NO_PACKET: WIN32_ERROR = WIN32_ERROR(9503u32);
pub const DNS_ERROR_RCODE: WIN32_ERROR = WIN32_ERROR(9504u32);
pub const DNS_ERROR_UNSECURE_PACKET: WIN32_ERROR = WIN32_ERROR(9505u32);
pub const DNS_ERROR_NO_MEMORY: WIN32_ERROR = WIN32_ERROR(14u32);
pub const DNS_ERROR_INVALID_NAME: WIN32_ERROR = WIN32_ERROR(123u32);
pub const DNS_ERROR_INVALID_DATA: WIN32_ERROR = WIN32_ERROR(13u32);
pub const DNS_ERROR_GENERAL_API_BASE: WIN32_ERROR = WIN32_ERROR(9550u32);
pub const DNS_ERROR_INVALID_TYPE: WIN32_ERROR = WIN32_ERROR(9551u32);
pub const DNS_ERROR_INVALID_IP_ADDRESS: WIN32_ERROR = WIN32_ERROR(9552u32);
pub const DNS_ERROR_INVALID_PROPERTY: WIN32_ERROR = WIN32_ERROR(9553u32);
pub const DNS_ERROR_TRY_AGAIN_LATER: WIN32_ERROR = WIN32_ERROR(9554u32);
pub const DNS_ERROR_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(9555u32);
pub const DNS_ERROR_NON_RFC_NAME: WIN32_ERROR = WIN32_ERROR(9556u32);
pub const DNS_ERROR_INVALID_NAME_CHAR: WIN32_ERROR = WIN32_ERROR(9560u32);
pub const DNS_ERROR_NUMERIC_NAME: WIN32_ERROR = WIN32_ERROR(9561u32);
pub const DNS_ERROR_NOT_ALLOWED_ON_ROOT_SERVER: WIN32_ERROR = WIN32_ERROR(9562u32);
pub const DNS_ERROR_NOT_ALLOWED_UNDER_DELEGATION: WIN32_ERROR = WIN32_ERROR(9563u32);
pub const DNS_ERROR_CANNOT_FIND_ROOT_HINTS: WIN32_ERROR = WIN32_ERROR(9564u32);
pub const DNS_ERROR_INCONSISTENT_ROOT_HINTS: WIN32_ERROR = WIN32_ERROR(9565u32);
pub const DNS_ERROR_DWORD_VALUE_TOO_SMALL: WIN32_ERROR = WIN32_ERROR(9566u32);
pub const DNS_ERROR_DWORD_VALUE_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(9567u32);
pub const DNS_ERROR_BACKGROUND_LOADING: WIN32_ERROR = WIN32_ERROR(9568u32);
pub const DNS_ERROR_NOT_ALLOWED_ON_RODC: WIN32_ERROR = WIN32_ERROR(9569u32);
pub const DNS_ERROR_NOT_ALLOWED_UNDER_DNAME: WIN32_ERROR = WIN32_ERROR(9570u32);
pub const DNS_ERROR_DELEGATION_REQUIRED: WIN32_ERROR = WIN32_ERROR(9571u32);
pub const DNS_ERROR_INVALID_POLICY_TABLE: WIN32_ERROR = WIN32_ERROR(9572u32);
pub const DNS_ERROR_ADDRESS_REQUIRED: WIN32_ERROR = WIN32_ERROR(9573u32);
pub const DNS_ERROR_ZONE_BASE: WIN32_ERROR = WIN32_ERROR(9600u32);
pub const DNS_ERROR_ZONE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9601u32);
pub const DNS_ERROR_NO_ZONE_INFO: WIN32_ERROR = WIN32_ERROR(9602u32);
pub const DNS_ERROR_INVALID_ZONE_OPERATION: WIN32_ERROR = WIN32_ERROR(9603u32);
pub const DNS_ERROR_ZONE_CONFIGURATION_ERROR: WIN32_ERROR = WIN32_ERROR(9604u32);
pub const DNS_ERROR_ZONE_HAS_NO_SOA_RECORD: WIN32_ERROR = WIN32_ERROR(9605u32);
pub const DNS_ERROR_ZONE_HAS_NO_NS_RECORDS: WIN32_ERROR = WIN32_ERROR(9606u32);
pub const DNS_ERROR_ZONE_LOCKED: WIN32_ERROR = WIN32_ERROR(9607u32);
pub const DNS_ERROR_ZONE_CREATION_FAILED: WIN32_ERROR = WIN32_ERROR(9608u32);
pub const DNS_ERROR_ZONE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9609u32);
pub const DNS_ERROR_AUTOZONE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9610u32);
pub const DNS_ERROR_INVALID_ZONE_TYPE: WIN32_ERROR = WIN32_ERROR(9611u32);
pub const DNS_ERROR_SECONDARY_REQUIRES_MASTER_IP: WIN32_ERROR = WIN32_ERROR(9612u32);
pub const DNS_ERROR_ZONE_NOT_SECONDARY: WIN32_ERROR = WIN32_ERROR(9613u32);
pub const DNS_ERROR_NEED_SECONDARY_ADDRESSES: WIN32_ERROR = WIN32_ERROR(9614u32);
pub const DNS_ERROR_WINS_INIT_FAILED: WIN32_ERROR = WIN32_ERROR(9615u32);
pub const DNS_ERROR_NEED_WINS_SERVERS: WIN32_ERROR = WIN32_ERROR(9616u32);
pub const DNS_ERROR_NBSTAT_INIT_FAILED: WIN32_ERROR = WIN32_ERROR(9617u32);
pub const DNS_ERROR_SOA_DELETE_INVALID: WIN32_ERROR = WIN32_ERROR(9618u32);
pub const DNS_ERROR_FORWARDER_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9619u32);
pub const DNS_ERROR_ZONE_REQUIRES_MASTER_IP: WIN32_ERROR = WIN32_ERROR(9620u32);
pub const DNS_ERROR_ZONE_IS_SHUTDOWN: WIN32_ERROR = WIN32_ERROR(9621u32);
pub const DNS_ERROR_ZONE_LOCKED_FOR_SIGNING: WIN32_ERROR = WIN32_ERROR(9622u32);
pub const DNS_ERROR_DATAFILE_BASE: WIN32_ERROR = WIN32_ERROR(9650u32);
pub const DNS_ERROR_PRIMARY_REQUIRES_DATAFILE: WIN32_ERROR = WIN32_ERROR(9651u32);
pub const DNS_ERROR_INVALID_DATAFILE_NAME: WIN32_ERROR = WIN32_ERROR(9652u32);
pub const DNS_ERROR_DATAFILE_OPEN_FAILURE: WIN32_ERROR = WIN32_ERROR(9653u32);
pub const DNS_ERROR_FILE_WRITEBACK_FAILED: WIN32_ERROR = WIN32_ERROR(9654u32);
pub const DNS_ERROR_DATAFILE_PARSING: WIN32_ERROR = WIN32_ERROR(9655u32);
pub const DNS_ERROR_DATABASE_BASE: WIN32_ERROR = WIN32_ERROR(9700u32);
pub const DNS_ERROR_RECORD_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9701u32);
pub const DNS_ERROR_RECORD_FORMAT: WIN32_ERROR = WIN32_ERROR(9702u32);
pub const DNS_ERROR_NODE_CREATION_FAILED: WIN32_ERROR = WIN32_ERROR(9703u32);
pub const DNS_ERROR_UNKNOWN_RECORD_TYPE: WIN32_ERROR = WIN32_ERROR(9704u32);
pub const DNS_ERROR_RECORD_TIMED_OUT: WIN32_ERROR = WIN32_ERROR(9705u32);
pub const DNS_ERROR_NAME_NOT_IN_ZONE: WIN32_ERROR = WIN32_ERROR(9706u32);
pub const DNS_ERROR_CNAME_LOOP: WIN32_ERROR = WIN32_ERROR(9707u32);
pub const DNS_ERROR_NODE_IS_CNAME: WIN32_ERROR = WIN32_ERROR(9708u32);
pub const DNS_ERROR_CNAME_COLLISION: WIN32_ERROR = WIN32_ERROR(9709u32);
pub const DNS_ERROR_RECORD_ONLY_AT_ZONE_ROOT: WIN32_ERROR = WIN32_ERROR(9710u32);
pub const DNS_ERROR_RECORD_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9711u32);
pub const DNS_ERROR_SECONDARY_DATA: WIN32_ERROR = WIN32_ERROR(9712u32);
pub const DNS_ERROR_NO_CREATE_CACHE_DATA: WIN32_ERROR = WIN32_ERROR(9713u32);
pub const DNS_ERROR_NAME_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9714u32);
pub const DNS_ERROR_DS_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(9717u32);
pub const DNS_ERROR_DS_ZONE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9718u32);
pub const DNS_ERROR_NO_BOOTFILE_IF_DS_ZONE: WIN32_ERROR = WIN32_ERROR(9719u32);
pub const DNS_ERROR_NODE_IS_DNAME: WIN32_ERROR = WIN32_ERROR(9720u32);
pub const DNS_ERROR_DNAME_COLLISION: WIN32_ERROR = WIN32_ERROR(9721u32);
pub const DNS_ERROR_ALIAS_LOOP: WIN32_ERROR = WIN32_ERROR(9722u32);
pub const DNS_ERROR_OPERATION_BASE: WIN32_ERROR = WIN32_ERROR(9750u32);
pub const DNS_ERROR_AXFR: WIN32_ERROR = WIN32_ERROR(9752u32);
pub const DNS_ERROR_SECURE_BASE: WIN32_ERROR = WIN32_ERROR(9800u32);
pub const DNS_ERROR_SETUP_BASE: WIN32_ERROR = WIN32_ERROR(9850u32);
pub const DNS_ERROR_NO_TCPIP: WIN32_ERROR = WIN32_ERROR(9851u32);
pub const DNS_ERROR_NO_DNS_SERVERS: WIN32_ERROR = WIN32_ERROR(9852u32);
pub const DNS_ERROR_DP_BASE: WIN32_ERROR = WIN32_ERROR(9900u32);
pub const DNS_ERROR_DP_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9901u32);
pub const DNS_ERROR_DP_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9902u32);
pub const DNS_ERROR_DP_NOT_ENLISTED: WIN32_ERROR = WIN32_ERROR(9903u32);
pub const DNS_ERROR_DP_ALREADY_ENLISTED: WIN32_ERROR = WIN32_ERROR(9904u32);
pub const DNS_ERROR_DP_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(9905u32);
pub const DNS_ERROR_DP_FSMO_ERROR: WIN32_ERROR = WIN32_ERROR(9906u32);
pub const DNS_ERROR_RRL_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(9911u32);
pub const DNS_ERROR_RRL_INVALID_WINDOW_SIZE: WIN32_ERROR = WIN32_ERROR(9912u32);
pub const DNS_ERROR_RRL_INVALID_IPV4_PREFIX: WIN32_ERROR = WIN32_ERROR(9913u32);
pub const DNS_ERROR_RRL_INVALID_IPV6_PREFIX: WIN32_ERROR = WIN32_ERROR(9914u32);
pub const DNS_ERROR_RRL_INVALID_TC_RATE: WIN32_ERROR = WIN32_ERROR(9915u32);
pub const DNS_ERROR_RRL_INVALID_LEAK_RATE: WIN32_ERROR = WIN32_ERROR(9916u32);
pub const DNS_ERROR_RRL_LEAK_RATE_LESSTHAN_TC_RATE: WIN32_ERROR = WIN32_ERROR(9917u32);
pub const DNS_ERROR_VIRTUALIZATION_INSTANCE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9921u32);
pub const DNS_ERROR_VIRTUALIZATION_INSTANCE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9922u32);
pub const DNS_ERROR_VIRTUALIZATION_TREE_LOCKED: WIN32_ERROR = WIN32_ERROR(9923u32);
pub const DNS_ERROR_INVAILD_VIRTUALIZATION_INSTANCE_NAME: WIN32_ERROR = WIN32_ERROR(9924u32);
pub const DNS_ERROR_DEFAULT_VIRTUALIZATION_INSTANCE: WIN32_ERROR = WIN32_ERROR(9925u32);
pub const DNS_ERROR_ZONESCOPE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9951u32);
pub const DNS_ERROR_ZONESCOPE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9952u32);
pub const DNS_ERROR_DEFAULT_ZONESCOPE: WIN32_ERROR = WIN32_ERROR(9953u32);
pub const DNS_ERROR_INVALID_ZONESCOPE_NAME: WIN32_ERROR = WIN32_ERROR(9954u32);
pub const DNS_ERROR_NOT_ALLOWED_WITH_ZONESCOPES: WIN32_ERROR = WIN32_ERROR(9955u32);
pub const DNS_ERROR_LOAD_ZONESCOPE_FAILED: WIN32_ERROR = WIN32_ERROR(9956u32);
pub const DNS_ERROR_ZONESCOPE_FILE_WRITEBACK_FAILED: WIN32_ERROR = WIN32_ERROR(9957u32);
pub const DNS_ERROR_INVALID_SCOPE_NAME: WIN32_ERROR = WIN32_ERROR(9958u32);
pub const DNS_ERROR_SCOPE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9959u32);
pub const DNS_ERROR_DEFAULT_SCOPE: WIN32_ERROR = WIN32_ERROR(9960u32);
pub const DNS_ERROR_INVALID_SCOPE_OPERATION: WIN32_ERROR = WIN32_ERROR(9961u32);
pub const DNS_ERROR_SCOPE_LOCKED: WIN32_ERROR = WIN32_ERROR(9962u32);
pub const DNS_ERROR_SCOPE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9963u32);
pub const DNS_ERROR_POLICY_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9971u32);
pub const DNS_ERROR_POLICY_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9972u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA: WIN32_ERROR = WIN32_ERROR(9973u32);
pub const DNS_ERROR_POLICY_INVALID_SETTINGS: WIN32_ERROR = WIN32_ERROR(9974u32);
pub const DNS_ERROR_CLIENT_SUBNET_IS_ACCESSED: WIN32_ERROR = WIN32_ERROR(9975u32);
pub const DNS_ERROR_CLIENT_SUBNET_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9976u32);
pub const DNS_ERROR_CLIENT_SUBNET_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9977u32);
pub const DNS_ERROR_SUBNET_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9978u32);
pub const DNS_ERROR_SUBNET_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9979u32);
pub const DNS_ERROR_POLICY_LOCKED: WIN32_ERROR = WIN32_ERROR(9980u32);
pub const DNS_ERROR_POLICY_INVALID_WEIGHT: WIN32_ERROR = WIN32_ERROR(9981u32);
pub const DNS_ERROR_POLICY_INVALID_NAME: WIN32_ERROR = WIN32_ERROR(9982u32);
pub const DNS_ERROR_POLICY_MISSING_CRITERIA: WIN32_ERROR = WIN32_ERROR(9983u32);
pub const DNS_ERROR_INVALID_CLIENT_SUBNET_NAME: WIN32_ERROR = WIN32_ERROR(9984u32);
pub const DNS_ERROR_POLICY_PROCESSING_ORDER_INVALID: WIN32_ERROR = WIN32_ERROR(9985u32);
pub const DNS_ERROR_POLICY_SCOPE_MISSING: WIN32_ERROR = WIN32_ERROR(9986u32);
pub const DNS_ERROR_POLICY_SCOPE_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(9987u32);
pub const DNS_ERROR_SERVERSCOPE_IS_REFERENCED: WIN32_ERROR = WIN32_ERROR(9988u32);
pub const DNS_ERROR_ZONESCOPE_IS_REFERENCED: WIN32_ERROR = WIN32_ERROR(9989u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_CLIENT_SUBNET: WIN32_ERROR = WIN32_ERROR(9990u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_TRANSPORT_PROTOCOL: WIN32_ERROR = WIN32_ERROR(9991u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_NETWORK_PROTOCOL: WIN32_ERROR = WIN32_ERROR(9992u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_INTERFACE: WIN32_ERROR = WIN32_ERROR(9993u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_FQDN: WIN32_ERROR = WIN32_ERROR(9994u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_QUERY_TYPE: WIN32_ERROR = WIN32_ERROR(9995u32);
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_TIME_OF_DAY: WIN32_ERROR = WIN32_ERROR(9996u32);
pub const ERROR_IPSEC_QM_POLICY_EXISTS: WIN32_ERROR = WIN32_ERROR(13000u32);
pub const ERROR_IPSEC_QM_POLICY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13001u32);
pub const ERROR_IPSEC_QM_POLICY_IN_USE: WIN32_ERROR = WIN32_ERROR(13002u32);
pub const ERROR_IPSEC_MM_POLICY_EXISTS: WIN32_ERROR = WIN32_ERROR(13003u32);
pub const ERROR_IPSEC_MM_POLICY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13004u32);
pub const ERROR_IPSEC_MM_POLICY_IN_USE: WIN32_ERROR = WIN32_ERROR(13005u32);
pub const ERROR_IPSEC_MM_FILTER_EXISTS: WIN32_ERROR = WIN32_ERROR(13006u32);
pub const ERROR_IPSEC_MM_FILTER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13007u32);
pub const ERROR_IPSEC_TRANSPORT_FILTER_EXISTS: WIN32_ERROR = WIN32_ERROR(13008u32);
pub const ERROR_IPSEC_TRANSPORT_FILTER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13009u32);
pub const ERROR_IPSEC_MM_AUTH_EXISTS: WIN32_ERROR = WIN32_ERROR(13010u32);
pub const ERROR_IPSEC_MM_AUTH_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13011u32);
pub const ERROR_IPSEC_MM_AUTH_IN_USE: WIN32_ERROR = WIN32_ERROR(13012u32);
pub const ERROR_IPSEC_DEFAULT_MM_POLICY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13013u32);
pub const ERROR_IPSEC_DEFAULT_MM_AUTH_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13014u32);
pub const ERROR_IPSEC_DEFAULT_QM_POLICY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13015u32);
pub const ERROR_IPSEC_TUNNEL_FILTER_EXISTS: WIN32_ERROR = WIN32_ERROR(13016u32);
pub const ERROR_IPSEC_TUNNEL_FILTER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13017u32);
pub const ERROR_IPSEC_MM_FILTER_PENDING_DELETION: WIN32_ERROR = WIN32_ERROR(13018u32);
pub const ERROR_IPSEC_TRANSPORT_FILTER_PENDING_DELETION: WIN32_ERROR = WIN32_ERROR(13019u32);
pub const ERROR_IPSEC_TUNNEL_FILTER_PENDING_DELETION: WIN32_ERROR = WIN32_ERROR(13020u32);
pub const ERROR_IPSEC_MM_POLICY_PENDING_DELETION: WIN32_ERROR = WIN32_ERROR(13021u32);
pub const ERROR_IPSEC_MM_AUTH_PENDING_DELETION: WIN32_ERROR = WIN32_ERROR(13022u32);
pub const ERROR_IPSEC_QM_POLICY_PENDING_DELETION: WIN32_ERROR = WIN32_ERROR(13023u32);
pub const ERROR_IPSEC_IKE_NEG_STATUS_BEGIN: WIN32_ERROR = WIN32_ERROR(13800u32);
pub const ERROR_IPSEC_IKE_AUTH_FAIL: WIN32_ERROR = WIN32_ERROR(13801u32);
pub const ERROR_IPSEC_IKE_ATTRIB_FAIL: WIN32_ERROR = WIN32_ERROR(13802u32);
pub const ERROR_IPSEC_IKE_NEGOTIATION_PENDING: WIN32_ERROR = WIN32_ERROR(13803u32);
pub const ERROR_IPSEC_IKE_GENERAL_PROCESSING_ERROR: WIN32_ERROR = WIN32_ERROR(13804u32);
pub const ERROR_IPSEC_IKE_TIMED_OUT: WIN32_ERROR = WIN32_ERROR(13805u32);
pub const ERROR_IPSEC_IKE_NO_CERT: WIN32_ERROR = WIN32_ERROR(13806u32);
pub const ERROR_IPSEC_IKE_SA_DELETED: WIN32_ERROR = WIN32_ERROR(13807u32);
pub const ERROR_IPSEC_IKE_SA_REAPED: WIN32_ERROR = WIN32_ERROR(13808u32);
pub const ERROR_IPSEC_IKE_MM_ACQUIRE_DROP: WIN32_ERROR = WIN32_ERROR(13809u32);
pub const ERROR_IPSEC_IKE_QM_ACQUIRE_DROP: WIN32_ERROR = WIN32_ERROR(13810u32);
pub const ERROR_IPSEC_IKE_QUEUE_DROP_MM: WIN32_ERROR = WIN32_ERROR(13811u32);
pub const ERROR_IPSEC_IKE_QUEUE_DROP_NO_MM: WIN32_ERROR = WIN32_ERROR(13812u32);
pub const ERROR_IPSEC_IKE_DROP_NO_RESPONSE: WIN32_ERROR = WIN32_ERROR(13813u32);
pub const ERROR_IPSEC_IKE_MM_DELAY_DROP: WIN32_ERROR = WIN32_ERROR(13814u32);
pub const ERROR_IPSEC_IKE_QM_DELAY_DROP: WIN32_ERROR = WIN32_ERROR(13815u32);
pub const ERROR_IPSEC_IKE_ERROR: WIN32_ERROR = WIN32_ERROR(13816u32);
pub const ERROR_IPSEC_IKE_CRL_FAILED: WIN32_ERROR = WIN32_ERROR(13817u32);
pub const ERROR_IPSEC_IKE_INVALID_KEY_USAGE: WIN32_ERROR = WIN32_ERROR(13818u32);
pub const ERROR_IPSEC_IKE_INVALID_CERT_TYPE: WIN32_ERROR = WIN32_ERROR(13819u32);
pub const ERROR_IPSEC_IKE_NO_PRIVATE_KEY: WIN32_ERROR = WIN32_ERROR(13820u32);
pub const ERROR_IPSEC_IKE_SIMULTANEOUS_REKEY: WIN32_ERROR = WIN32_ERROR(13821u32);
pub const ERROR_IPSEC_IKE_DH_FAIL: WIN32_ERROR = WIN32_ERROR(13822u32);
pub const ERROR_IPSEC_IKE_CRITICAL_PAYLOAD_NOT_RECOGNIZED: WIN32_ERROR = WIN32_ERROR(13823u32);
pub const ERROR_IPSEC_IKE_INVALID_HEADER: WIN32_ERROR = WIN32_ERROR(13824u32);
pub const ERROR_IPSEC_IKE_NO_POLICY: WIN32_ERROR = WIN32_ERROR(13825u32);
pub const ERROR_IPSEC_IKE_INVALID_SIGNATURE: WIN32_ERROR = WIN32_ERROR(13826u32);
pub const ERROR_IPSEC_IKE_KERBEROS_ERROR: WIN32_ERROR = WIN32_ERROR(13827u32);
pub const ERROR_IPSEC_IKE_NO_PUBLIC_KEY: WIN32_ERROR = WIN32_ERROR(13828u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR: WIN32_ERROR = WIN32_ERROR(13829u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_SA: WIN32_ERROR = WIN32_ERROR(13830u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_PROP: WIN32_ERROR = WIN32_ERROR(13831u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_TRANS: WIN32_ERROR = WIN32_ERROR(13832u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_KE: WIN32_ERROR = WIN32_ERROR(13833u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_ID: WIN32_ERROR = WIN32_ERROR(13834u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_CERT: WIN32_ERROR = WIN32_ERROR(13835u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_CERT_REQ: WIN32_ERROR = WIN32_ERROR(13836u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_HASH: WIN32_ERROR = WIN32_ERROR(13837u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_SIG: WIN32_ERROR = WIN32_ERROR(13838u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_NONCE: WIN32_ERROR = WIN32_ERROR(13839u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_NOTIFY: WIN32_ERROR = WIN32_ERROR(13840u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_DELETE: WIN32_ERROR = WIN32_ERROR(13841u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_VENDOR: WIN32_ERROR = WIN32_ERROR(13842u32);
pub const ERROR_IPSEC_IKE_INVALID_PAYLOAD: WIN32_ERROR = WIN32_ERROR(13843u32);
pub const ERROR_IPSEC_IKE_LOAD_SOFT_SA: WIN32_ERROR = WIN32_ERROR(13844u32);
pub const ERROR_IPSEC_IKE_SOFT_SA_TORN_DOWN: WIN32_ERROR = WIN32_ERROR(13845u32);
pub const ERROR_IPSEC_IKE_INVALID_COOKIE: WIN32_ERROR = WIN32_ERROR(13846u32);
pub const ERROR_IPSEC_IKE_NO_PEER_CERT: WIN32_ERROR = WIN32_ERROR(13847u32);
pub const ERROR_IPSEC_IKE_PEER_CRL_FAILED: WIN32_ERROR = WIN32_ERROR(13848u32);
pub const ERROR_IPSEC_IKE_POLICY_CHANGE: WIN32_ERROR = WIN32_ERROR(13849u32);
pub const ERROR_IPSEC_IKE_NO_MM_POLICY: WIN32_ERROR = WIN32_ERROR(13850u32);
pub const ERROR_IPSEC_IKE_NOTCBPRIV: WIN32_ERROR = WIN32_ERROR(13851u32);
pub const ERROR_IPSEC_IKE_SECLOADFAIL: WIN32_ERROR = WIN32_ERROR(13852u32);
pub const ERROR_IPSEC_IKE_FAILSSPINIT: WIN32_ERROR = WIN32_ERROR(13853u32);
pub const ERROR_IPSEC_IKE_FAILQUERYSSP: WIN32_ERROR = WIN32_ERROR(13854u32);
pub const ERROR_IPSEC_IKE_SRVACQFAIL: WIN32_ERROR = WIN32_ERROR(13855u32);
pub const ERROR_IPSEC_IKE_SRVQUERYCRED: WIN32_ERROR = WIN32_ERROR(13856u32);
pub const ERROR_IPSEC_IKE_GETSPIFAIL: WIN32_ERROR = WIN32_ERROR(13857u32);
pub const ERROR_IPSEC_IKE_INVALID_FILTER: WIN32_ERROR = WIN32_ERROR(13858u32);
pub const ERROR_IPSEC_IKE_OUT_OF_MEMORY: WIN32_ERROR = WIN32_ERROR(13859u32);
pub const ERROR_IPSEC_IKE_ADD_UPDATE_KEY_FAILED: WIN32_ERROR = WIN32_ERROR(13860u32);
pub const ERROR_IPSEC_IKE_INVALID_POLICY: WIN32_ERROR = WIN32_ERROR(13861u32);
pub const ERROR_IPSEC_IKE_UNKNOWN_DOI: WIN32_ERROR = WIN32_ERROR(13862u32);
pub const ERROR_IPSEC_IKE_INVALID_SITUATION: WIN32_ERROR = WIN32_ERROR(13863u32);
pub const ERROR_IPSEC_IKE_DH_FAILURE: WIN32_ERROR = WIN32_ERROR(13864u32);
pub const ERROR_IPSEC_IKE_INVALID_GROUP: WIN32_ERROR = WIN32_ERROR(13865u32);
pub const ERROR_IPSEC_IKE_ENCRYPT: WIN32_ERROR = WIN32_ERROR(13866u32);
pub const ERROR_IPSEC_IKE_DECRYPT: WIN32_ERROR = WIN32_ERROR(13867u32);
pub const ERROR_IPSEC_IKE_POLICY_MATCH: WIN32_ERROR = WIN32_ERROR(13868u32);
pub const ERROR_IPSEC_IKE_UNSUPPORTED_ID: WIN32_ERROR = WIN32_ERROR(13869u32);
pub const ERROR_IPSEC_IKE_INVALID_HASH: WIN32_ERROR = WIN32_ERROR(13870u32);
pub const ERROR_IPSEC_IKE_INVALID_HASH_ALG: WIN32_ERROR = WIN32_ERROR(13871u32);
pub const ERROR_IPSEC_IKE_INVALID_HASH_SIZE: WIN32_ERROR = WIN32_ERROR(13872u32);
pub const ERROR_IPSEC_IKE_INVALID_ENCRYPT_ALG: WIN32_ERROR = WIN32_ERROR(13873u32);
pub const ERROR_IPSEC_IKE_INVALID_AUTH_ALG: WIN32_ERROR = WIN32_ERROR(13874u32);
pub const ERROR_IPSEC_IKE_INVALID_SIG: WIN32_ERROR = WIN32_ERROR(13875u32);
pub const ERROR_IPSEC_IKE_LOAD_FAILED: WIN32_ERROR = WIN32_ERROR(13876u32);
pub const ERROR_IPSEC_IKE_RPC_DELETE: WIN32_ERROR = WIN32_ERROR(13877u32);
pub const ERROR_IPSEC_IKE_BENIGN_REINIT: WIN32_ERROR = WIN32_ERROR(13878u32);
pub const ERROR_IPSEC_IKE_INVALID_RESPONDER_LIFETIME_NOTIFY: WIN32_ERROR = WIN32_ERROR(13879u32);
pub const ERROR_IPSEC_IKE_INVALID_MAJOR_VERSION: WIN32_ERROR = WIN32_ERROR(13880u32);
pub const ERROR_IPSEC_IKE_INVALID_CERT_KEYLEN: WIN32_ERROR = WIN32_ERROR(13881u32);
pub const ERROR_IPSEC_IKE_MM_LIMIT: WIN32_ERROR = WIN32_ERROR(13882u32);
pub const ERROR_IPSEC_IKE_NEGOTIATION_DISABLED: WIN32_ERROR = WIN32_ERROR(13883u32);
pub const ERROR_IPSEC_IKE_QM_LIMIT: WIN32_ERROR = WIN32_ERROR(13884u32);
pub const ERROR_IPSEC_IKE_MM_EXPIRED: WIN32_ERROR = WIN32_ERROR(13885u32);
pub const ERROR_IPSEC_IKE_PEER_MM_ASSUMED_INVALID: WIN32_ERROR = WIN32_ERROR(13886u32);
pub const ERROR_IPSEC_IKE_CERT_CHAIN_POLICY_MISMATCH: WIN32_ERROR = WIN32_ERROR(13887u32);
pub const ERROR_IPSEC_IKE_UNEXPECTED_MESSAGE_ID: WIN32_ERROR = WIN32_ERROR(13888u32);
pub const ERROR_IPSEC_IKE_INVALID_AUTH_PAYLOAD: WIN32_ERROR = WIN32_ERROR(13889u32);
pub const ERROR_IPSEC_IKE_DOS_COOKIE_SENT: WIN32_ERROR = WIN32_ERROR(13890u32);
pub const ERROR_IPSEC_IKE_SHUTTING_DOWN: WIN32_ERROR = WIN32_ERROR(13891u32);
pub const ERROR_IPSEC_IKE_CGA_AUTH_FAILED: WIN32_ERROR = WIN32_ERROR(13892u32);
pub const ERROR_IPSEC_IKE_PROCESS_ERR_NATOA: WIN32_ERROR = WIN32_ERROR(13893u32);
pub const ERROR_IPSEC_IKE_INVALID_MM_FOR_QM: WIN32_ERROR = WIN32_ERROR(13894u32);
pub const ERROR_IPSEC_IKE_QM_EXPIRED: WIN32_ERROR = WIN32_ERROR(13895u32);
pub const ERROR_IPSEC_IKE_TOO_MANY_FILTERS: WIN32_ERROR = WIN32_ERROR(13896u32);
pub const ERROR_IPSEC_IKE_NEG_STATUS_END: WIN32_ERROR = WIN32_ERROR(13897u32);
pub const ERROR_IPSEC_IKE_KILL_DUMMY_NAP_TUNNEL: WIN32_ERROR = WIN32_ERROR(13898u32);
pub const ERROR_IPSEC_IKE_INNER_IP_ASSIGNMENT_FAILURE: WIN32_ERROR = WIN32_ERROR(13899u32);
pub const ERROR_IPSEC_IKE_REQUIRE_CP_PAYLOAD_MISSING: WIN32_ERROR = WIN32_ERROR(13900u32);
pub const ERROR_IPSEC_KEY_MODULE_IMPERSONATION_NEGOTIATION_PENDING: WIN32_ERROR =
    WIN32_ERROR(13901u32);
pub const ERROR_IPSEC_IKE_COEXISTENCE_SUPPRESS: WIN32_ERROR = WIN32_ERROR(13902u32);
pub const ERROR_IPSEC_IKE_RATELIMIT_DROP: WIN32_ERROR = WIN32_ERROR(13903u32);
pub const ERROR_IPSEC_IKE_PEER_DOESNT_SUPPORT_MOBIKE: WIN32_ERROR = WIN32_ERROR(13904u32);
pub const ERROR_IPSEC_IKE_AUTHORIZATION_FAILURE: WIN32_ERROR = WIN32_ERROR(13905u32);
pub const ERROR_IPSEC_IKE_STRONG_CRED_AUTHORIZATION_FAILURE: WIN32_ERROR = WIN32_ERROR(13906u32);
pub const ERROR_IPSEC_IKE_AUTHORIZATION_FAILURE_WITH_OPTIONAL_RETRY: WIN32_ERROR =
    WIN32_ERROR(13907u32);
pub const ERROR_IPSEC_IKE_STRONG_CRED_AUTHORIZATION_AND_CERTMAP_FAILURE: WIN32_ERROR =
    WIN32_ERROR(13908u32);
pub const ERROR_IPSEC_IKE_NEG_STATUS_EXTENDED_END: WIN32_ERROR = WIN32_ERROR(13909u32);
pub const ERROR_IPSEC_BAD_SPI: WIN32_ERROR = WIN32_ERROR(13910u32);
pub const ERROR_IPSEC_SA_LIFETIME_EXPIRED: WIN32_ERROR = WIN32_ERROR(13911u32);
pub const ERROR_IPSEC_WRONG_SA: WIN32_ERROR = WIN32_ERROR(13912u32);
pub const ERROR_IPSEC_REPLAY_CHECK_FAILED: WIN32_ERROR = WIN32_ERROR(13913u32);
pub const ERROR_IPSEC_INVALID_PACKET: WIN32_ERROR = WIN32_ERROR(13914u32);
pub const ERROR_IPSEC_INTEGRITY_CHECK_FAILED: WIN32_ERROR = WIN32_ERROR(13915u32);
pub const ERROR_IPSEC_CLEAR_TEXT_DROP: WIN32_ERROR = WIN32_ERROR(13916u32);
pub const ERROR_IPSEC_AUTH_FIREWALL_DROP: WIN32_ERROR = WIN32_ERROR(13917u32);
pub const ERROR_IPSEC_THROTTLE_DROP: WIN32_ERROR = WIN32_ERROR(13918u32);
pub const ERROR_IPSEC_DOSP_BLOCK: WIN32_ERROR = WIN32_ERROR(13925u32);
pub const ERROR_IPSEC_DOSP_RECEIVED_MULTICAST: WIN32_ERROR = WIN32_ERROR(13926u32);
pub const ERROR_IPSEC_DOSP_INVALID_PACKET: WIN32_ERROR = WIN32_ERROR(13927u32);
pub const ERROR_IPSEC_DOSP_STATE_LOOKUP_FAILED: WIN32_ERROR = WIN32_ERROR(13928u32);
pub const ERROR_IPSEC_DOSP_MAX_ENTRIES: WIN32_ERROR = WIN32_ERROR(13929u32);
pub const ERROR_IPSEC_DOSP_KEYMOD_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(13930u32);
pub const ERROR_IPSEC_DOSP_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(13931u32);
pub const ERROR_IPSEC_DOSP_MAX_PER_IP_RATELIMIT_QUEUES: WIN32_ERROR = WIN32_ERROR(13932u32);
pub const ERROR_SXS_SECTION_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(14000u32);
pub const ERROR_SXS_CANT_GEN_ACTCTX: WIN32_ERROR = WIN32_ERROR(14001u32);
pub const ERROR_SXS_INVALID_ACTCTXDATA_FORMAT: WIN32_ERROR = WIN32_ERROR(14002u32);
pub const ERROR_SXS_ASSEMBLY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(14003u32);
pub const ERROR_SXS_MANIFEST_FORMAT_ERROR: WIN32_ERROR = WIN32_ERROR(14004u32);
pub const ERROR_SXS_MANIFEST_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(14005u32);
pub const ERROR_SXS_ACTIVATION_CONTEXT_DISABLED: WIN32_ERROR = WIN32_ERROR(14006u32);
pub const ERROR_SXS_KEY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(14007u32);
pub const ERROR_SXS_VERSION_CONFLICT: WIN32_ERROR = WIN32_ERROR(14008u32);
pub const ERROR_SXS_WRONG_SECTION_TYPE: WIN32_ERROR = WIN32_ERROR(14009u32);
pub const ERROR_SXS_THREAD_QUERIES_DISABLED: WIN32_ERROR = WIN32_ERROR(14010u32);
pub const ERROR_SXS_PROCESS_DEFAULT_ALREADY_SET: WIN32_ERROR = WIN32_ERROR(14011u32);
pub const ERROR_SXS_UNKNOWN_ENCODING_GROUP: WIN32_ERROR = WIN32_ERROR(14012u32);
pub const ERROR_SXS_UNKNOWN_ENCODING: WIN32_ERROR = WIN32_ERROR(14013u32);
pub const ERROR_SXS_INVALID_XML_NAMESPACE_URI: WIN32_ERROR = WIN32_ERROR(14014u32);
pub const ERROR_SXS_ROOT_MANIFEST_DEPENDENCY_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(14015u32);
pub const ERROR_SXS_LEAF_MANIFEST_DEPENDENCY_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(14016u32);
pub const ERROR_SXS_INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE: WIN32_ERROR = WIN32_ERROR(14017u32);
pub const ERROR_SXS_MANIFEST_MISSING_REQUIRED_DEFAULT_NAMESPACE: WIN32_ERROR =
    WIN32_ERROR(14018u32);
pub const ERROR_SXS_MANIFEST_INVALID_REQUIRED_DEFAULT_NAMESPACE: WIN32_ERROR =
    WIN32_ERROR(14019u32);
pub const ERROR_SXS_PRIVATE_MANIFEST_CROSS_PATH_WITH_REPARSE_POINT: WIN32_ERROR =
    WIN32_ERROR(14020u32);
pub const ERROR_SXS_DUPLICATE_DLL_NAME: WIN32_ERROR = WIN32_ERROR(14021u32);
pub const ERROR_SXS_DUPLICATE_WINDOWCLASS_NAME: WIN32_ERROR = WIN32_ERROR(14022u32);
pub const ERROR_SXS_DUPLICATE_CLSID: WIN32_ERROR = WIN32_ERROR(14023u32);
pub const ERROR_SXS_DUPLICATE_IID: WIN32_ERROR = WIN32_ERROR(14024u32);
pub const ERROR_SXS_DUPLICATE_TLBID: WIN32_ERROR = WIN32_ERROR(14025u32);
pub const ERROR_SXS_DUPLICATE_PROGID: WIN32_ERROR = WIN32_ERROR(14026u32);
pub const ERROR_SXS_DUPLICATE_ASSEMBLY_NAME: WIN32_ERROR = WIN32_ERROR(14027u32);
pub const ERROR_SXS_FILE_HASH_MISMATCH: WIN32_ERROR = WIN32_ERROR(14028u32);
pub const ERROR_SXS_POLICY_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(14029u32);
pub const ERROR_SXS_XML_E_MISSINGQUOTE: WIN32_ERROR = WIN32_ERROR(14030u32);
pub const ERROR_SXS_XML_E_COMMENTSYNTAX: WIN32_ERROR = WIN32_ERROR(14031u32);
pub const ERROR_SXS_XML_E_BADSTARTNAMECHAR: WIN32_ERROR = WIN32_ERROR(14032u32);
pub const ERROR_SXS_XML_E_BADNAMECHAR: WIN32_ERROR = WIN32_ERROR(14033u32);
pub const ERROR_SXS_XML_E_BADCHARINSTRING: WIN32_ERROR = WIN32_ERROR(14034u32);
pub const ERROR_SXS_XML_E_XMLDECLSYNTAX: WIN32_ERROR = WIN32_ERROR(14035u32);
pub const ERROR_SXS_XML_E_BADCHARDATA: WIN32_ERROR = WIN32_ERROR(14036u32);
pub const ERROR_SXS_XML_E_MISSINGWHITESPACE: WIN32_ERROR = WIN32_ERROR(14037u32);
pub const ERROR_SXS_XML_E_EXPECTINGTAGEND: WIN32_ERROR = WIN32_ERROR(14038u32);
pub const ERROR_SXS_XML_E_MISSINGSEMICOLON: WIN32_ERROR = WIN32_ERROR(14039u32);
pub const ERROR_SXS_XML_E_UNBALANCEDPAREN: WIN32_ERROR = WIN32_ERROR(14040u32);
pub const ERROR_SXS_XML_E_INTERNALERROR: WIN32_ERROR = WIN32_ERROR(14041u32);
pub const ERROR_SXS_XML_E_UNEXPECTED_WHITESPACE: WIN32_ERROR = WIN32_ERROR(14042u32);
pub const ERROR_SXS_XML_E_INCOMPLETE_ENCODING: WIN32_ERROR = WIN32_ERROR(14043u32);
pub const ERROR_SXS_XML_E_MISSING_PAREN: WIN32_ERROR = WIN32_ERROR(14044u32);
pub const ERROR_SXS_XML_E_EXPECTINGCLOSEQUOTE: WIN32_ERROR = WIN32_ERROR(14045u32);
pub const ERROR_SXS_XML_E_MULTIPLE_COLONS: WIN32_ERROR = WIN32_ERROR(14046u32);
pub const ERROR_SXS_XML_E_INVALID_DECIMAL: WIN32_ERROR = WIN32_ERROR(14047u32);
pub const ERROR_SXS_XML_E_INVALID_HEXIDECIMAL: WIN32_ERROR = WIN32_ERROR(14048u32);
pub const ERROR_SXS_XML_E_INVALID_UNICODE: WIN32_ERROR = WIN32_ERROR(14049u32);
pub const ERROR_SXS_XML_E_WHITESPACEORQUESTIONMARK: WIN32_ERROR = WIN32_ERROR(14050u32);
pub const ERROR_SXS_XML_E_UNEXPECTEDENDTAG: WIN32_ERROR = WIN32_ERROR(14051u32);
pub const ERROR_SXS_XML_E_UNCLOSEDTAG: WIN32_ERROR = WIN32_ERROR(14052u32);
pub const ERROR_SXS_XML_E_DUPLICATEATTRIBUTE: WIN32_ERROR = WIN32_ERROR(14053u32);
pub const ERROR_SXS_XML_E_MULTIPLEROOTS: WIN32_ERROR = WIN32_ERROR(14054u32);
pub const ERROR_SXS_XML_E_INVALIDATROOTLEVEL: WIN32_ERROR = WIN32_ERROR(14055u32);
pub const ERROR_SXS_XML_E_BADXMLDECL: WIN32_ERROR = WIN32_ERROR(14056u32);
pub const ERROR_SXS_XML_E_MISSINGROOT: WIN32_ERROR = WIN32_ERROR(14057u32);
pub const ERROR_SXS_XML_E_UNEXPECTEDEOF: WIN32_ERROR = WIN32_ERROR(14058u32);
pub const ERROR_SXS_XML_E_BADPEREFINSUBSET: WIN32_ERROR = WIN32_ERROR(14059u32);
pub const ERROR_SXS_XML_E_UNCLOSEDSTARTTAG: WIN32_ERROR = WIN32_ERROR(14060u32);
pub const ERROR_SXS_XML_E_UNCLOSEDENDTAG: WIN32_ERROR = WIN32_ERROR(14061u32);
pub const ERROR_SXS_XML_E_UNCLOSEDSTRING: WIN32_ERROR = WIN32_ERROR(14062u32);
pub const ERROR_SXS_XML_E_UNCLOSEDCOMMENT: WIN32_ERROR = WIN32_ERROR(14063u32);
pub const ERROR_SXS_XML_E_UNCLOSEDDECL: WIN32_ERROR = WIN32_ERROR(14064u32);
pub const ERROR_SXS_XML_E_UNCLOSEDCDATA: WIN32_ERROR = WIN32_ERROR(14065u32);
pub const ERROR_SXS_XML_E_RESERVEDNAMESPACE: WIN32_ERROR = WIN32_ERROR(14066u32);
pub const ERROR_SXS_XML_E_INVALIDENCODING: WIN32_ERROR = WIN32_ERROR(14067u32);
pub const ERROR_SXS_XML_E_INVALIDSWITCH: WIN32_ERROR = WIN32_ERROR(14068u32);
pub const ERROR_SXS_XML_E_BADXMLCASE: WIN32_ERROR = WIN32_ERROR(14069u32);
pub const ERROR_SXS_XML_E_INVALID_STANDALONE: WIN32_ERROR = WIN32_ERROR(14070u32);
pub const ERROR_SXS_XML_E_UNEXPECTED_STANDALONE: WIN32_ERROR = WIN32_ERROR(14071u32);
pub const ERROR_SXS_XML_E_INVALID_VERSION: WIN32_ERROR = WIN32_ERROR(14072u32);
pub const ERROR_SXS_XML_E_MISSINGEQUALS: WIN32_ERROR = WIN32_ERROR(14073u32);
pub const ERROR_SXS_PROTECTION_RECOVERY_FAILED: WIN32_ERROR = WIN32_ERROR(14074u32);
pub const ERROR_SXS_PROTECTION_PUBLIC_KEY_TOO_SHORT: WIN32_ERROR = WIN32_ERROR(14075u32);
pub const ERROR_SXS_PROTECTION_CATALOG_NOT_VALID: WIN32_ERROR = WIN32_ERROR(14076u32);
pub const ERROR_SXS_UNTRANSLATABLE_HRESULT: WIN32_ERROR = WIN32_ERROR(14077u32);
pub const ERROR_SXS_PROTECTION_CATALOG_FILE_MISSING: WIN32_ERROR = WIN32_ERROR(14078u32);
pub const ERROR_SXS_MISSING_ASSEMBLY_IDENTITY_ATTRIBUTE: WIN32_ERROR = WIN32_ERROR(14079u32);
pub const ERROR_SXS_INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE_NAME: WIN32_ERROR = WIN32_ERROR(14080u32);
pub const ERROR_SXS_ASSEMBLY_MISSING: WIN32_ERROR = WIN32_ERROR(14081u32);
pub const ERROR_SXS_CORRUPT_ACTIVATION_STACK: WIN32_ERROR = WIN32_ERROR(14082u32);
pub const ERROR_SXS_CORRUPTION: WIN32_ERROR = WIN32_ERROR(14083u32);
pub const ERROR_SXS_EARLY_DEACTIVATION: WIN32_ERROR = WIN32_ERROR(14084u32);
pub const ERROR_SXS_INVALID_DEACTIVATION: WIN32_ERROR = WIN32_ERROR(14085u32);
pub const ERROR_SXS_MULTIPLE_DEACTIVATION: WIN32_ERROR = WIN32_ERROR(14086u32);
pub const ERROR_SXS_PROCESS_TERMINATION_REQUESTED: WIN32_ERROR = WIN32_ERROR(14087u32);
pub const ERROR_SXS_RELEASE_ACTIVATION_CONTEXT: WIN32_ERROR = WIN32_ERROR(14088u32);
pub const ERROR_SXS_SYSTEM_DEFAULT_ACTIVATION_CONTEXT_EMPTY: WIN32_ERROR = WIN32_ERROR(14089u32);
pub const ERROR_SXS_INVALID_IDENTITY_ATTRIBUTE_VALUE: WIN32_ERROR = WIN32_ERROR(14090u32);
pub const ERROR_SXS_INVALID_IDENTITY_ATTRIBUTE_NAME: WIN32_ERROR = WIN32_ERROR(14091u32);
pub const ERROR_SXS_IDENTITY_DUPLICATE_ATTRIBUTE: WIN32_ERROR = WIN32_ERROR(14092u32);
pub const ERROR_SXS_IDENTITY_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(14093u32);
pub const ERROR_MALFORMED_SUBSTITUTION_STRING: WIN32_ERROR = WIN32_ERROR(14094u32);
pub const ERROR_SXS_INCORRECT_PUBLIC_KEY_TOKEN: WIN32_ERROR = WIN32_ERROR(14095u32);
pub const ERROR_UNMAPPED_SUBSTITUTION_STRING: WIN32_ERROR = WIN32_ERROR(14096u32);
pub const ERROR_SXS_ASSEMBLY_NOT_LOCKED: WIN32_ERROR = WIN32_ERROR(14097u32);
pub const ERROR_SXS_COMPONENT_STORE_CORRUPT: WIN32_ERROR = WIN32_ERROR(14098u32);
pub const ERROR_ADVANCED_INSTALLER_FAILED: WIN32_ERROR = WIN32_ERROR(14099u32);
pub const ERROR_XML_ENCODING_MISMATCH: WIN32_ERROR = WIN32_ERROR(14100u32);
pub const ERROR_SXS_MANIFEST_IDENTITY_SAME_BUT_CONTENTS_DIFFERENT: WIN32_ERROR =
    WIN32_ERROR(14101u32);
pub const ERROR_SXS_IDENTITIES_DIFFERENT: WIN32_ERROR = WIN32_ERROR(14102u32);
pub const ERROR_SXS_ASSEMBLY_IS_NOT_A_DEPLOYMENT: WIN32_ERROR = WIN32_ERROR(14103u32);
pub const ERROR_SXS_FILE_NOT_PART_OF_ASSEMBLY: WIN32_ERROR = WIN32_ERROR(14104u32);
pub const ERROR_SXS_MANIFEST_TOO_BIG: WIN32_ERROR = WIN32_ERROR(14105u32);
pub const ERROR_SXS_SETTING_NOT_REGISTERED: WIN32_ERROR = WIN32_ERROR(14106u32);
pub const ERROR_SXS_TRANSACTION_CLOSURE_INCOMPLETE: WIN32_ERROR = WIN32_ERROR(14107u32);
pub const ERROR_SMI_PRIMITIVE_INSTALLER_FAILED: WIN32_ERROR = WIN32_ERROR(14108u32);
pub const ERROR_GENERIC_COMMAND_FAILED: WIN32_ERROR = WIN32_ERROR(14109u32);
pub const ERROR_SXS_FILE_HASH_MISSING: WIN32_ERROR = WIN32_ERROR(14110u32);
pub const ERROR_SXS_DUPLICATE_ACTIVATABLE_CLASS: WIN32_ERROR = WIN32_ERROR(14111u32);
pub const ERROR_EVT_INVALID_CHANNEL_PATH: WIN32_ERROR = WIN32_ERROR(15000u32);
pub const ERROR_EVT_INVALID_QUERY: WIN32_ERROR = WIN32_ERROR(15001u32);
pub const ERROR_EVT_PUBLISHER_METADATA_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15002u32);
pub const ERROR_EVT_EVENT_TEMPLATE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15003u32);
pub const ERROR_EVT_INVALID_PUBLISHER_NAME: WIN32_ERROR = WIN32_ERROR(15004u32);
pub const ERROR_EVT_INVALID_EVENT_DATA: WIN32_ERROR = WIN32_ERROR(15005u32);
pub const ERROR_EVT_CHANNEL_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15007u32);
pub const ERROR_EVT_MALFORMED_XML_TEXT: WIN32_ERROR = WIN32_ERROR(15008u32);
pub const ERROR_EVT_SUBSCRIPTION_TO_DIRECT_CHANNEL: WIN32_ERROR = WIN32_ERROR(15009u32);
pub const ERROR_EVT_CONFIGURATION_ERROR: WIN32_ERROR = WIN32_ERROR(15010u32);
pub const ERROR_EVT_QUERY_RESULT_STALE: WIN32_ERROR = WIN32_ERROR(15011u32);
pub const ERROR_EVT_QUERY_RESULT_INVALID_POSITION: WIN32_ERROR = WIN32_ERROR(15012u32);
pub const ERROR_EVT_NON_VALIDATING_MSXML: WIN32_ERROR = WIN32_ERROR(15013u32);
pub const ERROR_EVT_FILTER_ALREADYSCOPED: WIN32_ERROR = WIN32_ERROR(15014u32);
pub const ERROR_EVT_FILTER_NOTELTSET: WIN32_ERROR = WIN32_ERROR(15015u32);
pub const ERROR_EVT_FILTER_INVARG: WIN32_ERROR = WIN32_ERROR(15016u32);
pub const ERROR_EVT_FILTER_INVTEST: WIN32_ERROR = WIN32_ERROR(15017u32);
pub const ERROR_EVT_FILTER_INVTYPE: WIN32_ERROR = WIN32_ERROR(15018u32);
pub const ERROR_EVT_FILTER_PARSEERR: WIN32_ERROR = WIN32_ERROR(15019u32);
pub const ERROR_EVT_FILTER_UNSUPPORTEDOP: WIN32_ERROR = WIN32_ERROR(15020u32);
pub const ERROR_EVT_FILTER_UNEXPECTEDTOKEN: WIN32_ERROR = WIN32_ERROR(15021u32);
pub const ERROR_EVT_INVALID_OPERATION_OVER_ENABLED_DIRECT_CHANNEL: WIN32_ERROR =
    WIN32_ERROR(15022u32);
pub const ERROR_EVT_INVALID_CHANNEL_PROPERTY_VALUE: WIN32_ERROR = WIN32_ERROR(15023u32);
pub const ERROR_EVT_INVALID_PUBLISHER_PROPERTY_VALUE: WIN32_ERROR = WIN32_ERROR(15024u32);
pub const ERROR_EVT_CHANNEL_CANNOT_ACTIVATE: WIN32_ERROR = WIN32_ERROR(15025u32);
pub const ERROR_EVT_FILTER_TOO_COMPLEX: WIN32_ERROR = WIN32_ERROR(15026u32);
pub const ERROR_EVT_MESSAGE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15027u32);
pub const ERROR_EVT_MESSAGE_ID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15028u32);
pub const ERROR_EVT_UNRESOLVED_VALUE_INSERT: WIN32_ERROR = WIN32_ERROR(15029u32);
pub const ERROR_EVT_UNRESOLVED_PARAMETER_INSERT: WIN32_ERROR = WIN32_ERROR(15030u32);
pub const ERROR_EVT_MAX_INSERTS_REACHED: WIN32_ERROR = WIN32_ERROR(15031u32);
pub const ERROR_EVT_EVENT_DEFINITION_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15032u32);
pub const ERROR_EVT_MESSAGE_LOCALE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15033u32);
pub const ERROR_EVT_VERSION_TOO_OLD: WIN32_ERROR = WIN32_ERROR(15034u32);
pub const ERROR_EVT_VERSION_TOO_NEW: WIN32_ERROR = WIN32_ERROR(15035u32);
pub const ERROR_EVT_CANNOT_OPEN_CHANNEL_OF_QUERY: WIN32_ERROR = WIN32_ERROR(15036u32);
pub const ERROR_EVT_PUBLISHER_DISABLED: WIN32_ERROR = WIN32_ERROR(15037u32);
pub const ERROR_EVT_FILTER_OUT_OF_RANGE: WIN32_ERROR = WIN32_ERROR(15038u32);
pub const ERROR_EC_SUBSCRIPTION_CANNOT_ACTIVATE: WIN32_ERROR = WIN32_ERROR(15080u32);
pub const ERROR_EC_LOG_DISABLED: WIN32_ERROR = WIN32_ERROR(15081u32);
pub const ERROR_EC_CIRCULAR_FORWARDING: WIN32_ERROR = WIN32_ERROR(15082u32);
pub const ERROR_EC_CREDSTORE_FULL: WIN32_ERROR = WIN32_ERROR(15083u32);
pub const ERROR_EC_CRED_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15084u32);
pub const ERROR_EC_NO_ACTIVE_CHANNEL: WIN32_ERROR = WIN32_ERROR(15085u32);
pub const ERROR_MUI_FILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15100u32);
pub const ERROR_MUI_INVALID_FILE: WIN32_ERROR = WIN32_ERROR(15101u32);
pub const ERROR_MUI_INVALID_RC_CONFIG: WIN32_ERROR = WIN32_ERROR(15102u32);
pub const ERROR_MUI_INVALID_LOCALE_NAME: WIN32_ERROR = WIN32_ERROR(15103u32);
pub const ERROR_MUI_INVALID_ULTIMATEFALLBACK_NAME: WIN32_ERROR = WIN32_ERROR(15104u32);
pub const ERROR_MUI_FILE_NOT_LOADED: WIN32_ERROR = WIN32_ERROR(15105u32);
pub const ERROR_RESOURCE_ENUM_USER_STOP: WIN32_ERROR = WIN32_ERROR(15106u32);
pub const ERROR_MUI_INTLSETTINGS_UILANG_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(15107u32);
pub const ERROR_MUI_INTLSETTINGS_INVALID_LOCALE_NAME: WIN32_ERROR = WIN32_ERROR(15108u32);
pub const ERROR_MRM_RUNTIME_NO_DEFAULT_OR_NEUTRAL_RESOURCE: WIN32_ERROR = WIN32_ERROR(15110u32);
pub const ERROR_MRM_INVALID_PRICONFIG: WIN32_ERROR = WIN32_ERROR(15111u32);
pub const ERROR_MRM_INVALID_FILE_TYPE: WIN32_ERROR = WIN32_ERROR(15112u32);
pub const ERROR_MRM_UNKNOWN_QUALIFIER: WIN32_ERROR = WIN32_ERROR(15113u32);
pub const ERROR_MRM_INVALID_QUALIFIER_VALUE: WIN32_ERROR = WIN32_ERROR(15114u32);
pub const ERROR_MRM_NO_CANDIDATE: WIN32_ERROR = WIN32_ERROR(15115u32);
pub const ERROR_MRM_NO_MATCH_OR_DEFAULT_CANDIDATE: WIN32_ERROR = WIN32_ERROR(15116u32);
pub const ERROR_MRM_RESOURCE_TYPE_MISMATCH: WIN32_ERROR = WIN32_ERROR(15117u32);
pub const ERROR_MRM_DUPLICATE_MAP_NAME: WIN32_ERROR = WIN32_ERROR(15118u32);
pub const ERROR_MRM_DUPLICATE_ENTRY: WIN32_ERROR = WIN32_ERROR(15119u32);
pub const ERROR_MRM_INVALID_RESOURCE_IDENTIFIER: WIN32_ERROR = WIN32_ERROR(15120u32);
pub const ERROR_MRM_FILEPATH_TOO_LONG: WIN32_ERROR = WIN32_ERROR(15121u32);
pub const ERROR_MRM_UNSUPPORTED_DIRECTORY_TYPE: WIN32_ERROR = WIN32_ERROR(15122u32);
pub const ERROR_MRM_INVALID_PRI_FILE: WIN32_ERROR = WIN32_ERROR(15126u32);
pub const ERROR_MRM_NAMED_RESOURCE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15127u32);
pub const ERROR_MRM_MAP_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15135u32);
pub const ERROR_MRM_UNSUPPORTED_PROFILE_TYPE: WIN32_ERROR = WIN32_ERROR(15136u32);
pub const ERROR_MRM_INVALID_QUALIFIER_OPERATOR: WIN32_ERROR = WIN32_ERROR(15137u32);
pub const ERROR_MRM_INDETERMINATE_QUALIFIER_VALUE: WIN32_ERROR = WIN32_ERROR(15138u32);
pub const ERROR_MRM_AUTOMERGE_ENABLED: WIN32_ERROR = WIN32_ERROR(15139u32);
pub const ERROR_MRM_TOO_MANY_RESOURCES: WIN32_ERROR = WIN32_ERROR(15140u32);
pub const ERROR_MRM_UNSUPPORTED_FILE_TYPE_FOR_MERGE: WIN32_ERROR = WIN32_ERROR(15141u32);
pub const ERROR_MRM_UNSUPPORTED_FILE_TYPE_FOR_LOAD_UNLOAD_PRI_FILE: WIN32_ERROR =
    WIN32_ERROR(15142u32);
pub const ERROR_MRM_NO_CURRENT_VIEW_ON_THREAD: WIN32_ERROR = WIN32_ERROR(15143u32);
pub const ERROR_DIFFERENT_PROFILE_RESOURCE_MANAGER_EXIST: WIN32_ERROR = WIN32_ERROR(15144u32);
pub const ERROR_OPERATION_NOT_ALLOWED_FROM_SYSTEM_COMPONENT: WIN32_ERROR = WIN32_ERROR(15145u32);
pub const ERROR_MRM_DIRECT_REF_TO_NON_DEFAULT_RESOURCE: WIN32_ERROR = WIN32_ERROR(15146u32);
pub const ERROR_MRM_GENERATION_COUNT_MISMATCH: WIN32_ERROR = WIN32_ERROR(15147u32);
pub const ERROR_PRI_MERGE_VERSION_MISMATCH: WIN32_ERROR = WIN32_ERROR(15148u32);
pub const ERROR_PRI_MERGE_MISSING_SCHEMA: WIN32_ERROR = WIN32_ERROR(15149u32);
pub const ERROR_PRI_MERGE_LOAD_FILE_FAILED: WIN32_ERROR = WIN32_ERROR(15150u32);
pub const ERROR_PRI_MERGE_ADD_FILE_FAILED: WIN32_ERROR = WIN32_ERROR(15151u32);
pub const ERROR_PRI_MERGE_WRITE_FILE_FAILED: WIN32_ERROR = WIN32_ERROR(15152u32);
pub const ERROR_PRI_MERGE_MULTIPLE_PACKAGE_FAMILIES_NOT_ALLOWED: WIN32_ERROR =
    WIN32_ERROR(15153u32);
pub const ERROR_PRI_MERGE_MULTIPLE_MAIN_PACKAGES_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(15154u32);
pub const ERROR_PRI_MERGE_BUNDLE_PACKAGES_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(15155u32);
pub const ERROR_PRI_MERGE_MAIN_PACKAGE_REQUIRED: WIN32_ERROR = WIN32_ERROR(15156u32);
pub const ERROR_PRI_MERGE_RESOURCE_PACKAGE_REQUIRED: WIN32_ERROR = WIN32_ERROR(15157u32);
pub const ERROR_PRI_MERGE_INVALID_FILE_NAME: WIN32_ERROR = WIN32_ERROR(15158u32);
pub const ERROR_MRM_PACKAGE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15159u32);
pub const ERROR_MRM_MISSING_DEFAULT_LANGUAGE: WIN32_ERROR = WIN32_ERROR(15160u32);
pub const ERROR_MCA_INVALID_CAPABILITIES_STRING: WIN32_ERROR = WIN32_ERROR(15200u32);
pub const ERROR_MCA_INVALID_VCP_VERSION: WIN32_ERROR = WIN32_ERROR(15201u32);
pub const ERROR_MCA_MONITOR_VIOLATES_MCCS_SPECIFICATION: WIN32_ERROR = WIN32_ERROR(15202u32);
pub const ERROR_MCA_MCCS_VERSION_MISMATCH: WIN32_ERROR = WIN32_ERROR(15203u32);
pub const ERROR_MCA_UNSUPPORTED_MCCS_VERSION: WIN32_ERROR = WIN32_ERROR(15204u32);
pub const ERROR_MCA_INTERNAL_ERROR: WIN32_ERROR = WIN32_ERROR(15205u32);
pub const ERROR_MCA_INVALID_TECHNOLOGY_TYPE_RETURNED: WIN32_ERROR = WIN32_ERROR(15206u32);
pub const ERROR_MCA_UNSUPPORTED_COLOR_TEMPERATURE: WIN32_ERROR = WIN32_ERROR(15207u32);
pub const ERROR_AMBIGUOUS_SYSTEM_DEVICE: WIN32_ERROR = WIN32_ERROR(15250u32);
pub const ERROR_SYSTEM_DEVICE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15299u32);
pub const ERROR_HASH_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(15300u32);
pub const ERROR_HASH_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(15301u32);
pub const ERROR_SECONDARY_IC_PROVIDER_NOT_REGISTERED: WIN32_ERROR = WIN32_ERROR(15321u32);
pub const ERROR_GPIO_CLIENT_INFORMATION_INVALID: WIN32_ERROR = WIN32_ERROR(15322u32);
pub const ERROR_GPIO_VERSION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(15323u32);
pub const ERROR_GPIO_INVALID_REGISTRATION_PACKET: WIN32_ERROR = WIN32_ERROR(15324u32);
pub const ERROR_GPIO_OPERATION_DENIED: WIN32_ERROR = WIN32_ERROR(15325u32);
pub const ERROR_GPIO_INCOMPATIBLE_CONNECT_MODE: WIN32_ERROR = WIN32_ERROR(15326u32);
pub const ERROR_GPIO_INTERRUPT_ALREADY_UNMASKED: WIN32_ERROR = WIN32_ERROR(15327u32);
pub const ERROR_CANNOT_SWITCH_RUNLEVEL: WIN32_ERROR = WIN32_ERROR(15400u32);
pub const ERROR_INVALID_RUNLEVEL_SETTING: WIN32_ERROR = WIN32_ERROR(15401u32);
pub const ERROR_RUNLEVEL_SWITCH_TIMEOUT: WIN32_ERROR = WIN32_ERROR(15402u32);
pub const ERROR_RUNLEVEL_SWITCH_AGENT_TIMEOUT: WIN32_ERROR = WIN32_ERROR(15403u32);
pub const ERROR_RUNLEVEL_SWITCH_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(15404u32);
pub const ERROR_SERVICES_FAILED_AUTOSTART: WIN32_ERROR = WIN32_ERROR(15405u32);
pub const ERROR_COM_TASK_STOP_PENDING: WIN32_ERROR = WIN32_ERROR(15501u32);
pub const ERROR_INSTALL_OPEN_PACKAGE_FAILED: WIN32_ERROR = WIN32_ERROR(15600u32);
pub const ERROR_INSTALL_PACKAGE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15601u32);
pub const ERROR_INSTALL_INVALID_PACKAGE: WIN32_ERROR = WIN32_ERROR(15602u32);
pub const ERROR_INSTALL_RESOLVE_DEPENDENCY_FAILED: WIN32_ERROR = WIN32_ERROR(15603u32);
pub const ERROR_INSTALL_OUT_OF_DISK_SPACE: WIN32_ERROR = WIN32_ERROR(15604u32);
pub const ERROR_INSTALL_NETWORK_FAILURE: WIN32_ERROR = WIN32_ERROR(15605u32);
pub const ERROR_INSTALL_REGISTRATION_FAILURE: WIN32_ERROR = WIN32_ERROR(15606u32);
pub const ERROR_INSTALL_DEREGISTRATION_FAILURE: WIN32_ERROR = WIN32_ERROR(15607u32);
pub const ERROR_INSTALL_CANCEL: WIN32_ERROR = WIN32_ERROR(15608u32);
pub const ERROR_INSTALL_FAILED: WIN32_ERROR = WIN32_ERROR(15609u32);
pub const ERROR_REMOVE_FAILED: WIN32_ERROR = WIN32_ERROR(15610u32);
pub const ERROR_PACKAGE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(15611u32);
pub const ERROR_NEEDS_REMEDIATION: WIN32_ERROR = WIN32_ERROR(15612u32);
pub const ERROR_INSTALL_PREREQUISITE_FAILED: WIN32_ERROR = WIN32_ERROR(15613u32);
pub const ERROR_PACKAGE_REPOSITORY_CORRUPTED: WIN32_ERROR = WIN32_ERROR(15614u32);
pub const ERROR_INSTALL_POLICY_FAILURE: WIN32_ERROR = WIN32_ERROR(15615u32);
pub const ERROR_PACKAGE_UPDATING: WIN32_ERROR = WIN32_ERROR(15616u32);
pub const ERROR_DEPLOYMENT_BLOCKED_BY_POLICY: WIN32_ERROR = WIN32_ERROR(15617u32);
pub const ERROR_PACKAGES_IN_USE: WIN32_ERROR = WIN32_ERROR(15618u32);
pub const ERROR_RECOVERY_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(15619u32);
pub const ERROR_INVALID_STAGED_SIGNATURE: WIN32_ERROR = WIN32_ERROR(15620u32);
pub const ERROR_DELETING_EXISTING_APPLICATIONDATA_STORE_FAILED: WIN32_ERROR = WIN32_ERROR(15621u32);
pub const ERROR_INSTALL_PACKAGE_DOWNGRADE: WIN32_ERROR = WIN32_ERROR(15622u32);
pub const ERROR_SYSTEM_NEEDS_REMEDIATION: WIN32_ERROR = WIN32_ERROR(15623u32);
pub const ERROR_APPX_INTEGRITY_FAILURE_CLR_NGEN: WIN32_ERROR = WIN32_ERROR(15624u32);
pub const ERROR_RESILIENCY_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(15625u32);
pub const ERROR_INSTALL_FIREWALL_SERVICE_NOT_RUNNING: WIN32_ERROR = WIN32_ERROR(15626u32);
pub const ERROR_PACKAGE_MOVE_FAILED: WIN32_ERROR = WIN32_ERROR(15627u32);
pub const ERROR_INSTALL_VOLUME_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(15628u32);
pub const ERROR_INSTALL_VOLUME_OFFLINE: WIN32_ERROR = WIN32_ERROR(15629u32);
pub const ERROR_INSTALL_VOLUME_CORRUPT: WIN32_ERROR = WIN32_ERROR(15630u32);
pub const ERROR_NEEDS_REGISTRATION: WIN32_ERROR = WIN32_ERROR(15631u32);
pub const ERROR_INSTALL_WRONG_PROCESSOR_ARCHITECTURE: WIN32_ERROR = WIN32_ERROR(15632u32);
pub const ERROR_DEV_SIDELOAD_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(15633u32);
pub const ERROR_INSTALL_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE: WIN32_ERROR = WIN32_ERROR(15634u32);
pub const ERROR_PACKAGE_NOT_SUPPORTED_ON_FILESYSTEM: WIN32_ERROR = WIN32_ERROR(15635u32);
pub const ERROR_PACKAGE_MOVE_BLOCKED_BY_STREAMING: WIN32_ERROR = WIN32_ERROR(15636u32);
pub const ERROR_INSTALL_OPTIONAL_PACKAGE_APPLICATIONID_NOT_UNIQUE: WIN32_ERROR =
    WIN32_ERROR(15637u32);
pub const ERROR_PACKAGE_STAGING_ONHOLD: WIN32_ERROR = WIN32_ERROR(15638u32);
pub const ERROR_INSTALL_INVALID_RELATED_SET_UPDATE: WIN32_ERROR = WIN32_ERROR(15639u32);
pub const ERROR_INSTALL_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE_FULLTRUST_CAPABILITY: WIN32_ERROR =
    WIN32_ERROR(15640u32);
pub const ERROR_DEPLOYMENT_BLOCKED_BY_USER_LOG_OFF: WIN32_ERROR = WIN32_ERROR(15641u32);
pub const ERROR_PROVISION_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE_PROVISIONED: WIN32_ERROR =
    WIN32_ERROR(15642u32);
pub const ERROR_PACKAGES_REPUTATION_CHECK_FAILED: WIN32_ERROR = WIN32_ERROR(15643u32);
pub const ERROR_PACKAGES_REPUTATION_CHECK_TIMEDOUT: WIN32_ERROR = WIN32_ERROR(15644u32);
pub const ERROR_DEPLOYMENT_OPTION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(15645u32);
pub const ERROR_APPINSTALLER_ACTIVATION_BLOCKED: WIN32_ERROR = WIN32_ERROR(15646u32);
pub const ERROR_REGISTRATION_FROM_REMOTE_DRIVE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(15647u32);
pub const ERROR_APPX_RAW_DATA_WRITE_FAILED: WIN32_ERROR = WIN32_ERROR(15648u32);
pub const ERROR_DEPLOYMENT_BLOCKED_BY_VOLUME_POLICY_PACKAGE: WIN32_ERROR = WIN32_ERROR(15649u32);
pub const ERROR_DEPLOYMENT_BLOCKED_BY_VOLUME_POLICY_MACHINE: WIN32_ERROR = WIN32_ERROR(15650u32);
pub const ERROR_DEPLOYMENT_BLOCKED_BY_PROFILE_POLICY: WIN32_ERROR = WIN32_ERROR(15651u32);
pub const ERROR_DEPLOYMENT_FAILED_CONFLICTING_MUTABLE_PACKAGE_DIRECTORY: WIN32_ERROR =
    WIN32_ERROR(15652u32);
pub const ERROR_SINGLETON_RESOURCE_INSTALLED_IN_ACTIVE_USER: WIN32_ERROR = WIN32_ERROR(15653u32);
pub const ERROR_DIFFERENT_VERSION_OF_PACKAGED_SERVICE_INSTALLED: WIN32_ERROR =
    WIN32_ERROR(15654u32);
pub const ERROR_SERVICE_EXISTS_AS_NON_PACKAGED_SERVICE: WIN32_ERROR = WIN32_ERROR(15655u32);
pub const ERROR_PACKAGED_SERVICE_REQUIRES_ADMIN_PRIVILEGES: WIN32_ERROR = WIN32_ERROR(15656u32);
pub const ERROR_REDIRECTION_TO_DEFAULT_ACCOUNT_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(15657u32);
pub const ERROR_PACKAGE_LACKS_CAPABILITY_TO_DEPLOY_ON_HOST: WIN32_ERROR = WIN32_ERROR(15658u32);
pub const ERROR_UNSIGNED_PACKAGE_INVALID_CONTENT: WIN32_ERROR = WIN32_ERROR(15659u32);
pub const ERROR_UNSIGNED_PACKAGE_INVALID_PUBLISHER_NAMESPACE: WIN32_ERROR = WIN32_ERROR(15660u32);
pub const ERROR_SIGNED_PACKAGE_INVALID_PUBLISHER_NAMESPACE: WIN32_ERROR = WIN32_ERROR(15661u32);
pub const ERROR_PACKAGE_EXTERNAL_LOCATION_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(15662u32);
pub const ERROR_INSTALL_FULLTRUST_HOSTRUNTIME_REQUIRES_MAIN_PACKAGE_FULLTRUST_CAPABILITY:
    WIN32_ERROR = WIN32_ERROR(15663u32);
pub const ERROR_PACKAGE_LACKS_CAPABILITY_FOR_MANDATORY_STARTUPTASKS: WIN32_ERROR =
    WIN32_ERROR(15664u32);
pub const ERROR_INSTALL_RESOLVE_HOSTRUNTIME_DEPENDENCY_FAILED: WIN32_ERROR = WIN32_ERROR(15665u32);
pub const ERROR_MACHINE_SCOPE_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(15666u32);
pub const ERROR_CLASSIC_COMPAT_MODE_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(15667u32);
pub const ERROR_STAGEFROMUPDATEAGENT_PACKAGE_NOT_APPLICABLE: WIN32_ERROR = WIN32_ERROR(15668u32);
pub const ERROR_PACKAGE_NOT_REGISTERED_FOR_USER: WIN32_ERROR = WIN32_ERROR(15669u32);
pub const ERROR_STATE_LOAD_STORE_FAILED: WIN32_ERROR = WIN32_ERROR(15800u32);
pub const ERROR_STATE_GET_VERSION_FAILED: WIN32_ERROR = WIN32_ERROR(15801u32);
pub const ERROR_STATE_SET_VERSION_FAILED: WIN32_ERROR = WIN32_ERROR(15802u32);
pub const ERROR_STATE_STRUCTURED_RESET_FAILED: WIN32_ERROR = WIN32_ERROR(15803u32);
pub const ERROR_STATE_OPEN_CONTAINER_FAILED: WIN32_ERROR = WIN32_ERROR(15804u32);
pub const ERROR_STATE_CREATE_CONTAINER_FAILED: WIN32_ERROR = WIN32_ERROR(15805u32);
pub const ERROR_STATE_DELETE_CONTAINER_FAILED: WIN32_ERROR = WIN32_ERROR(15806u32);
pub const ERROR_STATE_READ_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15807u32);
pub const ERROR_STATE_WRITE_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15808u32);
pub const ERROR_STATE_DELETE_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15809u32);
pub const ERROR_STATE_QUERY_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15810u32);
pub const ERROR_STATE_READ_COMPOSITE_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15811u32);
pub const ERROR_STATE_WRITE_COMPOSITE_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15812u32);
pub const ERROR_STATE_ENUMERATE_CONTAINER_FAILED: WIN32_ERROR = WIN32_ERROR(15813u32);
pub const ERROR_STATE_ENUMERATE_SETTINGS_FAILED: WIN32_ERROR = WIN32_ERROR(15814u32);
pub const ERROR_STATE_COMPOSITE_SETTING_VALUE_SIZE_LIMIT_EXCEEDED: WIN32_ERROR =
    WIN32_ERROR(15815u32);
pub const ERROR_STATE_SETTING_VALUE_SIZE_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(15816u32);
pub const ERROR_STATE_SETTING_NAME_SIZE_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(15817u32);
pub const ERROR_STATE_CONTAINER_NAME_SIZE_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(15818u32);
pub const ERROR_API_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(15841u32);
pub const ERROR_NDIS_INTERFACE_CLOSING: WIN32_ERROR = WIN32_ERROR(2150891522u32);
pub const ERROR_NDIS_BAD_VERSION: WIN32_ERROR = WIN32_ERROR(2150891524u32);
pub const ERROR_NDIS_BAD_CHARACTERISTICS: WIN32_ERROR = WIN32_ERROR(2150891525u32);
pub const ERROR_NDIS_ADAPTER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2150891526u32);
pub const ERROR_NDIS_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(2150891527u32);
pub const ERROR_NDIS_DEVICE_FAILED: WIN32_ERROR = WIN32_ERROR(2150891528u32);
pub const ERROR_NDIS_MULTICAST_FULL: WIN32_ERROR = WIN32_ERROR(2150891529u32);
pub const ERROR_NDIS_MULTICAST_EXISTS: WIN32_ERROR = WIN32_ERROR(2150891530u32);
pub const ERROR_NDIS_MULTICAST_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2150891531u32);
pub const ERROR_NDIS_REQUEST_ABORTED: WIN32_ERROR = WIN32_ERROR(2150891532u32);
pub const ERROR_NDIS_RESET_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(2150891533u32);
pub const ERROR_NDIS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(2150891707u32);
pub const ERROR_NDIS_INVALID_PACKET: WIN32_ERROR = WIN32_ERROR(2150891535u32);
pub const ERROR_NDIS_ADAPTER_NOT_READY: WIN32_ERROR = WIN32_ERROR(2150891537u32);
pub const ERROR_NDIS_INVALID_LENGTH: WIN32_ERROR = WIN32_ERROR(2150891540u32);
pub const ERROR_NDIS_INVALID_DATA: WIN32_ERROR = WIN32_ERROR(2150891541u32);
pub const ERROR_NDIS_BUFFER_TOO_SHORT: WIN32_ERROR = WIN32_ERROR(2150891542u32);
pub const ERROR_NDIS_INVALID_OID: WIN32_ERROR = WIN32_ERROR(2150891543u32);
pub const ERROR_NDIS_ADAPTER_REMOVED: WIN32_ERROR = WIN32_ERROR(2150891544u32);
pub const ERROR_NDIS_UNSUPPORTED_MEDIA: WIN32_ERROR = WIN32_ERROR(2150891545u32);
pub const ERROR_NDIS_GROUP_ADDRESS_IN_USE: WIN32_ERROR = WIN32_ERROR(2150891546u32);
pub const ERROR_NDIS_FILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2150891547u32);
pub const ERROR_NDIS_ERROR_READING_FILE: WIN32_ERROR = WIN32_ERROR(2150891548u32);
pub const ERROR_NDIS_ALREADY_MAPPED: WIN32_ERROR = WIN32_ERROR(2150891549u32);
pub const ERROR_NDIS_RESOURCE_CONFLICT: WIN32_ERROR = WIN32_ERROR(2150891550u32);
pub const ERROR_NDIS_MEDIA_DISCONNECTED: WIN32_ERROR = WIN32_ERROR(2150891551u32);
pub const ERROR_NDIS_INVALID_ADDRESS: WIN32_ERROR = WIN32_ERROR(2150891554u32);
pub const ERROR_NDIS_INVALID_DEVICE_REQUEST: WIN32_ERROR = WIN32_ERROR(2150891536u32);
pub const ERROR_NDIS_PAUSED: WIN32_ERROR = WIN32_ERROR(2150891562u32);
pub const ERROR_NDIS_INTERFACE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2150891563u32);
pub const ERROR_NDIS_UNSUPPORTED_REVISION: WIN32_ERROR = WIN32_ERROR(2150891564u32);
pub const ERROR_NDIS_INVALID_PORT: WIN32_ERROR = WIN32_ERROR(2150891565u32);
pub const ERROR_NDIS_INVALID_PORT_STATE: WIN32_ERROR = WIN32_ERROR(2150891566u32);
pub const ERROR_NDIS_LOW_POWER_STATE: WIN32_ERROR = WIN32_ERROR(2150891567u32);
pub const ERROR_NDIS_REINIT_REQUIRED: WIN32_ERROR = WIN32_ERROR(2150891568u32);
pub const ERROR_NDIS_NO_QUEUES: WIN32_ERROR = WIN32_ERROR(2150891569u32);
pub const ERROR_NDIS_DOT11_AUTO_CONFIG_ENABLED: WIN32_ERROR = WIN32_ERROR(2150899712u32);
pub const ERROR_NDIS_DOT11_MEDIA_IN_USE: WIN32_ERROR = WIN32_ERROR(2150899713u32);
pub const ERROR_NDIS_DOT11_POWER_STATE_INVALID: WIN32_ERROR = WIN32_ERROR(2150899714u32);
pub const ERROR_NDIS_PM_WOL_PATTERN_LIST_FULL: WIN32_ERROR = WIN32_ERROR(2150899715u32);
pub const ERROR_NDIS_PM_PROTOCOL_OFFLOAD_LIST_FULL: WIN32_ERROR = WIN32_ERROR(2150899716u32);
pub const ERROR_NDIS_DOT11_AP_CHANNEL_CURRENTLY_NOT_AVAILABLE: WIN32_ERROR =
    WIN32_ERROR(2150899717u32);
pub const ERROR_NDIS_DOT11_AP_BAND_CURRENTLY_NOT_AVAILABLE: WIN32_ERROR =
    WIN32_ERROR(2150899718u32);
pub const ERROR_NDIS_DOT11_AP_CHANNEL_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(2150899719u32);
pub const ERROR_NDIS_DOT11_AP_BAND_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(2150899720u32);
pub const ERROR_NDIS_INDICATION_REQUIRED: WIN32_ERROR = WIN32_ERROR(3407873u32);
pub const ERROR_NDIS_OFFLOAD_POLICY: WIN32_ERROR = WIN32_ERROR(3224637455u32);
pub const ERROR_NDIS_OFFLOAD_CONNECTION_REJECTED: WIN32_ERROR = WIN32_ERROR(3224637458u32);
pub const ERROR_NDIS_OFFLOAD_PATH_REJECTED: WIN32_ERROR = WIN32_ERROR(3224637459u32);
pub const ERROR_HV_INVALID_HYPERCALL_CODE: WIN32_ERROR = WIN32_ERROR(3224698882u32);
pub const ERROR_HV_INVALID_HYPERCALL_INPUT: WIN32_ERROR = WIN32_ERROR(3224698883u32);
pub const ERROR_HV_INVALID_ALIGNMENT: WIN32_ERROR = WIN32_ERROR(3224698884u32);
pub const ERROR_HV_INVALID_PARAMETER: WIN32_ERROR = WIN32_ERROR(3224698885u32);
pub const ERROR_HV_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(3224698886u32);
pub const ERROR_HV_INVALID_PARTITION_STATE: WIN32_ERROR = WIN32_ERROR(3224698887u32);
pub const ERROR_HV_OPERATION_DENIED: WIN32_ERROR = WIN32_ERROR(3224698888u32);
pub const ERROR_HV_UNKNOWN_PROPERTY: WIN32_ERROR = WIN32_ERROR(3224698889u32);
pub const ERROR_HV_PROPERTY_VALUE_OUT_OF_RANGE: WIN32_ERROR = WIN32_ERROR(3224698890u32);
pub const ERROR_HV_INSUFFICIENT_MEMORY: WIN32_ERROR = WIN32_ERROR(3224698891u32);
pub const ERROR_HV_PARTITION_TOO_DEEP: WIN32_ERROR = WIN32_ERROR(3224698892u32);
pub const ERROR_HV_INVALID_PARTITION_ID: WIN32_ERROR = WIN32_ERROR(3224698893u32);
pub const ERROR_HV_INVALID_VP_INDEX: WIN32_ERROR = WIN32_ERROR(3224698894u32);
pub const ERROR_HV_INVALID_PORT_ID: WIN32_ERROR = WIN32_ERROR(3224698897u32);
pub const ERROR_HV_INVALID_CONNECTION_ID: WIN32_ERROR = WIN32_ERROR(3224698898u32);
pub const ERROR_HV_INSUFFICIENT_BUFFERS: WIN32_ERROR = WIN32_ERROR(3224698899u32);
pub const ERROR_HV_NOT_ACKNOWLEDGED: WIN32_ERROR = WIN32_ERROR(3224698900u32);
pub const ERROR_HV_INVALID_VP_STATE: WIN32_ERROR = WIN32_ERROR(3224698901u32);
pub const ERROR_HV_ACKNOWLEDGED: WIN32_ERROR = WIN32_ERROR(3224698902u32);
pub const ERROR_HV_INVALID_SAVE_RESTORE_STATE: WIN32_ERROR = WIN32_ERROR(3224698903u32);
pub const ERROR_HV_INVALID_SYNIC_STATE: WIN32_ERROR = WIN32_ERROR(3224698904u32);
pub const ERROR_HV_OBJECT_IN_USE: WIN32_ERROR = WIN32_ERROR(3224698905u32);
pub const ERROR_HV_INVALID_PROXIMITY_DOMAIN_INFO: WIN32_ERROR = WIN32_ERROR(3224698906u32);
pub const ERROR_HV_NO_DATA: WIN32_ERROR = WIN32_ERROR(3224698907u32);
pub const ERROR_HV_INACTIVE: WIN32_ERROR = WIN32_ERROR(3224698908u32);
pub const ERROR_HV_NO_RESOURCES: WIN32_ERROR = WIN32_ERROR(3224698909u32);
pub const ERROR_HV_FEATURE_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(3224698910u32);
pub const ERROR_HV_INSUFFICIENT_BUFFER: WIN32_ERROR = WIN32_ERROR(3224698931u32);
pub const ERROR_HV_INSUFFICIENT_DEVICE_DOMAINS: WIN32_ERROR = WIN32_ERROR(3224698936u32);
pub const ERROR_HV_CPUID_FEATURE_VALIDATION: WIN32_ERROR = WIN32_ERROR(3224698940u32);
pub const ERROR_HV_CPUID_XSAVE_FEATURE_VALIDATION: WIN32_ERROR = WIN32_ERROR(3224698941u32);
pub const ERROR_HV_PROCESSOR_STARTUP_TIMEOUT: WIN32_ERROR = WIN32_ERROR(3224698942u32);
pub const ERROR_HV_SMX_ENABLED: WIN32_ERROR = WIN32_ERROR(3224698943u32);
pub const ERROR_HV_INVALID_LP_INDEX: WIN32_ERROR = WIN32_ERROR(3224698945u32);
pub const ERROR_HV_INVALID_REGISTER_VALUE: WIN32_ERROR = WIN32_ERROR(3224698960u32);
pub const ERROR_HV_INVALID_VTL_STATE: WIN32_ERROR = WIN32_ERROR(3224698961u32);
pub const ERROR_HV_NX_NOT_DETECTED: WIN32_ERROR = WIN32_ERROR(3224698965u32);
pub const ERROR_HV_INVALID_DEVICE_ID: WIN32_ERROR = WIN32_ERROR(3224698967u32);
pub const ERROR_HV_INVALID_DEVICE_STATE: WIN32_ERROR = WIN32_ERROR(3224698968u32);
pub const ERROR_HV_PENDING_PAGE_REQUESTS: WIN32_ERROR = WIN32_ERROR(3473497u32);
pub const ERROR_HV_PAGE_REQUEST_INVALID: WIN32_ERROR = WIN32_ERROR(3224698976u32);
pub const ERROR_HV_INVALID_CPU_GROUP_ID: WIN32_ERROR = WIN32_ERROR(3224698991u32);
pub const ERROR_HV_INVALID_CPU_GROUP_STATE: WIN32_ERROR = WIN32_ERROR(3224698992u32);
pub const ERROR_HV_OPERATION_FAILED: WIN32_ERROR = WIN32_ERROR(3224698993u32);
pub const ERROR_HV_NOT_ALLOWED_WITH_NESTED_VIRT_ACTIVE: WIN32_ERROR = WIN32_ERROR(3224698994u32);
pub const ERROR_HV_INSUFFICIENT_ROOT_MEMORY: WIN32_ERROR = WIN32_ERROR(3224698995u32);
pub const ERROR_HV_EVENT_BUFFER_ALREADY_FREED: WIN32_ERROR = WIN32_ERROR(3224698996u32);
pub const ERROR_HV_INSUFFICIENT_CONTIGUOUS_MEMORY: WIN32_ERROR = WIN32_ERROR(3224698997u32);
pub const ERROR_HV_DEVICE_NOT_IN_DOMAIN: WIN32_ERROR = WIN32_ERROR(3224698998u32);
pub const ERROR_HV_NESTED_VM_EXIT: WIN32_ERROR = WIN32_ERROR(3224698999u32);
pub const ERROR_HV_MSR_ACCESS_FAILED: WIN32_ERROR = WIN32_ERROR(3224699008u32);
pub const ERROR_HV_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(3224702976u32);
pub const ERROR_VID_DUPLICATE_HANDLER: WIN32_ERROR = WIN32_ERROR(3224829953u32);
pub const ERROR_VID_TOO_MANY_HANDLERS: WIN32_ERROR = WIN32_ERROR(3224829954u32);
pub const ERROR_VID_QUEUE_FULL: WIN32_ERROR = WIN32_ERROR(3224829955u32);
pub const ERROR_VID_HANDLER_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(3224829956u32);
pub const ERROR_VID_INVALID_OBJECT_NAME: WIN32_ERROR = WIN32_ERROR(3224829957u32);
pub const ERROR_VID_PARTITION_NAME_TOO_LONG: WIN32_ERROR = WIN32_ERROR(3224829958u32);
pub const ERROR_VID_MESSAGE_QUEUE_NAME_TOO_LONG: WIN32_ERROR = WIN32_ERROR(3224829959u32);
pub const ERROR_VID_PARTITION_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(3224829960u32);
pub const ERROR_VID_PARTITION_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(3224829961u32);
pub const ERROR_VID_PARTITION_NAME_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3224829962u32);
pub const ERROR_VID_MESSAGE_QUEUE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(3224829963u32);
pub const ERROR_VID_EXCEEDED_MBP_ENTRY_MAP_LIMIT: WIN32_ERROR = WIN32_ERROR(3224829964u32);
pub const ERROR_VID_MB_STILL_REFERENCED: WIN32_ERROR = WIN32_ERROR(3224829965u32);
pub const ERROR_VID_CHILD_GPA_PAGE_SET_CORRUPTED: WIN32_ERROR = WIN32_ERROR(3224829966u32);
pub const ERROR_VID_INVALID_NUMA_SETTINGS: WIN32_ERROR = WIN32_ERROR(3224829967u32);
pub const ERROR_VID_INVALID_NUMA_NODE_INDEX: WIN32_ERROR = WIN32_ERROR(3224829968u32);
pub const ERROR_VID_NOTIFICATION_QUEUE_ALREADY_ASSOCIATED: WIN32_ERROR = WIN32_ERROR(3224829969u32);
pub const ERROR_VID_INVALID_MEMORY_BLOCK_HANDLE: WIN32_ERROR = WIN32_ERROR(3224829970u32);
pub const ERROR_VID_PAGE_RANGE_OVERFLOW: WIN32_ERROR = WIN32_ERROR(3224829971u32);
pub const ERROR_VID_INVALID_MESSAGE_QUEUE_HANDLE: WIN32_ERROR = WIN32_ERROR(3224829972u32);
pub const ERROR_VID_INVALID_GPA_RANGE_HANDLE: WIN32_ERROR = WIN32_ERROR(3224829973u32);
pub const ERROR_VID_NO_MEMORY_BLOCK_NOTIFICATION_QUEUE: WIN32_ERROR = WIN32_ERROR(3224829974u32);
pub const ERROR_VID_MEMORY_BLOCK_LOCK_COUNT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(3224829975u32);
pub const ERROR_VID_INVALID_PPM_HANDLE: WIN32_ERROR = WIN32_ERROR(3224829976u32);
pub const ERROR_VID_MBPS_ARE_LOCKED: WIN32_ERROR = WIN32_ERROR(3224829977u32);
pub const ERROR_VID_MESSAGE_QUEUE_CLOSED: WIN32_ERROR = WIN32_ERROR(3224829978u32);
pub const ERROR_VID_VIRTUAL_PROCESSOR_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(3224829979u32);
pub const ERROR_VID_STOP_PENDING: WIN32_ERROR = WIN32_ERROR(3224829980u32);
pub const ERROR_VID_INVALID_PROCESSOR_STATE: WIN32_ERROR = WIN32_ERROR(3224829981u32);
pub const ERROR_VID_EXCEEDED_KM_CONTEXT_COUNT_LIMIT: WIN32_ERROR = WIN32_ERROR(3224829982u32);
pub const ERROR_VID_KM_INTERFACE_ALREADY_INITIALIZED: WIN32_ERROR = WIN32_ERROR(3224829983u32);
pub const ERROR_VID_MB_PROPERTY_ALREADY_SET_RESET: WIN32_ERROR = WIN32_ERROR(3224829984u32);
pub const ERROR_VID_MMIO_RANGE_DESTROYED: WIN32_ERROR = WIN32_ERROR(3224829985u32);
pub const ERROR_VID_INVALID_CHILD_GPA_PAGE_SET: WIN32_ERROR = WIN32_ERROR(3224829986u32);
pub const ERROR_VID_RESERVE_PAGE_SET_IS_BEING_USED: WIN32_ERROR = WIN32_ERROR(3224829987u32);
pub const ERROR_VID_RESERVE_PAGE_SET_TOO_SMALL: WIN32_ERROR = WIN32_ERROR(3224829988u32);
pub const ERROR_VID_MBP_ALREADY_LOCKED_USING_RESERVED_PAGE: WIN32_ERROR =
    WIN32_ERROR(3224829989u32);
pub const ERROR_VID_MBP_COUNT_EXCEEDED_LIMIT: WIN32_ERROR = WIN32_ERROR(3224829990u32);
pub const ERROR_VID_SAVED_STATE_CORRUPT: WIN32_ERROR = WIN32_ERROR(3224829991u32);
pub const ERROR_VID_SAVED_STATE_UNRECOGNIZED_ITEM: WIN32_ERROR = WIN32_ERROR(3224829992u32);
pub const ERROR_VID_SAVED_STATE_INCOMPATIBLE: WIN32_ERROR = WIN32_ERROR(3224829993u32);
pub const ERROR_VID_VTL_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(3224829994u32);
pub const ERROR_VMCOMPUTE_TERMINATED_DURING_START: WIN32_ERROR = WIN32_ERROR(3224830208u32);
pub const ERROR_VMCOMPUTE_IMAGE_MISMATCH: WIN32_ERROR = WIN32_ERROR(3224830209u32);
pub const ERROR_VMCOMPUTE_HYPERV_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(3224830210u32);
pub const ERROR_VMCOMPUTE_OPERATION_PENDING: WIN32_ERROR = WIN32_ERROR(3224830211u32);
pub const ERROR_VMCOMPUTE_TOO_MANY_NOTIFICATIONS: WIN32_ERROR = WIN32_ERROR(3224830212u32);
pub const ERROR_VMCOMPUTE_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(3224830213u32);
pub const ERROR_VMCOMPUTE_UNEXPECTED_EXIT: WIN32_ERROR = WIN32_ERROR(3224830214u32);
pub const ERROR_VMCOMPUTE_TERMINATED: WIN32_ERROR = WIN32_ERROR(3224830215u32);
pub const ERROR_VMCOMPUTE_CONNECT_FAILED: WIN32_ERROR = WIN32_ERROR(3224830216u32);
pub const ERROR_VMCOMPUTE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(3224830217u32);
pub const ERROR_VMCOMPUTE_CONNECTION_CLOSED: WIN32_ERROR = WIN32_ERROR(3224830218u32);
pub const ERROR_VMCOMPUTE_UNKNOWN_MESSAGE: WIN32_ERROR = WIN32_ERROR(3224830219u32);
pub const ERROR_VMCOMPUTE_UNSUPPORTED_PROTOCOL_VERSION: WIN32_ERROR = WIN32_ERROR(3224830220u32);
pub const ERROR_VMCOMPUTE_INVALID_JSON: WIN32_ERROR = WIN32_ERROR(3224830221u32);
pub const ERROR_VMCOMPUTE_SYSTEM_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3224830222u32);
pub const ERROR_VMCOMPUTE_SYSTEM_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(3224830223u32);
pub const ERROR_VMCOMPUTE_SYSTEM_ALREADY_STOPPED: WIN32_ERROR = WIN32_ERROR(3224830224u32);
pub const ERROR_VMCOMPUTE_PROTOCOL_ERROR: WIN32_ERROR = WIN32_ERROR(3224830225u32);
pub const ERROR_VMCOMPUTE_INVALID_LAYER: WIN32_ERROR = WIN32_ERROR(3224830226u32);
pub const ERROR_VMCOMPUTE_WINDOWS_INSIDER_REQUIRED: WIN32_ERROR = WIN32_ERROR(3224830227u32);
pub const ERROR_VNET_VIRTUAL_SWITCH_NAME_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3224830464u32);
pub const ERROR_VID_REMOTE_NODE_PARENT_GPA_PAGES_USED: WIN32_ERROR = WIN32_ERROR(2151088129u32);
pub const ERROR_VSMB_SAVED_STATE_FILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3224830976u32);
pub const ERROR_VSMB_SAVED_STATE_CORRUPT: WIN32_ERROR = WIN32_ERROR(3224830977u32);
pub const ERROR_VOLMGR_INCOMPLETE_REGENERATION: WIN32_ERROR = WIN32_ERROR(2151153665u32);
pub const ERROR_VOLMGR_INCOMPLETE_DISK_MIGRATION: WIN32_ERROR = WIN32_ERROR(2151153666u32);
pub const ERROR_VOLMGR_DATABASE_FULL: WIN32_ERROR = WIN32_ERROR(3224895489u32);
pub const ERROR_VOLMGR_DISK_CONFIGURATION_CORRUPTED: WIN32_ERROR = WIN32_ERROR(3224895490u32);
pub const ERROR_VOLMGR_DISK_CONFIGURATION_NOT_IN_SYNC: WIN32_ERROR = WIN32_ERROR(3224895491u32);
pub const ERROR_VOLMGR_PACK_CONFIG_UPDATE_FAILED: WIN32_ERROR = WIN32_ERROR(3224895492u32);
pub const ERROR_VOLMGR_DISK_CONTAINS_NON_SIMPLE_VOLUME: WIN32_ERROR = WIN32_ERROR(3224895493u32);
pub const ERROR_VOLMGR_DISK_DUPLICATE: WIN32_ERROR = WIN32_ERROR(3224895494u32);
pub const ERROR_VOLMGR_DISK_DYNAMIC: WIN32_ERROR = WIN32_ERROR(3224895495u32);
pub const ERROR_VOLMGR_DISK_ID_INVALID: WIN32_ERROR = WIN32_ERROR(3224895496u32);
pub const ERROR_VOLMGR_DISK_INVALID: WIN32_ERROR = WIN32_ERROR(3224895497u32);
pub const ERROR_VOLMGR_DISK_LAST_VOTER: WIN32_ERROR = WIN32_ERROR(3224895498u32);
pub const ERROR_VOLMGR_DISK_LAYOUT_INVALID: WIN32_ERROR = WIN32_ERROR(3224895499u32);
pub const ERROR_VOLMGR_DISK_LAYOUT_NON_BASIC_BETWEEN_BASIC_PARTITIONS: WIN32_ERROR =
    WIN32_ERROR(3224895500u32);
pub const ERROR_VOLMGR_DISK_LAYOUT_NOT_CYLINDER_ALIGNED: WIN32_ERROR = WIN32_ERROR(3224895501u32);
pub const ERROR_VOLMGR_DISK_LAYOUT_PARTITIONS_TOO_SMALL: WIN32_ERROR = WIN32_ERROR(3224895502u32);
pub const ERROR_VOLMGR_DISK_LAYOUT_PRIMARY_BETWEEN_LOGICAL_PARTITIONS: WIN32_ERROR =
    WIN32_ERROR(3224895503u32);
pub const ERROR_VOLMGR_DISK_LAYOUT_TOO_MANY_PARTITIONS: WIN32_ERROR = WIN32_ERROR(3224895504u32);
pub const ERROR_VOLMGR_DISK_MISSING: WIN32_ERROR = WIN32_ERROR(3224895505u32);
pub const ERROR_VOLMGR_DISK_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(3224895506u32);
pub const ERROR_VOLMGR_DISK_NOT_ENOUGH_SPACE: WIN32_ERROR = WIN32_ERROR(3224895507u32);
pub const ERROR_VOLMGR_DISK_REVECTORING_FAILED: WIN32_ERROR = WIN32_ERROR(3224895508u32);
pub const ERROR_VOLMGR_DISK_SECTOR_SIZE_INVALID: WIN32_ERROR = WIN32_ERROR(3224895509u32);
pub const ERROR_VOLMGR_DISK_SET_NOT_CONTAINED: WIN32_ERROR = WIN32_ERROR(3224895510u32);
pub const ERROR_VOLMGR_DISK_USED_BY_MULTIPLE_MEMBERS: WIN32_ERROR = WIN32_ERROR(3224895511u32);
pub const ERROR_VOLMGR_DISK_USED_BY_MULTIPLE_PLEXES: WIN32_ERROR = WIN32_ERROR(3224895512u32);
pub const ERROR_VOLMGR_DYNAMIC_DISK_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(3224895513u32);
pub const ERROR_VOLMGR_EXTENT_ALREADY_USED: WIN32_ERROR = WIN32_ERROR(3224895514u32);
pub const ERROR_VOLMGR_EXTENT_NOT_CONTIGUOUS: WIN32_ERROR = WIN32_ERROR(3224895515u32);
pub const ERROR_VOLMGR_EXTENT_NOT_IN_PUBLIC_REGION: WIN32_ERROR = WIN32_ERROR(3224895516u32);
pub const ERROR_VOLMGR_EXTENT_NOT_SECTOR_ALIGNED: WIN32_ERROR = WIN32_ERROR(3224895517u32);
pub const ERROR_VOLMGR_EXTENT_OVERLAPS_EBR_PARTITION: WIN32_ERROR = WIN32_ERROR(3224895518u32);
pub const ERROR_VOLMGR_EXTENT_VOLUME_LENGTHS_DO_NOT_MATCH: WIN32_ERROR = WIN32_ERROR(3224895519u32);
pub const ERROR_VOLMGR_FAULT_TOLERANT_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(3224895520u32);
pub const ERROR_VOLMGR_INTERLEAVE_LENGTH_INVALID: WIN32_ERROR = WIN32_ERROR(3224895521u32);
pub const ERROR_VOLMGR_MAXIMUM_REGISTERED_USERS: WIN32_ERROR = WIN32_ERROR(3224895522u32);
pub const ERROR_VOLMGR_MEMBER_IN_SYNC: WIN32_ERROR = WIN32_ERROR(3224895523u32);
pub const ERROR_VOLMGR_MEMBER_INDEX_DUPLICATE: WIN32_ERROR = WIN32_ERROR(3224895524u32);
pub const ERROR_VOLMGR_MEMBER_INDEX_INVALID: WIN32_ERROR = WIN32_ERROR(3224895525u32);
pub const ERROR_VOLMGR_MEMBER_MISSING: WIN32_ERROR = WIN32_ERROR(3224895526u32);
pub const ERROR_VOLMGR_MEMBER_NOT_DETACHED: WIN32_ERROR = WIN32_ERROR(3224895527u32);
pub const ERROR_VOLMGR_MEMBER_REGENERATING: WIN32_ERROR = WIN32_ERROR(3224895528u32);
pub const ERROR_VOLMGR_ALL_DISKS_FAILED: WIN32_ERROR = WIN32_ERROR(3224895529u32);
pub const ERROR_VOLMGR_NO_REGISTERED_USERS: WIN32_ERROR = WIN32_ERROR(3224895530u32);
pub const ERROR_VOLMGR_NO_SUCH_USER: WIN32_ERROR = WIN32_ERROR(3224895531u32);
pub const ERROR_VOLMGR_NOTIFICATION_RESET: WIN32_ERROR = WIN32_ERROR(3224895532u32);
pub const ERROR_VOLMGR_NUMBER_OF_MEMBERS_INVALID: WIN32_ERROR = WIN32_ERROR(3224895533u32);
pub const ERROR_VOLMGR_NUMBER_OF_PLEXES_INVALID: WIN32_ERROR = WIN32_ERROR(3224895534u32);
pub const ERROR_VOLMGR_PACK_DUPLICATE: WIN32_ERROR = WIN32_ERROR(3224895535u32);
pub const ERROR_VOLMGR_PACK_ID_INVALID: WIN32_ERROR = WIN32_ERROR(3224895536u32);
pub const ERROR_VOLMGR_PACK_INVALID: WIN32_ERROR = WIN32_ERROR(3224895537u32);
pub const ERROR_VOLMGR_PACK_NAME_INVALID: WIN32_ERROR = WIN32_ERROR(3224895538u32);
pub const ERROR_VOLMGR_PACK_OFFLINE: WIN32_ERROR = WIN32_ERROR(3224895539u32);
pub const ERROR_VOLMGR_PACK_HAS_QUORUM: WIN32_ERROR = WIN32_ERROR(3224895540u32);
pub const ERROR_VOLMGR_PACK_WITHOUT_QUORUM: WIN32_ERROR = WIN32_ERROR(3224895541u32);
pub const ERROR_VOLMGR_PARTITION_STYLE_INVALID: WIN32_ERROR = WIN32_ERROR(3224895542u32);
pub const ERROR_VOLMGR_PARTITION_UPDATE_FAILED: WIN32_ERROR = WIN32_ERROR(3224895543u32);
pub const ERROR_VOLMGR_PLEX_IN_SYNC: WIN32_ERROR = WIN32_ERROR(3224895544u32);
pub const ERROR_VOLMGR_PLEX_INDEX_DUPLICATE: WIN32_ERROR = WIN32_ERROR(3224895545u32);
pub const ERROR_VOLMGR_PLEX_INDEX_INVALID: WIN32_ERROR = WIN32_ERROR(3224895546u32);
pub const ERROR_VOLMGR_PLEX_LAST_ACTIVE: WIN32_ERROR = WIN32_ERROR(3224895547u32);
pub const ERROR_VOLMGR_PLEX_MISSING: WIN32_ERROR = WIN32_ERROR(3224895548u32);
pub const ERROR_VOLMGR_PLEX_REGENERATING: WIN32_ERROR = WIN32_ERROR(3224895549u32);
pub const ERROR_VOLMGR_PLEX_TYPE_INVALID: WIN32_ERROR = WIN32_ERROR(3224895550u32);
pub const ERROR_VOLMGR_PLEX_NOT_RAID5: WIN32_ERROR = WIN32_ERROR(3224895551u32);
pub const ERROR_VOLMGR_PLEX_NOT_SIMPLE: WIN32_ERROR = WIN32_ERROR(3224895552u32);
pub const ERROR_VOLMGR_STRUCTURE_SIZE_INVALID: WIN32_ERROR = WIN32_ERROR(3224895553u32);
pub const ERROR_VOLMGR_TOO_MANY_NOTIFICATION_REQUESTS: WIN32_ERROR = WIN32_ERROR(3224895554u32);
pub const ERROR_VOLMGR_TRANSACTION_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(3224895555u32);
pub const ERROR_VOLMGR_UNEXPECTED_DISK_LAYOUT_CHANGE: WIN32_ERROR = WIN32_ERROR(3224895556u32);
pub const ERROR_VOLMGR_VOLUME_CONTAINS_MISSING_DISK: WIN32_ERROR = WIN32_ERROR(3224895557u32);
pub const ERROR_VOLMGR_VOLUME_ID_INVALID: WIN32_ERROR = WIN32_ERROR(3224895558u32);
pub const ERROR_VOLMGR_VOLUME_LENGTH_INVALID: WIN32_ERROR = WIN32_ERROR(3224895559u32);
pub const ERROR_VOLMGR_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE: WIN32_ERROR =
    WIN32_ERROR(3224895560u32);
pub const ERROR_VOLMGR_VOLUME_NOT_MIRRORED: WIN32_ERROR = WIN32_ERROR(3224895561u32);
pub const ERROR_VOLMGR_VOLUME_NOT_RETAINED: WIN32_ERROR = WIN32_ERROR(3224895562u32);
pub const ERROR_VOLMGR_VOLUME_OFFLINE: WIN32_ERROR = WIN32_ERROR(3224895563u32);
pub const ERROR_VOLMGR_VOLUME_RETAINED: WIN32_ERROR = WIN32_ERROR(3224895564u32);
pub const ERROR_VOLMGR_NUMBER_OF_EXTENTS_INVALID: WIN32_ERROR = WIN32_ERROR(3224895565u32);
pub const ERROR_VOLMGR_DIFFERENT_SECTOR_SIZE: WIN32_ERROR = WIN32_ERROR(3224895566u32);
pub const ERROR_VOLMGR_BAD_BOOT_DISK: WIN32_ERROR = WIN32_ERROR(3224895567u32);
pub const ERROR_VOLMGR_PACK_CONFIG_OFFLINE: WIN32_ERROR = WIN32_ERROR(3224895568u32);
pub const ERROR_VOLMGR_PACK_CONFIG_ONLINE: WIN32_ERROR = WIN32_ERROR(3224895569u32);
pub const ERROR_VOLMGR_NOT_PRIMARY_PACK: WIN32_ERROR = WIN32_ERROR(3224895570u32);
pub const ERROR_VOLMGR_PACK_LOG_UPDATE_FAILED: WIN32_ERROR = WIN32_ERROR(3224895571u32);
pub const ERROR_VOLMGR_NUMBER_OF_DISKS_IN_PLEX_INVALID: WIN32_ERROR = WIN32_ERROR(3224895572u32);
pub const ERROR_VOLMGR_NUMBER_OF_DISKS_IN_MEMBER_INVALID: WIN32_ERROR = WIN32_ERROR(3224895573u32);
pub const ERROR_VOLMGR_VOLUME_MIRRORED: WIN32_ERROR = WIN32_ERROR(3224895574u32);
pub const ERROR_VOLMGR_PLEX_NOT_SIMPLE_SPANNED: WIN32_ERROR = WIN32_ERROR(3224895575u32);
pub const ERROR_VOLMGR_NO_VALID_LOG_COPIES: WIN32_ERROR = WIN32_ERROR(3224895576u32);
pub const ERROR_VOLMGR_PRIMARY_PACK_PRESENT: WIN32_ERROR = WIN32_ERROR(3224895577u32);
pub const ERROR_VOLMGR_NUMBER_OF_DISKS_INVALID: WIN32_ERROR = WIN32_ERROR(3224895578u32);
pub const ERROR_VOLMGR_MIRROR_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(3224895579u32);
pub const ERROR_VOLMGR_RAID5_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(3224895580u32);
pub const ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED: WIN32_ERROR = WIN32_ERROR(2151219201u32);
pub const ERROR_BCD_TOO_MANY_ELEMENTS: WIN32_ERROR = WIN32_ERROR(3224961026u32);
pub const ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED: WIN32_ERROR = WIN32_ERROR(2151219203u32);
pub const ERROR_VHD_DRIVE_FOOTER_MISSING: WIN32_ERROR = WIN32_ERROR(3225026561u32);
pub const ERROR_VHD_DRIVE_FOOTER_CHECKSUM_MISMATCH: WIN32_ERROR = WIN32_ERROR(3225026562u32);
pub const ERROR_VHD_DRIVE_FOOTER_CORRUPT: WIN32_ERROR = WIN32_ERROR(3225026563u32);
pub const ERROR_VHD_FORMAT_UNKNOWN: WIN32_ERROR = WIN32_ERROR(3225026564u32);
pub const ERROR_VHD_FORMAT_UNSUPPORTED_VERSION: WIN32_ERROR = WIN32_ERROR(3225026565u32);
pub const ERROR_VHD_SPARSE_HEADER_CHECKSUM_MISMATCH: WIN32_ERROR = WIN32_ERROR(3225026566u32);
pub const ERROR_VHD_SPARSE_HEADER_UNSUPPORTED_VERSION: WIN32_ERROR = WIN32_ERROR(3225026567u32);
pub const ERROR_VHD_SPARSE_HEADER_CORRUPT: WIN32_ERROR = WIN32_ERROR(3225026568u32);
pub const ERROR_VHD_BLOCK_ALLOCATION_FAILURE: WIN32_ERROR = WIN32_ERROR(3225026569u32);
pub const ERROR_VHD_BLOCK_ALLOCATION_TABLE_CORRUPT: WIN32_ERROR = WIN32_ERROR(3225026570u32);
pub const ERROR_VHD_INVALID_BLOCK_SIZE: WIN32_ERROR = WIN32_ERROR(3225026571u32);
pub const ERROR_VHD_BITMAP_MISMATCH: WIN32_ERROR = WIN32_ERROR(3225026572u32);
pub const ERROR_VHD_PARENT_VHD_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3225026573u32);
pub const ERROR_VHD_CHILD_PARENT_ID_MISMATCH: WIN32_ERROR = WIN32_ERROR(3225026574u32);
pub const ERROR_VHD_CHILD_PARENT_TIMESTAMP_MISMATCH: WIN32_ERROR = WIN32_ERROR(3225026575u32);
pub const ERROR_VHD_METADATA_READ_FAILURE: WIN32_ERROR = WIN32_ERROR(3225026576u32);
pub const ERROR_VHD_METADATA_WRITE_FAILURE: WIN32_ERROR = WIN32_ERROR(3225026577u32);
pub const ERROR_VHD_INVALID_SIZE: WIN32_ERROR = WIN32_ERROR(3225026578u32);
pub const ERROR_VHD_INVALID_FILE_SIZE: WIN32_ERROR = WIN32_ERROR(3225026579u32);
pub const ERROR_VIRTDISK_PROVIDER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3225026580u32);
pub const ERROR_VIRTDISK_NOT_VIRTUAL_DISK: WIN32_ERROR = WIN32_ERROR(3225026581u32);
pub const ERROR_VHD_PARENT_VHD_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(3225026582u32);
pub const ERROR_VHD_CHILD_PARENT_SIZE_MISMATCH: WIN32_ERROR = WIN32_ERROR(3225026583u32);
pub const ERROR_VHD_DIFFERENCING_CHAIN_CYCLE_DETECTED: WIN32_ERROR = WIN32_ERROR(3225026584u32);
pub const ERROR_VHD_DIFFERENCING_CHAIN_ERROR_IN_PARENT: WIN32_ERROR = WIN32_ERROR(3225026585u32);
pub const ERROR_VIRTUAL_DISK_LIMITATION: WIN32_ERROR = WIN32_ERROR(3225026586u32);
pub const ERROR_VHD_INVALID_TYPE: WIN32_ERROR = WIN32_ERROR(3225026587u32);
pub const ERROR_VHD_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(3225026588u32);
pub const ERROR_VIRTDISK_UNSUPPORTED_DISK_SECTOR_SIZE: WIN32_ERROR = WIN32_ERROR(3225026589u32);
pub const ERROR_VIRTDISK_DISK_ALREADY_OWNED: WIN32_ERROR = WIN32_ERROR(3225026590u32);
pub const ERROR_VIRTDISK_DISK_ONLINE_AND_WRITABLE: WIN32_ERROR = WIN32_ERROR(3225026591u32);
pub const ERROR_CTLOG_TRACKING_NOT_INITIALIZED: WIN32_ERROR = WIN32_ERROR(3225026592u32);
pub const ERROR_CTLOG_LOGFILE_SIZE_EXCEEDED_MAXSIZE: WIN32_ERROR = WIN32_ERROR(3225026593u32);
pub const ERROR_CTLOG_VHD_CHANGED_OFFLINE: WIN32_ERROR = WIN32_ERROR(3225026594u32);
pub const ERROR_CTLOG_INVALID_TRACKING_STATE: WIN32_ERROR = WIN32_ERROR(3225026595u32);
pub const ERROR_CTLOG_INCONSISTENT_TRACKING_FILE: WIN32_ERROR = WIN32_ERROR(3225026596u32);
pub const ERROR_VHD_RESIZE_WOULD_TRUNCATE_DATA: WIN32_ERROR = WIN32_ERROR(3225026597u32);
pub const ERROR_VHD_COULD_NOT_COMPUTE_MINIMUM_VIRTUAL_SIZE: WIN32_ERROR =
    WIN32_ERROR(3225026598u32);
pub const ERROR_VHD_ALREADY_AT_OR_BELOW_MINIMUM_VIRTUAL_SIZE: WIN32_ERROR =
    WIN32_ERROR(3225026599u32);
pub const ERROR_VHD_METADATA_FULL: WIN32_ERROR = WIN32_ERROR(3225026600u32);
pub const ERROR_VHD_INVALID_CHANGE_TRACKING_ID: WIN32_ERROR = WIN32_ERROR(3225026601u32);
pub const ERROR_VHD_CHANGE_TRACKING_DISABLED: WIN32_ERROR = WIN32_ERROR(3225026602u32);
pub const ERROR_VHD_MISSING_CHANGE_TRACKING_INFORMATION: WIN32_ERROR = WIN32_ERROR(3225026608u32);
pub const ERROR_QUERY_STORAGE_ERROR: WIN32_ERROR = WIN32_ERROR(2151284737u32);
impl ::core::marker::Copy for WIN32_ERROR {}
impl ::core::clone::Clone for WIN32_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WIN32_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WIN32_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN32_ERROR").field(&self.0).finish()
    }
}
impl FromIntoMemory for WIN32_ERROR {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<u32>()
    }
}
pub const WINCODEC_ERR_ALREADYLOCKED: crate::core::HRESULT = crate::core::HRESULT(-2003292403i32);
pub const WINCODEC_ERR_BADHEADER: crate::core::HRESULT = crate::core::HRESULT(-2003292319i32);
pub const WINCODEC_ERR_BADIMAGE: crate::core::HRESULT = crate::core::HRESULT(-2003292320i32);
pub const WINCODEC_ERR_BADMETADATAHEADER: crate::core::HRESULT =
    crate::core::HRESULT(-2003292317i32);
pub const WINCODEC_ERR_BADSTREAMDATA: crate::core::HRESULT = crate::core::HRESULT(-2003292304i32);
pub const WINCODEC_ERR_CODECNOTHUMBNAIL: crate::core::HRESULT =
    crate::core::HRESULT(-2003292348i32);
pub const WINCODEC_ERR_CODECPRESENT: crate::core::HRESULT = crate::core::HRESULT(-2003292349i32);
pub const WINCODEC_ERR_CODECTOOMANYSCANLINES: crate::core::HRESULT =
    crate::core::HRESULT(-2003292346i32);
pub const WINCODEC_ERR_COMPONENTINITIALIZEFAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-2003292277i32);
pub const WINCODEC_ERR_COMPONENTNOTFOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2003292336i32);
pub const WINCODEC_ERR_DUPLICATEMETADATAPRESENT: crate::core::HRESULT =
    crate::core::HRESULT(-2003292275i32);
pub const WINCODEC_ERR_FRAMEMISSING: crate::core::HRESULT = crate::core::HRESULT(-2003292318i32);
pub const WINCODEC_ERR_IMAGESIZEOUTOFRANGE: crate::core::HRESULT =
    crate::core::HRESULT(-2003292335i32);
pub const WINCODEC_ERR_INSUFFICIENTBUFFER: crate::core::HRESULT =
    crate::core::HRESULT(-2003292276i32);
pub const WINCODEC_ERR_INTERNALERROR: crate::core::HRESULT = crate::core::HRESULT(-2003292344i32);
pub const WINCODEC_ERR_INVALIDJPEGSCANINDEX: crate::core::HRESULT =
    crate::core::HRESULT(-2003292266i32);
pub const WINCODEC_ERR_INVALIDPROGRESSIVELEVEL: crate::core::HRESULT =
    crate::core::HRESULT(-2003292267i32);
pub const WINCODEC_ERR_INVALIDQUERYCHARACTER: crate::core::HRESULT =
    crate::core::HRESULT(-2003292269i32);
pub const WINCODEC_ERR_INVALIDQUERYREQUEST: crate::core::HRESULT =
    crate::core::HRESULT(-2003292272i32);
pub const WINCODEC_ERR_INVALIDREGISTRATION: crate::core::HRESULT =
    crate::core::HRESULT(-2003292278i32);
pub const WINCODEC_ERR_NOTINITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2003292404i32);
pub const WINCODEC_ERR_PALETTEUNAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2003292347i32);
pub const WINCODEC_ERR_PROPERTYNOTFOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2003292352i32);
pub const WINCODEC_ERR_PROPERTYNOTSUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2003292351i32);
pub const WINCODEC_ERR_PROPERTYSIZE: crate::core::HRESULT = crate::core::HRESULT(-2003292350i32);
pub const WINCODEC_ERR_PROPERTYUNEXPECTEDTYPE: crate::core::HRESULT =
    crate::core::HRESULT(-2003292274i32);
pub const WINCODEC_ERR_REQUESTONLYVALIDATMETADATAROOT: crate::core::HRESULT =
    crate::core::HRESULT(-2003292270i32);
pub const WINCODEC_ERR_SOURCERECTDOESNOTMATCHDIMENSIONS: crate::core::HRESULT =
    crate::core::HRESULT(-2003292343i32);
pub const WINCODEC_ERR_STREAMNOTAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2003292301i32);
pub const WINCODEC_ERR_STREAMREAD: crate::core::HRESULT = crate::core::HRESULT(-2003292302i32);
pub const WINCODEC_ERR_STREAMWRITE: crate::core::HRESULT = crate::core::HRESULT(-2003292303i32);
pub const WINCODEC_ERR_TOOMUCHMETADATA: crate::core::HRESULT = crate::core::HRESULT(-2003292334i32);
pub const WINCODEC_ERR_UNEXPECTEDMETADATATYPE: crate::core::HRESULT =
    crate::core::HRESULT(-2003292271i32);
pub const WINCODEC_ERR_UNEXPECTEDSIZE: crate::core::HRESULT = crate::core::HRESULT(-2003292273i32);
pub const WINCODEC_ERR_UNKNOWNIMAGEFORMAT: crate::core::HRESULT =
    crate::core::HRESULT(-2003292409i32);
pub const WINCODEC_ERR_UNSUPPORTEDOPERATION: crate::core::HRESULT =
    crate::core::HRESULT(-2003292287i32);
pub const WINCODEC_ERR_UNSUPPORTEDPIXELFORMAT: crate::core::HRESULT =
    crate::core::HRESULT(-2003292288i32);
pub const WINCODEC_ERR_UNSUPPORTEDVERSION: crate::core::HRESULT =
    crate::core::HRESULT(-2003292405i32);
pub const WINCODEC_ERR_VALUEOUTOFRANGE: crate::core::HRESULT = crate::core::HRESULT(-2003292411i32);
pub const WINCODEC_ERR_WIN32ERROR: crate::core::HRESULT = crate::core::HRESULT(-2003292268i32);
pub const WINCODEC_ERR_WRONGSTATE: crate::core::HRESULT = crate::core::HRESULT(-2003292412i32);
pub const WININET_E_ASYNC_THREAD_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2147012849i32);
pub const WININET_E_BAD_AUTO_PROXY_SCRIPT: crate::core::HRESULT =
    crate::core::HRESULT(-2147012730i32);
pub const WININET_E_BAD_OPTION_LENGTH: crate::core::HRESULT = crate::core::HRESULT(-2147012886i32);
pub const WININET_E_BAD_REGISTRY_PARAMETER: crate::core::HRESULT =
    crate::core::HRESULT(-2147012874i32);
pub const WININET_E_CANNOT_CONNECT: crate::core::HRESULT = crate::core::HRESULT(-2147012867i32);
pub const WININET_E_CHG_POST_IS_NON_SECURE: crate::core::HRESULT =
    crate::core::HRESULT(-2147012854i32);
pub const WININET_E_CLIENT_AUTH_CERT_NEEDED: crate::core::HRESULT =
    crate::core::HRESULT(-2147012852i32);
pub const WININET_E_CLIENT_AUTH_NOT_SETUP: crate::core::HRESULT =
    crate::core::HRESULT(-2147012850i32);
pub const WININET_E_CONNECTION_ABORTED: crate::core::HRESULT = crate::core::HRESULT(-2147012866i32);
pub const WININET_E_CONNECTION_RESET: crate::core::HRESULT = crate::core::HRESULT(-2147012865i32);
pub const WININET_E_COOKIE_DECLINED: crate::core::HRESULT = crate::core::HRESULT(-2147012734i32);
pub const WININET_E_COOKIE_NEEDS_CONFIRMATION: crate::core::HRESULT =
    crate::core::HRESULT(-2147012735i32);
pub const WININET_E_DECODING_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2147012721i32);
pub const WININET_E_DIALOG_PENDING: crate::core::HRESULT = crate::core::HRESULT(-2147012847i32);
pub const WININET_E_DISCONNECTED: crate::core::HRESULT = crate::core::HRESULT(-2147012733i32);
pub const WININET_E_DOWNLEVEL_SERVER: crate::core::HRESULT = crate::core::HRESULT(-2147012745i32);
pub const WININET_E_EXTENDED_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2147012893i32);
pub const WININET_E_FAILED_DUETOSECURITYCHECK: crate::core::HRESULT =
    crate::core::HRESULT(-2147012725i32);
pub const WININET_E_FORCE_RETRY: crate::core::HRESULT = crate::core::HRESULT(-2147012864i32);
pub const WININET_E_HANDLE_EXISTS: crate::core::HRESULT = crate::core::HRESULT(-2147012860i32);
pub const WININET_E_HEADER_ALREADY_EXISTS: crate::core::HRESULT =
    crate::core::HRESULT(-2147012741i32);
pub const WININET_E_HEADER_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2147012746i32);
pub const WININET_E_HTTPS_HTTP_SUBMIT_REDIR: crate::core::HRESULT =
    crate::core::HRESULT(-2147012844i32);
pub const WININET_E_HTTPS_TO_HTTP_ON_REDIR: crate::core::HRESULT =
    crate::core::HRESULT(-2147012856i32);
pub const WININET_E_HTTP_TO_HTTPS_ON_REDIR: crate::core::HRESULT =
    crate::core::HRESULT(-2147012857i32);
pub const WININET_E_INCORRECT_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-2147012869i32);
pub const WININET_E_INCORRECT_HANDLE_STATE: crate::core::HRESULT =
    crate::core::HRESULT(-2147012877i32);
pub const WININET_E_INCORRECT_HANDLE_TYPE: crate::core::HRESULT =
    crate::core::HRESULT(-2147012878i32);
pub const WININET_E_INCORRECT_PASSWORD: crate::core::HRESULT = crate::core::HRESULT(-2147012882i32);
pub const WININET_E_INCORRECT_USER_NAME: crate::core::HRESULT =
    crate::core::HRESULT(-2147012883i32);
pub const WININET_E_INTERNAL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2147012892i32);
pub const WININET_E_INVALID_CA: crate::core::HRESULT = crate::core::HRESULT(-2147012851i32);
pub const WININET_E_INVALID_HEADER: crate::core::HRESULT = crate::core::HRESULT(-2147012743i32);
pub const WININET_E_INVALID_OPERATION: crate::core::HRESULT = crate::core::HRESULT(-2147012880i32);
pub const WININET_E_INVALID_OPTION: crate::core::HRESULT = crate::core::HRESULT(-2147012887i32);
pub const WININET_E_INVALID_PROXY_REQUEST: crate::core::HRESULT =
    crate::core::HRESULT(-2147012863i32);
pub const WININET_E_INVALID_QUERY_REQUEST: crate::core::HRESULT =
    crate::core::HRESULT(-2147012742i32);
pub const WININET_E_INVALID_SERVER_RESPONSE: crate::core::HRESULT =
    crate::core::HRESULT(-2147012744i32);
pub const WININET_E_INVALID_URL: crate::core::HRESULT = crate::core::HRESULT(-2147012891i32);
pub const WININET_E_ITEM_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2147012868i32);
pub const WININET_E_LOGIN_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2147012881i32);
pub const WININET_E_LOGIN_FAILURE_DISPLAY_ENTITY_BODY: crate::core::HRESULT =
    crate::core::HRESULT(-2147012722i32);
pub const WININET_E_MIXED_SECURITY: crate::core::HRESULT = crate::core::HRESULT(-2147012855i32);
pub const WININET_E_NAME_NOT_RESOLVED: crate::core::HRESULT = crate::core::HRESULT(-2147012889i32);
pub const WININET_E_NEED_UI: crate::core::HRESULT = crate::core::HRESULT(-2147012862i32);
pub const WININET_E_NOT_INITIALIZED: crate::core::HRESULT = crate::core::HRESULT(-2147012724i32);
pub const WININET_E_NOT_PROXY_REQUEST: crate::core::HRESULT = crate::core::HRESULT(-2147012876i32);
pub const WININET_E_NOT_REDIRECTED: crate::core::HRESULT = crate::core::HRESULT(-2147012736i32);
pub const WININET_E_NO_CALLBACK: crate::core::HRESULT = crate::core::HRESULT(-2147012871i32);
pub const WININET_E_NO_CONTEXT: crate::core::HRESULT = crate::core::HRESULT(-2147012872i32);
pub const WININET_E_NO_DIRECT_ACCESS: crate::core::HRESULT = crate::core::HRESULT(-2147012873i32);
pub const WININET_E_NO_NEW_CONTAINERS: crate::core::HRESULT = crate::core::HRESULT(-2147012845i32);
pub const WININET_E_OPERATION_CANCELLED: crate::core::HRESULT =
    crate::core::HRESULT(-2147012879i32);
pub const WININET_E_OPTION_NOT_SETTABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2147012885i32);
pub const WININET_E_OUT_OF_HANDLES: crate::core::HRESULT = crate::core::HRESULT(-2147012895i32);
pub const WININET_E_POST_IS_NON_SECURE: crate::core::HRESULT = crate::core::HRESULT(-2147012853i32);
pub const WININET_E_PROTOCOL_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2147012888i32);
pub const WININET_E_PROXY_SERVER_UNREACHABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2147012731i32);
pub const WININET_E_REDIRECT_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2147012740i32);
pub const WININET_E_REDIRECT_NEEDS_CONFIRMATION: crate::core::HRESULT =
    crate::core::HRESULT(-2147012728i32);
pub const WININET_E_REDIRECT_SCHEME_CHANGE: crate::core::HRESULT =
    crate::core::HRESULT(-2147012848i32);
pub const WININET_E_REGISTRY_VALUE_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2147012875i32);
pub const WININET_E_REQUEST_PENDING: crate::core::HRESULT = crate::core::HRESULT(-2147012870i32);
pub const WININET_E_RETRY_DIALOG: crate::core::HRESULT = crate::core::HRESULT(-2147012846i32);
pub const WININET_E_SECURITY_CHANNEL_ERROR: crate::core::HRESULT =
    crate::core::HRESULT(-2147012739i32);
pub const WININET_E_SEC_CERT_CN_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2147012858i32);
pub const WININET_E_SEC_CERT_DATE_INVALID: crate::core::HRESULT =
    crate::core::HRESULT(-2147012859i32);
pub const WININET_E_SEC_CERT_ERRORS: crate::core::HRESULT = crate::core::HRESULT(-2147012841i32);
pub const WININET_E_SEC_CERT_REVOKED: crate::core::HRESULT = crate::core::HRESULT(-2147012726i32);
pub const WININET_E_SEC_CERT_REV_FAILED: crate::core::HRESULT =
    crate::core::HRESULT(-2147012839i32);
pub const WININET_E_SEC_INVALID_CERT: crate::core::HRESULT = crate::core::HRESULT(-2147012727i32);
pub const WININET_E_SERVER_UNREACHABLE: crate::core::HRESULT = crate::core::HRESULT(-2147012732i32);
pub const WININET_E_SHUTDOWN: crate::core::HRESULT = crate::core::HRESULT(-2147012884i32);
pub const WININET_E_TCPIP_NOT_INSTALLED: crate::core::HRESULT =
    crate::core::HRESULT(-2147012737i32);
pub const WININET_E_TIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2147012894i32);
pub const WININET_E_UNABLE_TO_CACHE_FILE: crate::core::HRESULT =
    crate::core::HRESULT(-2147012738i32);
pub const WININET_E_UNABLE_TO_DOWNLOAD_SCRIPT: crate::core::HRESULT =
    crate::core::HRESULT(-2147012729i32);
pub const WININET_E_UNRECOGNIZED_SCHEME: crate::core::HRESULT =
    crate::core::HRESULT(-2147012890i32);
pub const WINML_ERR_INVALID_BINDING: crate::core::HRESULT = crate::core::HRESULT(-2003828734i32);
pub const WINML_ERR_INVALID_DEVICE: crate::core::HRESULT = crate::core::HRESULT(-2003828735i32);
pub const WINML_ERR_SIZE_MISMATCH: crate::core::HRESULT = crate::core::HRESULT(-2003828732i32);
pub const WINML_ERR_VALUE_NOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2003828733i32);
pub const WINVER: u32 = 1280u32;
pub const WINVER_MAXVER: u32 = 2560u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WPARAM(pub PtrRepr);
impl WPARAM {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for WPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for WPARAM {}
impl ::core::fmt::Debug for WPARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPARAM").field(&self.0).finish()
    }
}
impl FromIntoMemory for WPARAM {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<PtrRepr as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        std::mem::size_of::<PtrRepr>()
    }
}
pub const WPN_E_ACCESS_DENIED: crate::core::HRESULT = crate::core::HRESULT(-2143420137i32);
pub const WPN_E_ALL_URL_NOT_COMPLETED: crate::core::HRESULT = crate::core::HRESULT(-2143419901i32);
pub const WPN_E_CALLBACK_ALREADY_REGISTERED: crate::core::HRESULT =
    crate::core::HRESULT(-2143419898i32);
pub const WPN_E_CHANNEL_CLOSED: crate::core::HRESULT = crate::core::HRESULT(-2143420160i32);
pub const WPN_E_CHANNEL_REQUEST_NOT_COMPLETE: crate::core::HRESULT =
    crate::core::HRESULT(-2143420159i32);
pub const WPN_E_CLOUD_AUTH_UNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2143420134i32);
pub const WPN_E_CLOUD_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2143420151i32);
pub const WPN_E_CLOUD_DISABLED_FOR_APP: crate::core::HRESULT = crate::core::HRESULT(-2143419893i32);
pub const WPN_E_CLOUD_INCAPABLE: crate::core::HRESULT = crate::core::HRESULT(-2143420144i32);
pub const WPN_E_CLOUD_SERVICE_UNAVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2143420133i32);
pub const WPN_E_DEV_ID_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2143420128i32);
pub const WPN_E_DUPLICATE_CHANNEL: crate::core::HRESULT = crate::core::HRESULT(-2143420156i32);
pub const WPN_E_DUPLICATE_REGISTRATION: crate::core::HRESULT = crate::core::HRESULT(-2143420136i32);
pub const WPN_E_FAILED_LOCK_SCREEN_UPDATE_INTIALIZATION: crate::core::HRESULT =
    crate::core::HRESULT(-2143420132i32);
pub const WPN_E_GROUP_ALPHANUMERIC: crate::core::HRESULT = crate::core::HRESULT(-2143419894i32);
pub const WPN_E_GROUP_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2143419895i32);
pub const WPN_E_IMAGE_NOT_FOUND_IN_CACHE: crate::core::HRESULT =
    crate::core::HRESULT(-2143419902i32);
pub const WPN_E_INTERNET_INCAPABLE: crate::core::HRESULT = crate::core::HRESULT(-2143420141i32);
pub const WPN_E_INVALID_APP: crate::core::HRESULT = crate::core::HRESULT(-2143420158i32);
pub const WPN_E_INVALID_CLOUD_IMAGE: crate::core::HRESULT = crate::core::HRESULT(-2143419900i32);
pub const WPN_E_INVALID_HTTP_STATUS_CODE: crate::core::HRESULT =
    crate::core::HRESULT(-2143420117i32);
pub const WPN_E_NOTIFICATION_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2143420143i32);
pub const WPN_E_NOTIFICATION_HIDDEN: crate::core::HRESULT = crate::core::HRESULT(-2143420153i32);
pub const WPN_E_NOTIFICATION_ID_MATCHED: crate::core::HRESULT =
    crate::core::HRESULT(-2143419899i32);
pub const WPN_E_NOTIFICATION_INCAPABLE: crate::core::HRESULT = crate::core::HRESULT(-2143420142i32);
pub const WPN_E_NOTIFICATION_NOT_POSTED: crate::core::HRESULT =
    crate::core::HRESULT(-2143420152i32);
pub const WPN_E_NOTIFICATION_POSTED: crate::core::HRESULT = crate::core::HRESULT(-2143420154i32);
pub const WPN_E_NOTIFICATION_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2143420139i32);
pub const WPN_E_NOTIFICATION_TYPE_DISABLED: crate::core::HRESULT =
    crate::core::HRESULT(-2143420140i32);
pub const WPN_E_OUTSTANDING_CHANNEL_REQUEST: crate::core::HRESULT =
    crate::core::HRESULT(-2143420157i32);
pub const WPN_E_OUT_OF_SESSION: crate::core::HRESULT = crate::core::HRESULT(-2143419904i32);
pub const WPN_E_PLATFORM_UNAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2143420155i32);
pub const WPN_E_POWER_SAVE: crate::core::HRESULT = crate::core::HRESULT(-2143419903i32);
pub const WPN_E_PUSH_NOTIFICATION_INCAPABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2143420135i32);
pub const WPN_E_STORAGE_LOCKED: crate::core::HRESULT = crate::core::HRESULT(-2143419896i32);
pub const WPN_E_TAG_ALPHANUMERIC: crate::core::HRESULT = crate::core::HRESULT(-2143420118i32);
pub const WPN_E_TAG_SIZE: crate::core::HRESULT = crate::core::HRESULT(-2143420138i32);
pub const WPN_E_TOAST_NOTIFICATION_DROPPED: crate::core::HRESULT =
    crate::core::HRESULT(-2143419897i32);
pub const WS_E_ADDRESS_IN_USE: crate::core::HRESULT = crate::core::HRESULT(-2143485941i32);
pub const WS_E_ADDRESS_NOT_AVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2143485940i32);
pub const WS_E_ENDPOINT_ACCESS_DENIED: crate::core::HRESULT = crate::core::HRESULT(-2143485947i32);
pub const WS_E_ENDPOINT_ACTION_NOT_SUPPORTED: crate::core::HRESULT =
    crate::core::HRESULT(-2143485935i32);
pub const WS_E_ENDPOINT_DISCONNECTED: crate::core::HRESULT = crate::core::HRESULT(-2143485932i32);
pub const WS_E_ENDPOINT_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2143485937i32);
pub const WS_E_ENDPOINT_FAULT_RECEIVED: crate::core::HRESULT = crate::core::HRESULT(-2143485933i32);
pub const WS_E_ENDPOINT_NOT_AVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2143485938i32);
pub const WS_E_ENDPOINT_NOT_FOUND: crate::core::HRESULT = crate::core::HRESULT(-2143485939i32);
pub const WS_E_ENDPOINT_TOO_BUSY: crate::core::HRESULT = crate::core::HRESULT(-2143485934i32);
pub const WS_E_ENDPOINT_UNREACHABLE: crate::core::HRESULT = crate::core::HRESULT(-2143485936i32);
pub const WS_E_INVALID_ENDPOINT_URL: crate::core::HRESULT = crate::core::HRESULT(-2143485920i32);
pub const WS_E_INVALID_FORMAT: crate::core::HRESULT = crate::core::HRESULT(-2143485952i32);
pub const WS_E_INVALID_OPERATION: crate::core::HRESULT = crate::core::HRESULT(-2143485949i32);
pub const WS_E_NOT_SUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2143485929i32);
pub const WS_E_NO_TRANSLATION_AVAILABLE: crate::core::HRESULT =
    crate::core::HRESULT(-2143485943i32);
pub const WS_E_NUMERIC_OVERFLOW: crate::core::HRESULT = crate::core::HRESULT(-2143485950i32);
pub const WS_E_OBJECT_FAULTED: crate::core::HRESULT = crate::core::HRESULT(-2143485951i32);
pub const WS_E_OPERATION_ABANDONED: crate::core::HRESULT = crate::core::HRESULT(-2143485945i32);
pub const WS_E_OPERATION_ABORTED: crate::core::HRESULT = crate::core::HRESULT(-2143485948i32);
pub const WS_E_OPERATION_TIMED_OUT: crate::core::HRESULT = crate::core::HRESULT(-2143485946i32);
pub const WS_E_OTHER: crate::core::HRESULT = crate::core::HRESULT(-2143485919i32);
pub const WS_E_PROXY_ACCESS_DENIED: crate::core::HRESULT = crate::core::HRESULT(-2143485930i32);
pub const WS_E_PROXY_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2143485931i32);
pub const WS_E_PROXY_REQUIRES_BASIC_AUTH: crate::core::HRESULT =
    crate::core::HRESULT(-2143485928i32);
pub const WS_E_PROXY_REQUIRES_DIGEST_AUTH: crate::core::HRESULT =
    crate::core::HRESULT(-2143485927i32);
pub const WS_E_PROXY_REQUIRES_NEGOTIATE_AUTH: crate::core::HRESULT =
    crate::core::HRESULT(-2143485925i32);
pub const WS_E_PROXY_REQUIRES_NTLM_AUTH: crate::core::HRESULT =
    crate::core::HRESULT(-2143485926i32);
pub const WS_E_QUOTA_EXCEEDED: crate::core::HRESULT = crate::core::HRESULT(-2143485944i32);
pub const WS_E_SECURITY_SYSTEM_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2143485917i32);
pub const WS_E_SECURITY_TOKEN_EXPIRED: crate::core::HRESULT = crate::core::HRESULT(-2143485918i32);
pub const WS_E_SECURITY_VERIFICATION_FAILURE: crate::core::HRESULT =
    crate::core::HRESULT(-2143485942i32);
pub const WS_E_SERVER_REQUIRES_BASIC_AUTH: crate::core::HRESULT =
    crate::core::HRESULT(-2143485924i32);
pub const WS_E_SERVER_REQUIRES_DIGEST_AUTH: crate::core::HRESULT =
    crate::core::HRESULT(-2143485923i32);
pub const WS_E_SERVER_REQUIRES_NEGOTIATE_AUTH: crate::core::HRESULT =
    crate::core::HRESULT(-2143485921i32);
pub const WS_E_SERVER_REQUIRES_NTLM_AUTH: crate::core::HRESULT =
    crate::core::HRESULT(-2143485922i32);
pub const WS_S_ASYNC: crate::core::HRESULT = crate::core::HRESULT(3997696i32);
pub const WS_S_END: crate::core::HRESULT = crate::core::HRESULT(3997697i32);
pub const XACT_E_ABORTED: crate::core::HRESULT = crate::core::HRESULT(-2147168231i32);
pub const XACT_E_ABORTING: crate::core::HRESULT = crate::core::HRESULT(-2147168215i32);
pub const XACT_E_ALREADYINPROGRESS: crate::core::HRESULT = crate::core::HRESULT(-2147168232i32);
pub const XACT_E_ALREADYOTHERSINGLEPHASE: crate::core::HRESULT =
    crate::core::HRESULT(-2147168256i32);
pub const XACT_E_CANTRETAIN: crate::core::HRESULT = crate::core::HRESULT(-2147168255i32);
pub const XACT_E_CLERKEXISTS: crate::core::HRESULT = crate::core::HRESULT(-2147168127i32);
pub const XACT_E_CLERKNOTFOUND: crate::core::HRESULT = crate::core::HRESULT(-2147168128i32);
pub const XACT_E_COMMITFAILED: crate::core::HRESULT = crate::core::HRESULT(-2147168254i32);
pub const XACT_E_COMMITPREVENTED: crate::core::HRESULT = crate::core::HRESULT(-2147168253i32);
pub const XACT_E_CONNECTION_DENIED: crate::core::HRESULT = crate::core::HRESULT(-2147168227i32);
pub const XACT_E_CONNECTION_DOWN: crate::core::HRESULT = crate::core::HRESULT(-2147168228i32);
pub const XACT_E_DEST_TMNOTAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2147168222i32);
pub const XACT_E_FIRST: u32 = 2147799040u32;
pub const XACT_E_HEURISTICABORT: crate::core::HRESULT = crate::core::HRESULT(-2147168252i32);
pub const XACT_E_HEURISTICCOMMIT: crate::core::HRESULT = crate::core::HRESULT(-2147168251i32);
pub const XACT_E_HEURISTICDAMAGE: crate::core::HRESULT = crate::core::HRESULT(-2147168250i32);
pub const XACT_E_HEURISTICDANGER: crate::core::HRESULT = crate::core::HRESULT(-2147168249i32);
pub const XACT_E_INDOUBT: crate::core::HRESULT = crate::core::HRESULT(-2147168234i32);
pub const XACT_E_INVALIDCOOKIE: crate::core::HRESULT = crate::core::HRESULT(-2147168235i32);
pub const XACT_E_INVALIDLSN: crate::core::HRESULT = crate::core::HRESULT(-2147168124i32);
pub const XACT_E_ISOLATIONLEVEL: crate::core::HRESULT = crate::core::HRESULT(-2147168248i32);
pub const XACT_E_LAST: u32 = 2147799083u32;
pub const XACT_E_LOGFULL: crate::core::HRESULT = crate::core::HRESULT(-2147168230i32);
pub const XACT_E_LU_TX_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2147168212i32);
pub const XACT_E_NETWORK_TX_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2147168220i32);
pub const XACT_E_NOASYNC: crate::core::HRESULT = crate::core::HRESULT(-2147168247i32);
pub const XACT_E_NOENLIST: crate::core::HRESULT = crate::core::HRESULT(-2147168246i32);
pub const XACT_E_NOIMPORTOBJECT: crate::core::HRESULT = crate::core::HRESULT(-2147168236i32);
pub const XACT_E_NOISORETAIN: crate::core::HRESULT = crate::core::HRESULT(-2147168245i32);
pub const XACT_E_NORESOURCE: crate::core::HRESULT = crate::core::HRESULT(-2147168244i32);
pub const XACT_E_NOTCURRENT: crate::core::HRESULT = crate::core::HRESULT(-2147168243i32);
pub const XACT_E_NOTIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2147168233i32);
pub const XACT_E_NOTRANSACTION: crate::core::HRESULT = crate::core::HRESULT(-2147168242i32);
pub const XACT_E_NOTSUPPORTED: crate::core::HRESULT = crate::core::HRESULT(-2147168241i32);
pub const XACT_E_PARTNER_NETWORK_TX_DISABLED: crate::core::HRESULT =
    crate::core::HRESULT(-2147168219i32);
pub const XACT_E_PULL_COMM_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2147168213i32);
pub const XACT_E_PUSH_COMM_FAILURE: crate::core::HRESULT = crate::core::HRESULT(-2147168214i32);
pub const XACT_E_RECOVERYINPROGRESS: crate::core::HRESULT = crate::core::HRESULT(-2147168126i32);
pub const XACT_E_REENLISTTIMEOUT: crate::core::HRESULT = crate::core::HRESULT(-2147168226i32);
pub const XACT_E_REPLAYREQUEST: crate::core::HRESULT = crate::core::HRESULT(-2147168123i32);
pub const XACT_E_TIP_CONNECT_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2147168225i32);
pub const XACT_E_TIP_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2147168221i32);
pub const XACT_E_TIP_PROTOCOL_ERROR: crate::core::HRESULT = crate::core::HRESULT(-2147168224i32);
pub const XACT_E_TIP_PULL_FAILED: crate::core::HRESULT = crate::core::HRESULT(-2147168223i32);
pub const XACT_E_TMNOTAVAILABLE: crate::core::HRESULT = crate::core::HRESULT(-2147168229i32);
pub const XACT_E_TRANSACTIONCLOSED: crate::core::HRESULT = crate::core::HRESULT(-2147168125i32);
pub const XACT_E_UNABLE_TO_LOAD_DTC_PROXY: crate::core::HRESULT =
    crate::core::HRESULT(-2147168216i32);
pub const XACT_E_UNABLE_TO_READ_DTC_CONFIG: crate::core::HRESULT =
    crate::core::HRESULT(-2147168217i32);
pub const XACT_E_UNKNOWNRMGRID: crate::core::HRESULT = crate::core::HRESULT(-2147168240i32);
pub const XACT_E_WRONGSTATE: crate::core::HRESULT = crate::core::HRESULT(-2147168239i32);
pub const XACT_E_WRONGUOW: crate::core::HRESULT = crate::core::HRESULT(-2147168238i32);
pub const XACT_E_XA_TX_DISABLED: crate::core::HRESULT = crate::core::HRESULT(-2147168218i32);
pub const XACT_E_XTIONEXISTS: crate::core::HRESULT = crate::core::HRESULT(-2147168237i32);
pub const XACT_S_ABORTING: crate::core::HRESULT = crate::core::HRESULT(315400i32);
pub const XACT_S_ALLNORETAIN: crate::core::HRESULT = crate::core::HRESULT(315399i32);
pub const XACT_S_ASYNC: crate::core::HRESULT = crate::core::HRESULT(315392i32);
pub const XACT_S_DEFECT: crate::core::HRESULT = crate::core::HRESULT(315393i32);
pub const XACT_S_FIRST: u32 = 315392u32;
pub const XACT_S_LAST: u32 = 315408u32;
pub const XACT_S_LASTRESOURCEMANAGER: crate::core::HRESULT = crate::core::HRESULT(315408i32);
pub const XACT_S_LOCALLY_OK: crate::core::HRESULT = crate::core::HRESULT(315402i32);
pub const XACT_S_MADECHANGESCONTENT: crate::core::HRESULT = crate::core::HRESULT(315397i32);
pub const XACT_S_MADECHANGESINFORM: crate::core::HRESULT = crate::core::HRESULT(315398i32);
pub const XACT_S_OKINFORM: crate::core::HRESULT = crate::core::HRESULT(315396i32);
pub const XACT_S_READONLY: crate::core::HRESULT = crate::core::HRESULT(315394i32);
pub const XACT_S_SINGLEPHASE: crate::core::HRESULT = crate::core::HRESULT(315401i32);
pub const XACT_S_SOMENORETAIN: crate::core::HRESULT = crate::core::HRESULT(315395i32);
pub const XENROLL_E_CANNOT_ADD_ROOT_CERT: crate::core::HRESULT =
    crate::core::HRESULT(-2146873343i32);
pub const XENROLL_E_KEYSPEC_SMIME_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2146873339i32);
pub const XENROLL_E_KEY_NOT_EXPORTABLE: crate::core::HRESULT = crate::core::HRESULT(-2146873344i32);
pub const XENROLL_E_RESPONSE_KA_HASH_MISMATCH: crate::core::HRESULT =
    crate::core::HRESULT(-2146873340i32);
pub const XENROLL_E_RESPONSE_KA_HASH_NOT_FOUND: crate::core::HRESULT =
    crate::core::HRESULT(-2146873342i32);
pub const XENROLL_E_RESPONSE_UNEXPECTED_KA_HASH: crate::core::HRESULT =
    crate::core::HRESULT(-2146873341i32);
pub const _WIN32_IE_MAXVER: u32 = 2560u32;
pub const _WIN32_MAXVER: u32 = 2560u32;
pub const _WIN32_WINDOWS_MAXVER: u32 = 2560u32;
pub const _WIN32_WINNT_MAXVER: u32 = 2560u32;
pub trait Api {
    fn CloseHandle(&self, h_object: HANDLE) -> BOOL {
        todo!("CloseHandle")
    }
    fn CompareObjectHandles(
        &self,
        h_first_object_handle: HANDLE,
        h_second_object_handle: HANDLE,
    ) -> BOOL {
        todo!("CompareObjectHandles")
    }
    fn DuplicateHandle(
        &self,
        h_source_process_handle: HANDLE,
        h_source_handle: HANDLE,
        h_target_process_handle: HANDLE,
        lp_target_handle: MutPtr<HANDLE>,
        dw_desired_access: u32,
        b_inherit_handle: BOOL,
        dw_options: DUPLICATE_HANDLE_OPTIONS,
    ) -> BOOL {
        todo!("DuplicateHandle")
    }
    fn GetHandleInformation(&self, h_object: HANDLE, lpdw_flags: MutPtr<u32>) -> BOOL {
        todo!("GetHandleInformation")
    }
    fn GetLastError(&self) -> WIN32_ERROR {
        todo!("GetLastError")
    }
    fn SetHandleInformation(&self, h_object: HANDLE, dw_mask: u32, dw_flags: HANDLE_FLAGS) -> BOOL {
        todo!("SetHandleInformation")
    }
    fn SetLastError(&self, dw_err_code: WIN32_ERROR) {
        todo!("SetLastError")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> &dyn Api {
    ctx.get::<dyn Api>()
}
