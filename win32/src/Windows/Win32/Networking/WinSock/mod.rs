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
pub const AAL5_MODE_MESSAGE: u32 = 1u32;
pub const AAL5_MODE_STREAMING: u32 = 2u32;
pub struct AAL5_PARAMETERS {
    pub ForwardMaxCPCSSDUSize: u32,
    pub BackwardMaxCPCSSDUSize: u32,
    pub Mode: u8,
    pub SSCSType: u8,
}
impl ::core::marker::Copy for AAL5_PARAMETERS {}
impl ::core::clone::Clone for AAL5_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AAL5_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AAL5_PARAMETERS")
            .field("ForwardMaxCPCSSDUSize", &self.ForwardMaxCPCSSDUSize)
            .field("BackwardMaxCPCSSDUSize", &self.BackwardMaxCPCSSDUSize)
            .field("Mode", &self.Mode)
            .field("SSCSType", &self.SSCSType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AAL5_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.ForwardMaxCPCSSDUSize == other.ForwardMaxCPCSSDUSize
            && self.BackwardMaxCPCSSDUSize == other.BackwardMaxCPCSSDUSize
            && self.Mode == other.Mode
            && self.SSCSType == other.SSCSType
    }
}
impl ::core::cmp::Eq for AAL5_PARAMETERS {}
impl FromIntoMemory for AAL5_PARAMETERS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_ForwardMaxCPCSSDUSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_BackwardMaxCPCSSDUSize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Mode = <u8 as FromIntoMemory>::from_bytes(&from[8..8 + 1]);
        let f_SSCSType = <u8 as FromIntoMemory>::from_bytes(&from[9..9 + 1]);
        Self {
            ForwardMaxCPCSSDUSize: f_ForwardMaxCPCSSDUSize,
            BackwardMaxCPCSSDUSize: f_BackwardMaxCPCSSDUSize,
            Mode: f_Mode,
            SSCSType: f_SSCSType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.ForwardMaxCPCSSDUSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.BackwardMaxCPCSSDUSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Mode, &mut into[8..8 + 1]);
        FromIntoMemory::into_bytes(self.SSCSType, &mut into[9..9 + 1]);
    }
    fn size() -> usize {
        12
    }
}
pub const AAL5_SSCS_FRAME_RELAY: u32 = 4u32;
pub const AAL5_SSCS_NULL: u32 = 0u32;
pub const AAL5_SSCS_SSCOP_ASSURED: u32 = 1u32;
pub const AAL5_SSCS_SSCOP_NON_ASSURED: u32 = 2u32;
pub struct AALUSER_PARAMETERS {
    pub UserDefined: u32,
}
impl ::core::marker::Copy for AALUSER_PARAMETERS {}
impl ::core::clone::Clone for AALUSER_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AALUSER_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AALUSER_PARAMETERS")
            .field("UserDefined", &self.UserDefined)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AALUSER_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.UserDefined == other.UserDefined
    }
}
impl ::core::cmp::Eq for AALUSER_PARAMETERS {}
impl FromIntoMemory for AALUSER_PARAMETERS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_UserDefined = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            UserDefined: f_UserDefined,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.UserDefined, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct AAL_PARAMETERS_IE {
    pub AALType: AAL_TYPE,
    pub AALSpecificParameters: AAL_PARAMETERS_IE_0,
}
impl ::core::marker::Copy for AAL_PARAMETERS_IE {}
impl ::core::clone::Clone for AAL_PARAMETERS_IE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AAL_PARAMETERS_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AAL_PARAMETERS_IE")
            .field("AALType", &self.AALType)
            .field("AALSpecificParameters", &self.AALSpecificParameters)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AAL_PARAMETERS_IE {
    fn eq(&self, other: &Self) -> bool {
        self.AALType == other.AALType && self.AALSpecificParameters == other.AALSpecificParameters
    }
}
impl ::core::cmp::Eq for AAL_PARAMETERS_IE {}
impl FromIntoMemory for AAL_PARAMETERS_IE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_AALType = <AAL_TYPE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_AALSpecificParameters =
            <AAL_PARAMETERS_IE_0 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            AALType: f_AALType,
            AALSpecificParameters: f_AALSpecificParameters,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.AALType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.AALSpecificParameters, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct AAL_PARAMETERS_IE_0 {
    data: [u8; 4],
}
impl ::core::default::Default for AAL_PARAMETERS_IE_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for AAL_PARAMETERS_IE_0 {}
impl ::core::clone::Clone for AAL_PARAMETERS_IE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AAL_PARAMETERS_IE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AAL_PARAMETERS_IE_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AAL_PARAMETERS_IE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for AAL_PARAMETERS_IE_0 {}
impl FromIntoMemory for AAL_PARAMETERS_IE_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 4];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AAL_TYPE(pub i32);
pub const AALTYPE_5: AAL_TYPE = AAL_TYPE(5i32);
pub const AALTYPE_USER: AAL_TYPE = AAL_TYPE(16i32);
impl ::core::marker::Copy for AAL_TYPE {}
impl ::core::clone::Clone for AAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AAL_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for AAL_TYPE {
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
pub struct ADDRINFOA {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: PtrRepr,
    pub ai_canonname: PSTR,
    pub ai_addr: MutPtr<SOCKADDR>,
    pub ai_next: MutPtr<ADDRINFOA>,
}
impl ::core::marker::Copy for ADDRINFOA {}
impl ::core::clone::Clone for ADDRINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADDRINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRINFOA")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_next", &self.ai_next)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ADDRINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags
            && self.ai_family == other.ai_family
            && self.ai_socktype == other.ai_socktype
            && self.ai_protocol == other.ai_protocol
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_canonname == other.ai_canonname
            && self.ai_addr == other.ai_addr
            && self.ai_next == other.ai_next
    }
}
impl ::core::cmp::Eq for ADDRINFOA {}
impl FromIntoMemory for ADDRINFOA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_ai_flags = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ai_family = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ai_socktype = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ai_protocol = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ai_addrlen = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ai_canonname = <PSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ai_addr = <MutPtr<SOCKADDR> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ai_next = <MutPtr<ADDRINFOA> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            ai_flags: f_ai_flags,
            ai_family: f_ai_family,
            ai_socktype: f_ai_socktype,
            ai_protocol: f_ai_protocol,
            ai_addrlen: f_ai_addrlen,
            ai_canonname: f_ai_canonname,
            ai_addr: f_ai_addr,
            ai_next: f_ai_next,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.ai_flags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ai_family, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ai_socktype, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ai_protocol, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ai_addrlen, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ai_canonname, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ai_addr, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ai_next, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub const ADDRINFOEX_VERSION_2: u32 = 2u32;
pub const ADDRINFOEX_VERSION_3: u32 = 3u32;
pub const ADDRINFOEX_VERSION_4: u32 = 4u32;
pub const ADDRINFOEX_VERSION_5: u32 = 5u32;
pub const ADDRINFOEX_VERSION_6: u32 = 6u32;
pub struct AFPROTOCOLS {
    pub iAddressFamily: i32,
    pub iProtocol: i32,
}
impl ::core::marker::Copy for AFPROTOCOLS {}
impl ::core::clone::Clone for AFPROTOCOLS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AFPROTOCOLS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AFPROTOCOLS")
            .field("iAddressFamily", &self.iAddressFamily)
            .field("iProtocol", &self.iProtocol)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AFPROTOCOLS {
    fn eq(&self, other: &Self) -> bool {
        self.iAddressFamily == other.iAddressFamily && self.iProtocol == other.iProtocol
    }
}
impl ::core::cmp::Eq for AFPROTOCOLS {}
impl FromIntoMemory for AFPROTOCOLS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_iAddressFamily = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_iProtocol = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            iAddressFamily: f_iAddressFamily,
            iProtocol: f_iProtocol,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.iAddressFamily, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.iProtocol, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const AF_12844: u16 = 25u16;
pub const AF_APPLETALK: u16 = 16u16;
pub const AF_ATM: u16 = 22u16;
pub const AF_BAN: u16 = 21u16;
pub const AF_CCITT: u16 = 10u16;
pub const AF_CHAOS: u16 = 5u16;
pub const AF_CLUSTER: u16 = 24u16;
pub const AF_DATAKIT: u16 = 9u16;
pub const AF_DECnet: u16 = 12u16;
pub const AF_DLI: u16 = 13u16;
pub const AF_ECMA: u16 = 8u16;
pub const AF_FIREFOX: u16 = 19u16;
pub const AF_HYLINK: u16 = 15u16;
pub const AF_HYPERV: u16 = 34u16;
pub const AF_ICLFXBM: u16 = 31u16;
pub const AF_IMPLINK: u16 = 3u16;
pub const AF_IPX: u16 = 6u16;
pub const AF_IRDA: u16 = 26u16;
pub const AF_ISO: u16 = 7u16;
pub const AF_LAT: u16 = 14u16;
pub const AF_LINK: u16 = 33u16;
pub const AF_MAX: u16 = 29u16;
pub const AF_NETBIOS: u16 = 17u16;
pub const AF_NETDES: u16 = 28u16;
pub const AF_NS: u16 = 6u16;
pub const AF_OSI: u16 = 7u16;
pub const AF_PUP: u16 = 4u16;
pub const AF_SNA: u16 = 11u16;
pub const AF_TCNMESSAGE: u16 = 30u16;
pub const AF_TCNPROCESS: u16 = 29u16;
pub const AF_UNIX: u16 = 1u16;
pub const AF_UNKNOWN1: u16 = 20u16;
pub const AF_VOICEVIEW: u16 = 18u16;
pub const AI_ADDRCONFIG: u32 = 1024u32;
pub const AI_ALL: u32 = 256u32;
pub const AI_BYPASS_DNS_CACHE: u32 = 64u32;
pub const AI_CANONNAME: u32 = 2u32;
pub const AI_DISABLE_IDN_ENCODING: u32 = 524288u32;
pub const AI_DNS_ONLY: u32 = 16u32;
pub const AI_DNS_RESPONSE_HOSTFILE: u32 = 2u32;
pub const AI_DNS_RESPONSE_SECURE: u32 = 1u32;
pub const AI_DNS_SERVER_TYPE_DOH: u32 = 2u32;
pub const AI_DNS_SERVER_TYPE_UDP: u32 = 1u32;
pub const AI_DNS_SERVER_UDP_FALLBACK: u32 = 1u32;
pub const AI_EXCLUSIVE_CUSTOM_SERVERS: u32 = 2097152u32;
pub const AI_EXTENDED: u32 = 2147483648u32;
pub const AI_FILESERVER: u32 = 262144u32;
pub const AI_FORCE_CLEAR_TEXT: u32 = 32u32;
pub const AI_FQDN: u32 = 131072u32;
pub const AI_NON_AUTHORITATIVE: u32 = 16384u32;
pub const AI_NUMERICHOST: u32 = 4u32;
pub const AI_NUMERICSERV: u32 = 8u32;
pub const AI_PASSIVE: u32 = 1u32;
pub const AI_REQUIRE_SECURE: u32 = 536870912u32;
pub const AI_RESOLUTION_HANDLE: u32 = 1073741824u32;
pub const AI_RETURN_PREFERRED_NAMES: u32 = 65536u32;
pub const AI_RETURN_RESPONSE_FLAGS: u32 = 268435456u32;
pub const AI_RETURN_TTL: u32 = 128u32;
pub const AI_SECURE: u32 = 32768u32;
pub const AI_SECURE_WITH_FALLBACK: u32 = 1048576u32;
pub const AI_V4MAPPED: u32 = 2048u32;
pub const ASSOCIATE_NAMERES_CONTEXT: crate::core::GUID =
    crate::core::GUID::from_u128(0x59a38b67_d4fe_46e1_ba3c_87ea74ca3049);
pub struct ASSOCIATE_NAMERES_CONTEXT_INPUT {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub Handle: u64,
}
impl ::core::marker::Copy for ASSOCIATE_NAMERES_CONTEXT_INPUT {}
impl ::core::clone::Clone for ASSOCIATE_NAMERES_CONTEXT_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ASSOCIATE_NAMERES_CONTEXT_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ASSOCIATE_NAMERES_CONTEXT_INPUT")
            .field("TransportSettingId", &self.TransportSettingId)
            .field("Handle", &self.Handle)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ASSOCIATE_NAMERES_CONTEXT_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.TransportSettingId == other.TransportSettingId && self.Handle == other.Handle
    }
}
impl ::core::cmp::Eq for ASSOCIATE_NAMERES_CONTEXT_INPUT {}
impl FromIntoMemory for ASSOCIATE_NAMERES_CONTEXT_INPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_TransportSettingId =
            <TRANSPORT_SETTING_ID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_Handle = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        Self {
            TransportSettingId: f_TransportSettingId,
            Handle: f_Handle,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.TransportSettingId, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.Handle, &mut into[16..16 + 8]);
    }
    fn size() -> usize {
        24
    }
}
pub const ATMPROTO_AAL1: u32 = 1u32;
pub const ATMPROTO_AAL2: u32 = 2u32;
pub const ATMPROTO_AAL34: u32 = 3u32;
pub const ATMPROTO_AAL5: u32 = 5u32;
pub const ATMPROTO_AALUSER: u32 = 0u32;
pub struct ATM_ADDRESS {
    pub AddressType: u32,
    pub NumofDigits: u32,
    pub Addr: [u8; 20],
}
impl ::core::marker::Copy for ATM_ADDRESS {}
impl ::core::clone::Clone for ATM_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ATM_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_ADDRESS")
            .field("AddressType", &self.AddressType)
            .field("NumofDigits", &self.NumofDigits)
            .field("Addr", &self.Addr)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ATM_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressType == other.AddressType
            && self.NumofDigits == other.NumofDigits
            && self.Addr == other.Addr
    }
}
impl ::core::cmp::Eq for ATM_ADDRESS {}
impl FromIntoMemory for ATM_ADDRESS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_AddressType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_NumofDigits = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Addr = <[u8; 20] as FromIntoMemory>::from_bytes(&from[8..8 + 20]);
        Self {
            AddressType: f_AddressType,
            NumofDigits: f_NumofDigits,
            Addr: f_Addr,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.AddressType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.NumofDigits, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Addr, &mut into[8..8 + 20]);
    }
    fn size() -> usize {
        28
    }
}
pub const ATM_ADDR_SIZE: u32 = 20u32;
pub const ATM_AESA: u32 = 2u32;
pub struct ATM_BHLI {
    pub HighLayerInfoType: u32,
    pub HighLayerInfoLength: u32,
    pub HighLayerInfo: [u8; 8],
}
impl ::core::marker::Copy for ATM_BHLI {}
impl ::core::clone::Clone for ATM_BHLI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ATM_BHLI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_BHLI")
            .field("HighLayerInfoType", &self.HighLayerInfoType)
            .field("HighLayerInfoLength", &self.HighLayerInfoLength)
            .field("HighLayerInfo", &self.HighLayerInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ATM_BHLI {
    fn eq(&self, other: &Self) -> bool {
        self.HighLayerInfoType == other.HighLayerInfoType
            && self.HighLayerInfoLength == other.HighLayerInfoLength
            && self.HighLayerInfo == other.HighLayerInfo
    }
}
impl ::core::cmp::Eq for ATM_BHLI {}
impl FromIntoMemory for ATM_BHLI {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_HighLayerInfoType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_HighLayerInfoLength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_HighLayerInfo = <[u8; 8] as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            HighLayerInfoType: f_HighLayerInfoType,
            HighLayerInfoLength: f_HighLayerInfoLength,
            HighLayerInfo: f_HighLayerInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.HighLayerInfoType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.HighLayerInfoLength, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.HighLayerInfo, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub struct ATM_BLLI {
    pub Layer2Protocol: u32,
    pub Layer2UserSpecifiedProtocol: u32,
    pub Layer3Protocol: u32,
    pub Layer3UserSpecifiedProtocol: u32,
    pub Layer3IPI: u32,
    pub SnapID: [u8; 5],
}
impl ::core::marker::Copy for ATM_BLLI {}
impl ::core::clone::Clone for ATM_BLLI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ATM_BLLI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_BLLI")
            .field("Layer2Protocol", &self.Layer2Protocol)
            .field(
                "Layer2UserSpecifiedProtocol",
                &self.Layer2UserSpecifiedProtocol,
            )
            .field("Layer3Protocol", &self.Layer3Protocol)
            .field(
                "Layer3UserSpecifiedProtocol",
                &self.Layer3UserSpecifiedProtocol,
            )
            .field("Layer3IPI", &self.Layer3IPI)
            .field("SnapID", &self.SnapID)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ATM_BLLI {
    fn eq(&self, other: &Self) -> bool {
        self.Layer2Protocol == other.Layer2Protocol
            && self.Layer2UserSpecifiedProtocol == other.Layer2UserSpecifiedProtocol
            && self.Layer3Protocol == other.Layer3Protocol
            && self.Layer3UserSpecifiedProtocol == other.Layer3UserSpecifiedProtocol
            && self.Layer3IPI == other.Layer3IPI
            && self.SnapID == other.SnapID
    }
}
impl ::core::cmp::Eq for ATM_BLLI {}
impl FromIntoMemory for ATM_BLLI {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_Layer2Protocol = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Layer2UserSpecifiedProtocol = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Layer3Protocol = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_Layer3UserSpecifiedProtocol = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Layer3IPI = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_SnapID = <[u8; 5] as FromIntoMemory>::from_bytes(&from[20..20 + 5]);
        Self {
            Layer2Protocol: f_Layer2Protocol,
            Layer2UserSpecifiedProtocol: f_Layer2UserSpecifiedProtocol,
            Layer3Protocol: f_Layer3Protocol,
            Layer3UserSpecifiedProtocol: f_Layer3UserSpecifiedProtocol,
            Layer3IPI: f_Layer3IPI,
            SnapID: f_SnapID,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.Layer2Protocol, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Layer2UserSpecifiedProtocol, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Layer3Protocol, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.Layer3UserSpecifiedProtocol, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Layer3IPI, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.SnapID, &mut into[20..20 + 5]);
    }
    fn size() -> usize {
        28
    }
}
pub struct ATM_BLLI_IE {
    pub Layer2Protocol: u32,
    pub Layer2Mode: u8,
    pub Layer2WindowSize: u8,
    pub Layer2UserSpecifiedProtocol: u32,
    pub Layer3Protocol: u32,
    pub Layer3Mode: u8,
    pub Layer3DefaultPacketSize: u8,
    pub Layer3PacketWindowSize: u8,
    pub Layer3UserSpecifiedProtocol: u32,
    pub Layer3IPI: u32,
    pub SnapID: [u8; 5],
}
impl ::core::marker::Copy for ATM_BLLI_IE {}
impl ::core::clone::Clone for ATM_BLLI_IE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ATM_BLLI_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_BLLI_IE")
            .field("Layer2Protocol", &self.Layer2Protocol)
            .field("Layer2Mode", &self.Layer2Mode)
            .field("Layer2WindowSize", &self.Layer2WindowSize)
            .field(
                "Layer2UserSpecifiedProtocol",
                &self.Layer2UserSpecifiedProtocol,
            )
            .field("Layer3Protocol", &self.Layer3Protocol)
            .field("Layer3Mode", &self.Layer3Mode)
            .field("Layer3DefaultPacketSize", &self.Layer3DefaultPacketSize)
            .field("Layer3PacketWindowSize", &self.Layer3PacketWindowSize)
            .field(
                "Layer3UserSpecifiedProtocol",
                &self.Layer3UserSpecifiedProtocol,
            )
            .field("Layer3IPI", &self.Layer3IPI)
            .field("SnapID", &self.SnapID)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ATM_BLLI_IE {
    fn eq(&self, other: &Self) -> bool {
        self.Layer2Protocol == other.Layer2Protocol
            && self.Layer2Mode == other.Layer2Mode
            && self.Layer2WindowSize == other.Layer2WindowSize
            && self.Layer2UserSpecifiedProtocol == other.Layer2UserSpecifiedProtocol
            && self.Layer3Protocol == other.Layer3Protocol
            && self.Layer3Mode == other.Layer3Mode
            && self.Layer3DefaultPacketSize == other.Layer3DefaultPacketSize
            && self.Layer3PacketWindowSize == other.Layer3PacketWindowSize
            && self.Layer3UserSpecifiedProtocol == other.Layer3UserSpecifiedProtocol
            && self.Layer3IPI == other.Layer3IPI
            && self.SnapID == other.SnapID
    }
}
impl ::core::cmp::Eq for ATM_BLLI_IE {}
impl FromIntoMemory for ATM_BLLI_IE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_Layer2Protocol = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Layer2Mode = <u8 as FromIntoMemory>::from_bytes(&from[4..4 + 1]);
        let f_Layer2WindowSize = <u8 as FromIntoMemory>::from_bytes(&from[5..5 + 1]);
        let f_Layer2UserSpecifiedProtocol = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_Layer3Protocol = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Layer3Mode = <u8 as FromIntoMemory>::from_bytes(&from[16..16 + 1]);
        let f_Layer3DefaultPacketSize = <u8 as FromIntoMemory>::from_bytes(&from[17..17 + 1]);
        let f_Layer3PacketWindowSize = <u8 as FromIntoMemory>::from_bytes(&from[18..18 + 1]);
        let f_Layer3UserSpecifiedProtocol = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_Layer3IPI = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_SnapID = <[u8; 5] as FromIntoMemory>::from_bytes(&from[28..28 + 5]);
        Self {
            Layer2Protocol: f_Layer2Protocol,
            Layer2Mode: f_Layer2Mode,
            Layer2WindowSize: f_Layer2WindowSize,
            Layer2UserSpecifiedProtocol: f_Layer2UserSpecifiedProtocol,
            Layer3Protocol: f_Layer3Protocol,
            Layer3Mode: f_Layer3Mode,
            Layer3DefaultPacketSize: f_Layer3DefaultPacketSize,
            Layer3PacketWindowSize: f_Layer3PacketWindowSize,
            Layer3UserSpecifiedProtocol: f_Layer3UserSpecifiedProtocol,
            Layer3IPI: f_Layer3IPI,
            SnapID: f_SnapID,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.Layer2Protocol, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Layer2Mode, &mut into[4..4 + 1]);
        FromIntoMemory::into_bytes(self.Layer2WindowSize, &mut into[5..5 + 1]);
        FromIntoMemory::into_bytes(self.Layer2UserSpecifiedProtocol, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.Layer3Protocol, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Layer3Mode, &mut into[16..16 + 1]);
        FromIntoMemory::into_bytes(self.Layer3DefaultPacketSize, &mut into[17..17 + 1]);
        FromIntoMemory::into_bytes(self.Layer3PacketWindowSize, &mut into[18..18 + 1]);
        FromIntoMemory::into_bytes(self.Layer3UserSpecifiedProtocol, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.Layer3IPI, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.SnapID, &mut into[28..28 + 5]);
    }
    fn size() -> usize {
        36
    }
}
pub struct ATM_BROADBAND_BEARER_CAPABILITY_IE {
    pub BearerClass: u8,
    pub TrafficType: u8,
    pub TimingRequirements: u8,
    pub ClippingSusceptability: u8,
    pub UserPlaneConnectionConfig: u8,
}
impl ::core::marker::Copy for ATM_BROADBAND_BEARER_CAPABILITY_IE {}
impl ::core::clone::Clone for ATM_BROADBAND_BEARER_CAPABILITY_IE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ATM_BROADBAND_BEARER_CAPABILITY_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_BROADBAND_BEARER_CAPABILITY_IE")
            .field("BearerClass", &self.BearerClass)
            .field("TrafficType", &self.TrafficType)
            .field("TimingRequirements", &self.TimingRequirements)
            .field("ClippingSusceptability", &self.ClippingSusceptability)
            .field("UserPlaneConnectionConfig", &self.UserPlaneConnectionConfig)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ATM_BROADBAND_BEARER_CAPABILITY_IE {
    fn eq(&self, other: &Self) -> bool {
        self.BearerClass == other.BearerClass
            && self.TrafficType == other.TrafficType
            && self.TimingRequirements == other.TimingRequirements
            && self.ClippingSusceptability == other.ClippingSusceptability
            && self.UserPlaneConnectionConfig == other.UserPlaneConnectionConfig
    }
}
impl ::core::cmp::Eq for ATM_BROADBAND_BEARER_CAPABILITY_IE {}
impl FromIntoMemory for ATM_BROADBAND_BEARER_CAPABILITY_IE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 5);
        let f_BearerClass = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_TrafficType = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_TimingRequirements = <u8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_ClippingSusceptability = <u8 as FromIntoMemory>::from_bytes(&from[3..3 + 1]);
        let f_UserPlaneConnectionConfig = <u8 as FromIntoMemory>::from_bytes(&from[4..4 + 1]);
        Self {
            BearerClass: f_BearerClass,
            TrafficType: f_TrafficType,
            TimingRequirements: f_TimingRequirements,
            ClippingSusceptability: f_ClippingSusceptability,
            UserPlaneConnectionConfig: f_UserPlaneConnectionConfig,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 5);
        FromIntoMemory::into_bytes(self.BearerClass, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.TrafficType, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.TimingRequirements, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.ClippingSusceptability, &mut into[3..3 + 1]);
        FromIntoMemory::into_bytes(self.UserPlaneConnectionConfig, &mut into[4..4 + 1]);
    }
    fn size() -> usize {
        5
    }
}
pub struct ATM_CALLING_PARTY_NUMBER_IE {
    pub ATM_Number: ATM_ADDRESS,
    pub Presentation_Indication: u8,
    pub Screening_Indicator: u8,
}
impl ::core::marker::Copy for ATM_CALLING_PARTY_NUMBER_IE {}
impl ::core::clone::Clone for ATM_CALLING_PARTY_NUMBER_IE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ATM_CALLING_PARTY_NUMBER_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_CALLING_PARTY_NUMBER_IE")
            .field("ATM_Number", &self.ATM_Number)
            .field("Presentation_Indication", &self.Presentation_Indication)
            .field("Screening_Indicator", &self.Screening_Indicator)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ATM_CALLING_PARTY_NUMBER_IE {
    fn eq(&self, other: &Self) -> bool {
        self.ATM_Number == other.ATM_Number
            && self.Presentation_Indication == other.Presentation_Indication
            && self.Screening_Indicator == other.Screening_Indicator
    }
}
impl ::core::cmp::Eq for ATM_CALLING_PARTY_NUMBER_IE {}
impl FromIntoMemory for ATM_CALLING_PARTY_NUMBER_IE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_ATM_Number = <ATM_ADDRESS as FromIntoMemory>::from_bytes(&from[0..0 + 28]);
        let f_Presentation_Indication = <u8 as FromIntoMemory>::from_bytes(&from[28..28 + 1]);
        let f_Screening_Indicator = <u8 as FromIntoMemory>::from_bytes(&from[29..29 + 1]);
        Self {
            ATM_Number: f_ATM_Number,
            Presentation_Indication: f_Presentation_Indication,
            Screening_Indicator: f_Screening_Indicator,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.ATM_Number, &mut into[0..0 + 28]);
        FromIntoMemory::into_bytes(self.Presentation_Indication, &mut into[28..28 + 1]);
        FromIntoMemory::into_bytes(self.Screening_Indicator, &mut into[29..29 + 1]);
    }
    fn size() -> usize {
        32
    }
}
pub struct ATM_CAUSE_IE {
    pub Location: u8,
    pub Cause: u8,
    pub DiagnosticsLength: u8,
    pub Diagnostics: [u8; 4],
}
impl ::core::marker::Copy for ATM_CAUSE_IE {}
impl ::core::clone::Clone for ATM_CAUSE_IE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ATM_CAUSE_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_CAUSE_IE")
            .field("Location", &self.Location)
            .field("Cause", &self.Cause)
            .field("DiagnosticsLength", &self.DiagnosticsLength)
            .field("Diagnostics", &self.Diagnostics)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ATM_CAUSE_IE {
    fn eq(&self, other: &Self) -> bool {
        self.Location == other.Location
            && self.Cause == other.Cause
            && self.DiagnosticsLength == other.DiagnosticsLength
            && self.Diagnostics == other.Diagnostics
    }
}
impl ::core::cmp::Eq for ATM_CAUSE_IE {}
impl FromIntoMemory for ATM_CAUSE_IE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 7);
        let f_Location = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Cause = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_DiagnosticsLength = <u8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_Diagnostics = <[u8; 4] as FromIntoMemory>::from_bytes(&from[3..3 + 4]);
        Self {
            Location: f_Location,
            Cause: f_Cause,
            DiagnosticsLength: f_DiagnosticsLength,
            Diagnostics: f_Diagnostics,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 7);
        FromIntoMemory::into_bytes(self.Location, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Cause, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.DiagnosticsLength, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.Diagnostics, &mut into[3..3 + 4]);
    }
    fn size() -> usize {
        7
    }
}
pub struct ATM_CONNECTION_ID {
    pub DeviceNumber: u32,
    pub VPI: u32,
    pub VCI: u32,
}
impl ::core::marker::Copy for ATM_CONNECTION_ID {}
impl ::core::clone::Clone for ATM_CONNECTION_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ATM_CONNECTION_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_CONNECTION_ID")
            .field("DeviceNumber", &self.DeviceNumber)
            .field("VPI", &self.VPI)
            .field("VCI", &self.VCI)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ATM_CONNECTION_ID {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceNumber == other.DeviceNumber && self.VPI == other.VPI && self.VCI == other.VCI
    }
}
impl ::core::cmp::Eq for ATM_CONNECTION_ID {}
impl FromIntoMemory for ATM_CONNECTION_ID {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_DeviceNumber = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_VPI = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_VCI = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            DeviceNumber: f_DeviceNumber,
            VPI: f_VPI,
            VCI: f_VCI,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.DeviceNumber, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.VPI, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.VCI, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const ATM_E164: u32 = 1u32;
pub const ATM_NSAP: u32 = 2u32;
#[doc = "*Required namespaces: 'Windows.Win32.NetworkManagement.QoS'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct ATM_PVC_PARAMS {
    pub PvcConnectionId: ATM_CONNECTION_ID,
    pub PvcQos: super::super::NetworkManagement::QoS::QOS,
}
#[doc = "*Required namespaces: 'Windows.Win32.NetworkManagement.QoS'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for ATM_PVC_PARAMS {}
#[doc = "*Required namespaces: 'Windows.Win32.NetworkManagement.QoS'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for ATM_PVC_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.NetworkManagement.QoS'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for ATM_PVC_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_PVC_PARAMS")
            .field("PvcConnectionId", &self.PvcConnectionId)
            .field("PvcQos", &self.PvcQos)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.NetworkManagement.QoS'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for ATM_PVC_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.PvcConnectionId == other.PvcConnectionId && self.PvcQos == other.PvcQos
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.NetworkManagement.QoS'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for ATM_PVC_PARAMS {}
#[doc = "*Required namespaces: 'Windows.Win32.NetworkManagement.QoS'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for ATM_PVC_PARAMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 84);
        let f_PvcConnectionId = <ATM_CONNECTION_ID as FromIntoMemory>::from_bytes(&from[0..0 + 12]);
        let f_PvcQos = <super::super::NetworkManagement::QoS::QOS as FromIntoMemory>::from_bytes(
            &from[12..12 + 72],
        );
        Self {
            PvcConnectionId: f_PvcConnectionId,
            PvcQos: f_PvcQos,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 84);
        FromIntoMemory::into_bytes(self.PvcConnectionId, &mut into[0..0 + 12]);
        FromIntoMemory::into_bytes(self.PvcQos, &mut into[12..12 + 72]);
    }
    fn size() -> usize {
        84
    }
}
pub struct ATM_QOS_CLASS_IE {
    pub QOSClassForward: u8,
    pub QOSClassBackward: u8,
}
impl ::core::marker::Copy for ATM_QOS_CLASS_IE {}
impl ::core::clone::Clone for ATM_QOS_CLASS_IE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ATM_QOS_CLASS_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_QOS_CLASS_IE")
            .field("QOSClassForward", &self.QOSClassForward)
            .field("QOSClassBackward", &self.QOSClassBackward)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ATM_QOS_CLASS_IE {
    fn eq(&self, other: &Self) -> bool {
        self.QOSClassForward == other.QOSClassForward
            && self.QOSClassBackward == other.QOSClassBackward
    }
}
impl ::core::cmp::Eq for ATM_QOS_CLASS_IE {}
impl FromIntoMemory for ATM_QOS_CLASS_IE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 2);
        let f_QOSClassForward = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_QOSClassBackward = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        Self {
            QOSClassForward: f_QOSClassForward,
            QOSClassBackward: f_QOSClassBackward,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 2);
        FromIntoMemory::into_bytes(self.QOSClassForward, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.QOSClassBackward, &mut into[1..1 + 1]);
    }
    fn size() -> usize {
        2
    }
}
pub struct ATM_TD {
    pub PeakCellRate_CLP0: u32,
    pub PeakCellRate_CLP01: u32,
    pub SustainableCellRate_CLP0: u32,
    pub SustainableCellRate_CLP01: u32,
    pub MaxBurstSize_CLP0: u32,
    pub MaxBurstSize_CLP01: u32,
    pub Tagging: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for ATM_TD {}
impl ::core::clone::Clone for ATM_TD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ATM_TD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_TD")
            .field("PeakCellRate_CLP0", &self.PeakCellRate_CLP0)
            .field("PeakCellRate_CLP01", &self.PeakCellRate_CLP01)
            .field("SustainableCellRate_CLP0", &self.SustainableCellRate_CLP0)
            .field("SustainableCellRate_CLP01", &self.SustainableCellRate_CLP01)
            .field("MaxBurstSize_CLP0", &self.MaxBurstSize_CLP0)
            .field("MaxBurstSize_CLP01", &self.MaxBurstSize_CLP01)
            .field("Tagging", &self.Tagging)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ATM_TD {
    fn eq(&self, other: &Self) -> bool {
        self.PeakCellRate_CLP0 == other.PeakCellRate_CLP0
            && self.PeakCellRate_CLP01 == other.PeakCellRate_CLP01
            && self.SustainableCellRate_CLP0 == other.SustainableCellRate_CLP0
            && self.SustainableCellRate_CLP01 == other.SustainableCellRate_CLP01
            && self.MaxBurstSize_CLP0 == other.MaxBurstSize_CLP0
            && self.MaxBurstSize_CLP01 == other.MaxBurstSize_CLP01
            && self.Tagging == other.Tagging
    }
}
impl ::core::cmp::Eq for ATM_TD {}
impl FromIntoMemory for ATM_TD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_PeakCellRate_CLP0 = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_PeakCellRate_CLP01 = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SustainableCellRate_CLP0 = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_SustainableCellRate_CLP01 = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_MaxBurstSize_CLP0 = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_MaxBurstSize_CLP01 = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_Tagging =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            PeakCellRate_CLP0: f_PeakCellRate_CLP0,
            PeakCellRate_CLP01: f_PeakCellRate_CLP01,
            SustainableCellRate_CLP0: f_SustainableCellRate_CLP0,
            SustainableCellRate_CLP01: f_SustainableCellRate_CLP01,
            MaxBurstSize_CLP0: f_MaxBurstSize_CLP0,
            MaxBurstSize_CLP01: f_MaxBurstSize_CLP01,
            Tagging: f_Tagging,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.PeakCellRate_CLP0, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.PeakCellRate_CLP01, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SustainableCellRate_CLP0, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.SustainableCellRate_CLP01, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.MaxBurstSize_CLP0, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.MaxBurstSize_CLP01, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.Tagging, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct ATM_TRAFFIC_DESCRIPTOR_IE {
    pub Forward: ATM_TD,
    pub Backward: ATM_TD,
    pub BestEffort: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for ATM_TRAFFIC_DESCRIPTOR_IE {}
impl ::core::clone::Clone for ATM_TRAFFIC_DESCRIPTOR_IE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ATM_TRAFFIC_DESCRIPTOR_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_TRAFFIC_DESCRIPTOR_IE")
            .field("Forward", &self.Forward)
            .field("Backward", &self.Backward)
            .field("BestEffort", &self.BestEffort)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ATM_TRAFFIC_DESCRIPTOR_IE {
    fn eq(&self, other: &Self) -> bool {
        self.Forward == other.Forward
            && self.Backward == other.Backward
            && self.BestEffort == other.BestEffort
    }
}
impl ::core::cmp::Eq for ATM_TRAFFIC_DESCRIPTOR_IE {}
impl FromIntoMemory for ATM_TRAFFIC_DESCRIPTOR_IE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 60);
        let f_Forward = <ATM_TD as FromIntoMemory>::from_bytes(&from[0..0 + 28]);
        let f_Backward = <ATM_TD as FromIntoMemory>::from_bytes(&from[28..28 + 28]);
        let f_BestEffort =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        Self {
            Forward: f_Forward,
            Backward: f_Backward,
            BestEffort: f_BestEffort,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 60);
        FromIntoMemory::into_bytes(self.Forward, &mut into[0..0 + 28]);
        FromIntoMemory::into_bytes(self.Backward, &mut into[28..28 + 28]);
        FromIntoMemory::into_bytes(self.BestEffort, &mut into[56..56 + 4]);
    }
    fn size() -> usize {
        60
    }
}
pub struct ATM_TRANSIT_NETWORK_SELECTION_IE {
    pub TypeOfNetworkId: u8,
    pub NetworkIdPlan: u8,
    pub NetworkIdLength: u8,
    pub NetworkId: [u8; 1],
}
impl ::core::marker::Copy for ATM_TRANSIT_NETWORK_SELECTION_IE {}
impl ::core::clone::Clone for ATM_TRANSIT_NETWORK_SELECTION_IE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ATM_TRANSIT_NETWORK_SELECTION_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATM_TRANSIT_NETWORK_SELECTION_IE")
            .field("TypeOfNetworkId", &self.TypeOfNetworkId)
            .field("NetworkIdPlan", &self.NetworkIdPlan)
            .field("NetworkIdLength", &self.NetworkIdLength)
            .field("NetworkId", &self.NetworkId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ATM_TRANSIT_NETWORK_SELECTION_IE {
    fn eq(&self, other: &Self) -> bool {
        self.TypeOfNetworkId == other.TypeOfNetworkId
            && self.NetworkIdPlan == other.NetworkIdPlan
            && self.NetworkIdLength == other.NetworkIdLength
            && self.NetworkId == other.NetworkId
    }
}
impl ::core::cmp::Eq for ATM_TRANSIT_NETWORK_SELECTION_IE {}
impl FromIntoMemory for ATM_TRANSIT_NETWORK_SELECTION_IE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_TypeOfNetworkId = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_NetworkIdPlan = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_NetworkIdLength = <u8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_NetworkId = <[u8; 1] as FromIntoMemory>::from_bytes(&from[3..3 + 1]);
        Self {
            TypeOfNetworkId: f_TypeOfNetworkId,
            NetworkIdPlan: f_NetworkIdPlan,
            NetworkIdLength: f_NetworkIdLength,
            NetworkId: f_NetworkId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.TypeOfNetworkId, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.NetworkIdPlan, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.NetworkIdLength, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.NetworkId, &mut into[3..3 + 1]);
    }
    fn size() -> usize {
        4
    }
}
pub const BASE_PROTOCOL: u32 = 1u32;
pub const BCOB_A: u32 = 1u32;
pub const BCOB_C: u32 = 3u32;
pub const BCOB_X: u32 = 16u32;
pub const BHLI_HighLayerProfile: u32 = 2u32;
pub const BHLI_ISO: u32 = 0u32;
pub const BHLI_UserSpecific: u32 = 1u32;
pub const BHLI_VendorSpecificAppId: u32 = 3u32;
pub const BIGENDIAN: u32 = 0u32;
pub const BITS_PER_BYTE: u32 = 8u32;
pub const BLLI_L2_ELAPB: u32 = 8u32;
pub const BLLI_L2_HDLC_ABM: u32 = 11u32;
pub const BLLI_L2_HDLC_ARM: u32 = 9u32;
pub const BLLI_L2_HDLC_NRM: u32 = 10u32;
pub const BLLI_L2_ISO_1745: u32 = 1u32;
pub const BLLI_L2_ISO_7776: u32 = 17u32;
pub const BLLI_L2_LLC: u32 = 12u32;
pub const BLLI_L2_MODE_EXT: u32 = 128u32;
pub const BLLI_L2_MODE_NORMAL: u32 = 64u32;
pub const BLLI_L2_Q921: u32 = 2u32;
pub const BLLI_L2_Q922: u32 = 14u32;
pub const BLLI_L2_USER_SPECIFIED: u32 = 16u32;
pub const BLLI_L2_X25L: u32 = 6u32;
pub const BLLI_L2_X25M: u32 = 7u32;
pub const BLLI_L2_X75: u32 = 13u32;
pub const BLLI_L3_IPI_IP: u32 = 204u32;
pub const BLLI_L3_IPI_SNAP: u32 = 128u32;
pub const BLLI_L3_ISO_8208: u32 = 7u32;
pub const BLLI_L3_ISO_TR9577: u32 = 11u32;
pub const BLLI_L3_MODE_EXT: u32 = 128u32;
pub const BLLI_L3_MODE_NORMAL: u32 = 64u32;
pub const BLLI_L3_PACKET_1024: u32 = 10u32;
pub const BLLI_L3_PACKET_128: u32 = 7u32;
pub const BLLI_L3_PACKET_16: u32 = 4u32;
pub const BLLI_L3_PACKET_2048: u32 = 11u32;
pub const BLLI_L3_PACKET_256: u32 = 8u32;
pub const BLLI_L3_PACKET_32: u32 = 5u32;
pub const BLLI_L3_PACKET_4096: u32 = 12u32;
pub const BLLI_L3_PACKET_512: u32 = 9u32;
pub const BLLI_L3_PACKET_64: u32 = 6u32;
pub const BLLI_L3_SIO_8473: u32 = 9u32;
pub const BLLI_L3_T70: u32 = 10u32;
pub const BLLI_L3_USER_SPECIFIED: u32 = 16u32;
pub const BLLI_L3_X223: u32 = 8u32;
pub const BLLI_L3_X25: u32 = 6u32;
pub const CAUSE_AAL_PARAMETERS_UNSUPPORTED: u32 = 93u32;
pub const CAUSE_ACCESS_INFORMAION_DISCARDED: u32 = 43u32;
pub const CAUSE_BEARER_CAPABILITY_UNAUTHORIZED: u32 = 57u32;
pub const CAUSE_BEARER_CAPABILITY_UNAVAILABLE: u32 = 58u32;
pub const CAUSE_BEARER_CAPABILITY_UNIMPLEMENTED: u32 = 65u32;
pub const CAUSE_CALL_REJECTED: u32 = 21u32;
pub const CAUSE_CHANNEL_NONEXISTENT: u32 = 82u32;
pub const CAUSE_COND_PERMANENT: u32 = 1u32;
pub const CAUSE_COND_TRANSIENT: u32 = 2u32;
pub const CAUSE_COND_UNKNOWN: u32 = 0u32;
pub const CAUSE_DESTINATION_OUT_OF_ORDER: u32 = 27u32;
pub const CAUSE_INCOMPATIBLE_DESTINATION: u32 = 88u32;
pub const CAUSE_INCORRECT_MESSAGE_LENGTH: u32 = 104u32;
pub const CAUSE_INVALID_CALL_REFERENCE: u32 = 81u32;
pub const CAUSE_INVALID_ENDPOINT_REFERENCE: u32 = 89u32;
pub const CAUSE_INVALID_IE_CONTENTS: u32 = 100u32;
pub const CAUSE_INVALID_NUMBER_FORMAT: u32 = 28u32;
pub const CAUSE_INVALID_STATE_FOR_MESSAGE: u32 = 101u32;
pub const CAUSE_INVALID_TRANSIT_NETWORK_SELECTION: u32 = 91u32;
pub const CAUSE_LOC_BEYOND_INTERWORKING: u32 = 10u32;
pub const CAUSE_LOC_INTERNATIONAL_NETWORK: u32 = 7u32;
pub const CAUSE_LOC_PRIVATE_LOCAL: u32 = 1u32;
pub const CAUSE_LOC_PRIVATE_REMOTE: u32 = 5u32;
pub const CAUSE_LOC_PUBLIC_LOCAL: u32 = 2u32;
pub const CAUSE_LOC_PUBLIC_REMOTE: u32 = 4u32;
pub const CAUSE_LOC_TRANSIT_NETWORK: u32 = 3u32;
pub const CAUSE_LOC_USER: u32 = 0u32;
pub const CAUSE_MANDATORY_IE_MISSING: u32 = 96u32;
pub const CAUSE_NA_ABNORMAL: u32 = 4u32;
pub const CAUSE_NA_NORMAL: u32 = 0u32;
pub const CAUSE_NETWORK_OUT_OF_ORDER: u32 = 38u32;
pub const CAUSE_NORMAL_CALL_CLEARING: u32 = 16u32;
pub const CAUSE_NORMAL_UNSPECIFIED: u32 = 31u32;
pub const CAUSE_NO_ROUTE_TO_DESTINATION: u32 = 3u32;
pub const CAUSE_NO_ROUTE_TO_TRANSIT_NETWORK: u32 = 2u32;
pub const CAUSE_NO_USER_RESPONDING: u32 = 18u32;
pub const CAUSE_NO_VPI_VCI_AVAILABLE: u32 = 45u32;
pub const CAUSE_NUMBER_CHANGED: u32 = 22u32;
pub const CAUSE_OPTION_UNAVAILABLE: u32 = 63u32;
pub const CAUSE_PROTOCOL_ERROR: u32 = 111u32;
pub const CAUSE_PU_PROVIDER: u32 = 0u32;
pub const CAUSE_PU_USER: u32 = 8u32;
pub const CAUSE_QOS_UNAVAILABLE: u32 = 49u32;
pub const CAUSE_REASON_IE_INSUFFICIENT: u32 = 8u32;
pub const CAUSE_REASON_IE_MISSING: u32 = 4u32;
pub const CAUSE_REASON_USER: u32 = 0u32;
pub const CAUSE_RECOVERY_ON_TIMEOUT: u32 = 102u32;
pub const CAUSE_RESOURCE_UNAVAILABLE: u32 = 47u32;
pub const CAUSE_STATUS_ENQUIRY_RESPONSE: u32 = 30u32;
pub const CAUSE_TEMPORARY_FAILURE: u32 = 41u32;
pub const CAUSE_TOO_MANY_PENDING_ADD_PARTY: u32 = 92u32;
pub const CAUSE_UNALLOCATED_NUMBER: u32 = 1u32;
pub const CAUSE_UNIMPLEMENTED_IE: u32 = 99u32;
pub const CAUSE_UNIMPLEMENTED_MESSAGE_TYPE: u32 = 97u32;
pub const CAUSE_UNSUPPORTED_TRAFFIC_PARAMETERS: u32 = 73u32;
pub const CAUSE_USER_BUSY: u32 = 17u32;
pub const CAUSE_USER_CELL_RATE_UNAVAILABLE: u32 = 51u32;
pub const CAUSE_USER_REJECTS_CLIR: u32 = 23u32;
pub const CAUSE_VPI_VCI_UNACCEPTABLE: u32 = 10u32;
pub const CAUSE_VPI_VCI_UNAVAILABLE: u32 = 35u32;
pub const CF_ACCEPT: u32 = 0u32;
pub const CF_DEFER: u32 = 2u32;
pub const CF_REJECT: u32 = 1u32;
pub const CLIP_NOT: u32 = 0u32;
pub const CLIP_SUS: u32 = 32u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CONTROL_CHANNEL_TRIGGER_STATUS(pub i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_INVALID: CONTROL_CHANNEL_TRIGGER_STATUS =
    CONTROL_CHANNEL_TRIGGER_STATUS(0i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SOFTWARE_SLOT_ALLOCATED: CONTROL_CHANNEL_TRIGGER_STATUS =
    CONTROL_CHANNEL_TRIGGER_STATUS(1i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_HARDWARE_SLOT_ALLOCATED: CONTROL_CHANNEL_TRIGGER_STATUS =
    CONTROL_CHANNEL_TRIGGER_STATUS(2i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_POLICY_ERROR: CONTROL_CHANNEL_TRIGGER_STATUS =
    CONTROL_CHANNEL_TRIGGER_STATUS(3i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SYSTEM_ERROR: CONTROL_CHANNEL_TRIGGER_STATUS =
    CONTROL_CHANNEL_TRIGGER_STATUS(4i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_TRANSPORT_DISCONNECTED: CONTROL_CHANNEL_TRIGGER_STATUS =
    CONTROL_CHANNEL_TRIGGER_STATUS(5i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SERVICE_UNAVAILABLE: CONTROL_CHANNEL_TRIGGER_STATUS =
    CONTROL_CHANNEL_TRIGGER_STATUS(6i32);
impl ::core::marker::Copy for CONTROL_CHANNEL_TRIGGER_STATUS {}
impl ::core::clone::Clone for CONTROL_CHANNEL_TRIGGER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONTROL_CHANNEL_TRIGGER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONTROL_CHANNEL_TRIGGER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTROL_CHANNEL_TRIGGER_STATUS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CONTROL_CHANNEL_TRIGGER_STATUS {
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
pub struct CSADDR_INFO {
    pub LocalAddr: SOCKET_ADDRESS,
    pub RemoteAddr: SOCKET_ADDRESS,
    pub iSocketType: i32,
    pub iProtocol: i32,
}
impl ::core::marker::Copy for CSADDR_INFO {}
impl ::core::clone::Clone for CSADDR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CSADDR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSADDR_INFO")
            .field("LocalAddr", &self.LocalAddr)
            .field("RemoteAddr", &self.RemoteAddr)
            .field("iSocketType", &self.iSocketType)
            .field("iProtocol", &self.iProtocol)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CSADDR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LocalAddr == other.LocalAddr
            && self.RemoteAddr == other.RemoteAddr
            && self.iSocketType == other.iSocketType
            && self.iProtocol == other.iProtocol
    }
}
impl ::core::cmp::Eq for CSADDR_INFO {}
impl FromIntoMemory for CSADDR_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_LocalAddr = <SOCKET_ADDRESS as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_RemoteAddr = <SOCKET_ADDRESS as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_iSocketType = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_iProtocol = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            LocalAddr: f_LocalAddr,
            RemoteAddr: f_RemoteAddr,
            iSocketType: f_iSocketType,
            iProtocol: f_iProtocol,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.LocalAddr, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.RemoteAddr, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.iSocketType, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.iProtocol, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const DE_REUSE_SOCKET: u32 = 2u32;
pub const FD_ACCEPT: u32 = 8u32;
pub const FD_ACCEPT_BIT: u32 = 3u32;
pub const FD_ADDRESS_LIST_CHANGE_BIT: u32 = 9u32;
pub const FD_CLOSE: u32 = 32u32;
pub const FD_CLOSE_BIT: u32 = 5u32;
pub const FD_CONNECT: u32 = 16u32;
pub const FD_CONNECT_BIT: u32 = 4u32;
pub const FD_GROUP_QOS_BIT: u32 = 7u32;
pub const FD_MAX_EVENTS: u32 = 10u32;
pub const FD_OOB: u32 = 4u32;
pub const FD_OOB_BIT: u32 = 2u32;
pub const FD_QOS_BIT: u32 = 6u32;
pub const FD_READ: u32 = 1u32;
pub const FD_READ_BIT: u32 = 0u32;
pub const FD_ROUTING_INTERFACE_CHANGE_BIT: u32 = 8u32;
pub const FD_SETSIZE: u32 = 64u32;
pub const FD_WRITE: u32 = 2u32;
pub const FD_WRITE_BIT: u32 = 1u32;
pub const FIOASYNC: i32 = -2147195267i32;
pub const FIONBIO: i32 = -2147195266i32;
pub const FIONREAD: i32 = 1074030207i32;
pub const FROM_PROTOCOL_INFO: i32 = -1i32;
pub const GAI_STRERROR_BUFFER_SIZE: u32 = 1024u32;
pub struct GROUP_FILTER {
    pub gf_interface: u32,
    pub gf_group: SOCKADDR_STORAGE,
    pub gf_fmode: MULTICAST_MODE_TYPE,
    pub gf_numsrc: u32,
    pub gf_slist: [SOCKADDR_STORAGE; 1],
}
impl ::core::marker::Copy for GROUP_FILTER {}
impl ::core::clone::Clone for GROUP_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GROUP_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_FILTER")
            .field("gf_interface", &self.gf_interface)
            .field("gf_group", &self.gf_group)
            .field("gf_fmode", &self.gf_fmode)
            .field("gf_numsrc", &self.gf_numsrc)
            .field("gf_slist", &self.gf_slist)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GROUP_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.gf_interface == other.gf_interface
            && self.gf_group == other.gf_group
            && self.gf_fmode == other.gf_fmode
            && self.gf_numsrc == other.gf_numsrc
            && self.gf_slist == other.gf_slist
    }
}
impl ::core::cmp::Eq for GROUP_FILTER {}
impl FromIntoMemory for GROUP_FILTER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 272);
        let f_gf_interface = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_gf_group = <SOCKADDR_STORAGE as FromIntoMemory>::from_bytes(&from[8..8 + 128]);
        let f_gf_fmode = <MULTICAST_MODE_TYPE as FromIntoMemory>::from_bytes(&from[136..136 + 4]);
        let f_gf_numsrc = <u32 as FromIntoMemory>::from_bytes(&from[140..140 + 4]);
        let f_gf_slist =
            <[SOCKADDR_STORAGE; 1] as FromIntoMemory>::from_bytes(&from[144..144 + 128]);
        Self {
            gf_interface: f_gf_interface,
            gf_group: f_gf_group,
            gf_fmode: f_gf_fmode,
            gf_numsrc: f_gf_numsrc,
            gf_slist: f_gf_slist,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 272);
        FromIntoMemory::into_bytes(self.gf_interface, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.gf_group, &mut into[8..8 + 128]);
        FromIntoMemory::into_bytes(self.gf_fmode, &mut into[136..136 + 4]);
        FromIntoMemory::into_bytes(self.gf_numsrc, &mut into[140..140 + 4]);
        FromIntoMemory::into_bytes(self.gf_slist, &mut into[144..144 + 128]);
    }
    fn size() -> usize {
        272
    }
}
pub struct GROUP_REQ {
    pub gr_interface: u32,
    pub gr_group: SOCKADDR_STORAGE,
}
impl ::core::marker::Copy for GROUP_REQ {}
impl ::core::clone::Clone for GROUP_REQ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GROUP_REQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_REQ")
            .field("gr_interface", &self.gr_interface)
            .field("gr_group", &self.gr_group)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GROUP_REQ {
    fn eq(&self, other: &Self) -> bool {
        self.gr_interface == other.gr_interface && self.gr_group == other.gr_group
    }
}
impl ::core::cmp::Eq for GROUP_REQ {}
impl FromIntoMemory for GROUP_REQ {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 136);
        let f_gr_interface = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_gr_group = <SOCKADDR_STORAGE as FromIntoMemory>::from_bytes(&from[8..8 + 128]);
        Self {
            gr_interface: f_gr_interface,
            gr_group: f_gr_group,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 136);
        FromIntoMemory::into_bytes(self.gr_interface, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.gr_group, &mut into[8..8 + 128]);
    }
    fn size() -> usize {
        136
    }
}
pub struct GROUP_SOURCE_REQ {
    pub gsr_interface: u32,
    pub gsr_group: SOCKADDR_STORAGE,
    pub gsr_source: SOCKADDR_STORAGE,
}
impl ::core::marker::Copy for GROUP_SOURCE_REQ {}
impl ::core::clone::Clone for GROUP_SOURCE_REQ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GROUP_SOURCE_REQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_SOURCE_REQ")
            .field("gsr_interface", &self.gsr_interface)
            .field("gsr_group", &self.gsr_group)
            .field("gsr_source", &self.gsr_source)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GROUP_SOURCE_REQ {
    fn eq(&self, other: &Self) -> bool {
        self.gsr_interface == other.gsr_interface
            && self.gsr_group == other.gsr_group
            && self.gsr_source == other.gsr_source
    }
}
impl ::core::cmp::Eq for GROUP_SOURCE_REQ {}
impl FromIntoMemory for GROUP_SOURCE_REQ {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 264);
        let f_gsr_interface = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_gsr_group = <SOCKADDR_STORAGE as FromIntoMemory>::from_bytes(&from[8..8 + 128]);
        let f_gsr_source = <SOCKADDR_STORAGE as FromIntoMemory>::from_bytes(&from[136..136 + 128]);
        Self {
            gsr_interface: f_gsr_interface,
            gsr_group: f_gsr_group,
            gsr_source: f_gsr_source,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 264);
        FromIntoMemory::into_bytes(self.gsr_interface, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.gsr_group, &mut into[8..8 + 128]);
        FromIntoMemory::into_bytes(self.gsr_source, &mut into[136..136 + 128]);
    }
    fn size() -> usize {
        264
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HWSAEVENT(pub PtrDiffRepr);
impl HWSAEVENT {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HWSAEVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWSAEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWSAEVENT {}
impl ::core::hash::Hash for HWSAEVENT {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HWSAEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWSAEVENT").field(&self.0).finish()
    }
}
impl FromIntoMemory for HWSAEVENT {
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
pub const IAS_ATTRIB_INT: u32 = 1u32;
pub const IAS_ATTRIB_NO_ATTRIB: u32 = 0u32;
pub const IAS_ATTRIB_NO_CLASS: u32 = 16u32;
pub const IAS_ATTRIB_OCTETSEQ: u32 = 2u32;
pub const IAS_ATTRIB_STR: u32 = 3u32;
pub const IAS_MAX_ATTRIBNAME: u32 = 256u32;
pub const IAS_MAX_CLASSNAME: u32 = 64u32;
pub const IAS_MAX_OCTET_STRING: u32 = 1024u32;
pub const IAS_MAX_USER_STRING: u32 = 256u32;
pub struct ICMP_ERROR_INFO {
    pub srcaddress: SOCKADDR_INET,
    pub protocol: IPPROTO,
    pub r#type: u8,
    pub code: u8,
}
impl ::core::marker::Copy for ICMP_ERROR_INFO {}
impl ::core::clone::Clone for ICMP_ERROR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICMP_ERROR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICMP_ERROR_INFO")
            .field("srcaddress", &self.srcaddress)
            .field("protocol", &self.protocol)
            .field("type", &self.r#type)
            .field("code", &self.code)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ICMP_ERROR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.srcaddress == other.srcaddress
            && self.protocol == other.protocol
            && self.r#type == other.r#type
            && self.code == other.code
    }
}
impl ::core::cmp::Eq for ICMP_ERROR_INFO {}
impl FromIntoMemory for ICMP_ERROR_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_srcaddress = <SOCKADDR_INET as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_protocol = <IPPROTO as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_type = <u8 as FromIntoMemory>::from_bytes(&from[8..8 + 1]);
        let f_code = <u8 as FromIntoMemory>::from_bytes(&from[9..9 + 1]);
        Self {
            srcaddress: f_srcaddress,
            protocol: f_protocol,
            r#type: f_type,
            code: f_code,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.srcaddress, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.protocol, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.r#type, &mut into[8..8 + 1]);
        FromIntoMemory::into_bytes(self.code, &mut into[9..9 + 1]);
    }
    fn size() -> usize {
        12
    }
}
pub const IFF_BROADCAST: u32 = 2u32;
pub const IFF_LOOPBACK: u32 = 4u32;
pub const IFF_MULTICAST: u32 = 16u32;
pub const IFF_POINTTOPOINT: u32 = 8u32;
pub const IFF_UP: u32 = 1u32;
pub const IMPLINK_HIGHEXPER: u32 = 158u32;
pub const IMPLINK_IP: u32 = 155u32;
pub const IMPLINK_LOWEXPER: u32 = 156u32;
pub const IN4ADDR_LINKLOCALPREFIX_LENGTH: u32 = 16u32;
pub const IN4ADDR_LOOPBACK: u32 = 16777343u32;
pub const IN4ADDR_LOOPBACKPREFIX_LENGTH: u32 = 8u32;
pub const IN4ADDR_MULTICASTPREFIX_LENGTH: u32 = 4u32;
pub const IN6ADDR_6TO4PREFIX_LENGTH: u32 = 16u32;
pub const IN6ADDR_LINKLOCALPREFIX_LENGTH: u32 = 64u32;
pub const IN6ADDR_MULTICASTPREFIX_LENGTH: u32 = 8u32;
pub const IN6ADDR_SOLICITEDNODEMULTICASTPREFIX_LENGTH: u32 = 104u32;
pub const IN6ADDR_TEREDOPREFIX_LENGTH: u32 = 32u32;
pub const IN6ADDR_V4MAPPEDPREFIX_LENGTH: u32 = 96u32;
pub struct IN6_ADDR {
    pub u: IN6_ADDR_0,
}
impl ::core::marker::Copy for IN6_ADDR {}
impl ::core::clone::Clone for IN6_ADDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IN6_ADDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IN6_ADDR").field("u", &self.u).finish()
    }
}
impl ::core::cmp::PartialEq for IN6_ADDR {
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u
    }
}
impl ::core::cmp::Eq for IN6_ADDR {}
impl FromIntoMemory for IN6_ADDR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_u = <IN6_ADDR_0 as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        Self { u: f_u }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.u, &mut into[0..0 + 16]);
    }
    fn size() -> usize {
        16
    }
}
pub struct IN6_ADDR_0 {
    data: [u8; 16],
}
impl ::core::default::Default for IN6_ADDR_0 {
    fn default() -> Self {
        Self { data: [0u8; 16] }
    }
}
impl ::core::marker::Copy for IN6_ADDR_0 {}
impl ::core::clone::Clone for IN6_ADDR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IN6_ADDR_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IN6_ADDR_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IN6_ADDR_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for IN6_ADDR_0 {}
impl FromIntoMemory for IN6_ADDR_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 16];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        16
    }
}
pub struct IN6_PKTINFO {
    pub ipi6_addr: IN6_ADDR,
    pub ipi6_ifindex: u32,
}
impl ::core::marker::Copy for IN6_PKTINFO {}
impl ::core::clone::Clone for IN6_PKTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IN6_PKTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IN6_PKTINFO")
            .field("ipi6_addr", &self.ipi6_addr)
            .field("ipi6_ifindex", &self.ipi6_ifindex)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IN6_PKTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ipi6_addr == other.ipi6_addr && self.ipi6_ifindex == other.ipi6_ifindex
    }
}
impl ::core::cmp::Eq for IN6_PKTINFO {}
impl FromIntoMemory for IN6_PKTINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_ipi6_addr = <IN6_ADDR as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_ipi6_ifindex = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            ipi6_addr: f_ipi6_addr,
            ipi6_ifindex: f_ipi6_ifindex,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.ipi6_addr, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.ipi6_ifindex, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub const INADDR_LOOPBACK: u32 = 2130706433u32;
pub const INADDR_NONE: u32 = 4294967295u32;
pub const INCL_WINSOCK_API_PROTOTYPES: u32 = 1u32;
pub const INCL_WINSOCK_API_TYPEDEFS: u32 = 0u32;
pub const INET6_ADDRSTRLEN: u32 = 65u32;
pub const INET_ADDRSTRLEN: u32 = 22u32;
pub struct INET_PORT_RANGE {
    pub StartPort: u16,
    pub NumberOfPorts: u16,
}
impl ::core::marker::Copy for INET_PORT_RANGE {}
impl ::core::clone::Clone for INET_PORT_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INET_PORT_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_PORT_RANGE")
            .field("StartPort", &self.StartPort)
            .field("NumberOfPorts", &self.NumberOfPorts)
            .finish()
    }
}
impl ::core::cmp::PartialEq for INET_PORT_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartPort == other.StartPort && self.NumberOfPorts == other.NumberOfPorts
    }
}
impl ::core::cmp::Eq for INET_PORT_RANGE {}
impl FromIntoMemory for INET_PORT_RANGE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_StartPort = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_NumberOfPorts = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            StartPort: f_StartPort,
            NumberOfPorts: f_NumberOfPorts,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.StartPort, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.NumberOfPorts, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4
    }
}
pub struct INET_PORT_RESERVATION_INFORMATION {
    pub OwningPid: u32,
}
impl ::core::marker::Copy for INET_PORT_RESERVATION_INFORMATION {}
impl ::core::clone::Clone for INET_PORT_RESERVATION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INET_PORT_RESERVATION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_PORT_RESERVATION_INFORMATION")
            .field("OwningPid", &self.OwningPid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for INET_PORT_RESERVATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.OwningPid == other.OwningPid
    }
}
impl ::core::cmp::Eq for INET_PORT_RESERVATION_INFORMATION {}
impl FromIntoMemory for INET_PORT_RESERVATION_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_OwningPid = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            OwningPid: f_OwningPid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.OwningPid, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct INET_PORT_RESERVATION_INSTANCE {
    pub Reservation: INET_PORT_RANGE,
    pub Token: INET_PORT_RESERVATION_TOKEN,
}
impl ::core::marker::Copy for INET_PORT_RESERVATION_INSTANCE {}
impl ::core::clone::Clone for INET_PORT_RESERVATION_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INET_PORT_RESERVATION_INSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_PORT_RESERVATION_INSTANCE")
            .field("Reservation", &self.Reservation)
            .field("Token", &self.Token)
            .finish()
    }
}
impl ::core::cmp::PartialEq for INET_PORT_RESERVATION_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        self.Reservation == other.Reservation && self.Token == other.Token
    }
}
impl ::core::cmp::Eq for INET_PORT_RESERVATION_INSTANCE {}
impl FromIntoMemory for INET_PORT_RESERVATION_INSTANCE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_Reservation = <INET_PORT_RANGE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Token = <INET_PORT_RESERVATION_TOKEN as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            Reservation: f_Reservation,
            Token: f_Token,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.Reservation, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Token, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub struct INET_PORT_RESERVATION_TOKEN {
    pub Token: u64,
}
impl ::core::marker::Copy for INET_PORT_RESERVATION_TOKEN {}
impl ::core::clone::Clone for INET_PORT_RESERVATION_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INET_PORT_RESERVATION_TOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_PORT_RESERVATION_TOKEN")
            .field("Token", &self.Token)
            .finish()
    }
}
impl ::core::cmp::PartialEq for INET_PORT_RESERVATION_TOKEN {
    fn eq(&self, other: &Self) -> bool {
        self.Token == other.Token
    }
}
impl ::core::cmp::Eq for INET_PORT_RESERVATION_TOKEN {}
impl FromIntoMemory for INET_PORT_RESERVATION_TOKEN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Token = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self { Token: f_Token }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Token, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
pub struct INTERFACE_INFO {
    pub iiFlags: u32,
    pub iiAddress: sockaddr_gen,
    pub iiBroadcastAddress: sockaddr_gen,
    pub iiNetmask: sockaddr_gen,
}
impl ::core::marker::Copy for INTERFACE_INFO {}
impl ::core::clone::Clone for INTERFACE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERFACE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACE_INFO")
            .field("iiFlags", &self.iiFlags)
            .field("iiAddress", &self.iiAddress)
            .field("iiBroadcastAddress", &self.iiBroadcastAddress)
            .field("iiNetmask", &self.iiNetmask)
            .finish()
    }
}
impl ::core::cmp::PartialEq for INTERFACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.iiFlags == other.iiFlags
            && self.iiAddress == other.iiAddress
            && self.iiBroadcastAddress == other.iiBroadcastAddress
            && self.iiNetmask == other.iiNetmask
    }
}
impl ::core::cmp::Eq for INTERFACE_INFO {}
impl FromIntoMemory for INTERFACE_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 76);
        let f_iiFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_iiAddress = <sockaddr_gen as FromIntoMemory>::from_bytes(&from[4..4 + 24]);
        let f_iiBroadcastAddress = <sockaddr_gen as FromIntoMemory>::from_bytes(&from[28..28 + 24]);
        let f_iiNetmask = <sockaddr_gen as FromIntoMemory>::from_bytes(&from[52..52 + 24]);
        Self {
            iiFlags: f_iiFlags,
            iiAddress: f_iiAddress,
            iiBroadcastAddress: f_iiBroadcastAddress,
            iiNetmask: f_iiNetmask,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 76);
        FromIntoMemory::into_bytes(self.iiFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.iiAddress, &mut into[4..4 + 24]);
        FromIntoMemory::into_bytes(self.iiBroadcastAddress, &mut into[28..28 + 24]);
        FromIntoMemory::into_bytes(self.iiNetmask, &mut into[52..52 + 24]);
    }
    fn size() -> usize {
        76
    }
}
pub struct INTERFACE_INFO_EX {
    pub iiFlags: u32,
    pub iiAddress: SOCKET_ADDRESS,
    pub iiBroadcastAddress: SOCKET_ADDRESS,
    pub iiNetmask: SOCKET_ADDRESS,
}
impl ::core::marker::Copy for INTERFACE_INFO_EX {}
impl ::core::clone::Clone for INTERFACE_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTERFACE_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACE_INFO_EX")
            .field("iiFlags", &self.iiFlags)
            .field("iiAddress", &self.iiAddress)
            .field("iiBroadcastAddress", &self.iiBroadcastAddress)
            .field("iiNetmask", &self.iiNetmask)
            .finish()
    }
}
impl ::core::cmp::PartialEq for INTERFACE_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.iiFlags == other.iiFlags
            && self.iiAddress == other.iiAddress
            && self.iiBroadcastAddress == other.iiBroadcastAddress
            && self.iiNetmask == other.iiNetmask
    }
}
impl ::core::cmp::Eq for INTERFACE_INFO_EX {}
impl FromIntoMemory for INTERFACE_INFO_EX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_iiFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_iiAddress = <SOCKET_ADDRESS as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        let f_iiBroadcastAddress =
            <SOCKET_ADDRESS as FromIntoMemory>::from_bytes(&from[12..12 + 8]);
        let f_iiNetmask = <SOCKET_ADDRESS as FromIntoMemory>::from_bytes(&from[20..20 + 8]);
        Self {
            iiFlags: f_iiFlags,
            iiAddress: f_iiAddress,
            iiBroadcastAddress: f_iiBroadcastAddress,
            iiNetmask: f_iiNetmask,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.iiFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.iiAddress, &mut into[4..4 + 8]);
        FromIntoMemory::into_bytes(self.iiBroadcastAddress, &mut into[12..12 + 8]);
        FromIntoMemory::into_bytes(self.iiNetmask, &mut into[20..20 + 8]);
    }
    fn size() -> usize {
        28
    }
}
pub const INVALID_SOCKET: SOCKET = SOCKET(4294967295u32 as _);
pub struct IN_ADDR {
    pub S_un: IN_ADDR_0,
}
impl ::core::marker::Copy for IN_ADDR {}
impl ::core::clone::Clone for IN_ADDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IN_ADDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IN_ADDR").field("S_un", &self.S_un).finish()
    }
}
impl ::core::cmp::PartialEq for IN_ADDR {
    fn eq(&self, other: &Self) -> bool {
        self.S_un == other.S_un
    }
}
impl ::core::cmp::Eq for IN_ADDR {}
impl FromIntoMemory for IN_ADDR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_S_un = <IN_ADDR_0 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { S_un: f_S_un }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.S_un, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct IN_ADDR_0 {
    data: [u8; 4],
}
impl ::core::default::Default for IN_ADDR_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for IN_ADDR_0 {}
impl ::core::clone::Clone for IN_ADDR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IN_ADDR_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IN_ADDR_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IN_ADDR_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for IN_ADDR_0 {}
impl FromIntoMemory for IN_ADDR_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 4];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        4
    }
}
pub struct IN_ADDR_0_0 {
    pub s_b1: u8,
    pub s_b2: u8,
    pub s_b3: u8,
    pub s_b4: u8,
}
impl ::core::marker::Copy for IN_ADDR_0_0 {}
impl ::core::clone::Clone for IN_ADDR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IN_ADDR_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IN_ADDR_0_0")
            .field("s_b1", &self.s_b1)
            .field("s_b2", &self.s_b2)
            .field("s_b3", &self.s_b3)
            .field("s_b4", &self.s_b4)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IN_ADDR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.s_b1 == other.s_b1
            && self.s_b2 == other.s_b2
            && self.s_b3 == other.s_b3
            && self.s_b4 == other.s_b4
    }
}
impl ::core::cmp::Eq for IN_ADDR_0_0 {}
impl FromIntoMemory for IN_ADDR_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_s_b1 = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_s_b2 = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_s_b3 = <u8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        let f_s_b4 = <u8 as FromIntoMemory>::from_bytes(&from[3..3 + 1]);
        Self {
            s_b1: f_s_b1,
            s_b2: f_s_b2,
            s_b3: f_s_b3,
            s_b4: f_s_b4,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.s_b1, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.s_b2, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.s_b3, &mut into[2..2 + 1]);
        FromIntoMemory::into_bytes(self.s_b4, &mut into[3..3 + 1]);
    }
    fn size() -> usize {
        4
    }
}
pub struct IN_ADDR_0_1 {
    pub s_w1: u16,
    pub s_w2: u16,
}
impl ::core::marker::Copy for IN_ADDR_0_1 {}
impl ::core::clone::Clone for IN_ADDR_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IN_ADDR_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IN_ADDR_0_1")
            .field("s_w1", &self.s_w1)
            .field("s_w2", &self.s_w2)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IN_ADDR_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.s_w1 == other.s_w1 && self.s_w2 == other.s_w2
    }
}
impl ::core::cmp::Eq for IN_ADDR_0_1 {}
impl FromIntoMemory for IN_ADDR_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_s_w1 = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_s_w2 = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            s_w1: f_s_w1,
            s_w2: f_s_w2,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.s_w1, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.s_w2, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4
    }
}
pub const IN_CLASSA_HOST: u32 = 16777215u32;
pub const IN_CLASSA_MAX: u32 = 128u32;
pub const IN_CLASSA_NET: u32 = 4278190080u32;
pub const IN_CLASSA_NSHIFT: u32 = 24u32;
pub const IN_CLASSB_HOST: u32 = 65535u32;
pub const IN_CLASSB_MAX: u32 = 65536u32;
pub const IN_CLASSB_NET: u32 = 4294901760u32;
pub const IN_CLASSB_NSHIFT: u32 = 16u32;
pub const IN_CLASSC_HOST: u32 = 255u32;
pub const IN_CLASSC_NET: u32 = 4294967040u32;
pub const IN_CLASSC_NSHIFT: u32 = 8u32;
pub const IN_CLASSD_HOST: u32 = 268435455u32;
pub const IN_CLASSD_NET: u32 = 4026531840u32;
pub const IN_CLASSD_NSHIFT: u32 = 28u32;
pub struct IN_PKTINFO {
    pub ipi_addr: IN_ADDR,
    pub ipi_ifindex: u32,
}
impl ::core::marker::Copy for IN_PKTINFO {}
impl ::core::clone::Clone for IN_PKTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IN_PKTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IN_PKTINFO")
            .field("ipi_addr", &self.ipi_addr)
            .field("ipi_ifindex", &self.ipi_ifindex)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IN_PKTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ipi_addr == other.ipi_addr && self.ipi_ifindex == other.ipi_ifindex
    }
}
impl ::core::cmp::Eq for IN_PKTINFO {}
impl FromIntoMemory for IN_PKTINFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_ipi_addr = <IN_ADDR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ipi_ifindex = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            ipi_addr: f_ipi_addr,
            ipi_ifindex: f_ipi_ifindex,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.ipi_addr, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ipi_ifindex, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct IN_PKTINFO_EX {
    pub pkt_info: IN_PKTINFO,
    pub scope_id: SCOPE_ID,
}
impl ::core::marker::Copy for IN_PKTINFO_EX {}
impl ::core::clone::Clone for IN_PKTINFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IN_PKTINFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IN_PKTINFO_EX")
            .field("pkt_info", &self.pkt_info)
            .field("scope_id", &self.scope_id)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IN_PKTINFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.pkt_info == other.pkt_info && self.scope_id == other.scope_id
    }
}
impl ::core::cmp::Eq for IN_PKTINFO_EX {}
impl FromIntoMemory for IN_PKTINFO_EX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_pkt_info = <IN_PKTINFO as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_scope_id = <SCOPE_ID as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            pkt_info: f_pkt_info,
            scope_id: f_scope_id,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.pkt_info, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.scope_id, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct IN_RECVERR {
    pub protocol: IPPROTO,
    pub info: u32,
    pub r#type: u8,
    pub code: u8,
}
impl ::core::marker::Copy for IN_RECVERR {}
impl ::core::clone::Clone for IN_RECVERR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IN_RECVERR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IN_RECVERR")
            .field("protocol", &self.protocol)
            .field("info", &self.info)
            .field("type", &self.r#type)
            .field("code", &self.code)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IN_RECVERR {
    fn eq(&self, other: &Self) -> bool {
        self.protocol == other.protocol
            && self.info == other.info
            && self.r#type == other.r#type
            && self.code == other.code
    }
}
impl ::core::cmp::Eq for IN_RECVERR {}
impl FromIntoMemory for IN_RECVERR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_protocol = <IPPROTO as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_info = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_type = <u8 as FromIntoMemory>::from_bytes(&from[8..8 + 1]);
        let f_code = <u8 as FromIntoMemory>::from_bytes(&from[9..9 + 1]);
        Self {
            protocol: f_protocol,
            info: f_info,
            r#type: f_type,
            code: f_code,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.protocol, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.info, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.r#type, &mut into[8..8 + 1]);
        FromIntoMemory::into_bytes(self.code, &mut into[9..9 + 1]);
    }
    fn size() -> usize {
        12
    }
}
pub const IOCPARM_MASK: u32 = 127u32;
pub const IOC_IN: u32 = 2147483648u32;
pub const IOC_INOUT: u32 = 3221225472u32;
pub const IOC_OUT: u32 = 1073741824u32;
pub const IOC_PROTOCOL: u32 = 268435456u32;
pub const IOC_UNIX: u32 = 0u32;
pub const IOC_VENDOR: u32 = 402653184u32;
pub const IOC_VOID: u32 = 536870912u32;
pub const IOC_WS2: u32 = 134217728u32;
pub const IP6T_SO_ORIGINAL_DST: u32 = 12303u32;
pub const IPPORT_BIFFUDP: u32 = 512u32;
pub const IPPORT_CHARGEN: u32 = 19u32;
pub const IPPORT_CMDSERVER: u32 = 514u32;
pub const IPPORT_DAYTIME: u32 = 13u32;
pub const IPPORT_DISCARD: u32 = 9u32;
pub const IPPORT_DYNAMIC_MAX: u32 = 65535u32;
pub const IPPORT_DYNAMIC_MIN: u32 = 49152u32;
pub const IPPORT_ECHO: u32 = 7u32;
pub const IPPORT_EFSSERVER: u32 = 520u32;
pub const IPPORT_EPMAP: u32 = 135u32;
pub const IPPORT_EXECSERVER: u32 = 512u32;
pub const IPPORT_FINGER: u32 = 79u32;
pub const IPPORT_FTP: u32 = 21u32;
pub const IPPORT_FTP_DATA: u32 = 20u32;
pub const IPPORT_HTTPS: u32 = 443u32;
pub const IPPORT_IMAP: u32 = 143u32;
pub const IPPORT_IMAP3: u32 = 220u32;
pub const IPPORT_LDAP: u32 = 389u32;
pub const IPPORT_LOGINSERVER: u32 = 513u32;
pub const IPPORT_MICROSOFT_DS: u32 = 445u32;
pub const IPPORT_MSP: u32 = 18u32;
pub const IPPORT_MTP: u32 = 57u32;
pub const IPPORT_NAMESERVER: u32 = 42u32;
pub const IPPORT_NETBIOS_DGM: u32 = 138u32;
pub const IPPORT_NETBIOS_NS: u32 = 137u32;
pub const IPPORT_NETBIOS_SSN: u32 = 139u32;
pub const IPPORT_NETSTAT: u32 = 15u32;
pub const IPPORT_NTP: u32 = 123u32;
pub const IPPORT_POP3: u32 = 110u32;
pub const IPPORT_QOTD: u32 = 17u32;
pub const IPPORT_REGISTERED_MAX: u32 = 49151u32;
pub const IPPORT_REGISTERED_MIN: u32 = 1024u32;
pub const IPPORT_RESERVED: u32 = 1024u32;
pub const IPPORT_RJE: u32 = 77u32;
pub const IPPORT_ROUTESERVER: u32 = 520u32;
pub const IPPORT_SMTP: u32 = 25u32;
pub const IPPORT_SNMP: u32 = 161u32;
pub const IPPORT_SNMP_TRAP: u32 = 162u32;
pub const IPPORT_SUPDUP: u32 = 95u32;
pub const IPPORT_SYSTAT: u32 = 11u32;
pub const IPPORT_TCPMUX: u32 = 1u32;
pub const IPPORT_TELNET: u32 = 23u32;
pub const IPPORT_TFTP: u32 = 69u32;
pub const IPPORT_TIMESERVER: u32 = 37u32;
pub const IPPORT_TTYLINK: u32 = 87u32;
pub const IPPORT_WHOIS: u32 = 43u32;
pub const IPPORT_WHOSERVER: u32 = 513u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPPROTO(pub i32);
pub const IPPROTO_HOPOPTS: IPPROTO = IPPROTO(0i32);
pub const IPPROTO_ICMP: IPPROTO = IPPROTO(1i32);
pub const IPPROTO_IGMP: IPPROTO = IPPROTO(2i32);
pub const IPPROTO_GGP: IPPROTO = IPPROTO(3i32);
pub const IPPROTO_IPV4: IPPROTO = IPPROTO(4i32);
pub const IPPROTO_ST: IPPROTO = IPPROTO(5i32);
pub const IPPROTO_TCP: IPPROTO = IPPROTO(6i32);
pub const IPPROTO_CBT: IPPROTO = IPPROTO(7i32);
pub const IPPROTO_EGP: IPPROTO = IPPROTO(8i32);
pub const IPPROTO_IGP: IPPROTO = IPPROTO(9i32);
pub const IPPROTO_PUP: IPPROTO = IPPROTO(12i32);
pub const IPPROTO_UDP: IPPROTO = IPPROTO(17i32);
pub const IPPROTO_IDP: IPPROTO = IPPROTO(22i32);
pub const IPPROTO_RDP: IPPROTO = IPPROTO(27i32);
pub const IPPROTO_IPV6: IPPROTO = IPPROTO(41i32);
pub const IPPROTO_ROUTING: IPPROTO = IPPROTO(43i32);
pub const IPPROTO_FRAGMENT: IPPROTO = IPPROTO(44i32);
pub const IPPROTO_ESP: IPPROTO = IPPROTO(50i32);
pub const IPPROTO_AH: IPPROTO = IPPROTO(51i32);
pub const IPPROTO_ICMPV6: IPPROTO = IPPROTO(58i32);
pub const IPPROTO_NONE: IPPROTO = IPPROTO(59i32);
pub const IPPROTO_DSTOPTS: IPPROTO = IPPROTO(60i32);
pub const IPPROTO_ND: IPPROTO = IPPROTO(77i32);
pub const IPPROTO_ICLFXBM: IPPROTO = IPPROTO(78i32);
pub const IPPROTO_PIM: IPPROTO = IPPROTO(103i32);
pub const IPPROTO_PGM: IPPROTO = IPPROTO(113i32);
pub const IPPROTO_L2TP: IPPROTO = IPPROTO(115i32);
pub const IPPROTO_SCTP: IPPROTO = IPPROTO(132i32);
pub const IPPROTO_RAW: IPPROTO = IPPROTO(255i32);
pub const IPPROTO_MAX: IPPROTO = IPPROTO(256i32);
pub const IPPROTO_RESERVED_RAW: IPPROTO = IPPROTO(257i32);
pub const IPPROTO_RESERVED_IPSEC: IPPROTO = IPPROTO(258i32);
pub const IPPROTO_RESERVED_IPSECOFFLOAD: IPPROTO = IPPROTO(259i32);
pub const IPPROTO_RESERVED_WNV: IPPROTO = IPPROTO(260i32);
pub const IPPROTO_RESERVED_MAX: IPPROTO = IPPROTO(261i32);
impl ::core::marker::Copy for IPPROTO {}
impl ::core::clone::Clone for IPPROTO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPPROTO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPPROTO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPPROTO").field(&self.0).finish()
    }
}
impl FromIntoMemory for IPPROTO {
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
pub const IPPROTO_IP: u32 = 0u32;
pub const IPPROTO_RM: u32 = 113u32;
pub const IPV6_ADD_IFLIST: u32 = 29u32;
pub const IPV6_ADD_MEMBERSHIP: u32 = 12u32;
pub const IPV6_CHECKSUM: u32 = 26u32;
pub const IPV6_DEL_IFLIST: u32 = 30u32;
pub const IPV6_DONTFRAG: u32 = 14u32;
pub const IPV6_DROP_MEMBERSHIP: u32 = 13u32;
pub const IPV6_ECN: u32 = 50u32;
pub const IPV6_GET_IFLIST: u32 = 33u32;
pub const IPV6_HDRINCL: u32 = 2u32;
pub const IPV6_HOPLIMIT: u32 = 21u32;
pub const IPV6_HOPOPTS: u32 = 1u32;
pub const IPV6_IFLIST: u32 = 28u32;
pub const IPV6_JOIN_GROUP: u32 = 12u32;
pub const IPV6_LEAVE_GROUP: u32 = 13u32;
pub struct IPV6_MREQ {
    pub ipv6mr_multiaddr: IN6_ADDR,
    pub ipv6mr_interface: u32,
}
impl ::core::marker::Copy for IPV6_MREQ {}
impl ::core::clone::Clone for IPV6_MREQ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPV6_MREQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_MREQ")
            .field("ipv6mr_multiaddr", &self.ipv6mr_multiaddr)
            .field("ipv6mr_interface", &self.ipv6mr_interface)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IPV6_MREQ {
    fn eq(&self, other: &Self) -> bool {
        self.ipv6mr_multiaddr == other.ipv6mr_multiaddr
            && self.ipv6mr_interface == other.ipv6mr_interface
    }
}
impl ::core::cmp::Eq for IPV6_MREQ {}
impl FromIntoMemory for IPV6_MREQ {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_ipv6mr_multiaddr = <IN6_ADDR as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_ipv6mr_interface = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            ipv6mr_multiaddr: f_ipv6mr_multiaddr,
            ipv6mr_interface: f_ipv6mr_interface,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.ipv6mr_multiaddr, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.ipv6mr_interface, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub const IPV6_MTU: u32 = 72u32;
pub const IPV6_MTU_DISCOVER: u32 = 71u32;
pub const IPV6_MULTICAST_HOPS: u32 = 10u32;
pub const IPV6_MULTICAST_IF: u32 = 9u32;
pub const IPV6_MULTICAST_LOOP: u32 = 11u32;
pub const IPV6_NRT_INTERFACE: u32 = 74u32;
pub const IPV6_PKTINFO: u32 = 19u32;
pub const IPV6_PKTINFO_EX: u32 = 51u32;
pub const IPV6_PROTECTION_LEVEL: u32 = 23u32;
pub const IPV6_RECVDSTADDR: u32 = 25u32;
pub const IPV6_RECVECN: u32 = 50u32;
pub const IPV6_RECVERR: u32 = 75u32;
pub const IPV6_RECVIF: u32 = 24u32;
pub const IPV6_RECVRTHDR: u32 = 38u32;
pub const IPV6_RECVTCLASS: u32 = 40u32;
pub const IPV6_RTHDR: u32 = 32u32;
pub const IPV6_TCLASS: u32 = 39u32;
pub const IPV6_UNICAST_HOPS: u32 = 4u32;
pub const IPV6_UNICAST_IF: u32 = 31u32;
pub const IPV6_USER_MTU: u32 = 76u32;
pub const IPV6_V6ONLY: u32 = 27u32;
pub const IPV6_WFP_REDIRECT_CONTEXT: u32 = 70u32;
pub const IPV6_WFP_REDIRECT_RECORDS: u32 = 60u32;
pub const IPX_ADDRESS: u32 = 16391u32;
pub struct IPX_ADDRESS_DATA {
    pub adapternum: i32,
    pub netnum: [u8; 4],
    pub nodenum: [u8; 6],
    pub wan: super::super::Foundation::BOOLEAN,
    pub status: super::super::Foundation::BOOLEAN,
    pub maxpkt: i32,
    pub linkspeed: u32,
}
impl ::core::marker::Copy for IPX_ADDRESS_DATA {}
impl ::core::clone::Clone for IPX_ADDRESS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPX_ADDRESS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPX_ADDRESS_DATA")
            .field("adapternum", &self.adapternum)
            .field("netnum", &self.netnum)
            .field("nodenum", &self.nodenum)
            .field("wan", &self.wan)
            .field("status", &self.status)
            .field("maxpkt", &self.maxpkt)
            .field("linkspeed", &self.linkspeed)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IPX_ADDRESS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.adapternum == other.adapternum
            && self.netnum == other.netnum
            && self.nodenum == other.nodenum
            && self.wan == other.wan
            && self.status == other.status
            && self.maxpkt == other.maxpkt
            && self.linkspeed == other.linkspeed
    }
}
impl ::core::cmp::Eq for IPX_ADDRESS_DATA {}
impl FromIntoMemory for IPX_ADDRESS_DATA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_adapternum = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_netnum = <[u8; 4] as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_nodenum = <[u8; 6] as FromIntoMemory>::from_bytes(&from[8..8 + 6]);
        let f_wan =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[14..14 + 1]);
        let f_status =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[15..15 + 1]);
        let f_maxpkt = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_linkspeed = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            adapternum: f_adapternum,
            netnum: f_netnum,
            nodenum: f_nodenum,
            wan: f_wan,
            status: f_status,
            maxpkt: f_maxpkt,
            linkspeed: f_linkspeed,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.adapternum, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.netnum, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.nodenum, &mut into[8..8 + 6]);
        FromIntoMemory::into_bytes(self.wan, &mut into[14..14 + 1]);
        FromIntoMemory::into_bytes(self.status, &mut into[15..15 + 1]);
        FromIntoMemory::into_bytes(self.maxpkt, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.linkspeed, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub const IPX_ADDRESS_NOTIFY: u32 = 16396u32;
pub const IPX_DSTYPE: u32 = 16386u32;
pub const IPX_EXTENDED_ADDRESS: u32 = 16388u32;
pub const IPX_FILTERPTYPE: u32 = 16385u32;
pub const IPX_GETNETINFO: u32 = 16392u32;
pub const IPX_GETNETINFO_NORIP: u32 = 16393u32;
pub const IPX_IMMEDIATESPXACK: u32 = 16400u32;
pub const IPX_MAXSIZE: u32 = 16390u32;
pub const IPX_MAX_ADAPTER_NUM: u32 = 16397u32;
pub struct IPX_NETNUM_DATA {
    pub netnum: [u8; 4],
    pub hopcount: u16,
    pub netdelay: u16,
    pub cardnum: i32,
    pub router: [u8; 6],
}
impl ::core::marker::Copy for IPX_NETNUM_DATA {}
impl ::core::clone::Clone for IPX_NETNUM_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPX_NETNUM_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPX_NETNUM_DATA")
            .field("netnum", &self.netnum)
            .field("hopcount", &self.hopcount)
            .field("netdelay", &self.netdelay)
            .field("cardnum", &self.cardnum)
            .field("router", &self.router)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IPX_NETNUM_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.netnum == other.netnum
            && self.hopcount == other.hopcount
            && self.netdelay == other.netdelay
            && self.cardnum == other.cardnum
            && self.router == other.router
    }
}
impl ::core::cmp::Eq for IPX_NETNUM_DATA {}
impl FromIntoMemory for IPX_NETNUM_DATA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_netnum = <[u8; 4] as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_hopcount = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_netdelay = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_cardnum = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_router = <[u8; 6] as FromIntoMemory>::from_bytes(&from[12..12 + 6]);
        Self {
            netnum: f_netnum,
            hopcount: f_hopcount,
            netdelay: f_netdelay,
            cardnum: f_cardnum,
            router: f_router,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.netnum, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.hopcount, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.netdelay, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.cardnum, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.router, &mut into[12..12 + 6]);
    }
    fn size() -> usize {
        20
    }
}
pub const IPX_PTYPE: u32 = 16384u32;
pub const IPX_RECEIVE_BROADCAST: u32 = 16399u32;
pub const IPX_RECVHDR: u32 = 16389u32;
pub const IPX_RERIPNETNUMBER: u32 = 16398u32;
pub struct IPX_SPXCONNSTATUS_DATA {
    pub ConnectionState: u8,
    pub WatchDogActive: u8,
    pub LocalConnectionId: u16,
    pub RemoteConnectionId: u16,
    pub LocalSequenceNumber: u16,
    pub LocalAckNumber: u16,
    pub LocalAllocNumber: u16,
    pub RemoteAckNumber: u16,
    pub RemoteAllocNumber: u16,
    pub LocalSocket: u16,
    pub ImmediateAddress: [u8; 6],
    pub RemoteNetwork: [u8; 4],
    pub RemoteNode: [u8; 6],
    pub RemoteSocket: u16,
    pub RetransmissionCount: u16,
    pub EstimatedRoundTripDelay: u16,
    pub RetransmittedPackets: u16,
    pub SuppressedPacket: u16,
}
impl ::core::marker::Copy for IPX_SPXCONNSTATUS_DATA {}
impl ::core::clone::Clone for IPX_SPXCONNSTATUS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPX_SPXCONNSTATUS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPX_SPXCONNSTATUS_DATA")
            .field("ConnectionState", &self.ConnectionState)
            .field("WatchDogActive", &self.WatchDogActive)
            .field("LocalConnectionId", &self.LocalConnectionId)
            .field("RemoteConnectionId", &self.RemoteConnectionId)
            .field("LocalSequenceNumber", &self.LocalSequenceNumber)
            .field("LocalAckNumber", &self.LocalAckNumber)
            .field("LocalAllocNumber", &self.LocalAllocNumber)
            .field("RemoteAckNumber", &self.RemoteAckNumber)
            .field("RemoteAllocNumber", &self.RemoteAllocNumber)
            .field("LocalSocket", &self.LocalSocket)
            .field("ImmediateAddress", &self.ImmediateAddress)
            .field("RemoteNetwork", &self.RemoteNetwork)
            .field("RemoteNode", &self.RemoteNode)
            .field("RemoteSocket", &self.RemoteSocket)
            .field("RetransmissionCount", &self.RetransmissionCount)
            .field("EstimatedRoundTripDelay", &self.EstimatedRoundTripDelay)
            .field("RetransmittedPackets", &self.RetransmittedPackets)
            .field("SuppressedPacket", &self.SuppressedPacket)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IPX_SPXCONNSTATUS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionState == other.ConnectionState
            && self.WatchDogActive == other.WatchDogActive
            && self.LocalConnectionId == other.LocalConnectionId
            && self.RemoteConnectionId == other.RemoteConnectionId
            && self.LocalSequenceNumber == other.LocalSequenceNumber
            && self.LocalAckNumber == other.LocalAckNumber
            && self.LocalAllocNumber == other.LocalAllocNumber
            && self.RemoteAckNumber == other.RemoteAckNumber
            && self.RemoteAllocNumber == other.RemoteAllocNumber
            && self.LocalSocket == other.LocalSocket
            && self.ImmediateAddress == other.ImmediateAddress
            && self.RemoteNetwork == other.RemoteNetwork
            && self.RemoteNode == other.RemoteNode
            && self.RemoteSocket == other.RemoteSocket
            && self.RetransmissionCount == other.RetransmissionCount
            && self.EstimatedRoundTripDelay == other.EstimatedRoundTripDelay
            && self.RetransmittedPackets == other.RetransmittedPackets
            && self.SuppressedPacket == other.SuppressedPacket
    }
}
impl ::core::cmp::Eq for IPX_SPXCONNSTATUS_DATA {}
impl FromIntoMemory for IPX_SPXCONNSTATUS_DATA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_ConnectionState = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_WatchDogActive = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_LocalConnectionId = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_RemoteConnectionId = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_LocalSequenceNumber = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_LocalAckNumber = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_LocalAllocNumber = <u16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        let f_RemoteAckNumber = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        let f_RemoteAllocNumber = <u16 as FromIntoMemory>::from_bytes(&from[14..14 + 2]);
        let f_LocalSocket = <u16 as FromIntoMemory>::from_bytes(&from[16..16 + 2]);
        let f_ImmediateAddress = <[u8; 6] as FromIntoMemory>::from_bytes(&from[18..18 + 6]);
        let f_RemoteNetwork = <[u8; 4] as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_RemoteNode = <[u8; 6] as FromIntoMemory>::from_bytes(&from[28..28 + 6]);
        let f_RemoteSocket = <u16 as FromIntoMemory>::from_bytes(&from[34..34 + 2]);
        let f_RetransmissionCount = <u16 as FromIntoMemory>::from_bytes(&from[36..36 + 2]);
        let f_EstimatedRoundTripDelay = <u16 as FromIntoMemory>::from_bytes(&from[38..38 + 2]);
        let f_RetransmittedPackets = <u16 as FromIntoMemory>::from_bytes(&from[40..40 + 2]);
        let f_SuppressedPacket = <u16 as FromIntoMemory>::from_bytes(&from[42..42 + 2]);
        Self {
            ConnectionState: f_ConnectionState,
            WatchDogActive: f_WatchDogActive,
            LocalConnectionId: f_LocalConnectionId,
            RemoteConnectionId: f_RemoteConnectionId,
            LocalSequenceNumber: f_LocalSequenceNumber,
            LocalAckNumber: f_LocalAckNumber,
            LocalAllocNumber: f_LocalAllocNumber,
            RemoteAckNumber: f_RemoteAckNumber,
            RemoteAllocNumber: f_RemoteAllocNumber,
            LocalSocket: f_LocalSocket,
            ImmediateAddress: f_ImmediateAddress,
            RemoteNetwork: f_RemoteNetwork,
            RemoteNode: f_RemoteNode,
            RemoteSocket: f_RemoteSocket,
            RetransmissionCount: f_RetransmissionCount,
            EstimatedRoundTripDelay: f_EstimatedRoundTripDelay,
            RetransmittedPackets: f_RetransmittedPackets,
            SuppressedPacket: f_SuppressedPacket,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.ConnectionState, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.WatchDogActive, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.LocalConnectionId, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.RemoteConnectionId, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.LocalSequenceNumber, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.LocalAckNumber, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.LocalAllocNumber, &mut into[10..10 + 2]);
        FromIntoMemory::into_bytes(self.RemoteAckNumber, &mut into[12..12 + 2]);
        FromIntoMemory::into_bytes(self.RemoteAllocNumber, &mut into[14..14 + 2]);
        FromIntoMemory::into_bytes(self.LocalSocket, &mut into[16..16 + 2]);
        FromIntoMemory::into_bytes(self.ImmediateAddress, &mut into[18..18 + 6]);
        FromIntoMemory::into_bytes(self.RemoteNetwork, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.RemoteNode, &mut into[28..28 + 6]);
        FromIntoMemory::into_bytes(self.RemoteSocket, &mut into[34..34 + 2]);
        FromIntoMemory::into_bytes(self.RetransmissionCount, &mut into[36..36 + 2]);
        FromIntoMemory::into_bytes(self.EstimatedRoundTripDelay, &mut into[38..38 + 2]);
        FromIntoMemory::into_bytes(self.RetransmittedPackets, &mut into[40..40 + 2]);
        FromIntoMemory::into_bytes(self.SuppressedPacket, &mut into[42..42 + 2]);
    }
    fn size() -> usize {
        44
    }
}
pub const IPX_SPXGETCONNECTIONSTATUS: u32 = 16395u32;
pub const IPX_STOPFILTERPTYPE: u32 = 16387u32;
pub const IP_ADD_IFLIST: u32 = 29u32;
pub const IP_ADD_MEMBERSHIP: u32 = 12u32;
pub const IP_ADD_SOURCE_MEMBERSHIP: u32 = 15u32;
pub const IP_BLOCK_SOURCE: u32 = 17u32;
pub const IP_DEFAULT_MULTICAST_LOOP: u32 = 1u32;
pub const IP_DEFAULT_MULTICAST_TTL: u32 = 1u32;
pub const IP_DEL_IFLIST: u32 = 30u32;
pub const IP_DONTFRAGMENT: u32 = 14u32;
pub const IP_DROP_MEMBERSHIP: u32 = 13u32;
pub const IP_DROP_SOURCE_MEMBERSHIP: u32 = 16u32;
pub const IP_ECN: u32 = 50u32;
pub const IP_GET_IFLIST: u32 = 33u32;
pub const IP_HDRINCL: u32 = 2u32;
pub const IP_HOPLIMIT: u32 = 21u32;
pub const IP_IFLIST: u32 = 28u32;
pub const IP_MAX_MEMBERSHIPS: u32 = 20u32;
pub struct IP_MREQ {
    pub imr_multiaddr: IN_ADDR,
    pub imr_interface: IN_ADDR,
}
impl ::core::marker::Copy for IP_MREQ {}
impl ::core::clone::Clone for IP_MREQ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IP_MREQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_MREQ")
            .field("imr_multiaddr", &self.imr_multiaddr)
            .field("imr_interface", &self.imr_interface)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IP_MREQ {
    fn eq(&self, other: &Self) -> bool {
        self.imr_multiaddr == other.imr_multiaddr && self.imr_interface == other.imr_interface
    }
}
impl ::core::cmp::Eq for IP_MREQ {}
impl FromIntoMemory for IP_MREQ {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_imr_multiaddr = <IN_ADDR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_imr_interface = <IN_ADDR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            imr_multiaddr: f_imr_multiaddr,
            imr_interface: f_imr_interface,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.imr_multiaddr, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.imr_interface, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct IP_MREQ_SOURCE {
    pub imr_multiaddr: IN_ADDR,
    pub imr_sourceaddr: IN_ADDR,
    pub imr_interface: IN_ADDR,
}
impl ::core::marker::Copy for IP_MREQ_SOURCE {}
impl ::core::clone::Clone for IP_MREQ_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IP_MREQ_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_MREQ_SOURCE")
            .field("imr_multiaddr", &self.imr_multiaddr)
            .field("imr_sourceaddr", &self.imr_sourceaddr)
            .field("imr_interface", &self.imr_interface)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IP_MREQ_SOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.imr_multiaddr == other.imr_multiaddr
            && self.imr_sourceaddr == other.imr_sourceaddr
            && self.imr_interface == other.imr_interface
    }
}
impl ::core::cmp::Eq for IP_MREQ_SOURCE {}
impl FromIntoMemory for IP_MREQ_SOURCE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_imr_multiaddr = <IN_ADDR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_imr_sourceaddr = <IN_ADDR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_imr_interface = <IN_ADDR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            imr_multiaddr: f_imr_multiaddr,
            imr_sourceaddr: f_imr_sourceaddr,
            imr_interface: f_imr_interface,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.imr_multiaddr, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.imr_sourceaddr, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.imr_interface, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct IP_MSFILTER {
    pub imsf_multiaddr: IN_ADDR,
    pub imsf_interface: IN_ADDR,
    pub imsf_fmode: MULTICAST_MODE_TYPE,
    pub imsf_numsrc: u32,
    pub imsf_slist: [IN_ADDR; 1],
}
impl ::core::marker::Copy for IP_MSFILTER {}
impl ::core::clone::Clone for IP_MSFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IP_MSFILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_MSFILTER")
            .field("imsf_multiaddr", &self.imsf_multiaddr)
            .field("imsf_interface", &self.imsf_interface)
            .field("imsf_fmode", &self.imsf_fmode)
            .field("imsf_numsrc", &self.imsf_numsrc)
            .field("imsf_slist", &self.imsf_slist)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IP_MSFILTER {
    fn eq(&self, other: &Self) -> bool {
        self.imsf_multiaddr == other.imsf_multiaddr
            && self.imsf_interface == other.imsf_interface
            && self.imsf_fmode == other.imsf_fmode
            && self.imsf_numsrc == other.imsf_numsrc
            && self.imsf_slist == other.imsf_slist
    }
}
impl ::core::cmp::Eq for IP_MSFILTER {}
impl FromIntoMemory for IP_MSFILTER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_imsf_multiaddr = <IN_ADDR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_imsf_interface = <IN_ADDR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_imsf_fmode = <MULTICAST_MODE_TYPE as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_imsf_numsrc = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_imsf_slist = <[IN_ADDR; 1] as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            imsf_multiaddr: f_imsf_multiaddr,
            imsf_interface: f_imsf_interface,
            imsf_fmode: f_imsf_fmode,
            imsf_numsrc: f_imsf_numsrc,
            imsf_slist: f_imsf_slist,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.imsf_multiaddr, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.imsf_interface, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.imsf_fmode, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.imsf_numsrc, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.imsf_slist, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub const IP_MTU: u32 = 73u32;
pub const IP_MTU_DISCOVER: u32 = 71u32;
pub const IP_MULTICAST_IF: u32 = 9u32;
pub const IP_MULTICAST_LOOP: u32 = 11u32;
pub const IP_MULTICAST_TTL: u32 = 10u32;
pub const IP_NRT_INTERFACE: u32 = 74u32;
pub const IP_OPTIONS: u32 = 1u32;
pub const IP_ORIGINAL_ARRIVAL_IF: u32 = 47u32;
pub const IP_PKTINFO: u32 = 19u32;
pub const IP_PKTINFO_EX: u32 = 51u32;
pub const IP_PROTECTION_LEVEL: u32 = 23u32;
pub const IP_RECEIVE_BROADCAST: u32 = 22u32;
pub const IP_RECVDSTADDR: u32 = 25u32;
pub const IP_RECVECN: u32 = 50u32;
pub const IP_RECVERR: u32 = 75u32;
pub const IP_RECVIF: u32 = 24u32;
pub const IP_RECVRTHDR: u32 = 38u32;
pub const IP_RECVTCLASS: u32 = 40u32;
pub const IP_RECVTOS: u32 = 40u32;
pub const IP_RECVTTL: u32 = 21u32;
pub const IP_RTHDR: u32 = 32u32;
pub const IP_TCLASS: u32 = 39u32;
pub const IP_TOS: u32 = 3u32;
pub const IP_TTL: u32 = 4u32;
pub const IP_UNBLOCK_SOURCE: u32 = 18u32;
pub const IP_UNICAST_IF: u32 = 31u32;
pub const IP_UNSPECIFIED_HOP_LIMIT: i32 = -1i32;
pub const IP_UNSPECIFIED_TYPE_OF_SERVICE: i32 = -1i32;
pub const IP_UNSPECIFIED_USER_MTU: u32 = 4294967295u32;
pub const IP_USER_MTU: u32 = 76u32;
pub const IP_WFP_REDIRECT_CONTEXT: u32 = 70u32;
pub const IP_WFP_REDIRECT_RECORDS: u32 = 60u32;
pub const IRDA_PROTO_SOCK_STREAM: u32 = 1u32;
pub const IRLMP_9WIRE_MODE: u32 = 22u32;
pub const IRLMP_DISCOVERY_MODE: u32 = 25u32;
pub const IRLMP_ENUMDEVICES: u32 = 16u32;
pub const IRLMP_EXCLUSIVE_MODE: u32 = 20u32;
pub const IRLMP_IAS_QUERY: u32 = 18u32;
pub const IRLMP_IAS_SET: u32 = 17u32;
pub const IRLMP_IRLPT_MODE: u32 = 21u32;
pub const IRLMP_PARAMETERS: u32 = 24u32;
pub const IRLMP_SEND_PDU_LEN: u32 = 19u32;
pub const IRLMP_SHARP_MODE: u32 = 32u32;
pub const IRLMP_TINYTP_MODE: u32 = 23u32;
pub const ISOPROTO_CLNP: u32 = 31u32;
pub const ISOPROTO_CLTP: u32 = 30u32;
pub const ISOPROTO_ESIS: u32 = 34u32;
pub const ISOPROTO_INACT_NL: u32 = 33u32;
pub const ISOPROTO_INTRAISIS: u32 = 35u32;
pub const ISOPROTO_TP: u32 = 29u32;
pub const ISOPROTO_TP0: u32 = 25u32;
pub const ISOPROTO_TP1: u32 = 26u32;
pub const ISOPROTO_TP2: u32 = 27u32;
pub const ISOPROTO_TP3: u32 = 28u32;
pub const ISOPROTO_TP4: u32 = 29u32;
pub const ISOPROTO_X25: u32 = 32u32;
pub const ISO_EXP_DATA_NUSE: u32 = 1u32;
pub const ISO_EXP_DATA_USE: u32 = 0u32;
pub const ISO_HIERARCHICAL: u32 = 0u32;
pub const ISO_MAX_ADDR_LENGTH: u32 = 64u32;
pub const ISO_NON_HIERARCHICAL: u32 = 1u32;
pub const JL_BOTH: u32 = 4u32;
pub const JL_RECEIVER_ONLY: u32 = 2u32;
pub const JL_SENDER_ONLY: u32 = 1u32;
pub const LAYERED_PROTOCOL: u32 = 0u32;
pub const LITTLEENDIAN: u32 = 1u32;
pub const LM_BAUD_115200: u32 = 115200u32;
pub const LM_BAUD_1152K: u32 = 1152000u32;
pub const LM_BAUD_1200: u32 = 1200u32;
pub const LM_BAUD_16M: u32 = 16000000u32;
pub const LM_BAUD_19200: u32 = 19200u32;
pub const LM_BAUD_2400: u32 = 2400u32;
pub const LM_BAUD_38400: u32 = 38400u32;
pub const LM_BAUD_4M: u32 = 4000000u32;
pub const LM_BAUD_57600: u32 = 57600u32;
pub const LM_BAUD_576K: u32 = 576000u32;
pub const LM_BAUD_9600: u32 = 9600u32;
pub const LM_HB1_Computer: i32 = 4i32;
pub const LM_HB1_Fax: i32 = 32i32;
pub const LM_HB1_LANAccess: i32 = 64i32;
pub const LM_HB1_Modem: i32 = 16i32;
pub const LM_HB1_PDA_Palmtop: i32 = 2i32;
pub const LM_HB1_PnP: i32 = 1i32;
pub const LM_HB1_Printer: i32 = 8i32;
pub const LM_HB2_FileServer: i32 = 2i32;
pub const LM_HB2_Telephony: i32 = 1i32;
pub const LM_HB_Extension: i32 = 128i32;
pub struct LM_IRPARMS {
    pub nTXDataBytes: u32,
    pub nRXDataBytes: u32,
    pub nBaudRate: u32,
    pub thresholdTime: u32,
    pub discTime: u32,
    pub nMSLinkTurn: u16,
    pub nTXPackets: u8,
    pub nRXPackets: u8,
}
impl ::core::marker::Copy for LM_IRPARMS {}
impl ::core::clone::Clone for LM_IRPARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LM_IRPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LM_IRPARMS")
            .field("nTXDataBytes", &self.nTXDataBytes)
            .field("nRXDataBytes", &self.nRXDataBytes)
            .field("nBaudRate", &self.nBaudRate)
            .field("thresholdTime", &self.thresholdTime)
            .field("discTime", &self.discTime)
            .field("nMSLinkTurn", &self.nMSLinkTurn)
            .field("nTXPackets", &self.nTXPackets)
            .field("nRXPackets", &self.nRXPackets)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LM_IRPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.nTXDataBytes == other.nTXDataBytes
            && self.nRXDataBytes == other.nRXDataBytes
            && self.nBaudRate == other.nBaudRate
            && self.thresholdTime == other.thresholdTime
            && self.discTime == other.discTime
            && self.nMSLinkTurn == other.nMSLinkTurn
            && self.nTXPackets == other.nTXPackets
            && self.nRXPackets == other.nRXPackets
    }
}
impl ::core::cmp::Eq for LM_IRPARMS {}
impl FromIntoMemory for LM_IRPARMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_nTXDataBytes = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_nRXDataBytes = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_nBaudRate = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_thresholdTime = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_discTime = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_nMSLinkTurn = <u16 as FromIntoMemory>::from_bytes(&from[20..20 + 2]);
        let f_nTXPackets = <u8 as FromIntoMemory>::from_bytes(&from[22..22 + 1]);
        let f_nRXPackets = <u8 as FromIntoMemory>::from_bytes(&from[23..23 + 1]);
        Self {
            nTXDataBytes: f_nTXDataBytes,
            nRXDataBytes: f_nRXDataBytes,
            nBaudRate: f_nBaudRate,
            thresholdTime: f_thresholdTime,
            discTime: f_discTime,
            nMSLinkTurn: f_nMSLinkTurn,
            nTXPackets: f_nTXPackets,
            nRXPackets: f_nRXPackets,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.nTXDataBytes, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.nRXDataBytes, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.nBaudRate, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.thresholdTime, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.discTime, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.nMSLinkTurn, &mut into[20..20 + 2]);
        FromIntoMemory::into_bytes(self.nTXPackets, &mut into[22..22 + 1]);
        FromIntoMemory::into_bytes(self.nRXPackets, &mut into[23..23 + 1]);
    }
    fn size() -> usize {
        24
    }
}
pub const LOG2_BITS_PER_BYTE: u32 = 3u32;
pub type LPBLOCKINGCALLBACK = StdCallFnPtr<(PtrRepr,), super::super::Foundation::BOOL>;
#[doc = "*Required namespaces: 'Windows.Win32.NetworkManagement.QoS'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub type LPCONDITIONPROC = StdCallFnPtr<
    (
        MutPtr<WSABUF>,
        MutPtr<WSABUF>,
        MutPtr<super::super::NetworkManagement::QoS::QOS>,
        MutPtr<super::super::NetworkManagement::QoS::QOS>,
        MutPtr<WSABUF>,
        MutPtr<WSABUF>,
        MutPtr<u32>,
        PtrRepr,
    ),
    i32,
>;
pub type LPFN_ACCEPTEX = StdCallFnPtr<
    (
        SOCKET,
        SOCKET,
        MutPtr<::core::ffi::c_void>,
        u32,
        u32,
        u32,
        MutPtr<u32>,
        MutPtr<super::super::System::IO::OVERLAPPED>,
    ),
    super::super::Foundation::BOOL,
>;
pub type LPFN_CONNECTEX = StdCallFnPtr<
    (
        SOCKET,
        ConstPtr<SOCKADDR>,
        i32,
        ConstPtr<::core::ffi::c_void>,
        u32,
        MutPtr<u32>,
        MutPtr<super::super::System::IO::OVERLAPPED>,
    ),
    super::super::Foundation::BOOL,
>;
pub type LPFN_DISCONNECTEX = StdCallFnPtr<
    (
        SOCKET,
        MutPtr<super::super::System::IO::OVERLAPPED>,
        u32,
        u32,
    ),
    super::super::Foundation::BOOL,
>;
pub type LPFN_GETACCEPTEXSOCKADDRS = StdCallFnPtr<
    (
        ConstPtr<::core::ffi::c_void>,
        u32,
        u32,
        u32,
        MutPtr<ConstPtr<SOCKADDR>>,
        MutPtr<i32>,
        MutPtr<ConstPtr<SOCKADDR>>,
        MutPtr<i32>,
    ),
    (),
>;
pub type LPFN_NSPAPI = StdCallFnPtr<(), u32>;
pub type LPFN_RIOCLOSECOMPLETIONQUEUE = StdCallFnPtr<(ConstPtr<RIO_CQ_t>,), ()>;
pub type LPFN_RIOCREATECOMPLETIONQUEUE =
    StdCallFnPtr<(u32, ConstPtr<RIO_NOTIFICATION_COMPLETION>), MutPtr<RIO_CQ_t>>;
pub type LPFN_RIOCREATEREQUESTQUEUE = StdCallFnPtr<
    (
        SOCKET,
        u32,
        u32,
        u32,
        u32,
        ConstPtr<RIO_CQ_t>,
        ConstPtr<RIO_CQ_t>,
        ConstPtr<::core::ffi::c_void>,
    ),
    MutPtr<RIO_RQ_t>,
>;
pub type LPFN_RIODEQUEUECOMPLETION =
    StdCallFnPtr<(ConstPtr<RIO_CQ_t>, MutPtr<RIORESULT>, u32), u32>;
pub type LPFN_RIODEREGISTERBUFFER = StdCallFnPtr<(ConstPtr<RIO_BUFFERID_t>,), ()>;
pub type LPFN_RIONOTIFY = StdCallFnPtr<(ConstPtr<RIO_CQ_t>,), i32>;
pub type LPFN_RIORECEIVE = StdCallFnPtr<
    (
        ConstPtr<RIO_RQ_t>,
        ConstPtr<RIO_BUF>,
        u32,
        u32,
        ConstPtr<::core::ffi::c_void>,
    ),
    super::super::Foundation::BOOL,
>;
pub type LPFN_RIORECEIVEEX = StdCallFnPtr<
    (
        ConstPtr<RIO_RQ_t>,
        ConstPtr<RIO_BUF>,
        u32,
        ConstPtr<RIO_BUF>,
        ConstPtr<RIO_BUF>,
        ConstPtr<RIO_BUF>,
        ConstPtr<RIO_BUF>,
        u32,
        ConstPtr<::core::ffi::c_void>,
    ),
    i32,
>;
pub type LPFN_RIOREGISTERBUFFER = StdCallFnPtr<(PCSTR, u32), MutPtr<RIO_BUFFERID_t>>;
pub type LPFN_RIORESIZECOMPLETIONQUEUE =
    StdCallFnPtr<(ConstPtr<RIO_CQ_t>, u32), super::super::Foundation::BOOL>;
pub type LPFN_RIORESIZEREQUESTQUEUE =
    StdCallFnPtr<(ConstPtr<RIO_RQ_t>, u32, u32), super::super::Foundation::BOOL>;
pub type LPFN_RIOSEND = StdCallFnPtr<
    (
        ConstPtr<RIO_RQ_t>,
        ConstPtr<RIO_BUF>,
        u32,
        u32,
        ConstPtr<::core::ffi::c_void>,
    ),
    super::super::Foundation::BOOL,
>;
pub type LPFN_RIOSENDEX = StdCallFnPtr<
    (
        ConstPtr<RIO_RQ_t>,
        ConstPtr<RIO_BUF>,
        u32,
        ConstPtr<RIO_BUF>,
        ConstPtr<RIO_BUF>,
        ConstPtr<RIO_BUF>,
        ConstPtr<RIO_BUF>,
        u32,
        ConstPtr<::core::ffi::c_void>,
    ),
    super::super::Foundation::BOOL,
>;
pub type LPFN_TRANSMITFILE = StdCallFnPtr<
    (
        SOCKET,
        super::super::Foundation::HANDLE,
        u32,
        u32,
        MutPtr<super::super::System::IO::OVERLAPPED>,
        ConstPtr<TRANSMIT_FILE_BUFFERS>,
        u32,
    ),
    super::super::Foundation::BOOL,
>;
pub type LPFN_TRANSMITPACKETS = StdCallFnPtr<
    (
        SOCKET,
        ConstPtr<TRANSMIT_PACKETS_ELEMENT>,
        u32,
        u32,
        MutPtr<super::super::System::IO::OVERLAPPED>,
        u32,
    ),
    super::super::Foundation::BOOL,
>;
pub type LPFN_WSAPOLL = StdCallFnPtr<(MutPtr<WSAPOLLFD>, u32, i32), i32>;
pub type LPFN_WSARECVMSG = StdCallFnPtr<
    (
        SOCKET,
        MutPtr<WSAMSG>,
        MutPtr<u32>,
        MutPtr<super::super::System::IO::OVERLAPPED>,
        LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ),
    i32,
>;
pub type LPFN_WSASENDMSG = StdCallFnPtr<
    (
        SOCKET,
        ConstPtr<WSAMSG>,
        u32,
        MutPtr<u32>,
        MutPtr<super::super::System::IO::OVERLAPPED>,
        LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ),
    i32,
>;
pub type LPLOOKUPSERVICE_COMPLETION_ROUTINE =
    StdCallFnPtr<(u32, u32, ConstPtr<super::super::System::IO::OVERLAPPED>), ()>;
pub type LPNSPCLEANUP = StdCallFnPtr<(ConstPtr<crate::core::GUID>,), i32>;
pub type LPNSPGETSERVICECLASSINFO = StdCallFnPtr<
    (
        ConstPtr<crate::core::GUID>,
        ConstPtr<u32>,
        ConstPtr<WSASERVICECLASSINFOW>,
    ),
    i32,
>;
pub type LPNSPINSTALLSERVICECLASS =
    StdCallFnPtr<(ConstPtr<crate::core::GUID>, ConstPtr<WSASERVICECLASSINFOW>), i32>;
pub type LPNSPIOCTL = StdCallFnPtr<
    (
        super::super::Foundation::HANDLE,
        u32,
        ConstPtr<::core::ffi::c_void>,
        u32,
        MutPtr<::core::ffi::c_void>,
        u32,
        MutPtr<u32>,
        ConstPtr<WSACOMPLETION>,
        ConstPtr<WSATHREADID>,
    ),
    i32,
>;
pub type LPNSPLOOKUPSERVICEBEGIN = StdCallFnPtr<
    (
        ConstPtr<crate::core::GUID>,
        ConstPtr<WSAQUERYSETW>,
        ConstPtr<WSASERVICECLASSINFOW>,
        u32,
        MutPtr<super::super::Foundation::HANDLE>,
    ),
    i32,
>;
pub type LPNSPLOOKUPSERVICEEND = StdCallFnPtr<(super::super::Foundation::HANDLE,), i32>;
pub type LPNSPLOOKUPSERVICENEXT = StdCallFnPtr<
    (
        super::super::Foundation::HANDLE,
        u32,
        MutPtr<u32>,
        MutPtr<WSAQUERYSETW>,
    ),
    i32,
>;
pub type LPNSPREMOVESERVICECLASS =
    StdCallFnPtr<(ConstPtr<crate::core::GUID>, ConstPtr<crate::core::GUID>), i32>;
pub type LPNSPSETSERVICE = StdCallFnPtr<
    (
        ConstPtr<crate::core::GUID>,
        ConstPtr<WSASERVICECLASSINFOW>,
        ConstPtr<WSAQUERYSETW>,
        WSAESETSERVICEOP,
        u32,
    ),
    i32,
>;
pub type LPNSPSTARTUP = StdCallFnPtr<(ConstPtr<crate::core::GUID>, MutPtr<NSP_ROUTINE>), i32>;
pub type LPNSPV2CLEANUP =
    StdCallFnPtr<(ConstPtr<crate::core::GUID>, ConstPtr<::core::ffi::c_void>), i32>;
pub type LPNSPV2CLIENTSESSIONRUNDOWN =
    StdCallFnPtr<(ConstPtr<crate::core::GUID>, ConstPtr<::core::ffi::c_void>), ()>;
pub type LPNSPV2LOOKUPSERVICEBEGIN = StdCallFnPtr<
    (
        ConstPtr<crate::core::GUID>,
        ConstPtr<WSAQUERYSET2W>,
        u32,
        ConstPtr<::core::ffi::c_void>,
        MutPtr<super::super::Foundation::HANDLE>,
    ),
    i32,
>;
pub type LPNSPV2LOOKUPSERVICEEND = StdCallFnPtr<(super::super::Foundation::HANDLE,), i32>;
pub type LPNSPV2LOOKUPSERVICENEXTEX = StdCallFnPtr<
    (
        super::super::Foundation::HANDLE,
        super::super::Foundation::HANDLE,
        u32,
        ConstPtr<u32>,
        MutPtr<WSAQUERYSET2W>,
    ),
    (),
>;
pub type LPNSPV2SETSERVICEEX = StdCallFnPtr<
    (
        super::super::Foundation::HANDLE,
        ConstPtr<crate::core::GUID>,
        ConstPtr<WSAQUERYSET2W>,
        WSAESETSERVICEOP,
        u32,
        ConstPtr<::core::ffi::c_void>,
    ),
    (),
>;
pub type LPNSPV2STARTUP = StdCallFnPtr<
    (
        ConstPtr<crate::core::GUID>,
        MutPtr<ConstPtr<::core::ffi::c_void>>,
    ),
    i32,
>;
pub type LPSERVICE_CALLBACK_PROC = StdCallFnPtr<
    (
        super::super::Foundation::LPARAM,
        super::super::Foundation::HANDLE,
    ),
    (),
>;
pub type LPWPUCLOSEEVENT =
    StdCallFnPtr<(super::super::Foundation::HANDLE, MutPtr<i32>), super::super::Foundation::BOOL>;
pub type LPWPUCLOSESOCKETHANDLE = StdCallFnPtr<(SOCKET, MutPtr<i32>), i32>;
pub type LPWPUCLOSETHREAD = StdCallFnPtr<(ConstPtr<WSATHREADID>, MutPtr<i32>), i32>;
pub type LPWPUCOMPLETEOVERLAPPEDREQUEST = StdCallFnPtr<
    (
        SOCKET,
        MutPtr<super::super::System::IO::OVERLAPPED>,
        u32,
        u32,
        MutPtr<i32>,
    ),
    i32,
>;
pub type LPWPUCREATEEVENT = StdCallFnPtr<(MutPtr<i32>,), super::super::Foundation::HANDLE>;
pub type LPWPUCREATESOCKETHANDLE = StdCallFnPtr<(u32, PtrRepr, MutPtr<i32>), SOCKET>;
pub type LPWPUFDISSET = StdCallFnPtr<(SOCKET, ConstPtr<fd_set>), i32>;
pub type LPWPUGETPROVIDERPATH =
    StdCallFnPtr<(ConstPtr<crate::core::GUID>, PWSTR, MutPtr<i32>, MutPtr<i32>), i32>;
pub type LPWPUMODIFYIFSHANDLE = StdCallFnPtr<(u32, SOCKET, MutPtr<i32>), SOCKET>;
pub type LPWPUOPENCURRENTTHREAD = StdCallFnPtr<(MutPtr<WSATHREADID>, MutPtr<i32>), i32>;
pub type LPWPUPOSTMESSAGE = StdCallFnPtr<
    (
        super::super::Foundation::HWND,
        u32,
        super::super::Foundation::WPARAM,
        super::super::Foundation::LPARAM,
    ),
    super::super::Foundation::BOOL,
>;
pub type LPWPUQUERYBLOCKINGCALLBACK = StdCallFnPtr<
    (
        u32,
        MutPtr<LPBLOCKINGCALLBACK>,
        MutPtr<PtrRepr>,
        MutPtr<i32>,
    ),
    i32,
>;
pub type LPWPUQUERYSOCKETHANDLECONTEXT = StdCallFnPtr<(SOCKET, MutPtr<PtrRepr>, MutPtr<i32>), i32>;
pub type LPWPUQUEUEAPC =
    StdCallFnPtr<(ConstPtr<WSATHREADID>, LPWSAUSERAPC, PtrRepr, MutPtr<i32>), i32>;
pub type LPWPURESETEVENT =
    StdCallFnPtr<(super::super::Foundation::HANDLE, MutPtr<i32>), super::super::Foundation::BOOL>;
pub type LPWPUSETEVENT =
    StdCallFnPtr<(super::super::Foundation::HANDLE, MutPtr<i32>), super::super::Foundation::BOOL>;
pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE =
    StdCallFnPtr<(u32, u32, MutPtr<super::super::System::IO::OVERLAPPED>, u32), ()>;
pub type LPWSAUSERAPC = StdCallFnPtr<(PtrRepr,), ()>;
pub type LPWSCDEINSTALLPROVIDER = StdCallFnPtr<(ConstPtr<crate::core::GUID>, MutPtr<i32>), i32>;
pub type LPWSCENABLENSPROVIDER =
    StdCallFnPtr<(ConstPtr<crate::core::GUID>, super::super::Foundation::BOOL), i32>;
pub type LPWSCENUMPROTOCOLS = StdCallFnPtr<
    (
        ConstPtr<i32>,
        MutPtr<WSAPROTOCOL_INFOW>,
        MutPtr<u32>,
        MutPtr<i32>,
    ),
    i32,
>;
pub type LPWSCGETPROVIDERPATH =
    StdCallFnPtr<(ConstPtr<crate::core::GUID>, PWSTR, MutPtr<i32>, MutPtr<i32>), i32>;
pub type LPWSCINSTALLNAMESPACE =
    StdCallFnPtr<(PCWSTR, PCWSTR, u32, u32, ConstPtr<crate::core::GUID>), i32>;
pub type LPWSCINSTALLPROVIDER = StdCallFnPtr<
    (
        ConstPtr<crate::core::GUID>,
        PCWSTR,
        ConstPtr<WSAPROTOCOL_INFOW>,
        u32,
        MutPtr<i32>,
    ),
    i32,
>;
pub type LPWSCUNINSTALLNAMESPACE = StdCallFnPtr<(ConstPtr<crate::core::GUID>,), i32>;
pub type LPWSCUPDATEPROVIDER = StdCallFnPtr<
    (
        ConstPtr<crate::core::GUID>,
        PCWSTR,
        ConstPtr<WSAPROTOCOL_INFOW>,
        u32,
        MutPtr<i32>,
    ),
    i32,
>;
pub type LPWSCWRITENAMESPACEORDER = StdCallFnPtr<(MutPtr<crate::core::GUID>, u32), i32>;
pub type LPWSCWRITEPROVIDERORDER = StdCallFnPtr<(MutPtr<u32>, u32), i32>;
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub type LPWSPACCEPT = StdCallFnPtr<
    (
        SOCKET,
        MutPtr<SOCKADDR>,
        MutPtr<i32>,
        LPCONDITIONPROC,
        PtrRepr,
        MutPtr<i32>,
    ),
    SOCKET,
>;
pub type LPWSPADDRESSTOSTRING = StdCallFnPtr<
    (
        ConstPtr<SOCKADDR>,
        u32,
        ConstPtr<WSAPROTOCOL_INFOW>,
        PWSTR,
        MutPtr<u32>,
        MutPtr<i32>,
    ),
    i32,
>;
pub type LPWSPASYNCSELECT = StdCallFnPtr<
    (
        SOCKET,
        super::super::Foundation::HWND,
        u32,
        i32,
        MutPtr<i32>,
    ),
    i32,
>;
pub type LPWSPBIND = StdCallFnPtr<(SOCKET, ConstPtr<SOCKADDR>, i32, MutPtr<i32>), i32>;
pub type LPWSPCANCELBLOCKINGCALL = StdCallFnPtr<(MutPtr<i32>,), i32>;
pub type LPWSPCLEANUP = StdCallFnPtr<(MutPtr<i32>,), i32>;
pub type LPWSPCLOSESOCKET = StdCallFnPtr<(SOCKET, MutPtr<i32>), i32>;
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub type LPWSPCONNECT = StdCallFnPtr<
    (
        SOCKET,
        ConstPtr<SOCKADDR>,
        i32,
        ConstPtr<WSABUF>,
        MutPtr<WSABUF>,
        ConstPtr<super::super::NetworkManagement::QoS::QOS>,
        ConstPtr<super::super::NetworkManagement::QoS::QOS>,
        MutPtr<i32>,
    ),
    i32,
>;
pub type LPWSPDUPLICATESOCKET =
    StdCallFnPtr<(SOCKET, u32, MutPtr<WSAPROTOCOL_INFOW>, MutPtr<i32>), i32>;
pub type LPWSPENUMNETWORKEVENTS = StdCallFnPtr<
    (
        SOCKET,
        super::super::Foundation::HANDLE,
        MutPtr<WSANETWORKEVENTS>,
        MutPtr<i32>,
    ),
    i32,
>;
pub type LPWSPEVENTSELECT =
    StdCallFnPtr<(SOCKET, super::super::Foundation::HANDLE, i32, MutPtr<i32>), i32>;
pub type LPWSPGETOVERLAPPEDRESULT = StdCallFnPtr<
    (
        SOCKET,
        ConstPtr<super::super::System::IO::OVERLAPPED>,
        MutPtr<u32>,
        super::super::Foundation::BOOL,
        MutPtr<u32>,
        MutPtr<i32>,
    ),
    super::super::Foundation::BOOL,
>;
pub type LPWSPGETPEERNAME = StdCallFnPtr<(SOCKET, MutPtr<SOCKADDR>, MutPtr<i32>, MutPtr<i32>), i32>;
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub type LPWSPGETQOSBYNAME = StdCallFnPtr<
    (
        SOCKET,
        ConstPtr<WSABUF>,
        MutPtr<super::super::NetworkManagement::QoS::QOS>,
        MutPtr<i32>,
    ),
    super::super::Foundation::BOOL,
>;
pub type LPWSPGETSOCKNAME = StdCallFnPtr<(SOCKET, MutPtr<SOCKADDR>, MutPtr<i32>, MutPtr<i32>), i32>;
pub type LPWSPGETSOCKOPT = StdCallFnPtr<(SOCKET, i32, i32, PSTR, MutPtr<i32>, MutPtr<i32>), i32>;
pub type LPWSPIOCTL = StdCallFnPtr<
    (
        SOCKET,
        u32,
        ConstPtr<::core::ffi::c_void>,
        u32,
        MutPtr<::core::ffi::c_void>,
        u32,
        MutPtr<u32>,
        MutPtr<super::super::System::IO::OVERLAPPED>,
        LPWSAOVERLAPPED_COMPLETION_ROUTINE,
        ConstPtr<WSATHREADID>,
        MutPtr<i32>,
    ),
    i32,
>;
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub type LPWSPJOINLEAF = StdCallFnPtr<
    (
        SOCKET,
        ConstPtr<SOCKADDR>,
        i32,
        ConstPtr<WSABUF>,
        MutPtr<WSABUF>,
        ConstPtr<super::super::NetworkManagement::QoS::QOS>,
        ConstPtr<super::super::NetworkManagement::QoS::QOS>,
        u32,
        MutPtr<i32>,
    ),
    SOCKET,
>;
pub type LPWSPLISTEN = StdCallFnPtr<(SOCKET, i32, MutPtr<i32>), i32>;
pub type LPWSPRECV = StdCallFnPtr<
    (
        SOCKET,
        ConstPtr<WSABUF>,
        u32,
        MutPtr<u32>,
        MutPtr<u32>,
        MutPtr<super::super::System::IO::OVERLAPPED>,
        LPWSAOVERLAPPED_COMPLETION_ROUTINE,
        ConstPtr<WSATHREADID>,
        ConstPtr<i32>,
    ),
    i32,
>;
pub type LPWSPRECVDISCONNECT = StdCallFnPtr<(SOCKET, ConstPtr<WSABUF>, MutPtr<i32>), i32>;
pub type LPWSPRECVFROM = StdCallFnPtr<
    (
        SOCKET,
        ConstPtr<WSABUF>,
        u32,
        MutPtr<u32>,
        MutPtr<u32>,
        MutPtr<SOCKADDR>,
        MutPtr<i32>,
        MutPtr<super::super::System::IO::OVERLAPPED>,
        LPWSAOVERLAPPED_COMPLETION_ROUTINE,
        ConstPtr<WSATHREADID>,
        MutPtr<i32>,
    ),
    i32,
>;
pub type LPWSPSELECT = StdCallFnPtr<
    (
        i32,
        MutPtr<fd_set>,
        MutPtr<fd_set>,
        MutPtr<fd_set>,
        ConstPtr<timeval>,
        MutPtr<i32>,
    ),
    i32,
>;
pub type LPWSPSEND = StdCallFnPtr<
    (
        SOCKET,
        ConstPtr<WSABUF>,
        u32,
        MutPtr<u32>,
        u32,
        MutPtr<super::super::System::IO::OVERLAPPED>,
        LPWSAOVERLAPPED_COMPLETION_ROUTINE,
        ConstPtr<WSATHREADID>,
        MutPtr<i32>,
    ),
    i32,
>;
pub type LPWSPSENDDISCONNECT = StdCallFnPtr<(SOCKET, ConstPtr<WSABUF>, MutPtr<i32>), i32>;
pub type LPWSPSENDTO = StdCallFnPtr<
    (
        SOCKET,
        ConstPtr<WSABUF>,
        u32,
        MutPtr<u32>,
        u32,
        ConstPtr<SOCKADDR>,
        i32,
        MutPtr<super::super::System::IO::OVERLAPPED>,
        LPWSAOVERLAPPED_COMPLETION_ROUTINE,
        ConstPtr<WSATHREADID>,
        MutPtr<i32>,
    ),
    i32,
>;
pub type LPWSPSETSOCKOPT = StdCallFnPtr<(SOCKET, i32, i32, PCSTR, i32, MutPtr<i32>), i32>;
pub type LPWSPSHUTDOWN = StdCallFnPtr<(SOCKET, i32, MutPtr<i32>), i32>;
pub type LPWSPSOCKET = StdCallFnPtr<
    (
        i32,
        i32,
        i32,
        ConstPtr<WSAPROTOCOL_INFOW>,
        u32,
        u32,
        MutPtr<i32>,
    ),
    SOCKET,
>;
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS', 'Windows.Win32.System.IO'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub type LPWSPSTARTUP = StdCallFnPtr<
    (
        u16,
        ConstPtr<WSPData>,
        ConstPtr<WSAPROTOCOL_INFOW>,
        WSPUPCALLTABLE,
        MutPtr<WSPPROC_TABLE>,
    ),
    i32,
>;
pub type LPWSPSTRINGTOADDRESS = StdCallFnPtr<
    (
        PCWSTR,
        i32,
        ConstPtr<WSAPROTOCOL_INFOW>,
        MutPtr<SOCKADDR>,
        MutPtr<i32>,
        MutPtr<i32>,
    ),
    i32,
>;
pub const LSP_CRYPTO_COMPRESS: u32 = 64u32;
pub const LSP_FIREWALL: u32 = 8u32;
pub const LSP_INBOUND_MODIFY: u32 = 16u32;
pub const LSP_INSPECTOR: u32 = 1u32;
pub const LSP_LOCAL_CACHE: u32 = 128u32;
pub const LSP_OUTBOUND_MODIFY: u32 = 32u32;
pub const LSP_PROXY: u32 = 4u32;
pub const LSP_REDIRECTOR: u32 = 2u32;
pub const LSP_SYSTEM: u32 = 2147483648u32;
pub const LUP_ADDRCONFIG: u32 = 1048576u32;
pub const LUP_API_ANSI: u32 = 16777216u32;
pub const LUP_CONTAINERS: u32 = 2u32;
pub const LUP_DEEP: u32 = 1u32;
pub const LUP_DISABLE_IDN_ENCODING: u32 = 8388608u32;
pub const LUP_DNS_ONLY: u32 = 131072u32;
pub const LUP_DUAL_ADDR: u32 = 2097152u32;
pub const LUP_EXCLUSIVE_CUSTOM_SERVERS: u32 = 134217728u32;
pub const LUP_EXTENDED_QUERYSET: u32 = 33554432u32;
pub const LUP_FILESERVER: u32 = 4194304u32;
pub const LUP_FLUSHCACHE: u32 = 4096u32;
pub const LUP_FLUSHPREVIOUS: u32 = 8192u32;
pub const LUP_FORCE_CLEAR_TEXT: u32 = 1073741824u32;
pub const LUP_NEAREST: u32 = 8u32;
pub const LUP_NOCONTAINERS: u32 = 4u32;
pub const LUP_NON_AUTHORITATIVE: u32 = 16384u32;
pub const LUP_REQUIRE_SECURE: u32 = 268435456u32;
pub const LUP_RESOLUTION_HANDLE: u32 = 2147483648u32;
pub const LUP_RES_SERVICE: u32 = 32768u32;
pub const LUP_RETURN_ADDR: u32 = 256u32;
pub const LUP_RETURN_ALIASES: u32 = 1024u32;
pub const LUP_RETURN_ALL: u32 = 4080u32;
pub const LUP_RETURN_BLOB: u32 = 512u32;
pub const LUP_RETURN_COMMENT: u32 = 128u32;
pub const LUP_RETURN_NAME: u32 = 16u32;
pub const LUP_RETURN_PREFERRED_NAMES: u32 = 65536u32;
pub const LUP_RETURN_QUERY_STRING: u32 = 2048u32;
pub const LUP_RETURN_RESPONSE_FLAGS: u32 = 262144u32;
pub const LUP_RETURN_TTL: u32 = 536870912u32;
pub const LUP_RETURN_TYPE: u32 = 32u32;
pub const LUP_RETURN_VERSION: u32 = 64u32;
pub const LUP_SECURE: u32 = 32768u32;
pub const LUP_SECURE_WITH_FALLBACK: u32 = 67108864u32;
pub const LmCharSetASCII: u32 = 0u32;
pub const LmCharSetISO_8859_1: u32 = 1u32;
pub const LmCharSetISO_8859_2: u32 = 2u32;
pub const LmCharSetISO_8859_3: u32 = 3u32;
pub const LmCharSetISO_8859_4: u32 = 4u32;
pub const LmCharSetISO_8859_5: u32 = 5u32;
pub const LmCharSetISO_8859_6: u32 = 6u32;
pub const LmCharSetISO_8859_7: u32 = 7u32;
pub const LmCharSetISO_8859_8: u32 = 8u32;
pub const LmCharSetISO_8859_9: u32 = 9u32;
pub const LmCharSetUNICODE: u32 = 255u32;
pub const MAXGETHOSTSTRUCT: u32 = 1024u32;
pub const MAX_MCAST_TTL: u32 = 255u32;
pub const MAX_PROTOCOL_CHAIN: u32 = 7u32;
pub const MAX_WINDOW_INCREMENT_PERCENTAGE: u32 = 25u32;
pub const MCAST_BLOCK_SOURCE: u32 = 43u32;
pub const MCAST_JOIN_GROUP: u32 = 41u32;
pub const MCAST_JOIN_SOURCE_GROUP: u32 = 45u32;
pub const MCAST_LEAVE_GROUP: u32 = 42u32;
pub const MCAST_LEAVE_SOURCE_GROUP: u32 = 46u32;
pub const MCAST_UNBLOCK_SOURCE: u32 = 44u32;
pub const MSG_BCAST: u32 = 1024u32;
pub const MSG_CTRUNC: u32 = 512u32;
pub const MSG_ERRQUEUE: u32 = 4096u32;
pub const MSG_INTERRUPT: u32 = 16u32;
pub const MSG_MAXIOVLEN: u32 = 16u32;
pub const MSG_MCAST: u32 = 2048u32;
pub const MSG_PARTIAL: u32 = 32768u32;
pub const MSG_PEEK: u32 = 2u32;
pub const MSG_PUSH_IMMEDIATE: u32 = 32u32;
pub const MSG_TRUNC: u32 = 256u32;
pub const MSG_WAITALL: u32 = 8u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MULTICAST_MODE_TYPE(pub i32);
pub const MCAST_INCLUDE: MULTICAST_MODE_TYPE = MULTICAST_MODE_TYPE(0i32);
pub const MCAST_EXCLUDE: MULTICAST_MODE_TYPE = MULTICAST_MODE_TYPE(1i32);
impl ::core::marker::Copy for MULTICAST_MODE_TYPE {}
impl ::core::clone::Clone for MULTICAST_MODE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MULTICAST_MODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MULTICAST_MODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MULTICAST_MODE_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for MULTICAST_MODE_TYPE {
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
pub struct NAPI_DOMAIN_DESCRIPTION_BLOB {
    pub AuthLevel: u32,
    pub cchDomainName: u32,
    pub OffsetNextDomainDescription: u32,
    pub OffsetThisDomainName: u32,
}
impl ::core::marker::Copy for NAPI_DOMAIN_DESCRIPTION_BLOB {}
impl ::core::clone::Clone for NAPI_DOMAIN_DESCRIPTION_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NAPI_DOMAIN_DESCRIPTION_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAPI_DOMAIN_DESCRIPTION_BLOB")
            .field("AuthLevel", &self.AuthLevel)
            .field("cchDomainName", &self.cchDomainName)
            .field(
                "OffsetNextDomainDescription",
                &self.OffsetNextDomainDescription,
            )
            .field("OffsetThisDomainName", &self.OffsetThisDomainName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NAPI_DOMAIN_DESCRIPTION_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.AuthLevel == other.AuthLevel
            && self.cchDomainName == other.cchDomainName
            && self.OffsetNextDomainDescription == other.OffsetNextDomainDescription
            && self.OffsetThisDomainName == other.OffsetThisDomainName
    }
}
impl ::core::cmp::Eq for NAPI_DOMAIN_DESCRIPTION_BLOB {}
impl FromIntoMemory for NAPI_DOMAIN_DESCRIPTION_BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_AuthLevel = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_cchDomainName = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_OffsetNextDomainDescription = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_OffsetThisDomainName = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            AuthLevel: f_AuthLevel,
            cchDomainName: f_cchDomainName,
            OffsetNextDomainDescription: f_OffsetNextDomainDescription,
            OffsetThisDomainName: f_OffsetThisDomainName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.AuthLevel, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.cchDomainName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.OffsetNextDomainDescription, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.OffsetThisDomainName, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct NAPI_PROVIDER_INSTALLATION_BLOB {
    pub dwVersion: u32,
    pub dwProviderType: u32,
    pub fSupportsWildCard: u32,
    pub cDomains: u32,
    pub OffsetFirstDomain: u32,
}
impl ::core::marker::Copy for NAPI_PROVIDER_INSTALLATION_BLOB {}
impl ::core::clone::Clone for NAPI_PROVIDER_INSTALLATION_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NAPI_PROVIDER_INSTALLATION_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAPI_PROVIDER_INSTALLATION_BLOB")
            .field("dwVersion", &self.dwVersion)
            .field("dwProviderType", &self.dwProviderType)
            .field("fSupportsWildCard", &self.fSupportsWildCard)
            .field("cDomains", &self.cDomains)
            .field("OffsetFirstDomain", &self.OffsetFirstDomain)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NAPI_PROVIDER_INSTALLATION_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion
            && self.dwProviderType == other.dwProviderType
            && self.fSupportsWildCard == other.fSupportsWildCard
            && self.cDomains == other.cDomains
            && self.OffsetFirstDomain == other.OffsetFirstDomain
    }
}
impl ::core::cmp::Eq for NAPI_PROVIDER_INSTALLATION_BLOB {}
impl FromIntoMemory for NAPI_PROVIDER_INSTALLATION_BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwProviderType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_fSupportsWildCard = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_cDomains = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_OffsetFirstDomain = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            dwVersion: f_dwVersion,
            dwProviderType: f_dwProviderType,
            fSupportsWildCard: f_fSupportsWildCard,
            cDomains: f_cDomains,
            OffsetFirstDomain: f_OffsetFirstDomain,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwProviderType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.fSupportsWildCard, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.cDomains, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.OffsetFirstDomain, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NAPI_PROVIDER_LEVEL(pub i32);
pub const ProviderLevel_None: NAPI_PROVIDER_LEVEL = NAPI_PROVIDER_LEVEL(0i32);
pub const ProviderLevel_Secondary: NAPI_PROVIDER_LEVEL = NAPI_PROVIDER_LEVEL(1i32);
pub const ProviderLevel_Primary: NAPI_PROVIDER_LEVEL = NAPI_PROVIDER_LEVEL(2i32);
impl ::core::marker::Copy for NAPI_PROVIDER_LEVEL {}
impl ::core::clone::Clone for NAPI_PROVIDER_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NAPI_PROVIDER_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NAPI_PROVIDER_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAPI_PROVIDER_LEVEL").field(&self.0).finish()
    }
}
impl FromIntoMemory for NAPI_PROVIDER_LEVEL {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NAPI_PROVIDER_TYPE(pub i32);
pub const ProviderType_Application: NAPI_PROVIDER_TYPE = NAPI_PROVIDER_TYPE(1i32);
pub const ProviderType_Service: NAPI_PROVIDER_TYPE = NAPI_PROVIDER_TYPE(2i32);
impl ::core::marker::Copy for NAPI_PROVIDER_TYPE {}
impl ::core::clone::Clone for NAPI_PROVIDER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NAPI_PROVIDER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NAPI_PROVIDER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAPI_PROVIDER_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for NAPI_PROVIDER_TYPE {
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
pub const NETBIOS_GROUP_NAME: u32 = 1u32;
pub const NETBIOS_NAME_LENGTH: u32 = 16u32;
pub const NETBIOS_TYPE_QUICK_GROUP: u32 = 3u32;
pub const NETBIOS_TYPE_QUICK_UNIQUE: u32 = 2u32;
pub const NETBIOS_UNIQUE_NAME: u32 = 0u32;
pub struct NETRESOURCE2A {
    pub dwScope: u32,
    pub dwType: u32,
    pub dwUsage: u32,
    pub dwDisplayType: u32,
    pub lpLocalName: PSTR,
    pub lpRemoteName: PSTR,
    pub lpComment: PSTR,
    pub ns_info: NS_INFOA,
    pub ServiceType: crate::core::GUID,
    pub dwProtocols: u32,
    pub lpiProtocols: MutPtr<i32>,
}
impl ::core::marker::Copy for NETRESOURCE2A {}
impl ::core::clone::Clone for NETRESOURCE2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETRESOURCE2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETRESOURCE2A")
            .field("dwScope", &self.dwScope)
            .field("dwType", &self.dwType)
            .field("dwUsage", &self.dwUsage)
            .field("dwDisplayType", &self.dwDisplayType)
            .field("lpLocalName", &self.lpLocalName)
            .field("lpRemoteName", &self.lpRemoteName)
            .field("lpComment", &self.lpComment)
            .field("ns_info", &self.ns_info)
            .field("ServiceType", &self.ServiceType)
            .field("dwProtocols", &self.dwProtocols)
            .field("lpiProtocols", &self.lpiProtocols)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NETRESOURCE2A {
    fn eq(&self, other: &Self) -> bool {
        self.dwScope == other.dwScope
            && self.dwType == other.dwType
            && self.dwUsage == other.dwUsage
            && self.dwDisplayType == other.dwDisplayType
            && self.lpLocalName == other.lpLocalName
            && self.lpRemoteName == other.lpRemoteName
            && self.lpComment == other.lpComment
            && self.ns_info == other.ns_info
            && self.ServiceType == other.ServiceType
            && self.dwProtocols == other.dwProtocols
            && self.lpiProtocols == other.lpiProtocols
    }
}
impl ::core::cmp::Eq for NETRESOURCE2A {}
impl FromIntoMemory for NETRESOURCE2A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64);
        let f_dwScope = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwUsage = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwDisplayType = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpLocalName = <PSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpRemoteName = <PSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_lpComment = <PSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ns_info = <NS_INFOA as FromIntoMemory>::from_bytes(&from[28..28 + 12]);
        let f_ServiceType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[40..40 + 16]);
        let f_dwProtocols = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_lpiProtocols = <MutPtr<i32> as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        Self {
            dwScope: f_dwScope,
            dwType: f_dwType,
            dwUsage: f_dwUsage,
            dwDisplayType: f_dwDisplayType,
            lpLocalName: f_lpLocalName,
            lpRemoteName: f_lpRemoteName,
            lpComment: f_lpComment,
            ns_info: f_ns_info,
            ServiceType: f_ServiceType,
            dwProtocols: f_dwProtocols,
            lpiProtocols: f_lpiProtocols,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64);
        FromIntoMemory::into_bytes(self.dwScope, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwUsage, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwDisplayType, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpLocalName, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpRemoteName, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lpComment, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ns_info, &mut into[28..28 + 12]);
        FromIntoMemory::into_bytes(self.ServiceType, &mut into[40..40 + 16]);
        FromIntoMemory::into_bytes(self.dwProtocols, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.lpiProtocols, &mut into[60..60 + 4]);
    }
    fn size() -> usize {
        64
    }
}
pub struct NETRESOURCE2W {
    pub dwScope: u32,
    pub dwType: u32,
    pub dwUsage: u32,
    pub dwDisplayType: u32,
    pub lpLocalName: PWSTR,
    pub lpRemoteName: PWSTR,
    pub lpComment: PWSTR,
    pub ns_info: NS_INFOA,
    pub ServiceType: crate::core::GUID,
    pub dwProtocols: u32,
    pub lpiProtocols: MutPtr<i32>,
}
impl ::core::marker::Copy for NETRESOURCE2W {}
impl ::core::clone::Clone for NETRESOURCE2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETRESOURCE2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETRESOURCE2W")
            .field("dwScope", &self.dwScope)
            .field("dwType", &self.dwType)
            .field("dwUsage", &self.dwUsage)
            .field("dwDisplayType", &self.dwDisplayType)
            .field("lpLocalName", &self.lpLocalName)
            .field("lpRemoteName", &self.lpRemoteName)
            .field("lpComment", &self.lpComment)
            .field("ns_info", &self.ns_info)
            .field("ServiceType", &self.ServiceType)
            .field("dwProtocols", &self.dwProtocols)
            .field("lpiProtocols", &self.lpiProtocols)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NETRESOURCE2W {
    fn eq(&self, other: &Self) -> bool {
        self.dwScope == other.dwScope
            && self.dwType == other.dwType
            && self.dwUsage == other.dwUsage
            && self.dwDisplayType == other.dwDisplayType
            && self.lpLocalName == other.lpLocalName
            && self.lpRemoteName == other.lpRemoteName
            && self.lpComment == other.lpComment
            && self.ns_info == other.ns_info
            && self.ServiceType == other.ServiceType
            && self.dwProtocols == other.dwProtocols
            && self.lpiProtocols == other.lpiProtocols
    }
}
impl ::core::cmp::Eq for NETRESOURCE2W {}
impl FromIntoMemory for NETRESOURCE2W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64);
        let f_dwScope = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwUsage = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwDisplayType = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpLocalName = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpRemoteName = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_lpComment = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ns_info = <NS_INFOA as FromIntoMemory>::from_bytes(&from[28..28 + 12]);
        let f_ServiceType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[40..40 + 16]);
        let f_dwProtocols = <u32 as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_lpiProtocols = <MutPtr<i32> as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        Self {
            dwScope: f_dwScope,
            dwType: f_dwType,
            dwUsage: f_dwUsage,
            dwDisplayType: f_dwDisplayType,
            lpLocalName: f_lpLocalName,
            lpRemoteName: f_lpRemoteName,
            lpComment: f_lpComment,
            ns_info: f_ns_info,
            ServiceType: f_ServiceType,
            dwProtocols: f_dwProtocols,
            lpiProtocols: f_lpiProtocols,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64);
        FromIntoMemory::into_bytes(self.dwScope, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwUsage, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwDisplayType, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpLocalName, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpRemoteName, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lpComment, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ns_info, &mut into[28..28 + 12]);
        FromIntoMemory::into_bytes(self.ServiceType, &mut into[40..40 + 16]);
        FromIntoMemory::into_bytes(self.dwProtocols, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.lpiProtocols, &mut into[60..60 + 4]);
    }
    fn size() -> usize {
        64
    }
}
pub const NI_DGRAM: u32 = 16u32;
pub const NI_MAXHOST: u32 = 1025u32;
pub const NI_MAXSERV: u32 = 32u32;
pub const NI_NAMEREQD: u32 = 4u32;
pub const NI_NOFQDN: u32 = 1u32;
pub const NI_NUMERICHOST: u32 = 2u32;
pub const NI_NUMERICSERV: u32 = 8u32;
pub const NLA_ALLUSERS_NETWORK: u32 = 1u32;
pub struct NLA_BLOB {
    pub header: NLA_BLOB_1,
    pub data: NLA_BLOB_0,
}
impl ::core::marker::Copy for NLA_BLOB {}
impl ::core::clone::Clone for NLA_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NLA_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLA_BLOB")
            .field("header", &self.header)
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NLA_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.data == other.data
    }
}
impl ::core::cmp::Eq for NLA_BLOB {}
impl FromIntoMemory for NLA_BLOB {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 536);
        let f_header = <NLA_BLOB_1 as FromIntoMemory>::from_bytes(&from[0..0 + 12]);
        let f_data = <NLA_BLOB_0 as FromIntoMemory>::from_bytes(&from[12..12 + 524]);
        Self {
            header: f_header,
            data: f_data,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 536);
        FromIntoMemory::into_bytes(self.header, &mut into[0..0 + 12]);
        FromIntoMemory::into_bytes(self.data, &mut into[12..12 + 524]);
    }
    fn size() -> usize {
        536
    }
}
pub struct NLA_BLOB_0 {
    data: [u8; 524],
}
impl ::core::default::Default for NLA_BLOB_0 {
    fn default() -> Self {
        Self { data: [0u8; 524] }
    }
}
impl ::core::marker::Copy for NLA_BLOB_0 {}
impl ::core::clone::Clone for NLA_BLOB_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NLA_BLOB_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLA_BLOB_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NLA_BLOB_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for NLA_BLOB_0 {}
impl FromIntoMemory for NLA_BLOB_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 524];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        524
    }
}
pub struct NLA_BLOB_0_0 {
    pub remote: NLA_BLOB_0_0_0,
}
impl ::core::marker::Copy for NLA_BLOB_0_0 {}
impl ::core::clone::Clone for NLA_BLOB_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NLA_BLOB_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLA_BLOB_0_0")
            .field("remote", &self.remote)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NLA_BLOB_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.remote == other.remote
    }
}
impl ::core::cmp::Eq for NLA_BLOB_0_0 {}
impl FromIntoMemory for NLA_BLOB_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 524);
        let f_remote = <NLA_BLOB_0_0_0 as FromIntoMemory>::from_bytes(&from[0..0 + 524]);
        Self { remote: f_remote }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 524);
        FromIntoMemory::into_bytes(self.remote, &mut into[0..0 + 524]);
    }
    fn size() -> usize {
        524
    }
}
pub struct NLA_BLOB_0_0_0 {
    pub speed: u32,
    pub r#type: u32,
    pub state: u32,
    pub machineName: [u16; 256],
    pub sharedAdapterName: [u16; 256],
}
impl ::core::marker::Copy for NLA_BLOB_0_0_0 {}
impl ::core::clone::Clone for NLA_BLOB_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NLA_BLOB_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLA_BLOB_0_0_0")
            .field("speed", &self.speed)
            .field("type", &self.r#type)
            .field("state", &self.state)
            .field("machineName", &self.machineName)
            .field("sharedAdapterName", &self.sharedAdapterName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NLA_BLOB_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.speed == other.speed
            && self.r#type == other.r#type
            && self.state == other.state
            && self.machineName == other.machineName
            && self.sharedAdapterName == other.sharedAdapterName
    }
}
impl ::core::cmp::Eq for NLA_BLOB_0_0_0 {}
impl FromIntoMemory for NLA_BLOB_0_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 524);
        let f_speed = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_type = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_state = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_machineName = <[u16; 256] as FromIntoMemory>::from_bytes(&from[12..12 + 256]);
        let f_sharedAdapterName = <[u16; 256] as FromIntoMemory>::from_bytes(&from[268..268 + 256]);
        Self {
            speed: f_speed,
            r#type: f_type,
            state: f_state,
            machineName: f_machineName,
            sharedAdapterName: f_sharedAdapterName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 524);
        FromIntoMemory::into_bytes(self.speed, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.r#type, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.state, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.machineName, &mut into[12..12 + 256]);
        FromIntoMemory::into_bytes(self.sharedAdapterName, &mut into[268..268 + 256]);
    }
    fn size() -> usize {
        524
    }
}
pub struct NLA_BLOB_0_1 {
    pub r#type: NLA_CONNECTIVITY_TYPE,
    pub internet: NLA_INTERNET,
}
impl ::core::marker::Copy for NLA_BLOB_0_1 {}
impl ::core::clone::Clone for NLA_BLOB_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NLA_BLOB_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLA_BLOB_0_1")
            .field("type", &self.r#type)
            .field("internet", &self.internet)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NLA_BLOB_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.internet == other.internet
    }
}
impl ::core::cmp::Eq for NLA_BLOB_0_1 {}
impl FromIntoMemory for NLA_BLOB_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_type = <NLA_CONNECTIVITY_TYPE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_internet = <NLA_INTERNET as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            r#type: f_type,
            internet: f_internet,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.r#type, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.internet, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct NLA_BLOB_0_2 {
    pub dwType: u32,
    pub dwSpeed: u32,
    pub adapterName: [super::super::Foundation::CHAR; 1],
}
impl ::core::marker::Copy for NLA_BLOB_0_2 {}
impl ::core::clone::Clone for NLA_BLOB_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NLA_BLOB_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLA_BLOB_0_2")
            .field("dwType", &self.dwType)
            .field("dwSpeed", &self.dwSpeed)
            .field("adapterName", &self.adapterName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NLA_BLOB_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType
            && self.dwSpeed == other.dwSpeed
            && self.adapterName == other.adapterName
    }
}
impl ::core::cmp::Eq for NLA_BLOB_0_2 {}
impl FromIntoMemory for NLA_BLOB_0_2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwSpeed = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_adapterName =
            <[super::super::Foundation::CHAR; 1] as FromIntoMemory>::from_bytes(&from[8..8 + 1]);
        Self {
            dwType: f_dwType,
            dwSpeed: f_dwSpeed,
            adapterName: f_adapterName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwSpeed, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.adapterName, &mut into[8..8 + 1]);
    }
    fn size() -> usize {
        12
    }
}
pub struct NLA_BLOB_0_3 {
    pub information: [super::super::Foundation::CHAR; 1],
}
impl ::core::marker::Copy for NLA_BLOB_0_3 {}
impl ::core::clone::Clone for NLA_BLOB_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NLA_BLOB_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLA_BLOB_0_3")
            .field("information", &self.information)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NLA_BLOB_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.information == other.information
    }
}
impl ::core::cmp::Eq for NLA_BLOB_0_3 {}
impl FromIntoMemory for NLA_BLOB_0_3 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 1);
        let f_information =
            <[super::super::Foundation::CHAR; 1] as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        Self {
            information: f_information,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 1);
        FromIntoMemory::into_bytes(self.information, &mut into[0..0 + 1]);
    }
    fn size() -> usize {
        1
    }
}
pub struct NLA_BLOB_1 {
    pub r#type: NLA_BLOB_DATA_TYPE,
    pub dwSize: u32,
    pub nextOffset: u32,
}
impl ::core::marker::Copy for NLA_BLOB_1 {}
impl ::core::clone::Clone for NLA_BLOB_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NLA_BLOB_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NLA_BLOB_1")
            .field("type", &self.r#type)
            .field("dwSize", &self.dwSize)
            .field("nextOffset", &self.nextOffset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NLA_BLOB_1 {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type
            && self.dwSize == other.dwSize
            && self.nextOffset == other.nextOffset
    }
}
impl ::core::cmp::Eq for NLA_BLOB_1 {}
impl FromIntoMemory for NLA_BLOB_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_type = <NLA_BLOB_DATA_TYPE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_nextOffset = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            r#type: f_type,
            dwSize: f_dwSize,
            nextOffset: f_nextOffset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.r#type, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.nextOffset, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NLA_BLOB_DATA_TYPE(pub i32);
pub const NLA_RAW_DATA: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(0i32);
pub const NLA_INTERFACE: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(1i32);
pub const NLA_802_1X_LOCATION: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(2i32);
pub const NLA_CONNECTIVITY: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(3i32);
pub const NLA_ICS: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(4i32);
impl ::core::marker::Copy for NLA_BLOB_DATA_TYPE {}
impl ::core::clone::Clone for NLA_BLOB_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NLA_BLOB_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NLA_BLOB_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLA_BLOB_DATA_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for NLA_BLOB_DATA_TYPE {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NLA_CONNECTIVITY_TYPE(pub i32);
pub const NLA_NETWORK_AD_HOC: NLA_CONNECTIVITY_TYPE = NLA_CONNECTIVITY_TYPE(0i32);
pub const NLA_NETWORK_MANAGED: NLA_CONNECTIVITY_TYPE = NLA_CONNECTIVITY_TYPE(1i32);
pub const NLA_NETWORK_UNMANAGED: NLA_CONNECTIVITY_TYPE = NLA_CONNECTIVITY_TYPE(2i32);
pub const NLA_NETWORK_UNKNOWN: NLA_CONNECTIVITY_TYPE = NLA_CONNECTIVITY_TYPE(3i32);
impl ::core::marker::Copy for NLA_CONNECTIVITY_TYPE {}
impl ::core::clone::Clone for NLA_CONNECTIVITY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NLA_CONNECTIVITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NLA_CONNECTIVITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLA_CONNECTIVITY_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NLA_CONNECTIVITY_TYPE {
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
pub const NLA_FRIENDLY_NAME: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NLA_INTERNET(pub i32);
pub const NLA_INTERNET_UNKNOWN: NLA_INTERNET = NLA_INTERNET(0i32);
pub const NLA_INTERNET_NO: NLA_INTERNET = NLA_INTERNET(1i32);
pub const NLA_INTERNET_YES: NLA_INTERNET = NLA_INTERNET(2i32);
impl ::core::marker::Copy for NLA_INTERNET {}
impl ::core::clone::Clone for NLA_INTERNET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NLA_INTERNET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NLA_INTERNET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NLA_INTERNET").field(&self.0).finish()
    }
}
impl FromIntoMemory for NLA_INTERNET {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NL_ADDRESS_TYPE(pub i32);
pub const NlatUnspecified: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(0i32);
pub const NlatUnicast: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(1i32);
pub const NlatAnycast: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(2i32);
pub const NlatMulticast: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(3i32);
pub const NlatBroadcast: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(4i32);
pub const NlatInvalid: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(5i32);
impl ::core::marker::Copy for NL_ADDRESS_TYPE {}
impl ::core::clone::Clone for NL_ADDRESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NL_ADDRESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_ADDRESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_ADDRESS_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for NL_ADDRESS_TYPE {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NL_BANDWIDTH_FLAG(pub i32);
pub const NlbwDisabled: NL_BANDWIDTH_FLAG = NL_BANDWIDTH_FLAG(0i32);
pub const NlbwEnabled: NL_BANDWIDTH_FLAG = NL_BANDWIDTH_FLAG(1i32);
pub const NlbwUnchanged: NL_BANDWIDTH_FLAG = NL_BANDWIDTH_FLAG(-1i32);
impl ::core::marker::Copy for NL_BANDWIDTH_FLAG {}
impl ::core::clone::Clone for NL_BANDWIDTH_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NL_BANDWIDTH_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_BANDWIDTH_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_BANDWIDTH_FLAG").field(&self.0).finish()
    }
}
impl FromIntoMemory for NL_BANDWIDTH_FLAG {
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
pub struct NL_BANDWIDTH_INFORMATION {
    pub Bandwidth: u64,
    pub Instability: u64,
    pub BandwidthPeaked: super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for NL_BANDWIDTH_INFORMATION {}
impl ::core::clone::Clone for NL_BANDWIDTH_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NL_BANDWIDTH_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NL_BANDWIDTH_INFORMATION")
            .field("Bandwidth", &self.Bandwidth)
            .field("Instability", &self.Instability)
            .field("BandwidthPeaked", &self.BandwidthPeaked)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NL_BANDWIDTH_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Bandwidth == other.Bandwidth
            && self.Instability == other.Instability
            && self.BandwidthPeaked == other.BandwidthPeaked
    }
}
impl ::core::cmp::Eq for NL_BANDWIDTH_INFORMATION {}
impl FromIntoMemory for NL_BANDWIDTH_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_Bandwidth = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_Instability = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_BandwidthPeaked =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[16..16 + 1]);
        Self {
            Bandwidth: f_Bandwidth,
            Instability: f_Instability,
            BandwidthPeaked: f_BandwidthPeaked,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.Bandwidth, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.Instability, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.BandwidthPeaked, &mut into[16..16 + 1]);
    }
    fn size() -> usize {
        24
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NL_DAD_STATE(pub i32);
pub const NldsInvalid: NL_DAD_STATE = NL_DAD_STATE(0i32);
pub const NldsTentative: NL_DAD_STATE = NL_DAD_STATE(1i32);
pub const NldsDuplicate: NL_DAD_STATE = NL_DAD_STATE(2i32);
pub const NldsDeprecated: NL_DAD_STATE = NL_DAD_STATE(3i32);
pub const NldsPreferred: NL_DAD_STATE = NL_DAD_STATE(4i32);
pub const IpDadStateInvalid: NL_DAD_STATE = NL_DAD_STATE(0i32);
pub const IpDadStateTentative: NL_DAD_STATE = NL_DAD_STATE(1i32);
pub const IpDadStateDuplicate: NL_DAD_STATE = NL_DAD_STATE(2i32);
pub const IpDadStateDeprecated: NL_DAD_STATE = NL_DAD_STATE(3i32);
pub const IpDadStatePreferred: NL_DAD_STATE = NL_DAD_STATE(4i32);
impl ::core::marker::Copy for NL_DAD_STATE {}
impl ::core::clone::Clone for NL_DAD_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NL_DAD_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_DAD_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_DAD_STATE").field(&self.0).finish()
    }
}
impl FromIntoMemory for NL_DAD_STATE {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NL_INTERFACE_NETWORK_CATEGORY_STATE(pub i32);
pub const NlincCategoryUnknown: NL_INTERFACE_NETWORK_CATEGORY_STATE =
    NL_INTERFACE_NETWORK_CATEGORY_STATE(0i32);
pub const NlincPublic: NL_INTERFACE_NETWORK_CATEGORY_STATE =
    NL_INTERFACE_NETWORK_CATEGORY_STATE(1i32);
pub const NlincPrivate: NL_INTERFACE_NETWORK_CATEGORY_STATE =
    NL_INTERFACE_NETWORK_CATEGORY_STATE(2i32);
pub const NlincDomainAuthenticated: NL_INTERFACE_NETWORK_CATEGORY_STATE =
    NL_INTERFACE_NETWORK_CATEGORY_STATE(3i32);
pub const NlincCategoryStateMax: NL_INTERFACE_NETWORK_CATEGORY_STATE =
    NL_INTERFACE_NETWORK_CATEGORY_STATE(4i32);
impl ::core::marker::Copy for NL_INTERFACE_NETWORK_CATEGORY_STATE {}
impl ::core::clone::Clone for NL_INTERFACE_NETWORK_CATEGORY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NL_INTERFACE_NETWORK_CATEGORY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_INTERFACE_NETWORK_CATEGORY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_INTERFACE_NETWORK_CATEGORY_STATE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NL_INTERFACE_NETWORK_CATEGORY_STATE {
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
pub struct NL_INTERFACE_OFFLOAD_ROD {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NL_INTERFACE_OFFLOAD_ROD {}
impl ::core::clone::Clone for NL_INTERFACE_OFFLOAD_ROD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NL_INTERFACE_OFFLOAD_ROD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NL_INTERFACE_OFFLOAD_ROD")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NL_INTERFACE_OFFLOAD_ROD {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NL_INTERFACE_OFFLOAD_ROD {}
impl FromIntoMemory for NL_INTERFACE_OFFLOAD_ROD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 1);
        let f__bitfield = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        Self {
            _bitfield: f__bitfield,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 1);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 1]);
    }
    fn size() -> usize {
        1
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NL_LINK_LOCAL_ADDRESS_BEHAVIOR(pub i32);
pub const LinkLocalAlwaysOff: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = NL_LINK_LOCAL_ADDRESS_BEHAVIOR(0i32);
pub const LinkLocalDelayed: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = NL_LINK_LOCAL_ADDRESS_BEHAVIOR(1i32);
pub const LinkLocalAlwaysOn: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = NL_LINK_LOCAL_ADDRESS_BEHAVIOR(2i32);
pub const LinkLocalUnchanged: NL_LINK_LOCAL_ADDRESS_BEHAVIOR =
    NL_LINK_LOCAL_ADDRESS_BEHAVIOR(-1i32);
impl ::core::marker::Copy for NL_LINK_LOCAL_ADDRESS_BEHAVIOR {}
impl ::core::clone::Clone for NL_LINK_LOCAL_ADDRESS_BEHAVIOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NL_LINK_LOCAL_ADDRESS_BEHAVIOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_LINK_LOCAL_ADDRESS_BEHAVIOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_LINK_LOCAL_ADDRESS_BEHAVIOR")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NL_LINK_LOCAL_ADDRESS_BEHAVIOR {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NL_NEIGHBOR_STATE(pub i32);
pub const NlnsUnreachable: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(0i32);
pub const NlnsIncomplete: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(1i32);
pub const NlnsProbe: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(2i32);
pub const NlnsDelay: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(3i32);
pub const NlnsStale: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(4i32);
pub const NlnsReachable: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(5i32);
pub const NlnsPermanent: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(6i32);
pub const NlnsMaximum: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(7i32);
impl ::core::marker::Copy for NL_NEIGHBOR_STATE {}
impl ::core::clone::Clone for NL_NEIGHBOR_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NL_NEIGHBOR_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_NEIGHBOR_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_NEIGHBOR_STATE").field(&self.0).finish()
    }
}
impl FromIntoMemory for NL_NEIGHBOR_STATE {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NL_NETWORK_CATEGORY(pub i32);
pub const NetworkCategoryPublic: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(0i32);
pub const NetworkCategoryPrivate: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(1i32);
pub const NetworkCategoryDomainAuthenticated: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(2i32);
pub const NetworkCategoryUnchanged: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(-1i32);
pub const NetworkCategoryUnknown: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(-1i32);
impl ::core::marker::Copy for NL_NETWORK_CATEGORY {}
impl ::core::clone::Clone for NL_NETWORK_CATEGORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NL_NETWORK_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_NETWORK_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_NETWORK_CATEGORY").field(&self.0).finish()
    }
}
impl FromIntoMemory for NL_NETWORK_CATEGORY {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NL_NETWORK_CONNECTIVITY_COST_HINT(pub i32);
pub const NetworkConnectivityCostHintUnknown: NL_NETWORK_CONNECTIVITY_COST_HINT =
    NL_NETWORK_CONNECTIVITY_COST_HINT(0i32);
pub const NetworkConnectivityCostHintUnrestricted: NL_NETWORK_CONNECTIVITY_COST_HINT =
    NL_NETWORK_CONNECTIVITY_COST_HINT(1i32);
pub const NetworkConnectivityCostHintFixed: NL_NETWORK_CONNECTIVITY_COST_HINT =
    NL_NETWORK_CONNECTIVITY_COST_HINT(2i32);
pub const NetworkConnectivityCostHintVariable: NL_NETWORK_CONNECTIVITY_COST_HINT =
    NL_NETWORK_CONNECTIVITY_COST_HINT(3i32);
impl ::core::marker::Copy for NL_NETWORK_CONNECTIVITY_COST_HINT {}
impl ::core::clone::Clone for NL_NETWORK_CONNECTIVITY_COST_HINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NL_NETWORK_CONNECTIVITY_COST_HINT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_NETWORK_CONNECTIVITY_COST_HINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_NETWORK_CONNECTIVITY_COST_HINT")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NL_NETWORK_CONNECTIVITY_COST_HINT {
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
pub struct NL_NETWORK_CONNECTIVITY_HINT {
    pub ConnectivityLevel: NL_NETWORK_CONNECTIVITY_LEVEL_HINT,
    pub ConnectivityCost: NL_NETWORK_CONNECTIVITY_COST_HINT,
    pub ApproachingDataLimit: super::super::Foundation::BOOLEAN,
    pub OverDataLimit: super::super::Foundation::BOOLEAN,
    pub Roaming: super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for NL_NETWORK_CONNECTIVITY_HINT {}
impl ::core::clone::Clone for NL_NETWORK_CONNECTIVITY_HINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NL_NETWORK_CONNECTIVITY_HINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NL_NETWORK_CONNECTIVITY_HINT")
            .field("ConnectivityLevel", &self.ConnectivityLevel)
            .field("ConnectivityCost", &self.ConnectivityCost)
            .field("ApproachingDataLimit", &self.ApproachingDataLimit)
            .field("OverDataLimit", &self.OverDataLimit)
            .field("Roaming", &self.Roaming)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NL_NETWORK_CONNECTIVITY_HINT {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectivityLevel == other.ConnectivityLevel
            && self.ConnectivityCost == other.ConnectivityCost
            && self.ApproachingDataLimit == other.ApproachingDataLimit
            && self.OverDataLimit == other.OverDataLimit
            && self.Roaming == other.Roaming
    }
}
impl ::core::cmp::Eq for NL_NETWORK_CONNECTIVITY_HINT {}
impl FromIntoMemory for NL_NETWORK_CONNECTIVITY_HINT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_ConnectivityLevel =
            <NL_NETWORK_CONNECTIVITY_LEVEL_HINT as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ConnectivityCost =
            <NL_NETWORK_CONNECTIVITY_COST_HINT as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ApproachingDataLimit =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[8..8 + 1]);
        let f_OverDataLimit =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[9..9 + 1]);
        let f_Roaming =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[10..10 + 1]);
        Self {
            ConnectivityLevel: f_ConnectivityLevel,
            ConnectivityCost: f_ConnectivityCost,
            ApproachingDataLimit: f_ApproachingDataLimit,
            OverDataLimit: f_OverDataLimit,
            Roaming: f_Roaming,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.ConnectivityLevel, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ConnectivityCost, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ApproachingDataLimit, &mut into[8..8 + 1]);
        FromIntoMemory::into_bytes(self.OverDataLimit, &mut into[9..9 + 1]);
        FromIntoMemory::into_bytes(self.Roaming, &mut into[10..10 + 1]);
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NL_NETWORK_CONNECTIVITY_LEVEL_HINT(pub i32);
pub const NetworkConnectivityLevelHintUnknown: NL_NETWORK_CONNECTIVITY_LEVEL_HINT =
    NL_NETWORK_CONNECTIVITY_LEVEL_HINT(0i32);
pub const NetworkConnectivityLevelHintNone: NL_NETWORK_CONNECTIVITY_LEVEL_HINT =
    NL_NETWORK_CONNECTIVITY_LEVEL_HINT(1i32);
pub const NetworkConnectivityLevelHintLocalAccess: NL_NETWORK_CONNECTIVITY_LEVEL_HINT =
    NL_NETWORK_CONNECTIVITY_LEVEL_HINT(2i32);
pub const NetworkConnectivityLevelHintInternetAccess: NL_NETWORK_CONNECTIVITY_LEVEL_HINT =
    NL_NETWORK_CONNECTIVITY_LEVEL_HINT(3i32);
pub const NetworkConnectivityLevelHintConstrainedInternetAccess:
    NL_NETWORK_CONNECTIVITY_LEVEL_HINT = NL_NETWORK_CONNECTIVITY_LEVEL_HINT(4i32);
pub const NetworkConnectivityLevelHintHidden: NL_NETWORK_CONNECTIVITY_LEVEL_HINT =
    NL_NETWORK_CONNECTIVITY_LEVEL_HINT(5i32);
impl ::core::marker::Copy for NL_NETWORK_CONNECTIVITY_LEVEL_HINT {}
impl ::core::clone::Clone for NL_NETWORK_CONNECTIVITY_LEVEL_HINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NL_NETWORK_CONNECTIVITY_LEVEL_HINT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_NETWORK_CONNECTIVITY_LEVEL_HINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_NETWORK_CONNECTIVITY_LEVEL_HINT")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NL_NETWORK_CONNECTIVITY_LEVEL_HINT {
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
pub struct NL_PATH_BANDWIDTH_ROD {
    pub Bandwidth: u64,
    pub Instability: u64,
    pub BandwidthPeaked: super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for NL_PATH_BANDWIDTH_ROD {}
impl ::core::clone::Clone for NL_PATH_BANDWIDTH_ROD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NL_PATH_BANDWIDTH_ROD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NL_PATH_BANDWIDTH_ROD")
            .field("Bandwidth", &self.Bandwidth)
            .field("Instability", &self.Instability)
            .field("BandwidthPeaked", &self.BandwidthPeaked)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NL_PATH_BANDWIDTH_ROD {
    fn eq(&self, other: &Self) -> bool {
        self.Bandwidth == other.Bandwidth
            && self.Instability == other.Instability
            && self.BandwidthPeaked == other.BandwidthPeaked
    }
}
impl ::core::cmp::Eq for NL_PATH_BANDWIDTH_ROD {}
impl FromIntoMemory for NL_PATH_BANDWIDTH_ROD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_Bandwidth = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_Instability = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_BandwidthPeaked =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[16..16 + 1]);
        Self {
            Bandwidth: f_Bandwidth,
            Instability: f_Instability,
            BandwidthPeaked: f_BandwidthPeaked,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.Bandwidth, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.Instability, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.BandwidthPeaked, &mut into[16..16 + 1]);
    }
    fn size() -> usize {
        24
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NL_PREFIX_ORIGIN(pub i32);
pub const IpPrefixOriginOther: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(0i32);
pub const IpPrefixOriginManual: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(1i32);
pub const IpPrefixOriginWellKnown: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(2i32);
pub const IpPrefixOriginDhcp: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(3i32);
pub const IpPrefixOriginRouterAdvertisement: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(4i32);
pub const IpPrefixOriginUnchanged: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(16i32);
impl ::core::marker::Copy for NL_PREFIX_ORIGIN {}
impl ::core::clone::Clone for NL_PREFIX_ORIGIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NL_PREFIX_ORIGIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_PREFIX_ORIGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_PREFIX_ORIGIN").field(&self.0).finish()
    }
}
impl FromIntoMemory for NL_PREFIX_ORIGIN {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NL_ROUTER_DISCOVERY_BEHAVIOR(pub i32);
pub const RouterDiscoveryDisabled: NL_ROUTER_DISCOVERY_BEHAVIOR =
    NL_ROUTER_DISCOVERY_BEHAVIOR(0i32);
pub const RouterDiscoveryEnabled: NL_ROUTER_DISCOVERY_BEHAVIOR = NL_ROUTER_DISCOVERY_BEHAVIOR(1i32);
pub const RouterDiscoveryDhcp: NL_ROUTER_DISCOVERY_BEHAVIOR = NL_ROUTER_DISCOVERY_BEHAVIOR(2i32);
pub const RouterDiscoveryUnchanged: NL_ROUTER_DISCOVERY_BEHAVIOR =
    NL_ROUTER_DISCOVERY_BEHAVIOR(-1i32);
impl ::core::marker::Copy for NL_ROUTER_DISCOVERY_BEHAVIOR {}
impl ::core::clone::Clone for NL_ROUTER_DISCOVERY_BEHAVIOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NL_ROUTER_DISCOVERY_BEHAVIOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_ROUTER_DISCOVERY_BEHAVIOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_ROUTER_DISCOVERY_BEHAVIOR")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NL_ROUTER_DISCOVERY_BEHAVIOR {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NL_ROUTE_ORIGIN(pub i32);
pub const NlroManual: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(0i32);
pub const NlroWellKnown: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(1i32);
pub const NlroDHCP: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(2i32);
pub const NlroRouterAdvertisement: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(3i32);
pub const Nlro6to4: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(4i32);
impl ::core::marker::Copy for NL_ROUTE_ORIGIN {}
impl ::core::clone::Clone for NL_ROUTE_ORIGIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NL_ROUTE_ORIGIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_ROUTE_ORIGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_ROUTE_ORIGIN").field(&self.0).finish()
    }
}
impl FromIntoMemory for NL_ROUTE_ORIGIN {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NL_ROUTE_PROTOCOL(pub i32);
pub const RouteProtocolOther: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(1i32);
pub const RouteProtocolLocal: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(2i32);
pub const RouteProtocolNetMgmt: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(3i32);
pub const RouteProtocolIcmp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(4i32);
pub const RouteProtocolEgp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(5i32);
pub const RouteProtocolGgp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(6i32);
pub const RouteProtocolHello: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(7i32);
pub const RouteProtocolRip: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(8i32);
pub const RouteProtocolIsIs: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(9i32);
pub const RouteProtocolEsIs: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10i32);
pub const RouteProtocolCisco: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(11i32);
pub const RouteProtocolBbn: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(12i32);
pub const RouteProtocolOspf: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(13i32);
pub const RouteProtocolBgp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(14i32);
pub const RouteProtocolIdpr: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(15i32);
pub const RouteProtocolEigrp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(16i32);
pub const RouteProtocolDvmrp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(17i32);
pub const RouteProtocolRpl: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(18i32);
pub const RouteProtocolDhcp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(19i32);
pub const MIB_IPPROTO_OTHER: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(1i32);
pub const PROTO_IP_OTHER: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(1i32);
pub const MIB_IPPROTO_LOCAL: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(2i32);
pub const PROTO_IP_LOCAL: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(2i32);
pub const MIB_IPPROTO_NETMGMT: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(3i32);
pub const PROTO_IP_NETMGMT: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(3i32);
pub const MIB_IPPROTO_ICMP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(4i32);
pub const PROTO_IP_ICMP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(4i32);
pub const MIB_IPPROTO_EGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(5i32);
pub const PROTO_IP_EGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(5i32);
pub const MIB_IPPROTO_GGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(6i32);
pub const PROTO_IP_GGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(6i32);
pub const MIB_IPPROTO_HELLO: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(7i32);
pub const PROTO_IP_HELLO: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(7i32);
pub const MIB_IPPROTO_RIP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(8i32);
pub const PROTO_IP_RIP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(8i32);
pub const MIB_IPPROTO_IS_IS: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(9i32);
pub const PROTO_IP_IS_IS: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(9i32);
pub const MIB_IPPROTO_ES_IS: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10i32);
pub const PROTO_IP_ES_IS: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10i32);
pub const MIB_IPPROTO_CISCO: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(11i32);
pub const PROTO_IP_CISCO: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(11i32);
pub const MIB_IPPROTO_BBN: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(12i32);
pub const PROTO_IP_BBN: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(12i32);
pub const MIB_IPPROTO_OSPF: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(13i32);
pub const PROTO_IP_OSPF: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(13i32);
pub const MIB_IPPROTO_BGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(14i32);
pub const PROTO_IP_BGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(14i32);
pub const MIB_IPPROTO_IDPR: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(15i32);
pub const PROTO_IP_IDPR: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(15i32);
pub const MIB_IPPROTO_EIGRP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(16i32);
pub const PROTO_IP_EIGRP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(16i32);
pub const MIB_IPPROTO_DVMRP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(17i32);
pub const PROTO_IP_DVMRP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(17i32);
pub const MIB_IPPROTO_RPL: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(18i32);
pub const PROTO_IP_RPL: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(18i32);
pub const MIB_IPPROTO_DHCP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(19i32);
pub const PROTO_IP_DHCP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(19i32);
pub const MIB_IPPROTO_NT_AUTOSTATIC: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10002i32);
pub const PROTO_IP_NT_AUTOSTATIC: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10002i32);
pub const MIB_IPPROTO_NT_STATIC: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10006i32);
pub const PROTO_IP_NT_STATIC: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10006i32);
pub const MIB_IPPROTO_NT_STATIC_NON_DOD: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10007i32);
pub const PROTO_IP_NT_STATIC_NON_DOD: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10007i32);
impl ::core::marker::Copy for NL_ROUTE_PROTOCOL {}
impl ::core::clone::Clone for NL_ROUTE_PROTOCOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NL_ROUTE_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_ROUTE_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_ROUTE_PROTOCOL").field(&self.0).finish()
    }
}
impl FromIntoMemory for NL_ROUTE_PROTOCOL {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NL_SUFFIX_ORIGIN(pub i32);
pub const NlsoOther: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(0i32);
pub const NlsoManual: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(1i32);
pub const NlsoWellKnown: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(2i32);
pub const NlsoDhcp: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(3i32);
pub const NlsoLinkLayerAddress: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(4i32);
pub const NlsoRandom: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(5i32);
pub const IpSuffixOriginOther: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(0i32);
pub const IpSuffixOriginManual: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(1i32);
pub const IpSuffixOriginWellKnown: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(2i32);
pub const IpSuffixOriginDhcp: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(3i32);
pub const IpSuffixOriginLinkLayerAddress: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(4i32);
pub const IpSuffixOriginRandom: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(5i32);
pub const IpSuffixOriginUnchanged: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(16i32);
impl ::core::marker::Copy for NL_SUFFIX_ORIGIN {}
impl ::core::clone::Clone for NL_SUFFIX_ORIGIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NL_SUFFIX_ORIGIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NL_SUFFIX_ORIGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NL_SUFFIX_ORIGIN").field(&self.0).finish()
    }
}
impl FromIntoMemory for NL_SUFFIX_ORIGIN {
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
pub const NSPROTO_IPX: u32 = 1000u32;
pub const NSPROTO_SPX: u32 = 1256u32;
pub const NSPROTO_SPXII: u32 = 1257u32;
pub struct NSPV2_ROUTINE {
    pub cbSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub NSPv2Startup: LPNSPV2STARTUP,
    pub NSPv2Cleanup: LPNSPV2CLEANUP,
    pub NSPv2LookupServiceBegin: LPNSPV2LOOKUPSERVICEBEGIN,
    pub NSPv2LookupServiceNextEx: LPNSPV2LOOKUPSERVICENEXTEX,
    pub NSPv2LookupServiceEnd: LPNSPV2LOOKUPSERVICEEND,
    pub NSPv2SetServiceEx: LPNSPV2SETSERVICEEX,
    pub NSPv2ClientSessionRundown: LPNSPV2CLIENTSESSIONRUNDOWN,
}
impl ::core::marker::Copy for NSPV2_ROUTINE {}
impl ::core::clone::Clone for NSPV2_ROUTINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NSPV2_ROUTINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NSPV2_ROUTINE")
            .field("cbSize", &self.cbSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("NSPv2Startup", &self.NSPv2Startup)
            .field("NSPv2Cleanup", &self.NSPv2Cleanup)
            .field("NSPv2LookupServiceBegin", &self.NSPv2LookupServiceBegin)
            .field("NSPv2LookupServiceNextEx", &self.NSPv2LookupServiceNextEx)
            .field("NSPv2LookupServiceEnd", &self.NSPv2LookupServiceEnd)
            .field("NSPv2SetServiceEx", &self.NSPv2SetServiceEx)
            .field("NSPv2ClientSessionRundown", &self.NSPv2ClientSessionRundown)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NSPV2_ROUTINE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwMajorVersion == other.dwMajorVersion
            && self.dwMinorVersion == other.dwMinorVersion
            && self.NSPv2Startup == other.NSPv2Startup
            && self.NSPv2Cleanup == other.NSPv2Cleanup
            && self.NSPv2LookupServiceBegin == other.NSPv2LookupServiceBegin
            && self.NSPv2LookupServiceNextEx == other.NSPv2LookupServiceNextEx
            && self.NSPv2LookupServiceEnd == other.NSPv2LookupServiceEnd
            && self.NSPv2SetServiceEx == other.NSPv2SetServiceEx
            && self.NSPv2ClientSessionRundown == other.NSPv2ClientSessionRundown
    }
}
impl ::core::cmp::Eq for NSPV2_ROUTINE {}
impl FromIntoMemory for NSPV2_ROUTINE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwMajorVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwMinorVersion = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_NSPv2Startup = <LPNSPV2STARTUP as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_NSPv2Cleanup = <LPNSPV2CLEANUP as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_NSPv2LookupServiceBegin =
            <LPNSPV2LOOKUPSERVICEBEGIN as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_NSPv2LookupServiceNextEx =
            <LPNSPV2LOOKUPSERVICENEXTEX as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_NSPv2LookupServiceEnd =
            <LPNSPV2LOOKUPSERVICEEND as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_NSPv2SetServiceEx =
            <LPNSPV2SETSERVICEEX as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_NSPv2ClientSessionRundown =
            <LPNSPV2CLIENTSESSIONRUNDOWN as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        Self {
            cbSize: f_cbSize,
            dwMajorVersion: f_dwMajorVersion,
            dwMinorVersion: f_dwMinorVersion,
            NSPv2Startup: f_NSPv2Startup,
            NSPv2Cleanup: f_NSPv2Cleanup,
            NSPv2LookupServiceBegin: f_NSPv2LookupServiceBegin,
            NSPv2LookupServiceNextEx: f_NSPv2LookupServiceNextEx,
            NSPv2LookupServiceEnd: f_NSPv2LookupServiceEnd,
            NSPv2SetServiceEx: f_NSPv2SetServiceEx,
            NSPv2ClientSessionRundown: f_NSPv2ClientSessionRundown,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwMajorVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwMinorVersion, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.NSPv2Startup, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.NSPv2Cleanup, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.NSPv2LookupServiceBegin, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.NSPv2LookupServiceNextEx, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.NSPv2LookupServiceEnd, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.NSPv2SetServiceEx, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.NSPv2ClientSessionRundown, &mut into[36..36 + 4]);
    }
    fn size() -> usize {
        40
    }
}
pub struct NSP_ROUTINE {
    pub cbSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub NSPCleanup: LPNSPCLEANUP,
    pub NSPLookupServiceBegin: LPNSPLOOKUPSERVICEBEGIN,
    pub NSPLookupServiceNext: LPNSPLOOKUPSERVICENEXT,
    pub NSPLookupServiceEnd: LPNSPLOOKUPSERVICEEND,
    pub NSPSetService: LPNSPSETSERVICE,
    pub NSPInstallServiceClass: LPNSPINSTALLSERVICECLASS,
    pub NSPRemoveServiceClass: LPNSPREMOVESERVICECLASS,
    pub NSPGetServiceClassInfo: LPNSPGETSERVICECLASSINFO,
    pub NSPIoctl: LPNSPIOCTL,
}
impl ::core::marker::Copy for NSP_ROUTINE {}
impl ::core::clone::Clone for NSP_ROUTINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NSP_ROUTINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NSP_ROUTINE")
            .field("cbSize", &self.cbSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("NSPCleanup", &self.NSPCleanup)
            .field("NSPLookupServiceBegin", &self.NSPLookupServiceBegin)
            .field("NSPLookupServiceNext", &self.NSPLookupServiceNext)
            .field("NSPLookupServiceEnd", &self.NSPLookupServiceEnd)
            .field("NSPSetService", &self.NSPSetService)
            .field("NSPInstallServiceClass", &self.NSPInstallServiceClass)
            .field("NSPRemoveServiceClass", &self.NSPRemoveServiceClass)
            .field("NSPGetServiceClassInfo", &self.NSPGetServiceClassInfo)
            .field("NSPIoctl", &self.NSPIoctl)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NSP_ROUTINE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwMajorVersion == other.dwMajorVersion
            && self.dwMinorVersion == other.dwMinorVersion
            && self.NSPCleanup == other.NSPCleanup
            && self.NSPLookupServiceBegin == other.NSPLookupServiceBegin
            && self.NSPLookupServiceNext == other.NSPLookupServiceNext
            && self.NSPLookupServiceEnd == other.NSPLookupServiceEnd
            && self.NSPSetService == other.NSPSetService
            && self.NSPInstallServiceClass == other.NSPInstallServiceClass
            && self.NSPRemoveServiceClass == other.NSPRemoveServiceClass
            && self.NSPGetServiceClassInfo == other.NSPGetServiceClassInfo
            && self.NSPIoctl == other.NSPIoctl
    }
}
impl ::core::cmp::Eq for NSP_ROUTINE {}
impl FromIntoMemory for NSP_ROUTINE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwMajorVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwMinorVersion = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_NSPCleanup = <LPNSPCLEANUP as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_NSPLookupServiceBegin =
            <LPNSPLOOKUPSERVICEBEGIN as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_NSPLookupServiceNext =
            <LPNSPLOOKUPSERVICENEXT as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_NSPLookupServiceEnd =
            <LPNSPLOOKUPSERVICEEND as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_NSPSetService = <LPNSPSETSERVICE as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_NSPInstallServiceClass =
            <LPNSPINSTALLSERVICECLASS as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_NSPRemoveServiceClass =
            <LPNSPREMOVESERVICECLASS as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_NSPGetServiceClassInfo =
            <LPNSPGETSERVICECLASSINFO as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_NSPIoctl = <LPNSPIOCTL as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            cbSize: f_cbSize,
            dwMajorVersion: f_dwMajorVersion,
            dwMinorVersion: f_dwMinorVersion,
            NSPCleanup: f_NSPCleanup,
            NSPLookupServiceBegin: f_NSPLookupServiceBegin,
            NSPLookupServiceNext: f_NSPLookupServiceNext,
            NSPLookupServiceEnd: f_NSPLookupServiceEnd,
            NSPSetService: f_NSPSetService,
            NSPInstallServiceClass: f_NSPInstallServiceClass,
            NSPRemoveServiceClass: f_NSPRemoveServiceClass,
            NSPGetServiceClassInfo: f_NSPGetServiceClassInfo,
            NSPIoctl: f_NSPIoctl,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwMajorVersion, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwMinorVersion, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.NSPCleanup, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.NSPLookupServiceBegin, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.NSPLookupServiceNext, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.NSPLookupServiceEnd, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.NSPSetService, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.NSPInstallServiceClass, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.NSPRemoveServiceClass, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.NSPGetServiceClassInfo, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.NSPIoctl, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub const NSTYPE_DYNAMIC: u32 = 2u32;
pub const NSTYPE_ENUMERABLE: u32 = 4u32;
pub const NSTYPE_HIERARCHICAL: u32 = 1u32;
pub const NSTYPE_WORKGROUP: u32 = 8u32;
pub const NS_ALL: u32 = 0u32;
pub const NS_DEFAULT: u32 = 0u32;
pub const NS_DHCP: u32 = 6u32;
pub const NS_DNS: u32 = 12u32;
pub const NS_EMAIL: u32 = 37u32;
pub struct NS_INFOA {
    pub dwNameSpace: u32,
    pub dwNameSpaceFlags: u32,
    pub lpNameSpace: PSTR,
}
impl ::core::marker::Copy for NS_INFOA {}
impl ::core::clone::Clone for NS_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NS_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NS_INFOA")
            .field("dwNameSpace", &self.dwNameSpace)
            .field("dwNameSpaceFlags", &self.dwNameSpaceFlags)
            .field("lpNameSpace", &self.lpNameSpace)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NS_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace
            && self.dwNameSpaceFlags == other.dwNameSpaceFlags
            && self.lpNameSpace == other.lpNameSpace
    }
}
impl ::core::cmp::Eq for NS_INFOA {}
impl FromIntoMemory for NS_INFOA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwNameSpaceFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpNameSpace = <PSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwNameSpace: f_dwNameSpace,
            dwNameSpaceFlags: f_dwNameSpaceFlags,
            lpNameSpace: f_lpNameSpace,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwNameSpaceFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpNameSpace, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct NS_INFOW {
    pub dwNameSpace: u32,
    pub dwNameSpaceFlags: u32,
    pub lpNameSpace: PWSTR,
}
impl ::core::marker::Copy for NS_INFOW {}
impl ::core::clone::Clone for NS_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NS_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NS_INFOW")
            .field("dwNameSpace", &self.dwNameSpace)
            .field("dwNameSpaceFlags", &self.dwNameSpaceFlags)
            .field("lpNameSpace", &self.lpNameSpace)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NS_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace
            && self.dwNameSpaceFlags == other.dwNameSpaceFlags
            && self.lpNameSpace == other.lpNameSpace
    }
}
impl ::core::cmp::Eq for NS_INFOW {}
impl FromIntoMemory for NS_INFOW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwNameSpaceFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpNameSpace = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            dwNameSpace: f_dwNameSpace,
            dwNameSpaceFlags: f_dwNameSpaceFlags,
            lpNameSpace: f_lpNameSpace,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwNameSpaceFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpNameSpace, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const NS_LOCALNAME: u32 = 19u32;
pub const NS_MS: u32 = 30u32;
pub const NS_NBP: u32 = 20u32;
pub const NS_NDS: u32 = 2u32;
pub const NS_NETBT: u32 = 13u32;
pub const NS_NETDES: u32 = 60u32;
pub const NS_NIS: u32 = 41u32;
pub const NS_NISPLUS: u32 = 42u32;
pub const NS_NLA: u32 = 15u32;
pub const NS_NTDS: u32 = 32u32;
pub const NS_PEER_BROWSE: u32 = 3u32;
pub const NS_SAP: u32 = 1u32;
pub struct NS_SERVICE_INFOA {
    pub dwNameSpace: u32,
    pub ServiceInfo: SERVICE_INFOA,
}
impl ::core::marker::Copy for NS_SERVICE_INFOA {}
impl ::core::clone::Clone for NS_SERVICE_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NS_SERVICE_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NS_SERVICE_INFOA")
            .field("dwNameSpace", &self.dwNameSpace)
            .field("ServiceInfo", &self.ServiceInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NS_SERVICE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.ServiceInfo == other.ServiceInfo
    }
}
impl ::core::cmp::Eq for NS_SERVICE_INFOA {}
impl FromIntoMemory for NS_SERVICE_INFOA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ServiceInfo = <SERVICE_INFOA as FromIntoMemory>::from_bytes(&from[4..4 + 44]);
        Self {
            dwNameSpace: f_dwNameSpace,
            ServiceInfo: f_ServiceInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ServiceInfo, &mut into[4..4 + 44]);
    }
    fn size() -> usize {
        48
    }
}
pub struct NS_SERVICE_INFOW {
    pub dwNameSpace: u32,
    pub ServiceInfo: SERVICE_INFOW,
}
impl ::core::marker::Copy for NS_SERVICE_INFOW {}
impl ::core::clone::Clone for NS_SERVICE_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NS_SERVICE_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NS_SERVICE_INFOW")
            .field("dwNameSpace", &self.dwNameSpace)
            .field("ServiceInfo", &self.ServiceInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for NS_SERVICE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace && self.ServiceInfo == other.ServiceInfo
    }
}
impl ::core::cmp::Eq for NS_SERVICE_INFOW {}
impl FromIntoMemory for NS_SERVICE_INFOW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ServiceInfo = <SERVICE_INFOW as FromIntoMemory>::from_bytes(&from[4..4 + 44]);
        Self {
            dwNameSpace: f_dwNameSpace,
            ServiceInfo: f_ServiceInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ServiceInfo, &mut into[4..4 + 44]);
    }
    fn size() -> usize {
        48
    }
}
pub const NS_SLP: u32 = 5u32;
pub const NS_STDA: u32 = 31u32;
pub const NS_TCPIP_HOSTS: u32 = 11u32;
pub const NS_TCPIP_LOCAL: u32 = 10u32;
pub const NS_VNS: u32 = 50u32;
pub const NS_WINS: u32 = 14u32;
pub const NS_WRQ: u32 = 50u32;
pub const NS_X500: u32 = 40u32;
pub const PFL_HIDDEN: u32 = 4u32;
pub const PFL_MATCHES_PROTOCOL_ZERO: u32 = 8u32;
pub const PFL_MULTIPLE_PROTO_ENTRIES: u32 = 1u32;
pub const PFL_NETWORKDIRECT_PROVIDER: u32 = 16u32;
pub const PFL_RECOMMENDED_PROTO_ENTRY: u32 = 2u32;
pub const PF_APPLETALK: u16 = 16u16;
pub const PF_ATM: u16 = 22u16;
pub const PF_BAN: u16 = 21u16;
pub const PF_CCITT: u16 = 10u16;
pub const PF_CHAOS: u16 = 5u16;
pub const PF_DATAKIT: u16 = 9u16;
pub const PF_DECnet: u16 = 12u16;
pub const PF_DLI: u16 = 13u16;
pub const PF_ECMA: u16 = 8u16;
pub const PF_FIREFOX: u16 = 19u16;
pub const PF_HYLINK: u16 = 15u16;
pub const PF_IMPLINK: u16 = 3u16;
pub const PF_IPX: u16 = 6u16;
pub const PF_IRDA: u16 = 26u16;
pub const PF_ISO: u16 = 7u16;
pub const PF_LAT: u16 = 14u16;
pub const PF_MAX: u16 = 29u16;
pub const PF_NS: u16 = 6u16;
pub const PF_OSI: u16 = 7u16;
pub const PF_PUP: u16 = 4u16;
pub const PF_SNA: u16 = 11u16;
pub const PF_UNIX: u16 = 1u16;
pub const PF_UNKNOWN1: u16 = 20u16;
pub const PF_VOICEVIEW: u16 = 18u16;
pub const PI_ALLOWED: u32 = 0u32;
pub const PI_NUMBER_NOT_AVAILABLE: u32 = 128u32;
pub const PI_RESTRICTED: u32 = 64u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PMTUD_STATE(pub i32);
pub const IP_PMTUDISC_NOT_SET: PMTUD_STATE = PMTUD_STATE(0i32);
pub const IP_PMTUDISC_DO: PMTUD_STATE = PMTUD_STATE(1i32);
pub const IP_PMTUDISC_DONT: PMTUD_STATE = PMTUD_STATE(2i32);
pub const IP_PMTUDISC_PROBE: PMTUD_STATE = PMTUD_STATE(3i32);
pub const IP_PMTUDISC_MAX: PMTUD_STATE = PMTUD_STATE(4i32);
impl ::core::marker::Copy for PMTUD_STATE {}
impl ::core::clone::Clone for PMTUD_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PMTUD_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PMTUD_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PMTUD_STATE").field(&self.0).finish()
    }
}
impl FromIntoMemory for PMTUD_STATE {
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
pub const POLLERR: u32 = 1u32;
pub const POLLHUP: u32 = 2u32;
pub const POLLNVAL: u32 = 4u32;
pub const POLLOUT: u32 = 16u32;
pub const POLLPRI: u32 = 1024u32;
pub const POLLRDBAND: u32 = 512u32;
pub const POLLRDNORM: u32 = 256u32;
pub const POLLWRBAND: u32 = 32u32;
pub const POLLWRNORM: u32 = 16u32;
pub struct PRIORITY_STATUS {
    pub Sender: SOCKET_PRIORITY_HINT,
    pub Receiver: SOCKET_PRIORITY_HINT,
}
impl ::core::marker::Copy for PRIORITY_STATUS {}
impl ::core::clone::Clone for PRIORITY_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PRIORITY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRIORITY_STATUS")
            .field("Sender", &self.Sender)
            .field("Receiver", &self.Receiver)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PRIORITY_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Sender == other.Sender && self.Receiver == other.Receiver
    }
}
impl ::core::cmp::Eq for PRIORITY_STATUS {}
impl FromIntoMemory for PRIORITY_STATUS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Sender = <SOCKET_PRIORITY_HINT as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Receiver = <SOCKET_PRIORITY_HINT as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Sender: f_Sender,
            Receiver: f_Receiver,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Sender, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Receiver, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub const PROP_ADDRESSES: u32 = 256u32;
pub const PROP_ALL: u32 = 2147483648u32;
pub const PROP_COMMENT: u32 = 1u32;
pub const PROP_DISPLAY_HINT: u32 = 4u32;
pub const PROP_LOCALE: u32 = 2u32;
pub const PROP_MACHINE: u32 = 32u32;
pub const PROP_SD: u32 = 512u32;
pub const PROP_START_TIME: u32 = 16u32;
pub const PROP_VERSION: u32 = 8u32;
pub const PROTECTION_LEVEL_DEFAULT: u32 = 20u32;
pub const PROTECTION_LEVEL_EDGERESTRICTED: u32 = 20u32;
pub const PROTECTION_LEVEL_RESTRICTED: u32 = 30u32;
pub const PROTECTION_LEVEL_UNRESTRICTED: u32 = 10u32;
pub struct PROTOCOL_INFOA {
    pub dwServiceFlags: u32,
    pub iAddressFamily: i32,
    pub iMaxSockAddr: i32,
    pub iMinSockAddr: i32,
    pub iSocketType: i32,
    pub iProtocol: i32,
    pub dwMessageSize: u32,
    pub lpProtocol: PSTR,
}
impl ::core::marker::Copy for PROTOCOL_INFOA {}
impl ::core::clone::Clone for PROTOCOL_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROTOCOL_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROTOCOL_INFOA")
            .field("dwServiceFlags", &self.dwServiceFlags)
            .field("iAddressFamily", &self.iAddressFamily)
            .field("iMaxSockAddr", &self.iMaxSockAddr)
            .field("iMinSockAddr", &self.iMinSockAddr)
            .field("iSocketType", &self.iSocketType)
            .field("iProtocol", &self.iProtocol)
            .field("dwMessageSize", &self.dwMessageSize)
            .field("lpProtocol", &self.lpProtocol)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROTOCOL_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceFlags == other.dwServiceFlags
            && self.iAddressFamily == other.iAddressFamily
            && self.iMaxSockAddr == other.iMaxSockAddr
            && self.iMinSockAddr == other.iMinSockAddr
            && self.iSocketType == other.iSocketType
            && self.iProtocol == other.iProtocol
            && self.dwMessageSize == other.dwMessageSize
            && self.lpProtocol == other.lpProtocol
    }
}
impl ::core::cmp::Eq for PROTOCOL_INFOA {}
impl FromIntoMemory for PROTOCOL_INFOA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_dwServiceFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_iAddressFamily = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_iMaxSockAddr = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_iMinSockAddr = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_iSocketType = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_iProtocol = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwMessageSize = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lpProtocol = <PSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            dwServiceFlags: f_dwServiceFlags,
            iAddressFamily: f_iAddressFamily,
            iMaxSockAddr: f_iMaxSockAddr,
            iMinSockAddr: f_iMinSockAddr,
            iSocketType: f_iSocketType,
            iProtocol: f_iProtocol,
            dwMessageSize: f_dwMessageSize,
            lpProtocol: f_lpProtocol,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.dwServiceFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.iAddressFamily, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.iMaxSockAddr, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.iMinSockAddr, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.iSocketType, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.iProtocol, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwMessageSize, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpProtocol, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct PROTOCOL_INFOW {
    pub dwServiceFlags: u32,
    pub iAddressFamily: i32,
    pub iMaxSockAddr: i32,
    pub iMinSockAddr: i32,
    pub iSocketType: i32,
    pub iProtocol: i32,
    pub dwMessageSize: u32,
    pub lpProtocol: PWSTR,
}
impl ::core::marker::Copy for PROTOCOL_INFOW {}
impl ::core::clone::Clone for PROTOCOL_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROTOCOL_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROTOCOL_INFOW")
            .field("dwServiceFlags", &self.dwServiceFlags)
            .field("iAddressFamily", &self.iAddressFamily)
            .field("iMaxSockAddr", &self.iMaxSockAddr)
            .field("iMinSockAddr", &self.iMinSockAddr)
            .field("iSocketType", &self.iSocketType)
            .field("iProtocol", &self.iProtocol)
            .field("dwMessageSize", &self.dwMessageSize)
            .field("lpProtocol", &self.lpProtocol)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PROTOCOL_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceFlags == other.dwServiceFlags
            && self.iAddressFamily == other.iAddressFamily
            && self.iMaxSockAddr == other.iMaxSockAddr
            && self.iMinSockAddr == other.iMinSockAddr
            && self.iSocketType == other.iSocketType
            && self.iProtocol == other.iProtocol
            && self.dwMessageSize == other.dwMessageSize
            && self.lpProtocol == other.lpProtocol
    }
}
impl ::core::cmp::Eq for PROTOCOL_INFOW {}
impl FromIntoMemory for PROTOCOL_INFOW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_dwServiceFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_iAddressFamily = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_iMaxSockAddr = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_iMinSockAddr = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_iSocketType = <i32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_iProtocol = <i32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwMessageSize = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lpProtocol = <PWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            dwServiceFlags: f_dwServiceFlags,
            iAddressFamily: f_iAddressFamily,
            iMaxSockAddr: f_iMaxSockAddr,
            iMinSockAddr: f_iMinSockAddr,
            iSocketType: f_iSocketType,
            iProtocol: f_iProtocol,
            dwMessageSize: f_dwMessageSize,
            lpProtocol: f_lpProtocol,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.dwServiceFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.iAddressFamily, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.iMaxSockAddr, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.iMinSockAddr, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.iSocketType, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.iProtocol, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwMessageSize, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpProtocol, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub const PVD_CONFIG: u32 = 12289u32;
pub struct Q2931_IE {
    pub IEType: Q2931_IE_TYPE,
    pub IELength: u32,
    pub IE: [u8; 1],
}
impl ::core::marker::Copy for Q2931_IE {}
impl ::core::clone::Clone for Q2931_IE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Q2931_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Q2931_IE")
            .field("IEType", &self.IEType)
            .field("IELength", &self.IELength)
            .field("IE", &self.IE)
            .finish()
    }
}
impl ::core::cmp::PartialEq for Q2931_IE {
    fn eq(&self, other: &Self) -> bool {
        self.IEType == other.IEType && self.IELength == other.IELength && self.IE == other.IE
    }
}
impl ::core::cmp::Eq for Q2931_IE {}
impl FromIntoMemory for Q2931_IE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_IEType = <Q2931_IE_TYPE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_IELength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_IE = <[u8; 1] as FromIntoMemory>::from_bytes(&from[8..8 + 1]);
        Self {
            IEType: f_IEType,
            IELength: f_IELength,
            IE: f_IE,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.IEType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.IELength, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.IE, &mut into[8..8 + 1]);
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct Q2931_IE_TYPE(pub i32);
pub const IE_AALParameters: Q2931_IE_TYPE = Q2931_IE_TYPE(0i32);
pub const IE_TrafficDescriptor: Q2931_IE_TYPE = Q2931_IE_TYPE(1i32);
pub const IE_BroadbandBearerCapability: Q2931_IE_TYPE = Q2931_IE_TYPE(2i32);
pub const IE_BHLI: Q2931_IE_TYPE = Q2931_IE_TYPE(3i32);
pub const IE_BLLI: Q2931_IE_TYPE = Q2931_IE_TYPE(4i32);
pub const IE_CalledPartyNumber: Q2931_IE_TYPE = Q2931_IE_TYPE(5i32);
pub const IE_CalledPartySubaddress: Q2931_IE_TYPE = Q2931_IE_TYPE(6i32);
pub const IE_CallingPartyNumber: Q2931_IE_TYPE = Q2931_IE_TYPE(7i32);
pub const IE_CallingPartySubaddress: Q2931_IE_TYPE = Q2931_IE_TYPE(8i32);
pub const IE_Cause: Q2931_IE_TYPE = Q2931_IE_TYPE(9i32);
pub const IE_QOSClass: Q2931_IE_TYPE = Q2931_IE_TYPE(10i32);
pub const IE_TransitNetworkSelection: Q2931_IE_TYPE = Q2931_IE_TYPE(11i32);
impl ::core::marker::Copy for Q2931_IE_TYPE {}
impl ::core::clone::Clone for Q2931_IE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Q2931_IE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Q2931_IE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Q2931_IE_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for Q2931_IE_TYPE {
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
pub const QOS_CLASS0: u32 = 0u32;
pub const QOS_CLASS1: u32 = 1u32;
pub const QOS_CLASS2: u32 = 2u32;
pub const QOS_CLASS3: u32 = 3u32;
pub const QOS_CLASS4: u32 = 4u32;
pub struct RCVALL_IF {
    pub Mode: RCVALL_VALUE,
    pub Interface: u32,
}
impl ::core::marker::Copy for RCVALL_IF {}
impl ::core::clone::Clone for RCVALL_IF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RCVALL_IF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RCVALL_IF")
            .field("Mode", &self.Mode)
            .field("Interface", &self.Interface)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RCVALL_IF {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.Interface == other.Interface
    }
}
impl ::core::cmp::Eq for RCVALL_IF {}
impl FromIntoMemory for RCVALL_IF {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Mode = <RCVALL_VALUE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Interface = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Mode: f_Mode,
            Interface: f_Interface,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Mode, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Interface, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RCVALL_VALUE(pub i32);
pub const RCVALL_OFF: RCVALL_VALUE = RCVALL_VALUE(0i32);
pub const RCVALL_ON: RCVALL_VALUE = RCVALL_VALUE(1i32);
pub const RCVALL_SOCKETLEVELONLY: RCVALL_VALUE = RCVALL_VALUE(2i32);
pub const RCVALL_IPLEVEL: RCVALL_VALUE = RCVALL_VALUE(3i32);
impl ::core::marker::Copy for RCVALL_VALUE {}
impl ::core::clone::Clone for RCVALL_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RCVALL_VALUE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RCVALL_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RCVALL_VALUE").field(&self.0).finish()
    }
}
impl FromIntoMemory for RCVALL_VALUE {
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
pub const REAL_TIME_NOTIFICATION_CAPABILITY: crate::core::GUID =
    crate::core::GUID::from_u128(0x6b59819a_5cae_492d_a901_2a3c2c50164f);
pub const REAL_TIME_NOTIFICATION_CAPABILITY_EX: crate::core::GUID =
    crate::core::GUID::from_u128(0x6843da03_154a_4616_a508_44371295f96b);
pub struct REAL_TIME_NOTIFICATION_SETTING_INPUT {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub BrokerEventGuid: crate::core::GUID,
}
impl ::core::marker::Copy for REAL_TIME_NOTIFICATION_SETTING_INPUT {}
impl ::core::clone::Clone for REAL_TIME_NOTIFICATION_SETTING_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REAL_TIME_NOTIFICATION_SETTING_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REAL_TIME_NOTIFICATION_SETTING_INPUT")
            .field("TransportSettingId", &self.TransportSettingId)
            .field("BrokerEventGuid", &self.BrokerEventGuid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for REAL_TIME_NOTIFICATION_SETTING_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.TransportSettingId == other.TransportSettingId
            && self.BrokerEventGuid == other.BrokerEventGuid
    }
}
impl ::core::cmp::Eq for REAL_TIME_NOTIFICATION_SETTING_INPUT {}
impl FromIntoMemory for REAL_TIME_NOTIFICATION_SETTING_INPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_TransportSettingId =
            <TRANSPORT_SETTING_ID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_BrokerEventGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        Self {
            TransportSettingId: f_TransportSettingId,
            BrokerEventGuid: f_BrokerEventGuid,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.TransportSettingId, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.BrokerEventGuid, &mut into[16..16 + 16]);
    }
    fn size() -> usize {
        32
    }
}
pub struct REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub BrokerEventGuid: crate::core::GUID,
    pub Unmark: super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {}
impl ::core::clone::Clone for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REAL_TIME_NOTIFICATION_SETTING_INPUT_EX")
            .field("TransportSettingId", &self.TransportSettingId)
            .field("BrokerEventGuid", &self.BrokerEventGuid)
            .field("Unmark", &self.Unmark)
            .finish()
    }
}
impl ::core::cmp::PartialEq for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.TransportSettingId == other.TransportSettingId
            && self.BrokerEventGuid == other.BrokerEventGuid
            && self.Unmark == other.Unmark
    }
}
impl ::core::cmp::Eq for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {}
impl FromIntoMemory for REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_TransportSettingId =
            <TRANSPORT_SETTING_ID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_BrokerEventGuid =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[16..16 + 16]);
        let f_Unmark =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[32..32 + 1]);
        Self {
            TransportSettingId: f_TransportSettingId,
            BrokerEventGuid: f_BrokerEventGuid,
            Unmark: f_Unmark,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.TransportSettingId, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.BrokerEventGuid, &mut into[16..16 + 16]);
        FromIntoMemory::into_bytes(self.Unmark, &mut into[32..32 + 1]);
    }
    fn size() -> usize {
        36
    }
}
pub struct REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    pub ChannelStatus: CONTROL_CHANNEL_TRIGGER_STATUS,
}
impl ::core::marker::Copy for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {}
impl ::core::clone::Clone for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REAL_TIME_NOTIFICATION_SETTING_OUTPUT")
            .field("ChannelStatus", &self.ChannelStatus)
            .finish()
    }
}
impl ::core::cmp::PartialEq for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ChannelStatus == other.ChannelStatus
    }
}
impl ::core::cmp::Eq for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {}
impl FromIntoMemory for REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_ChannelStatus =
            <CONTROL_CHANNEL_TRIGGER_STATUS as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            ChannelStatus: f_ChannelStatus,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.ChannelStatus, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RESOURCE_DISPLAY_TYPE(pub u32);
pub const RESOURCEDISPLAYTYPE_DOMAIN: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(1u32);
pub const RESOURCEDISPLAYTYPE_FILE: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(4u32);
pub const RESOURCEDISPLAYTYPE_GENERIC: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(0u32);
pub const RESOURCEDISPLAYTYPE_GROUP: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(5u32);
pub const RESOURCEDISPLAYTYPE_SERVER: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(2u32);
pub const RESOURCEDISPLAYTYPE_SHARE: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(3u32);
pub const RESOURCEDISPLAYTYPE_TREE: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(10u32);
impl ::core::marker::Copy for RESOURCE_DISPLAY_TYPE {}
impl ::core::clone::Clone for RESOURCE_DISPLAY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RESOURCE_DISPLAY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RESOURCE_DISPLAY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESOURCE_DISPLAY_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for RESOURCE_DISPLAY_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const RESULT_IS_ADDED: u32 = 16u32;
pub const RESULT_IS_ALIAS: u32 = 1u32;
pub const RESULT_IS_CHANGED: u32 = 32u32;
pub const RESULT_IS_DELETED: u32 = 64u32;
pub const RES_FIND_MULTIPLE: u32 = 2u32;
pub const RES_FLUSH_CACHE: u32 = 2u32;
pub const RES_SERVICE: u32 = 4u32;
pub const RES_SOFT_SEARCH: u32 = 1u32;
pub const RES_UNUSED_1: u32 = 1u32;
pub struct RIORESULT {
    pub Status: i32,
    pub BytesTransferred: u32,
    pub SocketContext: u64,
    pub RequestContext: u64,
}
impl ::core::marker::Copy for RIORESULT {}
impl ::core::clone::Clone for RIORESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RIORESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RIORESULT")
            .field("Status", &self.Status)
            .field("BytesTransferred", &self.BytesTransferred)
            .field("SocketContext", &self.SocketContext)
            .field("RequestContext", &self.RequestContext)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RIORESULT {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status
            && self.BytesTransferred == other.BytesTransferred
            && self.SocketContext == other.SocketContext
            && self.RequestContext == other.RequestContext
    }
}
impl ::core::cmp::Eq for RIORESULT {}
impl FromIntoMemory for RIORESULT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_Status = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_BytesTransferred = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SocketContext = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_RequestContext = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        Self {
            Status: f_Status,
            BytesTransferred: f_BytesTransferred,
            SocketContext: f_SocketContext,
            RequestContext: f_RequestContext,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.Status, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.BytesTransferred, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SocketContext, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.RequestContext, &mut into[16..16 + 8]);
    }
    fn size() -> usize {
        24
    }
}
pub struct RIO_BUF {
    pub BufferId: MutPtr<RIO_BUFFERID_t>,
    pub Offset: u32,
    pub Length: u32,
}
impl ::core::marker::Copy for RIO_BUF {}
impl ::core::clone::Clone for RIO_BUF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RIO_BUF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RIO_BUF")
            .field("BufferId", &self.BufferId)
            .field("Offset", &self.Offset)
            .field("Length", &self.Length)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RIO_BUF {
    fn eq(&self, other: &Self) -> bool {
        self.BufferId == other.BufferId
            && self.Offset == other.Offset
            && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for RIO_BUF {}
impl FromIntoMemory for RIO_BUF {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_BufferId = <MutPtr<RIO_BUFFERID_t> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Offset = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Length = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            BufferId: f_BufferId,
            Offset: f_Offset,
            Length: f_Length,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.BufferId, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Offset, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Length, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct RIO_BUFFERID_t(pub u8);
pub struct RIO_CMSG_BUFFER {
    pub TotalLength: u32,
}
impl ::core::marker::Copy for RIO_CMSG_BUFFER {}
impl ::core::clone::Clone for RIO_CMSG_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RIO_CMSG_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RIO_CMSG_BUFFER")
            .field("TotalLength", &self.TotalLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RIO_CMSG_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.TotalLength == other.TotalLength
    }
}
impl ::core::cmp::Eq for RIO_CMSG_BUFFER {}
impl FromIntoMemory for RIO_CMSG_BUFFER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_TotalLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            TotalLength: f_TotalLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.TotalLength, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub const RIO_CORRUPT_CQ: u32 = 4294967295u32;
pub struct RIO_CQ_t(pub u8);
pub struct RIO_EXTENSION_FUNCTION_TABLE {
    pub cbSize: u32,
    pub RIOReceive: LPFN_RIORECEIVE,
    pub RIOReceiveEx: LPFN_RIORECEIVEEX,
    pub RIOSend: LPFN_RIOSEND,
    pub RIOSendEx: LPFN_RIOSENDEX,
    pub RIOCloseCompletionQueue: LPFN_RIOCLOSECOMPLETIONQUEUE,
    pub RIOCreateCompletionQueue: LPFN_RIOCREATECOMPLETIONQUEUE,
    pub RIOCreateRequestQueue: LPFN_RIOCREATEREQUESTQUEUE,
    pub RIODequeueCompletion: LPFN_RIODEQUEUECOMPLETION,
    pub RIODeregisterBuffer: LPFN_RIODEREGISTERBUFFER,
    pub RIONotify: LPFN_RIONOTIFY,
    pub RIORegisterBuffer: LPFN_RIOREGISTERBUFFER,
    pub RIOResizeCompletionQueue: LPFN_RIORESIZECOMPLETIONQUEUE,
    pub RIOResizeRequestQueue: LPFN_RIORESIZEREQUESTQUEUE,
}
impl ::core::marker::Copy for RIO_EXTENSION_FUNCTION_TABLE {}
impl ::core::clone::Clone for RIO_EXTENSION_FUNCTION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RIO_EXTENSION_FUNCTION_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RIO_EXTENSION_FUNCTION_TABLE")
            .field("cbSize", &self.cbSize)
            .field("RIOReceive", &self.RIOReceive)
            .field("RIOReceiveEx", &self.RIOReceiveEx)
            .field("RIOSend", &self.RIOSend)
            .field("RIOSendEx", &self.RIOSendEx)
            .field("RIOCloseCompletionQueue", &self.RIOCloseCompletionQueue)
            .field("RIOCreateCompletionQueue", &self.RIOCreateCompletionQueue)
            .field("RIOCreateRequestQueue", &self.RIOCreateRequestQueue)
            .field("RIODequeueCompletion", &self.RIODequeueCompletion)
            .field("RIODeregisterBuffer", &self.RIODeregisterBuffer)
            .field("RIONotify", &self.RIONotify)
            .field("RIORegisterBuffer", &self.RIORegisterBuffer)
            .field("RIOResizeCompletionQueue", &self.RIOResizeCompletionQueue)
            .field("RIOResizeRequestQueue", &self.RIOResizeRequestQueue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RIO_EXTENSION_FUNCTION_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.RIOReceive == other.RIOReceive
            && self.RIOReceiveEx == other.RIOReceiveEx
            && self.RIOSend == other.RIOSend
            && self.RIOSendEx == other.RIOSendEx
            && self.RIOCloseCompletionQueue == other.RIOCloseCompletionQueue
            && self.RIOCreateCompletionQueue == other.RIOCreateCompletionQueue
            && self.RIOCreateRequestQueue == other.RIOCreateRequestQueue
            && self.RIODequeueCompletion == other.RIODequeueCompletion
            && self.RIODeregisterBuffer == other.RIODeregisterBuffer
            && self.RIONotify == other.RIONotify
            && self.RIORegisterBuffer == other.RIORegisterBuffer
            && self.RIOResizeCompletionQueue == other.RIOResizeCompletionQueue
            && self.RIOResizeRequestQueue == other.RIOResizeRequestQueue
    }
}
impl ::core::cmp::Eq for RIO_EXTENSION_FUNCTION_TABLE {}
impl FromIntoMemory for RIO_EXTENSION_FUNCTION_TABLE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 56);
        let f_cbSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_RIOReceive = <LPFN_RIORECEIVE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_RIOReceiveEx = <LPFN_RIORECEIVEEX as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_RIOSend = <LPFN_RIOSEND as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_RIOSendEx = <LPFN_RIOSENDEX as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_RIOCloseCompletionQueue =
            <LPFN_RIOCLOSECOMPLETIONQUEUE as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_RIOCreateCompletionQueue =
            <LPFN_RIOCREATECOMPLETIONQUEUE as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_RIOCreateRequestQueue =
            <LPFN_RIOCREATEREQUESTQUEUE as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_RIODequeueCompletion =
            <LPFN_RIODEQUEUECOMPLETION as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_RIODeregisterBuffer =
            <LPFN_RIODEREGISTERBUFFER as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_RIONotify = <LPFN_RIONOTIFY as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_RIORegisterBuffer =
            <LPFN_RIOREGISTERBUFFER as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_RIOResizeCompletionQueue =
            <LPFN_RIORESIZECOMPLETIONQUEUE as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_RIOResizeRequestQueue =
            <LPFN_RIORESIZEREQUESTQUEUE as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        Self {
            cbSize: f_cbSize,
            RIOReceive: f_RIOReceive,
            RIOReceiveEx: f_RIOReceiveEx,
            RIOSend: f_RIOSend,
            RIOSendEx: f_RIOSendEx,
            RIOCloseCompletionQueue: f_RIOCloseCompletionQueue,
            RIOCreateCompletionQueue: f_RIOCreateCompletionQueue,
            RIOCreateRequestQueue: f_RIOCreateRequestQueue,
            RIODequeueCompletion: f_RIODequeueCompletion,
            RIODeregisterBuffer: f_RIODeregisterBuffer,
            RIONotify: f_RIONotify,
            RIORegisterBuffer: f_RIORegisterBuffer,
            RIOResizeCompletionQueue: f_RIOResizeCompletionQueue,
            RIOResizeRequestQueue: f_RIOResizeRequestQueue,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 56);
        FromIntoMemory::into_bytes(self.cbSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.RIOReceive, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.RIOReceiveEx, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.RIOSend, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.RIOSendEx, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.RIOCloseCompletionQueue, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.RIOCreateCompletionQueue, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.RIOCreateRequestQueue, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.RIODequeueCompletion, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.RIODeregisterBuffer, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.RIONotify, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.RIORegisterBuffer, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.RIOResizeCompletionQueue, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.RIOResizeRequestQueue, &mut into[52..52 + 4]);
    }
    fn size() -> usize {
        56
    }
}
pub const RIO_MAX_CQ_SIZE: u32 = 134217728u32;
pub const RIO_MSG_COMMIT_ONLY: u32 = 8u32;
pub const RIO_MSG_DEFER: u32 = 2u32;
pub const RIO_MSG_DONT_NOTIFY: u32 = 1u32;
pub const RIO_MSG_WAITALL: u32 = 4u32;
pub struct RIO_NOTIFICATION_COMPLETION {
    pub Type: RIO_NOTIFICATION_COMPLETION_TYPE,
    pub Anonymous: RIO_NOTIFICATION_COMPLETION_0,
}
impl ::core::marker::Copy for RIO_NOTIFICATION_COMPLETION {}
impl ::core::clone::Clone for RIO_NOTIFICATION_COMPLETION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RIO_NOTIFICATION_COMPLETION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RIO_NOTIFICATION_COMPLETION")
            .field("Type", &self.Type)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RIO_NOTIFICATION_COMPLETION {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for RIO_NOTIFICATION_COMPLETION {}
impl FromIntoMemory for RIO_NOTIFICATION_COMPLETION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_Type =
            <RIO_NOTIFICATION_COMPLETION_TYPE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Anonymous =
            <RIO_NOTIFICATION_COMPLETION_0 as FromIntoMemory>::from_bytes(&from[4..4 + 12]);
        Self {
            Type: f_Type,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.Type, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[4..4 + 12]);
    }
    fn size() -> usize {
        16
    }
}
pub struct RIO_NOTIFICATION_COMPLETION_0 {
    data: [u8; 12],
}
impl ::core::default::Default for RIO_NOTIFICATION_COMPLETION_0 {
    fn default() -> Self {
        Self { data: [0u8; 12] }
    }
}
impl ::core::marker::Copy for RIO_NOTIFICATION_COMPLETION_0 {}
impl ::core::clone::Clone for RIO_NOTIFICATION_COMPLETION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RIO_NOTIFICATION_COMPLETION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RIO_NOTIFICATION_COMPLETION_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RIO_NOTIFICATION_COMPLETION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for RIO_NOTIFICATION_COMPLETION_0 {}
impl FromIntoMemory for RIO_NOTIFICATION_COMPLETION_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 12];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        12
    }
}
pub struct RIO_NOTIFICATION_COMPLETION_0_0 {
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyReset: super::super::Foundation::BOOL,
}
impl ::core::marker::Copy for RIO_NOTIFICATION_COMPLETION_0_0 {}
impl ::core::clone::Clone for RIO_NOTIFICATION_COMPLETION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RIO_NOTIFICATION_COMPLETION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RIO_NOTIFICATION_COMPLETION_0_0")
            .field("EventHandle", &self.EventHandle)
            .field("NotifyReset", &self.NotifyReset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RIO_NOTIFICATION_COMPLETION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.EventHandle == other.EventHandle && self.NotifyReset == other.NotifyReset
    }
}
impl ::core::cmp::Eq for RIO_NOTIFICATION_COMPLETION_0_0 {}
impl FromIntoMemory for RIO_NOTIFICATION_COMPLETION_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_EventHandle =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_NotifyReset =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            EventHandle: f_EventHandle,
            NotifyReset: f_NotifyReset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.EventHandle, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.NotifyReset, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct RIO_NOTIFICATION_COMPLETION_0_1 {
    pub IocpHandle: super::super::Foundation::HANDLE,
    pub CompletionKey: MutPtr<::core::ffi::c_void>,
    pub Overlapped: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for RIO_NOTIFICATION_COMPLETION_0_1 {}
impl ::core::clone::Clone for RIO_NOTIFICATION_COMPLETION_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RIO_NOTIFICATION_COMPLETION_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RIO_NOTIFICATION_COMPLETION_0_1")
            .field("IocpHandle", &self.IocpHandle)
            .field("CompletionKey", &self.CompletionKey)
            .field("Overlapped", &self.Overlapped)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RIO_NOTIFICATION_COMPLETION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.IocpHandle == other.IocpHandle
            && self.CompletionKey == other.CompletionKey
            && self.Overlapped == other.Overlapped
    }
}
impl ::core::cmp::Eq for RIO_NOTIFICATION_COMPLETION_0_1 {}
impl FromIntoMemory for RIO_NOTIFICATION_COMPLETION_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_IocpHandle =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_CompletionKey =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Overlapped =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            IocpHandle: f_IocpHandle,
            CompletionKey: f_CompletionKey,
            Overlapped: f_Overlapped,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.IocpHandle, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.CompletionKey, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Overlapped, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RIO_NOTIFICATION_COMPLETION_TYPE(pub i32);
pub const RIO_EVENT_COMPLETION: RIO_NOTIFICATION_COMPLETION_TYPE =
    RIO_NOTIFICATION_COMPLETION_TYPE(1i32);
pub const RIO_IOCP_COMPLETION: RIO_NOTIFICATION_COMPLETION_TYPE =
    RIO_NOTIFICATION_COMPLETION_TYPE(2i32);
impl ::core::marker::Copy for RIO_NOTIFICATION_COMPLETION_TYPE {}
impl ::core::clone::Clone for RIO_NOTIFICATION_COMPLETION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RIO_NOTIFICATION_COMPLETION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RIO_NOTIFICATION_COMPLETION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RIO_NOTIFICATION_COMPLETION_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for RIO_NOTIFICATION_COMPLETION_TYPE {
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
pub struct RIO_RQ_t(pub u8);
pub const RM_ADD_RECEIVE_IF: u32 = 1008u32;
pub const RM_DEL_RECEIVE_IF: u32 = 1009u32;
pub struct RM_FEC_INFO {
    pub FECBlockSize: u16,
    pub FECProActivePackets: u16,
    pub FECGroupSize: u8,
    pub fFECOnDemandParityEnabled: super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for RM_FEC_INFO {}
impl ::core::clone::Clone for RM_FEC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RM_FEC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RM_FEC_INFO")
            .field("FECBlockSize", &self.FECBlockSize)
            .field("FECProActivePackets", &self.FECProActivePackets)
            .field("FECGroupSize", &self.FECGroupSize)
            .field("fFECOnDemandParityEnabled", &self.fFECOnDemandParityEnabled)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RM_FEC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FECBlockSize == other.FECBlockSize
            && self.FECProActivePackets == other.FECProActivePackets
            && self.FECGroupSize == other.FECGroupSize
            && self.fFECOnDemandParityEnabled == other.fFECOnDemandParityEnabled
    }
}
impl ::core::cmp::Eq for RM_FEC_INFO {}
impl FromIntoMemory for RM_FEC_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 6);
        let f_FECBlockSize = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_FECProActivePackets = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_FECGroupSize = <u8 as FromIntoMemory>::from_bytes(&from[4..4 + 1]);
        let f_fFECOnDemandParityEnabled =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[5..5 + 1]);
        Self {
            FECBlockSize: f_FECBlockSize,
            FECProActivePackets: f_FECProActivePackets,
            FECGroupSize: f_FECGroupSize,
            fFECOnDemandParityEnabled: f_fFECOnDemandParityEnabled,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 6);
        FromIntoMemory::into_bytes(self.FECBlockSize, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.FECProActivePackets, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.FECGroupSize, &mut into[4..4 + 1]);
        FromIntoMemory::into_bytes(self.fFECOnDemandParityEnabled, &mut into[5..5 + 1]);
    }
    fn size() -> usize {
        6
    }
}
pub const RM_FLUSHCACHE: u32 = 1003u32;
pub const RM_HIGH_SPEED_INTRANET_OPT: u32 = 1014u32;
pub const RM_LATEJOIN: u32 = 1006u32;
pub const RM_OPTIONSBASE: u32 = 1000u32;
pub const RM_RATE_WINDOW_SIZE: u32 = 1001u32;
pub const RM_RECEIVER_STATISTICS: u32 = 1013u32;
pub struct RM_RECEIVER_STATS {
    pub NumODataPacketsReceived: u64,
    pub NumRDataPacketsReceived: u64,
    pub NumDuplicateDataPackets: u64,
    pub DataBytesReceived: u64,
    pub TotalBytesReceived: u64,
    pub RateKBitsPerSecOverall: u64,
    pub RateKBitsPerSecLast: u64,
    pub TrailingEdgeSeqId: u64,
    pub LeadingEdgeSeqId: u64,
    pub AverageSequencesInWindow: u64,
    pub MinSequencesInWindow: u64,
    pub MaxSequencesInWindow: u64,
    pub FirstNakSequenceNumber: u64,
    pub NumPendingNaks: u64,
    pub NumOutstandingNaks: u64,
    pub NumDataPacketsBuffered: u64,
    pub TotalSelectiveNaksSent: u64,
    pub TotalParityNaksSent: u64,
}
impl ::core::marker::Copy for RM_RECEIVER_STATS {}
impl ::core::clone::Clone for RM_RECEIVER_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RM_RECEIVER_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RM_RECEIVER_STATS")
            .field("NumODataPacketsReceived", &self.NumODataPacketsReceived)
            .field("NumRDataPacketsReceived", &self.NumRDataPacketsReceived)
            .field("NumDuplicateDataPackets", &self.NumDuplicateDataPackets)
            .field("DataBytesReceived", &self.DataBytesReceived)
            .field("TotalBytesReceived", &self.TotalBytesReceived)
            .field("RateKBitsPerSecOverall", &self.RateKBitsPerSecOverall)
            .field("RateKBitsPerSecLast", &self.RateKBitsPerSecLast)
            .field("TrailingEdgeSeqId", &self.TrailingEdgeSeqId)
            .field("LeadingEdgeSeqId", &self.LeadingEdgeSeqId)
            .field("AverageSequencesInWindow", &self.AverageSequencesInWindow)
            .field("MinSequencesInWindow", &self.MinSequencesInWindow)
            .field("MaxSequencesInWindow", &self.MaxSequencesInWindow)
            .field("FirstNakSequenceNumber", &self.FirstNakSequenceNumber)
            .field("NumPendingNaks", &self.NumPendingNaks)
            .field("NumOutstandingNaks", &self.NumOutstandingNaks)
            .field("NumDataPacketsBuffered", &self.NumDataPacketsBuffered)
            .field("TotalSelectiveNaksSent", &self.TotalSelectiveNaksSent)
            .field("TotalParityNaksSent", &self.TotalParityNaksSent)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RM_RECEIVER_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.NumODataPacketsReceived == other.NumODataPacketsReceived
            && self.NumRDataPacketsReceived == other.NumRDataPacketsReceived
            && self.NumDuplicateDataPackets == other.NumDuplicateDataPackets
            && self.DataBytesReceived == other.DataBytesReceived
            && self.TotalBytesReceived == other.TotalBytesReceived
            && self.RateKBitsPerSecOverall == other.RateKBitsPerSecOverall
            && self.RateKBitsPerSecLast == other.RateKBitsPerSecLast
            && self.TrailingEdgeSeqId == other.TrailingEdgeSeqId
            && self.LeadingEdgeSeqId == other.LeadingEdgeSeqId
            && self.AverageSequencesInWindow == other.AverageSequencesInWindow
            && self.MinSequencesInWindow == other.MinSequencesInWindow
            && self.MaxSequencesInWindow == other.MaxSequencesInWindow
            && self.FirstNakSequenceNumber == other.FirstNakSequenceNumber
            && self.NumPendingNaks == other.NumPendingNaks
            && self.NumOutstandingNaks == other.NumOutstandingNaks
            && self.NumDataPacketsBuffered == other.NumDataPacketsBuffered
            && self.TotalSelectiveNaksSent == other.TotalSelectiveNaksSent
            && self.TotalParityNaksSent == other.TotalParityNaksSent
    }
}
impl ::core::cmp::Eq for RM_RECEIVER_STATS {}
impl FromIntoMemory for RM_RECEIVER_STATS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 144);
        let f_NumODataPacketsReceived = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_NumRDataPacketsReceived = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_NumDuplicateDataPackets = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_DataBytesReceived = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_TotalBytesReceived = <u64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_RateKBitsPerSecOverall = <u64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        let f_RateKBitsPerSecLast = <u64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_TrailingEdgeSeqId = <u64 as FromIntoMemory>::from_bytes(&from[56..56 + 8]);
        let f_LeadingEdgeSeqId = <u64 as FromIntoMemory>::from_bytes(&from[64..64 + 8]);
        let f_AverageSequencesInWindow = <u64 as FromIntoMemory>::from_bytes(&from[72..72 + 8]);
        let f_MinSequencesInWindow = <u64 as FromIntoMemory>::from_bytes(&from[80..80 + 8]);
        let f_MaxSequencesInWindow = <u64 as FromIntoMemory>::from_bytes(&from[88..88 + 8]);
        let f_FirstNakSequenceNumber = <u64 as FromIntoMemory>::from_bytes(&from[96..96 + 8]);
        let f_NumPendingNaks = <u64 as FromIntoMemory>::from_bytes(&from[104..104 + 8]);
        let f_NumOutstandingNaks = <u64 as FromIntoMemory>::from_bytes(&from[112..112 + 8]);
        let f_NumDataPacketsBuffered = <u64 as FromIntoMemory>::from_bytes(&from[120..120 + 8]);
        let f_TotalSelectiveNaksSent = <u64 as FromIntoMemory>::from_bytes(&from[128..128 + 8]);
        let f_TotalParityNaksSent = <u64 as FromIntoMemory>::from_bytes(&from[136..136 + 8]);
        Self {
            NumODataPacketsReceived: f_NumODataPacketsReceived,
            NumRDataPacketsReceived: f_NumRDataPacketsReceived,
            NumDuplicateDataPackets: f_NumDuplicateDataPackets,
            DataBytesReceived: f_DataBytesReceived,
            TotalBytesReceived: f_TotalBytesReceived,
            RateKBitsPerSecOverall: f_RateKBitsPerSecOverall,
            RateKBitsPerSecLast: f_RateKBitsPerSecLast,
            TrailingEdgeSeqId: f_TrailingEdgeSeqId,
            LeadingEdgeSeqId: f_LeadingEdgeSeqId,
            AverageSequencesInWindow: f_AverageSequencesInWindow,
            MinSequencesInWindow: f_MinSequencesInWindow,
            MaxSequencesInWindow: f_MaxSequencesInWindow,
            FirstNakSequenceNumber: f_FirstNakSequenceNumber,
            NumPendingNaks: f_NumPendingNaks,
            NumOutstandingNaks: f_NumOutstandingNaks,
            NumDataPacketsBuffered: f_NumDataPacketsBuffered,
            TotalSelectiveNaksSent: f_TotalSelectiveNaksSent,
            TotalParityNaksSent: f_TotalParityNaksSent,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 144);
        FromIntoMemory::into_bytes(self.NumODataPacketsReceived, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.NumRDataPacketsReceived, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.NumDuplicateDataPackets, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.DataBytesReceived, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.TotalBytesReceived, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.RateKBitsPerSecOverall, &mut into[40..40 + 8]);
        FromIntoMemory::into_bytes(self.RateKBitsPerSecLast, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.TrailingEdgeSeqId, &mut into[56..56 + 8]);
        FromIntoMemory::into_bytes(self.LeadingEdgeSeqId, &mut into[64..64 + 8]);
        FromIntoMemory::into_bytes(self.AverageSequencesInWindow, &mut into[72..72 + 8]);
        FromIntoMemory::into_bytes(self.MinSequencesInWindow, &mut into[80..80 + 8]);
        FromIntoMemory::into_bytes(self.MaxSequencesInWindow, &mut into[88..88 + 8]);
        FromIntoMemory::into_bytes(self.FirstNakSequenceNumber, &mut into[96..96 + 8]);
        FromIntoMemory::into_bytes(self.NumPendingNaks, &mut into[104..104 + 8]);
        FromIntoMemory::into_bytes(self.NumOutstandingNaks, &mut into[112..112 + 8]);
        FromIntoMemory::into_bytes(self.NumDataPacketsBuffered, &mut into[120..120 + 8]);
        FromIntoMemory::into_bytes(self.TotalSelectiveNaksSent, &mut into[128..128 + 8]);
        FromIntoMemory::into_bytes(self.TotalParityNaksSent, &mut into[136..136 + 8]);
    }
    fn size() -> usize {
        144
    }
}
pub const RM_SENDER_STATISTICS: u32 = 1005u32;
pub struct RM_SENDER_STATS {
    pub DataBytesSent: u64,
    pub TotalBytesSent: u64,
    pub NaksReceived: u64,
    pub NaksReceivedTooLate: u64,
    pub NumOutstandingNaks: u64,
    pub NumNaksAfterRData: u64,
    pub RepairPacketsSent: u64,
    pub BufferSpaceAvailable: u64,
    pub TrailingEdgeSeqId: u64,
    pub LeadingEdgeSeqId: u64,
    pub RateKBitsPerSecOverall: u64,
    pub RateKBitsPerSecLast: u64,
    pub TotalODataPacketsSent: u64,
}
impl ::core::marker::Copy for RM_SENDER_STATS {}
impl ::core::clone::Clone for RM_SENDER_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RM_SENDER_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RM_SENDER_STATS")
            .field("DataBytesSent", &self.DataBytesSent)
            .field("TotalBytesSent", &self.TotalBytesSent)
            .field("NaksReceived", &self.NaksReceived)
            .field("NaksReceivedTooLate", &self.NaksReceivedTooLate)
            .field("NumOutstandingNaks", &self.NumOutstandingNaks)
            .field("NumNaksAfterRData", &self.NumNaksAfterRData)
            .field("RepairPacketsSent", &self.RepairPacketsSent)
            .field("BufferSpaceAvailable", &self.BufferSpaceAvailable)
            .field("TrailingEdgeSeqId", &self.TrailingEdgeSeqId)
            .field("LeadingEdgeSeqId", &self.LeadingEdgeSeqId)
            .field("RateKBitsPerSecOverall", &self.RateKBitsPerSecOverall)
            .field("RateKBitsPerSecLast", &self.RateKBitsPerSecLast)
            .field("TotalODataPacketsSent", &self.TotalODataPacketsSent)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RM_SENDER_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.DataBytesSent == other.DataBytesSent
            && self.TotalBytesSent == other.TotalBytesSent
            && self.NaksReceived == other.NaksReceived
            && self.NaksReceivedTooLate == other.NaksReceivedTooLate
            && self.NumOutstandingNaks == other.NumOutstandingNaks
            && self.NumNaksAfterRData == other.NumNaksAfterRData
            && self.RepairPacketsSent == other.RepairPacketsSent
            && self.BufferSpaceAvailable == other.BufferSpaceAvailable
            && self.TrailingEdgeSeqId == other.TrailingEdgeSeqId
            && self.LeadingEdgeSeqId == other.LeadingEdgeSeqId
            && self.RateKBitsPerSecOverall == other.RateKBitsPerSecOverall
            && self.RateKBitsPerSecLast == other.RateKBitsPerSecLast
            && self.TotalODataPacketsSent == other.TotalODataPacketsSent
    }
}
impl ::core::cmp::Eq for RM_SENDER_STATS {}
impl FromIntoMemory for RM_SENDER_STATS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 104);
        let f_DataBytesSent = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_TotalBytesSent = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_NaksReceived = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_NaksReceivedTooLate = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_NumOutstandingNaks = <u64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_NumNaksAfterRData = <u64 as FromIntoMemory>::from_bytes(&from[40..40 + 8]);
        let f_RepairPacketsSent = <u64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_BufferSpaceAvailable = <u64 as FromIntoMemory>::from_bytes(&from[56..56 + 8]);
        let f_TrailingEdgeSeqId = <u64 as FromIntoMemory>::from_bytes(&from[64..64 + 8]);
        let f_LeadingEdgeSeqId = <u64 as FromIntoMemory>::from_bytes(&from[72..72 + 8]);
        let f_RateKBitsPerSecOverall = <u64 as FromIntoMemory>::from_bytes(&from[80..80 + 8]);
        let f_RateKBitsPerSecLast = <u64 as FromIntoMemory>::from_bytes(&from[88..88 + 8]);
        let f_TotalODataPacketsSent = <u64 as FromIntoMemory>::from_bytes(&from[96..96 + 8]);
        Self {
            DataBytesSent: f_DataBytesSent,
            TotalBytesSent: f_TotalBytesSent,
            NaksReceived: f_NaksReceived,
            NaksReceivedTooLate: f_NaksReceivedTooLate,
            NumOutstandingNaks: f_NumOutstandingNaks,
            NumNaksAfterRData: f_NumNaksAfterRData,
            RepairPacketsSent: f_RepairPacketsSent,
            BufferSpaceAvailable: f_BufferSpaceAvailable,
            TrailingEdgeSeqId: f_TrailingEdgeSeqId,
            LeadingEdgeSeqId: f_LeadingEdgeSeqId,
            RateKBitsPerSecOverall: f_RateKBitsPerSecOverall,
            RateKBitsPerSecLast: f_RateKBitsPerSecLast,
            TotalODataPacketsSent: f_TotalODataPacketsSent,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 104);
        FromIntoMemory::into_bytes(self.DataBytesSent, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.TotalBytesSent, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.NaksReceived, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.NaksReceivedTooLate, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.NumOutstandingNaks, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.NumNaksAfterRData, &mut into[40..40 + 8]);
        FromIntoMemory::into_bytes(self.RepairPacketsSent, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.BufferSpaceAvailable, &mut into[56..56 + 8]);
        FromIntoMemory::into_bytes(self.TrailingEdgeSeqId, &mut into[64..64 + 8]);
        FromIntoMemory::into_bytes(self.LeadingEdgeSeqId, &mut into[72..72 + 8]);
        FromIntoMemory::into_bytes(self.RateKBitsPerSecOverall, &mut into[80..80 + 8]);
        FromIntoMemory::into_bytes(self.RateKBitsPerSecLast, &mut into[88..88 + 8]);
        FromIntoMemory::into_bytes(self.TotalODataPacketsSent, &mut into[96..96 + 8]);
    }
    fn size() -> usize {
        104
    }
}
pub const RM_SENDER_WINDOW_ADVANCE_METHOD: u32 = 1004u32;
pub struct RM_SEND_WINDOW {
    pub RateKbitsPerSec: u32,
    pub WindowSizeInMSecs: u32,
    pub WindowSizeInBytes: u32,
}
impl ::core::marker::Copy for RM_SEND_WINDOW {}
impl ::core::clone::Clone for RM_SEND_WINDOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RM_SEND_WINDOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RM_SEND_WINDOW")
            .field("RateKbitsPerSec", &self.RateKbitsPerSec)
            .field("WindowSizeInMSecs", &self.WindowSizeInMSecs)
            .field("WindowSizeInBytes", &self.WindowSizeInBytes)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RM_SEND_WINDOW {
    fn eq(&self, other: &Self) -> bool {
        self.RateKbitsPerSec == other.RateKbitsPerSec
            && self.WindowSizeInMSecs == other.WindowSizeInMSecs
            && self.WindowSizeInBytes == other.WindowSizeInBytes
    }
}
impl ::core::cmp::Eq for RM_SEND_WINDOW {}
impl FromIntoMemory for RM_SEND_WINDOW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_RateKbitsPerSec = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_WindowSizeInMSecs = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_WindowSizeInBytes = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            RateKbitsPerSec: f_RateKbitsPerSec,
            WindowSizeInMSecs: f_WindowSizeInMSecs,
            WindowSizeInBytes: f_WindowSizeInBytes,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.RateKbitsPerSec, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.WindowSizeInMSecs, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.WindowSizeInBytes, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const RM_SEND_WINDOW_ADV_RATE: u32 = 1010u32;
pub const RM_SET_MCAST_TTL: u32 = 1012u32;
pub const RM_SET_MESSAGE_BOUNDARY: u32 = 1002u32;
pub const RM_SET_SEND_IF: u32 = 1007u32;
pub const RM_USE_FEC: u32 = 1011u32;
pub struct RSS_SCALABILITY_INFO {
    pub RssEnabled: super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for RSS_SCALABILITY_INFO {}
impl ::core::clone::Clone for RSS_SCALABILITY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RSS_SCALABILITY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSS_SCALABILITY_INFO")
            .field("RssEnabled", &self.RssEnabled)
            .finish()
    }
}
impl ::core::cmp::PartialEq for RSS_SCALABILITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.RssEnabled == other.RssEnabled
    }
}
impl ::core::cmp::Eq for RSS_SCALABILITY_INFO {}
impl FromIntoMemory for RSS_SCALABILITY_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 1);
        let f_RssEnabled =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        Self {
            RssEnabled: f_RssEnabled,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 1);
        FromIntoMemory::into_bytes(self.RssEnabled, &mut into[0..0 + 1]);
    }
    fn size() -> usize {
        1
    }
}
pub const SAP_FIELD_ABSENT: u32 = 4294967294u32;
pub const SAP_FIELD_ANY: u32 = 4294967295u32;
pub const SAP_FIELD_ANY_AESA_REST: u32 = 4294967291u32;
pub const SAP_FIELD_ANY_AESA_SEL: u32 = 4294967290u32;
pub struct SCOPE_ID {
    pub Anonymous: SCOPE_ID_0,
}
impl ::core::marker::Copy for SCOPE_ID {}
impl ::core::clone::Clone for SCOPE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCOPE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_ID")
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCOPE_ID {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for SCOPE_ID {}
impl FromIntoMemory for SCOPE_ID {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Anonymous = <SCOPE_ID_0 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct SCOPE_ID_0 {
    data: [u8; 4],
}
impl ::core::default::Default for SCOPE_ID_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for SCOPE_ID_0 {}
impl ::core::clone::Clone for SCOPE_ID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCOPE_ID_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_ID_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCOPE_ID_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for SCOPE_ID_0 {}
impl FromIntoMemory for SCOPE_ID_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 4];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        4
    }
}
pub struct SCOPE_ID_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for SCOPE_ID_0_0 {}
impl ::core::clone::Clone for SCOPE_ID_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCOPE_ID_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPE_ID_0_0")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SCOPE_ID_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SCOPE_ID_0_0 {}
impl FromIntoMemory for SCOPE_ID_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f__bitfield = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            _bitfield: f__bitfield,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self._bitfield, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCOPE_LEVEL(pub i32);
pub const ScopeLevelInterface: SCOPE_LEVEL = SCOPE_LEVEL(1i32);
pub const ScopeLevelLink: SCOPE_LEVEL = SCOPE_LEVEL(2i32);
pub const ScopeLevelSubnet: SCOPE_LEVEL = SCOPE_LEVEL(3i32);
pub const ScopeLevelAdmin: SCOPE_LEVEL = SCOPE_LEVEL(4i32);
pub const ScopeLevelSite: SCOPE_LEVEL = SCOPE_LEVEL(5i32);
pub const ScopeLevelOrganization: SCOPE_LEVEL = SCOPE_LEVEL(8i32);
pub const ScopeLevelGlobal: SCOPE_LEVEL = SCOPE_LEVEL(14i32);
pub const ScopeLevelCount: SCOPE_LEVEL = SCOPE_LEVEL(16i32);
impl ::core::marker::Copy for SCOPE_LEVEL {}
impl ::core::clone::Clone for SCOPE_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCOPE_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCOPE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCOPE_LEVEL").field(&self.0).finish()
    }
}
impl FromIntoMemory for SCOPE_LEVEL {
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
pub const SD_BOTH: u32 = 2u32;
pub const SD_RECEIVE: u32 = 0u32;
pub const SD_SEND: u32 = 1u32;
pub const SECURITY_PROTOCOL_NONE: u32 = 0u32;
pub const SENDER_DEFAULT_LATE_JOINER_PERCENTAGE: u32 = 0u32;
pub const SENDER_DEFAULT_RATE_KBITS_PER_SEC: u32 = 56u32;
pub const SENDER_DEFAULT_WINDOW_ADV_PERCENTAGE: u32 = 15u32;
pub const SENDER_MAX_LATE_JOINER_PERCENTAGE: u32 = 75u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SEND_FLAGS(pub u32);
pub const MSG_DONTROUTE: SEND_FLAGS = SEND_FLAGS(4u32);
pub const MSG_OOB: SEND_FLAGS = SEND_FLAGS(1u32);
impl ::core::marker::Copy for SEND_FLAGS {}
impl ::core::clone::Clone for SEND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SEND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SEND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEND_FLAGS").field(&self.0).finish()
    }
}
impl FromIntoMemory for SEND_FLAGS {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub struct SERVICE_ADDRESS {
    pub dwAddressType: u32,
    pub dwAddressFlags: u32,
    pub dwAddressLength: u32,
    pub dwPrincipalLength: u32,
    pub lpAddress: MutPtr<u8>,
    pub lpPrincipal: MutPtr<u8>,
}
impl ::core::marker::Copy for SERVICE_ADDRESS {}
impl ::core::clone::Clone for SERVICE_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_ADDRESS")
            .field("dwAddressType", &self.dwAddressType)
            .field("dwAddressFlags", &self.dwAddressFlags)
            .field("dwAddressLength", &self.dwAddressLength)
            .field("dwPrincipalLength", &self.dwPrincipalLength)
            .field("lpAddress", &self.lpAddress)
            .field("lpPrincipal", &self.lpPrincipal)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SERVICE_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.dwAddressType == other.dwAddressType
            && self.dwAddressFlags == other.dwAddressFlags
            && self.dwAddressLength == other.dwAddressLength
            && self.dwPrincipalLength == other.dwPrincipalLength
            && self.lpAddress == other.lpAddress
            && self.lpPrincipal == other.lpPrincipal
    }
}
impl ::core::cmp::Eq for SERVICE_ADDRESS {}
impl FromIntoMemory for SERVICE_ADDRESS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_dwAddressType = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwAddressFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwAddressLength = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwPrincipalLength = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpAddress = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpPrincipal = <MutPtr<u8> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            dwAddressType: f_dwAddressType,
            dwAddressFlags: f_dwAddressFlags,
            dwAddressLength: f_dwAddressLength,
            dwPrincipalLength: f_dwPrincipalLength,
            lpAddress: f_lpAddress,
            lpPrincipal: f_lpPrincipal,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.dwAddressType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwAddressFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwAddressLength, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwPrincipalLength, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpAddress, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpPrincipal, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct SERVICE_ADDRESSES {
    pub dwAddressCount: u32,
    pub Addresses: [SERVICE_ADDRESS; 1],
}
impl ::core::marker::Copy for SERVICE_ADDRESSES {}
impl ::core::clone::Clone for SERVICE_ADDRESSES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_ADDRESSES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_ADDRESSES")
            .field("dwAddressCount", &self.dwAddressCount)
            .field("Addresses", &self.Addresses)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SERVICE_ADDRESSES {
    fn eq(&self, other: &Self) -> bool {
        self.dwAddressCount == other.dwAddressCount && self.Addresses == other.Addresses
    }
}
impl ::core::cmp::Eq for SERVICE_ADDRESSES {}
impl FromIntoMemory for SERVICE_ADDRESSES {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_dwAddressCount = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Addresses = <[SERVICE_ADDRESS; 1] as FromIntoMemory>::from_bytes(&from[4..4 + 24]);
        Self {
            dwAddressCount: f_dwAddressCount,
            Addresses: f_Addresses,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.dwAddressCount, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Addresses, &mut into[4..4 + 24]);
    }
    fn size() -> usize {
        28
    }
}
pub const SERVICE_ADDRESS_FLAG_RPC_CN: u32 = 1u32;
pub const SERVICE_ADDRESS_FLAG_RPC_DG: u32 = 2u32;
pub const SERVICE_ADDRESS_FLAG_RPC_NB: u32 = 4u32;
pub struct SERVICE_ASYNC_INFO {
    pub lpServiceCallbackProc: LPSERVICE_CALLBACK_PROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub hAsyncTaskHandle: super::super::Foundation::HANDLE,
}
impl ::core::marker::Copy for SERVICE_ASYNC_INFO {}
impl ::core::clone::Clone for SERVICE_ASYNC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_ASYNC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_ASYNC_INFO")
            .field("lpServiceCallbackProc", &self.lpServiceCallbackProc)
            .field("lParam", &self.lParam)
            .field("hAsyncTaskHandle", &self.hAsyncTaskHandle)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SERVICE_ASYNC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceCallbackProc == other.lpServiceCallbackProc
            && self.lParam == other.lParam
            && self.hAsyncTaskHandle == other.hAsyncTaskHandle
    }
}
impl ::core::cmp::Eq for SERVICE_ASYNC_INFO {}
impl FromIntoMemory for SERVICE_ASYNC_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_lpServiceCallbackProc =
            <LPSERVICE_CALLBACK_PROC as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lParam =
            <super::super::Foundation::LPARAM as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_hAsyncTaskHandle =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            lpServiceCallbackProc: f_lpServiceCallbackProc,
            lParam: f_lParam,
            hAsyncTaskHandle: f_hAsyncTaskHandle,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.lpServiceCallbackProc, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lParam, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.hAsyncTaskHandle, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub const SERVICE_FLAG_DEFER: u32 = 1u32;
pub const SERVICE_FLAG_HARD: u32 = 2u32;
pub struct SERVICE_INFOA {
    pub lpServiceType: MutPtr<crate::core::GUID>,
    pub lpServiceName: PSTR,
    pub lpComment: PSTR,
    pub lpLocale: PSTR,
    pub dwDisplayHint: RESOURCE_DISPLAY_TYPE,
    pub dwVersion: u32,
    pub dwTime: u32,
    pub lpMachineName: PSTR,
    pub lpServiceAddress: MutPtr<SERVICE_ADDRESSES>,
    pub ServiceSpecificInfo: super::super::System::Com::BLOB,
}
impl ::core::marker::Copy for SERVICE_INFOA {}
impl ::core::clone::Clone for SERVICE_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_INFOA")
            .field("lpServiceType", &self.lpServiceType)
            .field("lpServiceName", &self.lpServiceName)
            .field("lpComment", &self.lpComment)
            .field("lpLocale", &self.lpLocale)
            .field("dwDisplayHint", &self.dwDisplayHint)
            .field("dwVersion", &self.dwVersion)
            .field("dwTime", &self.dwTime)
            .field("lpMachineName", &self.lpMachineName)
            .field("lpServiceAddress", &self.lpServiceAddress)
            .field("ServiceSpecificInfo", &self.ServiceSpecificInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SERVICE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceType == other.lpServiceType
            && self.lpServiceName == other.lpServiceName
            && self.lpComment == other.lpComment
            && self.lpLocale == other.lpLocale
            && self.dwDisplayHint == other.dwDisplayHint
            && self.dwVersion == other.dwVersion
            && self.dwTime == other.dwTime
            && self.lpMachineName == other.lpMachineName
            && self.lpServiceAddress == other.lpServiceAddress
            && self.ServiceSpecificInfo == other.ServiceSpecificInfo
    }
}
impl ::core::cmp::Eq for SERVICE_INFOA {}
impl FromIntoMemory for SERVICE_INFOA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_lpServiceType =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpServiceName = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpComment = <PSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpLocale = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwDisplayHint =
            <RESOURCE_DISPLAY_TYPE as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwTime = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lpMachineName = <PSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_lpServiceAddress =
            <MutPtr<SERVICE_ADDRESSES> as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_ServiceSpecificInfo =
            <super::super::System::Com::BLOB as FromIntoMemory>::from_bytes(&from[36..36 + 8]);
        Self {
            lpServiceType: f_lpServiceType,
            lpServiceName: f_lpServiceName,
            lpComment: f_lpComment,
            lpLocale: f_lpLocale,
            dwDisplayHint: f_dwDisplayHint,
            dwVersion: f_dwVersion,
            dwTime: f_dwTime,
            lpMachineName: f_lpMachineName,
            lpServiceAddress: f_lpServiceAddress,
            ServiceSpecificInfo: f_ServiceSpecificInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.lpServiceType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpServiceName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpComment, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpLocale, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwDisplayHint, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwTime, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpMachineName, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.lpServiceAddress, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.ServiceSpecificInfo, &mut into[36..36 + 8]);
    }
    fn size() -> usize {
        44
    }
}
pub struct SERVICE_INFOW {
    pub lpServiceType: MutPtr<crate::core::GUID>,
    pub lpServiceName: PWSTR,
    pub lpComment: PWSTR,
    pub lpLocale: PWSTR,
    pub dwDisplayHint: RESOURCE_DISPLAY_TYPE,
    pub dwVersion: u32,
    pub dwTime: u32,
    pub lpMachineName: PWSTR,
    pub lpServiceAddress: MutPtr<SERVICE_ADDRESSES>,
    pub ServiceSpecificInfo: super::super::System::Com::BLOB,
}
impl ::core::marker::Copy for SERVICE_INFOW {}
impl ::core::clone::Clone for SERVICE_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_INFOW")
            .field("lpServiceType", &self.lpServiceType)
            .field("lpServiceName", &self.lpServiceName)
            .field("lpComment", &self.lpComment)
            .field("lpLocale", &self.lpLocale)
            .field("dwDisplayHint", &self.dwDisplayHint)
            .field("dwVersion", &self.dwVersion)
            .field("dwTime", &self.dwTime)
            .field("lpMachineName", &self.lpMachineName)
            .field("lpServiceAddress", &self.lpServiceAddress)
            .field("ServiceSpecificInfo", &self.ServiceSpecificInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SERVICE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceType == other.lpServiceType
            && self.lpServiceName == other.lpServiceName
            && self.lpComment == other.lpComment
            && self.lpLocale == other.lpLocale
            && self.dwDisplayHint == other.dwDisplayHint
            && self.dwVersion == other.dwVersion
            && self.dwTime == other.dwTime
            && self.lpMachineName == other.lpMachineName
            && self.lpServiceAddress == other.lpServiceAddress
            && self.ServiceSpecificInfo == other.ServiceSpecificInfo
    }
}
impl ::core::cmp::Eq for SERVICE_INFOW {}
impl FromIntoMemory for SERVICE_INFOW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_lpServiceType =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpServiceName = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpComment = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpLocale = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwDisplayHint =
            <RESOURCE_DISPLAY_TYPE as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwTime = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lpMachineName = <PWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_lpServiceAddress =
            <MutPtr<SERVICE_ADDRESSES> as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_ServiceSpecificInfo =
            <super::super::System::Com::BLOB as FromIntoMemory>::from_bytes(&from[36..36 + 8]);
        Self {
            lpServiceType: f_lpServiceType,
            lpServiceName: f_lpServiceName,
            lpComment: f_lpComment,
            lpLocale: f_lpLocale,
            dwDisplayHint: f_dwDisplayHint,
            dwVersion: f_dwVersion,
            dwTime: f_dwTime,
            lpMachineName: f_lpMachineName,
            lpServiceAddress: f_lpServiceAddress,
            ServiceSpecificInfo: f_ServiceSpecificInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.lpServiceType, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpServiceName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpComment, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpLocale, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwDisplayHint, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwTime, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpMachineName, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.lpServiceAddress, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.ServiceSpecificInfo, &mut into[36..36 + 8]);
    }
    fn size() -> usize {
        44
    }
}
pub const SERVICE_LOCAL: u32 = 4u32;
pub const SERVICE_MULTIPLE: u32 = 1u32;
pub const SERVICE_RESOURCE: u32 = 1u32;
pub const SERVICE_SERVICE: u32 = 2u32;
pub struct SERVICE_TYPE_INFO {
    pub dwTypeNameOffset: u32,
    pub dwValueCount: u32,
    pub Values: [SERVICE_TYPE_VALUE; 1],
}
impl ::core::marker::Copy for SERVICE_TYPE_INFO {}
impl ::core::clone::Clone for SERVICE_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_TYPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TYPE_INFO")
            .field("dwTypeNameOffset", &self.dwTypeNameOffset)
            .field("dwValueCount", &self.dwValueCount)
            .field("Values", &self.Values)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SERVICE_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwTypeNameOffset == other.dwTypeNameOffset
            && self.dwValueCount == other.dwValueCount
            && self.Values == other.Values
    }
}
impl ::core::cmp::Eq for SERVICE_TYPE_INFO {}
impl FromIntoMemory for SERVICE_TYPE_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_dwTypeNameOffset = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwValueCount = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Values = <[SERVICE_TYPE_VALUE; 1] as FromIntoMemory>::from_bytes(&from[8..8 + 20]);
        Self {
            dwTypeNameOffset: f_dwTypeNameOffset,
            dwValueCount: f_dwValueCount,
            Values: f_Values,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.dwTypeNameOffset, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwValueCount, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Values, &mut into[8..8 + 20]);
    }
    fn size() -> usize {
        28
    }
}
pub struct SERVICE_TYPE_INFO_ABSA {
    pub lpTypeName: PSTR,
    pub dwValueCount: u32,
    pub Values: [SERVICE_TYPE_VALUE_ABSA; 1],
}
impl ::core::marker::Copy for SERVICE_TYPE_INFO_ABSA {}
impl ::core::clone::Clone for SERVICE_TYPE_INFO_ABSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_TYPE_INFO_ABSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TYPE_INFO_ABSA")
            .field("lpTypeName", &self.lpTypeName)
            .field("dwValueCount", &self.dwValueCount)
            .field("Values", &self.Values)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SERVICE_TYPE_INFO_ABSA {
    fn eq(&self, other: &Self) -> bool {
        self.lpTypeName == other.lpTypeName
            && self.dwValueCount == other.dwValueCount
            && self.Values == other.Values
    }
}
impl ::core::cmp::Eq for SERVICE_TYPE_INFO_ABSA {}
impl FromIntoMemory for SERVICE_TYPE_INFO_ABSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_lpTypeName = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwValueCount = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Values =
            <[SERVICE_TYPE_VALUE_ABSA; 1] as FromIntoMemory>::from_bytes(&from[8..8 + 20]);
        Self {
            lpTypeName: f_lpTypeName,
            dwValueCount: f_dwValueCount,
            Values: f_Values,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.lpTypeName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwValueCount, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Values, &mut into[8..8 + 20]);
    }
    fn size() -> usize {
        28
    }
}
pub struct SERVICE_TYPE_INFO_ABSW {
    pub lpTypeName: PWSTR,
    pub dwValueCount: u32,
    pub Values: [SERVICE_TYPE_VALUE_ABSW; 1],
}
impl ::core::marker::Copy for SERVICE_TYPE_INFO_ABSW {}
impl ::core::clone::Clone for SERVICE_TYPE_INFO_ABSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_TYPE_INFO_ABSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TYPE_INFO_ABSW")
            .field("lpTypeName", &self.lpTypeName)
            .field("dwValueCount", &self.dwValueCount)
            .field("Values", &self.Values)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SERVICE_TYPE_INFO_ABSW {
    fn eq(&self, other: &Self) -> bool {
        self.lpTypeName == other.lpTypeName
            && self.dwValueCount == other.dwValueCount
            && self.Values == other.Values
    }
}
impl ::core::cmp::Eq for SERVICE_TYPE_INFO_ABSW {}
impl FromIntoMemory for SERVICE_TYPE_INFO_ABSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_lpTypeName = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwValueCount = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Values =
            <[SERVICE_TYPE_VALUE_ABSW; 1] as FromIntoMemory>::from_bytes(&from[8..8 + 20]);
        Self {
            lpTypeName: f_lpTypeName,
            dwValueCount: f_dwValueCount,
            Values: f_Values,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.lpTypeName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwValueCount, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Values, &mut into[8..8 + 20]);
    }
    fn size() -> usize {
        28
    }
}
pub struct SERVICE_TYPE_VALUE {
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub dwValueNameOffset: u32,
    pub dwValueOffset: u32,
}
impl ::core::marker::Copy for SERVICE_TYPE_VALUE {}
impl ::core::clone::Clone for SERVICE_TYPE_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_TYPE_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TYPE_VALUE")
            .field("dwNameSpace", &self.dwNameSpace)
            .field("dwValueType", &self.dwValueType)
            .field("dwValueSize", &self.dwValueSize)
            .field("dwValueNameOffset", &self.dwValueNameOffset)
            .field("dwValueOffset", &self.dwValueOffset)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SERVICE_TYPE_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace
            && self.dwValueType == other.dwValueType
            && self.dwValueSize == other.dwValueSize
            && self.dwValueNameOffset == other.dwValueNameOffset
            && self.dwValueOffset == other.dwValueOffset
    }
}
impl ::core::cmp::Eq for SERVICE_TYPE_VALUE {}
impl FromIntoMemory for SERVICE_TYPE_VALUE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwValueType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwValueSize = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwValueNameOffset = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwValueOffset = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            dwNameSpace: f_dwNameSpace,
            dwValueType: f_dwValueType,
            dwValueSize: f_dwValueSize,
            dwValueNameOffset: f_dwValueNameOffset,
            dwValueOffset: f_dwValueOffset,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwValueType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwValueSize, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwValueNameOffset, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwValueOffset, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct SERVICE_TYPE_VALUE_ABSA {
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub lpValueName: PSTR,
    pub lpValue: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for SERVICE_TYPE_VALUE_ABSA {}
impl ::core::clone::Clone for SERVICE_TYPE_VALUE_ABSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_TYPE_VALUE_ABSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TYPE_VALUE_ABSA")
            .field("dwNameSpace", &self.dwNameSpace)
            .field("dwValueType", &self.dwValueType)
            .field("dwValueSize", &self.dwValueSize)
            .field("lpValueName", &self.lpValueName)
            .field("lpValue", &self.lpValue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SERVICE_TYPE_VALUE_ABSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace
            && self.dwValueType == other.dwValueType
            && self.dwValueSize == other.dwValueSize
            && self.lpValueName == other.lpValueName
            && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for SERVICE_TYPE_VALUE_ABSA {}
impl FromIntoMemory for SERVICE_TYPE_VALUE_ABSA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwValueType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwValueSize = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpValueName = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpValue =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            dwNameSpace: f_dwNameSpace,
            dwValueType: f_dwValueType,
            dwValueSize: f_dwValueSize,
            lpValueName: f_lpValueName,
            lpValue: f_lpValue,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwValueType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwValueSize, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpValueName, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpValue, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct SERVICE_TYPE_VALUE_ABSW {
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub lpValueName: PWSTR,
    pub lpValue: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for SERVICE_TYPE_VALUE_ABSW {}
impl ::core::clone::Clone for SERVICE_TYPE_VALUE_ABSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_TYPE_VALUE_ABSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TYPE_VALUE_ABSW")
            .field("dwNameSpace", &self.dwNameSpace)
            .field("dwValueType", &self.dwValueType)
            .field("dwValueSize", &self.dwValueSize)
            .field("lpValueName", &self.lpValueName)
            .field("lpValue", &self.lpValue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SERVICE_TYPE_VALUE_ABSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwNameSpace == other.dwNameSpace
            && self.dwValueType == other.dwValueType
            && self.dwValueSize == other.dwValueSize
            && self.lpValueName == other.lpValueName
            && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for SERVICE_TYPE_VALUE_ABSW {}
impl FromIntoMemory for SERVICE_TYPE_VALUE_ABSW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwValueType = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwValueSize = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpValueName = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpValue =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            dwNameSpace: f_dwNameSpace,
            dwValueType: f_dwValueType,
            dwValueSize: f_dwValueSize,
            lpValueName: f_lpValueName,
            lpValue: f_lpValue,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwValueType, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwValueSize, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpValueName, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpValue, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub const SERVICE_TYPE_VALUE_CONN: &'static str = "ConnectionOriented";
pub const SERVICE_TYPE_VALUE_CONNA: &'static str = "ConnectionOriented";
pub const SERVICE_TYPE_VALUE_CONNW: &'static str = "ConnectionOriented";
pub const SERVICE_TYPE_VALUE_IPXPORTA: &'static str = "IpxSocket";
pub const SERVICE_TYPE_VALUE_IPXPORTW: &'static str = "IpxSocket";
pub const SERVICE_TYPE_VALUE_OBJECTID: &'static str = "ObjectId";
pub const SERVICE_TYPE_VALUE_OBJECTIDA: &'static str = "ObjectId";
pub const SERVICE_TYPE_VALUE_OBJECTIDW: &'static str = "ObjectId";
pub const SERVICE_TYPE_VALUE_SAPID: &'static str = "SapId";
pub const SERVICE_TYPE_VALUE_SAPIDA: &'static str = "SapId";
pub const SERVICE_TYPE_VALUE_SAPIDW: &'static str = "SapId";
pub const SERVICE_TYPE_VALUE_TCPPORT: &'static str = "TcpPort";
pub const SERVICE_TYPE_VALUE_TCPPORTA: &'static str = "TcpPort";
pub const SERVICE_TYPE_VALUE_TCPPORTW: &'static str = "TcpPort";
pub const SERVICE_TYPE_VALUE_UDPPORT: &'static str = "UdpPort";
pub const SERVICE_TYPE_VALUE_UDPPORTA: &'static str = "UdpPort";
pub const SERVICE_TYPE_VALUE_UDPPORTW: &'static str = "UdpPort";
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SET_SERVICE_OPERATION(pub u32);
pub const SERVICE_REGISTER: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(1u32);
pub const SERVICE_DEREGISTER: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(2u32);
pub const SERVICE_FLUSH: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(3u32);
pub const SERVICE_ADD_TYPE: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(4u32);
pub const SERVICE_DELETE_TYPE: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(5u32);
impl ::core::marker::Copy for SET_SERVICE_OPERATION {}
impl ::core::clone::Clone for SET_SERVICE_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SET_SERVICE_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SET_SERVICE_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SET_SERVICE_OPERATION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SET_SERVICE_OPERATION {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u32 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        4
    }
}
pub const SET_SERVICE_PARTIAL_SUCCESS: u32 = 1u32;
pub const SG_CONSTRAINED_GROUP: u32 = 2u32;
pub const SG_UNCONSTRAINED_GROUP: u32 = 1u32;
pub const SIOCATMARK: i32 = 1074033415i32;
pub const SIOCGHIWAT: i32 = 1074033409i32;
pub const SIOCGLOWAT: i32 = 1074033411i32;
pub const SIOCSHIWAT: i32 = -2147192064i32;
pub const SIOCSLOWAT: i32 = -2147192062i32;
pub const SIO_ASSOCIATE_PVC: u32 = 2417360899u32;
pub const SIO_GET_ATM_ADDRESS: u32 = 3491102722u32;
pub const SIO_GET_ATM_CONNECTION_ID: u32 = 1343619076u32;
pub const SIO_GET_NUMBER_OF_ATM_DEVICES: u32 = 1343619073u32;
pub const SI_NETWORK: u32 = 3u32;
pub const SI_USER_FAILED: u32 = 2u32;
pub const SI_USER_NOT_SCREENED: u32 = 0u32;
pub const SI_USER_PASSED: u32 = 1u32;
pub struct SOCKADDR {
    pub sa_family: u16,
    pub sa_data: [super::super::Foundation::CHAR; 14],
}
impl ::core::marker::Copy for SOCKADDR {}
impl ::core::clone::Clone for SOCKADDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKADDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR")
            .field("sa_family", &self.sa_family)
            .field("sa_data", &self.sa_data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKADDR {
    fn eq(&self, other: &Self) -> bool {
        self.sa_family == other.sa_family && self.sa_data == other.sa_data
    }
}
impl ::core::cmp::Eq for SOCKADDR {}
impl FromIntoMemory for SOCKADDR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_sa_family = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_sa_data =
            <[super::super::Foundation::CHAR; 14] as FromIntoMemory>::from_bytes(&from[2..2 + 14]);
        Self {
            sa_family: f_sa_family,
            sa_data: f_sa_data,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.sa_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.sa_data, &mut into[2..2 + 14]);
    }
    fn size() -> usize {
        16
    }
}
pub struct SOCKADDR_DL {
    pub sdl_family: u16,
    pub sdl_data: [u8; 8],
    pub sdl_zero: [u8; 4],
}
impl ::core::marker::Copy for SOCKADDR_DL {}
impl ::core::clone::Clone for SOCKADDR_DL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKADDR_DL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_DL")
            .field("sdl_family", &self.sdl_family)
            .field("sdl_data", &self.sdl_data)
            .field("sdl_zero", &self.sdl_zero)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_DL {
    fn eq(&self, other: &Self) -> bool {
        self.sdl_family == other.sdl_family
            && self.sdl_data == other.sdl_data
            && self.sdl_zero == other.sdl_zero
    }
}
impl ::core::cmp::Eq for SOCKADDR_DL {}
impl FromIntoMemory for SOCKADDR_DL {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 14);
        let f_sdl_family = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_sdl_data = <[u8; 8] as FromIntoMemory>::from_bytes(&from[2..2 + 8]);
        let f_sdl_zero = <[u8; 4] as FromIntoMemory>::from_bytes(&from[10..10 + 4]);
        Self {
            sdl_family: f_sdl_family,
            sdl_data: f_sdl_data,
            sdl_zero: f_sdl_zero,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 14);
        FromIntoMemory::into_bytes(self.sdl_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.sdl_data, &mut into[2..2 + 8]);
        FromIntoMemory::into_bytes(self.sdl_zero, &mut into[10..10 + 4]);
    }
    fn size() -> usize {
        14
    }
}
pub struct SOCKADDR_IN {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: IN_ADDR,
    pub sin_zero: [super::super::Foundation::CHAR; 8],
}
impl ::core::marker::Copy for SOCKADDR_IN {}
impl ::core::clone::Clone for SOCKADDR_IN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKADDR_IN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_IN")
            .field("sin_family", &self.sin_family)
            .field("sin_port", &self.sin_port)
            .field("sin_addr", &self.sin_addr)
            .field("sin_zero", &self.sin_zero)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_IN {
    fn eq(&self, other: &Self) -> bool {
        self.sin_family == other.sin_family
            && self.sin_port == other.sin_port
            && self.sin_addr == other.sin_addr
            && self.sin_zero == other.sin_zero
    }
}
impl ::core::cmp::Eq for SOCKADDR_IN {}
impl FromIntoMemory for SOCKADDR_IN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_sin_family = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_sin_port = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_sin_addr = <IN_ADDR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_sin_zero =
            <[super::super::Foundation::CHAR; 8] as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            sin_family: f_sin_family,
            sin_port: f_sin_port,
            sin_addr: f_sin_addr,
            sin_zero: f_sin_zero,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.sin_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.sin_port, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.sin_addr, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.sin_zero, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub struct SOCKADDR_IN6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: IN6_ADDR,
    pub Anonymous: SOCKADDR_IN6_0,
}
impl ::core::marker::Copy for SOCKADDR_IN6 {}
impl ::core::clone::Clone for SOCKADDR_IN6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKADDR_IN6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_IN6")
            .field("sin6_family", &self.sin6_family)
            .field("sin6_port", &self.sin6_port)
            .field("sin6_flowinfo", &self.sin6_flowinfo)
            .field("sin6_addr", &self.sin6_addr)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_IN6 {
    fn eq(&self, other: &Self) -> bool {
        self.sin6_family == other.sin6_family
            && self.sin6_port == other.sin6_port
            && self.sin6_flowinfo == other.sin6_flowinfo
            && self.sin6_addr == other.sin6_addr
            && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for SOCKADDR_IN6 {}
impl FromIntoMemory for SOCKADDR_IN6 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_sin6_family = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_sin6_port = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_sin6_flowinfo = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_sin6_addr = <IN6_ADDR as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_Anonymous = <SOCKADDR_IN6_0 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            sin6_family: f_sin6_family,
            sin6_port: f_sin6_port,
            sin6_flowinfo: f_sin6_flowinfo,
            sin6_addr: f_sin6_addr,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.sin6_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.sin6_port, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.sin6_flowinfo, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.sin6_addr, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct SOCKADDR_IN6_0 {
    data: [u8; 4],
}
impl ::core::default::Default for SOCKADDR_IN6_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for SOCKADDR_IN6_0 {}
impl ::core::clone::Clone for SOCKADDR_IN6_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKADDR_IN6_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_IN6_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_IN6_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for SOCKADDR_IN6_0 {}
impl FromIntoMemory for SOCKADDR_IN6_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 4];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        4
    }
}
pub struct SOCKADDR_IN6_PAIR {
    pub SourceAddress: MutPtr<SOCKADDR_IN6>,
    pub DestinationAddress: MutPtr<SOCKADDR_IN6>,
}
impl ::core::marker::Copy for SOCKADDR_IN6_PAIR {}
impl ::core::clone::Clone for SOCKADDR_IN6_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKADDR_IN6_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_IN6_PAIR")
            .field("SourceAddress", &self.SourceAddress)
            .field("DestinationAddress", &self.DestinationAddress)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_IN6_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.SourceAddress == other.SourceAddress
            && self.DestinationAddress == other.DestinationAddress
    }
}
impl ::core::cmp::Eq for SOCKADDR_IN6_PAIR {}
impl FromIntoMemory for SOCKADDR_IN6_PAIR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_SourceAddress = <MutPtr<SOCKADDR_IN6> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_DestinationAddress =
            <MutPtr<SOCKADDR_IN6> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            SourceAddress: f_SourceAddress,
            DestinationAddress: f_DestinationAddress,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.SourceAddress, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.DestinationAddress, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct SOCKADDR_IN6_W2KSP1 {
    pub sin6_family: i16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: IN6_ADDR,
    pub sin6_scope_id: u32,
}
impl ::core::marker::Copy for SOCKADDR_IN6_W2KSP1 {}
impl ::core::clone::Clone for SOCKADDR_IN6_W2KSP1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKADDR_IN6_W2KSP1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_IN6_W2KSP1")
            .field("sin6_family", &self.sin6_family)
            .field("sin6_port", &self.sin6_port)
            .field("sin6_flowinfo", &self.sin6_flowinfo)
            .field("sin6_addr", &self.sin6_addr)
            .field("sin6_scope_id", &self.sin6_scope_id)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_IN6_W2KSP1 {
    fn eq(&self, other: &Self) -> bool {
        self.sin6_family == other.sin6_family
            && self.sin6_port == other.sin6_port
            && self.sin6_flowinfo == other.sin6_flowinfo
            && self.sin6_addr == other.sin6_addr
            && self.sin6_scope_id == other.sin6_scope_id
    }
}
impl ::core::cmp::Eq for SOCKADDR_IN6_W2KSP1 {}
impl FromIntoMemory for SOCKADDR_IN6_W2KSP1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_sin6_family = <i16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_sin6_port = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_sin6_flowinfo = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_sin6_addr = <IN6_ADDR as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        let f_sin6_scope_id = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            sin6_family: f_sin6_family,
            sin6_port: f_sin6_port,
            sin6_flowinfo: f_sin6_flowinfo,
            sin6_addr: f_sin6_addr,
            sin6_scope_id: f_sin6_scope_id,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.sin6_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.sin6_port, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.sin6_flowinfo, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.sin6_addr, &mut into[8..8 + 16]);
        FromIntoMemory::into_bytes(self.sin6_scope_id, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct SOCKADDR_INET {
    data: [u8; 4],
}
impl ::core::default::Default for SOCKADDR_INET {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for SOCKADDR_INET {}
impl ::core::clone::Clone for SOCKADDR_INET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKADDR_INET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_INET")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_INET {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for SOCKADDR_INET {}
impl FromIntoMemory for SOCKADDR_INET {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 4];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        4
    }
}
pub struct SOCKADDR_IRDA {
    pub irdaAddressFamily: u16,
    pub irdaDeviceID: [u8; 4],
    pub irdaServiceName: [super::super::Foundation::CHAR; 25],
}
impl ::core::marker::Copy for SOCKADDR_IRDA {}
impl ::core::clone::Clone for SOCKADDR_IRDA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKADDR_IRDA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_IRDA")
            .field("irdaAddressFamily", &self.irdaAddressFamily)
            .field("irdaDeviceID", &self.irdaDeviceID)
            .field("irdaServiceName", &self.irdaServiceName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_IRDA {
    fn eq(&self, other: &Self) -> bool {
        self.irdaAddressFamily == other.irdaAddressFamily
            && self.irdaDeviceID == other.irdaDeviceID
            && self.irdaServiceName == other.irdaServiceName
    }
}
impl ::core::cmp::Eq for SOCKADDR_IRDA {}
impl FromIntoMemory for SOCKADDR_IRDA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_irdaAddressFamily = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_irdaDeviceID = <[u8; 4] as FromIntoMemory>::from_bytes(&from[2..2 + 4]);
        let f_irdaServiceName =
            <[super::super::Foundation::CHAR; 25] as FromIntoMemory>::from_bytes(&from[6..6 + 25]);
        Self {
            irdaAddressFamily: f_irdaAddressFamily,
            irdaDeviceID: f_irdaDeviceID,
            irdaServiceName: f_irdaServiceName,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.irdaAddressFamily, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.irdaDeviceID, &mut into[2..2 + 4]);
        FromIntoMemory::into_bytes(self.irdaServiceName, &mut into[6..6 + 25]);
    }
    fn size() -> usize {
        32
    }
}
pub struct SOCKADDR_STORAGE {
    pub ss_family: u16,
    pub __ss_pad1: [super::super::Foundation::CHAR; 6],
    pub __ss_align: i64,
    pub __ss_pad2: [super::super::Foundation::CHAR; 112],
}
impl ::core::marker::Copy for SOCKADDR_STORAGE {}
impl ::core::clone::Clone for SOCKADDR_STORAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKADDR_STORAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_STORAGE")
            .field("ss_family", &self.ss_family)
            .field("__ss_pad1", &self.__ss_pad1)
            .field("__ss_align", &self.__ss_align)
            .field("__ss_pad2", &self.__ss_pad2)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_STORAGE {
    fn eq(&self, other: &Self) -> bool {
        self.ss_family == other.ss_family
            && self.__ss_pad1 == other.__ss_pad1
            && self.__ss_align == other.__ss_align
            && self.__ss_pad2 == other.__ss_pad2
    }
}
impl ::core::cmp::Eq for SOCKADDR_STORAGE {}
impl FromIntoMemory for SOCKADDR_STORAGE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 128);
        let f_ss_family = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f___ss_pad1 =
            <[super::super::Foundation::CHAR; 6] as FromIntoMemory>::from_bytes(&from[2..2 + 6]);
        let f___ss_align = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f___ss_pad2 = <[super::super::Foundation::CHAR; 112] as FromIntoMemory>::from_bytes(
            &from[16..16 + 112],
        );
        Self {
            ss_family: f_ss_family,
            __ss_pad1: f___ss_pad1,
            __ss_align: f___ss_align,
            __ss_pad2: f___ss_pad2,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 128);
        FromIntoMemory::into_bytes(self.ss_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.__ss_pad1, &mut into[2..2 + 6]);
        FromIntoMemory::into_bytes(self.__ss_align, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.__ss_pad2, &mut into[16..16 + 112]);
    }
    fn size() -> usize {
        128
    }
}
pub struct SOCKADDR_STORAGE_XP {
    pub ss_family: i16,
    pub __ss_pad1: [super::super::Foundation::CHAR; 6],
    pub __ss_align: i64,
    pub __ss_pad2: [super::super::Foundation::CHAR; 112],
}
impl ::core::marker::Copy for SOCKADDR_STORAGE_XP {}
impl ::core::clone::Clone for SOCKADDR_STORAGE_XP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKADDR_STORAGE_XP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_STORAGE_XP")
            .field("ss_family", &self.ss_family)
            .field("__ss_pad1", &self.__ss_pad1)
            .field("__ss_align", &self.__ss_align)
            .field("__ss_pad2", &self.__ss_pad2)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKADDR_STORAGE_XP {
    fn eq(&self, other: &Self) -> bool {
        self.ss_family == other.ss_family
            && self.__ss_pad1 == other.__ss_pad1
            && self.__ss_align == other.__ss_align
            && self.__ss_pad2 == other.__ss_pad2
    }
}
impl ::core::cmp::Eq for SOCKADDR_STORAGE_XP {}
impl FromIntoMemory for SOCKADDR_STORAGE_XP {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 128);
        let f_ss_family = <i16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f___ss_pad1 =
            <[super::super::Foundation::CHAR; 6] as FromIntoMemory>::from_bytes(&from[2..2 + 6]);
        let f___ss_align = <i64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f___ss_pad2 = <[super::super::Foundation::CHAR; 112] as FromIntoMemory>::from_bytes(
            &from[16..16 + 112],
        );
        Self {
            ss_family: f_ss_family,
            __ss_pad1: f___ss_pad1,
            __ss_align: f___ss_align,
            __ss_pad2: f___ss_pad2,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 128);
        FromIntoMemory::into_bytes(self.ss_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.__ss_pad1, &mut into[2..2 + 6]);
        FromIntoMemory::into_bytes(self.__ss_align, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.__ss_pad2, &mut into[16..16 + 112]);
    }
    fn size() -> usize {
        128
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SOCKET(pub PtrRepr);
impl SOCKET {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SOCKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for SOCKET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for SOCKET {}
impl ::core::hash::Hash for SOCKET {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for SOCKET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SOCKET").field(&self.0).finish()
    }
}
impl FromIntoMemory for SOCKET {
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
pub struct SOCKET_ADDRESS {
    pub lpSockaddr: MutPtr<SOCKADDR>,
    pub iSockaddrLength: i32,
}
impl ::core::marker::Copy for SOCKET_ADDRESS {}
impl ::core::clone::Clone for SOCKET_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKET_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_ADDRESS")
            .field("lpSockaddr", &self.lpSockaddr)
            .field("iSockaddrLength", &self.iSockaddrLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKET_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.lpSockaddr == other.lpSockaddr && self.iSockaddrLength == other.iSockaddrLength
    }
}
impl ::core::cmp::Eq for SOCKET_ADDRESS {}
impl FromIntoMemory for SOCKET_ADDRESS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_lpSockaddr = <MutPtr<SOCKADDR> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_iSockaddrLength = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            lpSockaddr: f_lpSockaddr,
            iSockaddrLength: f_iSockaddrLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.lpSockaddr, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.iSockaddrLength, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct SOCKET_ADDRESS_LIST {
    pub iAddressCount: i32,
    pub Address: [SOCKET_ADDRESS; 1],
}
impl ::core::marker::Copy for SOCKET_ADDRESS_LIST {}
impl ::core::clone::Clone for SOCKET_ADDRESS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKET_ADDRESS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_ADDRESS_LIST")
            .field("iAddressCount", &self.iAddressCount)
            .field("Address", &self.Address)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKET_ADDRESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.iAddressCount == other.iAddressCount && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for SOCKET_ADDRESS_LIST {}
impl FromIntoMemory for SOCKET_ADDRESS_LIST {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_iAddressCount = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Address = <[SOCKET_ADDRESS; 1] as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        Self {
            iAddressCount: f_iAddressCount,
            Address: f_Address,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.iAddressCount, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Address, &mut into[4..4 + 8]);
    }
    fn size() -> usize {
        12
    }
}
pub const SOCKET_DEFAULT2_QM_POLICY: crate::core::GUID =
    crate::core::GUID::from_u128(0xaec2ef9c_3a4d_4d3e_8842_239942e39a47);
pub const SOCKET_ERROR: i32 = -1i32;
pub const SOCKET_INFO_CONNECTION_ENCRYPTED: u32 = 2u32;
pub const SOCKET_INFO_CONNECTION_IMPERSONATED: u32 = 4u32;
pub const SOCKET_INFO_CONNECTION_SECURED: u32 = 1u32;
pub struct SOCKET_PEER_TARGET_NAME {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: SOCKADDR_STORAGE,
    pub PeerTargetNameStringLen: u32,
    pub AllStrings: [u16; 1],
}
impl ::core::marker::Copy for SOCKET_PEER_TARGET_NAME {}
impl ::core::clone::Clone for SOCKET_PEER_TARGET_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKET_PEER_TARGET_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_PEER_TARGET_NAME")
            .field("SecurityProtocol", &self.SecurityProtocol)
            .field("PeerAddress", &self.PeerAddress)
            .field("PeerTargetNameStringLen", &self.PeerTargetNameStringLen)
            .field("AllStrings", &self.AllStrings)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKET_PEER_TARGET_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol
            && self.PeerAddress == other.PeerAddress
            && self.PeerTargetNameStringLen == other.PeerTargetNameStringLen
            && self.AllStrings == other.AllStrings
    }
}
impl ::core::cmp::Eq for SOCKET_PEER_TARGET_NAME {}
impl FromIntoMemory for SOCKET_PEER_TARGET_NAME {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 144);
        let f_SecurityProtocol =
            <SOCKET_SECURITY_PROTOCOL as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_PeerAddress = <SOCKADDR_STORAGE as FromIntoMemory>::from_bytes(&from[8..8 + 128]);
        let f_PeerTargetNameStringLen = <u32 as FromIntoMemory>::from_bytes(&from[136..136 + 4]);
        let f_AllStrings = <[u16; 1] as FromIntoMemory>::from_bytes(&from[140..140 + 1]);
        Self {
            SecurityProtocol: f_SecurityProtocol,
            PeerAddress: f_PeerAddress,
            PeerTargetNameStringLen: f_PeerTargetNameStringLen,
            AllStrings: f_AllStrings,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 144);
        FromIntoMemory::into_bytes(self.SecurityProtocol, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.PeerAddress, &mut into[8..8 + 128]);
        FromIntoMemory::into_bytes(self.PeerTargetNameStringLen, &mut into[136..136 + 4]);
        FromIntoMemory::into_bytes(self.AllStrings, &mut into[140..140 + 1]);
    }
    fn size() -> usize {
        144
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SOCKET_PRIORITY_HINT(pub i32);
pub const SocketPriorityHintVeryLow: SOCKET_PRIORITY_HINT = SOCKET_PRIORITY_HINT(0i32);
pub const SocketPriorityHintLow: SOCKET_PRIORITY_HINT = SOCKET_PRIORITY_HINT(1i32);
pub const SocketPriorityHintNormal: SOCKET_PRIORITY_HINT = SOCKET_PRIORITY_HINT(2i32);
pub const SocketMaximumPriorityHintType: SOCKET_PRIORITY_HINT = SOCKET_PRIORITY_HINT(3i32);
impl ::core::marker::Copy for SOCKET_PRIORITY_HINT {}
impl ::core::clone::Clone for SOCKET_PRIORITY_HINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SOCKET_PRIORITY_HINT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SOCKET_PRIORITY_HINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SOCKET_PRIORITY_HINT")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SOCKET_PRIORITY_HINT {
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
pub struct SOCKET_PROCESSOR_AFFINITY {
    pub Processor: super::super::System::Kernel::PROCESSOR_NUMBER,
    pub NumaNodeId: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for SOCKET_PROCESSOR_AFFINITY {}
impl ::core::clone::Clone for SOCKET_PROCESSOR_AFFINITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKET_PROCESSOR_AFFINITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_PROCESSOR_AFFINITY")
            .field("Processor", &self.Processor)
            .field("NumaNodeId", &self.NumaNodeId)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKET_PROCESSOR_AFFINITY {
    fn eq(&self, other: &Self) -> bool {
        self.Processor == other.Processor
            && self.NumaNodeId == other.NumaNodeId
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for SOCKET_PROCESSOR_AFFINITY {}
impl FromIntoMemory for SOCKET_PROCESSOR_AFFINITY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Processor =
            <super::super::System::Kernel::PROCESSOR_NUMBER as FromIntoMemory>::from_bytes(
                &from[0..0 + 4],
            );
        let f_NumaNodeId = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_Reserved = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        Self {
            Processor: f_Processor,
            NumaNodeId: f_NumaNodeId,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Processor, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.NumaNodeId, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[6..6 + 2]);
    }
    fn size() -> usize {
        8
    }
}
pub const SOCKET_QUERY_IPSEC2_ABORT_CONNECTION_ON_FIELD_CHANGE: u32 = 1u32;
pub const SOCKET_QUERY_IPSEC2_FIELD_MASK_MM_SA_ID: u32 = 1u32;
pub const SOCKET_QUERY_IPSEC2_FIELD_MASK_QM_SA_ID: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SOCKET_SECURITY_PROTOCOL(pub i32);
pub const SOCKET_SECURITY_PROTOCOL_DEFAULT: SOCKET_SECURITY_PROTOCOL =
    SOCKET_SECURITY_PROTOCOL(0i32);
pub const SOCKET_SECURITY_PROTOCOL_IPSEC: SOCKET_SECURITY_PROTOCOL = SOCKET_SECURITY_PROTOCOL(1i32);
pub const SOCKET_SECURITY_PROTOCOL_IPSEC2: SOCKET_SECURITY_PROTOCOL =
    SOCKET_SECURITY_PROTOCOL(2i32);
pub const SOCKET_SECURITY_PROTOCOL_INVALID: SOCKET_SECURITY_PROTOCOL =
    SOCKET_SECURITY_PROTOCOL(3i32);
impl ::core::marker::Copy for SOCKET_SECURITY_PROTOCOL {}
impl ::core::clone::Clone for SOCKET_SECURITY_PROTOCOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SOCKET_SECURITY_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SOCKET_SECURITY_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SOCKET_SECURITY_PROTOCOL")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SOCKET_SECURITY_PROTOCOL {
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
pub struct SOCKET_SECURITY_QUERY_INFO {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub Flags: u32,
    pub PeerApplicationAccessTokenHandle: u64,
    pub PeerMachineAccessTokenHandle: u64,
}
impl ::core::marker::Copy for SOCKET_SECURITY_QUERY_INFO {}
impl ::core::clone::Clone for SOCKET_SECURITY_QUERY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKET_SECURITY_QUERY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_SECURITY_QUERY_INFO")
            .field("SecurityProtocol", &self.SecurityProtocol)
            .field("Flags", &self.Flags)
            .field(
                "PeerApplicationAccessTokenHandle",
                &self.PeerApplicationAccessTokenHandle,
            )
            .field(
                "PeerMachineAccessTokenHandle",
                &self.PeerMachineAccessTokenHandle,
            )
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKET_SECURITY_QUERY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol
            && self.Flags == other.Flags
            && self.PeerApplicationAccessTokenHandle == other.PeerApplicationAccessTokenHandle
            && self.PeerMachineAccessTokenHandle == other.PeerMachineAccessTokenHandle
    }
}
impl ::core::cmp::Eq for SOCKET_SECURITY_QUERY_INFO {}
impl FromIntoMemory for SOCKET_SECURITY_QUERY_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_SecurityProtocol =
            <SOCKET_SECURITY_PROTOCOL as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_PeerApplicationAccessTokenHandle =
            <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_PeerMachineAccessTokenHandle = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        Self {
            SecurityProtocol: f_SecurityProtocol,
            Flags: f_Flags,
            PeerApplicationAccessTokenHandle: f_PeerApplicationAccessTokenHandle,
            PeerMachineAccessTokenHandle: f_PeerMachineAccessTokenHandle,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.SecurityProtocol, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.PeerApplicationAccessTokenHandle, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.PeerMachineAccessTokenHandle, &mut into[16..16 + 8]);
    }
    fn size() -> usize {
        24
    }
}
pub struct SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub Flags: u32,
    pub PeerApplicationAccessTokenHandle: u64,
    pub PeerMachineAccessTokenHandle: u64,
    pub MmSaId: u64,
    pub QmSaId: u64,
    pub NegotiationWinerr: u32,
    pub SaLookupContext: crate::core::GUID,
}
impl ::core::marker::Copy for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {}
impl ::core::clone::Clone for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_SECURITY_QUERY_INFO_IPSEC2")
            .field("SecurityProtocol", &self.SecurityProtocol)
            .field("Flags", &self.Flags)
            .field(
                "PeerApplicationAccessTokenHandle",
                &self.PeerApplicationAccessTokenHandle,
            )
            .field(
                "PeerMachineAccessTokenHandle",
                &self.PeerMachineAccessTokenHandle,
            )
            .field("MmSaId", &self.MmSaId)
            .field("QmSaId", &self.QmSaId)
            .field("NegotiationWinerr", &self.NegotiationWinerr)
            .field("SaLookupContext", &self.SaLookupContext)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol
            && self.Flags == other.Flags
            && self.PeerApplicationAccessTokenHandle == other.PeerApplicationAccessTokenHandle
            && self.PeerMachineAccessTokenHandle == other.PeerMachineAccessTokenHandle
            && self.MmSaId == other.MmSaId
            && self.QmSaId == other.QmSaId
            && self.NegotiationWinerr == other.NegotiationWinerr
            && self.SaLookupContext == other.SaLookupContext
    }
}
impl ::core::cmp::Eq for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {}
impl FromIntoMemory for SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64);
        let f_SecurityProtocol =
            <SOCKET_SECURITY_PROTOCOL as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_PeerApplicationAccessTokenHandle =
            <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_PeerMachineAccessTokenHandle = <u64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_MmSaId = <u64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        let f_QmSaId = <u64 as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        let f_NegotiationWinerr = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_SaLookupContext =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[44..44 + 16]);
        Self {
            SecurityProtocol: f_SecurityProtocol,
            Flags: f_Flags,
            PeerApplicationAccessTokenHandle: f_PeerApplicationAccessTokenHandle,
            PeerMachineAccessTokenHandle: f_PeerMachineAccessTokenHandle,
            MmSaId: f_MmSaId,
            QmSaId: f_QmSaId,
            NegotiationWinerr: f_NegotiationWinerr,
            SaLookupContext: f_SaLookupContext,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64);
        FromIntoMemory::into_bytes(self.SecurityProtocol, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.PeerApplicationAccessTokenHandle, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.PeerMachineAccessTokenHandle, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.MmSaId, &mut into[24..24 + 8]);
        FromIntoMemory::into_bytes(self.QmSaId, &mut into[32..32 + 8]);
        FromIntoMemory::into_bytes(self.NegotiationWinerr, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.SaLookupContext, &mut into[44..44 + 16]);
    }
    fn size() -> usize {
        64
    }
}
pub struct SOCKET_SECURITY_QUERY_TEMPLATE {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: SOCKADDR_STORAGE,
    pub PeerTokenAccessMask: u32,
}
impl ::core::marker::Copy for SOCKET_SECURITY_QUERY_TEMPLATE {}
impl ::core::clone::Clone for SOCKET_SECURITY_QUERY_TEMPLATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKET_SECURITY_QUERY_TEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_SECURITY_QUERY_TEMPLATE")
            .field("SecurityProtocol", &self.SecurityProtocol)
            .field("PeerAddress", &self.PeerAddress)
            .field("PeerTokenAccessMask", &self.PeerTokenAccessMask)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKET_SECURITY_QUERY_TEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol
            && self.PeerAddress == other.PeerAddress
            && self.PeerTokenAccessMask == other.PeerTokenAccessMask
    }
}
impl ::core::cmp::Eq for SOCKET_SECURITY_QUERY_TEMPLATE {}
impl FromIntoMemory for SOCKET_SECURITY_QUERY_TEMPLATE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 144);
        let f_SecurityProtocol =
            <SOCKET_SECURITY_PROTOCOL as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_PeerAddress = <SOCKADDR_STORAGE as FromIntoMemory>::from_bytes(&from[8..8 + 128]);
        let f_PeerTokenAccessMask = <u32 as FromIntoMemory>::from_bytes(&from[136..136 + 4]);
        Self {
            SecurityProtocol: f_SecurityProtocol,
            PeerAddress: f_PeerAddress,
            PeerTokenAccessMask: f_PeerTokenAccessMask,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 144);
        FromIntoMemory::into_bytes(self.SecurityProtocol, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.PeerAddress, &mut into[8..8 + 128]);
        FromIntoMemory::into_bytes(self.PeerTokenAccessMask, &mut into[136..136 + 4]);
    }
    fn size() -> usize {
        144
    }
}
pub struct SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: SOCKADDR_STORAGE,
    pub PeerTokenAccessMask: u32,
    pub Flags: u32,
    pub FieldMask: u32,
}
impl ::core::marker::Copy for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {}
impl ::core::clone::Clone for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2")
            .field("SecurityProtocol", &self.SecurityProtocol)
            .field("PeerAddress", &self.PeerAddress)
            .field("PeerTokenAccessMask", &self.PeerTokenAccessMask)
            .field("Flags", &self.Flags)
            .field("FieldMask", &self.FieldMask)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol
            && self.PeerAddress == other.PeerAddress
            && self.PeerTokenAccessMask == other.PeerTokenAccessMask
            && self.Flags == other.Flags
            && self.FieldMask == other.FieldMask
    }
}
impl ::core::cmp::Eq for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {}
impl FromIntoMemory for SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 152);
        let f_SecurityProtocol =
            <SOCKET_SECURITY_PROTOCOL as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_PeerAddress = <SOCKADDR_STORAGE as FromIntoMemory>::from_bytes(&from[8..8 + 128]);
        let f_PeerTokenAccessMask = <u32 as FromIntoMemory>::from_bytes(&from[136..136 + 4]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[140..140 + 4]);
        let f_FieldMask = <u32 as FromIntoMemory>::from_bytes(&from[144..144 + 4]);
        Self {
            SecurityProtocol: f_SecurityProtocol,
            PeerAddress: f_PeerAddress,
            PeerTokenAccessMask: f_PeerTokenAccessMask,
            Flags: f_Flags,
            FieldMask: f_FieldMask,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 152);
        FromIntoMemory::into_bytes(self.SecurityProtocol, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.PeerAddress, &mut into[8..8 + 128]);
        FromIntoMemory::into_bytes(self.PeerTokenAccessMask, &mut into[136..136 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[140..140 + 4]);
        FromIntoMemory::into_bytes(self.FieldMask, &mut into[144..144 + 4]);
    }
    fn size() -> usize {
        152
    }
}
pub struct SOCKET_SECURITY_SETTINGS {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub SecurityFlags: u32,
}
impl ::core::marker::Copy for SOCKET_SECURITY_SETTINGS {}
impl ::core::clone::Clone for SOCKET_SECURITY_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKET_SECURITY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_SECURITY_SETTINGS")
            .field("SecurityProtocol", &self.SecurityProtocol)
            .field("SecurityFlags", &self.SecurityFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKET_SECURITY_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol && self.SecurityFlags == other.SecurityFlags
    }
}
impl ::core::cmp::Eq for SOCKET_SECURITY_SETTINGS {}
impl FromIntoMemory for SOCKET_SECURITY_SETTINGS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_SecurityProtocol =
            <SOCKET_SECURITY_PROTOCOL as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_SecurityFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            SecurityProtocol: f_SecurityProtocol,
            SecurityFlags: f_SecurityFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.SecurityProtocol, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.SecurityFlags, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct SOCKET_SECURITY_SETTINGS_IPSEC {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub SecurityFlags: u32,
    pub IpsecFlags: u32,
    pub AuthipMMPolicyKey: crate::core::GUID,
    pub AuthipQMPolicyKey: crate::core::GUID,
    pub Reserved: crate::core::GUID,
    pub Reserved2: u64,
    pub UserNameStringLen: u32,
    pub DomainNameStringLen: u32,
    pub PasswordStringLen: u32,
    pub AllStrings: [u16; 1],
}
impl ::core::marker::Copy for SOCKET_SECURITY_SETTINGS_IPSEC {}
impl ::core::clone::Clone for SOCKET_SECURITY_SETTINGS_IPSEC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCKET_SECURITY_SETTINGS_IPSEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKET_SECURITY_SETTINGS_IPSEC")
            .field("SecurityProtocol", &self.SecurityProtocol)
            .field("SecurityFlags", &self.SecurityFlags)
            .field("IpsecFlags", &self.IpsecFlags)
            .field("AuthipMMPolicyKey", &self.AuthipMMPolicyKey)
            .field("AuthipQMPolicyKey", &self.AuthipQMPolicyKey)
            .field("Reserved", &self.Reserved)
            .field("Reserved2", &self.Reserved2)
            .field("UserNameStringLen", &self.UserNameStringLen)
            .field("DomainNameStringLen", &self.DomainNameStringLen)
            .field("PasswordStringLen", &self.PasswordStringLen)
            .field("AllStrings", &self.AllStrings)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCKET_SECURITY_SETTINGS_IPSEC {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityProtocol == other.SecurityProtocol
            && self.SecurityFlags == other.SecurityFlags
            && self.IpsecFlags == other.IpsecFlags
            && self.AuthipMMPolicyKey == other.AuthipMMPolicyKey
            && self.AuthipQMPolicyKey == other.AuthipQMPolicyKey
            && self.Reserved == other.Reserved
            && self.Reserved2 == other.Reserved2
            && self.UserNameStringLen == other.UserNameStringLen
            && self.DomainNameStringLen == other.DomainNameStringLen
            && self.PasswordStringLen == other.PasswordStringLen
            && self.AllStrings == other.AllStrings
    }
}
impl ::core::cmp::Eq for SOCKET_SECURITY_SETTINGS_IPSEC {}
impl FromIntoMemory for SOCKET_SECURITY_SETTINGS_IPSEC {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 88);
        let f_SecurityProtocol =
            <SOCKET_SECURITY_PROTOCOL as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_SecurityFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_IpsecFlags = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_AuthipMMPolicyKey =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_AuthipQMPolicyKey =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[28..28 + 16]);
        let f_Reserved = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[44..44 + 16]);
        let f_Reserved2 = <u64 as FromIntoMemory>::from_bytes(&from[64..64 + 8]);
        let f_UserNameStringLen = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_DomainNameStringLen = <u32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_PasswordStringLen = <u32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_AllStrings = <[u16; 1] as FromIntoMemory>::from_bytes(&from[84..84 + 1]);
        Self {
            SecurityProtocol: f_SecurityProtocol,
            SecurityFlags: f_SecurityFlags,
            IpsecFlags: f_IpsecFlags,
            AuthipMMPolicyKey: f_AuthipMMPolicyKey,
            AuthipQMPolicyKey: f_AuthipQMPolicyKey,
            Reserved: f_Reserved,
            Reserved2: f_Reserved2,
            UserNameStringLen: f_UserNameStringLen,
            DomainNameStringLen: f_DomainNameStringLen,
            PasswordStringLen: f_PasswordStringLen,
            AllStrings: f_AllStrings,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 88);
        FromIntoMemory::into_bytes(self.SecurityProtocol, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.SecurityFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.IpsecFlags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.AuthipMMPolicyKey, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.AuthipQMPolicyKey, &mut into[28..28 + 16]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[44..44 + 16]);
        FromIntoMemory::into_bytes(self.Reserved2, &mut into[64..64 + 8]);
        FromIntoMemory::into_bytes(self.UserNameStringLen, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.DomainNameStringLen, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.PasswordStringLen, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.AllStrings, &mut into[84..84 + 1]);
    }
    fn size() -> usize {
        88
    }
}
pub const SOCKET_SETTINGS_ALLOW_INSECURE: u32 = 2u32;
pub const SOCKET_SETTINGS_GUARANTEE_ENCRYPTION: u32 = 1u32;
pub const SOCKET_SETTINGS_IPSEC_ALLOW_FIRST_INBOUND_PKT_UNENCRYPTED: u32 = 4u32;
pub const SOCKET_SETTINGS_IPSEC_OPTIONAL_PEER_NAME_VERIFICATION: u32 = 2u32;
pub const SOCKET_SETTINGS_IPSEC_PEER_NAME_IS_RAW_FORMAT: u32 = 8u32;
pub const SOCKET_SETTINGS_IPSEC_SKIP_FILTER_INSTANTIATION: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SOCKET_USAGE_TYPE(pub i32);
pub const SYSTEM_CRITICAL_SOCKET: SOCKET_USAGE_TYPE = SOCKET_USAGE_TYPE(1i32);
impl ::core::marker::Copy for SOCKET_USAGE_TYPE {}
impl ::core::clone::Clone for SOCKET_USAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SOCKET_USAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SOCKET_USAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SOCKET_USAGE_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for SOCKET_USAGE_TYPE {
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
pub const SOCK_DGRAM: u16 = 2u16;
pub const SOCK_NOTIFY_EVENT_ERR: u32 = 64u32;
pub const SOCK_NOTIFY_EVENT_HANGUP: u32 = 4u32;
pub const SOCK_NOTIFY_EVENT_IN: u32 = 1u32;
pub const SOCK_NOTIFY_EVENT_OUT: u32 = 2u32;
pub const SOCK_NOTIFY_EVENT_REMOVE: u32 = 128u32;
pub const SOCK_NOTIFY_OP_DISABLE: u32 = 2u32;
pub const SOCK_NOTIFY_OP_ENABLE: u32 = 1u32;
pub const SOCK_NOTIFY_OP_NONE: u32 = 0u32;
pub const SOCK_NOTIFY_OP_REMOVE: u32 = 4u32;
pub const SOCK_NOTIFY_REGISTER_EVENT_HANGUP: u32 = 4u32;
pub const SOCK_NOTIFY_REGISTER_EVENT_IN: u32 = 1u32;
pub const SOCK_NOTIFY_REGISTER_EVENT_NONE: u32 = 0u32;
pub const SOCK_NOTIFY_REGISTER_EVENT_OUT: u32 = 2u32;
pub struct SOCK_NOTIFY_REGISTRATION {
    pub socket: SOCKET,
    pub completionKey: MutPtr<::core::ffi::c_void>,
    pub eventFilter: u16,
    pub operation: u8,
    pub triggerFlags: u8,
    pub registrationResult: u32,
}
impl ::core::marker::Copy for SOCK_NOTIFY_REGISTRATION {}
impl ::core::clone::Clone for SOCK_NOTIFY_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOCK_NOTIFY_REGISTRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCK_NOTIFY_REGISTRATION")
            .field("socket", &self.socket)
            .field("completionKey", &self.completionKey)
            .field("eventFilter", &self.eventFilter)
            .field("operation", &self.operation)
            .field("triggerFlags", &self.triggerFlags)
            .field("registrationResult", &self.registrationResult)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SOCK_NOTIFY_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.socket == other.socket
            && self.completionKey == other.completionKey
            && self.eventFilter == other.eventFilter
            && self.operation == other.operation
            && self.triggerFlags == other.triggerFlags
            && self.registrationResult == other.registrationResult
    }
}
impl ::core::cmp::Eq for SOCK_NOTIFY_REGISTRATION {}
impl FromIntoMemory for SOCK_NOTIFY_REGISTRATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_socket = <SOCKET as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_completionKey =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_eventFilter = <u16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_operation = <u8 as FromIntoMemory>::from_bytes(&from[10..10 + 1]);
        let f_triggerFlags = <u8 as FromIntoMemory>::from_bytes(&from[11..11 + 1]);
        let f_registrationResult = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            socket: f_socket,
            completionKey: f_completionKey,
            eventFilter: f_eventFilter,
            operation: f_operation,
            triggerFlags: f_triggerFlags,
            registrationResult: f_registrationResult,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.socket, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.completionKey, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.eventFilter, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.operation, &mut into[10..10 + 1]);
        FromIntoMemory::into_bytes(self.triggerFlags, &mut into[11..11 + 1]);
        FromIntoMemory::into_bytes(self.registrationResult, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const SOCK_NOTIFY_TRIGGER_EDGE: u32 = 8u32;
pub const SOCK_NOTIFY_TRIGGER_LEVEL: u32 = 4u32;
pub const SOCK_NOTIFY_TRIGGER_ONESHOT: u32 = 1u32;
pub const SOCK_NOTIFY_TRIGGER_PERSISTENT: u32 = 2u32;
pub const SOCK_RAW: u16 = 3u16;
pub const SOCK_RDM: u16 = 4u16;
pub const SOCK_SEQPACKET: u16 = 5u16;
pub const SOCK_STREAM: u16 = 1u16;
pub const SOL_IRLMP: u32 = 255u32;
pub const SOL_SOCKET: u32 = 65535u32;
pub const SOMAXCONN: u32 = 5u32;
pub const SO_ACCEPTCONN: u32 = 2u32;
pub const SO_BROADCAST: u32 = 32u32;
pub const SO_BSP_STATE: u32 = 4105u32;
pub const SO_COMPARTMENT_ID: u32 = 12292u32;
pub const SO_CONDITIONAL_ACCEPT: u32 = 12290u32;
pub const SO_CONNDATA: u32 = 28672u32;
pub const SO_CONNDATALEN: u32 = 28676u32;
pub const SO_CONNECT_TIME: u32 = 28684u32;
pub const SO_CONNOPT: u32 = 28673u32;
pub const SO_CONNOPTLEN: u32 = 28677u32;
pub const SO_DEBUG: u32 = 1u32;
pub const SO_DISCDATA: u32 = 28674u32;
pub const SO_DISCDATALEN: u32 = 28678u32;
pub const SO_DISCOPT: u32 = 28675u32;
pub const SO_DISCOPTLEN: u32 = 28679u32;
pub const SO_DONTROUTE: u32 = 16u32;
pub const SO_ERROR: u32 = 4103u32;
pub const SO_GROUP_ID: u32 = 8193u32;
pub const SO_GROUP_PRIORITY: u32 = 8194u32;
pub const SO_KEEPALIVE: u32 = 8u32;
pub const SO_LINGER: u32 = 128u32;
pub const SO_MAXDG: u32 = 28681u32;
pub const SO_MAXPATHDG: u32 = 28682u32;
pub const SO_MAX_MSG_SIZE: u32 = 8195u32;
pub const SO_OOBINLINE: u32 = 256u32;
pub const SO_OPENTYPE: u32 = 28680u32;
pub const SO_ORIGINAL_DST: u32 = 12303u32;
pub const SO_PAUSE_ACCEPT: u32 = 12291u32;
pub const SO_PORT_SCALABILITY: u32 = 12294u32;
pub const SO_PROTOCOL_INFO: u32 = 8197u32;
pub const SO_PROTOCOL_INFOA: u32 = 8196u32;
pub const SO_PROTOCOL_INFOW: u32 = 8197u32;
pub const SO_RANDOMIZE_PORT: u32 = 12293u32;
pub const SO_RCVBUF: u32 = 4098u32;
pub const SO_RCVLOWAT: u32 = 4100u32;
pub const SO_RCVTIMEO: u32 = 4102u32;
pub const SO_REUSEADDR: u32 = 4u32;
pub const SO_REUSE_MULTICASTPORT: u32 = 12296u32;
pub const SO_REUSE_UNICASTPORT: u32 = 12295u32;
pub const SO_SNDBUF: u32 = 4097u32;
pub const SO_SNDLOWAT: u32 = 4099u32;
pub const SO_SNDTIMEO: u32 = 4101u32;
pub const SO_SYNCHRONOUS_ALERT: u32 = 16u32;
pub const SO_SYNCHRONOUS_NONALERT: u32 = 32u32;
pub const SO_TIMESTAMP: u32 = 12298u32;
pub const SO_TIMESTAMP_ID: u32 = 12299u32;
pub const SO_TYPE: u32 = 4104u32;
pub const SO_UPDATE_ACCEPT_CONTEXT: u32 = 28683u32;
pub const SO_UPDATE_CONNECT_CONTEXT: u32 = 28688u32;
pub const SO_USELOOPBACK: u32 = 64u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TCPSTATE(pub i32);
pub const TCPSTATE_CLOSED: TCPSTATE = TCPSTATE(0i32);
pub const TCPSTATE_LISTEN: TCPSTATE = TCPSTATE(1i32);
pub const TCPSTATE_SYN_SENT: TCPSTATE = TCPSTATE(2i32);
pub const TCPSTATE_SYN_RCVD: TCPSTATE = TCPSTATE(3i32);
pub const TCPSTATE_ESTABLISHED: TCPSTATE = TCPSTATE(4i32);
pub const TCPSTATE_FIN_WAIT_1: TCPSTATE = TCPSTATE(5i32);
pub const TCPSTATE_FIN_WAIT_2: TCPSTATE = TCPSTATE(6i32);
pub const TCPSTATE_CLOSE_WAIT: TCPSTATE = TCPSTATE(7i32);
pub const TCPSTATE_CLOSING: TCPSTATE = TCPSTATE(8i32);
pub const TCPSTATE_LAST_ACK: TCPSTATE = TCPSTATE(9i32);
pub const TCPSTATE_TIME_WAIT: TCPSTATE = TCPSTATE(10i32);
pub const TCPSTATE_MAX: TCPSTATE = TCPSTATE(11i32);
impl ::core::marker::Copy for TCPSTATE {}
impl ::core::clone::Clone for TCPSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TCPSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TCPSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCPSTATE").field(&self.0).finish()
    }
}
impl FromIntoMemory for TCPSTATE {
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
pub struct TCP_ACK_FREQUENCY_PARAMETERS {
    pub TcpDelayedAckFrequency: u8,
}
impl ::core::marker::Copy for TCP_ACK_FREQUENCY_PARAMETERS {}
impl ::core::clone::Clone for TCP_ACK_FREQUENCY_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_ACK_FREQUENCY_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ACK_FREQUENCY_PARAMETERS")
            .field("TcpDelayedAckFrequency", &self.TcpDelayedAckFrequency)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TCP_ACK_FREQUENCY_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.TcpDelayedAckFrequency == other.TcpDelayedAckFrequency
    }
}
impl ::core::cmp::Eq for TCP_ACK_FREQUENCY_PARAMETERS {}
impl FromIntoMemory for TCP_ACK_FREQUENCY_PARAMETERS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 1);
        let f_TcpDelayedAckFrequency = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        Self {
            TcpDelayedAckFrequency: f_TcpDelayedAckFrequency,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 1);
        FromIntoMemory::into_bytes(self.TcpDelayedAckFrequency, &mut into[0..0 + 1]);
    }
    fn size() -> usize {
        1
    }
}
pub const TCP_ATMARK: u32 = 8u32;
pub const TCP_BSDURGENT: u32 = 28672u32;
pub const TCP_CONGESTION_ALGORITHM: u32 = 12u32;
pub const TCP_DELAY_FIN_ACK: u32 = 13u32;
pub const TCP_EXPEDITED_1122: u32 = 2u32;
pub const TCP_FAIL_CONNECT_ON_ICMP_ERROR: u32 = 18u32;
pub const TCP_FASTOPEN: u32 = 15u32;
pub const TCP_ICMP_ERROR_INFO: u32 = 19u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TCP_ICW_LEVEL(pub i32);
pub const TCP_ICW_LEVEL_DEFAULT: TCP_ICW_LEVEL = TCP_ICW_LEVEL(0i32);
pub const TCP_ICW_LEVEL_HIGH: TCP_ICW_LEVEL = TCP_ICW_LEVEL(1i32);
pub const TCP_ICW_LEVEL_VERY_HIGH: TCP_ICW_LEVEL = TCP_ICW_LEVEL(2i32);
pub const TCP_ICW_LEVEL_AGGRESSIVE: TCP_ICW_LEVEL = TCP_ICW_LEVEL(3i32);
pub const TCP_ICW_LEVEL_EXPERIMENTAL: TCP_ICW_LEVEL = TCP_ICW_LEVEL(4i32);
pub const TCP_ICW_LEVEL_COMPAT: TCP_ICW_LEVEL = TCP_ICW_LEVEL(254i32);
pub const TCP_ICW_LEVEL_MAX: TCP_ICW_LEVEL = TCP_ICW_LEVEL(255i32);
impl ::core::marker::Copy for TCP_ICW_LEVEL {}
impl ::core::clone::Clone for TCP_ICW_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TCP_ICW_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TCP_ICW_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCP_ICW_LEVEL").field(&self.0).finish()
    }
}
impl FromIntoMemory for TCP_ICW_LEVEL {
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
pub struct TCP_ICW_PARAMETERS {
    pub Level: TCP_ICW_LEVEL,
}
impl ::core::marker::Copy for TCP_ICW_PARAMETERS {}
impl ::core::clone::Clone for TCP_ICW_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_ICW_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_ICW_PARAMETERS")
            .field("Level", &self.Level)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TCP_ICW_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level
    }
}
impl ::core::cmp::Eq for TCP_ICW_PARAMETERS {}
impl FromIntoMemory for TCP_ICW_PARAMETERS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Level = <TCP_ICW_LEVEL as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { Level: f_Level }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Level, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct TCP_INFO_v0 {
    pub State: TCPSTATE,
    pub Mss: u32,
    pub ConnectionTimeMs: u64,
    pub TimestampsEnabled: super::super::Foundation::BOOLEAN,
    pub RttUs: u32,
    pub MinRttUs: u32,
    pub BytesInFlight: u32,
    pub Cwnd: u32,
    pub SndWnd: u32,
    pub RcvWnd: u32,
    pub RcvBuf: u32,
    pub BytesOut: u64,
    pub BytesIn: u64,
    pub BytesReordered: u32,
    pub BytesRetrans: u32,
    pub FastRetrans: u32,
    pub DupAcksIn: u32,
    pub TimeoutEpisodes: u32,
    pub SynRetrans: u8,
}
impl ::core::marker::Copy for TCP_INFO_v0 {}
impl ::core::clone::Clone for TCP_INFO_v0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_INFO_v0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_INFO_v0")
            .field("State", &self.State)
            .field("Mss", &self.Mss)
            .field("ConnectionTimeMs", &self.ConnectionTimeMs)
            .field("TimestampsEnabled", &self.TimestampsEnabled)
            .field("RttUs", &self.RttUs)
            .field("MinRttUs", &self.MinRttUs)
            .field("BytesInFlight", &self.BytesInFlight)
            .field("Cwnd", &self.Cwnd)
            .field("SndWnd", &self.SndWnd)
            .field("RcvWnd", &self.RcvWnd)
            .field("RcvBuf", &self.RcvBuf)
            .field("BytesOut", &self.BytesOut)
            .field("BytesIn", &self.BytesIn)
            .field("BytesReordered", &self.BytesReordered)
            .field("BytesRetrans", &self.BytesRetrans)
            .field("FastRetrans", &self.FastRetrans)
            .field("DupAcksIn", &self.DupAcksIn)
            .field("TimeoutEpisodes", &self.TimeoutEpisodes)
            .field("SynRetrans", &self.SynRetrans)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TCP_INFO_v0 {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State
            && self.Mss == other.Mss
            && self.ConnectionTimeMs == other.ConnectionTimeMs
            && self.TimestampsEnabled == other.TimestampsEnabled
            && self.RttUs == other.RttUs
            && self.MinRttUs == other.MinRttUs
            && self.BytesInFlight == other.BytesInFlight
            && self.Cwnd == other.Cwnd
            && self.SndWnd == other.SndWnd
            && self.RcvWnd == other.RcvWnd
            && self.RcvBuf == other.RcvBuf
            && self.BytesOut == other.BytesOut
            && self.BytesIn == other.BytesIn
            && self.BytesReordered == other.BytesReordered
            && self.BytesRetrans == other.BytesRetrans
            && self.FastRetrans == other.FastRetrans
            && self.DupAcksIn == other.DupAcksIn
            && self.TimeoutEpisodes == other.TimeoutEpisodes
            && self.SynRetrans == other.SynRetrans
    }
}
impl ::core::cmp::Eq for TCP_INFO_v0 {}
impl FromIntoMemory for TCP_INFO_v0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 88);
        let f_State = <TCPSTATE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mss = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ConnectionTimeMs = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_TimestampsEnabled =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[16..16 + 1]);
        let f_RttUs = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_MinRttUs = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_BytesInFlight = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_Cwnd = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_SndWnd = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_RcvWnd = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_RcvBuf = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_BytesOut = <u64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_BytesIn = <u64 as FromIntoMemory>::from_bytes(&from[56..56 + 8]);
        let f_BytesReordered = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_BytesRetrans = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_FastRetrans = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_DupAcksIn = <u32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_TimeoutEpisodes = <u32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_SynRetrans = <u8 as FromIntoMemory>::from_bytes(&from[84..84 + 1]);
        Self {
            State: f_State,
            Mss: f_Mss,
            ConnectionTimeMs: f_ConnectionTimeMs,
            TimestampsEnabled: f_TimestampsEnabled,
            RttUs: f_RttUs,
            MinRttUs: f_MinRttUs,
            BytesInFlight: f_BytesInFlight,
            Cwnd: f_Cwnd,
            SndWnd: f_SndWnd,
            RcvWnd: f_RcvWnd,
            RcvBuf: f_RcvBuf,
            BytesOut: f_BytesOut,
            BytesIn: f_BytesIn,
            BytesReordered: f_BytesReordered,
            BytesRetrans: f_BytesRetrans,
            FastRetrans: f_FastRetrans,
            DupAcksIn: f_DupAcksIn,
            TimeoutEpisodes: f_TimeoutEpisodes,
            SynRetrans: f_SynRetrans,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 88);
        FromIntoMemory::into_bytes(self.State, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mss, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ConnectionTimeMs, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.TimestampsEnabled, &mut into[16..16 + 1]);
        FromIntoMemory::into_bytes(self.RttUs, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.MinRttUs, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.BytesInFlight, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.Cwnd, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.SndWnd, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.RcvWnd, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.RcvBuf, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.BytesOut, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.BytesIn, &mut into[56..56 + 8]);
        FromIntoMemory::into_bytes(self.BytesReordered, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.BytesRetrans, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.FastRetrans, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.DupAcksIn, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.TimeoutEpisodes, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.SynRetrans, &mut into[84..84 + 1]);
    }
    fn size() -> usize {
        88
    }
}
pub struct TCP_INFO_v1 {
    pub State: TCPSTATE,
    pub Mss: u32,
    pub ConnectionTimeMs: u64,
    pub TimestampsEnabled: super::super::Foundation::BOOLEAN,
    pub RttUs: u32,
    pub MinRttUs: u32,
    pub BytesInFlight: u32,
    pub Cwnd: u32,
    pub SndWnd: u32,
    pub RcvWnd: u32,
    pub RcvBuf: u32,
    pub BytesOut: u64,
    pub BytesIn: u64,
    pub BytesReordered: u32,
    pub BytesRetrans: u32,
    pub FastRetrans: u32,
    pub DupAcksIn: u32,
    pub TimeoutEpisodes: u32,
    pub SynRetrans: u8,
    pub SndLimTransRwin: u32,
    pub SndLimTimeRwin: u32,
    pub SndLimBytesRwin: u64,
    pub SndLimTransCwnd: u32,
    pub SndLimTimeCwnd: u32,
    pub SndLimBytesCwnd: u64,
    pub SndLimTransSnd: u32,
    pub SndLimTimeSnd: u32,
    pub SndLimBytesSnd: u64,
}
impl ::core::marker::Copy for TCP_INFO_v1 {}
impl ::core::clone::Clone for TCP_INFO_v1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_INFO_v1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_INFO_v1")
            .field("State", &self.State)
            .field("Mss", &self.Mss)
            .field("ConnectionTimeMs", &self.ConnectionTimeMs)
            .field("TimestampsEnabled", &self.TimestampsEnabled)
            .field("RttUs", &self.RttUs)
            .field("MinRttUs", &self.MinRttUs)
            .field("BytesInFlight", &self.BytesInFlight)
            .field("Cwnd", &self.Cwnd)
            .field("SndWnd", &self.SndWnd)
            .field("RcvWnd", &self.RcvWnd)
            .field("RcvBuf", &self.RcvBuf)
            .field("BytesOut", &self.BytesOut)
            .field("BytesIn", &self.BytesIn)
            .field("BytesReordered", &self.BytesReordered)
            .field("BytesRetrans", &self.BytesRetrans)
            .field("FastRetrans", &self.FastRetrans)
            .field("DupAcksIn", &self.DupAcksIn)
            .field("TimeoutEpisodes", &self.TimeoutEpisodes)
            .field("SynRetrans", &self.SynRetrans)
            .field("SndLimTransRwin", &self.SndLimTransRwin)
            .field("SndLimTimeRwin", &self.SndLimTimeRwin)
            .field("SndLimBytesRwin", &self.SndLimBytesRwin)
            .field("SndLimTransCwnd", &self.SndLimTransCwnd)
            .field("SndLimTimeCwnd", &self.SndLimTimeCwnd)
            .field("SndLimBytesCwnd", &self.SndLimBytesCwnd)
            .field("SndLimTransSnd", &self.SndLimTransSnd)
            .field("SndLimTimeSnd", &self.SndLimTimeSnd)
            .field("SndLimBytesSnd", &self.SndLimBytesSnd)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TCP_INFO_v1 {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State
            && self.Mss == other.Mss
            && self.ConnectionTimeMs == other.ConnectionTimeMs
            && self.TimestampsEnabled == other.TimestampsEnabled
            && self.RttUs == other.RttUs
            && self.MinRttUs == other.MinRttUs
            && self.BytesInFlight == other.BytesInFlight
            && self.Cwnd == other.Cwnd
            && self.SndWnd == other.SndWnd
            && self.RcvWnd == other.RcvWnd
            && self.RcvBuf == other.RcvBuf
            && self.BytesOut == other.BytesOut
            && self.BytesIn == other.BytesIn
            && self.BytesReordered == other.BytesReordered
            && self.BytesRetrans == other.BytesRetrans
            && self.FastRetrans == other.FastRetrans
            && self.DupAcksIn == other.DupAcksIn
            && self.TimeoutEpisodes == other.TimeoutEpisodes
            && self.SynRetrans == other.SynRetrans
            && self.SndLimTransRwin == other.SndLimTransRwin
            && self.SndLimTimeRwin == other.SndLimTimeRwin
            && self.SndLimBytesRwin == other.SndLimBytesRwin
            && self.SndLimTransCwnd == other.SndLimTransCwnd
            && self.SndLimTimeCwnd == other.SndLimTimeCwnd
            && self.SndLimBytesCwnd == other.SndLimBytesCwnd
            && self.SndLimTransSnd == other.SndLimTransSnd
            && self.SndLimTimeSnd == other.SndLimTimeSnd
            && self.SndLimBytesSnd == other.SndLimBytesSnd
    }
}
impl ::core::cmp::Eq for TCP_INFO_v1 {}
impl FromIntoMemory for TCP_INFO_v1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 136);
        let f_State = <TCPSTATE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mss = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ConnectionTimeMs = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_TimestampsEnabled =
            <super::super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[16..16 + 1]);
        let f_RttUs = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_MinRttUs = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_BytesInFlight = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_Cwnd = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_SndWnd = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_RcvWnd = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_RcvBuf = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_BytesOut = <u64 as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        let f_BytesIn = <u64 as FromIntoMemory>::from_bytes(&from[56..56 + 8]);
        let f_BytesReordered = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_BytesRetrans = <u32 as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_FastRetrans = <u32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_DupAcksIn = <u32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_TimeoutEpisodes = <u32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_SynRetrans = <u8 as FromIntoMemory>::from_bytes(&from[84..84 + 1]);
        let f_SndLimTransRwin = <u32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_SndLimTimeRwin = <u32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_SndLimBytesRwin = <u64 as FromIntoMemory>::from_bytes(&from[96..96 + 8]);
        let f_SndLimTransCwnd = <u32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_SndLimTimeCwnd = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        let f_SndLimBytesCwnd = <u64 as FromIntoMemory>::from_bytes(&from[112..112 + 8]);
        let f_SndLimTransSnd = <u32 as FromIntoMemory>::from_bytes(&from[120..120 + 4]);
        let f_SndLimTimeSnd = <u32 as FromIntoMemory>::from_bytes(&from[124..124 + 4]);
        let f_SndLimBytesSnd = <u64 as FromIntoMemory>::from_bytes(&from[128..128 + 8]);
        Self {
            State: f_State,
            Mss: f_Mss,
            ConnectionTimeMs: f_ConnectionTimeMs,
            TimestampsEnabled: f_TimestampsEnabled,
            RttUs: f_RttUs,
            MinRttUs: f_MinRttUs,
            BytesInFlight: f_BytesInFlight,
            Cwnd: f_Cwnd,
            SndWnd: f_SndWnd,
            RcvWnd: f_RcvWnd,
            RcvBuf: f_RcvBuf,
            BytesOut: f_BytesOut,
            BytesIn: f_BytesIn,
            BytesReordered: f_BytesReordered,
            BytesRetrans: f_BytesRetrans,
            FastRetrans: f_FastRetrans,
            DupAcksIn: f_DupAcksIn,
            TimeoutEpisodes: f_TimeoutEpisodes,
            SynRetrans: f_SynRetrans,
            SndLimTransRwin: f_SndLimTransRwin,
            SndLimTimeRwin: f_SndLimTimeRwin,
            SndLimBytesRwin: f_SndLimBytesRwin,
            SndLimTransCwnd: f_SndLimTransCwnd,
            SndLimTimeCwnd: f_SndLimTimeCwnd,
            SndLimBytesCwnd: f_SndLimBytesCwnd,
            SndLimTransSnd: f_SndLimTransSnd,
            SndLimTimeSnd: f_SndLimTimeSnd,
            SndLimBytesSnd: f_SndLimBytesSnd,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 136);
        FromIntoMemory::into_bytes(self.State, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mss, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ConnectionTimeMs, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.TimestampsEnabled, &mut into[16..16 + 1]);
        FromIntoMemory::into_bytes(self.RttUs, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.MinRttUs, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.BytesInFlight, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.Cwnd, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.SndWnd, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.RcvWnd, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.RcvBuf, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.BytesOut, &mut into[48..48 + 8]);
        FromIntoMemory::into_bytes(self.BytesIn, &mut into[56..56 + 8]);
        FromIntoMemory::into_bytes(self.BytesReordered, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.BytesRetrans, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.FastRetrans, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.DupAcksIn, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.TimeoutEpisodes, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.SynRetrans, &mut into[84..84 + 1]);
        FromIntoMemory::into_bytes(self.SndLimTransRwin, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.SndLimTimeRwin, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.SndLimBytesRwin, &mut into[96..96 + 8]);
        FromIntoMemory::into_bytes(self.SndLimTransCwnd, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.SndLimTimeCwnd, &mut into[108..108 + 4]);
        FromIntoMemory::into_bytes(self.SndLimBytesCwnd, &mut into[112..112 + 8]);
        FromIntoMemory::into_bytes(self.SndLimTransSnd, &mut into[120..120 + 4]);
        FromIntoMemory::into_bytes(self.SndLimTimeSnd, &mut into[124..124 + 4]);
        FromIntoMemory::into_bytes(self.SndLimBytesSnd, &mut into[128..128 + 8]);
    }
    fn size() -> usize {
        136
    }
}
pub const TCP_INITIAL_RTO_DEFAULT_MAX_SYN_RETRANSMISSIONS: u32 = 0u32;
pub const TCP_INITIAL_RTO_DEFAULT_RTT: u32 = 0u32;
pub struct TCP_INITIAL_RTO_PARAMETERS {
    pub Rtt: u16,
    pub MaxSynRetransmissions: u8,
}
impl ::core::marker::Copy for TCP_INITIAL_RTO_PARAMETERS {}
impl ::core::clone::Clone for TCP_INITIAL_RTO_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_INITIAL_RTO_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_INITIAL_RTO_PARAMETERS")
            .field("Rtt", &self.Rtt)
            .field("MaxSynRetransmissions", &self.MaxSynRetransmissions)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TCP_INITIAL_RTO_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Rtt == other.Rtt && self.MaxSynRetransmissions == other.MaxSynRetransmissions
    }
}
impl ::core::cmp::Eq for TCP_INITIAL_RTO_PARAMETERS {}
impl FromIntoMemory for TCP_INITIAL_RTO_PARAMETERS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Rtt = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_MaxSynRetransmissions = <u8 as FromIntoMemory>::from_bytes(&from[2..2 + 1]);
        Self {
            Rtt: f_Rtt,
            MaxSynRetransmissions: f_MaxSynRetransmissions,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Rtt, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.MaxSynRetransmissions, &mut into[2..2 + 1]);
    }
    fn size() -> usize {
        4
    }
}
pub const TCP_KEEPALIVE: u32 = 3u32;
pub const TCP_KEEPCNT: u32 = 16u32;
pub const TCP_KEEPIDLE: u32 = 3u32;
pub const TCP_KEEPINTVL: u32 = 17u32;
pub const TCP_MAXRT: u32 = 5u32;
pub const TCP_MAXRTMS: u32 = 14u32;
pub const TCP_MAXSEG: u32 = 4u32;
pub const TCP_NODELAY: u32 = 1u32;
pub const TCP_NOSYNRETRIES: u32 = 9u32;
pub const TCP_NOURG: u32 = 7u32;
pub const TCP_OFFLOAD_NOT_PREFERRED: u32 = 1u32;
pub const TCP_OFFLOAD_NO_PREFERENCE: u32 = 0u32;
pub const TCP_OFFLOAD_PREFERENCE: u32 = 11u32;
pub const TCP_OFFLOAD_PREFERRED: u32 = 2u32;
pub const TCP_STDURG: u32 = 6u32;
pub const TCP_TIMESTAMPS: u32 = 10u32;
pub const TF_DISCONNECT: u32 = 1u32;
pub const TF_REUSE_SOCKET: u32 = 2u32;
pub const TF_USE_DEFAULT_WORKER: u32 = 0u32;
pub const TF_USE_KERNEL_APC: u32 = 32u32;
pub const TF_USE_SYSTEM_THREAD: u32 = 16u32;
pub const TF_WRITE_BEHIND: u32 = 4u32;
pub const TH_NETDEV: u32 = 1u32;
pub const TH_TAPI: u32 = 2u32;
pub struct TIMESTAMPING_CONFIG {
    pub Flags: u32,
    pub TxTimestampsBuffered: u16,
}
impl ::core::marker::Copy for TIMESTAMPING_CONFIG {}
impl ::core::clone::Clone for TIMESTAMPING_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TIMESTAMPING_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIMESTAMPING_CONFIG")
            .field("Flags", &self.Flags)
            .field("TxTimestampsBuffered", &self.TxTimestampsBuffered)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TIMESTAMPING_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.TxTimestampsBuffered == other.TxTimestampsBuffered
    }
}
impl ::core::cmp::Eq for TIMESTAMPING_CONFIG {}
impl FromIntoMemory for TIMESTAMPING_CONFIG {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_TxTimestampsBuffered = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        Self {
            Flags: f_Flags,
            TxTimestampsBuffered: f_TxTimestampsBuffered,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Flags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.TxTimestampsBuffered, &mut into[4..4 + 2]);
    }
    fn size() -> usize {
        8
    }
}
pub const TIMESTAMPING_FLAG_RX: u32 = 1u32;
pub const TIMESTAMPING_FLAG_TX: u32 = 2u32;
pub const TNS_PLAN_CARRIER_ID_CODE: u32 = 1u32;
pub const TNS_TYPE_NATIONAL: u32 = 64u32;
pub const TP_DISCONNECT: u32 = 1u32;
pub const TP_ELEMENT_EOP: u32 = 4u32;
pub const TP_ELEMENT_FILE: u32 = 2u32;
pub const TP_ELEMENT_MEMORY: u32 = 1u32;
pub const TP_REUSE_SOCKET: u32 = 2u32;
pub const TP_USE_DEFAULT_WORKER: u32 = 0u32;
pub const TP_USE_KERNEL_APC: u32 = 32u32;
pub const TP_USE_SYSTEM_THREAD: u32 = 16u32;
pub struct TRANSMIT_FILE_BUFFERS {
    pub Head: MutPtr<::core::ffi::c_void>,
    pub HeadLength: u32,
    pub Tail: MutPtr<::core::ffi::c_void>,
    pub TailLength: u32,
}
impl ::core::marker::Copy for TRANSMIT_FILE_BUFFERS {}
impl ::core::clone::Clone for TRANSMIT_FILE_BUFFERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSMIT_FILE_BUFFERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSMIT_FILE_BUFFERS")
            .field("Head", &self.Head)
            .field("HeadLength", &self.HeadLength)
            .field("Tail", &self.Tail)
            .field("TailLength", &self.TailLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRANSMIT_FILE_BUFFERS {
    fn eq(&self, other: &Self) -> bool {
        self.Head == other.Head
            && self.HeadLength == other.HeadLength
            && self.Tail == other.Tail
            && self.TailLength == other.TailLength
    }
}
impl ::core::cmp::Eq for TRANSMIT_FILE_BUFFERS {}
impl FromIntoMemory for TRANSMIT_FILE_BUFFERS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_Head = <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_HeadLength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Tail = <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_TailLength = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            Head: f_Head,
            HeadLength: f_HeadLength,
            Tail: f_Tail,
            TailLength: f_TailLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.Head, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.HeadLength, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Tail, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.TailLength, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct TRANSMIT_PACKETS_ELEMENT {
    pub dwElFlags: u32,
    pub cLength: u32,
    pub Anonymous: TRANSMIT_PACKETS_ELEMENT_0,
}
impl ::core::marker::Copy for TRANSMIT_PACKETS_ELEMENT {}
impl ::core::clone::Clone for TRANSMIT_PACKETS_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSMIT_PACKETS_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSMIT_PACKETS_ELEMENT")
            .field("dwElFlags", &self.dwElFlags)
            .field("cLength", &self.cLength)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRANSMIT_PACKETS_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.dwElFlags == other.dwElFlags
            && self.cLength == other.cLength
            && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for TRANSMIT_PACKETS_ELEMENT {}
impl FromIntoMemory for TRANSMIT_PACKETS_ELEMENT {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_dwElFlags = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_cLength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Anonymous =
            <TRANSMIT_PACKETS_ELEMENT_0 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            dwElFlags: f_dwElFlags,
            cLength: f_cLength,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.dwElFlags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.cLength, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub struct TRANSMIT_PACKETS_ELEMENT_0 {
    data: [u8; 8],
}
impl ::core::default::Default for TRANSMIT_PACKETS_ELEMENT_0 {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
impl ::core::marker::Copy for TRANSMIT_PACKETS_ELEMENT_0 {}
impl ::core::clone::Clone for TRANSMIT_PACKETS_ELEMENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSMIT_PACKETS_ELEMENT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSMIT_PACKETS_ELEMENT_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRANSMIT_PACKETS_ELEMENT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for TRANSMIT_PACKETS_ELEMENT_0 {}
impl FromIntoMemory for TRANSMIT_PACKETS_ELEMENT_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 8];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        8
    }
}
pub struct TRANSMIT_PACKETS_ELEMENT_0_0 {
    pub nFileOffset: i64,
    pub hFile: super::super::Foundation::HANDLE,
}
impl ::core::marker::Copy for TRANSMIT_PACKETS_ELEMENT_0_0 {}
impl ::core::clone::Clone for TRANSMIT_PACKETS_ELEMENT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSMIT_PACKETS_ELEMENT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSMIT_PACKETS_ELEMENT_0_0")
            .field("nFileOffset", &self.nFileOffset)
            .field("hFile", &self.hFile)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRANSMIT_PACKETS_ELEMENT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.nFileOffset == other.nFileOffset && self.hFile == other.hFile
    }
}
impl ::core::cmp::Eq for TRANSMIT_PACKETS_ELEMENT_0_0 {}
impl FromIntoMemory for TRANSMIT_PACKETS_ELEMENT_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_nFileOffset = <i64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_hFile =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            nFileOffset: f_nFileOffset,
            hFile: f_hFile,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.nFileOffset, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.hFile, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct TRANSPORT_SETTING_ID {
    pub Guid: crate::core::GUID,
}
impl ::core::marker::Copy for TRANSPORT_SETTING_ID {}
impl ::core::clone::Clone for TRANSPORT_SETTING_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSPORT_SETTING_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORT_SETTING_ID")
            .field("Guid", &self.Guid)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRANSPORT_SETTING_ID {
    fn eq(&self, other: &Self) -> bool {
        self.Guid == other.Guid
    }
}
impl ::core::cmp::Eq for TRANSPORT_SETTING_ID {}
impl FromIntoMemory for TRANSPORT_SETTING_ID {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_Guid = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        Self { Guid: f_Guid }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.Guid, &mut into[0..0 + 16]);
    }
    fn size() -> usize {
        16
    }
}
pub const TR_END_TO_END: u32 = 1u32;
pub const TR_NOIND: u32 = 0u32;
pub const TR_NO_END_TO_END: u32 = 2u32;
pub const TT_CBR: u32 = 4u32;
pub const TT_NOIND: u32 = 0u32;
pub const TT_VBR: u32 = 8u32;
pub const UDP_CHECKSUM_COVERAGE: u32 = 20u32;
pub const UDP_COALESCED_INFO: u32 = 3u32;
pub const UDP_NOCHECKSUM: u32 = 1u32;
pub const UDP_RECV_MAX_COALESCED_SIZE: u32 = 3u32;
pub const UDP_SEND_MSG_SIZE: u32 = 2u32;
pub const UNIX_PATH_MAX: u32 = 108u32;
pub const UP_P2MP: u32 = 1u32;
pub const UP_P2P: u32 = 0u32;
pub const VNSPROTO_IPC: u32 = 1u32;
pub const VNSPROTO_RELIABLE_IPC: u32 = 2u32;
pub const VNSPROTO_SPP: u32 = 3u32;
pub const WCE_AF_IRDA: u32 = 22u32;
pub struct WCE_DEVICELIST {
    pub numDevice: u32,
    pub Device: [WCE_IRDA_DEVICE_INFO; 1],
}
impl ::core::marker::Copy for WCE_DEVICELIST {}
impl ::core::clone::Clone for WCE_DEVICELIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WCE_DEVICELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCE_DEVICELIST")
            .field("numDevice", &self.numDevice)
            .field("Device", &self.Device)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WCE_DEVICELIST {
    fn eq(&self, other: &Self) -> bool {
        self.numDevice == other.numDevice && self.Device == other.Device
    }
}
impl ::core::cmp::Eq for WCE_DEVICELIST {}
impl FromIntoMemory for WCE_DEVICELIST {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_numDevice = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Device = <[WCE_IRDA_DEVICE_INFO; 1] as FromIntoMemory>::from_bytes(&from[4..4 + 28]);
        Self {
            numDevice: f_numDevice,
            Device: f_Device,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.numDevice, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Device, &mut into[4..4 + 28]);
    }
    fn size() -> usize {
        32
    }
}
pub struct WCE_IRDA_DEVICE_INFO {
    pub irdaDeviceID: [u8; 4],
    pub irdaDeviceName: [super::super::Foundation::CHAR; 22],
    pub Reserved: [u8; 2],
}
impl ::core::marker::Copy for WCE_IRDA_DEVICE_INFO {}
impl ::core::clone::Clone for WCE_IRDA_DEVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WCE_IRDA_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCE_IRDA_DEVICE_INFO")
            .field("irdaDeviceID", &self.irdaDeviceID)
            .field("irdaDeviceName", &self.irdaDeviceName)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WCE_IRDA_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.irdaDeviceID == other.irdaDeviceID
            && self.irdaDeviceName == other.irdaDeviceName
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WCE_IRDA_DEVICE_INFO {}
impl FromIntoMemory for WCE_IRDA_DEVICE_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_irdaDeviceID = <[u8; 4] as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_irdaDeviceName =
            <[super::super::Foundation::CHAR; 22] as FromIntoMemory>::from_bytes(&from[4..4 + 22]);
        let f_Reserved = <[u8; 2] as FromIntoMemory>::from_bytes(&from[26..26 + 2]);
        Self {
            irdaDeviceID: f_irdaDeviceID,
            irdaDeviceName: f_irdaDeviceName,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.irdaDeviceID, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.irdaDeviceName, &mut into[4..4 + 22]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[26..26 + 2]);
    }
    fn size() -> usize {
        28
    }
}
pub const WCE_PF_IRDA: u32 = 22u32;
pub const WINDOWS_AF_IRDA: u32 = 26u32;
pub struct WINDOWS_DEVICELIST {
    pub numDevice: u32,
    pub Device: [WINDOWS_IRDA_DEVICE_INFO; 1],
}
impl ::core::marker::Copy for WINDOWS_DEVICELIST {}
impl ::core::clone::Clone for WINDOWS_DEVICELIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOWS_DEVICELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWS_DEVICELIST")
            .field("numDevice", &self.numDevice)
            .field("Device", &self.Device)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINDOWS_DEVICELIST {
    fn eq(&self, other: &Self) -> bool {
        self.numDevice == other.numDevice && self.Device == other.Device
    }
}
impl ::core::cmp::Eq for WINDOWS_DEVICELIST {}
impl FromIntoMemory for WINDOWS_DEVICELIST {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 36);
        let f_numDevice = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Device =
            <[WINDOWS_IRDA_DEVICE_INFO; 1] as FromIntoMemory>::from_bytes(&from[4..4 + 29]);
        Self {
            numDevice: f_numDevice,
            Device: f_Device,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 36);
        FromIntoMemory::into_bytes(self.numDevice, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Device, &mut into[4..4 + 29]);
    }
    fn size() -> usize {
        36
    }
}
pub struct WINDOWS_IAS_QUERY {
    pub irdaDeviceID: [u8; 4],
    pub irdaClassName: [super::super::Foundation::CHAR; 64],
    pub irdaAttribName: [super::super::Foundation::CHAR; 256],
    pub irdaAttribType: u32,
    pub irdaAttribute: WINDOWS_IAS_QUERY_0,
}
impl ::core::marker::Copy for WINDOWS_IAS_QUERY {}
impl ::core::clone::Clone for WINDOWS_IAS_QUERY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOWS_IAS_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWS_IAS_QUERY")
            .field("irdaDeviceID", &self.irdaDeviceID)
            .field("irdaClassName", &self.irdaClassName)
            .field("irdaAttribName", &self.irdaAttribName)
            .field("irdaAttribType", &self.irdaAttribType)
            .field("irdaAttribute", &self.irdaAttribute)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINDOWS_IAS_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.irdaDeviceID == other.irdaDeviceID
            && self.irdaClassName == other.irdaClassName
            && self.irdaAttribName == other.irdaAttribName
            && self.irdaAttribType == other.irdaAttribType
            && self.irdaAttribute == other.irdaAttribute
    }
}
impl ::core::cmp::Eq for WINDOWS_IAS_QUERY {}
impl FromIntoMemory for WINDOWS_IAS_QUERY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 592);
        let f_irdaDeviceID = <[u8; 4] as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_irdaClassName =
            <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(&from[4..4 + 64]);
        let f_irdaAttribName =
            <[super::super::Foundation::CHAR; 256] as FromIntoMemory>::from_bytes(
                &from[68..68 + 256],
            );
        let f_irdaAttribType = <u32 as FromIntoMemory>::from_bytes(&from[324..324 + 4]);
        let f_irdaAttribute =
            <WINDOWS_IAS_QUERY_0 as FromIntoMemory>::from_bytes(&from[328..328 + 264]);
        Self {
            irdaDeviceID: f_irdaDeviceID,
            irdaClassName: f_irdaClassName,
            irdaAttribName: f_irdaAttribName,
            irdaAttribType: f_irdaAttribType,
            irdaAttribute: f_irdaAttribute,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 592);
        FromIntoMemory::into_bytes(self.irdaDeviceID, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.irdaClassName, &mut into[4..4 + 64]);
        FromIntoMemory::into_bytes(self.irdaAttribName, &mut into[68..68 + 256]);
        FromIntoMemory::into_bytes(self.irdaAttribType, &mut into[324..324 + 4]);
        FromIntoMemory::into_bytes(self.irdaAttribute, &mut into[328..328 + 264]);
    }
    fn size() -> usize {
        592
    }
}
pub struct WINDOWS_IAS_QUERY_0 {
    data: [u8; 264],
}
impl ::core::default::Default for WINDOWS_IAS_QUERY_0 {
    fn default() -> Self {
        Self { data: [0u8; 264] }
    }
}
impl ::core::marker::Copy for WINDOWS_IAS_QUERY_0 {}
impl ::core::clone::Clone for WINDOWS_IAS_QUERY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOWS_IAS_QUERY_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWS_IAS_QUERY_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINDOWS_IAS_QUERY_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for WINDOWS_IAS_QUERY_0 {}
impl FromIntoMemory for WINDOWS_IAS_QUERY_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 264];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        264
    }
}
pub struct WINDOWS_IAS_QUERY_0_0 {
    pub Len: u32,
    pub OctetSeq: [u8; 1024],
}
impl ::core::marker::Copy for WINDOWS_IAS_QUERY_0_0 {}
impl ::core::clone::Clone for WINDOWS_IAS_QUERY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOWS_IAS_QUERY_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWS_IAS_QUERY_0_0")
            .field("Len", &self.Len)
            .field("OctetSeq", &self.OctetSeq)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINDOWS_IAS_QUERY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Len == other.Len && self.OctetSeq == other.OctetSeq
    }
}
impl ::core::cmp::Eq for WINDOWS_IAS_QUERY_0_0 {}
impl FromIntoMemory for WINDOWS_IAS_QUERY_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 1028);
        let f_Len = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_OctetSeq = <[u8; 1024] as FromIntoMemory>::from_bytes(&from[4..4 + 1024]);
        Self {
            Len: f_Len,
            OctetSeq: f_OctetSeq,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 1028);
        FromIntoMemory::into_bytes(self.Len, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.OctetSeq, &mut into[4..4 + 1024]);
    }
    fn size() -> usize {
        1028
    }
}
pub struct WINDOWS_IAS_QUERY_0_1 {
    pub Len: u32,
    pub CharSet: u32,
    pub UsrStr: [u8; 256],
}
impl ::core::marker::Copy for WINDOWS_IAS_QUERY_0_1 {}
impl ::core::clone::Clone for WINDOWS_IAS_QUERY_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOWS_IAS_QUERY_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWS_IAS_QUERY_0_1")
            .field("Len", &self.Len)
            .field("CharSet", &self.CharSet)
            .field("UsrStr", &self.UsrStr)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINDOWS_IAS_QUERY_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Len == other.Len && self.CharSet == other.CharSet && self.UsrStr == other.UsrStr
    }
}
impl ::core::cmp::Eq for WINDOWS_IAS_QUERY_0_1 {}
impl FromIntoMemory for WINDOWS_IAS_QUERY_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 264);
        let f_Len = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_CharSet = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_UsrStr = <[u8; 256] as FromIntoMemory>::from_bytes(&from[8..8 + 256]);
        Self {
            Len: f_Len,
            CharSet: f_CharSet,
            UsrStr: f_UsrStr,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 264);
        FromIntoMemory::into_bytes(self.Len, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.CharSet, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.UsrStr, &mut into[8..8 + 256]);
    }
    fn size() -> usize {
        264
    }
}
pub struct WINDOWS_IAS_SET {
    pub irdaClassName: [super::super::Foundation::CHAR; 64],
    pub irdaAttribName: [super::super::Foundation::CHAR; 256],
    pub irdaAttribType: u32,
    pub irdaAttribute: WINDOWS_IAS_SET_0,
}
impl ::core::marker::Copy for WINDOWS_IAS_SET {}
impl ::core::clone::Clone for WINDOWS_IAS_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOWS_IAS_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWS_IAS_SET")
            .field("irdaClassName", &self.irdaClassName)
            .field("irdaAttribName", &self.irdaAttribName)
            .field("irdaAttribType", &self.irdaAttribType)
            .field("irdaAttribute", &self.irdaAttribute)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINDOWS_IAS_SET {
    fn eq(&self, other: &Self) -> bool {
        self.irdaClassName == other.irdaClassName
            && self.irdaAttribName == other.irdaAttribName
            && self.irdaAttribType == other.irdaAttribType
            && self.irdaAttribute == other.irdaAttribute
    }
}
impl ::core::cmp::Eq for WINDOWS_IAS_SET {}
impl FromIntoMemory for WINDOWS_IAS_SET {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 584);
        let f_irdaClassName =
            <[super::super::Foundation::CHAR; 64] as FromIntoMemory>::from_bytes(&from[0..0 + 64]);
        let f_irdaAttribName =
            <[super::super::Foundation::CHAR; 256] as FromIntoMemory>::from_bytes(
                &from[64..64 + 256],
            );
        let f_irdaAttribType = <u32 as FromIntoMemory>::from_bytes(&from[320..320 + 4]);
        let f_irdaAttribute =
            <WINDOWS_IAS_SET_0 as FromIntoMemory>::from_bytes(&from[324..324 + 260]);
        Self {
            irdaClassName: f_irdaClassName,
            irdaAttribName: f_irdaAttribName,
            irdaAttribType: f_irdaAttribType,
            irdaAttribute: f_irdaAttribute,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 584);
        FromIntoMemory::into_bytes(self.irdaClassName, &mut into[0..0 + 64]);
        FromIntoMemory::into_bytes(self.irdaAttribName, &mut into[64..64 + 256]);
        FromIntoMemory::into_bytes(self.irdaAttribType, &mut into[320..320 + 4]);
        FromIntoMemory::into_bytes(self.irdaAttribute, &mut into[324..324 + 260]);
    }
    fn size() -> usize {
        584
    }
}
pub struct WINDOWS_IAS_SET_0 {
    data: [u8; 260],
}
impl ::core::default::Default for WINDOWS_IAS_SET_0 {
    fn default() -> Self {
        Self { data: [0u8; 260] }
    }
}
impl ::core::marker::Copy for WINDOWS_IAS_SET_0 {}
impl ::core::clone::Clone for WINDOWS_IAS_SET_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOWS_IAS_SET_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWS_IAS_SET_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINDOWS_IAS_SET_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for WINDOWS_IAS_SET_0 {}
impl FromIntoMemory for WINDOWS_IAS_SET_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 260];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        260
    }
}
pub struct WINDOWS_IAS_SET_0_0 {
    pub Len: u16,
    pub OctetSeq: [u8; 1024],
}
impl ::core::marker::Copy for WINDOWS_IAS_SET_0_0 {}
impl ::core::clone::Clone for WINDOWS_IAS_SET_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOWS_IAS_SET_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWS_IAS_SET_0_0")
            .field("Len", &self.Len)
            .field("OctetSeq", &self.OctetSeq)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINDOWS_IAS_SET_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Len == other.Len && self.OctetSeq == other.OctetSeq
    }
}
impl ::core::cmp::Eq for WINDOWS_IAS_SET_0_0 {}
impl FromIntoMemory for WINDOWS_IAS_SET_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 1026);
        let f_Len = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_OctetSeq = <[u8; 1024] as FromIntoMemory>::from_bytes(&from[2..2 + 1024]);
        Self {
            Len: f_Len,
            OctetSeq: f_OctetSeq,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 1026);
        FromIntoMemory::into_bytes(self.Len, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.OctetSeq, &mut into[2..2 + 1024]);
    }
    fn size() -> usize {
        1026
    }
}
pub struct WINDOWS_IAS_SET_0_1 {
    pub Len: u8,
    pub CharSet: u8,
    pub UsrStr: [u8; 256],
}
impl ::core::marker::Copy for WINDOWS_IAS_SET_0_1 {}
impl ::core::clone::Clone for WINDOWS_IAS_SET_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOWS_IAS_SET_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWS_IAS_SET_0_1")
            .field("Len", &self.Len)
            .field("CharSet", &self.CharSet)
            .field("UsrStr", &self.UsrStr)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINDOWS_IAS_SET_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Len == other.Len && self.CharSet == other.CharSet && self.UsrStr == other.UsrStr
    }
}
impl ::core::cmp::Eq for WINDOWS_IAS_SET_0_1 {}
impl FromIntoMemory for WINDOWS_IAS_SET_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 258);
        let f_Len = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_CharSet = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_UsrStr = <[u8; 256] as FromIntoMemory>::from_bytes(&from[2..2 + 256]);
        Self {
            Len: f_Len,
            CharSet: f_CharSet,
            UsrStr: f_UsrStr,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 258);
        FromIntoMemory::into_bytes(self.Len, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.CharSet, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.UsrStr, &mut into[2..2 + 256]);
    }
    fn size() -> usize {
        258
    }
}
pub struct WINDOWS_IRDA_DEVICE_INFO {
    pub irdaDeviceID: [u8; 4],
    pub irdaDeviceName: [super::super::Foundation::CHAR; 22],
    pub irdaDeviceHints1: u8,
    pub irdaDeviceHints2: u8,
    pub irdaCharSet: u8,
}
impl ::core::marker::Copy for WINDOWS_IRDA_DEVICE_INFO {}
impl ::core::clone::Clone for WINDOWS_IRDA_DEVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOWS_IRDA_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWS_IRDA_DEVICE_INFO")
            .field("irdaDeviceID", &self.irdaDeviceID)
            .field("irdaDeviceName", &self.irdaDeviceName)
            .field("irdaDeviceHints1", &self.irdaDeviceHints1)
            .field("irdaDeviceHints2", &self.irdaDeviceHints2)
            .field("irdaCharSet", &self.irdaCharSet)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WINDOWS_IRDA_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.irdaDeviceID == other.irdaDeviceID
            && self.irdaDeviceName == other.irdaDeviceName
            && self.irdaDeviceHints1 == other.irdaDeviceHints1
            && self.irdaDeviceHints2 == other.irdaDeviceHints2
            && self.irdaCharSet == other.irdaCharSet
    }
}
impl ::core::cmp::Eq for WINDOWS_IRDA_DEVICE_INFO {}
impl FromIntoMemory for WINDOWS_IRDA_DEVICE_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 29);
        let f_irdaDeviceID = <[u8; 4] as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_irdaDeviceName =
            <[super::super::Foundation::CHAR; 22] as FromIntoMemory>::from_bytes(&from[4..4 + 22]);
        let f_irdaDeviceHints1 = <u8 as FromIntoMemory>::from_bytes(&from[26..26 + 1]);
        let f_irdaDeviceHints2 = <u8 as FromIntoMemory>::from_bytes(&from[27..27 + 1]);
        let f_irdaCharSet = <u8 as FromIntoMemory>::from_bytes(&from[28..28 + 1]);
        Self {
            irdaDeviceID: f_irdaDeviceID,
            irdaDeviceName: f_irdaDeviceName,
            irdaDeviceHints1: f_irdaDeviceHints1,
            irdaDeviceHints2: f_irdaDeviceHints2,
            irdaCharSet: f_irdaCharSet,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 29);
        FromIntoMemory::into_bytes(self.irdaDeviceID, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.irdaDeviceName, &mut into[4..4 + 22]);
        FromIntoMemory::into_bytes(self.irdaDeviceHints1, &mut into[26..26 + 1]);
        FromIntoMemory::into_bytes(self.irdaDeviceHints2, &mut into[27..27 + 1]);
        FromIntoMemory::into_bytes(self.irdaCharSet, &mut into[28..28 + 1]);
    }
    fn size() -> usize {
        29
    }
}
pub const WINDOWS_PF_IRDA: u32 = 26u32;
pub struct WSABUF {
    pub len: u32,
    pub buf: PSTR,
}
impl ::core::marker::Copy for WSABUF {}
impl ::core::clone::Clone for WSABUF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSABUF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSABUF")
            .field("len", &self.len)
            .field("buf", &self.buf)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSABUF {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.buf == other.buf
    }
}
impl ::core::cmp::Eq for WSABUF {}
impl FromIntoMemory for WSABUF {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_len = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_buf = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            len: f_len,
            buf: f_buf,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.len, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.buf, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct WSACOMPLETION {
    pub Type: WSACOMPLETIONTYPE,
    pub Parameters: WSACOMPLETION_0,
}
impl ::core::marker::Copy for WSACOMPLETION {}
impl ::core::clone::Clone for WSACOMPLETION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSACOMPLETION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSACOMPLETION")
            .field("Type", &self.Type)
            .field("Parameters", &self.Parameters)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSACOMPLETION {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Parameters == other.Parameters
    }
}
impl ::core::cmp::Eq for WSACOMPLETION {}
impl FromIntoMemory for WSACOMPLETION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_Type = <WSACOMPLETIONTYPE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Parameters = <WSACOMPLETION_0 as FromIntoMemory>::from_bytes(&from[4..4 + 12]);
        Self {
            Type: f_Type,
            Parameters: f_Parameters,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.Type, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Parameters, &mut into[4..4 + 12]);
    }
    fn size() -> usize {
        16
    }
}
pub struct WSACOMPLETION_0 {
    data: [u8; 12],
}
impl ::core::default::Default for WSACOMPLETION_0 {
    fn default() -> Self {
        Self { data: [0u8; 12] }
    }
}
impl ::core::marker::Copy for WSACOMPLETION_0 {}
impl ::core::clone::Clone for WSACOMPLETION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSACOMPLETION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSACOMPLETION_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSACOMPLETION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for WSACOMPLETION_0 {}
impl FromIntoMemory for WSACOMPLETION_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 12];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        12
    }
}
pub struct WSACOMPLETION_0_0 {
    pub lpOverlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
    pub lpfnCompletionProc: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
}
impl ::core::marker::Copy for WSACOMPLETION_0_0 {}
impl ::core::clone::Clone for WSACOMPLETION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSACOMPLETION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSACOMPLETION_0_0")
            .field("lpOverlapped", &self.lpOverlapped)
            .field("lpfnCompletionProc", &self.lpfnCompletionProc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSACOMPLETION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.lpOverlapped == other.lpOverlapped
            && self.lpfnCompletionProc == other.lpfnCompletionProc
    }
}
impl ::core::cmp::Eq for WSACOMPLETION_0_0 {}
impl FromIntoMemory for WSACOMPLETION_0_0 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_lpOverlapped =
            <MutPtr<super::super::System::IO::OVERLAPPED> as FromIntoMemory>::from_bytes(
                &from[0..0 + 4],
            );
        let f_lpfnCompletionProc =
            <LPWSAOVERLAPPED_COMPLETION_ROUTINE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            lpOverlapped: f_lpOverlapped,
            lpfnCompletionProc: f_lpfnCompletionProc,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.lpOverlapped, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpfnCompletionProc, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct WSACOMPLETION_0_1 {
    pub lpOverlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
}
impl ::core::marker::Copy for WSACOMPLETION_0_1 {}
impl ::core::clone::Clone for WSACOMPLETION_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSACOMPLETION_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSACOMPLETION_0_1")
            .field("lpOverlapped", &self.lpOverlapped)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSACOMPLETION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.lpOverlapped == other.lpOverlapped
    }
}
impl ::core::cmp::Eq for WSACOMPLETION_0_1 {}
impl FromIntoMemory for WSACOMPLETION_0_1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_lpOverlapped =
            <MutPtr<super::super::System::IO::OVERLAPPED> as FromIntoMemory>::from_bytes(
                &from[0..0 + 4],
            );
        Self {
            lpOverlapped: f_lpOverlapped,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.lpOverlapped, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct WSACOMPLETION_0_2 {
    pub lpOverlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
    pub hPort: super::super::Foundation::HANDLE,
    pub Key: PtrRepr,
}
impl ::core::marker::Copy for WSACOMPLETION_0_2 {}
impl ::core::clone::Clone for WSACOMPLETION_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSACOMPLETION_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSACOMPLETION_0_2")
            .field("lpOverlapped", &self.lpOverlapped)
            .field("hPort", &self.hPort)
            .field("Key", &self.Key)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSACOMPLETION_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.lpOverlapped == other.lpOverlapped
            && self.hPort == other.hPort
            && self.Key == other.Key
    }
}
impl ::core::cmp::Eq for WSACOMPLETION_0_2 {}
impl FromIntoMemory for WSACOMPLETION_0_2 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_lpOverlapped =
            <MutPtr<super::super::System::IO::OVERLAPPED> as FromIntoMemory>::from_bytes(
                &from[0..0 + 4],
            );
        let f_hPort =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Key = <PtrRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            lpOverlapped: f_lpOverlapped,
            hPort: f_hPort,
            Key: f_Key,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.lpOverlapped, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.hPort, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Key, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct WSACOMPLETION_0_3 {
    pub hWnd: super::super::Foundation::HWND,
    pub uMsg: u32,
    pub context: super::super::Foundation::WPARAM,
}
impl ::core::marker::Copy for WSACOMPLETION_0_3 {}
impl ::core::clone::Clone for WSACOMPLETION_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSACOMPLETION_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSACOMPLETION_0_3")
            .field("hWnd", &self.hWnd)
            .field("uMsg", &self.uMsg)
            .field("context", &self.context)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSACOMPLETION_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.hWnd == other.hWnd && self.uMsg == other.uMsg && self.context == other.context
    }
}
impl ::core::cmp::Eq for WSACOMPLETION_0_3 {}
impl FromIntoMemory for WSACOMPLETION_0_3 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_hWnd =
            <super::super::Foundation::HWND as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_uMsg = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_context =
            <super::super::Foundation::WPARAM as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            hWnd: f_hWnd,
            uMsg: f_uMsg,
            context: f_context,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.hWnd, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.uMsg, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.context, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSACOMPLETIONTYPE(pub i32);
pub const NSP_NOTIFY_IMMEDIATELY: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(0i32);
pub const NSP_NOTIFY_HWND: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(1i32);
pub const NSP_NOTIFY_EVENT: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(2i32);
pub const NSP_NOTIFY_PORT: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(3i32);
pub const NSP_NOTIFY_APC: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(4i32);
impl ::core::marker::Copy for WSACOMPLETIONTYPE {}
impl ::core::clone::Clone for WSACOMPLETIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSACOMPLETIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSACOMPLETIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSACOMPLETIONTYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for WSACOMPLETIONTYPE {
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
pub const WSADESCRIPTION_LEN: u32 = 256u32;
#[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct WSAData {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: PSTR,
    pub szDescription: [super::super::Foundation::CHAR; 257],
    pub szSystemStatus: [super::super::Foundation::CHAR; 129],
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for WSAData {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for WSAData {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for WSAData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAData")
            .field("wVersion", &self.wVersion)
            .field("wHighVersion", &self.wHighVersion)
            .field("iMaxSockets", &self.iMaxSockets)
            .field("iMaxUdpDg", &self.iMaxUdpDg)
            .field("lpVendorInfo", &self.lpVendorInfo)
            .field("szDescription", &self.szDescription)
            .field("szSystemStatus", &self.szSystemStatus)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for WSAData {
    fn eq(&self, other: &Self) -> bool {
        self.wVersion == other.wVersion
            && self.wHighVersion == other.wHighVersion
            && self.iMaxSockets == other.iMaxSockets
            && self.iMaxUdpDg == other.iMaxUdpDg
            && self.lpVendorInfo == other.lpVendorInfo
            && self.szDescription == other.szDescription
            && self.szSystemStatus == other.szSystemStatus
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for WSAData {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for WSAData {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 400);
        let f_wVersion = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wHighVersion = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_iMaxSockets = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_iMaxUdpDg = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_lpVendorInfo = <PSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_szDescription = <[super::super::Foundation::CHAR; 257] as FromIntoMemory>::from_bytes(
            &from[12..12 + 257],
        );
        let f_szSystemStatus =
            <[super::super::Foundation::CHAR; 129] as FromIntoMemory>::from_bytes(
                &from[269..269 + 129],
            );
        Self {
            wVersion: f_wVersion,
            wHighVersion: f_wHighVersion,
            iMaxSockets: f_iMaxSockets,
            iMaxUdpDg: f_iMaxUdpDg,
            lpVendorInfo: f_lpVendorInfo,
            szDescription: f_szDescription,
            szSystemStatus: f_szSystemStatus,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 400);
        FromIntoMemory::into_bytes(self.wVersion, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wHighVersion, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.iMaxSockets, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.iMaxUdpDg, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.lpVendorInfo, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.szDescription, &mut into[12..12 + 257]);
        FromIntoMemory::into_bytes(self.szSystemStatus, &mut into[269..269 + 129]);
    }
    fn size() -> usize {
        400
    }
}
pub struct WSAData {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub szDescription: [super::super::Foundation::CHAR; 257],
    pub szSystemStatus: [super::super::Foundation::CHAR; 129],
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: PSTR,
}
impl ::core::marker::Copy for WSAData {}
impl ::core::clone::Clone for WSAData {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSAData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAData")
            .field("wVersion", &self.wVersion)
            .field("wHighVersion", &self.wHighVersion)
            .field("szDescription", &self.szDescription)
            .field("szSystemStatus", &self.szSystemStatus)
            .field("iMaxSockets", &self.iMaxSockets)
            .field("iMaxUdpDg", &self.iMaxUdpDg)
            .field("lpVendorInfo", &self.lpVendorInfo)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSAData {
    fn eq(&self, other: &Self) -> bool {
        self.wVersion == other.wVersion
            && self.wHighVersion == other.wHighVersion
            && self.szDescription == other.szDescription
            && self.szSystemStatus == other.szSystemStatus
            && self.iMaxSockets == other.iMaxSockets
            && self.iMaxUdpDg == other.iMaxUdpDg
            && self.lpVendorInfo == other.lpVendorInfo
    }
}
impl ::core::cmp::Eq for WSAData {}
impl FromIntoMemory for WSAData {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 400);
        let f_wVersion = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wHighVersion = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_szDescription = <[super::super::Foundation::CHAR; 257] as FromIntoMemory>::from_bytes(
            &from[4..4 + 257],
        );
        let f_szSystemStatus =
            <[super::super::Foundation::CHAR; 129] as FromIntoMemory>::from_bytes(
                &from[261..261 + 129],
            );
        let f_iMaxSockets = <u16 as FromIntoMemory>::from_bytes(&from[390..390 + 2]);
        let f_iMaxUdpDg = <u16 as FromIntoMemory>::from_bytes(&from[392..392 + 2]);
        let f_lpVendorInfo = <PSTR as FromIntoMemory>::from_bytes(&from[396..396 + 4]);
        Self {
            wVersion: f_wVersion,
            wHighVersion: f_wHighVersion,
            szDescription: f_szDescription,
            szSystemStatus: f_szSystemStatus,
            iMaxSockets: f_iMaxSockets,
            iMaxUdpDg: f_iMaxUdpDg,
            lpVendorInfo: f_lpVendorInfo,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 400);
        FromIntoMemory::into_bytes(self.wVersion, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wHighVersion, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.szDescription, &mut into[4..4 + 257]);
        FromIntoMemory::into_bytes(self.szSystemStatus, &mut into[261..261 + 129]);
        FromIntoMemory::into_bytes(self.iMaxSockets, &mut into[390..390 + 2]);
        FromIntoMemory::into_bytes(self.iMaxUdpDg, &mut into[392..392 + 2]);
        FromIntoMemory::into_bytes(self.lpVendorInfo, &mut into[396..396 + 4]);
    }
    fn size() -> usize {
        400
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSAECOMPARATOR(pub i32);
pub const COMP_EQUAL: WSAECOMPARATOR = WSAECOMPARATOR(0i32);
pub const COMP_NOTLESS: WSAECOMPARATOR = WSAECOMPARATOR(1i32);
impl ::core::marker::Copy for WSAECOMPARATOR {}
impl ::core::clone::Clone for WSAECOMPARATOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSAECOMPARATOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSAECOMPARATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSAECOMPARATOR").field(&self.0).finish()
    }
}
impl FromIntoMemory for WSAECOMPARATOR {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSAESETSERVICEOP(pub i32);
pub const RNRSERVICE_REGISTER: WSAESETSERVICEOP = WSAESETSERVICEOP(0i32);
pub const RNRSERVICE_DEREGISTER: WSAESETSERVICEOP = WSAESETSERVICEOP(1i32);
pub const RNRSERVICE_DELETE: WSAESETSERVICEOP = WSAESETSERVICEOP(2i32);
impl ::core::marker::Copy for WSAESETSERVICEOP {}
impl ::core::clone::Clone for WSAESETSERVICEOP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSAESETSERVICEOP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSAESETSERVICEOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSAESETSERVICEOP").field(&self.0).finish()
    }
}
impl FromIntoMemory for WSAESETSERVICEOP {
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
pub struct WSAMSG {
    pub name: MutPtr<SOCKADDR>,
    pub namelen: i32,
    pub lpBuffers: MutPtr<WSABUF>,
    pub dwBufferCount: u32,
    pub Control: WSABUF,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for WSAMSG {}
impl ::core::clone::Clone for WSAMSG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSAMSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAMSG")
            .field("name", &self.name)
            .field("namelen", &self.namelen)
            .field("lpBuffers", &self.lpBuffers)
            .field("dwBufferCount", &self.dwBufferCount)
            .field("Control", &self.Control)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSAMSG {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.namelen == other.namelen
            && self.lpBuffers == other.lpBuffers
            && self.dwBufferCount == other.dwBufferCount
            && self.Control == other.Control
            && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for WSAMSG {}
impl FromIntoMemory for WSAMSG {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 28);
        let f_name = <MutPtr<SOCKADDR> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_namelen = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpBuffers = <MutPtr<WSABUF> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwBufferCount = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Control = <WSABUF as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            name: f_name,
            namelen: f_namelen,
            lpBuffers: f_lpBuffers,
            dwBufferCount: f_dwBufferCount,
            Control: f_Control,
            dwFlags: f_dwFlags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 28);
        FromIntoMemory::into_bytes(self.name, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.namelen, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpBuffers, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwBufferCount, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Control, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        28
    }
}
pub struct WSANAMESPACE_INFOA {
    pub NSProviderId: crate::core::GUID,
    pub dwNameSpace: u32,
    pub fActive: super::super::Foundation::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: PSTR,
}
impl ::core::marker::Copy for WSANAMESPACE_INFOA {}
impl ::core::clone::Clone for WSANAMESPACE_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSANAMESPACE_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSANAMESPACE_INFOA")
            .field("NSProviderId", &self.NSProviderId)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("fActive", &self.fActive)
            .field("dwVersion", &self.dwVersion)
            .field("lpszIdentifier", &self.lpszIdentifier)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSANAMESPACE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.NSProviderId == other.NSProviderId
            && self.dwNameSpace == other.dwNameSpace
            && self.fActive == other.fActive
            && self.dwVersion == other.dwVersion
            && self.lpszIdentifier == other.lpszIdentifier
    }
}
impl ::core::cmp::Eq for WSANAMESPACE_INFOA {}
impl FromIntoMemory for WSANAMESPACE_INFOA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_NSProviderId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_fActive =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lpszIdentifier = <PSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            NSProviderId: f_NSProviderId,
            dwNameSpace: f_dwNameSpace,
            fActive: f_fActive,
            dwVersion: f_dwVersion,
            lpszIdentifier: f_lpszIdentifier,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.NSProviderId, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.fActive, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpszIdentifier, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct WSANAMESPACE_INFOEXA {
    pub NSProviderId: crate::core::GUID,
    pub dwNameSpace: u32,
    pub fActive: super::super::Foundation::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: PSTR,
    pub ProviderSpecific: super::super::System::Com::BLOB,
}
impl ::core::marker::Copy for WSANAMESPACE_INFOEXA {}
impl ::core::clone::Clone for WSANAMESPACE_INFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSANAMESPACE_INFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSANAMESPACE_INFOEXA")
            .field("NSProviderId", &self.NSProviderId)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("fActive", &self.fActive)
            .field("dwVersion", &self.dwVersion)
            .field("lpszIdentifier", &self.lpszIdentifier)
            .field("ProviderSpecific", &self.ProviderSpecific)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSANAMESPACE_INFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.NSProviderId == other.NSProviderId
            && self.dwNameSpace == other.dwNameSpace
            && self.fActive == other.fActive
            && self.dwVersion == other.dwVersion
            && self.lpszIdentifier == other.lpszIdentifier
            && self.ProviderSpecific == other.ProviderSpecific
    }
}
impl ::core::cmp::Eq for WSANAMESPACE_INFOEXA {}
impl FromIntoMemory for WSANAMESPACE_INFOEXA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_NSProviderId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_fActive =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lpszIdentifier = <PSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_ProviderSpecific =
            <super::super::System::Com::BLOB as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        Self {
            NSProviderId: f_NSProviderId,
            dwNameSpace: f_dwNameSpace,
            fActive: f_fActive,
            dwVersion: f_dwVersion,
            lpszIdentifier: f_lpszIdentifier,
            ProviderSpecific: f_ProviderSpecific,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.NSProviderId, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.fActive, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpszIdentifier, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ProviderSpecific, &mut into[32..32 + 8]);
    }
    fn size() -> usize {
        40
    }
}
pub struct WSANAMESPACE_INFOEXW {
    pub NSProviderId: crate::core::GUID,
    pub dwNameSpace: u32,
    pub fActive: super::super::Foundation::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: PWSTR,
    pub ProviderSpecific: super::super::System::Com::BLOB,
}
impl ::core::marker::Copy for WSANAMESPACE_INFOEXW {}
impl ::core::clone::Clone for WSANAMESPACE_INFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSANAMESPACE_INFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSANAMESPACE_INFOEXW")
            .field("NSProviderId", &self.NSProviderId)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("fActive", &self.fActive)
            .field("dwVersion", &self.dwVersion)
            .field("lpszIdentifier", &self.lpszIdentifier)
            .field("ProviderSpecific", &self.ProviderSpecific)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSANAMESPACE_INFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.NSProviderId == other.NSProviderId
            && self.dwNameSpace == other.dwNameSpace
            && self.fActive == other.fActive
            && self.dwVersion == other.dwVersion
            && self.lpszIdentifier == other.lpszIdentifier
            && self.ProviderSpecific == other.ProviderSpecific
    }
}
impl ::core::cmp::Eq for WSANAMESPACE_INFOEXW {}
impl FromIntoMemory for WSANAMESPACE_INFOEXW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_NSProviderId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_fActive =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lpszIdentifier = <PWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_ProviderSpecific =
            <super::super::System::Com::BLOB as FromIntoMemory>::from_bytes(&from[32..32 + 8]);
        Self {
            NSProviderId: f_NSProviderId,
            dwNameSpace: f_dwNameSpace,
            fActive: f_fActive,
            dwVersion: f_dwVersion,
            lpszIdentifier: f_lpszIdentifier,
            ProviderSpecific: f_ProviderSpecific,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.NSProviderId, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.fActive, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpszIdentifier, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ProviderSpecific, &mut into[32..32 + 8]);
    }
    fn size() -> usize {
        40
    }
}
pub struct WSANAMESPACE_INFOW {
    pub NSProviderId: crate::core::GUID,
    pub dwNameSpace: u32,
    pub fActive: super::super::Foundation::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: PWSTR,
}
impl ::core::marker::Copy for WSANAMESPACE_INFOW {}
impl ::core::clone::Clone for WSANAMESPACE_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSANAMESPACE_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSANAMESPACE_INFOW")
            .field("NSProviderId", &self.NSProviderId)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("fActive", &self.fActive)
            .field("dwVersion", &self.dwVersion)
            .field("lpszIdentifier", &self.lpszIdentifier)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSANAMESPACE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.NSProviderId == other.NSProviderId
            && self.dwNameSpace == other.dwNameSpace
            && self.fActive == other.fActive
            && self.dwVersion == other.dwVersion
            && self.lpszIdentifier == other.lpszIdentifier
    }
}
impl ::core::cmp::Eq for WSANAMESPACE_INFOW {}
impl FromIntoMemory for WSANAMESPACE_INFOW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_NSProviderId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[0..0 + 16]);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_fActive =
            <super::super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lpszIdentifier = <PWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            NSProviderId: f_NSProviderId,
            dwNameSpace: f_dwNameSpace,
            fActive: f_fActive,
            dwVersion: f_dwVersion,
            lpszIdentifier: f_lpszIdentifier,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.NSProviderId, &mut into[0..0 + 16]);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.fActive, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpszIdentifier, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct WSANETWORKEVENTS {
    pub lNetworkEvents: i32,
    pub iErrorCode: [i32; 10],
}
impl ::core::marker::Copy for WSANETWORKEVENTS {}
impl ::core::clone::Clone for WSANETWORKEVENTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSANETWORKEVENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSANETWORKEVENTS")
            .field("lNetworkEvents", &self.lNetworkEvents)
            .field("iErrorCode", &self.iErrorCode)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSANETWORKEVENTS {
    fn eq(&self, other: &Self) -> bool {
        self.lNetworkEvents == other.lNetworkEvents && self.iErrorCode == other.iErrorCode
    }
}
impl ::core::cmp::Eq for WSANETWORKEVENTS {}
impl FromIntoMemory for WSANETWORKEVENTS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_lNetworkEvents = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_iErrorCode = <[i32; 10] as FromIntoMemory>::from_bytes(&from[4..4 + 40]);
        Self {
            lNetworkEvents: f_lNetworkEvents,
            iErrorCode: f_iErrorCode,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.lNetworkEvents, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.iErrorCode, &mut into[4..4 + 40]);
    }
    fn size() -> usize {
        44
    }
}
pub struct WSANSCLASSINFOA {
    pub lpszName: PSTR,
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub lpValue: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for WSANSCLASSINFOA {}
impl ::core::clone::Clone for WSANSCLASSINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSANSCLASSINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSANSCLASSINFOA")
            .field("lpszName", &self.lpszName)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("dwValueType", &self.dwValueType)
            .field("dwValueSize", &self.dwValueSize)
            .field("lpValue", &self.lpValue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSANSCLASSINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.lpszName == other.lpszName
            && self.dwNameSpace == other.dwNameSpace
            && self.dwValueType == other.dwValueType
            && self.dwValueSize == other.dwValueSize
            && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for WSANSCLASSINFOA {}
impl FromIntoMemory for WSANSCLASSINFOA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_lpszName = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwValueType = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwValueSize = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpValue =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            lpszName: f_lpszName,
            dwNameSpace: f_dwNameSpace,
            dwValueType: f_dwValueType,
            dwValueSize: f_dwValueSize,
            lpValue: f_lpValue,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.lpszName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwValueType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwValueSize, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpValue, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct WSANSCLASSINFOW {
    pub lpszName: PWSTR,
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub lpValue: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for WSANSCLASSINFOW {}
impl ::core::clone::Clone for WSANSCLASSINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSANSCLASSINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSANSCLASSINFOW")
            .field("lpszName", &self.lpszName)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("dwValueType", &self.dwValueType)
            .field("dwValueSize", &self.dwValueSize)
            .field("lpValue", &self.lpValue)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSANSCLASSINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.lpszName == other.lpszName
            && self.dwNameSpace == other.dwNameSpace
            && self.dwValueType == other.dwValueType
            && self.dwValueSize == other.dwValueSize
            && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for WSANSCLASSINFOW {}
impl FromIntoMemory for WSANSCLASSINFOW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_lpszName = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwValueType = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwValueSize = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpValue =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            lpszName: f_lpszName,
            dwNameSpace: f_dwNameSpace,
            dwValueType: f_dwValueType,
            dwValueSize: f_dwValueSize,
            lpValue: f_lpValue,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.lpszName, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwValueType, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwValueSize, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpValue, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct WSAPOLLDATA {
    pub result: i32,
    pub fds: u32,
    pub timeout: i32,
    pub fdArray: [WSAPOLLFD; 1],
}
impl ::core::marker::Copy for WSAPOLLDATA {}
impl ::core::clone::Clone for WSAPOLLDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSAPOLLDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAPOLLDATA")
            .field("result", &self.result)
            .field("fds", &self.fds)
            .field("timeout", &self.timeout)
            .field("fdArray", &self.fdArray)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSAPOLLDATA {
    fn eq(&self, other: &Self) -> bool {
        self.result == other.result
            && self.fds == other.fds
            && self.timeout == other.timeout
            && self.fdArray == other.fdArray
    }
}
impl ::core::cmp::Eq for WSAPOLLDATA {}
impl FromIntoMemory for WSAPOLLDATA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_result = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fds = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_timeout = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_fdArray = <[WSAPOLLFD; 1] as FromIntoMemory>::from_bytes(&from[12..12 + 8]);
        Self {
            result: f_result,
            fds: f_fds,
            timeout: f_timeout,
            fdArray: f_fdArray,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.result, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fds, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.timeout, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.fdArray, &mut into[12..12 + 8]);
    }
    fn size() -> usize {
        20
    }
}
pub struct WSAPOLLFD {
    pub fd: SOCKET,
    pub events: i16,
    pub revents: i16,
}
impl ::core::marker::Copy for WSAPOLLFD {}
impl ::core::clone::Clone for WSAPOLLFD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSAPOLLFD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAPOLLFD")
            .field("fd", &self.fd)
            .field("events", &self.events)
            .field("revents", &self.revents)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSAPOLLFD {
    fn eq(&self, other: &Self) -> bool {
        self.fd == other.fd && self.events == other.events && self.revents == other.revents
    }
}
impl ::core::cmp::Eq for WSAPOLLFD {}
impl FromIntoMemory for WSAPOLLFD {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_fd = <SOCKET as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_events = <i16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_revents = <i16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        Self {
            fd: f_fd,
            events: f_events,
            revents: f_revents,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.fd, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.events, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.revents, &mut into[6..6 + 2]);
    }
    fn size() -> usize {
        8
    }
}
pub struct WSAPROTOCOLCHAIN {
    pub ChainLen: i32,
    pub ChainEntries: [u32; 7],
}
impl ::core::marker::Copy for WSAPROTOCOLCHAIN {}
impl ::core::clone::Clone for WSAPROTOCOLCHAIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSAPROTOCOLCHAIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAPROTOCOLCHAIN")
            .field("ChainLen", &self.ChainLen)
            .field("ChainEntries", &self.ChainEntries)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSAPROTOCOLCHAIN {
    fn eq(&self, other: &Self) -> bool {
        self.ChainLen == other.ChainLen && self.ChainEntries == other.ChainEntries
    }
}
impl ::core::cmp::Eq for WSAPROTOCOLCHAIN {}
impl FromIntoMemory for WSAPROTOCOLCHAIN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_ChainLen = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ChainEntries = <[u32; 7] as FromIntoMemory>::from_bytes(&from[4..4 + 28]);
        Self {
            ChainLen: f_ChainLen,
            ChainEntries: f_ChainEntries,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.ChainLen, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ChainEntries, &mut into[4..4 + 28]);
    }
    fn size() -> usize {
        32
    }
}
pub struct WSAPROTOCOL_INFOA {
    pub dwServiceFlags1: u32,
    pub dwServiceFlags2: u32,
    pub dwServiceFlags3: u32,
    pub dwServiceFlags4: u32,
    pub dwProviderFlags: u32,
    pub ProviderId: crate::core::GUID,
    pub dwCatalogEntryId: u32,
    pub ProtocolChain: WSAPROTOCOLCHAIN,
    pub iVersion: i32,
    pub iAddressFamily: i32,
    pub iMaxSockAddr: i32,
    pub iMinSockAddr: i32,
    pub iSocketType: i32,
    pub iProtocol: i32,
    pub iProtocolMaxOffset: i32,
    pub iNetworkByteOrder: i32,
    pub iSecurityScheme: i32,
    pub dwMessageSize: u32,
    pub dwProviderReserved: u32,
    pub szProtocol: [super::super::Foundation::CHAR; 256],
}
impl ::core::marker::Copy for WSAPROTOCOL_INFOA {}
impl ::core::clone::Clone for WSAPROTOCOL_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSAPROTOCOL_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAPROTOCOL_INFOA")
            .field("dwServiceFlags1", &self.dwServiceFlags1)
            .field("dwServiceFlags2", &self.dwServiceFlags2)
            .field("dwServiceFlags3", &self.dwServiceFlags3)
            .field("dwServiceFlags4", &self.dwServiceFlags4)
            .field("dwProviderFlags", &self.dwProviderFlags)
            .field("ProviderId", &self.ProviderId)
            .field("dwCatalogEntryId", &self.dwCatalogEntryId)
            .field("ProtocolChain", &self.ProtocolChain)
            .field("iVersion", &self.iVersion)
            .field("iAddressFamily", &self.iAddressFamily)
            .field("iMaxSockAddr", &self.iMaxSockAddr)
            .field("iMinSockAddr", &self.iMinSockAddr)
            .field("iSocketType", &self.iSocketType)
            .field("iProtocol", &self.iProtocol)
            .field("iProtocolMaxOffset", &self.iProtocolMaxOffset)
            .field("iNetworkByteOrder", &self.iNetworkByteOrder)
            .field("iSecurityScheme", &self.iSecurityScheme)
            .field("dwMessageSize", &self.dwMessageSize)
            .field("dwProviderReserved", &self.dwProviderReserved)
            .field("szProtocol", &self.szProtocol)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSAPROTOCOL_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceFlags1 == other.dwServiceFlags1
            && self.dwServiceFlags2 == other.dwServiceFlags2
            && self.dwServiceFlags3 == other.dwServiceFlags3
            && self.dwServiceFlags4 == other.dwServiceFlags4
            && self.dwProviderFlags == other.dwProviderFlags
            && self.ProviderId == other.ProviderId
            && self.dwCatalogEntryId == other.dwCatalogEntryId
            && self.ProtocolChain == other.ProtocolChain
            && self.iVersion == other.iVersion
            && self.iAddressFamily == other.iAddressFamily
            && self.iMaxSockAddr == other.iMaxSockAddr
            && self.iMinSockAddr == other.iMinSockAddr
            && self.iSocketType == other.iSocketType
            && self.iProtocol == other.iProtocol
            && self.iProtocolMaxOffset == other.iProtocolMaxOffset
            && self.iNetworkByteOrder == other.iNetworkByteOrder
            && self.iSecurityScheme == other.iSecurityScheme
            && self.dwMessageSize == other.dwMessageSize
            && self.dwProviderReserved == other.dwProviderReserved
            && self.szProtocol == other.szProtocol
    }
}
impl ::core::cmp::Eq for WSAPROTOCOL_INFOA {}
impl FromIntoMemory for WSAPROTOCOL_INFOA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 372);
        let f_dwServiceFlags1 = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwServiceFlags2 = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwServiceFlags3 = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwServiceFlags4 = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwProviderFlags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ProviderId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[20..20 + 16]);
        let f_dwCatalogEntryId = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_ProtocolChain = <WSAPROTOCOLCHAIN as FromIntoMemory>::from_bytes(&from[40..40 + 32]);
        let f_iVersion = <i32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_iAddressFamily = <i32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_iMaxSockAddr = <i32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_iMinSockAddr = <i32 as FromIntoMemory>::from_bytes(&from[84..84 + 4]);
        let f_iSocketType = <i32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_iProtocol = <i32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_iProtocolMaxOffset = <i32 as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f_iNetworkByteOrder = <i32 as FromIntoMemory>::from_bytes(&from[100..100 + 4]);
        let f_iSecurityScheme = <i32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_dwMessageSize = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        let f_dwProviderReserved = <u32 as FromIntoMemory>::from_bytes(&from[112..112 + 4]);
        let f_szProtocol = <[super::super::Foundation::CHAR; 256] as FromIntoMemory>::from_bytes(
            &from[116..116 + 256],
        );
        Self {
            dwServiceFlags1: f_dwServiceFlags1,
            dwServiceFlags2: f_dwServiceFlags2,
            dwServiceFlags3: f_dwServiceFlags3,
            dwServiceFlags4: f_dwServiceFlags4,
            dwProviderFlags: f_dwProviderFlags,
            ProviderId: f_ProviderId,
            dwCatalogEntryId: f_dwCatalogEntryId,
            ProtocolChain: f_ProtocolChain,
            iVersion: f_iVersion,
            iAddressFamily: f_iAddressFamily,
            iMaxSockAddr: f_iMaxSockAddr,
            iMinSockAddr: f_iMinSockAddr,
            iSocketType: f_iSocketType,
            iProtocol: f_iProtocol,
            iProtocolMaxOffset: f_iProtocolMaxOffset,
            iNetworkByteOrder: f_iNetworkByteOrder,
            iSecurityScheme: f_iSecurityScheme,
            dwMessageSize: f_dwMessageSize,
            dwProviderReserved: f_dwProviderReserved,
            szProtocol: f_szProtocol,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 372);
        FromIntoMemory::into_bytes(self.dwServiceFlags1, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwServiceFlags2, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwServiceFlags3, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwServiceFlags4, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwProviderFlags, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ProviderId, &mut into[20..20 + 16]);
        FromIntoMemory::into_bytes(self.dwCatalogEntryId, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.ProtocolChain, &mut into[40..40 + 32]);
        FromIntoMemory::into_bytes(self.iVersion, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.iAddressFamily, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.iMaxSockAddr, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.iMinSockAddr, &mut into[84..84 + 4]);
        FromIntoMemory::into_bytes(self.iSocketType, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.iProtocol, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.iProtocolMaxOffset, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self.iNetworkByteOrder, &mut into[100..100 + 4]);
        FromIntoMemory::into_bytes(self.iSecurityScheme, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.dwMessageSize, &mut into[108..108 + 4]);
        FromIntoMemory::into_bytes(self.dwProviderReserved, &mut into[112..112 + 4]);
        FromIntoMemory::into_bytes(self.szProtocol, &mut into[116..116 + 256]);
    }
    fn size() -> usize {
        372
    }
}
pub struct WSAPROTOCOL_INFOW {
    pub dwServiceFlags1: u32,
    pub dwServiceFlags2: u32,
    pub dwServiceFlags3: u32,
    pub dwServiceFlags4: u32,
    pub dwProviderFlags: u32,
    pub ProviderId: crate::core::GUID,
    pub dwCatalogEntryId: u32,
    pub ProtocolChain: WSAPROTOCOLCHAIN,
    pub iVersion: i32,
    pub iAddressFamily: i32,
    pub iMaxSockAddr: i32,
    pub iMinSockAddr: i32,
    pub iSocketType: i32,
    pub iProtocol: i32,
    pub iProtocolMaxOffset: i32,
    pub iNetworkByteOrder: i32,
    pub iSecurityScheme: i32,
    pub dwMessageSize: u32,
    pub dwProviderReserved: u32,
    pub szProtocol: [u16; 256],
}
impl ::core::marker::Copy for WSAPROTOCOL_INFOW {}
impl ::core::clone::Clone for WSAPROTOCOL_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSAPROTOCOL_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAPROTOCOL_INFOW")
            .field("dwServiceFlags1", &self.dwServiceFlags1)
            .field("dwServiceFlags2", &self.dwServiceFlags2)
            .field("dwServiceFlags3", &self.dwServiceFlags3)
            .field("dwServiceFlags4", &self.dwServiceFlags4)
            .field("dwProviderFlags", &self.dwProviderFlags)
            .field("ProviderId", &self.ProviderId)
            .field("dwCatalogEntryId", &self.dwCatalogEntryId)
            .field("ProtocolChain", &self.ProtocolChain)
            .field("iVersion", &self.iVersion)
            .field("iAddressFamily", &self.iAddressFamily)
            .field("iMaxSockAddr", &self.iMaxSockAddr)
            .field("iMinSockAddr", &self.iMinSockAddr)
            .field("iSocketType", &self.iSocketType)
            .field("iProtocol", &self.iProtocol)
            .field("iProtocolMaxOffset", &self.iProtocolMaxOffset)
            .field("iNetworkByteOrder", &self.iNetworkByteOrder)
            .field("iSecurityScheme", &self.iSecurityScheme)
            .field("dwMessageSize", &self.dwMessageSize)
            .field("dwProviderReserved", &self.dwProviderReserved)
            .field("szProtocol", &self.szProtocol)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSAPROTOCOL_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceFlags1 == other.dwServiceFlags1
            && self.dwServiceFlags2 == other.dwServiceFlags2
            && self.dwServiceFlags3 == other.dwServiceFlags3
            && self.dwServiceFlags4 == other.dwServiceFlags4
            && self.dwProviderFlags == other.dwProviderFlags
            && self.ProviderId == other.ProviderId
            && self.dwCatalogEntryId == other.dwCatalogEntryId
            && self.ProtocolChain == other.ProtocolChain
            && self.iVersion == other.iVersion
            && self.iAddressFamily == other.iAddressFamily
            && self.iMaxSockAddr == other.iMaxSockAddr
            && self.iMinSockAddr == other.iMinSockAddr
            && self.iSocketType == other.iSocketType
            && self.iProtocol == other.iProtocol
            && self.iProtocolMaxOffset == other.iProtocolMaxOffset
            && self.iNetworkByteOrder == other.iNetworkByteOrder
            && self.iSecurityScheme == other.iSecurityScheme
            && self.dwMessageSize == other.dwMessageSize
            && self.dwProviderReserved == other.dwProviderReserved
            && self.szProtocol == other.szProtocol
    }
}
impl ::core::cmp::Eq for WSAPROTOCOL_INFOW {}
impl FromIntoMemory for WSAPROTOCOL_INFOW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 372);
        let f_dwServiceFlags1 = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwServiceFlags2 = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwServiceFlags3 = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_dwServiceFlags4 = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwProviderFlags = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ProviderId = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[20..20 + 16]);
        let f_dwCatalogEntryId = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_ProtocolChain = <WSAPROTOCOLCHAIN as FromIntoMemory>::from_bytes(&from[40..40 + 32]);
        let f_iVersion = <i32 as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_iAddressFamily = <i32 as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_iMaxSockAddr = <i32 as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_iMinSockAddr = <i32 as FromIntoMemory>::from_bytes(&from[84..84 + 4]);
        let f_iSocketType = <i32 as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_iProtocol = <i32 as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_iProtocolMaxOffset = <i32 as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f_iNetworkByteOrder = <i32 as FromIntoMemory>::from_bytes(&from[100..100 + 4]);
        let f_iSecurityScheme = <i32 as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_dwMessageSize = <u32 as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        let f_dwProviderReserved = <u32 as FromIntoMemory>::from_bytes(&from[112..112 + 4]);
        let f_szProtocol = <[u16; 256] as FromIntoMemory>::from_bytes(&from[116..116 + 256]);
        Self {
            dwServiceFlags1: f_dwServiceFlags1,
            dwServiceFlags2: f_dwServiceFlags2,
            dwServiceFlags3: f_dwServiceFlags3,
            dwServiceFlags4: f_dwServiceFlags4,
            dwProviderFlags: f_dwProviderFlags,
            ProviderId: f_ProviderId,
            dwCatalogEntryId: f_dwCatalogEntryId,
            ProtocolChain: f_ProtocolChain,
            iVersion: f_iVersion,
            iAddressFamily: f_iAddressFamily,
            iMaxSockAddr: f_iMaxSockAddr,
            iMinSockAddr: f_iMinSockAddr,
            iSocketType: f_iSocketType,
            iProtocol: f_iProtocol,
            iProtocolMaxOffset: f_iProtocolMaxOffset,
            iNetworkByteOrder: f_iNetworkByteOrder,
            iSecurityScheme: f_iSecurityScheme,
            dwMessageSize: f_dwMessageSize,
            dwProviderReserved: f_dwProviderReserved,
            szProtocol: f_szProtocol,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 372);
        FromIntoMemory::into_bytes(self.dwServiceFlags1, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwServiceFlags2, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwServiceFlags3, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.dwServiceFlags4, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwProviderFlags, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ProviderId, &mut into[20..20 + 16]);
        FromIntoMemory::into_bytes(self.dwCatalogEntryId, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.ProtocolChain, &mut into[40..40 + 32]);
        FromIntoMemory::into_bytes(self.iVersion, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.iAddressFamily, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.iMaxSockAddr, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.iMinSockAddr, &mut into[84..84 + 4]);
        FromIntoMemory::into_bytes(self.iSocketType, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.iProtocol, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.iProtocolMaxOffset, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self.iNetworkByteOrder, &mut into[100..100 + 4]);
        FromIntoMemory::into_bytes(self.iSecurityScheme, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.dwMessageSize, &mut into[108..108 + 4]);
        FromIntoMemory::into_bytes(self.dwProviderReserved, &mut into[112..112 + 4]);
        FromIntoMemory::into_bytes(self.szProtocol, &mut into[116..116 + 256]);
    }
    fn size() -> usize {
        372
    }
}
pub const WSAPROTOCOL_LEN: u32 = 255u32;
pub struct WSAQUERYSET2A {
    pub dwSize: u32,
    pub lpszServiceInstanceName: PSTR,
    pub lpVersion: MutPtr<WSAVERSION>,
    pub lpszComment: PSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: MutPtr<crate::core::GUID>,
    pub lpszContext: PSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: MutPtr<AFPROTOCOLS>,
    pub lpszQueryString: PSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: MutPtr<CSADDR_INFO>,
    pub dwOutputFlags: u32,
    pub lpBlob: MutPtr<super::super::System::Com::BLOB>,
}
impl ::core::marker::Copy for WSAQUERYSET2A {}
impl ::core::clone::Clone for WSAQUERYSET2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSAQUERYSET2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAQUERYSET2A")
            .field("dwSize", &self.dwSize)
            .field("lpszServiceInstanceName", &self.lpszServiceInstanceName)
            .field("lpVersion", &self.lpVersion)
            .field("lpszComment", &self.lpszComment)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("lpNSProviderId", &self.lpNSProviderId)
            .field("lpszContext", &self.lpszContext)
            .field("dwNumberOfProtocols", &self.dwNumberOfProtocols)
            .field("lpafpProtocols", &self.lpafpProtocols)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("dwNumberOfCsAddrs", &self.dwNumberOfCsAddrs)
            .field("lpcsaBuffer", &self.lpcsaBuffer)
            .field("dwOutputFlags", &self.dwOutputFlags)
            .field("lpBlob", &self.lpBlob)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSAQUERYSET2A {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.lpszServiceInstanceName == other.lpszServiceInstanceName
            && self.lpVersion == other.lpVersion
            && self.lpszComment == other.lpszComment
            && self.dwNameSpace == other.dwNameSpace
            && self.lpNSProviderId == other.lpNSProviderId
            && self.lpszContext == other.lpszContext
            && self.dwNumberOfProtocols == other.dwNumberOfProtocols
            && self.lpafpProtocols == other.lpafpProtocols
            && self.lpszQueryString == other.lpszQueryString
            && self.dwNumberOfCsAddrs == other.dwNumberOfCsAddrs
            && self.lpcsaBuffer == other.lpcsaBuffer
            && self.dwOutputFlags == other.dwOutputFlags
            && self.lpBlob == other.lpBlob
    }
}
impl ::core::cmp::Eq for WSAQUERYSET2A {}
impl FromIntoMemory for WSAQUERYSET2A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 56);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpszServiceInstanceName = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpVersion = <MutPtr<WSAVERSION> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpszComment = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpNSProviderId =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_lpszContext = <PSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwNumberOfProtocols = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_lpafpProtocols =
            <MutPtr<AFPROTOCOLS> as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_lpszQueryString = <PSTR as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwNumberOfCsAddrs = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_lpcsaBuffer = <MutPtr<CSADDR_INFO> as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dwOutputFlags = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_lpBlob = <MutPtr<super::super::System::Com::BLOB> as FromIntoMemory>::from_bytes(
            &from[52..52 + 4],
        );
        Self {
            dwSize: f_dwSize,
            lpszServiceInstanceName: f_lpszServiceInstanceName,
            lpVersion: f_lpVersion,
            lpszComment: f_lpszComment,
            dwNameSpace: f_dwNameSpace,
            lpNSProviderId: f_lpNSProviderId,
            lpszContext: f_lpszContext,
            dwNumberOfProtocols: f_dwNumberOfProtocols,
            lpafpProtocols: f_lpafpProtocols,
            lpszQueryString: f_lpszQueryString,
            dwNumberOfCsAddrs: f_dwNumberOfCsAddrs,
            lpcsaBuffer: f_lpcsaBuffer,
            dwOutputFlags: f_dwOutputFlags,
            lpBlob: f_lpBlob,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 56);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpszServiceInstanceName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpVersion, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpszComment, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpNSProviderId, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lpszContext, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfProtocols, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.lpafpProtocols, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.lpszQueryString, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfCsAddrs, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.lpcsaBuffer, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dwOutputFlags, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.lpBlob, &mut into[52..52 + 4]);
    }
    fn size() -> usize {
        56
    }
}
pub struct WSAQUERYSET2W {
    pub dwSize: u32,
    pub lpszServiceInstanceName: PWSTR,
    pub lpVersion: MutPtr<WSAVERSION>,
    pub lpszComment: PWSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: MutPtr<crate::core::GUID>,
    pub lpszContext: PWSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: MutPtr<AFPROTOCOLS>,
    pub lpszQueryString: PWSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: MutPtr<CSADDR_INFO>,
    pub dwOutputFlags: u32,
    pub lpBlob: MutPtr<super::super::System::Com::BLOB>,
}
impl ::core::marker::Copy for WSAQUERYSET2W {}
impl ::core::clone::Clone for WSAQUERYSET2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSAQUERYSET2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAQUERYSET2W")
            .field("dwSize", &self.dwSize)
            .field("lpszServiceInstanceName", &self.lpszServiceInstanceName)
            .field("lpVersion", &self.lpVersion)
            .field("lpszComment", &self.lpszComment)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("lpNSProviderId", &self.lpNSProviderId)
            .field("lpszContext", &self.lpszContext)
            .field("dwNumberOfProtocols", &self.dwNumberOfProtocols)
            .field("lpafpProtocols", &self.lpafpProtocols)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("dwNumberOfCsAddrs", &self.dwNumberOfCsAddrs)
            .field("lpcsaBuffer", &self.lpcsaBuffer)
            .field("dwOutputFlags", &self.dwOutputFlags)
            .field("lpBlob", &self.lpBlob)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSAQUERYSET2W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.lpszServiceInstanceName == other.lpszServiceInstanceName
            && self.lpVersion == other.lpVersion
            && self.lpszComment == other.lpszComment
            && self.dwNameSpace == other.dwNameSpace
            && self.lpNSProviderId == other.lpNSProviderId
            && self.lpszContext == other.lpszContext
            && self.dwNumberOfProtocols == other.dwNumberOfProtocols
            && self.lpafpProtocols == other.lpafpProtocols
            && self.lpszQueryString == other.lpszQueryString
            && self.dwNumberOfCsAddrs == other.dwNumberOfCsAddrs
            && self.lpcsaBuffer == other.lpcsaBuffer
            && self.dwOutputFlags == other.dwOutputFlags
            && self.lpBlob == other.lpBlob
    }
}
impl ::core::cmp::Eq for WSAQUERYSET2W {}
impl FromIntoMemory for WSAQUERYSET2W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 56);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpszServiceInstanceName = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpVersion = <MutPtr<WSAVERSION> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpszComment = <PWSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpNSProviderId =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_lpszContext = <PWSTR as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_dwNumberOfProtocols = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_lpafpProtocols =
            <MutPtr<AFPROTOCOLS> as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_lpszQueryString = <PWSTR as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_dwNumberOfCsAddrs = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_lpcsaBuffer = <MutPtr<CSADDR_INFO> as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_dwOutputFlags = <u32 as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_lpBlob = <MutPtr<super::super::System::Com::BLOB> as FromIntoMemory>::from_bytes(
            &from[52..52 + 4],
        );
        Self {
            dwSize: f_dwSize,
            lpszServiceInstanceName: f_lpszServiceInstanceName,
            lpVersion: f_lpVersion,
            lpszComment: f_lpszComment,
            dwNameSpace: f_dwNameSpace,
            lpNSProviderId: f_lpNSProviderId,
            lpszContext: f_lpszContext,
            dwNumberOfProtocols: f_dwNumberOfProtocols,
            lpafpProtocols: f_lpafpProtocols,
            lpszQueryString: f_lpszQueryString,
            dwNumberOfCsAddrs: f_dwNumberOfCsAddrs,
            lpcsaBuffer: f_lpcsaBuffer,
            dwOutputFlags: f_dwOutputFlags,
            lpBlob: f_lpBlob,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 56);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpszServiceInstanceName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpVersion, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpszComment, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpNSProviderId, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lpszContext, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfProtocols, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.lpafpProtocols, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.lpszQueryString, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfCsAddrs, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.lpcsaBuffer, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.dwOutputFlags, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.lpBlob, &mut into[52..52 + 4]);
    }
    fn size() -> usize {
        56
    }
}
pub struct WSAQUERYSETA {
    pub dwSize: u32,
    pub lpszServiceInstanceName: PSTR,
    pub lpServiceClassId: MutPtr<crate::core::GUID>,
    pub lpVersion: MutPtr<WSAVERSION>,
    pub lpszComment: PSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: MutPtr<crate::core::GUID>,
    pub lpszContext: PSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: MutPtr<AFPROTOCOLS>,
    pub lpszQueryString: PSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: MutPtr<CSADDR_INFO>,
    pub dwOutputFlags: u32,
    pub lpBlob: MutPtr<super::super::System::Com::BLOB>,
}
impl ::core::marker::Copy for WSAQUERYSETA {}
impl ::core::clone::Clone for WSAQUERYSETA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSAQUERYSETA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAQUERYSETA")
            .field("dwSize", &self.dwSize)
            .field("lpszServiceInstanceName", &self.lpszServiceInstanceName)
            .field("lpServiceClassId", &self.lpServiceClassId)
            .field("lpVersion", &self.lpVersion)
            .field("lpszComment", &self.lpszComment)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("lpNSProviderId", &self.lpNSProviderId)
            .field("lpszContext", &self.lpszContext)
            .field("dwNumberOfProtocols", &self.dwNumberOfProtocols)
            .field("lpafpProtocols", &self.lpafpProtocols)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("dwNumberOfCsAddrs", &self.dwNumberOfCsAddrs)
            .field("lpcsaBuffer", &self.lpcsaBuffer)
            .field("dwOutputFlags", &self.dwOutputFlags)
            .field("lpBlob", &self.lpBlob)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSAQUERYSETA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.lpszServiceInstanceName == other.lpszServiceInstanceName
            && self.lpServiceClassId == other.lpServiceClassId
            && self.lpVersion == other.lpVersion
            && self.lpszComment == other.lpszComment
            && self.dwNameSpace == other.dwNameSpace
            && self.lpNSProviderId == other.lpNSProviderId
            && self.lpszContext == other.lpszContext
            && self.dwNumberOfProtocols == other.dwNumberOfProtocols
            && self.lpafpProtocols == other.lpafpProtocols
            && self.lpszQueryString == other.lpszQueryString
            && self.dwNumberOfCsAddrs == other.dwNumberOfCsAddrs
            && self.lpcsaBuffer == other.lpcsaBuffer
            && self.dwOutputFlags == other.dwOutputFlags
            && self.lpBlob == other.lpBlob
    }
}
impl ::core::cmp::Eq for WSAQUERYSETA {}
impl FromIntoMemory for WSAQUERYSETA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 60);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpszServiceInstanceName = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpServiceClassId =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpVersion = <MutPtr<WSAVERSION> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpszComment = <PSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_lpNSProviderId =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lpszContext = <PSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwNumberOfProtocols = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_lpafpProtocols =
            <MutPtr<AFPROTOCOLS> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_lpszQueryString = <PSTR as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dwNumberOfCsAddrs = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_lpcsaBuffer = <MutPtr<CSADDR_INFO> as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_dwOutputFlags = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_lpBlob = <MutPtr<super::super::System::Com::BLOB> as FromIntoMemory>::from_bytes(
            &from[56..56 + 4],
        );
        Self {
            dwSize: f_dwSize,
            lpszServiceInstanceName: f_lpszServiceInstanceName,
            lpServiceClassId: f_lpServiceClassId,
            lpVersion: f_lpVersion,
            lpszComment: f_lpszComment,
            dwNameSpace: f_dwNameSpace,
            lpNSProviderId: f_lpNSProviderId,
            lpszContext: f_lpszContext,
            dwNumberOfProtocols: f_dwNumberOfProtocols,
            lpafpProtocols: f_lpafpProtocols,
            lpszQueryString: f_lpszQueryString,
            dwNumberOfCsAddrs: f_dwNumberOfCsAddrs,
            lpcsaBuffer: f_lpcsaBuffer,
            dwOutputFlags: f_dwOutputFlags,
            lpBlob: f_lpBlob,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 60);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpszServiceInstanceName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpServiceClassId, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpVersion, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpszComment, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lpNSProviderId, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpszContext, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfProtocols, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.lpafpProtocols, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.lpszQueryString, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfCsAddrs, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.lpcsaBuffer, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.dwOutputFlags, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.lpBlob, &mut into[56..56 + 4]);
    }
    fn size() -> usize {
        60
    }
}
pub struct WSAQUERYSETW {
    pub dwSize: u32,
    pub lpszServiceInstanceName: PWSTR,
    pub lpServiceClassId: MutPtr<crate::core::GUID>,
    pub lpVersion: MutPtr<WSAVERSION>,
    pub lpszComment: PWSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: MutPtr<crate::core::GUID>,
    pub lpszContext: PWSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: MutPtr<AFPROTOCOLS>,
    pub lpszQueryString: PWSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: MutPtr<CSADDR_INFO>,
    pub dwOutputFlags: u32,
    pub lpBlob: MutPtr<super::super::System::Com::BLOB>,
}
impl ::core::marker::Copy for WSAQUERYSETW {}
impl ::core::clone::Clone for WSAQUERYSETW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSAQUERYSETW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAQUERYSETW")
            .field("dwSize", &self.dwSize)
            .field("lpszServiceInstanceName", &self.lpszServiceInstanceName)
            .field("lpServiceClassId", &self.lpServiceClassId)
            .field("lpVersion", &self.lpVersion)
            .field("lpszComment", &self.lpszComment)
            .field("dwNameSpace", &self.dwNameSpace)
            .field("lpNSProviderId", &self.lpNSProviderId)
            .field("lpszContext", &self.lpszContext)
            .field("dwNumberOfProtocols", &self.dwNumberOfProtocols)
            .field("lpafpProtocols", &self.lpafpProtocols)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("dwNumberOfCsAddrs", &self.dwNumberOfCsAddrs)
            .field("lpcsaBuffer", &self.lpcsaBuffer)
            .field("dwOutputFlags", &self.dwOutputFlags)
            .field("lpBlob", &self.lpBlob)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSAQUERYSETW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.lpszServiceInstanceName == other.lpszServiceInstanceName
            && self.lpServiceClassId == other.lpServiceClassId
            && self.lpVersion == other.lpVersion
            && self.lpszComment == other.lpszComment
            && self.dwNameSpace == other.dwNameSpace
            && self.lpNSProviderId == other.lpNSProviderId
            && self.lpszContext == other.lpszContext
            && self.dwNumberOfProtocols == other.dwNumberOfProtocols
            && self.lpafpProtocols == other.lpafpProtocols
            && self.lpszQueryString == other.lpszQueryString
            && self.dwNumberOfCsAddrs == other.dwNumberOfCsAddrs
            && self.lpcsaBuffer == other.lpcsaBuffer
            && self.dwOutputFlags == other.dwOutputFlags
            && self.lpBlob == other.lpBlob
    }
}
impl ::core::cmp::Eq for WSAQUERYSETW {}
impl FromIntoMemory for WSAQUERYSETW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 60);
        let f_dwSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpszServiceInstanceName = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpServiceClassId =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpVersion = <MutPtr<WSAVERSION> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpszComment = <PWSTR as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_dwNameSpace = <u32 as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_lpNSProviderId =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lpszContext = <PWSTR as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_dwNumberOfProtocols = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_lpafpProtocols =
            <MutPtr<AFPROTOCOLS> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_lpszQueryString = <PWSTR as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_dwNumberOfCsAddrs = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_lpcsaBuffer = <MutPtr<CSADDR_INFO> as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_dwOutputFlags = <u32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_lpBlob = <MutPtr<super::super::System::Com::BLOB> as FromIntoMemory>::from_bytes(
            &from[56..56 + 4],
        );
        Self {
            dwSize: f_dwSize,
            lpszServiceInstanceName: f_lpszServiceInstanceName,
            lpServiceClassId: f_lpServiceClassId,
            lpVersion: f_lpVersion,
            lpszComment: f_lpszComment,
            dwNameSpace: f_dwNameSpace,
            lpNSProviderId: f_lpNSProviderId,
            lpszContext: f_lpszContext,
            dwNumberOfProtocols: f_dwNumberOfProtocols,
            lpafpProtocols: f_lpafpProtocols,
            lpszQueryString: f_lpszQueryString,
            dwNumberOfCsAddrs: f_dwNumberOfCsAddrs,
            lpcsaBuffer: f_lpcsaBuffer,
            dwOutputFlags: f_dwOutputFlags,
            lpBlob: f_lpBlob,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 60);
        FromIntoMemory::into_bytes(self.dwSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpszServiceInstanceName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpServiceClassId, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpVersion, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpszComment, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.dwNameSpace, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lpNSProviderId, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpszContext, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfProtocols, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.lpafpProtocols, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.lpszQueryString, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.dwNumberOfCsAddrs, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.lpcsaBuffer, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.dwOutputFlags, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.lpBlob, &mut into[56..56 + 4]);
    }
    fn size() -> usize {
        60
    }
}
pub struct WSASENDMSG {
    pub lpMsg: MutPtr<WSAMSG>,
    pub dwFlags: u32,
    pub lpNumberOfBytesSent: MutPtr<u32>,
    pub lpOverlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
    pub lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
}
impl ::core::marker::Copy for WSASENDMSG {}
impl ::core::clone::Clone for WSASENDMSG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSASENDMSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSASENDMSG")
            .field("lpMsg", &self.lpMsg)
            .field("dwFlags", &self.dwFlags)
            .field("lpNumberOfBytesSent", &self.lpNumberOfBytesSent)
            .field("lpOverlapped", &self.lpOverlapped)
            .field("lpCompletionRoutine", &self.lpCompletionRoutine)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSASENDMSG {
    fn eq(&self, other: &Self) -> bool {
        self.lpMsg == other.lpMsg
            && self.dwFlags == other.dwFlags
            && self.lpNumberOfBytesSent == other.lpNumberOfBytesSent
            && self.lpOverlapped == other.lpOverlapped
            && self.lpCompletionRoutine == other.lpCompletionRoutine
    }
}
impl ::core::cmp::Eq for WSASENDMSG {}
impl FromIntoMemory for WSASENDMSG {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_lpMsg = <MutPtr<WSAMSG> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_dwFlags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpNumberOfBytesSent = <MutPtr<u32> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpOverlapped =
            <MutPtr<super::super::System::IO::OVERLAPPED> as FromIntoMemory>::from_bytes(
                &from[12..12 + 4],
            );
        let f_lpCompletionRoutine =
            <LPWSAOVERLAPPED_COMPLETION_ROUTINE as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            lpMsg: f_lpMsg,
            dwFlags: f_dwFlags,
            lpNumberOfBytesSent: f_lpNumberOfBytesSent,
            lpOverlapped: f_lpOverlapped,
            lpCompletionRoutine: f_lpCompletionRoutine,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.lpMsg, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.dwFlags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpNumberOfBytesSent, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpOverlapped, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpCompletionRoutine, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct WSASERVICECLASSINFOA {
    pub lpServiceClassId: MutPtr<crate::core::GUID>,
    pub lpszServiceClassName: PSTR,
    pub dwCount: u32,
    pub lpClassInfos: MutPtr<WSANSCLASSINFOA>,
}
impl ::core::marker::Copy for WSASERVICECLASSINFOA {}
impl ::core::clone::Clone for WSASERVICECLASSINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSASERVICECLASSINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSASERVICECLASSINFOA")
            .field("lpServiceClassId", &self.lpServiceClassId)
            .field("lpszServiceClassName", &self.lpszServiceClassName)
            .field("dwCount", &self.dwCount)
            .field("lpClassInfos", &self.lpClassInfos)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSASERVICECLASSINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceClassId == other.lpServiceClassId
            && self.lpszServiceClassName == other.lpszServiceClassName
            && self.dwCount == other.dwCount
            && self.lpClassInfos == other.lpClassInfos
    }
}
impl ::core::cmp::Eq for WSASERVICECLASSINFOA {}
impl FromIntoMemory for WSASERVICECLASSINFOA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_lpServiceClassId =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpszServiceClassName = <PSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwCount = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpClassInfos =
            <MutPtr<WSANSCLASSINFOA> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            lpServiceClassId: f_lpServiceClassId,
            lpszServiceClassName: f_lpszServiceClassName,
            dwCount: f_dwCount,
            lpClassInfos: f_lpClassInfos,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.lpServiceClassId, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpszServiceClassName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwCount, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpClassInfos, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct WSASERVICECLASSINFOW {
    pub lpServiceClassId: MutPtr<crate::core::GUID>,
    pub lpszServiceClassName: PWSTR,
    pub dwCount: u32,
    pub lpClassInfos: MutPtr<WSANSCLASSINFOW>,
}
impl ::core::marker::Copy for WSASERVICECLASSINFOW {}
impl ::core::clone::Clone for WSASERVICECLASSINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSASERVICECLASSINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSASERVICECLASSINFOW")
            .field("lpServiceClassId", &self.lpServiceClassId)
            .field("lpszServiceClassName", &self.lpszServiceClassName)
            .field("dwCount", &self.dwCount)
            .field("lpClassInfos", &self.lpClassInfos)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSASERVICECLASSINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceClassId == other.lpServiceClassId
            && self.lpszServiceClassName == other.lpszServiceClassName
            && self.dwCount == other.dwCount
            && self.lpClassInfos == other.lpClassInfos
    }
}
impl ::core::cmp::Eq for WSASERVICECLASSINFOW {}
impl FromIntoMemory for WSASERVICECLASSINFOW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_lpServiceClassId =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpszServiceClassName = <PWSTR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_dwCount = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpClassInfos =
            <MutPtr<WSANSCLASSINFOW> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            lpServiceClassId: f_lpServiceClassId,
            lpszServiceClassName: f_lpszServiceClassName,
            dwCount: f_dwCount,
            lpClassInfos: f_lpClassInfos,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.lpServiceClassId, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpszServiceClassName, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.dwCount, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpClassInfos, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub const WSASYS_STATUS_LEN: u32 = 128u32;
pub struct WSATHREADID {
    pub ThreadHandle: super::super::Foundation::HANDLE,
    pub Reserved: PtrRepr,
}
impl ::core::marker::Copy for WSATHREADID {}
impl ::core::clone::Clone for WSATHREADID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSATHREADID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSATHREADID")
            .field("ThreadHandle", &self.ThreadHandle)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSATHREADID {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadHandle == other.ThreadHandle && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WSATHREADID {}
impl FromIntoMemory for WSATHREADID {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_ThreadHandle =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Reserved = <PtrRepr as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            ThreadHandle: f_ThreadHandle,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.ThreadHandle, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct WSAVERSION {
    pub dwVersion: u32,
    pub ecHow: WSAECOMPARATOR,
}
impl ::core::marker::Copy for WSAVERSION {}
impl ::core::clone::Clone for WSAVERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSAVERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSAVERSION")
            .field("dwVersion", &self.dwVersion)
            .field("ecHow", &self.ecHow)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSAVERSION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.ecHow == other.ecHow
    }
}
impl ::core::cmp::Eq for WSAVERSION {}
impl FromIntoMemory for WSAVERSION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_dwVersion = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ecHow = <WSAECOMPARATOR as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            dwVersion: f_dwVersion,
            ecHow: f_ecHow,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.dwVersion, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ecHow, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSA_COMPATIBILITY_BEHAVIOR_ID(pub i32);
pub const WsaBehaviorAll: WSA_COMPATIBILITY_BEHAVIOR_ID = WSA_COMPATIBILITY_BEHAVIOR_ID(0i32);
pub const WsaBehaviorReceiveBuffering: WSA_COMPATIBILITY_BEHAVIOR_ID =
    WSA_COMPATIBILITY_BEHAVIOR_ID(1i32);
pub const WsaBehaviorAutoTuning: WSA_COMPATIBILITY_BEHAVIOR_ID =
    WSA_COMPATIBILITY_BEHAVIOR_ID(2i32);
impl ::core::marker::Copy for WSA_COMPATIBILITY_BEHAVIOR_ID {}
impl ::core::clone::Clone for WSA_COMPATIBILITY_BEHAVIOR_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSA_COMPATIBILITY_BEHAVIOR_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSA_COMPATIBILITY_BEHAVIOR_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSA_COMPATIBILITY_BEHAVIOR_ID")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for WSA_COMPATIBILITY_BEHAVIOR_ID {
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
pub struct WSA_COMPATIBILITY_MODE {
    pub BehaviorId: WSA_COMPATIBILITY_BEHAVIOR_ID,
    pub TargetOsVersion: u32,
}
impl ::core::marker::Copy for WSA_COMPATIBILITY_MODE {}
impl ::core::clone::Clone for WSA_COMPATIBILITY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSA_COMPATIBILITY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSA_COMPATIBILITY_MODE")
            .field("BehaviorId", &self.BehaviorId)
            .field("TargetOsVersion", &self.TargetOsVersion)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSA_COMPATIBILITY_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.BehaviorId == other.BehaviorId && self.TargetOsVersion == other.TargetOsVersion
    }
}
impl ::core::cmp::Eq for WSA_COMPATIBILITY_MODE {}
impl FromIntoMemory for WSA_COMPATIBILITY_MODE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_BehaviorId =
            <WSA_COMPATIBILITY_BEHAVIOR_ID as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_TargetOsVersion = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            BehaviorId: f_BehaviorId,
            TargetOsVersion: f_TargetOsVersion,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.BehaviorId, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.TargetOsVersion, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSA_ERROR(pub i32);
pub const WSA_IO_PENDING: WSA_ERROR = WSA_ERROR(997i32);
pub const WSA_IO_INCOMPLETE: WSA_ERROR = WSA_ERROR(996i32);
pub const WSA_INVALID_HANDLE: WSA_ERROR = WSA_ERROR(6i32);
pub const WSA_INVALID_PARAMETER: WSA_ERROR = WSA_ERROR(87i32);
pub const WSA_NOT_ENOUGH_MEMORY: WSA_ERROR = WSA_ERROR(8i32);
pub const WSA_OPERATION_ABORTED: WSA_ERROR = WSA_ERROR(995i32);
pub const WSABASEERR: WSA_ERROR = WSA_ERROR(10000i32);
pub const WSAEINTR: WSA_ERROR = WSA_ERROR(10004i32);
pub const WSAEBADF: WSA_ERROR = WSA_ERROR(10009i32);
pub const WSAEACCES: WSA_ERROR = WSA_ERROR(10013i32);
pub const WSAEFAULT: WSA_ERROR = WSA_ERROR(10014i32);
pub const WSAEINVAL: WSA_ERROR = WSA_ERROR(10022i32);
pub const WSAEMFILE: WSA_ERROR = WSA_ERROR(10024i32);
pub const WSAEWOULDBLOCK: WSA_ERROR = WSA_ERROR(10035i32);
pub const WSAEINPROGRESS: WSA_ERROR = WSA_ERROR(10036i32);
pub const WSAEALREADY: WSA_ERROR = WSA_ERROR(10037i32);
pub const WSAENOTSOCK: WSA_ERROR = WSA_ERROR(10038i32);
pub const WSAEDESTADDRREQ: WSA_ERROR = WSA_ERROR(10039i32);
pub const WSAEMSGSIZE: WSA_ERROR = WSA_ERROR(10040i32);
pub const WSAEPROTOTYPE: WSA_ERROR = WSA_ERROR(10041i32);
pub const WSAENOPROTOOPT: WSA_ERROR = WSA_ERROR(10042i32);
pub const WSAEPROTONOSUPPORT: WSA_ERROR = WSA_ERROR(10043i32);
pub const WSAESOCKTNOSUPPORT: WSA_ERROR = WSA_ERROR(10044i32);
pub const WSAEOPNOTSUPP: WSA_ERROR = WSA_ERROR(10045i32);
pub const WSAEPFNOSUPPORT: WSA_ERROR = WSA_ERROR(10046i32);
pub const WSAEAFNOSUPPORT: WSA_ERROR = WSA_ERROR(10047i32);
pub const WSAEADDRINUSE: WSA_ERROR = WSA_ERROR(10048i32);
pub const WSAEADDRNOTAVAIL: WSA_ERROR = WSA_ERROR(10049i32);
pub const WSAENETDOWN: WSA_ERROR = WSA_ERROR(10050i32);
pub const WSAENETUNREACH: WSA_ERROR = WSA_ERROR(10051i32);
pub const WSAENETRESET: WSA_ERROR = WSA_ERROR(10052i32);
pub const WSAECONNABORTED: WSA_ERROR = WSA_ERROR(10053i32);
pub const WSAECONNRESET: WSA_ERROR = WSA_ERROR(10054i32);
pub const WSAENOBUFS: WSA_ERROR = WSA_ERROR(10055i32);
pub const WSAEISCONN: WSA_ERROR = WSA_ERROR(10056i32);
pub const WSAENOTCONN: WSA_ERROR = WSA_ERROR(10057i32);
pub const WSAESHUTDOWN: WSA_ERROR = WSA_ERROR(10058i32);
pub const WSAETOOMANYREFS: WSA_ERROR = WSA_ERROR(10059i32);
pub const WSAETIMEDOUT: WSA_ERROR = WSA_ERROR(10060i32);
pub const WSAECONNREFUSED: WSA_ERROR = WSA_ERROR(10061i32);
pub const WSAELOOP: WSA_ERROR = WSA_ERROR(10062i32);
pub const WSAENAMETOOLONG: WSA_ERROR = WSA_ERROR(10063i32);
pub const WSAEHOSTDOWN: WSA_ERROR = WSA_ERROR(10064i32);
pub const WSAEHOSTUNREACH: WSA_ERROR = WSA_ERROR(10065i32);
pub const WSAENOTEMPTY: WSA_ERROR = WSA_ERROR(10066i32);
pub const WSAEPROCLIM: WSA_ERROR = WSA_ERROR(10067i32);
pub const WSAEUSERS: WSA_ERROR = WSA_ERROR(10068i32);
pub const WSAEDQUOT: WSA_ERROR = WSA_ERROR(10069i32);
pub const WSAESTALE: WSA_ERROR = WSA_ERROR(10070i32);
pub const WSAEREMOTE: WSA_ERROR = WSA_ERROR(10071i32);
pub const WSASYSNOTREADY: WSA_ERROR = WSA_ERROR(10091i32);
pub const WSAVERNOTSUPPORTED: WSA_ERROR = WSA_ERROR(10092i32);
pub const WSANOTINITIALISED: WSA_ERROR = WSA_ERROR(10093i32);
pub const WSAEDISCON: WSA_ERROR = WSA_ERROR(10101i32);
pub const WSAENOMORE: WSA_ERROR = WSA_ERROR(10102i32);
pub const WSAECANCELLED: WSA_ERROR = WSA_ERROR(10103i32);
pub const WSAEINVALIDPROCTABLE: WSA_ERROR = WSA_ERROR(10104i32);
pub const WSAEINVALIDPROVIDER: WSA_ERROR = WSA_ERROR(10105i32);
pub const WSAEPROVIDERFAILEDINIT: WSA_ERROR = WSA_ERROR(10106i32);
pub const WSASYSCALLFAILURE: WSA_ERROR = WSA_ERROR(10107i32);
pub const WSASERVICE_NOT_FOUND: WSA_ERROR = WSA_ERROR(10108i32);
pub const WSATYPE_NOT_FOUND: WSA_ERROR = WSA_ERROR(10109i32);
pub const WSA_E_NO_MORE: WSA_ERROR = WSA_ERROR(10110i32);
pub const WSA_E_CANCELLED: WSA_ERROR = WSA_ERROR(10111i32);
pub const WSAEREFUSED: WSA_ERROR = WSA_ERROR(10112i32);
pub const WSAHOST_NOT_FOUND: WSA_ERROR = WSA_ERROR(11001i32);
pub const WSATRY_AGAIN: WSA_ERROR = WSA_ERROR(11002i32);
pub const WSANO_RECOVERY: WSA_ERROR = WSA_ERROR(11003i32);
pub const WSANO_DATA: WSA_ERROR = WSA_ERROR(11004i32);
pub const WSA_QOS_RECEIVERS: WSA_ERROR = WSA_ERROR(11005i32);
pub const WSA_QOS_SENDERS: WSA_ERROR = WSA_ERROR(11006i32);
pub const WSA_QOS_NO_SENDERS: WSA_ERROR = WSA_ERROR(11007i32);
pub const WSA_QOS_NO_RECEIVERS: WSA_ERROR = WSA_ERROR(11008i32);
pub const WSA_QOS_REQUEST_CONFIRMED: WSA_ERROR = WSA_ERROR(11009i32);
pub const WSA_QOS_ADMISSION_FAILURE: WSA_ERROR = WSA_ERROR(11010i32);
pub const WSA_QOS_POLICY_FAILURE: WSA_ERROR = WSA_ERROR(11011i32);
pub const WSA_QOS_BAD_STYLE: WSA_ERROR = WSA_ERROR(11012i32);
pub const WSA_QOS_BAD_OBJECT: WSA_ERROR = WSA_ERROR(11013i32);
pub const WSA_QOS_TRAFFIC_CTRL_ERROR: WSA_ERROR = WSA_ERROR(11014i32);
pub const WSA_QOS_GENERIC_ERROR: WSA_ERROR = WSA_ERROR(11015i32);
pub const WSA_QOS_ESERVICETYPE: WSA_ERROR = WSA_ERROR(11016i32);
pub const WSA_QOS_EFLOWSPEC: WSA_ERROR = WSA_ERROR(11017i32);
pub const WSA_QOS_EPROVSPECBUF: WSA_ERROR = WSA_ERROR(11018i32);
pub const WSA_QOS_EFILTERSTYLE: WSA_ERROR = WSA_ERROR(11019i32);
pub const WSA_QOS_EFILTERTYPE: WSA_ERROR = WSA_ERROR(11020i32);
pub const WSA_QOS_EFILTERCOUNT: WSA_ERROR = WSA_ERROR(11021i32);
pub const WSA_QOS_EOBJLENGTH: WSA_ERROR = WSA_ERROR(11022i32);
pub const WSA_QOS_EFLOWCOUNT: WSA_ERROR = WSA_ERROR(11023i32);
pub const WSA_QOS_EUNKOWNPSOBJ: WSA_ERROR = WSA_ERROR(11024i32);
pub const WSA_QOS_EPOLICYOBJ: WSA_ERROR = WSA_ERROR(11025i32);
pub const WSA_QOS_EFLOWDESC: WSA_ERROR = WSA_ERROR(11026i32);
pub const WSA_QOS_EPSFLOWSPEC: WSA_ERROR = WSA_ERROR(11027i32);
pub const WSA_QOS_EPSFILTERSPEC: WSA_ERROR = WSA_ERROR(11028i32);
pub const WSA_QOS_ESDMODEOBJ: WSA_ERROR = WSA_ERROR(11029i32);
pub const WSA_QOS_ESHAPERATEOBJ: WSA_ERROR = WSA_ERROR(11030i32);
pub const WSA_QOS_RESERVED_PETYPE: WSA_ERROR = WSA_ERROR(11031i32);
pub const WSA_SECURE_HOST_NOT_FOUND: WSA_ERROR = WSA_ERROR(11032i32);
pub const WSA_IPSEC_NAME_POLICY_ERROR: WSA_ERROR = WSA_ERROR(11033i32);
impl ::core::marker::Copy for WSA_ERROR {}
impl ::core::clone::Clone for WSA_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSA_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSA_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSA_ERROR").field(&self.0).finish()
    }
}
impl FromIntoMemory for WSA_ERROR {
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
pub const WSA_FLAG_ACCESS_SYSTEM_SECURITY: u32 = 64u32;
pub const WSA_FLAG_MULTIPOINT_C_LEAF: u32 = 4u32;
pub const WSA_FLAG_MULTIPOINT_C_ROOT: u32 = 2u32;
pub const WSA_FLAG_MULTIPOINT_D_LEAF: u32 = 16u32;
pub const WSA_FLAG_MULTIPOINT_D_ROOT: u32 = 8u32;
pub const WSA_FLAG_NO_HANDLE_INHERIT: u32 = 128u32;
pub const WSA_FLAG_OVERLAPPED: u32 = 1u32;
pub const WSA_FLAG_REGISTERED_IO: u32 = 256u32;
pub const WSA_INFINITE: u32 = 4294967295u32;
pub const WSA_MAXIMUM_WAIT_EVENTS: u32 = 64u32;
pub const WSA_WAIT_EVENT_0: u32 = 0u32;
pub const WSA_WAIT_FAILED: u32 = 4294967295u32;
pub const WSA_WAIT_IO_COMPLETION: u32 = 192u32;
pub struct WSC_PROVIDER_AUDIT_INFO {
    pub RecordSize: u32,
    pub Reserved: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for WSC_PROVIDER_AUDIT_INFO {}
impl ::core::clone::Clone for WSC_PROVIDER_AUDIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSC_PROVIDER_AUDIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSC_PROVIDER_AUDIT_INFO")
            .field("RecordSize", &self.RecordSize)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSC_PROVIDER_AUDIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.RecordSize == other.RecordSize && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WSC_PROVIDER_AUDIT_INFO {}
impl FromIntoMemory for WSC_PROVIDER_AUDIT_INFO {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_RecordSize = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Reserved =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            RecordSize: f_RecordSize,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.RecordSize, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSC_PROVIDER_INFO_TYPE(pub i32);
pub const ProviderInfoLspCategories: WSC_PROVIDER_INFO_TYPE = WSC_PROVIDER_INFO_TYPE(0i32);
pub const ProviderInfoAudit: WSC_PROVIDER_INFO_TYPE = WSC_PROVIDER_INFO_TYPE(1i32);
impl ::core::marker::Copy for WSC_PROVIDER_INFO_TYPE {}
impl ::core::clone::Clone for WSC_PROVIDER_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_PROVIDER_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSC_PROVIDER_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_PROVIDER_INFO_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for WSC_PROVIDER_INFO_TYPE {
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
pub const WSK_SO_BASE: u32 = 16384u32;
pub const WSPDESCRIPTION_LEN: u32 = 255u32;
pub struct WSPData {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub szDescription: [u16; 256],
}
impl ::core::marker::Copy for WSPData {}
impl ::core::clone::Clone for WSPData {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSPData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSPData")
            .field("wVersion", &self.wVersion)
            .field("wHighVersion", &self.wHighVersion)
            .field("szDescription", &self.szDescription)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSPData {
    fn eq(&self, other: &Self) -> bool {
        self.wVersion == other.wVersion
            && self.wHighVersion == other.wHighVersion
            && self.szDescription == other.szDescription
    }
}
impl ::core::cmp::Eq for WSPData {}
impl FromIntoMemory for WSPData {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 260);
        let f_wVersion = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_wHighVersion = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_szDescription = <[u16; 256] as FromIntoMemory>::from_bytes(&from[4..4 + 256]);
        Self {
            wVersion: f_wVersion,
            wHighVersion: f_wHighVersion,
            szDescription: f_szDescription,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 260);
        FromIntoMemory::into_bytes(self.wVersion, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.wHighVersion, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.szDescription, &mut into[4..4 + 256]);
    }
    fn size() -> usize {
        260
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS', 'Windows.Win32.System.IO'*"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct WSPPROC_TABLE {
    pub lpWSPAccept: LPWSPACCEPT,
    pub lpWSPAddressToString: LPWSPADDRESSTOSTRING,
    pub lpWSPAsyncSelect: LPWSPASYNCSELECT,
    pub lpWSPBind: LPWSPBIND,
    pub lpWSPCancelBlockingCall: LPWSPCANCELBLOCKINGCALL,
    pub lpWSPCleanup: LPWSPCLEANUP,
    pub lpWSPCloseSocket: LPWSPCLOSESOCKET,
    pub lpWSPConnect: LPWSPCONNECT,
    pub lpWSPDuplicateSocket: LPWSPDUPLICATESOCKET,
    pub lpWSPEnumNetworkEvents: LPWSPENUMNETWORKEVENTS,
    pub lpWSPEventSelect: LPWSPEVENTSELECT,
    pub lpWSPGetOverlappedResult: LPWSPGETOVERLAPPEDRESULT,
    pub lpWSPGetPeerName: LPWSPGETPEERNAME,
    pub lpWSPGetSockName: LPWSPGETSOCKNAME,
    pub lpWSPGetSockOpt: LPWSPGETSOCKOPT,
    pub lpWSPGetQOSByName: LPWSPGETQOSBYNAME,
    pub lpWSPIoctl: LPWSPIOCTL,
    pub lpWSPJoinLeaf: LPWSPJOINLEAF,
    pub lpWSPListen: LPWSPLISTEN,
    pub lpWSPRecv: LPWSPRECV,
    pub lpWSPRecvDisconnect: LPWSPRECVDISCONNECT,
    pub lpWSPRecvFrom: LPWSPRECVFROM,
    pub lpWSPSelect: LPWSPSELECT,
    pub lpWSPSend: LPWSPSEND,
    pub lpWSPSendDisconnect: LPWSPSENDDISCONNECT,
    pub lpWSPSendTo: LPWSPSENDTO,
    pub lpWSPSetSockOpt: LPWSPSETSOCKOPT,
    pub lpWSPShutdown: LPWSPSHUTDOWN,
    pub lpWSPSocket: LPWSPSOCKET,
    pub lpWSPStringToAddress: LPWSPSTRINGTOADDRESS,
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS', 'Windows.Win32.System.IO'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for WSPPROC_TABLE {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS', 'Windows.Win32.System.IO'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for WSPPROC_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS', 'Windows.Win32.System.IO'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for WSPPROC_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSPPROC_TABLE")
            .field("lpWSPAccept", &self.lpWSPAccept)
            .field("lpWSPAddressToString", &self.lpWSPAddressToString)
            .field("lpWSPAsyncSelect", &self.lpWSPAsyncSelect)
            .field("lpWSPBind", &self.lpWSPBind)
            .field("lpWSPCancelBlockingCall", &self.lpWSPCancelBlockingCall)
            .field("lpWSPCleanup", &self.lpWSPCleanup)
            .field("lpWSPCloseSocket", &self.lpWSPCloseSocket)
            .field("lpWSPConnect", &self.lpWSPConnect)
            .field("lpWSPDuplicateSocket", &self.lpWSPDuplicateSocket)
            .field("lpWSPEnumNetworkEvents", &self.lpWSPEnumNetworkEvents)
            .field("lpWSPEventSelect", &self.lpWSPEventSelect)
            .field("lpWSPGetOverlappedResult", &self.lpWSPGetOverlappedResult)
            .field("lpWSPGetPeerName", &self.lpWSPGetPeerName)
            .field("lpWSPGetSockName", &self.lpWSPGetSockName)
            .field("lpWSPGetSockOpt", &self.lpWSPGetSockOpt)
            .field("lpWSPGetQOSByName", &self.lpWSPGetQOSByName)
            .field("lpWSPIoctl", &self.lpWSPIoctl)
            .field("lpWSPJoinLeaf", &self.lpWSPJoinLeaf)
            .field("lpWSPListen", &self.lpWSPListen)
            .field("lpWSPRecv", &self.lpWSPRecv)
            .field("lpWSPRecvDisconnect", &self.lpWSPRecvDisconnect)
            .field("lpWSPRecvFrom", &self.lpWSPRecvFrom)
            .field("lpWSPSelect", &self.lpWSPSelect)
            .field("lpWSPSend", &self.lpWSPSend)
            .field("lpWSPSendDisconnect", &self.lpWSPSendDisconnect)
            .field("lpWSPSendTo", &self.lpWSPSendTo)
            .field("lpWSPSetSockOpt", &self.lpWSPSetSockOpt)
            .field("lpWSPShutdown", &self.lpWSPShutdown)
            .field("lpWSPSocket", &self.lpWSPSocket)
            .field("lpWSPStringToAddress", &self.lpWSPStringToAddress)
            .finish()
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS', 'Windows.Win32.System.IO'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for WSPPROC_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.lpWSPAccept == other.lpWSPAccept
            && self.lpWSPAddressToString == other.lpWSPAddressToString
            && self.lpWSPAsyncSelect == other.lpWSPAsyncSelect
            && self.lpWSPBind == other.lpWSPBind
            && self.lpWSPCancelBlockingCall == other.lpWSPCancelBlockingCall
            && self.lpWSPCleanup == other.lpWSPCleanup
            && self.lpWSPCloseSocket == other.lpWSPCloseSocket
            && self.lpWSPConnect == other.lpWSPConnect
            && self.lpWSPDuplicateSocket == other.lpWSPDuplicateSocket
            && self.lpWSPEnumNetworkEvents == other.lpWSPEnumNetworkEvents
            && self.lpWSPEventSelect == other.lpWSPEventSelect
            && self.lpWSPGetOverlappedResult == other.lpWSPGetOverlappedResult
            && self.lpWSPGetPeerName == other.lpWSPGetPeerName
            && self.lpWSPGetSockName == other.lpWSPGetSockName
            && self.lpWSPGetSockOpt == other.lpWSPGetSockOpt
            && self.lpWSPGetQOSByName == other.lpWSPGetQOSByName
            && self.lpWSPIoctl == other.lpWSPIoctl
            && self.lpWSPJoinLeaf == other.lpWSPJoinLeaf
            && self.lpWSPListen == other.lpWSPListen
            && self.lpWSPRecv == other.lpWSPRecv
            && self.lpWSPRecvDisconnect == other.lpWSPRecvDisconnect
            && self.lpWSPRecvFrom == other.lpWSPRecvFrom
            && self.lpWSPSelect == other.lpWSPSelect
            && self.lpWSPSend == other.lpWSPSend
            && self.lpWSPSendDisconnect == other.lpWSPSendDisconnect
            && self.lpWSPSendTo == other.lpWSPSendTo
            && self.lpWSPSetSockOpt == other.lpWSPSetSockOpt
            && self.lpWSPShutdown == other.lpWSPShutdown
            && self.lpWSPSocket == other.lpWSPSocket
            && self.lpWSPStringToAddress == other.lpWSPStringToAddress
    }
}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS', 'Windows.Win32.System.IO'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for WSPPROC_TABLE {}
#[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS', 'Windows.Win32.System.IO'*"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for WSPPROC_TABLE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 120);
        let f_lpWSPAccept = <LPWSPACCEPT as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpWSPAddressToString =
            <LPWSPADDRESSTOSTRING as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpWSPAsyncSelect = <LPWSPASYNCSELECT as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpWSPBind = <LPWSPBIND as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpWSPCancelBlockingCall =
            <LPWSPCANCELBLOCKINGCALL as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpWSPCleanup = <LPWSPCLEANUP as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_lpWSPCloseSocket =
            <LPWSPCLOSESOCKET as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lpWSPConnect = <LPWSPCONNECT as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_lpWSPDuplicateSocket =
            <LPWSPDUPLICATESOCKET as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_lpWSPEnumNetworkEvents =
            <LPWSPENUMNETWORKEVENTS as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_lpWSPEventSelect =
            <LPWSPEVENTSELECT as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_lpWSPGetOverlappedResult =
            <LPWSPGETOVERLAPPEDRESULT as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_lpWSPGetPeerName =
            <LPWSPGETPEERNAME as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_lpWSPGetSockName =
            <LPWSPGETSOCKNAME as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_lpWSPGetSockOpt = <LPWSPGETSOCKOPT as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_lpWSPGetQOSByName =
            <LPWSPGETQOSBYNAME as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_lpWSPIoctl = <LPWSPIOCTL as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_lpWSPJoinLeaf = <LPWSPJOINLEAF as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_lpWSPListen = <LPWSPLISTEN as FromIntoMemory>::from_bytes(&from[72..72 + 4]);
        let f_lpWSPRecv = <LPWSPRECV as FromIntoMemory>::from_bytes(&from[76..76 + 4]);
        let f_lpWSPRecvDisconnect =
            <LPWSPRECVDISCONNECT as FromIntoMemory>::from_bytes(&from[80..80 + 4]);
        let f_lpWSPRecvFrom = <LPWSPRECVFROM as FromIntoMemory>::from_bytes(&from[84..84 + 4]);
        let f_lpWSPSelect = <LPWSPSELECT as FromIntoMemory>::from_bytes(&from[88..88 + 4]);
        let f_lpWSPSend = <LPWSPSEND as FromIntoMemory>::from_bytes(&from[92..92 + 4]);
        let f_lpWSPSendDisconnect =
            <LPWSPSENDDISCONNECT as FromIntoMemory>::from_bytes(&from[96..96 + 4]);
        let f_lpWSPSendTo = <LPWSPSENDTO as FromIntoMemory>::from_bytes(&from[100..100 + 4]);
        let f_lpWSPSetSockOpt =
            <LPWSPSETSOCKOPT as FromIntoMemory>::from_bytes(&from[104..104 + 4]);
        let f_lpWSPShutdown = <LPWSPSHUTDOWN as FromIntoMemory>::from_bytes(&from[108..108 + 4]);
        let f_lpWSPSocket = <LPWSPSOCKET as FromIntoMemory>::from_bytes(&from[112..112 + 4]);
        let f_lpWSPStringToAddress =
            <LPWSPSTRINGTOADDRESS as FromIntoMemory>::from_bytes(&from[116..116 + 4]);
        Self {
            lpWSPAccept: f_lpWSPAccept,
            lpWSPAddressToString: f_lpWSPAddressToString,
            lpWSPAsyncSelect: f_lpWSPAsyncSelect,
            lpWSPBind: f_lpWSPBind,
            lpWSPCancelBlockingCall: f_lpWSPCancelBlockingCall,
            lpWSPCleanup: f_lpWSPCleanup,
            lpWSPCloseSocket: f_lpWSPCloseSocket,
            lpWSPConnect: f_lpWSPConnect,
            lpWSPDuplicateSocket: f_lpWSPDuplicateSocket,
            lpWSPEnumNetworkEvents: f_lpWSPEnumNetworkEvents,
            lpWSPEventSelect: f_lpWSPEventSelect,
            lpWSPGetOverlappedResult: f_lpWSPGetOverlappedResult,
            lpWSPGetPeerName: f_lpWSPGetPeerName,
            lpWSPGetSockName: f_lpWSPGetSockName,
            lpWSPGetSockOpt: f_lpWSPGetSockOpt,
            lpWSPGetQOSByName: f_lpWSPGetQOSByName,
            lpWSPIoctl: f_lpWSPIoctl,
            lpWSPJoinLeaf: f_lpWSPJoinLeaf,
            lpWSPListen: f_lpWSPListen,
            lpWSPRecv: f_lpWSPRecv,
            lpWSPRecvDisconnect: f_lpWSPRecvDisconnect,
            lpWSPRecvFrom: f_lpWSPRecvFrom,
            lpWSPSelect: f_lpWSPSelect,
            lpWSPSend: f_lpWSPSend,
            lpWSPSendDisconnect: f_lpWSPSendDisconnect,
            lpWSPSendTo: f_lpWSPSendTo,
            lpWSPSetSockOpt: f_lpWSPSetSockOpt,
            lpWSPShutdown: f_lpWSPShutdown,
            lpWSPSocket: f_lpWSPSocket,
            lpWSPStringToAddress: f_lpWSPStringToAddress,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 120);
        FromIntoMemory::into_bytes(self.lpWSPAccept, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPAddressToString, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPAsyncSelect, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPBind, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPCancelBlockingCall, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPCleanup, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPCloseSocket, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPConnect, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPDuplicateSocket, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPEnumNetworkEvents, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPEventSelect, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPGetOverlappedResult, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPGetPeerName, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPGetSockName, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPGetSockOpt, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPGetQOSByName, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPIoctl, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPJoinLeaf, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPListen, &mut into[72..72 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPRecv, &mut into[76..76 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPRecvDisconnect, &mut into[80..80 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPRecvFrom, &mut into[84..84 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPSelect, &mut into[88..88 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPSend, &mut into[92..92 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPSendDisconnect, &mut into[96..96 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPSendTo, &mut into[100..100 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPSetSockOpt, &mut into[104..104 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPShutdown, &mut into[108..108 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPSocket, &mut into[112..112 + 4]);
        FromIntoMemory::into_bytes(self.lpWSPStringToAddress, &mut into[116..116 + 4]);
    }
    fn size() -> usize {
        120
    }
}
pub struct WSPUPCALLTABLE {
    pub lpWPUCloseEvent: LPWPUCLOSEEVENT,
    pub lpWPUCloseSocketHandle: LPWPUCLOSESOCKETHANDLE,
    pub lpWPUCreateEvent: LPWPUCREATEEVENT,
    pub lpWPUCreateSocketHandle: LPWPUCREATESOCKETHANDLE,
    pub lpWPUFDIsSet: LPWPUFDISSET,
    pub lpWPUGetProviderPath: LPWPUGETPROVIDERPATH,
    pub lpWPUModifyIFSHandle: LPWPUMODIFYIFSHANDLE,
    pub lpWPUPostMessage: LPWPUPOSTMESSAGE,
    pub lpWPUQueryBlockingCallback: LPWPUQUERYBLOCKINGCALLBACK,
    pub lpWPUQuerySocketHandleContext: LPWPUQUERYSOCKETHANDLECONTEXT,
    pub lpWPUQueueApc: LPWPUQUEUEAPC,
    pub lpWPUResetEvent: LPWPURESETEVENT,
    pub lpWPUSetEvent: LPWPUSETEVENT,
    pub lpWPUOpenCurrentThread: LPWPUOPENCURRENTTHREAD,
    pub lpWPUCloseThread: LPWPUCLOSETHREAD,
}
impl ::core::marker::Copy for WSPUPCALLTABLE {}
impl ::core::clone::Clone for WSPUPCALLTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WSPUPCALLTABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSPUPCALLTABLE")
            .field("lpWPUCloseEvent", &self.lpWPUCloseEvent)
            .field("lpWPUCloseSocketHandle", &self.lpWPUCloseSocketHandle)
            .field("lpWPUCreateEvent", &self.lpWPUCreateEvent)
            .field("lpWPUCreateSocketHandle", &self.lpWPUCreateSocketHandle)
            .field("lpWPUFDIsSet", &self.lpWPUFDIsSet)
            .field("lpWPUGetProviderPath", &self.lpWPUGetProviderPath)
            .field("lpWPUModifyIFSHandle", &self.lpWPUModifyIFSHandle)
            .field("lpWPUPostMessage", &self.lpWPUPostMessage)
            .field(
                "lpWPUQueryBlockingCallback",
                &self.lpWPUQueryBlockingCallback,
            )
            .field(
                "lpWPUQuerySocketHandleContext",
                &self.lpWPUQuerySocketHandleContext,
            )
            .field("lpWPUQueueApc", &self.lpWPUQueueApc)
            .field("lpWPUResetEvent", &self.lpWPUResetEvent)
            .field("lpWPUSetEvent", &self.lpWPUSetEvent)
            .field("lpWPUOpenCurrentThread", &self.lpWPUOpenCurrentThread)
            .field("lpWPUCloseThread", &self.lpWPUCloseThread)
            .finish()
    }
}
impl ::core::cmp::PartialEq for WSPUPCALLTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.lpWPUCloseEvent == other.lpWPUCloseEvent
            && self.lpWPUCloseSocketHandle == other.lpWPUCloseSocketHandle
            && self.lpWPUCreateEvent == other.lpWPUCreateEvent
            && self.lpWPUCreateSocketHandle == other.lpWPUCreateSocketHandle
            && self.lpWPUFDIsSet == other.lpWPUFDIsSet
            && self.lpWPUGetProviderPath == other.lpWPUGetProviderPath
            && self.lpWPUModifyIFSHandle == other.lpWPUModifyIFSHandle
            && self.lpWPUPostMessage == other.lpWPUPostMessage
            && self.lpWPUQueryBlockingCallback == other.lpWPUQueryBlockingCallback
            && self.lpWPUQuerySocketHandleContext == other.lpWPUQuerySocketHandleContext
            && self.lpWPUQueueApc == other.lpWPUQueueApc
            && self.lpWPUResetEvent == other.lpWPUResetEvent
            && self.lpWPUSetEvent == other.lpWPUSetEvent
            && self.lpWPUOpenCurrentThread == other.lpWPUOpenCurrentThread
            && self.lpWPUCloseThread == other.lpWPUCloseThread
    }
}
impl ::core::cmp::Eq for WSPUPCALLTABLE {}
impl FromIntoMemory for WSPUPCALLTABLE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 60);
        let f_lpWPUCloseEvent = <LPWPUCLOSEEVENT as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpWPUCloseSocketHandle =
            <LPWPUCLOSESOCKETHANDLE as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_lpWPUCreateEvent = <LPWPUCREATEEVENT as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_lpWPUCreateSocketHandle =
            <LPWPUCREATESOCKETHANDLE as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_lpWPUFDIsSet = <LPWPUFDISSET as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_lpWPUGetProviderPath =
            <LPWPUGETPROVIDERPATH as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_lpWPUModifyIFSHandle =
            <LPWPUMODIFYIFSHANDLE as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_lpWPUPostMessage =
            <LPWPUPOSTMESSAGE as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_lpWPUQueryBlockingCallback =
            <LPWPUQUERYBLOCKINGCALLBACK as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_lpWPUQuerySocketHandleContext =
            <LPWPUQUERYSOCKETHANDLECONTEXT as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_lpWPUQueueApc = <LPWPUQUEUEAPC as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_lpWPUResetEvent = <LPWPURESETEVENT as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_lpWPUSetEvent = <LPWPUSETEVENT as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_lpWPUOpenCurrentThread =
            <LPWPUOPENCURRENTTHREAD as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_lpWPUCloseThread =
            <LPWPUCLOSETHREAD as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        Self {
            lpWPUCloseEvent: f_lpWPUCloseEvent,
            lpWPUCloseSocketHandle: f_lpWPUCloseSocketHandle,
            lpWPUCreateEvent: f_lpWPUCreateEvent,
            lpWPUCreateSocketHandle: f_lpWPUCreateSocketHandle,
            lpWPUFDIsSet: f_lpWPUFDIsSet,
            lpWPUGetProviderPath: f_lpWPUGetProviderPath,
            lpWPUModifyIFSHandle: f_lpWPUModifyIFSHandle,
            lpWPUPostMessage: f_lpWPUPostMessage,
            lpWPUQueryBlockingCallback: f_lpWPUQueryBlockingCallback,
            lpWPUQuerySocketHandleContext: f_lpWPUQuerySocketHandleContext,
            lpWPUQueueApc: f_lpWPUQueueApc,
            lpWPUResetEvent: f_lpWPUResetEvent,
            lpWPUSetEvent: f_lpWPUSetEvent,
            lpWPUOpenCurrentThread: f_lpWPUOpenCurrentThread,
            lpWPUCloseThread: f_lpWPUCloseThread,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 60);
        FromIntoMemory::into_bytes(self.lpWPUCloseEvent, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpWPUCloseSocketHandle, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.lpWPUCreateEvent, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.lpWPUCreateSocketHandle, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.lpWPUFDIsSet, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.lpWPUGetProviderPath, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.lpWPUModifyIFSHandle, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.lpWPUPostMessage, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.lpWPUQueryBlockingCallback, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.lpWPUQuerySocketHandleContext, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.lpWPUQueueApc, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.lpWPUResetEvent, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.lpWPUSetEvent, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.lpWPUOpenCurrentThread, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.lpWPUCloseThread, &mut into[56..56 + 4]);
    }
    fn size() -> usize {
        60
    }
}
pub const WSS_OPERATION_IN_PROGRESS: i32 = 259i32;
pub const XP1_CONNECTIONLESS: u32 = 1u32;
pub const XP1_CONNECT_DATA: u32 = 128u32;
pub const XP1_DISCONNECT_DATA: u32 = 256u32;
pub const XP1_EXPEDITED_DATA: u32 = 64u32;
pub const XP1_GRACEFUL_CLOSE: u32 = 32u32;
pub const XP1_GUARANTEED_DELIVERY: u32 = 2u32;
pub const XP1_GUARANTEED_ORDER: u32 = 4u32;
pub const XP1_IFS_HANDLES: u32 = 131072u32;
pub const XP1_INTERRUPT: u32 = 16384u32;
pub const XP1_MESSAGE_ORIENTED: u32 = 8u32;
pub const XP1_MULTIPOINT_CONTROL_PLANE: u32 = 2048u32;
pub const XP1_MULTIPOINT_DATA_PLANE: u32 = 4096u32;
pub const XP1_PARTIAL_MESSAGE: u32 = 262144u32;
pub const XP1_PSEUDO_STREAM: u32 = 16u32;
pub const XP1_QOS_SUPPORTED: u32 = 8192u32;
pub const XP1_SAN_SUPPORT_SDP: u32 = 524288u32;
pub const XP1_SUPPORT_BROADCAST: u32 = 512u32;
pub const XP1_SUPPORT_MULTIPOINT: u32 = 1024u32;
pub const XP1_UNI_RECV: u32 = 65536u32;
pub const XP1_UNI_SEND: u32 = 32768u32;
pub const XP_BANDWIDTH_ALLOCATION: u32 = 2048u32;
pub const XP_CONNECTIONLESS: u32 = 1u32;
pub const XP_CONNECT_DATA: u32 = 128u32;
pub const XP_DISCONNECT_DATA: u32 = 256u32;
pub const XP_ENCRYPTS: u32 = 8192u32;
pub const XP_EXPEDITED_DATA: u32 = 64u32;
pub const XP_FRAGMENTATION: u32 = 4096u32;
pub const XP_GRACEFUL_CLOSE: u32 = 32u32;
pub const XP_GUARANTEED_DELIVERY: u32 = 2u32;
pub const XP_GUARANTEED_ORDER: u32 = 4u32;
pub const XP_MESSAGE_ORIENTED: u32 = 8u32;
pub const XP_PSEUDO_STREAM: u32 = 16u32;
pub const XP_SUPPORTS_BROADCAST: u32 = 512u32;
pub const XP_SUPPORTS_MULTICAST: u32 = 1024u32;
pub const _SS_MAXSIZE: u32 = 128u32;
pub struct addrinfoW {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: PtrRepr,
    pub ai_canonname: PWSTR,
    pub ai_addr: MutPtr<SOCKADDR>,
    pub ai_next: MutPtr<addrinfoW>,
}
impl ::core::marker::Copy for addrinfoW {}
impl ::core::clone::Clone for addrinfoW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for addrinfoW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("addrinfoW")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_next", &self.ai_next)
            .finish()
    }
}
impl ::core::cmp::PartialEq for addrinfoW {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags
            && self.ai_family == other.ai_family
            && self.ai_socktype == other.ai_socktype
            && self.ai_protocol == other.ai_protocol
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_canonname == other.ai_canonname
            && self.ai_addr == other.ai_addr
            && self.ai_next == other.ai_next
    }
}
impl ::core::cmp::Eq for addrinfoW {}
impl FromIntoMemory for addrinfoW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_ai_flags = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ai_family = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ai_socktype = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ai_protocol = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ai_addrlen = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ai_canonname = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ai_addr = <MutPtr<SOCKADDR> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ai_next = <MutPtr<addrinfoW> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            ai_flags: f_ai_flags,
            ai_family: f_ai_family,
            ai_socktype: f_ai_socktype,
            ai_protocol: f_ai_protocol,
            ai_addrlen: f_ai_addrlen,
            ai_canonname: f_ai_canonname,
            ai_addr: f_ai_addr,
            ai_next: f_ai_next,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.ai_flags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ai_family, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ai_socktype, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ai_protocol, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ai_addrlen, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ai_canonname, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ai_addr, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ai_next, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct addrinfo_dns_server {
    pub ai_servertype: u32,
    pub ai_flags: u64,
    pub ai_addrlen: u32,
    pub ai_addr: MutPtr<SOCKADDR>,
    pub Anonymous: addrinfo_dns_server_0,
}
impl ::core::marker::Copy for addrinfo_dns_server {}
impl ::core::clone::Clone for addrinfo_dns_server {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for addrinfo_dns_server {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("addrinfo_dns_server")
            .field("ai_servertype", &self.ai_servertype)
            .field("ai_flags", &self.ai_flags)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_addr", &self.ai_addr)
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for addrinfo_dns_server {
    fn eq(&self, other: &Self) -> bool {
        self.ai_servertype == other.ai_servertype
            && self.ai_flags == other.ai_flags
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_addr == other.ai_addr
            && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for addrinfo_dns_server {}
impl FromIntoMemory for addrinfo_dns_server {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_ai_servertype = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ai_flags = <u64 as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_ai_addrlen = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ai_addr = <MutPtr<SOCKADDR> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_Anonymous = <addrinfo_dns_server_0 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        Self {
            ai_servertype: f_ai_servertype,
            ai_flags: f_ai_flags,
            ai_addrlen: f_ai_addrlen,
            ai_addr: f_ai_addr,
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.ai_servertype, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ai_flags, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.ai_addrlen, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ai_addr, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[24..24 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct addrinfo_dns_server_0 {
    data: [u8; 4],
}
impl ::core::default::Default for addrinfo_dns_server_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for addrinfo_dns_server_0 {}
impl ::core::clone::Clone for addrinfo_dns_server_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for addrinfo_dns_server_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("addrinfo_dns_server_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for addrinfo_dns_server_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for addrinfo_dns_server_0 {}
impl FromIntoMemory for addrinfo_dns_server_0 {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 4];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        4
    }
}
pub struct addrinfoex2A {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: PtrRepr,
    pub ai_canonname: PSTR,
    pub ai_addr: MutPtr<SOCKADDR>,
    pub ai_blob: MutPtr<::core::ffi::c_void>,
    pub ai_bloblen: PtrRepr,
    pub ai_provider: MutPtr<crate::core::GUID>,
    pub ai_next: MutPtr<addrinfoex2A>,
    pub ai_version: i32,
    pub ai_fqdn: PSTR,
}
impl ::core::marker::Copy for addrinfoex2A {}
impl ::core::clone::Clone for addrinfoex2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for addrinfoex2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("addrinfoex2A")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .finish()
    }
}
impl ::core::cmp::PartialEq for addrinfoex2A {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags
            && self.ai_family == other.ai_family
            && self.ai_socktype == other.ai_socktype
            && self.ai_protocol == other.ai_protocol
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_canonname == other.ai_canonname
            && self.ai_addr == other.ai_addr
            && self.ai_blob == other.ai_blob
            && self.ai_bloblen == other.ai_bloblen
            && self.ai_provider == other.ai_provider
            && self.ai_next == other.ai_next
            && self.ai_version == other.ai_version
            && self.ai_fqdn == other.ai_fqdn
    }
}
impl ::core::cmp::Eq for addrinfoex2A {}
impl FromIntoMemory for addrinfoex2A {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52);
        let f_ai_flags = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ai_family = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ai_socktype = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ai_protocol = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ai_addrlen = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ai_canonname = <PSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ai_addr = <MutPtr<SOCKADDR> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ai_blob =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_ai_bloblen = <PtrRepr as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_ai_provider =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_ai_next = <MutPtr<addrinfoex2A> as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_ai_version = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_ai_fqdn = <PSTR as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        Self {
            ai_flags: f_ai_flags,
            ai_family: f_ai_family,
            ai_socktype: f_ai_socktype,
            ai_protocol: f_ai_protocol,
            ai_addrlen: f_ai_addrlen,
            ai_canonname: f_ai_canonname,
            ai_addr: f_ai_addr,
            ai_blob: f_ai_blob,
            ai_bloblen: f_ai_bloblen,
            ai_provider: f_ai_provider,
            ai_next: f_ai_next,
            ai_version: f_ai_version,
            ai_fqdn: f_ai_fqdn,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52);
        FromIntoMemory::into_bytes(self.ai_flags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ai_family, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ai_socktype, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ai_protocol, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ai_addrlen, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ai_canonname, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ai_addr, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ai_blob, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ai_bloblen, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.ai_provider, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.ai_next, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.ai_version, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.ai_fqdn, &mut into[48..48 + 4]);
    }
    fn size() -> usize {
        52
    }
}
pub struct addrinfoex2W {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: PtrRepr,
    pub ai_canonname: PWSTR,
    pub ai_addr: MutPtr<SOCKADDR>,
    pub ai_blob: MutPtr<::core::ffi::c_void>,
    pub ai_bloblen: PtrRepr,
    pub ai_provider: MutPtr<crate::core::GUID>,
    pub ai_next: MutPtr<addrinfoex2W>,
    pub ai_version: i32,
    pub ai_fqdn: PWSTR,
}
impl ::core::marker::Copy for addrinfoex2W {}
impl ::core::clone::Clone for addrinfoex2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for addrinfoex2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("addrinfoex2W")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .finish()
    }
}
impl ::core::cmp::PartialEq for addrinfoex2W {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags
            && self.ai_family == other.ai_family
            && self.ai_socktype == other.ai_socktype
            && self.ai_protocol == other.ai_protocol
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_canonname == other.ai_canonname
            && self.ai_addr == other.ai_addr
            && self.ai_blob == other.ai_blob
            && self.ai_bloblen == other.ai_bloblen
            && self.ai_provider == other.ai_provider
            && self.ai_next == other.ai_next
            && self.ai_version == other.ai_version
            && self.ai_fqdn == other.ai_fqdn
    }
}
impl ::core::cmp::Eq for addrinfoex2W {}
impl FromIntoMemory for addrinfoex2W {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 52);
        let f_ai_flags = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ai_family = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ai_socktype = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ai_protocol = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ai_addrlen = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ai_canonname = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ai_addr = <MutPtr<SOCKADDR> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ai_blob =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_ai_bloblen = <PtrRepr as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_ai_provider =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_ai_next = <MutPtr<addrinfoex2W> as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_ai_version = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_ai_fqdn = <PWSTR as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        Self {
            ai_flags: f_ai_flags,
            ai_family: f_ai_family,
            ai_socktype: f_ai_socktype,
            ai_protocol: f_ai_protocol,
            ai_addrlen: f_ai_addrlen,
            ai_canonname: f_ai_canonname,
            ai_addr: f_ai_addr,
            ai_blob: f_ai_blob,
            ai_bloblen: f_ai_bloblen,
            ai_provider: f_ai_provider,
            ai_next: f_ai_next,
            ai_version: f_ai_version,
            ai_fqdn: f_ai_fqdn,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 52);
        FromIntoMemory::into_bytes(self.ai_flags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ai_family, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ai_socktype, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ai_protocol, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ai_addrlen, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ai_canonname, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ai_addr, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ai_blob, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ai_bloblen, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.ai_provider, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.ai_next, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.ai_version, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.ai_fqdn, &mut into[48..48 + 4]);
    }
    fn size() -> usize {
        52
    }
}
pub struct addrinfoex3 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: PtrRepr,
    pub ai_canonname: PWSTR,
    pub ai_addr: MutPtr<SOCKADDR>,
    pub ai_blob: MutPtr<::core::ffi::c_void>,
    pub ai_bloblen: PtrRepr,
    pub ai_provider: MutPtr<crate::core::GUID>,
    pub ai_next: MutPtr<addrinfoex3>,
    pub ai_version: i32,
    pub ai_fqdn: PWSTR,
    pub ai_interfaceindex: i32,
}
impl ::core::marker::Copy for addrinfoex3 {}
impl ::core::clone::Clone for addrinfoex3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for addrinfoex3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("addrinfoex3")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .field("ai_interfaceindex", &self.ai_interfaceindex)
            .finish()
    }
}
impl ::core::cmp::PartialEq for addrinfoex3 {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags
            && self.ai_family == other.ai_family
            && self.ai_socktype == other.ai_socktype
            && self.ai_protocol == other.ai_protocol
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_canonname == other.ai_canonname
            && self.ai_addr == other.ai_addr
            && self.ai_blob == other.ai_blob
            && self.ai_bloblen == other.ai_bloblen
            && self.ai_provider == other.ai_provider
            && self.ai_next == other.ai_next
            && self.ai_version == other.ai_version
            && self.ai_fqdn == other.ai_fqdn
            && self.ai_interfaceindex == other.ai_interfaceindex
    }
}
impl ::core::cmp::Eq for addrinfoex3 {}
impl FromIntoMemory for addrinfoex3 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 56);
        let f_ai_flags = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ai_family = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ai_socktype = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ai_protocol = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ai_addrlen = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ai_canonname = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ai_addr = <MutPtr<SOCKADDR> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ai_blob =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_ai_bloblen = <PtrRepr as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_ai_provider =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_ai_next = <MutPtr<addrinfoex3> as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_ai_version = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_ai_fqdn = <PWSTR as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_ai_interfaceindex = <i32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        Self {
            ai_flags: f_ai_flags,
            ai_family: f_ai_family,
            ai_socktype: f_ai_socktype,
            ai_protocol: f_ai_protocol,
            ai_addrlen: f_ai_addrlen,
            ai_canonname: f_ai_canonname,
            ai_addr: f_ai_addr,
            ai_blob: f_ai_blob,
            ai_bloblen: f_ai_bloblen,
            ai_provider: f_ai_provider,
            ai_next: f_ai_next,
            ai_version: f_ai_version,
            ai_fqdn: f_ai_fqdn,
            ai_interfaceindex: f_ai_interfaceindex,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 56);
        FromIntoMemory::into_bytes(self.ai_flags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ai_family, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ai_socktype, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ai_protocol, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ai_addrlen, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ai_canonname, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ai_addr, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ai_blob, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ai_bloblen, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.ai_provider, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.ai_next, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.ai_version, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.ai_fqdn, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.ai_interfaceindex, &mut into[52..52 + 4]);
    }
    fn size() -> usize {
        56
    }
}
pub struct addrinfoex4 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: PtrRepr,
    pub ai_canonname: PWSTR,
    pub ai_addr: MutPtr<SOCKADDR>,
    pub ai_blob: MutPtr<::core::ffi::c_void>,
    pub ai_bloblen: PtrRepr,
    pub ai_provider: MutPtr<crate::core::GUID>,
    pub ai_next: MutPtr<addrinfoex4>,
    pub ai_version: i32,
    pub ai_fqdn: PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: super::super::Foundation::HANDLE,
}
impl ::core::marker::Copy for addrinfoex4 {}
impl ::core::clone::Clone for addrinfoex4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for addrinfoex4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("addrinfoex4")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .field("ai_interfaceindex", &self.ai_interfaceindex)
            .field("ai_resolutionhandle", &self.ai_resolutionhandle)
            .finish()
    }
}
impl ::core::cmp::PartialEq for addrinfoex4 {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags
            && self.ai_family == other.ai_family
            && self.ai_socktype == other.ai_socktype
            && self.ai_protocol == other.ai_protocol
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_canonname == other.ai_canonname
            && self.ai_addr == other.ai_addr
            && self.ai_blob == other.ai_blob
            && self.ai_bloblen == other.ai_bloblen
            && self.ai_provider == other.ai_provider
            && self.ai_next == other.ai_next
            && self.ai_version == other.ai_version
            && self.ai_fqdn == other.ai_fqdn
            && self.ai_interfaceindex == other.ai_interfaceindex
            && self.ai_resolutionhandle == other.ai_resolutionhandle
    }
}
impl ::core::cmp::Eq for addrinfoex4 {}
impl FromIntoMemory for addrinfoex4 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 60);
        let f_ai_flags = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ai_family = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ai_socktype = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ai_protocol = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ai_addrlen = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ai_canonname = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ai_addr = <MutPtr<SOCKADDR> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ai_blob =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_ai_bloblen = <PtrRepr as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_ai_provider =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_ai_next = <MutPtr<addrinfoex4> as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_ai_version = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_ai_fqdn = <PWSTR as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_ai_interfaceindex = <i32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_ai_resolutionhandle =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        Self {
            ai_flags: f_ai_flags,
            ai_family: f_ai_family,
            ai_socktype: f_ai_socktype,
            ai_protocol: f_ai_protocol,
            ai_addrlen: f_ai_addrlen,
            ai_canonname: f_ai_canonname,
            ai_addr: f_ai_addr,
            ai_blob: f_ai_blob,
            ai_bloblen: f_ai_bloblen,
            ai_provider: f_ai_provider,
            ai_next: f_ai_next,
            ai_version: f_ai_version,
            ai_fqdn: f_ai_fqdn,
            ai_interfaceindex: f_ai_interfaceindex,
            ai_resolutionhandle: f_ai_resolutionhandle,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 60);
        FromIntoMemory::into_bytes(self.ai_flags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ai_family, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ai_socktype, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ai_protocol, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ai_addrlen, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ai_canonname, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ai_addr, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ai_blob, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ai_bloblen, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.ai_provider, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.ai_next, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.ai_version, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.ai_fqdn, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.ai_interfaceindex, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.ai_resolutionhandle, &mut into[56..56 + 4]);
    }
    fn size() -> usize {
        60
    }
}
pub struct addrinfoex5 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: PtrRepr,
    pub ai_canonname: PWSTR,
    pub ai_addr: MutPtr<SOCKADDR>,
    pub ai_blob: MutPtr<::core::ffi::c_void>,
    pub ai_bloblen: PtrRepr,
    pub ai_provider: MutPtr<crate::core::GUID>,
    pub ai_next: MutPtr<addrinfoex5>,
    pub ai_version: i32,
    pub ai_fqdn: PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: super::super::Foundation::HANDLE,
    pub ai_ttl: u32,
}
impl ::core::marker::Copy for addrinfoex5 {}
impl ::core::clone::Clone for addrinfoex5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for addrinfoex5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("addrinfoex5")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .field("ai_interfaceindex", &self.ai_interfaceindex)
            .field("ai_resolutionhandle", &self.ai_resolutionhandle)
            .field("ai_ttl", &self.ai_ttl)
            .finish()
    }
}
impl ::core::cmp::PartialEq for addrinfoex5 {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags
            && self.ai_family == other.ai_family
            && self.ai_socktype == other.ai_socktype
            && self.ai_protocol == other.ai_protocol
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_canonname == other.ai_canonname
            && self.ai_addr == other.ai_addr
            && self.ai_blob == other.ai_blob
            && self.ai_bloblen == other.ai_bloblen
            && self.ai_provider == other.ai_provider
            && self.ai_next == other.ai_next
            && self.ai_version == other.ai_version
            && self.ai_fqdn == other.ai_fqdn
            && self.ai_interfaceindex == other.ai_interfaceindex
            && self.ai_resolutionhandle == other.ai_resolutionhandle
            && self.ai_ttl == other.ai_ttl
    }
}
impl ::core::cmp::Eq for addrinfoex5 {}
impl FromIntoMemory for addrinfoex5 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 64);
        let f_ai_flags = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ai_family = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ai_socktype = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ai_protocol = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ai_addrlen = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ai_canonname = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ai_addr = <MutPtr<SOCKADDR> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ai_blob =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_ai_bloblen = <PtrRepr as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_ai_provider =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_ai_next = <MutPtr<addrinfoex5> as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_ai_version = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_ai_fqdn = <PWSTR as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_ai_interfaceindex = <i32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_ai_resolutionhandle =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_ai_ttl = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        Self {
            ai_flags: f_ai_flags,
            ai_family: f_ai_family,
            ai_socktype: f_ai_socktype,
            ai_protocol: f_ai_protocol,
            ai_addrlen: f_ai_addrlen,
            ai_canonname: f_ai_canonname,
            ai_addr: f_ai_addr,
            ai_blob: f_ai_blob,
            ai_bloblen: f_ai_bloblen,
            ai_provider: f_ai_provider,
            ai_next: f_ai_next,
            ai_version: f_ai_version,
            ai_fqdn: f_ai_fqdn,
            ai_interfaceindex: f_ai_interfaceindex,
            ai_resolutionhandle: f_ai_resolutionhandle,
            ai_ttl: f_ai_ttl,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 64);
        FromIntoMemory::into_bytes(self.ai_flags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ai_family, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ai_socktype, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ai_protocol, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ai_addrlen, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ai_canonname, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ai_addr, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ai_blob, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ai_bloblen, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.ai_provider, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.ai_next, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.ai_version, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.ai_fqdn, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.ai_interfaceindex, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.ai_resolutionhandle, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.ai_ttl, &mut into[60..60 + 4]);
    }
    fn size() -> usize {
        64
    }
}
pub struct addrinfoex6 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: PtrRepr,
    pub ai_canonname: PWSTR,
    pub ai_addr: MutPtr<SOCKADDR>,
    pub ai_blob: MutPtr<::core::ffi::c_void>,
    pub ai_bloblen: PtrRepr,
    pub ai_provider: MutPtr<crate::core::GUID>,
    pub ai_next: MutPtr<addrinfoex5>,
    pub ai_version: i32,
    pub ai_fqdn: PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: super::super::Foundation::HANDLE,
    pub ai_ttl: u32,
    pub ai_numservers: u32,
    pub ai_servers: MutPtr<addrinfo_dns_server>,
    pub ai_responseflags: u64,
}
impl ::core::marker::Copy for addrinfoex6 {}
impl ::core::clone::Clone for addrinfoex6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for addrinfoex6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("addrinfoex6")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .field("ai_version", &self.ai_version)
            .field("ai_fqdn", &self.ai_fqdn)
            .field("ai_interfaceindex", &self.ai_interfaceindex)
            .field("ai_resolutionhandle", &self.ai_resolutionhandle)
            .field("ai_ttl", &self.ai_ttl)
            .field("ai_numservers", &self.ai_numservers)
            .field("ai_servers", &self.ai_servers)
            .field("ai_responseflags", &self.ai_responseflags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for addrinfoex6 {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags
            && self.ai_family == other.ai_family
            && self.ai_socktype == other.ai_socktype
            && self.ai_protocol == other.ai_protocol
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_canonname == other.ai_canonname
            && self.ai_addr == other.ai_addr
            && self.ai_blob == other.ai_blob
            && self.ai_bloblen == other.ai_bloblen
            && self.ai_provider == other.ai_provider
            && self.ai_next == other.ai_next
            && self.ai_version == other.ai_version
            && self.ai_fqdn == other.ai_fqdn
            && self.ai_interfaceindex == other.ai_interfaceindex
            && self.ai_resolutionhandle == other.ai_resolutionhandle
            && self.ai_ttl == other.ai_ttl
            && self.ai_numservers == other.ai_numservers
            && self.ai_servers == other.ai_servers
            && self.ai_responseflags == other.ai_responseflags
    }
}
impl ::core::cmp::Eq for addrinfoex6 {}
impl FromIntoMemory for addrinfoex6 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 80);
        let f_ai_flags = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ai_family = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ai_socktype = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ai_protocol = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ai_addrlen = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ai_canonname = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ai_addr = <MutPtr<SOCKADDR> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ai_blob =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_ai_bloblen = <PtrRepr as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_ai_provider =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_ai_next = <MutPtr<addrinfoex5> as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_ai_version = <i32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_ai_fqdn = <PWSTR as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_ai_interfaceindex = <i32 as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        let f_ai_resolutionhandle =
            <super::super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[56..56 + 4]);
        let f_ai_ttl = <u32 as FromIntoMemory>::from_bytes(&from[60..60 + 4]);
        let f_ai_numservers = <u32 as FromIntoMemory>::from_bytes(&from[64..64 + 4]);
        let f_ai_servers =
            <MutPtr<addrinfo_dns_server> as FromIntoMemory>::from_bytes(&from[68..68 + 4]);
        let f_ai_responseflags = <u64 as FromIntoMemory>::from_bytes(&from[72..72 + 8]);
        Self {
            ai_flags: f_ai_flags,
            ai_family: f_ai_family,
            ai_socktype: f_ai_socktype,
            ai_protocol: f_ai_protocol,
            ai_addrlen: f_ai_addrlen,
            ai_canonname: f_ai_canonname,
            ai_addr: f_ai_addr,
            ai_blob: f_ai_blob,
            ai_bloblen: f_ai_bloblen,
            ai_provider: f_ai_provider,
            ai_next: f_ai_next,
            ai_version: f_ai_version,
            ai_fqdn: f_ai_fqdn,
            ai_interfaceindex: f_ai_interfaceindex,
            ai_resolutionhandle: f_ai_resolutionhandle,
            ai_ttl: f_ai_ttl,
            ai_numservers: f_ai_numservers,
            ai_servers: f_ai_servers,
            ai_responseflags: f_ai_responseflags,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 80);
        FromIntoMemory::into_bytes(self.ai_flags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ai_family, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ai_socktype, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ai_protocol, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ai_addrlen, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ai_canonname, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ai_addr, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ai_blob, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ai_bloblen, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.ai_provider, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.ai_next, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.ai_version, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.ai_fqdn, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.ai_interfaceindex, &mut into[52..52 + 4]);
        FromIntoMemory::into_bytes(self.ai_resolutionhandle, &mut into[56..56 + 4]);
        FromIntoMemory::into_bytes(self.ai_ttl, &mut into[60..60 + 4]);
        FromIntoMemory::into_bytes(self.ai_numservers, &mut into[64..64 + 4]);
        FromIntoMemory::into_bytes(self.ai_servers, &mut into[68..68 + 4]);
        FromIntoMemory::into_bytes(self.ai_responseflags, &mut into[72..72 + 8]);
    }
    fn size() -> usize {
        80
    }
}
pub struct addrinfoexA {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: PtrRepr,
    pub ai_canonname: PSTR,
    pub ai_addr: MutPtr<SOCKADDR>,
    pub ai_blob: MutPtr<::core::ffi::c_void>,
    pub ai_bloblen: PtrRepr,
    pub ai_provider: MutPtr<crate::core::GUID>,
    pub ai_next: MutPtr<addrinfoexA>,
}
impl ::core::marker::Copy for addrinfoexA {}
impl ::core::clone::Clone for addrinfoexA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for addrinfoexA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("addrinfoexA")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .finish()
    }
}
impl ::core::cmp::PartialEq for addrinfoexA {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags
            && self.ai_family == other.ai_family
            && self.ai_socktype == other.ai_socktype
            && self.ai_protocol == other.ai_protocol
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_canonname == other.ai_canonname
            && self.ai_addr == other.ai_addr
            && self.ai_blob == other.ai_blob
            && self.ai_bloblen == other.ai_bloblen
            && self.ai_provider == other.ai_provider
            && self.ai_next == other.ai_next
    }
}
impl ::core::cmp::Eq for addrinfoexA {}
impl FromIntoMemory for addrinfoexA {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_ai_flags = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ai_family = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ai_socktype = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ai_protocol = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ai_addrlen = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ai_canonname = <PSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ai_addr = <MutPtr<SOCKADDR> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ai_blob =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_ai_bloblen = <PtrRepr as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_ai_provider =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_ai_next = <MutPtr<addrinfoexA> as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        Self {
            ai_flags: f_ai_flags,
            ai_family: f_ai_family,
            ai_socktype: f_ai_socktype,
            ai_protocol: f_ai_protocol,
            ai_addrlen: f_ai_addrlen,
            ai_canonname: f_ai_canonname,
            ai_addr: f_ai_addr,
            ai_blob: f_ai_blob,
            ai_bloblen: f_ai_bloblen,
            ai_provider: f_ai_provider,
            ai_next: f_ai_next,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.ai_flags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ai_family, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ai_socktype, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ai_protocol, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ai_addrlen, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ai_canonname, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ai_addr, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ai_blob, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ai_bloblen, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.ai_provider, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.ai_next, &mut into[40..40 + 4]);
    }
    fn size() -> usize {
        44
    }
}
pub struct addrinfoexW {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: PtrRepr,
    pub ai_canonname: PWSTR,
    pub ai_addr: MutPtr<SOCKADDR>,
    pub ai_blob: MutPtr<::core::ffi::c_void>,
    pub ai_bloblen: PtrRepr,
    pub ai_provider: MutPtr<crate::core::GUID>,
    pub ai_next: MutPtr<addrinfoexW>,
}
impl ::core::marker::Copy for addrinfoexW {}
impl ::core::clone::Clone for addrinfoexW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for addrinfoexW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("addrinfoexW")
            .field("ai_flags", &self.ai_flags)
            .field("ai_family", &self.ai_family)
            .field("ai_socktype", &self.ai_socktype)
            .field("ai_protocol", &self.ai_protocol)
            .field("ai_addrlen", &self.ai_addrlen)
            .field("ai_canonname", &self.ai_canonname)
            .field("ai_addr", &self.ai_addr)
            .field("ai_blob", &self.ai_blob)
            .field("ai_bloblen", &self.ai_bloblen)
            .field("ai_provider", &self.ai_provider)
            .field("ai_next", &self.ai_next)
            .finish()
    }
}
impl ::core::cmp::PartialEq for addrinfoexW {
    fn eq(&self, other: &Self) -> bool {
        self.ai_flags == other.ai_flags
            && self.ai_family == other.ai_family
            && self.ai_socktype == other.ai_socktype
            && self.ai_protocol == other.ai_protocol
            && self.ai_addrlen == other.ai_addrlen
            && self.ai_canonname == other.ai_canonname
            && self.ai_addr == other.ai_addr
            && self.ai_blob == other.ai_blob
            && self.ai_bloblen == other.ai_bloblen
            && self.ai_provider == other.ai_provider
            && self.ai_next == other.ai_next
    }
}
impl ::core::cmp::Eq for addrinfoexW {}
impl FromIntoMemory for addrinfoexW {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_ai_flags = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ai_family = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ai_socktype = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ai_protocol = <i32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_ai_addrlen = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_ai_canonname = <PWSTR as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ai_addr = <MutPtr<SOCKADDR> as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ai_blob =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_ai_bloblen = <PtrRepr as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_ai_provider =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_ai_next = <MutPtr<addrinfoexW> as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        Self {
            ai_flags: f_ai_flags,
            ai_family: f_ai_family,
            ai_socktype: f_ai_socktype,
            ai_protocol: f_ai_protocol,
            ai_addrlen: f_ai_addrlen,
            ai_canonname: f_ai_canonname,
            ai_addr: f_ai_addr,
            ai_blob: f_ai_blob,
            ai_bloblen: f_ai_bloblen,
            ai_provider: f_ai_provider,
            ai_next: f_ai_next,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.ai_flags, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ai_family, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ai_socktype, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ai_protocol, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.ai_addrlen, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.ai_canonname, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ai_addr, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ai_blob, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.ai_bloblen, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.ai_provider, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.ai_next, &mut into[40..40 + 4]);
    }
    fn size() -> usize {
        44
    }
}
pub struct cmsghdr {
    pub cmsg_len: PtrRepr,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
}
impl ::core::marker::Copy for cmsghdr {}
impl ::core::clone::Clone for cmsghdr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for cmsghdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("cmsghdr")
            .field("cmsg_len", &self.cmsg_len)
            .field("cmsg_level", &self.cmsg_level)
            .field("cmsg_type", &self.cmsg_type)
            .finish()
    }
}
impl ::core::cmp::PartialEq for cmsghdr {
    fn eq(&self, other: &Self) -> bool {
        self.cmsg_len == other.cmsg_len
            && self.cmsg_level == other.cmsg_level
            && self.cmsg_type == other.cmsg_type
    }
}
impl ::core::cmp::Eq for cmsghdr {}
impl FromIntoMemory for cmsghdr {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_cmsg_len = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_cmsg_level = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_cmsg_type = <i32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            cmsg_len: f_cmsg_len,
            cmsg_level: f_cmsg_level,
            cmsg_type: f_cmsg_type,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.cmsg_len, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.cmsg_level, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.cmsg_type, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct eWINDOW_ADVANCE_METHOD(pub i32);
pub const E_WINDOW_ADVANCE_BY_TIME: eWINDOW_ADVANCE_METHOD = eWINDOW_ADVANCE_METHOD(1i32);
pub const E_WINDOW_USE_AS_DATA_CACHE: eWINDOW_ADVANCE_METHOD = eWINDOW_ADVANCE_METHOD(2i32);
impl ::core::marker::Copy for eWINDOW_ADVANCE_METHOD {}
impl ::core::clone::Clone for eWINDOW_ADVANCE_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for eWINDOW_ADVANCE_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eWINDOW_ADVANCE_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eWINDOW_ADVANCE_METHOD")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for eWINDOW_ADVANCE_METHOD {
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
pub struct fd_set {
    pub fd_count: u32,
    pub fd_array: [SOCKET; 64],
}
impl ::core::marker::Copy for fd_set {}
impl ::core::clone::Clone for fd_set {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for fd_set {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("fd_set")
            .field("fd_count", &self.fd_count)
            .field("fd_array", &self.fd_array)
            .finish()
    }
}
impl ::core::cmp::PartialEq for fd_set {
    fn eq(&self, other: &Self) -> bool {
        self.fd_count == other.fd_count && self.fd_array == other.fd_array
    }
}
impl ::core::cmp::Eq for fd_set {}
impl FromIntoMemory for fd_set {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 260);
        let f_fd_count = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_fd_array = <[SOCKET; 64] as FromIntoMemory>::from_bytes(&from[4..4 + 256]);
        Self {
            fd_count: f_fd_count,
            fd_array: f_fd_array,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 260);
        FromIntoMemory::into_bytes(self.fd_count, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.fd_array, &mut into[4..4 + 256]);
    }
    fn size() -> usize {
        260
    }
}
pub struct hostent {
    pub h_name: PSTR,
    pub h_aliases: MutPtr<ConstPtr<i8>>,
    pub h_addrtype: i16,
    pub h_length: i16,
    pub h_addr_list: MutPtr<ConstPtr<i8>>,
}
impl ::core::marker::Copy for hostent {}
impl ::core::clone::Clone for hostent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for hostent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("hostent")
            .field("h_name", &self.h_name)
            .field("h_aliases", &self.h_aliases)
            .field("h_addrtype", &self.h_addrtype)
            .field("h_length", &self.h_length)
            .field("h_addr_list", &self.h_addr_list)
            .finish()
    }
}
impl ::core::cmp::PartialEq for hostent {
    fn eq(&self, other: &Self) -> bool {
        self.h_name == other.h_name
            && self.h_aliases == other.h_aliases
            && self.h_addrtype == other.h_addrtype
            && self.h_length == other.h_length
            && self.h_addr_list == other.h_addr_list
    }
}
impl ::core::cmp::Eq for hostent {}
impl FromIntoMemory for hostent {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_h_name = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_h_aliases = <MutPtr<ConstPtr<i8>> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_h_addrtype = <i16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_h_length = <i16 as FromIntoMemory>::from_bytes(&from[10..10 + 2]);
        let f_h_addr_list = <MutPtr<ConstPtr<i8>> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            h_name: f_h_name,
            h_aliases: f_h_aliases,
            h_addrtype: f_h_addrtype,
            h_length: f_h_length,
            h_addr_list: f_h_addr_list,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.h_name, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.h_aliases, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.h_addrtype, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.h_length, &mut into[10..10 + 2]);
        FromIntoMemory::into_bytes(self.h_addr_list, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct in6_pktinfo_ex {
    pub pkt_info: IN6_PKTINFO,
    pub scope_id: SCOPE_ID,
}
impl ::core::marker::Copy for in6_pktinfo_ex {}
impl ::core::clone::Clone for in6_pktinfo_ex {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for in6_pktinfo_ex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("in6_pktinfo_ex")
            .field("pkt_info", &self.pkt_info)
            .field("scope_id", &self.scope_id)
            .finish()
    }
}
impl ::core::cmp::PartialEq for in6_pktinfo_ex {
    fn eq(&self, other: &Self) -> bool {
        self.pkt_info == other.pkt_info && self.scope_id == other.scope_id
    }
}
impl ::core::cmp::Eq for in6_pktinfo_ex {}
impl FromIntoMemory for in6_pktinfo_ex {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_pkt_info = <IN6_PKTINFO as FromIntoMemory>::from_bytes(&from[0..0 + 20]);
        let f_scope_id = <SCOPE_ID as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            pkt_info: f_pkt_info,
            scope_id: f_scope_id,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.pkt_info, &mut into[0..0 + 20]);
        FromIntoMemory::into_bytes(self.scope_id, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct linger {
    pub l_onoff: u16,
    pub l_linger: u16,
}
impl ::core::marker::Copy for linger {}
impl ::core::clone::Clone for linger {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for linger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("linger")
            .field("l_onoff", &self.l_onoff)
            .field("l_linger", &self.l_linger)
            .finish()
    }
}
impl ::core::cmp::PartialEq for linger {
    fn eq(&self, other: &Self) -> bool {
        self.l_onoff == other.l_onoff && self.l_linger == other.l_linger
    }
}
impl ::core::cmp::Eq for linger {}
impl FromIntoMemory for linger {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_l_onoff = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_l_linger = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            l_onoff: f_l_onoff,
            l_linger: f_l_linger,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.l_onoff, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.l_linger, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4
    }
}
pub struct netent {
    pub n_name: PSTR,
    pub n_aliases: MutPtr<ConstPtr<i8>>,
    pub n_addrtype: i16,
    pub n_net: u32,
}
impl ::core::marker::Copy for netent {}
impl ::core::clone::Clone for netent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for netent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("netent")
            .field("n_name", &self.n_name)
            .field("n_aliases", &self.n_aliases)
            .field("n_addrtype", &self.n_addrtype)
            .field("n_net", &self.n_net)
            .finish()
    }
}
impl ::core::cmp::PartialEq for netent {
    fn eq(&self, other: &Self) -> bool {
        self.n_name == other.n_name
            && self.n_aliases == other.n_aliases
            && self.n_addrtype == other.n_addrtype
            && self.n_net == other.n_net
    }
}
impl ::core::cmp::Eq for netent {}
impl FromIntoMemory for netent {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_n_name = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_n_aliases = <MutPtr<ConstPtr<i8>> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_n_addrtype = <i16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_n_net = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            n_name: f_n_name,
            n_aliases: f_n_aliases,
            n_addrtype: f_n_addrtype,
            n_net: f_n_net,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.n_name, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.n_aliases, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.n_addrtype, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.n_net, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct protoent {
    pub p_name: PSTR,
    pub p_aliases: MutPtr<ConstPtr<i8>>,
    pub p_proto: i16,
}
impl ::core::marker::Copy for protoent {}
impl ::core::clone::Clone for protoent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for protoent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("protoent")
            .field("p_name", &self.p_name)
            .field("p_aliases", &self.p_aliases)
            .field("p_proto", &self.p_proto)
            .finish()
    }
}
impl ::core::cmp::PartialEq for protoent {
    fn eq(&self, other: &Self) -> bool {
        self.p_name == other.p_name
            && self.p_aliases == other.p_aliases
            && self.p_proto == other.p_proto
    }
}
impl ::core::cmp::Eq for protoent {}
impl FromIntoMemory for protoent {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_p_name = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_p_aliases = <MutPtr<ConstPtr<i8>> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_p_proto = <i16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        Self {
            p_name: f_p_name,
            p_aliases: f_p_aliases,
            p_proto: f_p_proto,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.p_name, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.p_aliases, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.p_proto, &mut into[8..8 + 2]);
    }
    fn size() -> usize {
        12
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
pub struct servent {
    pub s_name: PSTR,
    pub s_aliases: MutPtr<ConstPtr<i8>>,
    pub s_proto: PSTR,
    pub s_port: i16,
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::marker::Copy for servent {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::clone::Clone for servent {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::fmt::Debug for servent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("servent")
            .field("s_name", &self.s_name)
            .field("s_aliases", &self.s_aliases)
            .field("s_proto", &self.s_proto)
            .field("s_port", &self.s_port)
            .finish()
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::PartialEq for servent {
    fn eq(&self, other: &Self) -> bool {
        self.s_name == other.s_name
            && self.s_aliases == other.s_aliases
            && self.s_proto == other.s_proto
            && self.s_port == other.s_port
    }
}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl ::core::cmp::Eq for servent {}
#[doc = "*Required namespaces: *"]
#[cfg(dummy_option_that_does_not_exist)]
impl FromIntoMemory for servent {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_s_name = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_s_aliases = <MutPtr<ConstPtr<i8>> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_s_proto = <PSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_s_port = <i16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        Self {
            s_name: f_s_name,
            s_aliases: f_s_aliases,
            s_proto: f_s_proto,
            s_port: f_s_port,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.s_name, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.s_aliases, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.s_proto, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.s_port, &mut into[12..12 + 2]);
    }
    fn size() -> usize {
        16
    }
}
pub struct servent {
    pub s_name: PSTR,
    pub s_aliases: MutPtr<ConstPtr<i8>>,
    pub s_port: i16,
    pub s_proto: PSTR,
}
impl ::core::marker::Copy for servent {}
impl ::core::clone::Clone for servent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for servent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("servent")
            .field("s_name", &self.s_name)
            .field("s_aliases", &self.s_aliases)
            .field("s_port", &self.s_port)
            .field("s_proto", &self.s_proto)
            .finish()
    }
}
impl ::core::cmp::PartialEq for servent {
    fn eq(&self, other: &Self) -> bool {
        self.s_name == other.s_name
            && self.s_aliases == other.s_aliases
            && self.s_port == other.s_port
            && self.s_proto == other.s_proto
    }
}
impl ::core::cmp::Eq for servent {}
impl FromIntoMemory for servent {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_s_name = <PSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_s_aliases = <MutPtr<ConstPtr<i8>> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_s_port = <i16 as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_s_proto = <PSTR as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            s_name: f_s_name,
            s_aliases: f_s_aliases,
            s_port: f_s_port,
            s_proto: f_s_proto,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.s_name, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.s_aliases, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.s_port, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.s_proto, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct sockaddr_atm {
    pub satm_family: u16,
    pub satm_number: ATM_ADDRESS,
    pub satm_blli: ATM_BLLI,
    pub satm_bhli: ATM_BHLI,
}
impl ::core::marker::Copy for sockaddr_atm {}
impl ::core::clone::Clone for sockaddr_atm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for sockaddr_atm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("sockaddr_atm")
            .field("satm_family", &self.satm_family)
            .field("satm_number", &self.satm_number)
            .field("satm_blli", &self.satm_blli)
            .field("satm_bhli", &self.satm_bhli)
            .finish()
    }
}
impl ::core::cmp::PartialEq for sockaddr_atm {
    fn eq(&self, other: &Self) -> bool {
        self.satm_family == other.satm_family
            && self.satm_number == other.satm_number
            && self.satm_blli == other.satm_blli
            && self.satm_bhli == other.satm_bhli
    }
}
impl ::core::cmp::Eq for sockaddr_atm {}
impl FromIntoMemory for sockaddr_atm {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 76);
        let f_satm_family = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_satm_number = <ATM_ADDRESS as FromIntoMemory>::from_bytes(&from[4..4 + 28]);
        let f_satm_blli = <ATM_BLLI as FromIntoMemory>::from_bytes(&from[32..32 + 28]);
        let f_satm_bhli = <ATM_BHLI as FromIntoMemory>::from_bytes(&from[60..60 + 16]);
        Self {
            satm_family: f_satm_family,
            satm_number: f_satm_number,
            satm_blli: f_satm_blli,
            satm_bhli: f_satm_bhli,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 76);
        FromIntoMemory::into_bytes(self.satm_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.satm_number, &mut into[4..4 + 28]);
        FromIntoMemory::into_bytes(self.satm_blli, &mut into[32..32 + 28]);
        FromIntoMemory::into_bytes(self.satm_bhli, &mut into[60..60 + 16]);
    }
    fn size() -> usize {
        76
    }
}
pub struct sockaddr_gen {
    data: [u8; 24],
}
impl ::core::default::Default for sockaddr_gen {
    fn default() -> Self {
        Self { data: [0u8; 24] }
    }
}
impl ::core::marker::Copy for sockaddr_gen {}
impl ::core::clone::Clone for sockaddr_gen {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for sockaddr_gen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("sockaddr_gen")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for sockaddr_gen {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for sockaddr_gen {}
impl FromIntoMemory for sockaddr_gen {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 24];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        24
    }
}
pub struct sockaddr_in6_old {
    pub sin6_family: i16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: IN6_ADDR,
}
impl ::core::marker::Copy for sockaddr_in6_old {}
impl ::core::clone::Clone for sockaddr_in6_old {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for sockaddr_in6_old {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("sockaddr_in6_old")
            .field("sin6_family", &self.sin6_family)
            .field("sin6_port", &self.sin6_port)
            .field("sin6_flowinfo", &self.sin6_flowinfo)
            .field("sin6_addr", &self.sin6_addr)
            .finish()
    }
}
impl ::core::cmp::PartialEq for sockaddr_in6_old {
    fn eq(&self, other: &Self) -> bool {
        self.sin6_family == other.sin6_family
            && self.sin6_port == other.sin6_port
            && self.sin6_flowinfo == other.sin6_flowinfo
            && self.sin6_addr == other.sin6_addr
    }
}
impl ::core::cmp::Eq for sockaddr_in6_old {}
impl FromIntoMemory for sockaddr_in6_old {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_sin6_family = <i16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_sin6_port = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_sin6_flowinfo = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_sin6_addr = <IN6_ADDR as FromIntoMemory>::from_bytes(&from[8..8 + 16]);
        Self {
            sin6_family: f_sin6_family,
            sin6_port: f_sin6_port,
            sin6_flowinfo: f_sin6_flowinfo,
            sin6_addr: f_sin6_addr,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.sin6_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.sin6_port, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.sin6_flowinfo, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.sin6_addr, &mut into[8..8 + 16]);
    }
    fn size() -> usize {
        24
    }
}
pub struct sockaddr_ipx {
    pub sa_family: i16,
    pub sa_netnum: [super::super::Foundation::CHAR; 4],
    pub sa_nodenum: [super::super::Foundation::CHAR; 6],
    pub sa_socket: u16,
}
impl ::core::marker::Copy for sockaddr_ipx {}
impl ::core::clone::Clone for sockaddr_ipx {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for sockaddr_ipx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("sockaddr_ipx")
            .field("sa_family", &self.sa_family)
            .field("sa_netnum", &self.sa_netnum)
            .field("sa_nodenum", &self.sa_nodenum)
            .field("sa_socket", &self.sa_socket)
            .finish()
    }
}
impl ::core::cmp::PartialEq for sockaddr_ipx {
    fn eq(&self, other: &Self) -> bool {
        self.sa_family == other.sa_family
            && self.sa_netnum == other.sa_netnum
            && self.sa_nodenum == other.sa_nodenum
            && self.sa_socket == other.sa_socket
    }
}
impl ::core::cmp::Eq for sockaddr_ipx {}
impl FromIntoMemory for sockaddr_ipx {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 14);
        let f_sa_family = <i16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_sa_netnum =
            <[super::super::Foundation::CHAR; 4] as FromIntoMemory>::from_bytes(&from[2..2 + 4]);
        let f_sa_nodenum =
            <[super::super::Foundation::CHAR; 6] as FromIntoMemory>::from_bytes(&from[6..6 + 6]);
        let f_sa_socket = <u16 as FromIntoMemory>::from_bytes(&from[12..12 + 2]);
        Self {
            sa_family: f_sa_family,
            sa_netnum: f_sa_netnum,
            sa_nodenum: f_sa_nodenum,
            sa_socket: f_sa_socket,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 14);
        FromIntoMemory::into_bytes(self.sa_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.sa_netnum, &mut into[2..2 + 4]);
        FromIntoMemory::into_bytes(self.sa_nodenum, &mut into[6..6 + 6]);
        FromIntoMemory::into_bytes(self.sa_socket, &mut into[12..12 + 2]);
    }
    fn size() -> usize {
        14
    }
}
pub struct sockaddr_nb {
    pub snb_family: i16,
    pub snb_type: u16,
    pub snb_name: [super::super::Foundation::CHAR; 16],
}
impl ::core::marker::Copy for sockaddr_nb {}
impl ::core::clone::Clone for sockaddr_nb {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for sockaddr_nb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("sockaddr_nb")
            .field("snb_family", &self.snb_family)
            .field("snb_type", &self.snb_type)
            .field("snb_name", &self.snb_name)
            .finish()
    }
}
impl ::core::cmp::PartialEq for sockaddr_nb {
    fn eq(&self, other: &Self) -> bool {
        self.snb_family == other.snb_family
            && self.snb_type == other.snb_type
            && self.snb_name == other.snb_name
    }
}
impl ::core::cmp::Eq for sockaddr_nb {}
impl FromIntoMemory for sockaddr_nb {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_snb_family = <i16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_snb_type = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_snb_name =
            <[super::super::Foundation::CHAR; 16] as FromIntoMemory>::from_bytes(&from[4..4 + 16]);
        Self {
            snb_family: f_snb_family,
            snb_type: f_snb_type,
            snb_name: f_snb_name,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.snb_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.snb_type, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.snb_name, &mut into[4..4 + 16]);
    }
    fn size() -> usize {
        20
    }
}
pub struct sockaddr_tp {
    pub tp_family: u16,
    pub tp_addr_type: u16,
    pub tp_taddr_len: u16,
    pub tp_tsel_len: u16,
    pub tp_addr: [u8; 64],
}
impl ::core::marker::Copy for sockaddr_tp {}
impl ::core::clone::Clone for sockaddr_tp {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for sockaddr_tp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("sockaddr_tp")
            .field("tp_family", &self.tp_family)
            .field("tp_addr_type", &self.tp_addr_type)
            .field("tp_taddr_len", &self.tp_taddr_len)
            .field("tp_tsel_len", &self.tp_tsel_len)
            .field("tp_addr", &self.tp_addr)
            .finish()
    }
}
impl ::core::cmp::PartialEq for sockaddr_tp {
    fn eq(&self, other: &Self) -> bool {
        self.tp_family == other.tp_family
            && self.tp_addr_type == other.tp_addr_type
            && self.tp_taddr_len == other.tp_taddr_len
            && self.tp_tsel_len == other.tp_tsel_len
            && self.tp_addr == other.tp_addr
    }
}
impl ::core::cmp::Eq for sockaddr_tp {}
impl FromIntoMemory for sockaddr_tp {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 72);
        let f_tp_family = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_tp_addr_type = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_tp_taddr_len = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_tp_tsel_len = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_tp_addr = <[u8; 64] as FromIntoMemory>::from_bytes(&from[8..8 + 64]);
        Self {
            tp_family: f_tp_family,
            tp_addr_type: f_tp_addr_type,
            tp_taddr_len: f_tp_taddr_len,
            tp_tsel_len: f_tp_tsel_len,
            tp_addr: f_tp_addr,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 72);
        FromIntoMemory::into_bytes(self.tp_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.tp_addr_type, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.tp_taddr_len, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.tp_tsel_len, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.tp_addr, &mut into[8..8 + 64]);
    }
    fn size() -> usize {
        72
    }
}
pub struct sockaddr_un {
    pub sun_family: u16,
    pub sun_path: [super::super::Foundation::CHAR; 108],
}
impl ::core::marker::Copy for sockaddr_un {}
impl ::core::clone::Clone for sockaddr_un {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for sockaddr_un {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("sockaddr_un")
            .field("sun_family", &self.sun_family)
            .field("sun_path", &self.sun_path)
            .finish()
    }
}
impl ::core::cmp::PartialEq for sockaddr_un {
    fn eq(&self, other: &Self) -> bool {
        self.sun_family == other.sun_family && self.sun_path == other.sun_path
    }
}
impl ::core::cmp::Eq for sockaddr_un {}
impl FromIntoMemory for sockaddr_un {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 110);
        let f_sun_family = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_sun_path = <[super::super::Foundation::CHAR; 108] as FromIntoMemory>::from_bytes(
            &from[2..2 + 108],
        );
        Self {
            sun_family: f_sun_family,
            sun_path: f_sun_path,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 110);
        FromIntoMemory::into_bytes(self.sun_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.sun_path, &mut into[2..2 + 108]);
    }
    fn size() -> usize {
        110
    }
}
pub struct sockaddr_vns {
    pub sin_family: u16,
    pub net_address: [u8; 4],
    pub subnet_addr: [u8; 2],
    pub port: [u8; 2],
    pub hops: u8,
    pub filler: [u8; 5],
}
impl ::core::marker::Copy for sockaddr_vns {}
impl ::core::clone::Clone for sockaddr_vns {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for sockaddr_vns {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("sockaddr_vns")
            .field("sin_family", &self.sin_family)
            .field("net_address", &self.net_address)
            .field("subnet_addr", &self.subnet_addr)
            .field("port", &self.port)
            .field("hops", &self.hops)
            .field("filler", &self.filler)
            .finish()
    }
}
impl ::core::cmp::PartialEq for sockaddr_vns {
    fn eq(&self, other: &Self) -> bool {
        self.sin_family == other.sin_family
            && self.net_address == other.net_address
            && self.subnet_addr == other.subnet_addr
            && self.port == other.port
            && self.hops == other.hops
            && self.filler == other.filler
    }
}
impl ::core::cmp::Eq for sockaddr_vns {}
impl FromIntoMemory for sockaddr_vns {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_sin_family = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_net_address = <[u8; 4] as FromIntoMemory>::from_bytes(&from[2..2 + 4]);
        let f_subnet_addr = <[u8; 2] as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_port = <[u8; 2] as FromIntoMemory>::from_bytes(&from[8..8 + 2]);
        let f_hops = <u8 as FromIntoMemory>::from_bytes(&from[10..10 + 1]);
        let f_filler = <[u8; 5] as FromIntoMemory>::from_bytes(&from[11..11 + 5]);
        Self {
            sin_family: f_sin_family,
            net_address: f_net_address,
            subnet_addr: f_subnet_addr,
            port: f_port,
            hops: f_hops,
            filler: f_filler,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.sin_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.net_address, &mut into[2..2 + 4]);
        FromIntoMemory::into_bytes(self.subnet_addr, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.port, &mut into[8..8 + 2]);
        FromIntoMemory::into_bytes(self.hops, &mut into[10..10 + 1]);
        FromIntoMemory::into_bytes(self.filler, &mut into[11..11 + 5]);
    }
    fn size() -> usize {
        16
    }
}
pub struct sockproto {
    pub sp_family: u16,
    pub sp_protocol: u16,
}
impl ::core::marker::Copy for sockproto {}
impl ::core::clone::Clone for sockproto {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for sockproto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("sockproto")
            .field("sp_family", &self.sp_family)
            .field("sp_protocol", &self.sp_protocol)
            .finish()
    }
}
impl ::core::cmp::PartialEq for sockproto {
    fn eq(&self, other: &Self) -> bool {
        self.sp_family == other.sp_family && self.sp_protocol == other.sp_protocol
    }
}
impl ::core::cmp::Eq for sockproto {}
impl FromIntoMemory for sockproto {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_sp_family = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_sp_protocol = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            sp_family: f_sp_family,
            sp_protocol: f_sp_protocol,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.sp_family, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.sp_protocol, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4
    }
}
pub struct tcp_keepalive {
    pub onoff: u32,
    pub keepalivetime: u32,
    pub keepaliveinterval: u32,
}
impl ::core::marker::Copy for tcp_keepalive {}
impl ::core::clone::Clone for tcp_keepalive {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tcp_keepalive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_keepalive")
            .field("onoff", &self.onoff)
            .field("keepalivetime", &self.keepalivetime)
            .field("keepaliveinterval", &self.keepaliveinterval)
            .finish()
    }
}
impl ::core::cmp::PartialEq for tcp_keepalive {
    fn eq(&self, other: &Self) -> bool {
        self.onoff == other.onoff
            && self.keepalivetime == other.keepalivetime
            && self.keepaliveinterval == other.keepaliveinterval
    }
}
impl ::core::cmp::Eq for tcp_keepalive {}
impl FromIntoMemory for tcp_keepalive {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_onoff = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_keepalivetime = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_keepaliveinterval = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            onoff: f_onoff,
            keepalivetime: f_keepalivetime,
            keepaliveinterval: f_keepaliveinterval,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.onoff, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.keepalivetime, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.keepaliveinterval, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct timeval {
    pub tv_sec: i32,
    pub tv_usec: i32,
}
impl ::core::marker::Copy for timeval {}
impl ::core::clone::Clone for timeval {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for timeval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("timeval")
            .field("tv_sec", &self.tv_sec)
            .field("tv_usec", &self.tv_usec)
            .finish()
    }
}
impl ::core::cmp::PartialEq for timeval {
    fn eq(&self, other: &Self) -> bool {
        self.tv_sec == other.tv_sec && self.tv_usec == other.tv_usec
    }
}
impl ::core::cmp::Eq for timeval {}
impl FromIntoMemory for timeval {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_tv_sec = <i32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_tv_usec = <i32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            tv_sec: f_tv_sec,
            tv_usec: f_tv_usec,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.tv_sec, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.tv_usec, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub trait Api {
    #[doc = "AcceptEx from MSWSOCK"]
    fn AcceptEx(
        &self,
        s_listen_socket: SOCKET,
        s_accept_socket: SOCKET,
        lp_output_buffer: MutPtr<::core::ffi::c_void>,
        dw_receive_data_length: u32,
        dw_local_address_length: u32,
        dw_remote_address_length: u32,
        lpdw_bytes_received: MutPtr<u32>,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!("AcceptEx")
    }
    #[doc = "EnumProtocolsA from MSWSOCK"]
    fn EnumProtocolsA(
        &self,
        lpi_protocols: ConstPtr<i32>,
        lp_protocol_buffer: MutPtr<::core::ffi::c_void>,
        lpdw_buffer_length: MutPtr<u32>,
    ) -> i32 {
        todo!("EnumProtocolsA")
    }
    #[doc = "EnumProtocolsW from MSWSOCK"]
    fn EnumProtocolsW(
        &self,
        lpi_protocols: ConstPtr<i32>,
        lp_protocol_buffer: MutPtr<::core::ffi::c_void>,
        lpdw_buffer_length: MutPtr<u32>,
    ) -> i32 {
        todo!("EnumProtocolsW")
    }
    #[doc = "FreeAddrInfoEx from WS2_32"]
    fn FreeAddrInfoEx(&self, p_addr_info_ex: ConstPtr<addrinfoexA>) {
        todo!("FreeAddrInfoEx")
    }
    #[doc = "FreeAddrInfoExW from WS2_32"]
    fn FreeAddrInfoExW(&self, p_addr_info_ex: ConstPtr<addrinfoexW>) {
        todo!("FreeAddrInfoExW")
    }
    #[doc = "FreeAddrInfoW from WS2_32"]
    fn FreeAddrInfoW(&self, p_addr_info: ConstPtr<addrinfoW>) {
        todo!("FreeAddrInfoW")
    }
    #[doc = "GetAcceptExSockaddrs from MSWSOCK"]
    fn GetAcceptExSockaddrs(
        &self,
        lp_output_buffer: ConstPtr<::core::ffi::c_void>,
        dw_receive_data_length: u32,
        dw_local_address_length: u32,
        dw_remote_address_length: u32,
        local_sockaddr: MutPtr<ConstPtr<SOCKADDR>>,
        local_sockaddr_length: MutPtr<i32>,
        remote_sockaddr: MutPtr<ConstPtr<SOCKADDR>>,
        remote_sockaddr_length: MutPtr<i32>,
    ) {
        todo!("GetAcceptExSockaddrs")
    }
    #[doc = "GetAddrInfoExA from WS2_32"]
    fn GetAddrInfoExA(
        &self,
        p_name: PCSTR,
        p_service_name: PCSTR,
        dw_name_space: u32,
        lp_nsp_id: ConstPtr<crate::core::GUID>,
        hints: ConstPtr<addrinfoexA>,
        pp_result: MutPtr<ConstPtr<addrinfoexA>>,
        timeout: ConstPtr<timeval>,
        lp_overlapped: ConstPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
        lp_name_handle: MutPtr<super::super::Foundation::HANDLE>,
    ) -> i32 {
        todo!("GetAddrInfoExA")
    }
    #[doc = "GetAddrInfoExCancel from WS2_32"]
    fn GetAddrInfoExCancel(&self, lp_handle: ConstPtr<super::super::Foundation::HANDLE>) -> i32 {
        todo!("GetAddrInfoExCancel")
    }
    #[doc = "GetAddrInfoExOverlappedResult from WS2_32"]
    fn GetAddrInfoExOverlappedResult(
        &self,
        lp_overlapped: ConstPtr<super::super::System::IO::OVERLAPPED>,
    ) -> i32 {
        todo!("GetAddrInfoExOverlappedResult")
    }
    #[doc = "GetAddrInfoExW from WS2_32"]
    fn GetAddrInfoExW(
        &self,
        p_name: PCWSTR,
        p_service_name: PCWSTR,
        dw_name_space: u32,
        lp_nsp_id: ConstPtr<crate::core::GUID>,
        hints: ConstPtr<addrinfoexW>,
        pp_result: MutPtr<ConstPtr<addrinfoexW>>,
        timeout: ConstPtr<timeval>,
        lp_overlapped: ConstPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
        lp_handle: MutPtr<super::super::Foundation::HANDLE>,
    ) -> i32 {
        todo!("GetAddrInfoExW")
    }
    #[doc = "GetAddrInfoW from WS2_32"]
    fn GetAddrInfoW(
        &self,
        p_node_name: PCWSTR,
        p_service_name: PCWSTR,
        p_hints: ConstPtr<addrinfoW>,
        pp_result: MutPtr<ConstPtr<addrinfoW>>,
    ) -> i32 {
        todo!("GetAddrInfoW")
    }
    #[doc = "GetAddressByNameA from MSWSOCK"]
    fn GetAddressByNameA(
        &self,
        dw_name_space: u32,
        lp_service_type: ConstPtr<crate::core::GUID>,
        lp_service_name: PCSTR,
        lpi_protocols: ConstPtr<i32>,
        dw_resolution: u32,
        lp_service_async_info: ConstPtr<SERVICE_ASYNC_INFO>,
        lp_csaddr_buffer: MutPtr<::core::ffi::c_void>,
        lpdw_buffer_length: MutPtr<u32>,
        lp_alias_buffer: PSTR,
        lpdw_alias_buffer_length: MutPtr<u32>,
    ) -> i32 {
        todo!("GetAddressByNameA")
    }
    #[doc = "GetAddressByNameW from MSWSOCK"]
    fn GetAddressByNameW(
        &self,
        dw_name_space: u32,
        lp_service_type: ConstPtr<crate::core::GUID>,
        lp_service_name: PCWSTR,
        lpi_protocols: ConstPtr<i32>,
        dw_resolution: u32,
        lp_service_async_info: ConstPtr<SERVICE_ASYNC_INFO>,
        lp_csaddr_buffer: MutPtr<::core::ffi::c_void>,
        lpdw_buffer_length: MutPtr<u32>,
        lp_alias_buffer: PWSTR,
        lpdw_alias_buffer_length: MutPtr<u32>,
    ) -> i32 {
        todo!("GetAddressByNameW")
    }
    #[doc = "GetHostNameW from WS2_32"]
    fn GetHostNameW(&self, name: PWSTR, namelen: i32) -> i32 {
        todo!("GetHostNameW")
    }
    #[doc = "GetNameByTypeA from MSWSOCK"]
    fn GetNameByTypeA(
        &self,
        lp_service_type: ConstPtr<crate::core::GUID>,
        lp_service_name: PSTR,
        dw_name_length: u32,
    ) -> i32 {
        todo!("GetNameByTypeA")
    }
    #[doc = "GetNameByTypeW from MSWSOCK"]
    fn GetNameByTypeW(
        &self,
        lp_service_type: ConstPtr<crate::core::GUID>,
        lp_service_name: PWSTR,
        dw_name_length: u32,
    ) -> i32 {
        todo!("GetNameByTypeW")
    }
    #[doc = "GetNameInfoW from WS2_32"]
    fn GetNameInfoW(
        &self,
        p_sockaddr: ConstPtr<SOCKADDR>,
        sockaddr_length: i32,
        p_node_buffer: PWSTR,
        node_buffer_size: u32,
        p_service_buffer: PWSTR,
        service_buffer_size: u32,
        flags: i32,
    ) -> i32 {
        todo!("GetNameInfoW")
    }
    #[doc = "GetServiceA from MSWSOCK"]
    fn GetServiceA(
        &self,
        dw_name_space: u32,
        lp_guid: ConstPtr<crate::core::GUID>,
        lp_service_name: PCSTR,
        dw_properties: u32,
        lp_buffer: MutPtr<::core::ffi::c_void>,
        lpdw_buffer_size: MutPtr<u32>,
        lp_service_async_info: ConstPtr<SERVICE_ASYNC_INFO>,
    ) -> i32 {
        todo!("GetServiceA")
    }
    #[doc = "GetServiceW from MSWSOCK"]
    fn GetServiceW(
        &self,
        dw_name_space: u32,
        lp_guid: ConstPtr<crate::core::GUID>,
        lp_service_name: PCWSTR,
        dw_properties: u32,
        lp_buffer: MutPtr<::core::ffi::c_void>,
        lpdw_buffer_size: MutPtr<u32>,
        lp_service_async_info: ConstPtr<SERVICE_ASYNC_INFO>,
    ) -> i32 {
        todo!("GetServiceW")
    }
    #[doc = "GetTypeByNameA from MSWSOCK"]
    fn GetTypeByNameA(
        &self,
        lp_service_name: PCSTR,
        lp_service_type: MutPtr<crate::core::GUID>,
    ) -> i32 {
        todo!("GetTypeByNameA")
    }
    #[doc = "GetTypeByNameW from MSWSOCK"]
    fn GetTypeByNameW(
        &self,
        lp_service_name: PCWSTR,
        lp_service_type: MutPtr<crate::core::GUID>,
    ) -> i32 {
        todo!("GetTypeByNameW")
    }
    #[doc = "InetNtopW from WS2_32"]
    fn InetNtopW(
        &self,
        family: i32,
        p_addr: ConstPtr<::core::ffi::c_void>,
        p_string_buf: PWSTR,
        string_buf_size: PtrRepr,
    ) -> PWSTR {
        todo!("InetNtopW")
    }
    #[doc = "InetPtonW from WS2_32"]
    fn InetPtonW(
        &self,
        family: i32,
        psz_addr_string: PCWSTR,
        p_addr_buf: MutPtr<::core::ffi::c_void>,
    ) -> i32 {
        todo!("InetPtonW")
    }
    #[doc = "ProcessSocketNotifications from WS2_32"]
    fn ProcessSocketNotifications(
        &self,
        completion_port: super::super::Foundation::HANDLE,
        registration_count: u32,
        registration_infos: MutPtr<SOCK_NOTIFY_REGISTRATION>,
        timeout_ms: u32,
        completion_count: u32,
        completion_port_entries: MutPtr<super::super::System::IO::OVERLAPPED_ENTRY>,
        received_entry_count: MutPtr<u32>,
    ) -> u32 {
        todo!("ProcessSocketNotifications")
    }
    #[doc = "SetAddrInfoExA from WS2_32"]
    fn SetAddrInfoExA(
        &self,
        p_name: PCSTR,
        p_service_name: PCSTR,
        p_addresses: ConstPtr<SOCKET_ADDRESS>,
        dw_address_count: u32,
        lp_blob: ConstPtr<super::super::System::Com::BLOB>,
        dw_flags: u32,
        dw_name_space: u32,
        lp_nsp_id: ConstPtr<crate::core::GUID>,
        timeout: ConstPtr<timeval>,
        lp_overlapped: ConstPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
        lp_name_handle: MutPtr<super::super::Foundation::HANDLE>,
    ) -> i32 {
        todo!("SetAddrInfoExA")
    }
    #[doc = "SetAddrInfoExW from WS2_32"]
    fn SetAddrInfoExW(
        &self,
        p_name: PCWSTR,
        p_service_name: PCWSTR,
        p_addresses: ConstPtr<SOCKET_ADDRESS>,
        dw_address_count: u32,
        lp_blob: ConstPtr<super::super::System::Com::BLOB>,
        dw_flags: u32,
        dw_name_space: u32,
        lp_nsp_id: ConstPtr<crate::core::GUID>,
        timeout: ConstPtr<timeval>,
        lp_overlapped: ConstPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
        lp_name_handle: MutPtr<super::super::Foundation::HANDLE>,
    ) -> i32 {
        todo!("SetAddrInfoExW")
    }
    #[doc = "SetServiceA from MSWSOCK"]
    fn SetServiceA(
        &self,
        dw_name_space: u32,
        dw_operation: SET_SERVICE_OPERATION,
        dw_flags: u32,
        lp_service_info: ConstPtr<SERVICE_INFOA>,
        lp_service_async_info: ConstPtr<SERVICE_ASYNC_INFO>,
        lpdw_status_flags: MutPtr<u32>,
    ) -> i32 {
        todo!("SetServiceA")
    }
    #[doc = "SetServiceW from MSWSOCK"]
    fn SetServiceW(
        &self,
        dw_name_space: u32,
        dw_operation: SET_SERVICE_OPERATION,
        dw_flags: u32,
        lp_service_info: ConstPtr<SERVICE_INFOW>,
        lp_service_async_info: ConstPtr<SERVICE_ASYNC_INFO>,
        lpdw_status_flags: MutPtr<u32>,
    ) -> i32 {
        todo!("SetServiceW")
    }
    #[doc = "TransmitFile from MSWSOCK"]
    fn TransmitFile(
        &self,
        h_socket: SOCKET,
        h_file: super::super::Foundation::HANDLE,
        n_number_of_bytes_to_write: u32,
        n_number_of_bytes_per_send: u32,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
        lp_transmit_buffers: ConstPtr<TRANSMIT_FILE_BUFFERS>,
        dw_reserved: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("TransmitFile")
    }
    #[doc = "WPUCompleteOverlappedRequest from WS2_32"]
    fn WPUCompleteOverlappedRequest(
        &self,
        s: SOCKET,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
        dw_error: u32,
        cb_transferred: u32,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WPUCompleteOverlappedRequest")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSAAccept from WS2_32"]
    fn WSAAccept(
        &self,
        s: SOCKET,
        addr: MutPtr<SOCKADDR>,
        addrlen: MutPtr<i32>,
        lpfn_condition: LPCONDITIONPROC,
        dw_callback_data: PtrRepr,
    ) -> SOCKET {
        todo!("WSAAccept")
    }
    #[doc = "WSAAddressToStringA from WS2_32"]
    fn WSAAddressToStringA(
        &self,
        lpsa_address: ConstPtr<SOCKADDR>,
        dw_address_length: u32,
        lp_protocol_info: ConstPtr<WSAPROTOCOL_INFOA>,
        lpsz_address_string: PSTR,
        lpdw_address_string_length: MutPtr<u32>,
    ) -> i32 {
        todo!("WSAAddressToStringA")
    }
    #[doc = "WSAAddressToStringW from WS2_32"]
    fn WSAAddressToStringW(
        &self,
        lpsa_address: ConstPtr<SOCKADDR>,
        dw_address_length: u32,
        lp_protocol_info: ConstPtr<WSAPROTOCOL_INFOW>,
        lpsz_address_string: PWSTR,
        lpdw_address_string_length: MutPtr<u32>,
    ) -> i32 {
        todo!("WSAAddressToStringW")
    }
    #[doc = "WSAAdvertiseProvider from WS2_32"]
    fn WSAAdvertiseProvider(
        &self,
        puuid_provider_id: ConstPtr<crate::core::GUID>,
        p_ns_pv_2_routine: ConstPtr<NSPV2_ROUTINE>,
    ) -> i32 {
        todo!("WSAAdvertiseProvider")
    }
    #[doc = "WSAAsyncGetHostByAddr from WS2_32"]
    fn WSAAsyncGetHostByAddr(
        &self,
        h_wnd: super::super::Foundation::HWND,
        w_msg: u32,
        addr: PCSTR,
        len: i32,
        r#type: i32,
        buf: PSTR,
        buflen: i32,
    ) -> super::super::Foundation::HANDLE {
        todo!("WSAAsyncGetHostByAddr")
    }
    #[doc = "WSAAsyncGetHostByName from WS2_32"]
    fn WSAAsyncGetHostByName(
        &self,
        h_wnd: super::super::Foundation::HWND,
        w_msg: u32,
        name: PCSTR,
        buf: PSTR,
        buflen: i32,
    ) -> super::super::Foundation::HANDLE {
        todo!("WSAAsyncGetHostByName")
    }
    #[doc = "WSAAsyncGetProtoByName from WS2_32"]
    fn WSAAsyncGetProtoByName(
        &self,
        h_wnd: super::super::Foundation::HWND,
        w_msg: u32,
        name: PCSTR,
        buf: PSTR,
        buflen: i32,
    ) -> super::super::Foundation::HANDLE {
        todo!("WSAAsyncGetProtoByName")
    }
    #[doc = "WSAAsyncGetProtoByNumber from WS2_32"]
    fn WSAAsyncGetProtoByNumber(
        &self,
        h_wnd: super::super::Foundation::HWND,
        w_msg: u32,
        number: i32,
        buf: PSTR,
        buflen: i32,
    ) -> super::super::Foundation::HANDLE {
        todo!("WSAAsyncGetProtoByNumber")
    }
    #[doc = "WSAAsyncGetServByName from WS2_32"]
    fn WSAAsyncGetServByName(
        &self,
        h_wnd: super::super::Foundation::HWND,
        w_msg: u32,
        name: PCSTR,
        proto: PCSTR,
        buf: PSTR,
        buflen: i32,
    ) -> super::super::Foundation::HANDLE {
        todo!("WSAAsyncGetServByName")
    }
    #[doc = "WSAAsyncGetServByPort from WS2_32"]
    fn WSAAsyncGetServByPort(
        &self,
        h_wnd: super::super::Foundation::HWND,
        w_msg: u32,
        port: i32,
        proto: PCSTR,
        buf: PSTR,
        buflen: i32,
    ) -> super::super::Foundation::HANDLE {
        todo!("WSAAsyncGetServByPort")
    }
    #[doc = "WSAAsyncSelect from WS2_32"]
    fn WSAAsyncSelect(
        &self,
        s: SOCKET,
        h_wnd: super::super::Foundation::HWND,
        w_msg: u32,
        l_event: i32,
    ) -> i32 {
        todo!("WSAAsyncSelect")
    }
    #[doc = "WSACancelAsyncRequest from WS2_32"]
    fn WSACancelAsyncRequest(&self, h_async_task_handle: super::super::Foundation::HANDLE) -> i32 {
        todo!("WSACancelAsyncRequest")
    }
    #[doc = "WSACancelBlockingCall from WS2_32"]
    fn WSACancelBlockingCall(&self) -> i32 {
        todo!("WSACancelBlockingCall")
    }
    #[doc = "WSACleanup from WS2_32"]
    fn WSACleanup(&self) -> i32 {
        todo!("WSACleanup")
    }
    #[doc = "WSACloseEvent from WS2_32"]
    fn WSACloseEvent(
        &self,
        h_event: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("WSACloseEvent")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSAConnect from WS2_32"]
    fn WSAConnect(
        &self,
        s: SOCKET,
        name: ConstPtr<SOCKADDR>,
        namelen: i32,
        lp_caller_data: ConstPtr<WSABUF>,
        lp_callee_data: MutPtr<WSABUF>,
        lp_sqos: ConstPtr<super::super::NetworkManagement::QoS::QOS>,
        lp_gqos: ConstPtr<super::super::NetworkManagement::QoS::QOS>,
    ) -> i32 {
        todo!("WSAConnect")
    }
    #[doc = "WSAConnectByList from WS2_32"]
    fn WSAConnectByList(
        &self,
        s: SOCKET,
        socket_address: ConstPtr<SOCKET_ADDRESS_LIST>,
        local_address_length: MutPtr<u32>,
        local_address: MutPtr<SOCKADDR>,
        remote_address_length: MutPtr<u32>,
        remote_address: MutPtr<SOCKADDR>,
        timeout: ConstPtr<timeval>,
        reserved: MutPtr<super::super::System::IO::OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!("WSAConnectByList")
    }
    #[doc = "WSAConnectByNameA from WS2_32"]
    fn WSAConnectByNameA(
        &self,
        s: SOCKET,
        nodename: PCSTR,
        servicename: PCSTR,
        local_address_length: MutPtr<u32>,
        local_address: MutPtr<SOCKADDR>,
        remote_address_length: MutPtr<u32>,
        remote_address: MutPtr<SOCKADDR>,
        timeout: ConstPtr<timeval>,
        reserved: MutPtr<super::super::System::IO::OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!("WSAConnectByNameA")
    }
    #[doc = "WSAConnectByNameW from WS2_32"]
    fn WSAConnectByNameW(
        &self,
        s: SOCKET,
        nodename: PCWSTR,
        servicename: PCWSTR,
        local_address_length: MutPtr<u32>,
        local_address: MutPtr<SOCKADDR>,
        remote_address_length: MutPtr<u32>,
        remote_address: MutPtr<SOCKADDR>,
        timeout: ConstPtr<timeval>,
        reserved: MutPtr<super::super::System::IO::OVERLAPPED>,
    ) -> super::super::Foundation::BOOL {
        todo!("WSAConnectByNameW")
    }
    #[doc = "WSACreateEvent from WS2_32"]
    fn WSACreateEvent(&self) -> super::super::Foundation::HANDLE {
        todo!("WSACreateEvent")
    }
    #[doc = "WSADuplicateSocketA from WS2_32"]
    fn WSADuplicateSocketA(
        &self,
        s: SOCKET,
        dw_process_id: u32,
        lp_protocol_info: MutPtr<WSAPROTOCOL_INFOA>,
    ) -> i32 {
        todo!("WSADuplicateSocketA")
    }
    #[doc = "WSADuplicateSocketW from WS2_32"]
    fn WSADuplicateSocketW(
        &self,
        s: SOCKET,
        dw_process_id: u32,
        lp_protocol_info: MutPtr<WSAPROTOCOL_INFOW>,
    ) -> i32 {
        todo!("WSADuplicateSocketW")
    }
    #[doc = "WSAEnumNameSpaceProvidersA from WS2_32"]
    fn WSAEnumNameSpaceProvidersA(
        &self,
        lpdw_buffer_length: MutPtr<u32>,
        lpnsp_buffer: MutPtr<WSANAMESPACE_INFOA>,
    ) -> i32 {
        todo!("WSAEnumNameSpaceProvidersA")
    }
    #[doc = "WSAEnumNameSpaceProvidersExA from WS2_32"]
    fn WSAEnumNameSpaceProvidersExA(
        &self,
        lpdw_buffer_length: MutPtr<u32>,
        lpnsp_buffer: MutPtr<WSANAMESPACE_INFOEXA>,
    ) -> i32 {
        todo!("WSAEnumNameSpaceProvidersExA")
    }
    #[doc = "WSAEnumNameSpaceProvidersExW from WS2_32"]
    fn WSAEnumNameSpaceProvidersExW(
        &self,
        lpdw_buffer_length: MutPtr<u32>,
        lpnsp_buffer: MutPtr<WSANAMESPACE_INFOEXW>,
    ) -> i32 {
        todo!("WSAEnumNameSpaceProvidersExW")
    }
    #[doc = "WSAEnumNameSpaceProvidersW from WS2_32"]
    fn WSAEnumNameSpaceProvidersW(
        &self,
        lpdw_buffer_length: MutPtr<u32>,
        lpnsp_buffer: MutPtr<WSANAMESPACE_INFOW>,
    ) -> i32 {
        todo!("WSAEnumNameSpaceProvidersW")
    }
    #[doc = "WSAEnumNetworkEvents from WS2_32"]
    fn WSAEnumNetworkEvents(
        &self,
        s: SOCKET,
        h_event_object: super::super::Foundation::HANDLE,
        lp_network_events: MutPtr<WSANETWORKEVENTS>,
    ) -> i32 {
        todo!("WSAEnumNetworkEvents")
    }
    #[doc = "WSAEnumProtocolsA from WS2_32"]
    fn WSAEnumProtocolsA(
        &self,
        lpi_protocols: ConstPtr<i32>,
        lp_protocol_buffer: MutPtr<WSAPROTOCOL_INFOA>,
        lpdw_buffer_length: MutPtr<u32>,
    ) -> i32 {
        todo!("WSAEnumProtocolsA")
    }
    #[doc = "WSAEnumProtocolsW from WS2_32"]
    fn WSAEnumProtocolsW(
        &self,
        lpi_protocols: ConstPtr<i32>,
        lp_protocol_buffer: MutPtr<WSAPROTOCOL_INFOW>,
        lpdw_buffer_length: MutPtr<u32>,
    ) -> i32 {
        todo!("WSAEnumProtocolsW")
    }
    #[doc = "WSAEventSelect from WS2_32"]
    fn WSAEventSelect(
        &self,
        s: SOCKET,
        h_event_object: super::super::Foundation::HANDLE,
        l_network_events: i32,
    ) -> i32 {
        todo!("WSAEventSelect")
    }
    #[doc = "WSAGetLastError from WS2_32"]
    fn WSAGetLastError(&self) -> WSA_ERROR {
        todo!("WSAGetLastError")
    }
    #[doc = "WSAGetOverlappedResult from WS2_32"]
    fn WSAGetOverlappedResult(
        &self,
        s: SOCKET,
        lp_overlapped: ConstPtr<super::super::System::IO::OVERLAPPED>,
        lpcb_transfer: MutPtr<u32>,
        f_wait: super::super::Foundation::BOOL,
        lpdw_flags: MutPtr<u32>,
    ) -> super::super::Foundation::BOOL {
        todo!("WSAGetOverlappedResult")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSAGetQOSByName from WS2_32"]
    fn WSAGetQOSByName(
        &self,
        s: SOCKET,
        lp_qos_name: ConstPtr<WSABUF>,
        lp_qos: MutPtr<super::super::NetworkManagement::QoS::QOS>,
    ) -> super::super::Foundation::BOOL {
        todo!("WSAGetQOSByName")
    }
    #[doc = "WSAGetServiceClassInfoA from WS2_32"]
    fn WSAGetServiceClassInfoA(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        lp_service_class_id: ConstPtr<crate::core::GUID>,
        lpdw_buf_size: MutPtr<u32>,
        lp_service_class_info: MutPtr<WSASERVICECLASSINFOA>,
    ) -> i32 {
        todo!("WSAGetServiceClassInfoA")
    }
    #[doc = "WSAGetServiceClassInfoW from WS2_32"]
    fn WSAGetServiceClassInfoW(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        lp_service_class_id: ConstPtr<crate::core::GUID>,
        lpdw_buf_size: MutPtr<u32>,
        lp_service_class_info: MutPtr<WSASERVICECLASSINFOW>,
    ) -> i32 {
        todo!("WSAGetServiceClassInfoW")
    }
    #[doc = "WSAGetServiceClassNameByClassIdA from WS2_32"]
    fn WSAGetServiceClassNameByClassIdA(
        &self,
        lp_service_class_id: ConstPtr<crate::core::GUID>,
        lpsz_service_class_name: PSTR,
        lpdw_buffer_length: MutPtr<u32>,
    ) -> i32 {
        todo!("WSAGetServiceClassNameByClassIdA")
    }
    #[doc = "WSAGetServiceClassNameByClassIdW from WS2_32"]
    fn WSAGetServiceClassNameByClassIdW(
        &self,
        lp_service_class_id: ConstPtr<crate::core::GUID>,
        lpsz_service_class_name: PWSTR,
        lpdw_buffer_length: MutPtr<u32>,
    ) -> i32 {
        todo!("WSAGetServiceClassNameByClassIdW")
    }
    #[doc = "WSAHtonl from WS2_32"]
    fn WSAHtonl(&self, s: SOCKET, hostlong: u32, lpnetlong: MutPtr<u32>) -> i32 {
        todo!("WSAHtonl")
    }
    #[doc = "WSAHtons from WS2_32"]
    fn WSAHtons(&self, s: SOCKET, hostshort: u16, lpnetshort: MutPtr<u16>) -> i32 {
        todo!("WSAHtons")
    }
    #[doc = "WSAInstallServiceClassA from WS2_32"]
    fn WSAInstallServiceClassA(
        &self,
        lp_service_class_info: ConstPtr<WSASERVICECLASSINFOA>,
    ) -> i32 {
        todo!("WSAInstallServiceClassA")
    }
    #[doc = "WSAInstallServiceClassW from WS2_32"]
    fn WSAInstallServiceClassW(
        &self,
        lp_service_class_info: ConstPtr<WSASERVICECLASSINFOW>,
    ) -> i32 {
        todo!("WSAInstallServiceClassW")
    }
    #[doc = "WSAIoctl from WS2_32"]
    fn WSAIoctl(
        &self,
        s: SOCKET,
        dw_io_control_code: u32,
        lpv_in_buffer: ConstPtr<::core::ffi::c_void>,
        cb_in_buffer: u32,
        lpv_out_buffer: MutPtr<::core::ffi::c_void>,
        cb_out_buffer: u32,
        lpcb_bytes_returned: MutPtr<u32>,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> i32 {
        todo!("WSAIoctl")
    }
    #[doc = "WSAIsBlocking from WS2_32"]
    fn WSAIsBlocking(&self) -> super::super::Foundation::BOOL {
        todo!("WSAIsBlocking")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.NetworkManagement.QoS'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSAJoinLeaf from WS2_32"]
    fn WSAJoinLeaf(
        &self,
        s: SOCKET,
        name: ConstPtr<SOCKADDR>,
        namelen: i32,
        lp_caller_data: ConstPtr<WSABUF>,
        lp_callee_data: MutPtr<WSABUF>,
        lp_sqos: ConstPtr<super::super::NetworkManagement::QoS::QOS>,
        lp_gqos: ConstPtr<super::super::NetworkManagement::QoS::QOS>,
        dw_flags: u32,
    ) -> SOCKET {
        todo!("WSAJoinLeaf")
    }
    #[doc = "WSALookupServiceBeginA from WS2_32"]
    fn WSALookupServiceBeginA(
        &self,
        lpqs_restrictions: ConstPtr<WSAQUERYSETA>,
        dw_control_flags: u32,
        lph_lookup: MutPtr<super::super::Foundation::HANDLE>,
    ) -> i32 {
        todo!("WSALookupServiceBeginA")
    }
    #[doc = "WSALookupServiceBeginW from WS2_32"]
    fn WSALookupServiceBeginW(
        &self,
        lpqs_restrictions: ConstPtr<WSAQUERYSETW>,
        dw_control_flags: u32,
        lph_lookup: MutPtr<super::super::Foundation::HANDLE>,
    ) -> i32 {
        todo!("WSALookupServiceBeginW")
    }
    #[doc = "WSALookupServiceEnd from WS2_32"]
    fn WSALookupServiceEnd(&self, h_lookup: super::super::Foundation::HANDLE) -> i32 {
        todo!("WSALookupServiceEnd")
    }
    #[doc = "WSALookupServiceNextA from WS2_32"]
    fn WSALookupServiceNextA(
        &self,
        h_lookup: super::super::Foundation::HANDLE,
        dw_control_flags: u32,
        lpdw_buffer_length: MutPtr<u32>,
        lpqs_results: MutPtr<WSAQUERYSETA>,
    ) -> i32 {
        todo!("WSALookupServiceNextA")
    }
    #[doc = "WSALookupServiceNextW from WS2_32"]
    fn WSALookupServiceNextW(
        &self,
        h_lookup: super::super::Foundation::HANDLE,
        dw_control_flags: u32,
        lpdw_buffer_length: MutPtr<u32>,
        lpqs_results: MutPtr<WSAQUERYSETW>,
    ) -> i32 {
        todo!("WSALookupServiceNextW")
    }
    #[doc = "WSANSPIoctl from WS2_32"]
    fn WSANSPIoctl(
        &self,
        h_lookup: super::super::Foundation::HANDLE,
        dw_control_code: u32,
        lpv_in_buffer: ConstPtr<::core::ffi::c_void>,
        cb_in_buffer: u32,
        lpv_out_buffer: MutPtr<::core::ffi::c_void>,
        cb_out_buffer: u32,
        lpcb_bytes_returned: MutPtr<u32>,
        lp_completion: ConstPtr<WSACOMPLETION>,
    ) -> i32 {
        todo!("WSANSPIoctl")
    }
    #[doc = "WSANtohl from WS2_32"]
    fn WSANtohl(&self, s: SOCKET, netlong: u32, lphostlong: MutPtr<u32>) -> i32 {
        todo!("WSANtohl")
    }
    #[doc = "WSANtohs from WS2_32"]
    fn WSANtohs(&self, s: SOCKET, netshort: u16, lphostshort: MutPtr<u16>) -> i32 {
        todo!("WSANtohs")
    }
    #[doc = "WSAPoll from WS2_32"]
    fn WSAPoll(&self, fd_array: MutPtr<WSAPOLLFD>, fds: u32, timeout: i32) -> i32 {
        todo!("WSAPoll")
    }
    #[doc = "WSAProviderCompleteAsyncCall from WS2_32"]
    fn WSAProviderCompleteAsyncCall(
        &self,
        h_async_call: super::super::Foundation::HANDLE,
        i_ret_code: i32,
    ) -> i32 {
        todo!("WSAProviderCompleteAsyncCall")
    }
    #[doc = "WSAProviderConfigChange from WS2_32"]
    fn WSAProviderConfigChange(
        &self,
        lp_notification_handle: MutPtr<super::super::Foundation::HANDLE>,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> i32 {
        todo!("WSAProviderConfigChange")
    }
    #[doc = "WSARecv from WS2_32"]
    fn WSARecv(
        &self,
        s: SOCKET,
        lp_buffers: ConstPtr<WSABUF>,
        dw_buffer_count: u32,
        lp_number_of_bytes_recvd: MutPtr<u32>,
        lp_flags: MutPtr<u32>,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> i32 {
        todo!("WSARecv")
    }
    #[doc = "WSARecvDisconnect from WS2_32"]
    fn WSARecvDisconnect(&self, s: SOCKET, lp_inbound_disconnect_data: ConstPtr<WSABUF>) -> i32 {
        todo!("WSARecvDisconnect")
    }
    #[doc = "WSARecvEx from MSWSOCK"]
    fn WSARecvEx(&self, s: SOCKET, buf: PSTR, len: i32, flags: MutPtr<i32>) -> i32 {
        todo!("WSARecvEx")
    }
    #[doc = "WSARecvFrom from WS2_32"]
    fn WSARecvFrom(
        &self,
        s: SOCKET,
        lp_buffers: ConstPtr<WSABUF>,
        dw_buffer_count: u32,
        lp_number_of_bytes_recvd: MutPtr<u32>,
        lp_flags: MutPtr<u32>,
        lp_from: MutPtr<SOCKADDR>,
        lp_fromlen: MutPtr<i32>,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> i32 {
        todo!("WSARecvFrom")
    }
    #[doc = "WSARemoveServiceClass from WS2_32"]
    fn WSARemoveServiceClass(&self, lp_service_class_id: ConstPtr<crate::core::GUID>) -> i32 {
        todo!("WSARemoveServiceClass")
    }
    #[doc = "WSAResetEvent from WS2_32"]
    fn WSAResetEvent(
        &self,
        h_event: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("WSAResetEvent")
    }
    #[doc = "WSASend from WS2_32"]
    fn WSASend(
        &self,
        s: SOCKET,
        lp_buffers: ConstPtr<WSABUF>,
        dw_buffer_count: u32,
        lp_number_of_bytes_sent: MutPtr<u32>,
        dw_flags: u32,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> i32 {
        todo!("WSASend")
    }
    #[doc = "WSASendDisconnect from WS2_32"]
    fn WSASendDisconnect(&self, s: SOCKET, lp_outbound_disconnect_data: ConstPtr<WSABUF>) -> i32 {
        todo!("WSASendDisconnect")
    }
    #[doc = "WSASendMsg from WS2_32"]
    fn WSASendMsg(
        &self,
        handle: SOCKET,
        lp_msg: ConstPtr<WSAMSG>,
        dw_flags: u32,
        lp_number_of_bytes_sent: MutPtr<u32>,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> i32 {
        todo!("WSASendMsg")
    }
    #[doc = "WSASendTo from WS2_32"]
    fn WSASendTo(
        &self,
        s: SOCKET,
        lp_buffers: ConstPtr<WSABUF>,
        dw_buffer_count: u32,
        lp_number_of_bytes_sent: MutPtr<u32>,
        dw_flags: u32,
        lp_to: ConstPtr<SOCKADDR>,
        i_tolen: i32,
        lp_overlapped: MutPtr<super::super::System::IO::OVERLAPPED>,
        lp_completion_routine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> i32 {
        todo!("WSASendTo")
    }
    #[doc = "WSASetBlockingHook from WS2_32"]
    fn WSASetBlockingHook(
        &self,
        lp_block_func: super::super::Foundation::FARPROC,
    ) -> super::super::Foundation::FARPROC {
        todo!("WSASetBlockingHook")
    }
    #[doc = "WSASetEvent from WS2_32"]
    fn WSASetEvent(
        &self,
        h_event: super::super::Foundation::HANDLE,
    ) -> super::super::Foundation::BOOL {
        todo!("WSASetEvent")
    }
    #[doc = "WSASetLastError from WS2_32"]
    fn WSASetLastError(&self, i_error: i32) {
        todo!("WSASetLastError")
    }
    #[doc = "WSASetServiceA from WS2_32"]
    fn WSASetServiceA(
        &self,
        lpqs_reg_info: ConstPtr<WSAQUERYSETA>,
        essoperation: WSAESETSERVICEOP,
        dw_control_flags: u32,
    ) -> i32 {
        todo!("WSASetServiceA")
    }
    #[doc = "WSASetServiceW from WS2_32"]
    fn WSASetServiceW(
        &self,
        lpqs_reg_info: ConstPtr<WSAQUERYSETW>,
        essoperation: WSAESETSERVICEOP,
        dw_control_flags: u32,
    ) -> i32 {
        todo!("WSASetServiceW")
    }
    #[doc = "WSASocketA from WS2_32"]
    fn WSASocketA(
        &self,
        af: i32,
        r#type: i32,
        protocol: i32,
        lp_protocol_info: ConstPtr<WSAPROTOCOL_INFOA>,
        g: u32,
        dw_flags: u32,
    ) -> SOCKET {
        todo!("WSASocketA")
    }
    #[doc = "WSASocketW from WS2_32"]
    fn WSASocketW(
        &self,
        af: i32,
        r#type: i32,
        protocol: i32,
        lp_protocol_info: ConstPtr<WSAPROTOCOL_INFOW>,
        g: u32,
        dw_flags: u32,
    ) -> SOCKET {
        todo!("WSASocketW")
    }
    #[doc = "WSAStartup from WS2_32"]
    fn WSAStartup(&self, w_version_requested: u16, lp_wsa_data: MutPtr<WSAData>) -> i32 {
        todo!("WSAStartup")
    }
    #[doc = "WSAStringToAddressA from WS2_32"]
    fn WSAStringToAddressA(
        &self,
        address_string: PCSTR,
        address_family: i32,
        lp_protocol_info: ConstPtr<WSAPROTOCOL_INFOA>,
        lp_address: MutPtr<SOCKADDR>,
        lp_address_length: MutPtr<i32>,
    ) -> i32 {
        todo!("WSAStringToAddressA")
    }
    #[doc = "WSAStringToAddressW from WS2_32"]
    fn WSAStringToAddressW(
        &self,
        address_string: PCWSTR,
        address_family: i32,
        lp_protocol_info: ConstPtr<WSAPROTOCOL_INFOW>,
        lp_address: MutPtr<SOCKADDR>,
        lp_address_length: MutPtr<i32>,
    ) -> i32 {
        todo!("WSAStringToAddressW")
    }
    #[doc = "WSAUnadvertiseProvider from WS2_32"]
    fn WSAUnadvertiseProvider(&self, puuid_provider_id: ConstPtr<crate::core::GUID>) -> i32 {
        todo!("WSAUnadvertiseProvider")
    }
    #[doc = "WSAUnhookBlockingHook from WS2_32"]
    fn WSAUnhookBlockingHook(&self) -> i32 {
        todo!("WSAUnhookBlockingHook")
    }
    #[doc = "WSAWaitForMultipleEvents from WS2_32"]
    fn WSAWaitForMultipleEvents(
        &self,
        c_events: u32,
        lph_events: ConstPtr<super::super::Foundation::HANDLE>,
        f_wait_all: super::super::Foundation::BOOL,
        dw_timeout: u32,
        f_alertable: super::super::Foundation::BOOL,
    ) -> u32 {
        todo!("WSAWaitForMultipleEvents")
    }
    #[doc = "WSCDeinstallProvider from WS2_32"]
    fn WSCDeinstallProvider(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCDeinstallProvider")
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCDeinstallProvider32 from WS2_32"]
    fn WSCDeinstallProvider32(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCDeinstallProvider32")
    }
    #[doc = "WSCEnableNSProvider from WS2_32"]
    fn WSCEnableNSProvider(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        f_enable: super::super::Foundation::BOOL,
    ) -> i32 {
        todo!("WSCEnableNSProvider")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCEnableNSProvider32 from WS2_32"]
    fn WSCEnableNSProvider32(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        f_enable: super::super::Foundation::BOOL,
    ) -> i32 {
        todo!("WSCEnableNSProvider32")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCEnumNameSpaceProviders32 from WS2_32"]
    fn WSCEnumNameSpaceProviders32(
        &self,
        lpdw_buffer_length: MutPtr<u32>,
        lpnsp_buffer: MutPtr<WSANAMESPACE_INFOW>,
    ) -> i32 {
        todo!("WSCEnumNameSpaceProviders32")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.Foundation', 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCEnumNameSpaceProvidersEx32 from WS2_32"]
    fn WSCEnumNameSpaceProvidersEx32(
        &self,
        lpdw_buffer_length: MutPtr<u32>,
        lpnsp_buffer: MutPtr<WSANAMESPACE_INFOEXW>,
    ) -> i32 {
        todo!("WSCEnumNameSpaceProvidersEx32")
    }
    #[doc = "WSCEnumProtocols from WS2_32"]
    fn WSCEnumProtocols(
        &self,
        lpi_protocols: ConstPtr<i32>,
        lp_protocol_buffer: MutPtr<WSAPROTOCOL_INFOW>,
        lpdw_buffer_length: MutPtr<u32>,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCEnumProtocols")
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCEnumProtocols32 from WS2_32"]
    fn WSCEnumProtocols32(
        &self,
        lpi_protocols: ConstPtr<i32>,
        lp_protocol_buffer: MutPtr<WSAPROTOCOL_INFOW>,
        lpdw_buffer_length: MutPtr<u32>,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCEnumProtocols32")
    }
    #[doc = "WSCGetApplicationCategory from WS2_32"]
    fn WSCGetApplicationCategory(
        &self,
        path: PCWSTR,
        path_length: u32,
        extra: PCWSTR,
        extra_length: u32,
        p_permitted_lsp_categories: MutPtr<u32>,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCGetApplicationCategory")
    }
    #[doc = "WSCGetProviderInfo from WS2_32"]
    fn WSCGetProviderInfo(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        info_type: WSC_PROVIDER_INFO_TYPE,
        info: MutPtr<u8>,
        info_size: MutPtr<PtrRepr>,
        flags: u32,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCGetProviderInfo")
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCGetProviderInfo32 from WS2_32"]
    fn WSCGetProviderInfo32(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        info_type: WSC_PROVIDER_INFO_TYPE,
        info: MutPtr<u8>,
        info_size: MutPtr<PtrRepr>,
        flags: u32,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCGetProviderInfo32")
    }
    #[doc = "WSCGetProviderPath from WS2_32"]
    fn WSCGetProviderPath(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        lpsz_provider_dll_path: PWSTR,
        lp_provider_dll_path_len: MutPtr<i32>,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCGetProviderPath")
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCGetProviderPath32 from WS2_32"]
    fn WSCGetProviderPath32(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        lpsz_provider_dll_path: PWSTR,
        lp_provider_dll_path_len: MutPtr<i32>,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCGetProviderPath32")
    }
    #[doc = "WSCInstallNameSpace from WS2_32"]
    fn WSCInstallNameSpace(
        &self,
        lpsz_identifier: PCWSTR,
        lpsz_path_name: PCWSTR,
        dw_name_space: u32,
        dw_version: u32,
        lp_provider_id: ConstPtr<crate::core::GUID>,
    ) -> i32 {
        todo!("WSCInstallNameSpace")
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCInstallNameSpace32 from WS2_32"]
    fn WSCInstallNameSpace32(
        &self,
        lpsz_identifier: PCWSTR,
        lpsz_path_name: PCWSTR,
        dw_name_space: u32,
        dw_version: u32,
        lp_provider_id: ConstPtr<crate::core::GUID>,
    ) -> i32 {
        todo!("WSCInstallNameSpace32")
    }
    #[doc = "WSCInstallNameSpaceEx from WS2_32"]
    fn WSCInstallNameSpaceEx(
        &self,
        lpsz_identifier: PCWSTR,
        lpsz_path_name: PCWSTR,
        dw_name_space: u32,
        dw_version: u32,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        lp_provider_specific: ConstPtr<super::super::System::Com::BLOB>,
    ) -> i32 {
        todo!("WSCInstallNameSpaceEx")
    }
    #[doc = "*Required namespaces: 'Windows.Win32.System.Com'*"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCInstallNameSpaceEx32 from WS2_32"]
    fn WSCInstallNameSpaceEx32(
        &self,
        lpsz_identifier: PCWSTR,
        lpsz_path_name: PCWSTR,
        dw_name_space: u32,
        dw_version: u32,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        lp_provider_specific: ConstPtr<super::super::System::Com::BLOB>,
    ) -> i32 {
        todo!("WSCInstallNameSpaceEx32")
    }
    #[doc = "WSCInstallProvider from WS2_32"]
    fn WSCInstallProvider(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        lpsz_provider_dll_path: PCWSTR,
        lp_protocol_info_list: ConstPtr<WSAPROTOCOL_INFOW>,
        dw_number_of_entries: u32,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCInstallProvider")
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCInstallProvider64_32 from WS2_32"]
    fn WSCInstallProvider64_32(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        lpsz_provider_dll_path: PCWSTR,
        lp_protocol_info_list: ConstPtr<WSAPROTOCOL_INFOW>,
        dw_number_of_entries: u32,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCInstallProvider64_32")
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCInstallProviderAndChains64_32 from WS2_32"]
    fn WSCInstallProviderAndChains64_32(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        lpsz_provider_dll_path: PCWSTR,
        lpsz_provider_dll_path_32: PCWSTR,
        lpsz_lsp_name: PCWSTR,
        dw_service_flags: u32,
        lp_protocol_info_list: MutPtr<WSAPROTOCOL_INFOW>,
        dw_number_of_entries: u32,
        lpdw_catalog_entry_id: MutPtr<u32>,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCInstallProviderAndChains64_32")
    }
    #[doc = "WSCSetApplicationCategory from WS2_32"]
    fn WSCSetApplicationCategory(
        &self,
        path: PCWSTR,
        path_length: u32,
        extra: PCWSTR,
        extra_length: u32,
        permitted_lsp_categories: u32,
        p_prev_perm_lsp_cat: MutPtr<u32>,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCSetApplicationCategory")
    }
    #[doc = "WSCSetProviderInfo from WS2_32"]
    fn WSCSetProviderInfo(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        info_type: WSC_PROVIDER_INFO_TYPE,
        info: ConstPtr<u8>,
        info_size: PtrRepr,
        flags: u32,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCSetProviderInfo")
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCSetProviderInfo32 from WS2_32"]
    fn WSCSetProviderInfo32(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        info_type: WSC_PROVIDER_INFO_TYPE,
        info: ConstPtr<u8>,
        info_size: PtrRepr,
        flags: u32,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCSetProviderInfo32")
    }
    #[doc = "WSCUnInstallNameSpace from WS2_32"]
    fn WSCUnInstallNameSpace(&self, lp_provider_id: ConstPtr<crate::core::GUID>) -> i32 {
        todo!("WSCUnInstallNameSpace")
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCUnInstallNameSpace32 from WS2_32"]
    fn WSCUnInstallNameSpace32(&self, lp_provider_id: ConstPtr<crate::core::GUID>) -> i32 {
        todo!("WSCUnInstallNameSpace32")
    }
    #[doc = "WSCUpdateProvider from WS2_32"]
    fn WSCUpdateProvider(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        lpsz_provider_dll_path: PCWSTR,
        lp_protocol_info_list: ConstPtr<WSAPROTOCOL_INFOW>,
        dw_number_of_entries: u32,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCUpdateProvider")
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCUpdateProvider32 from WS2_32"]
    fn WSCUpdateProvider32(
        &self,
        lp_provider_id: ConstPtr<crate::core::GUID>,
        lpsz_provider_dll_path: PCWSTR,
        lp_protocol_info_list: ConstPtr<WSAPROTOCOL_INFOW>,
        dw_number_of_entries: u32,
        lp_errno: MutPtr<i32>,
    ) -> i32 {
        todo!("WSCUpdateProvider32")
    }
    #[doc = "WSCWriteNameSpaceOrder from WS2_32"]
    fn WSCWriteNameSpaceOrder(
        &self,
        lp_provider_id: MutPtr<crate::core::GUID>,
        dw_number_of_entries: u32,
    ) -> i32 {
        todo!("WSCWriteNameSpaceOrder")
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCWriteNameSpaceOrder32 from WS2_32"]
    fn WSCWriteNameSpaceOrder32(
        &self,
        lp_provider_id: MutPtr<crate::core::GUID>,
        dw_number_of_entries: u32,
    ) -> i32 {
        todo!("WSCWriteNameSpaceOrder32")
    }
    #[doc = "WSCWriteProviderOrder from WS2_32"]
    fn WSCWriteProviderOrder(
        &self,
        lpwd_catalog_entry_id: MutPtr<u32>,
        dw_number_of_entries: u32,
    ) -> i32 {
        todo!("WSCWriteProviderOrder")
    }
    #[doc = "*Required namespaces: *"]
    #[cfg(dummy_option_that_does_not_exist)]
    #[doc = "WSCWriteProviderOrder32 from WS2_32"]
    fn WSCWriteProviderOrder32(
        &self,
        lpwd_catalog_entry_id: MutPtr<u32>,
        dw_number_of_entries: u32,
    ) -> i32 {
        todo!("WSCWriteProviderOrder32")
    }
    #[doc = "__WSAFDIsSet from WS2_32"]
    fn __WSAFDIsSet(&self, fd: SOCKET, param_1: MutPtr<fd_set>) -> i32 {
        todo!("__WSAFDIsSet")
    }
    #[doc = "accept from WS2_32"]
    fn accept(&self, s: SOCKET, addr: MutPtr<SOCKADDR>, addrlen: MutPtr<i32>) -> SOCKET {
        todo!("accept")
    }
    #[doc = "bind from WS2_32"]
    fn bind(&self, s: SOCKET, name: ConstPtr<SOCKADDR>, namelen: i32) -> i32 {
        todo!("bind")
    }
    #[doc = "closesocket from WS2_32"]
    fn closesocket(&self, s: SOCKET) -> i32 {
        todo!("closesocket")
    }
    #[doc = "connect from WS2_32"]
    fn connect(&self, s: SOCKET, name: ConstPtr<SOCKADDR>, namelen: i32) -> i32 {
        todo!("connect")
    }
    #[doc = "freeaddrinfo from WS2_32"]
    fn freeaddrinfo(&self, p_addr_info: ConstPtr<ADDRINFOA>) {
        todo!("freeaddrinfo")
    }
    #[doc = "getaddrinfo from WS2_32"]
    fn getaddrinfo(
        &self,
        p_node_name: PCSTR,
        p_service_name: PCSTR,
        p_hints: ConstPtr<ADDRINFOA>,
        pp_result: MutPtr<ConstPtr<ADDRINFOA>>,
    ) -> i32 {
        todo!("getaddrinfo")
    }
    #[doc = "gethostbyaddr from WS2_32"]
    fn gethostbyaddr(&self, addr: PCSTR, len: i32, r#type: i32) -> MutPtr<hostent> {
        todo!("gethostbyaddr")
    }
    #[doc = "gethostbyname from WS2_32"]
    fn gethostbyname(&self, name: PCSTR) -> MutPtr<hostent> {
        todo!("gethostbyname")
    }
    #[doc = "gethostname from WS2_32"]
    fn gethostname(&self, name: PSTR, namelen: i32) -> i32 {
        todo!("gethostname")
    }
    #[doc = "getnameinfo from WS2_32"]
    fn getnameinfo(
        &self,
        p_sockaddr: ConstPtr<SOCKADDR>,
        sockaddr_length: i32,
        p_node_buffer: PSTR,
        node_buffer_size: u32,
        p_service_buffer: PSTR,
        service_buffer_size: u32,
        flags: i32,
    ) -> i32 {
        todo!("getnameinfo")
    }
    #[doc = "getpeername from WS2_32"]
    fn getpeername(&self, s: SOCKET, name: MutPtr<SOCKADDR>, namelen: MutPtr<i32>) -> i32 {
        todo!("getpeername")
    }
    #[doc = "getprotobyname from WS2_32"]
    fn getprotobyname(&self, name: PCSTR) -> MutPtr<protoent> {
        todo!("getprotobyname")
    }
    #[doc = "getprotobynumber from WS2_32"]
    fn getprotobynumber(&self, number: i32) -> MutPtr<protoent> {
        todo!("getprotobynumber")
    }
    #[doc = "getservbyname from WS2_32"]
    fn getservbyname(&self, name: PCSTR, proto: PCSTR) -> MutPtr<servent> {
        todo!("getservbyname")
    }
    #[doc = "getservbyport from WS2_32"]
    fn getservbyport(&self, port: i32, proto: PCSTR) -> MutPtr<servent> {
        todo!("getservbyport")
    }
    #[doc = "getsockname from WS2_32"]
    fn getsockname(&self, s: SOCKET, name: MutPtr<SOCKADDR>, namelen: MutPtr<i32>) -> i32 {
        todo!("getsockname")
    }
    #[doc = "getsockopt from WS2_32"]
    fn getsockopt(
        &self,
        s: SOCKET,
        level: i32,
        optname: i32,
        optval: PSTR,
        optlen: MutPtr<i32>,
    ) -> i32 {
        todo!("getsockopt")
    }
    #[doc = "htonl from WS2_32"]
    fn htonl(&self, hostlong: u32) -> u32 {
        todo!("htonl")
    }
    #[doc = "htons from WS2_32"]
    fn htons(&self, hostshort: u16) -> u16 {
        todo!("htons")
    }
    #[doc = "inet_addr from WS2_32"]
    fn inet_addr(&self, cp: PCSTR) -> u32 {
        todo!("inet_addr")
    }
    #[doc = "inet_ntoa from WS2_32"]
    fn inet_ntoa(&self, r#in: IN_ADDR) -> PSTR {
        todo!("inet_ntoa")
    }
    #[doc = "inet_ntop from WS2_32"]
    fn inet_ntop(
        &self,
        family: i32,
        p_addr: ConstPtr<::core::ffi::c_void>,
        p_string_buf: PSTR,
        string_buf_size: PtrRepr,
    ) -> PSTR {
        todo!("inet_ntop")
    }
    #[doc = "inet_pton from WS2_32"]
    fn inet_pton(
        &self,
        family: i32,
        psz_addr_string: PCSTR,
        p_addr_buf: MutPtr<::core::ffi::c_void>,
    ) -> i32 {
        todo!("inet_pton")
    }
    #[doc = "ioctlsocket from WS2_32"]
    fn ioctlsocket(&self, s: SOCKET, cmd: i32, argp: MutPtr<u32>) -> i32 {
        todo!("ioctlsocket")
    }
    #[doc = "listen from WS2_32"]
    fn listen(&self, s: SOCKET, backlog: i32) -> i32 {
        todo!("listen")
    }
    #[doc = "ntohl from WS2_32"]
    fn ntohl(&self, netlong: u32) -> u32 {
        todo!("ntohl")
    }
    #[doc = "ntohs from WS2_32"]
    fn ntohs(&self, netshort: u16) -> u16 {
        todo!("ntohs")
    }
    #[doc = "recv from WS2_32"]
    fn recv(&self, s: SOCKET, buf: PSTR, len: i32, flags: i32) -> i32 {
        todo!("recv")
    }
    #[doc = "recvfrom from WS2_32"]
    fn recvfrom(
        &self,
        s: SOCKET,
        buf: PSTR,
        len: i32,
        flags: i32,
        from: MutPtr<SOCKADDR>,
        fromlen: MutPtr<i32>,
    ) -> i32 {
        todo!("recvfrom")
    }
    #[doc = "select from WS2_32"]
    fn select(
        &self,
        nfds: i32,
        readfds: MutPtr<fd_set>,
        writefds: MutPtr<fd_set>,
        exceptfds: MutPtr<fd_set>,
        timeout: ConstPtr<timeval>,
    ) -> i32 {
        todo!("select")
    }
    #[doc = "send from WS2_32"]
    fn send(&self, s: SOCKET, buf: PCSTR, len: i32, flags: SEND_FLAGS) -> i32 {
        todo!("send")
    }
    #[doc = "sendto from WS2_32"]
    fn sendto(
        &self,
        s: SOCKET,
        buf: PCSTR,
        len: i32,
        flags: i32,
        to: ConstPtr<SOCKADDR>,
        tolen: i32,
    ) -> i32 {
        todo!("sendto")
    }
    #[doc = "setsockopt from WS2_32"]
    fn setsockopt(&self, s: SOCKET, level: i32, optname: i32, optval: PCSTR, optlen: i32) -> i32 {
        todo!("setsockopt")
    }
    #[doc = "shutdown from WS2_32"]
    fn shutdown(&self, s: SOCKET, how: i32) -> i32 {
        todo!("shutdown")
    }
    #[doc = "socket from WS2_32"]
    fn socket(&self, af: i32, r#type: i32, protocol: i32) -> SOCKET {
        todo!("socket")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
