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
pub struct ACCESS_ALLOWED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_ALLOWED_ACE {}
impl ::core::clone::Clone for ACCESS_ALLOWED_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_ALLOWED_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_ALLOWED_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACCESS_ALLOWED_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_ALLOWED_ACE {}
impl FromIntoMemory for ACCESS_ALLOWED_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct ACCESS_ALLOWED_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_ALLOWED_CALLBACK_ACE {}
impl ::core::clone::Clone for ACCESS_ALLOWED_CALLBACK_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_ALLOWED_CALLBACK_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_ALLOWED_CALLBACK_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACCESS_ALLOWED_CALLBACK_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_ALLOWED_CALLBACK_ACE {}
impl FromIntoMemory for ACCESS_ALLOWED_CALLBACK_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: crate::core::GUID,
    pub InheritedObjectType: crate::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {}
impl ::core::clone::Clone for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_ALLOWED_CALLBACK_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {}
impl FromIntoMemory for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Flags =
            <SYSTEM_AUDIT_OBJECT_ACE_FLAGS as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ObjectType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_InheritedObjectType =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[28..28 + 16]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            Flags: f_Flags,
            ObjectType: f_ObjectType,
            InheritedObjectType: f_InheritedObjectType,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ObjectType, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.InheritedObjectType, &mut into[28..28 + 16]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub struct ACCESS_ALLOWED_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: crate::core::GUID,
    pub InheritedObjectType: crate::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_ALLOWED_OBJECT_ACE {}
impl ::core::clone::Clone for ACCESS_ALLOWED_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_ALLOWED_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_ALLOWED_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACCESS_ALLOWED_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_ALLOWED_OBJECT_ACE {}
impl FromIntoMemory for ACCESS_ALLOWED_OBJECT_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Flags =
            <SYSTEM_AUDIT_OBJECT_ACE_FLAGS as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ObjectType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_InheritedObjectType =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[28..28 + 16]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            Flags: f_Flags,
            ObjectType: f_ObjectType,
            InheritedObjectType: f_InheritedObjectType,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ObjectType, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.InheritedObjectType, &mut into[28..28 + 16]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub struct ACCESS_DENIED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_DENIED_ACE {}
impl ::core::clone::Clone for ACCESS_DENIED_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_DENIED_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_DENIED_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACCESS_DENIED_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_DENIED_ACE {}
impl FromIntoMemory for ACCESS_DENIED_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct ACCESS_DENIED_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_DENIED_CALLBACK_ACE {}
impl ::core::clone::Clone for ACCESS_DENIED_CALLBACK_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_DENIED_CALLBACK_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_DENIED_CALLBACK_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACCESS_DENIED_CALLBACK_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_DENIED_CALLBACK_ACE {}
impl FromIntoMemory for ACCESS_DENIED_CALLBACK_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: crate::core::GUID,
    pub InheritedObjectType: crate::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_DENIED_CALLBACK_OBJECT_ACE {}
impl ::core::clone::Clone for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_DENIED_CALLBACK_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_DENIED_CALLBACK_OBJECT_ACE {}
impl FromIntoMemory for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Flags =
            <SYSTEM_AUDIT_OBJECT_ACE_FLAGS as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ObjectType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_InheritedObjectType =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[28..28 + 16]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            Flags: f_Flags,
            ObjectType: f_ObjectType,
            InheritedObjectType: f_InheritedObjectType,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ObjectType, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.InheritedObjectType, &mut into[28..28 + 16]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub struct ACCESS_DENIED_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: crate::core::GUID,
    pub InheritedObjectType: crate::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_DENIED_OBJECT_ACE {}
impl ::core::clone::Clone for ACCESS_DENIED_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_DENIED_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_DENIED_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACCESS_DENIED_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for ACCESS_DENIED_OBJECT_ACE {}
impl FromIntoMemory for ACCESS_DENIED_OBJECT_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Flags =
            <SYSTEM_AUDIT_OBJECT_ACE_FLAGS as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ObjectType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_InheritedObjectType =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[28..28 + 16]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            Flags: f_Flags,
            ObjectType: f_ObjectType,
            InheritedObjectType: f_InheritedObjectType,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ObjectType, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.InheritedObjectType, &mut into[28..28 + 16]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub struct ACCESS_REASONS {
    pub Data: [u32; 32],
}
impl ::core::marker::Copy for ACCESS_REASONS {}
impl ::core::clone::Clone for ACCESS_REASONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACCESS_REASONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_REASONS")
            .field("Data", &self.Data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACCESS_REASONS {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
    }
}
impl ::core::cmp::Eq for ACCESS_REASONS {}
impl FromIntoMemory for ACCESS_REASONS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 128);
        let f_Data = <[u32; 32] as FromIntoMemory>::from_bytes(&from[0..0 + 128]);
        Self { Data: f_Data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 128);
        FromIntoMemory::into_bytes(self.Data, &mut into[0..0 + 128]);
    }
    fn size() -> usize {
        128
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ACE_FLAGS(pub u32);
pub const CONTAINER_INHERIT_ACE: ACE_FLAGS = ACE_FLAGS(2u32);
pub const FAILED_ACCESS_ACE_FLAG: ACE_FLAGS = ACE_FLAGS(128u32);
pub const INHERIT_ONLY_ACE: ACE_FLAGS = ACE_FLAGS(8u32);
pub const INHERITED_ACE: ACE_FLAGS = ACE_FLAGS(16u32);
pub const NO_PROPAGATE_INHERIT_ACE: ACE_FLAGS = ACE_FLAGS(4u32);
pub const OBJECT_INHERIT_ACE: ACE_FLAGS = ACE_FLAGS(1u32);
pub const SUCCESSFUL_ACCESS_ACE_FLAG: ACE_FLAGS = ACE_FLAGS(64u32);
pub const SUB_CONTAINERS_AND_OBJECTS_INHERIT: ACE_FLAGS = ACE_FLAGS(3u32);
pub const SUB_CONTAINERS_ONLY_INHERIT: ACE_FLAGS = ACE_FLAGS(2u32);
pub const SUB_OBJECTS_ONLY_INHERIT: ACE_FLAGS = ACE_FLAGS(1u32);
pub const INHERIT_NO_PROPAGATE: ACE_FLAGS = ACE_FLAGS(4u32);
pub const INHERIT_ONLY: ACE_FLAGS = ACE_FLAGS(8u32);
pub const NO_INHERITANCE: ACE_FLAGS = ACE_FLAGS(0u32);
pub const INHERIT_ONLY_ACE_: ACE_FLAGS = ACE_FLAGS(8u32);
impl ::core::marker::Copy for ACE_FLAGS {}
impl ::core::clone::Clone for ACE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ACE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ACE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ACE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ACE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ACE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for ACE_FLAGS {
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
pub struct ACE_HEADER {
    pub AceType: u8,
    pub AceFlags: u8,
    pub AceSize: u16,
}
impl ::core::marker::Copy for ACE_HEADER {}
impl ::core::clone::Clone for ACE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACE_HEADER")
            .field("AceType", &self.AceType)
            .field("AceFlags", &self.AceFlags)
            .field("AceSize", &self.AceSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.AceType == other.AceType
            && self.AceFlags == other.AceFlags
            && self.AceSize == other.AceSize
    }
}
impl ::core::cmp::Eq for ACE_HEADER {}
impl FromIntoMemory for ACE_HEADER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_AceType = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_AceFlags = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_AceSize = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        Self {
            AceType: f_AceType,
            AceFlags: f_AceFlags,
            AceSize: f_AceSize,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.AceType, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.AceFlags, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.AceSize, &mut into[2..2 + 2]);
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ACE_REVISION(pub u32);
pub const ACL_REVISION: ACE_REVISION = ACE_REVISION(2u32);
pub const ACL_REVISION_DS: ACE_REVISION = ACE_REVISION(4u32);
impl ::core::marker::Copy for ACE_REVISION {}
impl ::core::clone::Clone for ACE_REVISION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACE_REVISION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACE_REVISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACE_REVISION").field(&self.0).finish()
    }
}
impl FromIntoMemory for ACE_REVISION {
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
pub struct ACL {
    pub AclRevision: u8,
    pub Sbz1: u8,
    pub AclSize: u16,
    pub AceCount: u16,
    pub Sbz2: u16,
}
impl ::core::marker::Copy for ACL {}
impl ::core::clone::Clone for ACL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACL")
            .field("AclRevision", &self.AclRevision)
            .field("Sbz1", &self.Sbz1)
            .field("AclSize", &self.AclSize)
            .field("AceCount", &self.AceCount)
            .field("Sbz2", &self.Sbz2)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACL {
    fn eq(&self, other: &Self) -> bool {
        self.AclRevision == other.AclRevision
            && self.Sbz1 == other.Sbz1
            && self.AclSize == other.AclSize
            && self.AceCount == other.AceCount
            && self.Sbz2 == other.Sbz2
    }
}
impl ::core::cmp::Eq for ACL {}
impl FromIntoMemory for ACL {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_AclRevision = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Sbz1 = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_AclSize = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_AceCount = <u16 as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_Sbz2 = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        Self {
            AclRevision: f_AclRevision,
            Sbz1: f_Sbz1,
            AclSize: f_AclSize,
            AceCount: f_AceCount,
            Sbz2: f_Sbz2,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.AclRevision, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Sbz1, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.AclSize, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.AceCount, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.Sbz2, &mut into[6..6 + 2]);
    }
    fn size() -> usize {
        8
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ACL_INFORMATION_CLASS(pub i32);
pub const AclRevisionInformation: ACL_INFORMATION_CLASS = ACL_INFORMATION_CLASS(1i32);
pub const AclSizeInformation: ACL_INFORMATION_CLASS = ACL_INFORMATION_CLASS(2i32);
impl ::core::marker::Copy for ACL_INFORMATION_CLASS {}
impl ::core::clone::Clone for ACL_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACL_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACL_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACL_INFORMATION_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for ACL_INFORMATION_CLASS {
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
pub struct ACL_REVISION_INFORMATION {
    pub AclRevision: u32,
}
impl ::core::marker::Copy for ACL_REVISION_INFORMATION {}
impl ::core::clone::Clone for ACL_REVISION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACL_REVISION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACL_REVISION_INFORMATION")
            .field("AclRevision", &self.AclRevision)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACL_REVISION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AclRevision == other.AclRevision
    }
}
impl ::core::cmp::Eq for ACL_REVISION_INFORMATION {}
impl FromIntoMemory for ACL_REVISION_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_AclRevision = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            AclRevision: f_AclRevision,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.AclRevision, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct ACL_SIZE_INFORMATION {
    pub AceCount: u32,
    pub AclBytesInUse: u32,
    pub AclBytesFree: u32,
}
impl ::core::marker::Copy for ACL_SIZE_INFORMATION {}
impl ::core::clone::Clone for ACL_SIZE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACL_SIZE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACL_SIZE_INFORMATION")
            .field("AceCount", &self.AceCount)
            .field("AclBytesInUse", &self.AclBytesInUse)
            .field("AclBytesFree", &self.AclBytesFree)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ACL_SIZE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AceCount == other.AceCount
            && self.AclBytesInUse == other.AclBytesInUse
            && self.AclBytesFree == other.AclBytesFree
    }
}
impl ::core::cmp::Eq for ACL_SIZE_INFORMATION {}
impl FromIntoMemory for ACL_SIZE_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_AceCount = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_AclBytesInUse = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_AclBytesFree = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            AceCount: f_AceCount,
            AclBytesInUse: f_AclBytesInUse,
            AclBytesFree: f_AclBytesFree,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.AceCount, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.AclBytesInUse, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.AclBytesFree, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDIT_EVENT_TYPE(pub i32);
pub const AuditEventObjectAccess: AUDIT_EVENT_TYPE = AUDIT_EVENT_TYPE(0i32);
pub const AuditEventDirectoryServiceAccess: AUDIT_EVENT_TYPE = AUDIT_EVENT_TYPE(1i32);
impl ::core::marker::Copy for AUDIT_EVENT_TYPE {}
impl ::core::clone::Clone for AUDIT_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIT_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDIT_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIT_EVENT_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for AUDIT_EVENT_TYPE {
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
pub struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: u16,
    pub Reserved: u16,
    pub AttributeCount: u32,
    pub Attribute: CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLAIM_SECURITY_ATTRIBUTES_INFORMATION")
            .field("Version", &self.Version)
            .field("Reserved", &self.Reserved)
            .field("AttributeCount", &self.AttributeCount)
            .field("Attribute", &self.Attribute)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Reserved == other.Reserved
            && self.AttributeCount == other.AttributeCount
            && self.Attribute == other.Attribute
    }
}
impl ::core::cmp::Eq for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {}
impl FromIntoMemory for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Version = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_Reserved = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_AttributeCount = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Attribute = <CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 as FromIntoMemory>::from_bytes(
            &from[8..8 + 4],
        );
        Self {
            Version: f_Version,
            Reserved: f_Reserved,
            AttributeCount: f_AttributeCount,
            Attribute: f_Attribute,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Version, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.AttributeCount, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Attribute, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    data: [u8; 4],
}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {}
impl FromIntoMemory for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
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
pub struct CLAIM_SECURITY_ATTRIBUTE_FLAGS(pub u32);
pub const CLAIM_SECURITY_ATTRIBUTE_NON_INHERITABLE: CLAIM_SECURITY_ATTRIBUTE_FLAGS =
    CLAIM_SECURITY_ATTRIBUTE_FLAGS(1u32);
pub const CLAIM_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: CLAIM_SECURITY_ATTRIBUTE_FLAGS =
    CLAIM_SECURITY_ATTRIBUTE_FLAGS(2u32);
pub const CLAIM_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY: CLAIM_SECURITY_ATTRIBUTE_FLAGS =
    CLAIM_SECURITY_ATTRIBUTE_FLAGS(4u32);
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT: CLAIM_SECURITY_ATTRIBUTE_FLAGS =
    CLAIM_SECURITY_ATTRIBUTE_FLAGS(8u32);
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED: CLAIM_SECURITY_ATTRIBUTE_FLAGS =
    CLAIM_SECURITY_ATTRIBUTE_FLAGS(16u32);
pub const CLAIM_SECURITY_ATTRIBUTE_MANDATORY: CLAIM_SECURITY_ATTRIBUTE_FLAGS =
    CLAIM_SECURITY_ATTRIBUTE_FLAGS(32u32);
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_FLAGS {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLAIM_SECURITY_ATTRIBUTE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
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
pub struct CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: u64,
    pub Name: PWSTR,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE")
            .field("Version", &self.Version)
            .field("Name", &self.Name)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {}
impl FromIntoMemory for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_Version = <u64 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_Name = <PWSTR as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Version: f_Version,
            Name: f_Name,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.Version, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.Name, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: MutPtr<::core::ffi::c_void>,
    pub ValueLength: u32,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE")
            .field("pValue", &self.pValue)
            .field("ValueLength", &self.ValueLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.pValue == other.pValue && self.ValueLength == other.ValueLength
    }
}
impl ::core::cmp::Eq for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {}
impl FromIntoMemory for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_pValue = <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ValueLength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            pValue: f_pValue,
            ValueLength: f_ValueLength,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.pValue, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ValueLength, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    pub Name: u32,
    pub ValueType: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE,
    pub Reserved: u16,
    pub Flags: CLAIM_SECURITY_ATTRIBUTE_FLAGS,
    pub ValueCount: u32,
    pub Values: CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1")
            .field("Name", &self.Name)
            .field("ValueType", &self.ValueType)
            .field("Reserved", &self.Reserved)
            .field("Flags", &self.Flags)
            .field("ValueCount", &self.ValueCount)
            .field("Values", &self.Values)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name
            && self.ValueType == other.ValueType
            && self.Reserved == other.Reserved
            && self.Flags == other.Flags
            && self.ValueCount == other.ValueCount
            && self.Values == other.Values
    }
}
impl ::core::cmp::Eq for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {}
impl FromIntoMemory for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_Name = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ValueType =
            <CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_Reserved = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_Flags =
            <CLAIM_SECURITY_ATTRIBUTE_FLAGS as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ValueCount = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Values = <CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 as FromIntoMemory>::from_bytes(
            &from[16..16 + 4],
        );
        Self {
            Name: f_Name,
            ValueType: f_ValueType,
            Reserved: f_Reserved,
            Flags: f_Flags,
            ValueCount: f_ValueCount,
            Values: f_Values,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.Name, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ValueType, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ValueCount, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Values, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    data: [u8; 4],
}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {}
impl FromIntoMemory for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
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
pub struct CLAIM_SECURITY_ATTRIBUTE_V1 {
    pub Name: PWSTR,
    pub ValueType: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE,
    pub Reserved: u16,
    pub Flags: u32,
    pub ValueCount: u32,
    pub Values: CLAIM_SECURITY_ATTRIBUTE_V1_0,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_V1 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLAIM_SECURITY_ATTRIBUTE_V1")
            .field("Name", &self.Name)
            .field("ValueType", &self.ValueType)
            .field("Reserved", &self.Reserved)
            .field("Flags", &self.Flags)
            .field("ValueCount", &self.ValueCount)
            .field("Values", &self.Values)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTE_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name
            && self.ValueType == other.ValueType
            && self.Reserved == other.Reserved
            && self.Flags == other.Flags
            && self.ValueCount == other.ValueCount
            && self.Values == other.Values
    }
}
impl ::core::cmp::Eq for CLAIM_SECURITY_ATTRIBUTE_V1 {}
impl FromIntoMemory for CLAIM_SECURITY_ATTRIBUTE_V1 {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_Name = <PWSTR as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ValueType =
            <CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE as FromIntoMemory>::from_bytes(&from[4..4 + 2]);
        let f_Reserved = <u16 as FromIntoMemory>::from_bytes(&from[6..6 + 2]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ValueCount = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Values =
            <CLAIM_SECURITY_ATTRIBUTE_V1_0 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            Name: f_Name,
            ValueType: f_ValueType,
            Reserved: f_Reserved,
            Flags: f_Flags,
            ValueCount: f_ValueCount,
            Values: f_Values,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.Name, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ValueType, &mut into[4..4 + 2]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[6..6 + 2]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ValueCount, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Values, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
pub struct CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    data: [u8; 4],
}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    fn default() -> Self {
        Self { data: [0u8; 4] }
    }
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_V1_0 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLAIM_SECURITY_ATTRIBUTE_V1_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for CLAIM_SECURITY_ATTRIBUTE_V1_0 {}
impl FromIntoMemory for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
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
pub struct CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(pub u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INT64: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE =
    CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(1u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_UINT64: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE =
    CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(2u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_STRING: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE =
    CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(3u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE =
    CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(16u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_FQBN: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE =
    CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(4u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_SID: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE =
    CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(5u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE =
    CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(6u16);
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {
    fn from_bytes(from: &[u8]) -> Self {
        Self(<u16 as FromIntoMemory>::from_bytes(from))
    }
    fn into_bytes(self, into: &mut [u8]) {
        FromIntoMemory::into_bytes(self.0, into)
    }
    fn size() -> usize {
        2
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CREATE_RESTRICTED_TOKEN_FLAGS(pub u32);
pub const DISABLE_MAX_PRIVILEGE: CREATE_RESTRICTED_TOKEN_FLAGS =
    CREATE_RESTRICTED_TOKEN_FLAGS(1u32);
pub const SANDBOX_INERT: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(2u32);
pub const LUA_TOKEN: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(4u32);
pub const WRITE_RESTRICTED: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(8u32);
impl ::core::marker::Copy for CREATE_RESTRICTED_TOKEN_FLAGS {}
impl ::core::clone::Clone for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_RESTRICTED_TOKEN_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for CREATE_RESTRICTED_TOKEN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CREATE_RESTRICTED_TOKEN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CREATE_RESTRICTED_TOKEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for CREATE_RESTRICTED_TOKEN_FLAGS {
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
pub const CVT_SECONDS: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ENUM_PERIOD(pub i32);
pub const ENUM_PERIOD_INVALID: ENUM_PERIOD = ENUM_PERIOD(-1i32);
pub const ENUM_PERIOD_SECONDS: ENUM_PERIOD = ENUM_PERIOD(0i32);
pub const ENUM_PERIOD_MINUTES: ENUM_PERIOD = ENUM_PERIOD(1i32);
pub const ENUM_PERIOD_HOURS: ENUM_PERIOD = ENUM_PERIOD(2i32);
pub const ENUM_PERIOD_DAYS: ENUM_PERIOD = ENUM_PERIOD(3i32);
pub const ENUM_PERIOD_WEEKS: ENUM_PERIOD = ENUM_PERIOD(4i32);
pub const ENUM_PERIOD_MONTHS: ENUM_PERIOD = ENUM_PERIOD(5i32);
pub const ENUM_PERIOD_YEARS: ENUM_PERIOD = ENUM_PERIOD(6i32);
impl ::core::marker::Copy for ENUM_PERIOD {}
impl ::core::clone::Clone for ENUM_PERIOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENUM_PERIOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENUM_PERIOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_PERIOD").field(&self.0).finish()
    }
}
impl FromIntoMemory for ENUM_PERIOD {
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
pub struct GENERIC_MAPPING {
    pub GenericRead: u32,
    pub GenericWrite: u32,
    pub GenericExecute: u32,
    pub GenericAll: u32,
}
impl ::core::marker::Copy for GENERIC_MAPPING {}
impl ::core::clone::Clone for GENERIC_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GENERIC_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GENERIC_MAPPING")
            .field("GenericRead", &self.GenericRead)
            .field("GenericWrite", &self.GenericWrite)
            .field("GenericExecute", &self.GenericExecute)
            .field("GenericAll", &self.GenericAll)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GENERIC_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.GenericRead == other.GenericRead
            && self.GenericWrite == other.GenericWrite
            && self.GenericExecute == other.GenericExecute
            && self.GenericAll == other.GenericAll
    }
}
impl ::core::cmp::Eq for GENERIC_MAPPING {}
impl FromIntoMemory for GENERIC_MAPPING {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_GenericRead = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_GenericWrite = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_GenericExecute = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_GenericAll = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            GenericRead: f_GenericRead,
            GenericWrite: f_GenericWrite,
            GenericExecute: f_GenericExecute,
            GenericAll: f_GenericAll,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.GenericRead, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.GenericWrite, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.GenericExecute, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.GenericAll, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HDIAGNOSTIC_DATA_QUERY_SESSION(pub PtrDiffRepr);
impl HDIAGNOSTIC_DATA_QUERY_SESSION {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HDIAGNOSTIC_DATA_QUERY_SESSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDIAGNOSTIC_DATA_QUERY_SESSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDIAGNOSTIC_DATA_QUERY_SESSION {}
impl ::core::hash::Hash for HDIAGNOSTIC_DATA_QUERY_SESSION {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HDIAGNOSTIC_DATA_QUERY_SESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDIAGNOSTIC_DATA_QUERY_SESSION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for HDIAGNOSTIC_DATA_QUERY_SESSION {
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
pub struct HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION(pub PtrDiffRepr);
impl HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {}
impl ::core::hash::Hash for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
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
pub struct HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION(pub PtrDiffRepr);
impl HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {}
impl ::core::hash::Hash for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
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
pub struct HDIAGNOSTIC_EVENT_TAG_DESCRIPTION(pub PtrDiffRepr);
impl HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {}
impl ::core::hash::Hash for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDIAGNOSTIC_EVENT_TAG_DESCRIPTION")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
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
pub struct HDIAGNOSTIC_RECORD(pub PtrDiffRepr);
impl HDIAGNOSTIC_RECORD {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HDIAGNOSTIC_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDIAGNOSTIC_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDIAGNOSTIC_RECORD {}
impl ::core::hash::Hash for HDIAGNOSTIC_RECORD {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HDIAGNOSTIC_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDIAGNOSTIC_RECORD").field(&self.0).finish()
    }
}
impl FromIntoMemory for HDIAGNOSTIC_RECORD {
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
pub struct HDIAGNOSTIC_REPORT(pub PtrDiffRepr);
impl HDIAGNOSTIC_REPORT {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HDIAGNOSTIC_REPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDIAGNOSTIC_REPORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDIAGNOSTIC_REPORT {}
impl ::core::hash::Hash for HDIAGNOSTIC_REPORT {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for HDIAGNOSTIC_REPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDIAGNOSTIC_REPORT").field(&self.0).finish()
    }
}
impl FromIntoMemory for HDIAGNOSTIC_REPORT {
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
pub struct LLFILETIME {
    pub Anonymous: LLFILETIME_0,
}
impl ::core::marker::Copy for LLFILETIME {}
impl ::core::clone::Clone for LLFILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LLFILETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LLFILETIME")
            .field("Anonymous", &self.Anonymous)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LLFILETIME {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for LLFILETIME {}
impl FromIntoMemory for LLFILETIME {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Anonymous = <LLFILETIME_0 as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self {
            Anonymous: f_Anonymous,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Anonymous, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
pub struct LLFILETIME_0 {
    data: [u8; 8],
}
impl ::core::default::Default for LLFILETIME_0 {
    fn default() -> Self {
        Self { data: [0u8; 8] }
    }
}
impl ::core::marker::Copy for LLFILETIME_0 {}
impl ::core::clone::Clone for LLFILETIME_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LLFILETIME_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LLFILETIME_0")
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LLFILETIME_0 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for LLFILETIME_0 {}
impl FromIntoMemory for LLFILETIME_0 {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LOGON32_LOGON(pub u32);
pub const LOGON32_LOGON_BATCH: LOGON32_LOGON = LOGON32_LOGON(4u32);
pub const LOGON32_LOGON_INTERACTIVE: LOGON32_LOGON = LOGON32_LOGON(2u32);
pub const LOGON32_LOGON_NETWORK: LOGON32_LOGON = LOGON32_LOGON(3u32);
pub const LOGON32_LOGON_NETWORK_CLEARTEXT: LOGON32_LOGON = LOGON32_LOGON(8u32);
pub const LOGON32_LOGON_NEW_CREDENTIALS: LOGON32_LOGON = LOGON32_LOGON(9u32);
pub const LOGON32_LOGON_SERVICE: LOGON32_LOGON = LOGON32_LOGON(5u32);
pub const LOGON32_LOGON_UNLOCK: LOGON32_LOGON = LOGON32_LOGON(7u32);
impl ::core::marker::Copy for LOGON32_LOGON {}
impl ::core::clone::Clone for LOGON32_LOGON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOGON32_LOGON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOGON32_LOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOGON32_LOGON").field(&self.0).finish()
    }
}
impl FromIntoMemory for LOGON32_LOGON {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LOGON32_PROVIDER(pub u32);
pub const LOGON32_PROVIDER_DEFAULT: LOGON32_PROVIDER = LOGON32_PROVIDER(0u32);
pub const LOGON32_PROVIDER_WINNT50: LOGON32_PROVIDER = LOGON32_PROVIDER(3u32);
pub const LOGON32_PROVIDER_WINNT40: LOGON32_PROVIDER = LOGON32_PROVIDER(2u32);
impl ::core::marker::Copy for LOGON32_PROVIDER {}
impl ::core::clone::Clone for LOGON32_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOGON32_PROVIDER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOGON32_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOGON32_PROVIDER").field(&self.0).finish()
    }
}
impl FromIntoMemory for LOGON32_PROVIDER {
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
pub struct LUID_AND_ATTRIBUTES {
    pub Luid: super::Foundation::LUID,
    pub Attributes: TOKEN_PRIVILEGES_ATTRIBUTES,
}
impl ::core::marker::Copy for LUID_AND_ATTRIBUTES {}
impl ::core::clone::Clone for LUID_AND_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LUID_AND_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LUID_AND_ATTRIBUTES")
            .field("Luid", &self.Luid)
            .field("Attributes", &self.Attributes)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LUID_AND_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Luid == other.Luid && self.Attributes == other.Attributes
    }
}
impl ::core::cmp::Eq for LUID_AND_ATTRIBUTES {}
impl FromIntoMemory for LUID_AND_ATTRIBUTES {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Luid = <super::Foundation::LUID as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_Attributes =
            <TOKEN_PRIVILEGES_ATTRIBUTES as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Luid: f_Luid,
            Attributes: f_Attributes,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Luid, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MANDATORY_LEVEL(pub i32);
pub const MandatoryLevelUntrusted: MANDATORY_LEVEL = MANDATORY_LEVEL(0i32);
pub const MandatoryLevelLow: MANDATORY_LEVEL = MANDATORY_LEVEL(1i32);
pub const MandatoryLevelMedium: MANDATORY_LEVEL = MANDATORY_LEVEL(2i32);
pub const MandatoryLevelHigh: MANDATORY_LEVEL = MANDATORY_LEVEL(3i32);
pub const MandatoryLevelSystem: MANDATORY_LEVEL = MANDATORY_LEVEL(4i32);
pub const MandatoryLevelSecureProcess: MANDATORY_LEVEL = MANDATORY_LEVEL(5i32);
pub const MandatoryLevelCount: MANDATORY_LEVEL = MANDATORY_LEVEL(6i32);
impl ::core::marker::Copy for MANDATORY_LEVEL {}
impl ::core::clone::Clone for MANDATORY_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MANDATORY_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MANDATORY_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MANDATORY_LEVEL").field(&self.0).finish()
    }
}
impl FromIntoMemory for MANDATORY_LEVEL {
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
pub struct NCRYPT_DESCRIPTOR_HANDLE(pub PtrDiffRepr);
impl NCRYPT_DESCRIPTOR_HANDLE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NCRYPT_DESCRIPTOR_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NCRYPT_DESCRIPTOR_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NCRYPT_DESCRIPTOR_HANDLE {}
impl ::core::hash::Hash for NCRYPT_DESCRIPTOR_HANDLE {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for NCRYPT_DESCRIPTOR_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NCRYPT_DESCRIPTOR_HANDLE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NCRYPT_DESCRIPTOR_HANDLE {
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
pub struct NCRYPT_STREAM_HANDLE(pub PtrDiffRepr);
impl NCRYPT_STREAM_HANDLE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NCRYPT_STREAM_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NCRYPT_STREAM_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NCRYPT_STREAM_HANDLE {}
impl ::core::hash::Hash for NCRYPT_STREAM_HANDLE {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for NCRYPT_STREAM_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NCRYPT_STREAM_HANDLE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for NCRYPT_STREAM_HANDLE {
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
pub struct OBJECT_SECURITY_INFORMATION(pub u32);
pub const ATTRIBUTE_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION =
    OBJECT_SECURITY_INFORMATION(32u32);
pub const BACKUP_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION =
    OBJECT_SECURITY_INFORMATION(65536u32);
pub const DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION =
    OBJECT_SECURITY_INFORMATION(4u32);
pub const GROUP_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION =
    OBJECT_SECURITY_INFORMATION(2u32);
pub const LABEL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION =
    OBJECT_SECURITY_INFORMATION(16u32);
pub const OWNER_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION =
    OBJECT_SECURITY_INFORMATION(1u32);
pub const PROTECTED_DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION =
    OBJECT_SECURITY_INFORMATION(2147483648u32);
pub const PROTECTED_SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION =
    OBJECT_SECURITY_INFORMATION(1073741824u32);
pub const SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION =
    OBJECT_SECURITY_INFORMATION(8u32);
pub const SCOPE_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION =
    OBJECT_SECURITY_INFORMATION(64u32);
pub const UNPROTECTED_DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION =
    OBJECT_SECURITY_INFORMATION(536870912u32);
pub const UNPROTECTED_SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION =
    OBJECT_SECURITY_INFORMATION(268435456u32);
impl ::core::marker::Copy for OBJECT_SECURITY_INFORMATION {}
impl ::core::clone::Clone for OBJECT_SECURITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OBJECT_SECURITY_INFORMATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OBJECT_SECURITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECT_SECURITY_INFORMATION")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for OBJECT_SECURITY_INFORMATION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OBJECT_SECURITY_INFORMATION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OBJECT_SECURITY_INFORMATION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OBJECT_SECURITY_INFORMATION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OBJECT_SECURITY_INFORMATION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for OBJECT_SECURITY_INFORMATION {
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
pub struct OBJECT_TYPE_LIST {
    pub Level: u16,
    pub Sbz: u16,
    pub ObjectType: MutPtr<crate::core::GUID>,
}
impl ::core::marker::Copy for OBJECT_TYPE_LIST {}
impl ::core::clone::Clone for OBJECT_TYPE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OBJECT_TYPE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECT_TYPE_LIST")
            .field("Level", &self.Level)
            .field("Sbz", &self.Sbz)
            .field("ObjectType", &self.ObjectType)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OBJECT_TYPE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level && self.Sbz == other.Sbz && self.ObjectType == other.ObjectType
    }
}
impl ::core::cmp::Eq for OBJECT_TYPE_LIST {}
impl FromIntoMemory for OBJECT_TYPE_LIST {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Level = <u16 as FromIntoMemory>::from_bytes(&from[0..0 + 2]);
        let f_Sbz = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_ObjectType =
            <MutPtr<crate::core::GUID> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Level: f_Level,
            Sbz: f_Sbz,
            ObjectType: f_ObjectType,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Level, &mut into[0..0 + 2]);
        FromIntoMemory::into_bytes(self.Sbz, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.ObjectType, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub type PLSA_AP_CALL_PACKAGE_UNTRUSTED = StdCallFnPtr<
    (
        ConstPtr<ConstPtr<::core::ffi::c_void>>,
        ConstPtr<::core::ffi::c_void>,
        ConstPtr<::core::ffi::c_void>,
        u32,
        MutPtr<ConstPtr<::core::ffi::c_void>>,
        MutPtr<u32>,
        MutPtr<i32>,
    ),
    super::Foundation::NTSTATUS,
>;
pub struct PRIVILEGE_SET {
    pub PrivilegeCount: u32,
    pub Control: u32,
    pub Privilege: [LUID_AND_ATTRIBUTES; 1],
}
impl ::core::marker::Copy for PRIVILEGE_SET {}
impl ::core::clone::Clone for PRIVILEGE_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PRIVILEGE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRIVILEGE_SET")
            .field("PrivilegeCount", &self.PrivilegeCount)
            .field("Control", &self.Control)
            .field("Privilege", &self.Privilege)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PRIVILEGE_SET {
    fn eq(&self, other: &Self) -> bool {
        self.PrivilegeCount == other.PrivilegeCount
            && self.Control == other.Control
            && self.Privilege == other.Privilege
    }
}
impl ::core::cmp::Eq for PRIVILEGE_SET {}
impl FromIntoMemory for PRIVILEGE_SET {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_PrivilegeCount = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Control = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Privilege =
            <[LUID_AND_ATTRIBUTES; 1] as FromIntoMemory>::from_bytes(&from[8..8 + 12]);
        Self {
            PrivilegeCount: f_PrivilegeCount,
            Control: f_Control,
            Privilege: f_Privilege,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.PrivilegeCount, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Control, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Privilege, &mut into[8..8 + 12]);
    }
    fn size() -> usize {
        20
    }
}
pub struct QUOTA_LIMITS {
    pub PagedPoolLimit: PtrRepr,
    pub NonPagedPoolLimit: PtrRepr,
    pub MinimumWorkingSetSize: PtrRepr,
    pub MaximumWorkingSetSize: PtrRepr,
    pub PagefileLimit: PtrRepr,
    pub TimeLimit: i64,
}
impl ::core::marker::Copy for QUOTA_LIMITS {}
impl ::core::clone::Clone for QUOTA_LIMITS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUOTA_LIMITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUOTA_LIMITS")
            .field("PagedPoolLimit", &self.PagedPoolLimit)
            .field("NonPagedPoolLimit", &self.NonPagedPoolLimit)
            .field("MinimumWorkingSetSize", &self.MinimumWorkingSetSize)
            .field("MaximumWorkingSetSize", &self.MaximumWorkingSetSize)
            .field("PagefileLimit", &self.PagefileLimit)
            .field("TimeLimit", &self.TimeLimit)
            .finish()
    }
}
impl ::core::cmp::PartialEq for QUOTA_LIMITS {
    fn eq(&self, other: &Self) -> bool {
        self.PagedPoolLimit == other.PagedPoolLimit
            && self.NonPagedPoolLimit == other.NonPagedPoolLimit
            && self.MinimumWorkingSetSize == other.MinimumWorkingSetSize
            && self.MaximumWorkingSetSize == other.MaximumWorkingSetSize
            && self.PagefileLimit == other.PagefileLimit
            && self.TimeLimit == other.TimeLimit
    }
}
impl ::core::cmp::Eq for QUOTA_LIMITS {}
impl FromIntoMemory for QUOTA_LIMITS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_PagedPoolLimit = <PtrRepr as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_NonPagedPoolLimit = <PtrRepr as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_MinimumWorkingSetSize = <PtrRepr as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_MaximumWorkingSetSize = <PtrRepr as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_PagefileLimit = <PtrRepr as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_TimeLimit = <i64 as FromIntoMemory>::from_bytes(&from[24..24 + 8]);
        Self {
            PagedPoolLimit: f_PagedPoolLimit,
            NonPagedPoolLimit: f_NonPagedPoolLimit,
            MinimumWorkingSetSize: f_MinimumWorkingSetSize,
            MaximumWorkingSetSize: f_MaximumWorkingSetSize,
            PagefileLimit: f_PagefileLimit,
            TimeLimit: f_TimeLimit,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.PagedPoolLimit, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.NonPagedPoolLimit, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.MinimumWorkingSetSize, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.MaximumWorkingSetSize, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.PagefileLimit, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.TimeLimit, &mut into[24..24 + 8]);
    }
    fn size() -> usize {
        32
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SAFER_LEVEL_HANDLE(pub PtrDiffRepr);
impl SAFER_LEVEL_HANDLE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SAFER_LEVEL_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for SAFER_LEVEL_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for SAFER_LEVEL_HANDLE {}
impl ::core::hash::Hash for SAFER_LEVEL_HANDLE {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for SAFER_LEVEL_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SAFER_LEVEL_HANDLE").field(&self.0).finish()
    }
}
impl FromIntoMemory for SAFER_LEVEL_HANDLE {
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
pub struct SC_HANDLE(pub PtrDiffRepr);
impl SC_HANDLE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SC_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for SC_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for SC_HANDLE {}
impl ::core::hash::Hash for SC_HANDLE {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl ::core::fmt::Debug for SC_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SC_HANDLE").field(&self.0).finish()
    }
}
impl FromIntoMemory for SC_HANDLE {
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
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: MutPtr<::core::ffi::c_void>,
    pub bInheritHandle: super::Foundation::BOOL,
}
impl ::core::marker::Copy for SECURITY_ATTRIBUTES {}
impl ::core::clone::Clone for SECURITY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SECURITY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_ATTRIBUTES")
            .field("nLength", &self.nLength)
            .field("lpSecurityDescriptor", &self.lpSecurityDescriptor)
            .field("bInheritHandle", &self.bInheritHandle)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SECURITY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.nLength == other.nLength
            && self.lpSecurityDescriptor == other.lpSecurityDescriptor
            && self.bInheritHandle == other.bInheritHandle
    }
}
impl ::core::cmp::Eq for SECURITY_ATTRIBUTES {}
impl FromIntoMemory for SECURITY_ATTRIBUTES {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_nLength = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_lpSecurityDescriptor =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_bInheritHandle =
            <super::Foundation::BOOL as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            nLength: f_nLength,
            lpSecurityDescriptor: f_lpSecurityDescriptor,
            bInheritHandle: f_bInheritHandle,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.nLength, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.lpSecurityDescriptor, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.bInheritHandle, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SECURITY_AUTO_INHERIT_FLAGS(pub u32);
pub const SEF_AVOID_OWNER_CHECK: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(16u32);
pub const SEF_AVOID_OWNER_RESTRICTION: SECURITY_AUTO_INHERIT_FLAGS =
    SECURITY_AUTO_INHERIT_FLAGS(4096u32);
pub const SEF_AVOID_PRIVILEGE_CHECK: SECURITY_AUTO_INHERIT_FLAGS =
    SECURITY_AUTO_INHERIT_FLAGS(8u32);
pub const SEF_DACL_AUTO_INHERIT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(1u32);
pub const SEF_DEFAULT_DESCRIPTOR_FOR_OBJECT: SECURITY_AUTO_INHERIT_FLAGS =
    SECURITY_AUTO_INHERIT_FLAGS(4u32);
pub const SEF_DEFAULT_GROUP_FROM_PARENT: SECURITY_AUTO_INHERIT_FLAGS =
    SECURITY_AUTO_INHERIT_FLAGS(64u32);
pub const SEF_DEFAULT_OWNER_FROM_PARENT: SECURITY_AUTO_INHERIT_FLAGS =
    SECURITY_AUTO_INHERIT_FLAGS(32u32);
pub const SEF_MACL_NO_EXECUTE_UP: SECURITY_AUTO_INHERIT_FLAGS =
    SECURITY_AUTO_INHERIT_FLAGS(1024u32);
pub const SEF_MACL_NO_READ_UP: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(512u32);
pub const SEF_MACL_NO_WRITE_UP: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(256u32);
pub const SEF_SACL_AUTO_INHERIT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(2u32);
impl ::core::marker::Copy for SECURITY_AUTO_INHERIT_FLAGS {}
impl ::core::clone::Clone for SECURITY_AUTO_INHERIT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SECURITY_AUTO_INHERIT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECURITY_AUTO_INHERIT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURITY_AUTO_INHERIT_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for SECURITY_AUTO_INHERIT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SECURITY_AUTO_INHERIT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SECURITY_AUTO_INHERIT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SECURITY_AUTO_INHERIT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SECURITY_AUTO_INHERIT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for SECURITY_AUTO_INHERIT_FLAGS {
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
pub struct SECURITY_CAPABILITIES {
    pub AppContainerSid: super::Foundation::PSID,
    pub Capabilities: MutPtr<SID_AND_ATTRIBUTES>,
    pub CapabilityCount: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for SECURITY_CAPABILITIES {}
impl ::core::clone::Clone for SECURITY_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SECURITY_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_CAPABILITIES")
            .field("AppContainerSid", &self.AppContainerSid)
            .field("Capabilities", &self.Capabilities)
            .field("CapabilityCount", &self.CapabilityCount)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SECURITY_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.AppContainerSid == other.AppContainerSid
            && self.Capabilities == other.Capabilities
            && self.CapabilityCount == other.CapabilityCount
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for SECURITY_CAPABILITIES {}
impl FromIntoMemory for SECURITY_CAPABILITIES {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_AppContainerSid =
            <super::Foundation::PSID as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Capabilities =
            <MutPtr<SID_AND_ATTRIBUTES> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_CapabilityCount = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_Reserved = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        Self {
            AppContainerSid: f_AppContainerSid,
            Capabilities: f_Capabilities,
            CapabilityCount: f_CapabilityCount,
            Reserved: f_Reserved,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.AppContainerSid, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Capabilities, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.CapabilityCount, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.Reserved, &mut into[12..12 + 4]);
    }
    fn size() -> usize {
        16
    }
}
pub struct SECURITY_DESCRIPTOR {
    pub Revision: u8,
    pub Sbz1: u8,
    pub Control: u16,
    pub Owner: super::Foundation::PSID,
    pub Group: super::Foundation::PSID,
    pub Sacl: MutPtr<ACL>,
    pub Dacl: MutPtr<ACL>,
}
impl ::core::marker::Copy for SECURITY_DESCRIPTOR {}
impl ::core::clone::Clone for SECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SECURITY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_DESCRIPTOR")
            .field("Revision", &self.Revision)
            .field("Sbz1", &self.Sbz1)
            .field("Control", &self.Control)
            .field("Owner", &self.Owner)
            .field("Group", &self.Group)
            .field("Sacl", &self.Sacl)
            .field("Dacl", &self.Dacl)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SECURITY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision
            && self.Sbz1 == other.Sbz1
            && self.Control == other.Control
            && self.Owner == other.Owner
            && self.Group == other.Group
            && self.Sacl == other.Sacl
            && self.Dacl == other.Dacl
    }
}
impl ::core::cmp::Eq for SECURITY_DESCRIPTOR {}
impl FromIntoMemory for SECURITY_DESCRIPTOR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 20);
        let f_Revision = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_Sbz1 = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_Control = <u16 as FromIntoMemory>::from_bytes(&from[2..2 + 2]);
        let f_Owner = <super::Foundation::PSID as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Group = <super::Foundation::PSID as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_Sacl = <MutPtr<ACL> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_Dacl = <MutPtr<ACL> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        Self {
            Revision: f_Revision,
            Sbz1: f_Sbz1,
            Control: f_Control,
            Owner: f_Owner,
            Group: f_Group,
            Sacl: f_Sacl,
            Dacl: f_Dacl,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 20);
        FromIntoMemory::into_bytes(self.Revision, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.Sbz1, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.Control, &mut into[2..2 + 2]);
        FromIntoMemory::into_bytes(self.Owner, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Group, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.Sacl, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.Dacl, &mut into[16..16 + 4]);
    }
    fn size() -> usize {
        20
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SECURITY_IMPERSONATION_LEVEL(pub i32);
pub const SecurityAnonymous: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(0i32);
pub const SecurityIdentification: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(1i32);
pub const SecurityImpersonation: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(2i32);
pub const SecurityDelegation: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(3i32);
impl ::core::marker::Copy for SECURITY_IMPERSONATION_LEVEL {}
impl ::core::clone::Clone for SECURITY_IMPERSONATION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SECURITY_IMPERSONATION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECURITY_IMPERSONATION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURITY_IMPERSONATION_LEVEL")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for SECURITY_IMPERSONATION_LEVEL {
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
pub struct SECURITY_QUALITY_OF_SERVICE {
    pub Length: u32,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub ContextTrackingMode: u8,
    pub EffectiveOnly: super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for SECURITY_QUALITY_OF_SERVICE {}
impl ::core::clone::Clone for SECURITY_QUALITY_OF_SERVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SECURITY_QUALITY_OF_SERVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_QUALITY_OF_SERVICE")
            .field("Length", &self.Length)
            .field("ImpersonationLevel", &self.ImpersonationLevel)
            .field("ContextTrackingMode", &self.ContextTrackingMode)
            .field("EffectiveOnly", &self.EffectiveOnly)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SECURITY_QUALITY_OF_SERVICE {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.ImpersonationLevel == other.ImpersonationLevel
            && self.ContextTrackingMode == other.ContextTrackingMode
            && self.EffectiveOnly == other.EffectiveOnly
    }
}
impl ::core::cmp::Eq for SECURITY_QUALITY_OF_SERVICE {}
impl FromIntoMemory for SECURITY_QUALITY_OF_SERVICE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Length = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ImpersonationLevel =
            <SECURITY_IMPERSONATION_LEVEL as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_ContextTrackingMode = <u8 as FromIntoMemory>::from_bytes(&from[8..8 + 1]);
        let f_EffectiveOnly =
            <super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[9..9 + 1]);
        Self {
            Length: f_Length,
            ImpersonationLevel: f_ImpersonationLevel,
            ContextTrackingMode: f_ContextTrackingMode,
            EffectiveOnly: f_EffectiveOnly,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Length, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ImpersonationLevel, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.ContextTrackingMode, &mut into[8..8 + 1]);
        FromIntoMemory::into_bytes(self.EffectiveOnly, &mut into[9..9 + 1]);
    }
    fn size() -> usize {
        12
    }
}
pub type SEC_THREAD_START = StdCallFnPtr<(MutPtr<::core::ffi::c_void>,), u32>;
pub struct SE_ACCESS_REPLY {
    pub Size: u32,
    pub ResultListCount: u32,
    pub GrantedAccess: MutPtr<u32>,
    pub AccessStatus: MutPtr<u32>,
    pub AccessReason: MutPtr<ACCESS_REASONS>,
    pub Privileges: MutPtr<ConstPtr<PRIVILEGE_SET>>,
}
impl ::core::marker::Copy for SE_ACCESS_REPLY {}
impl ::core::clone::Clone for SE_ACCESS_REPLY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SE_ACCESS_REPLY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_ACCESS_REPLY")
            .field("Size", &self.Size)
            .field("ResultListCount", &self.ResultListCount)
            .field("GrantedAccess", &self.GrantedAccess)
            .field("AccessStatus", &self.AccessStatus)
            .field("AccessReason", &self.AccessReason)
            .field("Privileges", &self.Privileges)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SE_ACCESS_REPLY {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.ResultListCount == other.ResultListCount
            && self.GrantedAccess == other.GrantedAccess
            && self.AccessStatus == other.AccessStatus
            && self.AccessReason == other.AccessReason
            && self.Privileges == other.Privileges
    }
}
impl ::core::cmp::Eq for SE_ACCESS_REPLY {}
impl FromIntoMemory for SE_ACCESS_REPLY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 24);
        let f_Size = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_ResultListCount = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_GrantedAccess = <MutPtr<u32> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_AccessStatus = <MutPtr<u32> as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_AccessReason =
            <MutPtr<ACCESS_REASONS> as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_Privileges =
            <MutPtr<ConstPtr<PRIVILEGE_SET>> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        Self {
            Size: f_Size,
            ResultListCount: f_ResultListCount,
            GrantedAccess: f_GrantedAccess,
            AccessStatus: f_AccessStatus,
            AccessReason: f_AccessReason,
            Privileges: f_Privileges,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 24);
        FromIntoMemory::into_bytes(self.Size, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.ResultListCount, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.GrantedAccess, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.AccessStatus, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.AccessReason, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.Privileges, &mut into[20..20 + 4]);
    }
    fn size() -> usize {
        24
    }
}
pub struct SE_ACCESS_REQUEST {
    pub Size: u32,
    pub SeSecurityDescriptor: MutPtr<SE_SECURITY_DESCRIPTOR>,
    pub DesiredAccess: u32,
    pub PreviouslyGrantedAccess: u32,
    pub PrincipalSelfSid: super::Foundation::PSID,
    pub GenericMapping: MutPtr<GENERIC_MAPPING>,
    pub ObjectTypeListCount: u32,
    pub ObjectTypeList: MutPtr<OBJECT_TYPE_LIST>,
}
impl ::core::marker::Copy for SE_ACCESS_REQUEST {}
impl ::core::clone::Clone for SE_ACCESS_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SE_ACCESS_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_ACCESS_REQUEST")
            .field("Size", &self.Size)
            .field("SeSecurityDescriptor", &self.SeSecurityDescriptor)
            .field("DesiredAccess", &self.DesiredAccess)
            .field("PreviouslyGrantedAccess", &self.PreviouslyGrantedAccess)
            .field("PrincipalSelfSid", &self.PrincipalSelfSid)
            .field("GenericMapping", &self.GenericMapping)
            .field("ObjectTypeListCount", &self.ObjectTypeListCount)
            .field("ObjectTypeList", &self.ObjectTypeList)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SE_ACCESS_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.SeSecurityDescriptor == other.SeSecurityDescriptor
            && self.DesiredAccess == other.DesiredAccess
            && self.PreviouslyGrantedAccess == other.PreviouslyGrantedAccess
            && self.PrincipalSelfSid == other.PrincipalSelfSid
            && self.GenericMapping == other.GenericMapping
            && self.ObjectTypeListCount == other.ObjectTypeListCount
            && self.ObjectTypeList == other.ObjectTypeList
    }
}
impl ::core::cmp::Eq for SE_ACCESS_REQUEST {}
impl FromIntoMemory for SE_ACCESS_REQUEST {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 32);
        let f_Size = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_SeSecurityDescriptor =
            <MutPtr<SE_SECURITY_DESCRIPTOR> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_DesiredAccess = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_PreviouslyGrantedAccess = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_PrincipalSelfSid =
            <super::Foundation::PSID as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_GenericMapping =
            <MutPtr<GENERIC_MAPPING> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ObjectTypeListCount = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ObjectTypeList =
            <MutPtr<OBJECT_TYPE_LIST> as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        Self {
            Size: f_Size,
            SeSecurityDescriptor: f_SeSecurityDescriptor,
            DesiredAccess: f_DesiredAccess,
            PreviouslyGrantedAccess: f_PreviouslyGrantedAccess,
            PrincipalSelfSid: f_PrincipalSelfSid,
            GenericMapping: f_GenericMapping,
            ObjectTypeListCount: f_ObjectTypeListCount,
            ObjectTypeList: f_ObjectTypeList,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 32);
        FromIntoMemory::into_bytes(self.Size, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.SeSecurityDescriptor, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.DesiredAccess, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.PreviouslyGrantedAccess, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.PrincipalSelfSid, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.GenericMapping, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ObjectTypeListCount, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ObjectTypeList, &mut into[28..28 + 4]);
    }
    fn size() -> usize {
        32
    }
}
pub struct SE_IMPERSONATION_STATE {
    pub Token: MutPtr<::core::ffi::c_void>,
    pub CopyOnOpen: super::Foundation::BOOLEAN,
    pub EffectiveOnly: super::Foundation::BOOLEAN,
    pub Level: SECURITY_IMPERSONATION_LEVEL,
}
impl ::core::marker::Copy for SE_IMPERSONATION_STATE {}
impl ::core::clone::Clone for SE_IMPERSONATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SE_IMPERSONATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_IMPERSONATION_STATE")
            .field("Token", &self.Token)
            .field("CopyOnOpen", &self.CopyOnOpen)
            .field("EffectiveOnly", &self.EffectiveOnly)
            .field("Level", &self.Level)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SE_IMPERSONATION_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Token == other.Token
            && self.CopyOnOpen == other.CopyOnOpen
            && self.EffectiveOnly == other.EffectiveOnly
            && self.Level == other.Level
    }
}
impl ::core::cmp::Eq for SE_IMPERSONATION_STATE {}
impl FromIntoMemory for SE_IMPERSONATION_STATE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Token = <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_CopyOnOpen =
            <super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[4..4 + 1]);
        let f_EffectiveOnly =
            <super::Foundation::BOOLEAN as FromIntoMemory>::from_bytes(&from[5..5 + 1]);
        let f_Level = <SECURITY_IMPERSONATION_LEVEL as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Token: f_Token,
            CopyOnOpen: f_CopyOnOpen,
            EffectiveOnly: f_EffectiveOnly,
            Level: f_Level,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Token, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.CopyOnOpen, &mut into[4..4 + 1]);
        FromIntoMemory::into_bytes(self.EffectiveOnly, &mut into[5..5 + 1]);
        FromIntoMemory::into_bytes(self.Level, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct SE_SECURITY_DESCRIPTOR {
    pub Size: u32,
    pub Flags: u32,
    pub SecurityDescriptor: MutPtr<SECURITY_DESCRIPTOR>,
}
impl ::core::marker::Copy for SE_SECURITY_DESCRIPTOR {}
impl ::core::clone::Clone for SE_SECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SE_SECURITY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_SECURITY_DESCRIPTOR")
            .field("Size", &self.Size)
            .field("Flags", &self.Flags)
            .field("SecurityDescriptor", &self.SecurityDescriptor)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SE_SECURITY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Flags == other.Flags
            && self.SecurityDescriptor == other.SecurityDescriptor
    }
}
impl ::core::cmp::Eq for SE_SECURITY_DESCRIPTOR {}
impl FromIntoMemory for SE_SECURITY_DESCRIPTOR {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Size = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SecurityDescriptor =
            <MutPtr<SECURITY_DESCRIPTOR> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Size: f_Size,
            Flags: f_Flags,
            SecurityDescriptor: f_SecurityDescriptor,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Size, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SecurityDescriptor, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct SE_SID {
    data: [u8; 68],
}
impl ::core::default::Default for SE_SID {
    fn default() -> Self {
        Self { data: [0u8; 68] }
    }
}
impl ::core::marker::Copy for SE_SID {}
impl ::core::clone::Clone for SE_SID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SE_SID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SE_SID").field("data", &self.data).finish()
    }
}
impl ::core::cmp::PartialEq for SE_SID {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for SE_SID {}
impl FromIntoMemory for SE_SID {
    fn from_bytes(from: &[u8]) -> Self {
        let mut data = [0u8; 68];
        <_ as AsMut<[u8]>>::as_mut(&mut data).clone_from_slice(from);
        Self { data }
    }
    fn into_bytes(self, into: &mut [u8]) {
        into.clone_from_slice(<_ as AsRef<[u8]>>::as_ref(&self.data));
    }
    fn size() -> usize {
        68
    }
}
pub struct SID {
    pub Revision: u8,
    pub SubAuthorityCount: u8,
    pub IdentifierAuthority: SID_IDENTIFIER_AUTHORITY,
    pub SubAuthority: [u32; 1],
}
impl ::core::marker::Copy for SID {}
impl ::core::clone::Clone for SID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SID")
            .field("Revision", &self.Revision)
            .field("SubAuthorityCount", &self.SubAuthorityCount)
            .field("IdentifierAuthority", &self.IdentifierAuthority)
            .field("SubAuthority", &self.SubAuthority)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SID {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision
            && self.SubAuthorityCount == other.SubAuthorityCount
            && self.IdentifierAuthority == other.IdentifierAuthority
            && self.SubAuthority == other.SubAuthority
    }
}
impl ::core::cmp::Eq for SID {}
impl FromIntoMemory for SID {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Revision = <u8 as FromIntoMemory>::from_bytes(&from[0..0 + 1]);
        let f_SubAuthorityCount = <u8 as FromIntoMemory>::from_bytes(&from[1..1 + 1]);
        let f_IdentifierAuthority =
            <SID_IDENTIFIER_AUTHORITY as FromIntoMemory>::from_bytes(&from[2..2 + 6]);
        let f_SubAuthority = <[u32; 1] as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Revision: f_Revision,
            SubAuthorityCount: f_SubAuthorityCount,
            IdentifierAuthority: f_IdentifierAuthority,
            SubAuthority: f_SubAuthority,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Revision, &mut into[0..0 + 1]);
        FromIntoMemory::into_bytes(self.SubAuthorityCount, &mut into[1..1 + 1]);
        FromIntoMemory::into_bytes(self.IdentifierAuthority, &mut into[2..2 + 6]);
        FromIntoMemory::into_bytes(self.SubAuthority, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct SID_AND_ATTRIBUTES {
    pub Sid: super::Foundation::PSID,
    pub Attributes: u32,
}
impl ::core::marker::Copy for SID_AND_ATTRIBUTES {}
impl ::core::clone::Clone for SID_AND_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SID_AND_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SID_AND_ATTRIBUTES")
            .field("Sid", &self.Sid)
            .field("Attributes", &self.Attributes)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SID_AND_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Sid == other.Sid && self.Attributes == other.Attributes
    }
}
impl ::core::cmp::Eq for SID_AND_ATTRIBUTES {}
impl FromIntoMemory for SID_AND_ATTRIBUTES {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Sid = <super::Foundation::PSID as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Attributes = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        Self {
            Sid: f_Sid,
            Attributes: f_Attributes,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Sid, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Attributes, &mut into[4..4 + 4]);
    }
    fn size() -> usize {
        8
    }
}
pub struct SID_AND_ATTRIBUTES_HASH {
    pub SidCount: u32,
    pub SidAttr: MutPtr<SID_AND_ATTRIBUTES>,
    pub Hash: [PtrRepr; 32],
}
impl ::core::marker::Copy for SID_AND_ATTRIBUTES_HASH {}
impl ::core::clone::Clone for SID_AND_ATTRIBUTES_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SID_AND_ATTRIBUTES_HASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SID_AND_ATTRIBUTES_HASH")
            .field("SidCount", &self.SidCount)
            .field("SidAttr", &self.SidAttr)
            .field("Hash", &self.Hash)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SID_AND_ATTRIBUTES_HASH {
    fn eq(&self, other: &Self) -> bool {
        self.SidCount == other.SidCount && self.SidAttr == other.SidAttr && self.Hash == other.Hash
    }
}
impl ::core::cmp::Eq for SID_AND_ATTRIBUTES_HASH {}
impl FromIntoMemory for SID_AND_ATTRIBUTES_HASH {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 136);
        let f_SidCount = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_SidAttr = <MutPtr<SID_AND_ATTRIBUTES> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Hash = <[PtrRepr; 32] as FromIntoMemory>::from_bytes(&from[8..8 + 128]);
        Self {
            SidCount: f_SidCount,
            SidAttr: f_SidAttr,
            Hash: f_Hash,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 136);
        FromIntoMemory::into_bytes(self.SidCount, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.SidAttr, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Hash, &mut into[8..8 + 128]);
    }
    fn size() -> usize {
        136
    }
}
pub struct SID_IDENTIFIER_AUTHORITY {
    pub Value: [u8; 6],
}
impl ::core::marker::Copy for SID_IDENTIFIER_AUTHORITY {}
impl ::core::clone::Clone for SID_IDENTIFIER_AUTHORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SID_IDENTIFIER_AUTHORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SID_IDENTIFIER_AUTHORITY")
            .field("Value", &self.Value)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SID_IDENTIFIER_AUTHORITY {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for SID_IDENTIFIER_AUTHORITY {}
impl FromIntoMemory for SID_IDENTIFIER_AUTHORITY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 6);
        let f_Value = <[u8; 6] as FromIntoMemory>::from_bytes(&from[0..0 + 6]);
        Self { Value: f_Value }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 6);
        FromIntoMemory::into_bytes(self.Value, &mut into[0..0 + 6]);
    }
    fn size() -> usize {
        6
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SID_NAME_USE(pub i32);
pub const SidTypeUser: SID_NAME_USE = SID_NAME_USE(1i32);
pub const SidTypeGroup: SID_NAME_USE = SID_NAME_USE(2i32);
pub const SidTypeDomain: SID_NAME_USE = SID_NAME_USE(3i32);
pub const SidTypeAlias: SID_NAME_USE = SID_NAME_USE(4i32);
pub const SidTypeWellKnownGroup: SID_NAME_USE = SID_NAME_USE(5i32);
pub const SidTypeDeletedAccount: SID_NAME_USE = SID_NAME_USE(6i32);
pub const SidTypeInvalid: SID_NAME_USE = SID_NAME_USE(7i32);
pub const SidTypeUnknown: SID_NAME_USE = SID_NAME_USE(8i32);
pub const SidTypeComputer: SID_NAME_USE = SID_NAME_USE(9i32);
pub const SidTypeLabel: SID_NAME_USE = SID_NAME_USE(10i32);
pub const SidTypeLogonSession: SID_NAME_USE = SID_NAME_USE(11i32);
impl ::core::marker::Copy for SID_NAME_USE {}
impl ::core::clone::Clone for SID_NAME_USE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SID_NAME_USE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SID_NAME_USE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SID_NAME_USE").field(&self.0).finish()
    }
}
impl FromIntoMemory for SID_NAME_USE {
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
pub struct SYSTEM_ACCESS_FILTER_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ACCESS_FILTER_ACE {}
impl ::core::clone::Clone for SYSTEM_ACCESS_FILTER_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_ACCESS_FILTER_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_ACCESS_FILTER_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_ACCESS_FILTER_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_ACCESS_FILTER_ACE {}
impl FromIntoMemory for SYSTEM_ACCESS_FILTER_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct SYSTEM_ALARM_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ALARM_ACE {}
impl ::core::clone::Clone for SYSTEM_ALARM_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_ALARM_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_ALARM_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_ALARM_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_ALARM_ACE {}
impl FromIntoMemory for SYSTEM_ALARM_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct SYSTEM_ALARM_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ALARM_CALLBACK_ACE {}
impl ::core::clone::Clone for SYSTEM_ALARM_CALLBACK_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_ALARM_CALLBACK_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_ALARM_CALLBACK_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_ALARM_CALLBACK_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_ALARM_CALLBACK_ACE {}
impl FromIntoMemory for SYSTEM_ALARM_CALLBACK_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: crate::core::GUID,
    pub InheritedObjectType: crate::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {}
impl ::core::clone::Clone for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_ALARM_CALLBACK_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {}
impl FromIntoMemory for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Flags =
            <SYSTEM_AUDIT_OBJECT_ACE_FLAGS as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ObjectType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_InheritedObjectType =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[28..28 + 16]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            Flags: f_Flags,
            ObjectType: f_ObjectType,
            InheritedObjectType: f_InheritedObjectType,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ObjectType, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.InheritedObjectType, &mut into[28..28 + 16]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub struct SYSTEM_ALARM_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: u32,
    pub ObjectType: crate::core::GUID,
    pub InheritedObjectType: crate::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ALARM_OBJECT_ACE {}
impl ::core::clone::Clone for SYSTEM_ALARM_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_ALARM_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_ALARM_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_ALARM_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_ALARM_OBJECT_ACE {}
impl FromIntoMemory for SYSTEM_ALARM_OBJECT_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ObjectType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_InheritedObjectType =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[28..28 + 16]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            Flags: f_Flags,
            ObjectType: f_ObjectType,
            InheritedObjectType: f_InheritedObjectType,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ObjectType, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.InheritedObjectType, &mut into[28..28 + 16]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub struct SYSTEM_AUDIT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_AUDIT_ACE {}
impl ::core::clone::Clone for SYSTEM_AUDIT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_AUDIT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_AUDIT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_AUDIT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_AUDIT_ACE {}
impl FromIntoMemory for SYSTEM_AUDIT_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct SYSTEM_AUDIT_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_AUDIT_CALLBACK_ACE {}
impl ::core::clone::Clone for SYSTEM_AUDIT_CALLBACK_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_AUDIT_CALLBACK_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_AUDIT_CALLBACK_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_AUDIT_CALLBACK_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_AUDIT_CALLBACK_ACE {}
impl FromIntoMemory for SYSTEM_AUDIT_CALLBACK_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: crate::core::GUID,
    pub InheritedObjectType: crate::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {}
impl ::core::clone::Clone for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_AUDIT_CALLBACK_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {}
impl FromIntoMemory for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Flags =
            <SYSTEM_AUDIT_OBJECT_ACE_FLAGS as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ObjectType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_InheritedObjectType =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[28..28 + 16]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            Flags: f_Flags,
            ObjectType: f_ObjectType,
            InheritedObjectType: f_InheritedObjectType,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ObjectType, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.InheritedObjectType, &mut into[28..28 + 16]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
pub struct SYSTEM_AUDIT_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: crate::core::GUID,
    pub InheritedObjectType: crate::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_AUDIT_OBJECT_ACE {}
impl ::core::clone::Clone for SYSTEM_AUDIT_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_AUDIT_OBJECT_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_AUDIT_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_AUDIT_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_AUDIT_OBJECT_ACE {}
impl FromIntoMemory for SYSTEM_AUDIT_OBJECT_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 48);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Flags =
            <SYSTEM_AUDIT_OBJECT_ACE_FLAGS as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_ObjectType = <crate::core::GUID as FromIntoMemory>::from_bytes(&from[12..12 + 16]);
        let f_InheritedObjectType =
            <crate::core::GUID as FromIntoMemory>::from_bytes(&from[28..28 + 16]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            Flags: f_Flags,
            ObjectType: f_ObjectType,
            InheritedObjectType: f_InheritedObjectType,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 48);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.ObjectType, &mut into[12..12 + 16]);
        FromIntoMemory::into_bytes(self.InheritedObjectType, &mut into[28..28 + 16]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[44..44 + 4]);
    }
    fn size() -> usize {
        48
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYSTEM_AUDIT_OBJECT_ACE_FLAGS(pub u32);
pub const ACE_OBJECT_TYPE_PRESENT: SYSTEM_AUDIT_OBJECT_ACE_FLAGS =
    SYSTEM_AUDIT_OBJECT_ACE_FLAGS(1u32);
pub const ACE_INHERITED_OBJECT_TYPE_PRESENT: SYSTEM_AUDIT_OBJECT_ACE_FLAGS =
    SYSTEM_AUDIT_OBJECT_ACE_FLAGS(2u32);
impl ::core::marker::Copy for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {}
impl ::core::clone::Clone for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_AUDIT_OBJECT_ACE_FLAGS")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
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
pub struct SYSTEM_MANDATORY_LABEL_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_MANDATORY_LABEL_ACE {}
impl ::core::clone::Clone for SYSTEM_MANDATORY_LABEL_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_MANDATORY_LABEL_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_MANDATORY_LABEL_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_MANDATORY_LABEL_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_MANDATORY_LABEL_ACE {}
impl FromIntoMemory for SYSTEM_MANDATORY_LABEL_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct SYSTEM_PROCESS_TRUST_LABEL_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_PROCESS_TRUST_LABEL_ACE {}
impl ::core::clone::Clone for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PROCESS_TRUST_LABEL_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_PROCESS_TRUST_LABEL_ACE {}
impl FromIntoMemory for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_RESOURCE_ATTRIBUTE_ACE {}
impl ::core::clone::Clone for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_RESOURCE_ATTRIBUTE_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_RESOURCE_ATTRIBUTE_ACE {}
impl FromIntoMemory for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct SYSTEM_SCOPED_POLICY_ID_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_SCOPED_POLICY_ID_ACE {}
impl ::core::clone::Clone for SYSTEM_SCOPED_POLICY_ID_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_SCOPED_POLICY_ID_ACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_SCOPED_POLICY_ID_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_SCOPED_POLICY_ID_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::core::cmp::Eq for SYSTEM_SCOPED_POLICY_ID_ACE {}
impl FromIntoMemory for SYSTEM_SCOPED_POLICY_ID_ACE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_Header = <ACE_HEADER as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Mask = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_SidStart = <u32 as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        Self {
            Header: f_Header,
            Mask: f_Mask,
            SidStart: f_SidStart,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.Header, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Mask, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.SidStart, &mut into[8..8 + 4]);
    }
    fn size() -> usize {
        12
    }
}
pub struct TOKEN_ACCESS_INFORMATION {
    pub SidHash: MutPtr<SID_AND_ATTRIBUTES_HASH>,
    pub RestrictedSidHash: MutPtr<SID_AND_ATTRIBUTES_HASH>,
    pub Privileges: MutPtr<TOKEN_PRIVILEGES>,
    pub AuthenticationId: super::Foundation::LUID,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub MandatoryPolicy: TOKEN_MANDATORY_POLICY,
    pub Flags: u32,
    pub AppContainerNumber: u32,
    pub PackageSid: super::Foundation::PSID,
    pub CapabilitiesHash: MutPtr<SID_AND_ATTRIBUTES_HASH>,
    pub TrustLevelSid: super::Foundation::PSID,
    pub SecurityAttributes: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for TOKEN_ACCESS_INFORMATION {}
impl ::core::clone::Clone for TOKEN_ACCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_ACCESS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_ACCESS_INFORMATION")
            .field("SidHash", &self.SidHash)
            .field("RestrictedSidHash", &self.RestrictedSidHash)
            .field("Privileges", &self.Privileges)
            .field("AuthenticationId", &self.AuthenticationId)
            .field("TokenType", &self.TokenType)
            .field("ImpersonationLevel", &self.ImpersonationLevel)
            .field("MandatoryPolicy", &self.MandatoryPolicy)
            .field("Flags", &self.Flags)
            .field("AppContainerNumber", &self.AppContainerNumber)
            .field("PackageSid", &self.PackageSid)
            .field("CapabilitiesHash", &self.CapabilitiesHash)
            .field("TrustLevelSid", &self.TrustLevelSid)
            .field("SecurityAttributes", &self.SecurityAttributes)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_ACCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.SidHash == other.SidHash
            && self.RestrictedSidHash == other.RestrictedSidHash
            && self.Privileges == other.Privileges
            && self.AuthenticationId == other.AuthenticationId
            && self.TokenType == other.TokenType
            && self.ImpersonationLevel == other.ImpersonationLevel
            && self.MandatoryPolicy == other.MandatoryPolicy
            && self.Flags == other.Flags
            && self.AppContainerNumber == other.AppContainerNumber
            && self.PackageSid == other.PackageSid
            && self.CapabilitiesHash == other.CapabilitiesHash
            && self.TrustLevelSid == other.TrustLevelSid
            && self.SecurityAttributes == other.SecurityAttributes
    }
}
impl ::core::cmp::Eq for TOKEN_ACCESS_INFORMATION {}
impl FromIntoMemory for TOKEN_ACCESS_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 56);
        let f_SidHash =
            <MutPtr<SID_AND_ATTRIBUTES_HASH> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_RestrictedSidHash =
            <MutPtr<SID_AND_ATTRIBUTES_HASH> as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Privileges =
            <MutPtr<TOKEN_PRIVILEGES> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_AuthenticationId =
            <super::Foundation::LUID as FromIntoMemory>::from_bytes(&from[12..12 + 8]);
        let f_TokenType = <TOKEN_TYPE as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_ImpersonationLevel =
            <SECURITY_IMPERSONATION_LEVEL as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_MandatoryPolicy =
            <TOKEN_MANDATORY_POLICY as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_Flags = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_AppContainerNumber = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_PackageSid =
            <super::Foundation::PSID as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_CapabilitiesHash =
            <MutPtr<SID_AND_ATTRIBUTES_HASH> as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_TrustLevelSid =
            <super::Foundation::PSID as FromIntoMemory>::from_bytes(&from[48..48 + 4]);
        let f_SecurityAttributes =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[52..52 + 4]);
        Self {
            SidHash: f_SidHash,
            RestrictedSidHash: f_RestrictedSidHash,
            Privileges: f_Privileges,
            AuthenticationId: f_AuthenticationId,
            TokenType: f_TokenType,
            ImpersonationLevel: f_ImpersonationLevel,
            MandatoryPolicy: f_MandatoryPolicy,
            Flags: f_Flags,
            AppContainerNumber: f_AppContainerNumber,
            PackageSid: f_PackageSid,
            CapabilitiesHash: f_CapabilitiesHash,
            TrustLevelSid: f_TrustLevelSid,
            SecurityAttributes: f_SecurityAttributes,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 56);
        FromIntoMemory::into_bytes(self.SidHash, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.RestrictedSidHash, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Privileges, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.AuthenticationId, &mut into[12..12 + 8]);
        FromIntoMemory::into_bytes(self.TokenType, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.ImpersonationLevel, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.MandatoryPolicy, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.Flags, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.AppContainerNumber, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.PackageSid, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.CapabilitiesHash, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.TrustLevelSid, &mut into[48..48 + 4]);
        FromIntoMemory::into_bytes(self.SecurityAttributes, &mut into[52..52 + 4]);
    }
    fn size() -> usize {
        56
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TOKEN_ACCESS_MASK(pub u32);
pub const TOKEN_DELETE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(65536u32);
pub const TOKEN_READ_CONTROL: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(131072u32);
pub const TOKEN_WRITE_DAC: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(262144u32);
pub const TOKEN_WRITE_OWNER: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(524288u32);
pub const TOKEN_ACCESS_SYSTEM_SECURITY: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(16777216u32);
pub const TOKEN_ASSIGN_PRIMARY: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(1u32);
pub const TOKEN_DUPLICATE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(2u32);
pub const TOKEN_IMPERSONATE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(4u32);
pub const TOKEN_QUERY: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(8u32);
pub const TOKEN_QUERY_SOURCE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(16u32);
pub const TOKEN_ADJUST_PRIVILEGES: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(32u32);
pub const TOKEN_ADJUST_GROUPS: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(64u32);
pub const TOKEN_ADJUST_DEFAULT: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(128u32);
pub const TOKEN_ADJUST_SESSIONID: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(256u32);
pub const TOKEN_ALL_ACCESS: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(983295u32);
impl ::core::marker::Copy for TOKEN_ACCESS_MASK {}
impl ::core::clone::Clone for TOKEN_ACCESS_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOKEN_ACCESS_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOKEN_ACCESS_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKEN_ACCESS_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TOKEN_ACCESS_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TOKEN_ACCESS_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TOKEN_ACCESS_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TOKEN_ACCESS_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TOKEN_ACCESS_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for TOKEN_ACCESS_MASK {
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
pub struct TOKEN_APPCONTAINER_INFORMATION {
    pub TokenAppContainer: super::Foundation::PSID,
}
impl ::core::marker::Copy for TOKEN_APPCONTAINER_INFORMATION {}
impl ::core::clone::Clone for TOKEN_APPCONTAINER_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_APPCONTAINER_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_APPCONTAINER_INFORMATION")
            .field("TokenAppContainer", &self.TokenAppContainer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_APPCONTAINER_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TokenAppContainer == other.TokenAppContainer
    }
}
impl ::core::cmp::Eq for TOKEN_APPCONTAINER_INFORMATION {}
impl FromIntoMemory for TOKEN_APPCONTAINER_INFORMATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_TokenAppContainer =
            <super::Foundation::PSID as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            TokenAppContainer: f_TokenAppContainer,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.TokenAppContainer, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct TOKEN_AUDIT_POLICY {
    pub PerUserPolicy: [u8; 30],
}
impl ::core::marker::Copy for TOKEN_AUDIT_POLICY {}
impl ::core::clone::Clone for TOKEN_AUDIT_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_AUDIT_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_AUDIT_POLICY")
            .field("PerUserPolicy", &self.PerUserPolicy)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_AUDIT_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.PerUserPolicy == other.PerUserPolicy
    }
}
impl ::core::cmp::Eq for TOKEN_AUDIT_POLICY {}
impl FromIntoMemory for TOKEN_AUDIT_POLICY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 30);
        let f_PerUserPolicy = <[u8; 30] as FromIntoMemory>::from_bytes(&from[0..0 + 30]);
        Self {
            PerUserPolicy: f_PerUserPolicy,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 30);
        FromIntoMemory::into_bytes(self.PerUserPolicy, &mut into[0..0 + 30]);
    }
    fn size() -> usize {
        30
    }
}
pub struct TOKEN_CONTROL {
    pub TokenId: super::Foundation::LUID,
    pub AuthenticationId: super::Foundation::LUID,
    pub ModifiedId: super::Foundation::LUID,
    pub TokenSource: TOKEN_SOURCE,
}
impl ::core::marker::Copy for TOKEN_CONTROL {}
impl ::core::clone::Clone for TOKEN_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_CONTROL")
            .field("TokenId", &self.TokenId)
            .field("AuthenticationId", &self.AuthenticationId)
            .field("ModifiedId", &self.ModifiedId)
            .field("TokenSource", &self.TokenSource)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.TokenId == other.TokenId
            && self.AuthenticationId == other.AuthenticationId
            && self.ModifiedId == other.ModifiedId
            && self.TokenSource == other.TokenSource
    }
}
impl ::core::cmp::Eq for TOKEN_CONTROL {}
impl FromIntoMemory for TOKEN_CONTROL {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 40);
        let f_TokenId = <super::Foundation::LUID as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_AuthenticationId =
            <super::Foundation::LUID as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_ModifiedId =
            <super::Foundation::LUID as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_TokenSource = <TOKEN_SOURCE as FromIntoMemory>::from_bytes(&from[24..24 + 16]);
        Self {
            TokenId: f_TokenId,
            AuthenticationId: f_AuthenticationId,
            ModifiedId: f_ModifiedId,
            TokenSource: f_TokenSource,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 40);
        FromIntoMemory::into_bytes(self.TokenId, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.AuthenticationId, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.ModifiedId, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.TokenSource, &mut into[24..24 + 16]);
    }
    fn size() -> usize {
        40
    }
}
pub struct TOKEN_DEFAULT_DACL {
    pub DefaultDacl: MutPtr<ACL>,
}
impl ::core::marker::Copy for TOKEN_DEFAULT_DACL {}
impl ::core::clone::Clone for TOKEN_DEFAULT_DACL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_DEFAULT_DACL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_DEFAULT_DACL")
            .field("DefaultDacl", &self.DefaultDacl)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_DEFAULT_DACL {
    fn eq(&self, other: &Self) -> bool {
        self.DefaultDacl == other.DefaultDacl
    }
}
impl ::core::cmp::Eq for TOKEN_DEFAULT_DACL {}
impl FromIntoMemory for TOKEN_DEFAULT_DACL {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_DefaultDacl = <MutPtr<ACL> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            DefaultDacl: f_DefaultDacl,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.DefaultDacl, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct TOKEN_DEVICE_CLAIMS {
    pub DeviceClaims: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for TOKEN_DEVICE_CLAIMS {}
impl ::core::clone::Clone for TOKEN_DEVICE_CLAIMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_DEVICE_CLAIMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_DEVICE_CLAIMS")
            .field("DeviceClaims", &self.DeviceClaims)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_DEVICE_CLAIMS {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceClaims == other.DeviceClaims
    }
}
impl ::core::cmp::Eq for TOKEN_DEVICE_CLAIMS {}
impl FromIntoMemory for TOKEN_DEVICE_CLAIMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_DeviceClaims =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            DeviceClaims: f_DeviceClaims,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.DeviceClaims, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct TOKEN_ELEVATION {
    pub TokenIsElevated: u32,
}
impl ::core::marker::Copy for TOKEN_ELEVATION {}
impl ::core::clone::Clone for TOKEN_ELEVATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_ELEVATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_ELEVATION")
            .field("TokenIsElevated", &self.TokenIsElevated)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_ELEVATION {
    fn eq(&self, other: &Self) -> bool {
        self.TokenIsElevated == other.TokenIsElevated
    }
}
impl ::core::cmp::Eq for TOKEN_ELEVATION {}
impl FromIntoMemory for TOKEN_ELEVATION {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_TokenIsElevated = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            TokenIsElevated: f_TokenIsElevated,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.TokenIsElevated, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TOKEN_ELEVATION_TYPE(pub i32);
pub const TokenElevationTypeDefault: TOKEN_ELEVATION_TYPE = TOKEN_ELEVATION_TYPE(1i32);
pub const TokenElevationTypeFull: TOKEN_ELEVATION_TYPE = TOKEN_ELEVATION_TYPE(2i32);
pub const TokenElevationTypeLimited: TOKEN_ELEVATION_TYPE = TOKEN_ELEVATION_TYPE(3i32);
impl ::core::marker::Copy for TOKEN_ELEVATION_TYPE {}
impl ::core::clone::Clone for TOKEN_ELEVATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOKEN_ELEVATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOKEN_ELEVATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKEN_ELEVATION_TYPE")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for TOKEN_ELEVATION_TYPE {
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
pub struct TOKEN_GROUPS {
    pub GroupCount: u32,
    pub Groups: [SID_AND_ATTRIBUTES; 1],
}
impl ::core::marker::Copy for TOKEN_GROUPS {}
impl ::core::clone::Clone for TOKEN_GROUPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_GROUPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_GROUPS")
            .field("GroupCount", &self.GroupCount)
            .field("Groups", &self.Groups)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_GROUPS {
    fn eq(&self, other: &Self) -> bool {
        self.GroupCount == other.GroupCount && self.Groups == other.Groups
    }
}
impl ::core::cmp::Eq for TOKEN_GROUPS {}
impl FromIntoMemory for TOKEN_GROUPS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 12);
        let f_GroupCount = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Groups = <[SID_AND_ATTRIBUTES; 1] as FromIntoMemory>::from_bytes(&from[4..4 + 8]);
        Self {
            GroupCount: f_GroupCount,
            Groups: f_Groups,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 12);
        FromIntoMemory::into_bytes(self.GroupCount, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Groups, &mut into[4..4 + 8]);
    }
    fn size() -> usize {
        12
    }
}
pub struct TOKEN_GROUPS_AND_PRIVILEGES {
    pub SidCount: u32,
    pub SidLength: u32,
    pub Sids: MutPtr<SID_AND_ATTRIBUTES>,
    pub RestrictedSidCount: u32,
    pub RestrictedSidLength: u32,
    pub RestrictedSids: MutPtr<SID_AND_ATTRIBUTES>,
    pub PrivilegeCount: u32,
    pub PrivilegeLength: u32,
    pub Privileges: MutPtr<LUID_AND_ATTRIBUTES>,
    pub AuthenticationId: super::Foundation::LUID,
}
impl ::core::marker::Copy for TOKEN_GROUPS_AND_PRIVILEGES {}
impl ::core::clone::Clone for TOKEN_GROUPS_AND_PRIVILEGES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_GROUPS_AND_PRIVILEGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_GROUPS_AND_PRIVILEGES")
            .field("SidCount", &self.SidCount)
            .field("SidLength", &self.SidLength)
            .field("Sids", &self.Sids)
            .field("RestrictedSidCount", &self.RestrictedSidCount)
            .field("RestrictedSidLength", &self.RestrictedSidLength)
            .field("RestrictedSids", &self.RestrictedSids)
            .field("PrivilegeCount", &self.PrivilegeCount)
            .field("PrivilegeLength", &self.PrivilegeLength)
            .field("Privileges", &self.Privileges)
            .field("AuthenticationId", &self.AuthenticationId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_GROUPS_AND_PRIVILEGES {
    fn eq(&self, other: &Self) -> bool {
        self.SidCount == other.SidCount
            && self.SidLength == other.SidLength
            && self.Sids == other.Sids
            && self.RestrictedSidCount == other.RestrictedSidCount
            && self.RestrictedSidLength == other.RestrictedSidLength
            && self.RestrictedSids == other.RestrictedSids
            && self.PrivilegeCount == other.PrivilegeCount
            && self.PrivilegeLength == other.PrivilegeLength
            && self.Privileges == other.Privileges
            && self.AuthenticationId == other.AuthenticationId
    }
}
impl ::core::cmp::Eq for TOKEN_GROUPS_AND_PRIVILEGES {}
impl FromIntoMemory for TOKEN_GROUPS_AND_PRIVILEGES {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 44);
        let f_SidCount = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_SidLength = <u32 as FromIntoMemory>::from_bytes(&from[4..4 + 4]);
        let f_Sids = <MutPtr<SID_AND_ATTRIBUTES> as FromIntoMemory>::from_bytes(&from[8..8 + 4]);
        let f_RestrictedSidCount = <u32 as FromIntoMemory>::from_bytes(&from[12..12 + 4]);
        let f_RestrictedSidLength = <u32 as FromIntoMemory>::from_bytes(&from[16..16 + 4]);
        let f_RestrictedSids =
            <MutPtr<SID_AND_ATTRIBUTES> as FromIntoMemory>::from_bytes(&from[20..20 + 4]);
        let f_PrivilegeCount = <u32 as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_PrivilegeLength = <u32 as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_Privileges =
            <MutPtr<LUID_AND_ATTRIBUTES> as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_AuthenticationId =
            <super::Foundation::LUID as FromIntoMemory>::from_bytes(&from[36..36 + 8]);
        Self {
            SidCount: f_SidCount,
            SidLength: f_SidLength,
            Sids: f_Sids,
            RestrictedSidCount: f_RestrictedSidCount,
            RestrictedSidLength: f_RestrictedSidLength,
            RestrictedSids: f_RestrictedSids,
            PrivilegeCount: f_PrivilegeCount,
            PrivilegeLength: f_PrivilegeLength,
            Privileges: f_Privileges,
            AuthenticationId: f_AuthenticationId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 44);
        FromIntoMemory::into_bytes(self.SidCount, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.SidLength, &mut into[4..4 + 4]);
        FromIntoMemory::into_bytes(self.Sids, &mut into[8..8 + 4]);
        FromIntoMemory::into_bytes(self.RestrictedSidCount, &mut into[12..12 + 4]);
        FromIntoMemory::into_bytes(self.RestrictedSidLength, &mut into[16..16 + 4]);
        FromIntoMemory::into_bytes(self.RestrictedSids, &mut into[20..20 + 4]);
        FromIntoMemory::into_bytes(self.PrivilegeCount, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.PrivilegeLength, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.Privileges, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.AuthenticationId, &mut into[36..36 + 8]);
    }
    fn size() -> usize {
        44
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TOKEN_INFORMATION_CLASS(pub i32);
pub const TokenUser: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(1i32);
pub const TokenGroups: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(2i32);
pub const TokenPrivileges: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(3i32);
pub const TokenOwner: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(4i32);
pub const TokenPrimaryGroup: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(5i32);
pub const TokenDefaultDacl: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(6i32);
pub const TokenSource: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(7i32);
pub const TokenType: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(8i32);
pub const TokenImpersonationLevel: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(9i32);
pub const TokenStatistics: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(10i32);
pub const TokenRestrictedSids: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(11i32);
pub const TokenSessionId: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(12i32);
pub const TokenGroupsAndPrivileges: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(13i32);
pub const TokenSessionReference: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(14i32);
pub const TokenSandBoxInert: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(15i32);
pub const TokenAuditPolicy: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(16i32);
pub const TokenOrigin: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(17i32);
pub const TokenElevationType: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(18i32);
pub const TokenLinkedToken: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(19i32);
pub const TokenElevation: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(20i32);
pub const TokenHasRestrictions: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(21i32);
pub const TokenAccessInformation: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(22i32);
pub const TokenVirtualizationAllowed: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(23i32);
pub const TokenVirtualizationEnabled: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(24i32);
pub const TokenIntegrityLevel: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(25i32);
pub const TokenUIAccess: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(26i32);
pub const TokenMandatoryPolicy: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(27i32);
pub const TokenLogonSid: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(28i32);
pub const TokenIsAppContainer: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(29i32);
pub const TokenCapabilities: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(30i32);
pub const TokenAppContainerSid: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(31i32);
pub const TokenAppContainerNumber: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(32i32);
pub const TokenUserClaimAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(33i32);
pub const TokenDeviceClaimAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(34i32);
pub const TokenRestrictedUserClaimAttributes: TOKEN_INFORMATION_CLASS =
    TOKEN_INFORMATION_CLASS(35i32);
pub const TokenRestrictedDeviceClaimAttributes: TOKEN_INFORMATION_CLASS =
    TOKEN_INFORMATION_CLASS(36i32);
pub const TokenDeviceGroups: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(37i32);
pub const TokenRestrictedDeviceGroups: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(38i32);
pub const TokenSecurityAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(39i32);
pub const TokenIsRestricted: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(40i32);
pub const TokenProcessTrustLevel: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(41i32);
pub const TokenPrivateNameSpace: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(42i32);
pub const TokenSingletonAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(43i32);
pub const TokenBnoIsolation: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(44i32);
pub const TokenChildProcessFlags: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(45i32);
pub const TokenIsLessPrivilegedAppContainer: TOKEN_INFORMATION_CLASS =
    TOKEN_INFORMATION_CLASS(46i32);
pub const TokenIsSandboxed: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(47i32);
pub const MaxTokenInfoClass: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(48i32);
impl ::core::marker::Copy for TOKEN_INFORMATION_CLASS {}
impl ::core::clone::Clone for TOKEN_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOKEN_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOKEN_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKEN_INFORMATION_CLASS")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for TOKEN_INFORMATION_CLASS {
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
pub struct TOKEN_LINKED_TOKEN {
    pub LinkedToken: super::Foundation::HANDLE,
}
impl ::core::marker::Copy for TOKEN_LINKED_TOKEN {}
impl ::core::clone::Clone for TOKEN_LINKED_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_LINKED_TOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_LINKED_TOKEN")
            .field("LinkedToken", &self.LinkedToken)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_LINKED_TOKEN {
    fn eq(&self, other: &Self) -> bool {
        self.LinkedToken == other.LinkedToken
    }
}
impl ::core::cmp::Eq for TOKEN_LINKED_TOKEN {}
impl FromIntoMemory for TOKEN_LINKED_TOKEN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_LinkedToken =
            <super::Foundation::HANDLE as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            LinkedToken: f_LinkedToken,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.LinkedToken, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct TOKEN_MANDATORY_LABEL {
    pub Label: SID_AND_ATTRIBUTES,
}
impl ::core::marker::Copy for TOKEN_MANDATORY_LABEL {}
impl ::core::clone::Clone for TOKEN_MANDATORY_LABEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_MANDATORY_LABEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_MANDATORY_LABEL")
            .field("Label", &self.Label)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_MANDATORY_LABEL {
    fn eq(&self, other: &Self) -> bool {
        self.Label == other.Label
    }
}
impl ::core::cmp::Eq for TOKEN_MANDATORY_LABEL {}
impl FromIntoMemory for TOKEN_MANDATORY_LABEL {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_Label = <SID_AND_ATTRIBUTES as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self { Label: f_Label }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.Label, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
pub struct TOKEN_MANDATORY_POLICY {
    pub Policy: TOKEN_MANDATORY_POLICY_ID,
}
impl ::core::marker::Copy for TOKEN_MANDATORY_POLICY {}
impl ::core::clone::Clone for TOKEN_MANDATORY_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_MANDATORY_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_MANDATORY_POLICY")
            .field("Policy", &self.Policy)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_MANDATORY_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Policy == other.Policy
    }
}
impl ::core::cmp::Eq for TOKEN_MANDATORY_POLICY {}
impl FromIntoMemory for TOKEN_MANDATORY_POLICY {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Policy = <TOKEN_MANDATORY_POLICY_ID as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { Policy: f_Policy }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Policy, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TOKEN_MANDATORY_POLICY_ID(pub u32);
pub const TOKEN_MANDATORY_POLICY_OFF: TOKEN_MANDATORY_POLICY_ID = TOKEN_MANDATORY_POLICY_ID(0u32);
pub const TOKEN_MANDATORY_POLICY_NO_WRITE_UP: TOKEN_MANDATORY_POLICY_ID =
    TOKEN_MANDATORY_POLICY_ID(1u32);
pub const TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN: TOKEN_MANDATORY_POLICY_ID =
    TOKEN_MANDATORY_POLICY_ID(2u32);
pub const TOKEN_MANDATORY_POLICY_VALID_MASK: TOKEN_MANDATORY_POLICY_ID =
    TOKEN_MANDATORY_POLICY_ID(3u32);
impl ::core::marker::Copy for TOKEN_MANDATORY_POLICY_ID {}
impl ::core::clone::Clone for TOKEN_MANDATORY_POLICY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOKEN_MANDATORY_POLICY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOKEN_MANDATORY_POLICY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKEN_MANDATORY_POLICY_ID")
            .field(&self.0)
            .finish()
    }
}
impl FromIntoMemory for TOKEN_MANDATORY_POLICY_ID {
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
pub struct TOKEN_ORIGIN {
    pub OriginatingLogonSession: super::Foundation::LUID,
}
impl ::core::marker::Copy for TOKEN_ORIGIN {}
impl ::core::clone::Clone for TOKEN_ORIGIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_ORIGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_ORIGIN")
            .field("OriginatingLogonSession", &self.OriginatingLogonSession)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_ORIGIN {
    fn eq(&self, other: &Self) -> bool {
        self.OriginatingLogonSession == other.OriginatingLogonSession
    }
}
impl ::core::cmp::Eq for TOKEN_ORIGIN {}
impl FromIntoMemory for TOKEN_ORIGIN {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_OriginatingLogonSession =
            <super::Foundation::LUID as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self {
            OriginatingLogonSession: f_OriginatingLogonSession,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.OriginatingLogonSession, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
pub struct TOKEN_OWNER {
    pub Owner: super::Foundation::PSID,
}
impl ::core::marker::Copy for TOKEN_OWNER {}
impl ::core::clone::Clone for TOKEN_OWNER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_OWNER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_OWNER")
            .field("Owner", &self.Owner)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_OWNER {
    fn eq(&self, other: &Self) -> bool {
        self.Owner == other.Owner
    }
}
impl ::core::cmp::Eq for TOKEN_OWNER {}
impl FromIntoMemory for TOKEN_OWNER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_Owner = <super::Foundation::PSID as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self { Owner: f_Owner }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.Owner, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct TOKEN_PRIMARY_GROUP {
    pub PrimaryGroup: super::Foundation::PSID,
}
impl ::core::marker::Copy for TOKEN_PRIMARY_GROUP {}
impl ::core::clone::Clone for TOKEN_PRIMARY_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_PRIMARY_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_PRIMARY_GROUP")
            .field("PrimaryGroup", &self.PrimaryGroup)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_PRIMARY_GROUP {
    fn eq(&self, other: &Self) -> bool {
        self.PrimaryGroup == other.PrimaryGroup
    }
}
impl ::core::cmp::Eq for TOKEN_PRIMARY_GROUP {}
impl FromIntoMemory for TOKEN_PRIMARY_GROUP {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_PrimaryGroup =
            <super::Foundation::PSID as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            PrimaryGroup: f_PrimaryGroup,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.PrimaryGroup, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
pub struct TOKEN_PRIVILEGES {
    pub PrivilegeCount: u32,
    pub Privileges: [LUID_AND_ATTRIBUTES; 1],
}
impl ::core::marker::Copy for TOKEN_PRIVILEGES {}
impl ::core::clone::Clone for TOKEN_PRIVILEGES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_PRIVILEGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_PRIVILEGES")
            .field("PrivilegeCount", &self.PrivilegeCount)
            .field("Privileges", &self.Privileges)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_PRIVILEGES {
    fn eq(&self, other: &Self) -> bool {
        self.PrivilegeCount == other.PrivilegeCount && self.Privileges == other.Privileges
    }
}
impl ::core::cmp::Eq for TOKEN_PRIVILEGES {}
impl FromIntoMemory for TOKEN_PRIVILEGES {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_PrivilegeCount = <u32 as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        let f_Privileges =
            <[LUID_AND_ATTRIBUTES; 1] as FromIntoMemory>::from_bytes(&from[4..4 + 12]);
        Self {
            PrivilegeCount: f_PrivilegeCount,
            Privileges: f_Privileges,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.PrivilegeCount, &mut into[0..0 + 4]);
        FromIntoMemory::into_bytes(self.Privileges, &mut into[4..4 + 12]);
    }
    fn size() -> usize {
        16
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TOKEN_PRIVILEGES_ATTRIBUTES(pub u32);
pub const SE_PRIVILEGE_ENABLED: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(2u32);
pub const SE_PRIVILEGE_ENABLED_BY_DEFAULT: TOKEN_PRIVILEGES_ATTRIBUTES =
    TOKEN_PRIVILEGES_ATTRIBUTES(1u32);
pub const SE_PRIVILEGE_REMOVED: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(4u32);
pub const SE_PRIVILEGE_USED_FOR_ACCESS: TOKEN_PRIVILEGES_ATTRIBUTES =
    TOKEN_PRIVILEGES_ATTRIBUTES(2147483648u32);
impl ::core::marker::Copy for TOKEN_PRIVILEGES_ATTRIBUTES {}
impl ::core::clone::Clone for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKEN_PRIVILEGES_ATTRIBUTES")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for TOKEN_PRIVILEGES_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TOKEN_PRIVILEGES_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TOKEN_PRIVILEGES_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for TOKEN_PRIVILEGES_ATTRIBUTES {
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
pub struct TOKEN_SOURCE {
    pub SourceName: [super::Foundation::CHAR; 8],
    pub SourceIdentifier: super::Foundation::LUID,
}
impl ::core::marker::Copy for TOKEN_SOURCE {}
impl ::core::clone::Clone for TOKEN_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_SOURCE")
            .field("SourceName", &self.SourceName)
            .field("SourceIdentifier", &self.SourceIdentifier)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_SOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.SourceName == other.SourceName && self.SourceIdentifier == other.SourceIdentifier
    }
}
impl ::core::cmp::Eq for TOKEN_SOURCE {}
impl FromIntoMemory for TOKEN_SOURCE {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 16);
        let f_SourceName =
            <[super::Foundation::CHAR; 8] as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_SourceIdentifier =
            <super::Foundation::LUID as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        Self {
            SourceName: f_SourceName,
            SourceIdentifier: f_SourceIdentifier,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 16);
        FromIntoMemory::into_bytes(self.SourceName, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.SourceIdentifier, &mut into[8..8 + 8]);
    }
    fn size() -> usize {
        16
    }
}
pub struct TOKEN_STATISTICS {
    pub TokenId: super::Foundation::LUID,
    pub AuthenticationId: super::Foundation::LUID,
    pub ExpirationTime: i64,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub DynamicCharged: u32,
    pub DynamicAvailable: u32,
    pub GroupCount: u32,
    pub PrivilegeCount: u32,
    pub ModifiedId: super::Foundation::LUID,
}
impl ::core::marker::Copy for TOKEN_STATISTICS {}
impl ::core::clone::Clone for TOKEN_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_STATISTICS")
            .field("TokenId", &self.TokenId)
            .field("AuthenticationId", &self.AuthenticationId)
            .field("ExpirationTime", &self.ExpirationTime)
            .field("TokenType", &self.TokenType)
            .field("ImpersonationLevel", &self.ImpersonationLevel)
            .field("DynamicCharged", &self.DynamicCharged)
            .field("DynamicAvailable", &self.DynamicAvailable)
            .field("GroupCount", &self.GroupCount)
            .field("PrivilegeCount", &self.PrivilegeCount)
            .field("ModifiedId", &self.ModifiedId)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.TokenId == other.TokenId
            && self.AuthenticationId == other.AuthenticationId
            && self.ExpirationTime == other.ExpirationTime
            && self.TokenType == other.TokenType
            && self.ImpersonationLevel == other.ImpersonationLevel
            && self.DynamicCharged == other.DynamicCharged
            && self.DynamicAvailable == other.DynamicAvailable
            && self.GroupCount == other.GroupCount
            && self.PrivilegeCount == other.PrivilegeCount
            && self.ModifiedId == other.ModifiedId
    }
}
impl ::core::cmp::Eq for TOKEN_STATISTICS {}
impl FromIntoMemory for TOKEN_STATISTICS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 56);
        let f_TokenId = <super::Foundation::LUID as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        let f_AuthenticationId =
            <super::Foundation::LUID as FromIntoMemory>::from_bytes(&from[8..8 + 8]);
        let f_ExpirationTime = <i64 as FromIntoMemory>::from_bytes(&from[16..16 + 8]);
        let f_TokenType = <TOKEN_TYPE as FromIntoMemory>::from_bytes(&from[24..24 + 4]);
        let f_ImpersonationLevel =
            <SECURITY_IMPERSONATION_LEVEL as FromIntoMemory>::from_bytes(&from[28..28 + 4]);
        let f_DynamicCharged = <u32 as FromIntoMemory>::from_bytes(&from[32..32 + 4]);
        let f_DynamicAvailable = <u32 as FromIntoMemory>::from_bytes(&from[36..36 + 4]);
        let f_GroupCount = <u32 as FromIntoMemory>::from_bytes(&from[40..40 + 4]);
        let f_PrivilegeCount = <u32 as FromIntoMemory>::from_bytes(&from[44..44 + 4]);
        let f_ModifiedId =
            <super::Foundation::LUID as FromIntoMemory>::from_bytes(&from[48..48 + 8]);
        Self {
            TokenId: f_TokenId,
            AuthenticationId: f_AuthenticationId,
            ExpirationTime: f_ExpirationTime,
            TokenType: f_TokenType,
            ImpersonationLevel: f_ImpersonationLevel,
            DynamicCharged: f_DynamicCharged,
            DynamicAvailable: f_DynamicAvailable,
            GroupCount: f_GroupCount,
            PrivilegeCount: f_PrivilegeCount,
            ModifiedId: f_ModifiedId,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 56);
        FromIntoMemory::into_bytes(self.TokenId, &mut into[0..0 + 8]);
        FromIntoMemory::into_bytes(self.AuthenticationId, &mut into[8..8 + 8]);
        FromIntoMemory::into_bytes(self.ExpirationTime, &mut into[16..16 + 8]);
        FromIntoMemory::into_bytes(self.TokenType, &mut into[24..24 + 4]);
        FromIntoMemory::into_bytes(self.ImpersonationLevel, &mut into[28..28 + 4]);
        FromIntoMemory::into_bytes(self.DynamicCharged, &mut into[32..32 + 4]);
        FromIntoMemory::into_bytes(self.DynamicAvailable, &mut into[36..36 + 4]);
        FromIntoMemory::into_bytes(self.GroupCount, &mut into[40..40 + 4]);
        FromIntoMemory::into_bytes(self.PrivilegeCount, &mut into[44..44 + 4]);
        FromIntoMemory::into_bytes(self.ModifiedId, &mut into[48..48 + 8]);
    }
    fn size() -> usize {
        56
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TOKEN_TYPE(pub i32);
pub const TokenPrimary: TOKEN_TYPE = TOKEN_TYPE(1i32);
pub const TokenImpersonation: TOKEN_TYPE = TOKEN_TYPE(2i32);
impl ::core::marker::Copy for TOKEN_TYPE {}
impl ::core::clone::Clone for TOKEN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOKEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOKEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOKEN_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for TOKEN_TYPE {
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
pub struct TOKEN_USER {
    pub User: SID_AND_ATTRIBUTES,
}
impl ::core::marker::Copy for TOKEN_USER {}
impl ::core::clone::Clone for TOKEN_USER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_USER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_USER")
            .field("User", &self.User)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_USER {
    fn eq(&self, other: &Self) -> bool {
        self.User == other.User
    }
}
impl ::core::cmp::Eq for TOKEN_USER {}
impl FromIntoMemory for TOKEN_USER {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 8);
        let f_User = <SID_AND_ATTRIBUTES as FromIntoMemory>::from_bytes(&from[0..0 + 8]);
        Self { User: f_User }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 8);
        FromIntoMemory::into_bytes(self.User, &mut into[0..0 + 8]);
    }
    fn size() -> usize {
        8
    }
}
pub struct TOKEN_USER_CLAIMS {
    pub UserClaims: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for TOKEN_USER_CLAIMS {}
impl ::core::clone::Clone for TOKEN_USER_CLAIMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOKEN_USER_CLAIMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOKEN_USER_CLAIMS")
            .field("UserClaims", &self.UserClaims)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TOKEN_USER_CLAIMS {
    fn eq(&self, other: &Self) -> bool {
        self.UserClaims == other.UserClaims
    }
}
impl ::core::cmp::Eq for TOKEN_USER_CLAIMS {}
impl FromIntoMemory for TOKEN_USER_CLAIMS {
    fn from_bytes(from: &[u8]) -> Self {
        assert_eq!(from.len(), 4);
        let f_UserClaims =
            <MutPtr<::core::ffi::c_void> as FromIntoMemory>::from_bytes(&from[0..0 + 4]);
        Self {
            UserClaims: f_UserClaims,
        }
    }
    fn into_bytes(self, into: &mut [u8]) {
        assert_eq!(into.len(), 4);
        FromIntoMemory::into_bytes(self.UserClaims, &mut into[0..0 + 4]);
    }
    fn size() -> usize {
        4
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WELL_KNOWN_SID_TYPE(pub i32);
pub const WinNullSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(0i32);
pub const WinWorldSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(1i32);
pub const WinLocalSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(2i32);
pub const WinCreatorOwnerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(3i32);
pub const WinCreatorGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(4i32);
pub const WinCreatorOwnerServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(5i32);
pub const WinCreatorGroupServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(6i32);
pub const WinNtAuthoritySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(7i32);
pub const WinDialupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(8i32);
pub const WinNetworkSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(9i32);
pub const WinBatchSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(10i32);
pub const WinInteractiveSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(11i32);
pub const WinServiceSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(12i32);
pub const WinAnonymousSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(13i32);
pub const WinProxySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(14i32);
pub const WinEnterpriseControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(15i32);
pub const WinSelfSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(16i32);
pub const WinAuthenticatedUserSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(17i32);
pub const WinRestrictedCodeSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(18i32);
pub const WinTerminalServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(19i32);
pub const WinRemoteLogonIdSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(20i32);
pub const WinLogonIdsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(21i32);
pub const WinLocalSystemSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(22i32);
pub const WinLocalServiceSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(23i32);
pub const WinNetworkServiceSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(24i32);
pub const WinBuiltinDomainSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(25i32);
pub const WinBuiltinAdministratorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(26i32);
pub const WinBuiltinUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(27i32);
pub const WinBuiltinGuestsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(28i32);
pub const WinBuiltinPowerUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(29i32);
pub const WinBuiltinAccountOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(30i32);
pub const WinBuiltinSystemOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(31i32);
pub const WinBuiltinPrintOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(32i32);
pub const WinBuiltinBackupOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(33i32);
pub const WinBuiltinReplicatorSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(34i32);
pub const WinBuiltinPreWindows2000CompatibleAccessSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(35i32);
pub const WinBuiltinRemoteDesktopUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(36i32);
pub const WinBuiltinNetworkConfigurationOperatorsSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(37i32);
pub const WinAccountAdministratorSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(38i32);
pub const WinAccountGuestSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(39i32);
pub const WinAccountKrbtgtSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(40i32);
pub const WinAccountDomainAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(41i32);
pub const WinAccountDomainUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(42i32);
pub const WinAccountDomainGuestsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(43i32);
pub const WinAccountComputersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(44i32);
pub const WinAccountControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(45i32);
pub const WinAccountCertAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(46i32);
pub const WinAccountSchemaAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(47i32);
pub const WinAccountEnterpriseAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(48i32);
pub const WinAccountPolicyAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(49i32);
pub const WinAccountRasAndIasServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(50i32);
pub const WinNTLMAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(51i32);
pub const WinDigestAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(52i32);
pub const WinSChannelAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(53i32);
pub const WinThisOrganizationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(54i32);
pub const WinOtherOrganizationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(55i32);
pub const WinBuiltinIncomingForestTrustBuildersSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(56i32);
pub const WinBuiltinPerfMonitoringUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(57i32);
pub const WinBuiltinPerfLoggingUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(58i32);
pub const WinBuiltinAuthorizationAccessSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(59i32);
pub const WinBuiltinTerminalServerLicenseServersSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(60i32);
pub const WinBuiltinDCOMUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(61i32);
pub const WinBuiltinIUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(62i32);
pub const WinIUserSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(63i32);
pub const WinBuiltinCryptoOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(64i32);
pub const WinUntrustedLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(65i32);
pub const WinLowLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(66i32);
pub const WinMediumLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(67i32);
pub const WinHighLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(68i32);
pub const WinSystemLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(69i32);
pub const WinWriteRestrictedCodeSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(70i32);
pub const WinCreatorOwnerRightsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(71i32);
pub const WinCacheablePrincipalsGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(72i32);
pub const WinNonCacheablePrincipalsGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(73i32);
pub const WinEnterpriseReadonlyControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(74i32);
pub const WinAccountReadonlyControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(75i32);
pub const WinBuiltinEventLogReadersGroup: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(76i32);
pub const WinNewEnterpriseReadonlyControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(77i32);
pub const WinBuiltinCertSvcDComAccessGroup: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(78i32);
pub const WinMediumPlusLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(79i32);
pub const WinLocalLogonSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(80i32);
pub const WinConsoleLogonSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(81i32);
pub const WinThisOrganizationCertificateSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(82i32);
pub const WinApplicationPackageAuthoritySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(83i32);
pub const WinBuiltinAnyPackageSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(84i32);
pub const WinCapabilityInternetClientSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(85i32);
pub const WinCapabilityInternetClientServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(86i32);
pub const WinCapabilityPrivateNetworkClientServerSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(87i32);
pub const WinCapabilityPicturesLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(88i32);
pub const WinCapabilityVideosLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(89i32);
pub const WinCapabilityMusicLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(90i32);
pub const WinCapabilityDocumentsLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(91i32);
pub const WinCapabilitySharedUserCertificatesSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(92i32);
pub const WinCapabilityEnterpriseAuthenticationSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(93i32);
pub const WinCapabilityRemovableStorageSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(94i32);
pub const WinBuiltinRDSRemoteAccessServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(95i32);
pub const WinBuiltinRDSEndpointServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(96i32);
pub const WinBuiltinRDSManagementServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(97i32);
pub const WinUserModeDriversSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(98i32);
pub const WinBuiltinHyperVAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(99i32);
pub const WinAccountCloneableControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(100i32);
pub const WinBuiltinAccessControlAssistanceOperatorsSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(101i32);
pub const WinBuiltinRemoteManagementUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(102i32);
pub const WinAuthenticationAuthorityAssertedSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(103i32);
pub const WinAuthenticationServiceAssertedSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(104i32);
pub const WinLocalAccountSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(105i32);
pub const WinLocalAccountAndAdministratorSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(106i32);
pub const WinAccountProtectedUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(107i32);
pub const WinCapabilityAppointmentsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(108i32);
pub const WinCapabilityContactsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(109i32);
pub const WinAccountDefaultSystemManagedSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(110i32);
pub const WinBuiltinDefaultSystemManagedGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(111i32);
pub const WinBuiltinStorageReplicaAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(112i32);
pub const WinAccountKeyAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(113i32);
pub const WinAccountEnterpriseKeyAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(114i32);
pub const WinAuthenticationKeyTrustSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(115i32);
pub const WinAuthenticationKeyPropertyMFASid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(116i32);
pub const WinAuthenticationKeyPropertyAttestationSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(117i32);
pub const WinAuthenticationFreshKeyAuthSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(118i32);
pub const WinBuiltinDeviceOwnersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(119i32);
impl ::core::marker::Copy for WELL_KNOWN_SID_TYPE {}
impl ::core::clone::Clone for WELL_KNOWN_SID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WELL_KNOWN_SID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WELL_KNOWN_SID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WELL_KNOWN_SID_TYPE").field(&self.0).finish()
    }
}
impl FromIntoMemory for WELL_KNOWN_SID_TYPE {
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
pub const cwcFILENAMESUFFIXMAX: u32 = 20u32;
pub const cwcHRESULTSTRING: u32 = 40u32;
pub const szLBRACE: &'static str = "{";
pub const szLPAREN: &'static str = "(";
pub const szRBRACE: &'static str = "}";
pub const szRPAREN: &'static str = ")";
pub const wszCERTENROLLSHAREPATH: &'static str = "CertSrv\\CertEnroll";
pub const wszFCSAPARM_CERTFILENAMESUFFIX: &'static str = "%4";
pub const wszFCSAPARM_CONFIGDN: &'static str = "%6";
pub const wszFCSAPARM_CRLDELTAFILENAMESUFFIX: &'static str = "%9";
pub const wszFCSAPARM_CRLFILENAMESUFFIX: &'static str = "%8";
pub const wszFCSAPARM_DOMAINDN: &'static str = "%5";
pub const wszFCSAPARM_DSCACERTATTRIBUTE: &'static str = "%11";
pub const wszFCSAPARM_DSCRLATTRIBUTE: &'static str = "%10";
pub const wszFCSAPARM_DSCROSSCERTPAIRATTRIBUTE: &'static str = "%14";
pub const wszFCSAPARM_DSKRACERTATTRIBUTE: &'static str = "%13";
pub const wszFCSAPARM_DSUSERCERTATTRIBUTE: &'static str = "%12";
pub const wszFCSAPARM_SANITIZEDCANAME: &'static str = "%3";
pub const wszFCSAPARM_SANITIZEDCANAMEHASH: &'static str = "%7";
pub const wszFCSAPARM_SERVERDNSNAME: &'static str = "%1";
pub const wszFCSAPARM_SERVERSHORTNAME: &'static str = "%2";
pub const wszLBRACE: &'static str = "{";
pub const wszLPAREN: &'static str = "(";
pub const wszRBRACE: &'static str = "}";
pub const wszRPAREN: &'static str = ")";
pub trait Api {
    #[doc = "AccessCheck from ADVAPI32"]
    fn AccessCheck(
        &self,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        client_token: super::Foundation::HANDLE,
        desired_access: u32,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
        privilege_set: MutPtr<PRIVILEGE_SET>,
        privilege_set_length: MutPtr<u32>,
        granted_access: MutPtr<u32>,
        access_status: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("AccessCheck")
    }
    #[doc = "AccessCheckAndAuditAlarmA from ADVAPI32"]
    fn AccessCheckAndAuditAlarmA(
        &self,
        subsystem_name: PCSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        object_type_name: PCSTR,
        object_name: PCSTR,
        security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        desired_access: u32,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
        object_creation: super::Foundation::BOOL,
        granted_access: MutPtr<u32>,
        access_status: MutPtr<i32>,
        pf_generate_on_close: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("AccessCheckAndAuditAlarmA")
    }
    #[doc = "AccessCheckAndAuditAlarmW from ADVAPI32"]
    fn AccessCheckAndAuditAlarmW(
        &self,
        subsystem_name: PCWSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        object_type_name: PCWSTR,
        object_name: PCWSTR,
        security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        desired_access: u32,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
        object_creation: super::Foundation::BOOL,
        granted_access: MutPtr<u32>,
        access_status: MutPtr<i32>,
        pf_generate_on_close: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("AccessCheckAndAuditAlarmW")
    }
    #[doc = "AccessCheckByType from ADVAPI32"]
    fn AccessCheckByType(
        &self,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        principal_self_sid: super::Foundation::PSID,
        client_token: super::Foundation::HANDLE,
        desired_access: u32,
        object_type_list: MutPtr<OBJECT_TYPE_LIST>,
        object_type_list_length: u32,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
        privilege_set: MutPtr<PRIVILEGE_SET>,
        privilege_set_length: MutPtr<u32>,
        granted_access: MutPtr<u32>,
        access_status: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("AccessCheckByType")
    }
    #[doc = "AccessCheckByTypeAndAuditAlarmA from ADVAPI32"]
    fn AccessCheckByTypeAndAuditAlarmA(
        &self,
        subsystem_name: PCSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        object_type_name: PCSTR,
        object_name: PCSTR,
        security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        principal_self_sid: super::Foundation::PSID,
        desired_access: u32,
        audit_type: AUDIT_EVENT_TYPE,
        flags: u32,
        object_type_list: MutPtr<OBJECT_TYPE_LIST>,
        object_type_list_length: u32,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
        object_creation: super::Foundation::BOOL,
        granted_access: MutPtr<u32>,
        access_status: MutPtr<i32>,
        pf_generate_on_close: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("AccessCheckByTypeAndAuditAlarmA")
    }
    #[doc = "AccessCheckByTypeAndAuditAlarmW from ADVAPI32"]
    fn AccessCheckByTypeAndAuditAlarmW(
        &self,
        subsystem_name: PCWSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        object_type_name: PCWSTR,
        object_name: PCWSTR,
        security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        principal_self_sid: super::Foundation::PSID,
        desired_access: u32,
        audit_type: AUDIT_EVENT_TYPE,
        flags: u32,
        object_type_list: MutPtr<OBJECT_TYPE_LIST>,
        object_type_list_length: u32,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
        object_creation: super::Foundation::BOOL,
        granted_access: MutPtr<u32>,
        access_status: MutPtr<i32>,
        pf_generate_on_close: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("AccessCheckByTypeAndAuditAlarmW")
    }
    #[doc = "AccessCheckByTypeResultList from ADVAPI32"]
    fn AccessCheckByTypeResultList(
        &self,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        principal_self_sid: super::Foundation::PSID,
        client_token: super::Foundation::HANDLE,
        desired_access: u32,
        object_type_list: MutPtr<OBJECT_TYPE_LIST>,
        object_type_list_length: u32,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
        privilege_set: MutPtr<PRIVILEGE_SET>,
        privilege_set_length: MutPtr<u32>,
        granted_access_list: MutPtr<u32>,
        access_status_list: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("AccessCheckByTypeResultList")
    }
    #[doc = "AccessCheckByTypeResultListAndAuditAlarmA from ADVAPI32"]
    fn AccessCheckByTypeResultListAndAuditAlarmA(
        &self,
        subsystem_name: PCSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        object_type_name: PCSTR,
        object_name: PCSTR,
        security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        principal_self_sid: super::Foundation::PSID,
        desired_access: u32,
        audit_type: AUDIT_EVENT_TYPE,
        flags: u32,
        object_type_list: MutPtr<OBJECT_TYPE_LIST>,
        object_type_list_length: u32,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
        object_creation: super::Foundation::BOOL,
        granted_access: MutPtr<u32>,
        access_status_list: MutPtr<u32>,
        pf_generate_on_close: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("AccessCheckByTypeResultListAndAuditAlarmA")
    }
    #[doc = "AccessCheckByTypeResultListAndAuditAlarmByHandleA from ADVAPI32"]
    fn AccessCheckByTypeResultListAndAuditAlarmByHandleA(
        &self,
        subsystem_name: PCSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        client_token: super::Foundation::HANDLE,
        object_type_name: PCSTR,
        object_name: PCSTR,
        security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        principal_self_sid: super::Foundation::PSID,
        desired_access: u32,
        audit_type: AUDIT_EVENT_TYPE,
        flags: u32,
        object_type_list: MutPtr<OBJECT_TYPE_LIST>,
        object_type_list_length: u32,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
        object_creation: super::Foundation::BOOL,
        granted_access: MutPtr<u32>,
        access_status_list: MutPtr<u32>,
        pf_generate_on_close: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("AccessCheckByTypeResultListAndAuditAlarmByHandleA")
    }
    #[doc = "AccessCheckByTypeResultListAndAuditAlarmByHandleW from ADVAPI32"]
    fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(
        &self,
        subsystem_name: PCWSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        client_token: super::Foundation::HANDLE,
        object_type_name: PCWSTR,
        object_name: PCWSTR,
        security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        principal_self_sid: super::Foundation::PSID,
        desired_access: u32,
        audit_type: AUDIT_EVENT_TYPE,
        flags: u32,
        object_type_list: MutPtr<OBJECT_TYPE_LIST>,
        object_type_list_length: u32,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
        object_creation: super::Foundation::BOOL,
        granted_access_list: MutPtr<u32>,
        access_status_list: MutPtr<u32>,
        pf_generate_on_close: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("AccessCheckByTypeResultListAndAuditAlarmByHandleW")
    }
    #[doc = "AccessCheckByTypeResultListAndAuditAlarmW from ADVAPI32"]
    fn AccessCheckByTypeResultListAndAuditAlarmW(
        &self,
        subsystem_name: PCWSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        object_type_name: PCWSTR,
        object_name: PCWSTR,
        security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        principal_self_sid: super::Foundation::PSID,
        desired_access: u32,
        audit_type: AUDIT_EVENT_TYPE,
        flags: u32,
        object_type_list: MutPtr<OBJECT_TYPE_LIST>,
        object_type_list_length: u32,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
        object_creation: super::Foundation::BOOL,
        granted_access_list: MutPtr<u32>,
        access_status_list: MutPtr<u32>,
        pf_generate_on_close: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("AccessCheckByTypeResultListAndAuditAlarmW")
    }
    #[doc = "AddAccessAllowedAce from ADVAPI32"]
    fn AddAccessAllowedAce(
        &self,
        p_acl: MutPtr<ACL>,
        dw_ace_revision: u32,
        access_mask: u32,
        p_sid: super::Foundation::PSID,
    ) -> super::Foundation::BOOL {
        todo!("AddAccessAllowedAce")
    }
    #[doc = "AddAccessAllowedAceEx from ADVAPI32"]
    fn AddAccessAllowedAceEx(
        &self,
        p_acl: MutPtr<ACL>,
        dw_ace_revision: u32,
        ace_flags: ACE_FLAGS,
        access_mask: u32,
        p_sid: super::Foundation::PSID,
    ) -> super::Foundation::BOOL {
        todo!("AddAccessAllowedAceEx")
    }
    #[doc = "AddAccessAllowedObjectAce from ADVAPI32"]
    fn AddAccessAllowedObjectAce(
        &self,
        p_acl: MutPtr<ACL>,
        dw_ace_revision: u32,
        ace_flags: ACE_FLAGS,
        access_mask: u32,
        object_type_guid: ConstPtr<crate::core::GUID>,
        inherited_object_type_guid: ConstPtr<crate::core::GUID>,
        p_sid: super::Foundation::PSID,
    ) -> super::Foundation::BOOL {
        todo!("AddAccessAllowedObjectAce")
    }
    #[doc = "AddAccessDeniedAce from ADVAPI32"]
    fn AddAccessDeniedAce(
        &self,
        p_acl: MutPtr<ACL>,
        dw_ace_revision: u32,
        access_mask: u32,
        p_sid: super::Foundation::PSID,
    ) -> super::Foundation::BOOL {
        todo!("AddAccessDeniedAce")
    }
    #[doc = "AddAccessDeniedAceEx from ADVAPI32"]
    fn AddAccessDeniedAceEx(
        &self,
        p_acl: MutPtr<ACL>,
        dw_ace_revision: u32,
        ace_flags: ACE_FLAGS,
        access_mask: u32,
        p_sid: super::Foundation::PSID,
    ) -> super::Foundation::BOOL {
        todo!("AddAccessDeniedAceEx")
    }
    #[doc = "AddAccessDeniedObjectAce from ADVAPI32"]
    fn AddAccessDeniedObjectAce(
        &self,
        p_acl: MutPtr<ACL>,
        dw_ace_revision: u32,
        ace_flags: ACE_FLAGS,
        access_mask: u32,
        object_type_guid: ConstPtr<crate::core::GUID>,
        inherited_object_type_guid: ConstPtr<crate::core::GUID>,
        p_sid: super::Foundation::PSID,
    ) -> super::Foundation::BOOL {
        todo!("AddAccessDeniedObjectAce")
    }
    #[doc = "AddAce from ADVAPI32"]
    fn AddAce(
        &self,
        p_acl: MutPtr<ACL>,
        dw_ace_revision: u32,
        dw_starting_ace_index: u32,
        p_ace_list: ConstPtr<::core::ffi::c_void>,
        n_ace_list_length: u32,
    ) -> super::Foundation::BOOL {
        todo!("AddAce")
    }
    #[doc = "AddAuditAccessAce from ADVAPI32"]
    fn AddAuditAccessAce(
        &self,
        p_acl: MutPtr<ACL>,
        dw_ace_revision: u32,
        dw_access_mask: u32,
        p_sid: super::Foundation::PSID,
        b_audit_success: super::Foundation::BOOL,
        b_audit_failure: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("AddAuditAccessAce")
    }
    #[doc = "AddAuditAccessAceEx from ADVAPI32"]
    fn AddAuditAccessAceEx(
        &self,
        p_acl: MutPtr<ACL>,
        dw_ace_revision: u32,
        ace_flags: ACE_FLAGS,
        dw_access_mask: u32,
        p_sid: super::Foundation::PSID,
        b_audit_success: super::Foundation::BOOL,
        b_audit_failure: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("AddAuditAccessAceEx")
    }
    #[doc = "AddAuditAccessObjectAce from ADVAPI32"]
    fn AddAuditAccessObjectAce(
        &self,
        p_acl: MutPtr<ACL>,
        dw_ace_revision: u32,
        ace_flags: ACE_FLAGS,
        access_mask: u32,
        object_type_guid: ConstPtr<crate::core::GUID>,
        inherited_object_type_guid: ConstPtr<crate::core::GUID>,
        p_sid: super::Foundation::PSID,
        b_audit_success: super::Foundation::BOOL,
        b_audit_failure: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("AddAuditAccessObjectAce")
    }
    #[doc = "AddConditionalAce from ADVAPI32"]
    fn AddConditionalAce(
        &self,
        p_acl: MutPtr<ACL>,
        dw_ace_revision: u32,
        ace_flags: ACE_FLAGS,
        ace_type: u8,
        access_mask: u32,
        p_sid: super::Foundation::PSID,
        condition_str: PCWSTR,
        return_length: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("AddConditionalAce")
    }
    #[doc = "AddMandatoryAce from ADVAPI32"]
    fn AddMandatoryAce(
        &self,
        p_acl: MutPtr<ACL>,
        dw_ace_revision: ACE_REVISION,
        ace_flags: ACE_FLAGS,
        mandatory_policy: u32,
        p_label_sid: super::Foundation::PSID,
    ) -> super::Foundation::BOOL {
        todo!("AddMandatoryAce")
    }
    #[doc = "AddResourceAttributeAce from KERNEL32"]
    fn AddResourceAttributeAce(
        &self,
        p_acl: MutPtr<ACL>,
        dw_ace_revision: u32,
        ace_flags: ACE_FLAGS,
        access_mask: u32,
        p_sid: super::Foundation::PSID,
        p_attribute_info: ConstPtr<CLAIM_SECURITY_ATTRIBUTES_INFORMATION>,
        p_return_length: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("AddResourceAttributeAce")
    }
    #[doc = "AddScopedPolicyIDAce from KERNEL32"]
    fn AddScopedPolicyIDAce(
        &self,
        p_acl: MutPtr<ACL>,
        dw_ace_revision: u32,
        ace_flags: ACE_FLAGS,
        access_mask: u32,
        p_sid: super::Foundation::PSID,
    ) -> super::Foundation::BOOL {
        todo!("AddScopedPolicyIDAce")
    }
    #[doc = "AdjustTokenGroups from ADVAPI32"]
    fn AdjustTokenGroups(
        &self,
        token_handle: super::Foundation::HANDLE,
        reset_to_default: super::Foundation::BOOL,
        new_state: ConstPtr<TOKEN_GROUPS>,
        buffer_length: u32,
        previous_state: MutPtr<TOKEN_GROUPS>,
        return_length: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("AdjustTokenGroups")
    }
    #[doc = "AdjustTokenPrivileges from ADVAPI32"]
    fn AdjustTokenPrivileges(
        &self,
        token_handle: super::Foundation::HANDLE,
        disable_all_privileges: super::Foundation::BOOL,
        new_state: ConstPtr<TOKEN_PRIVILEGES>,
        buffer_length: u32,
        previous_state: MutPtr<TOKEN_PRIVILEGES>,
        return_length: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("AdjustTokenPrivileges")
    }
    #[doc = "AllocateAndInitializeSid from ADVAPI32"]
    fn AllocateAndInitializeSid(
        &self,
        p_identifier_authority: ConstPtr<SID_IDENTIFIER_AUTHORITY>,
        n_sub_authority_count: u8,
        n_sub_authority_0: u32,
        n_sub_authority_1: u32,
        n_sub_authority_2: u32,
        n_sub_authority_3: u32,
        n_sub_authority_4: u32,
        n_sub_authority_5: u32,
        n_sub_authority_6: u32,
        n_sub_authority_7: u32,
        p_sid: MutPtr<super::Foundation::PSID>,
    ) -> super::Foundation::BOOL {
        todo!("AllocateAndInitializeSid")
    }
    #[doc = "AllocateLocallyUniqueId from ADVAPI32"]
    fn AllocateLocallyUniqueId(
        &self,
        luid: MutPtr<super::Foundation::LUID>,
    ) -> super::Foundation::BOOL {
        todo!("AllocateLocallyUniqueId")
    }
    #[doc = "AreAllAccessesGranted from ADVAPI32"]
    fn AreAllAccessesGranted(
        &self,
        granted_access: u32,
        desired_access: u32,
    ) -> super::Foundation::BOOL {
        todo!("AreAllAccessesGranted")
    }
    #[doc = "AreAnyAccessesGranted from ADVAPI32"]
    fn AreAnyAccessesGranted(
        &self,
        granted_access: u32,
        desired_access: u32,
    ) -> super::Foundation::BOOL {
        todo!("AreAnyAccessesGranted")
    }
    #[doc = "CheckTokenCapability from KERNEL32"]
    fn CheckTokenCapability(
        &self,
        token_handle: super::Foundation::HANDLE,
        capability_sid_to_check: super::Foundation::PSID,
        has_capability: MutPtr<super::Foundation::BOOL>,
    ) -> super::Foundation::BOOL {
        todo!("CheckTokenCapability")
    }
    #[doc = "CheckTokenMembership from ADVAPI32"]
    fn CheckTokenMembership(
        &self,
        token_handle: super::Foundation::HANDLE,
        sid_to_check: super::Foundation::PSID,
        is_member: MutPtr<super::Foundation::BOOL>,
    ) -> super::Foundation::BOOL {
        todo!("CheckTokenMembership")
    }
    #[doc = "CheckTokenMembershipEx from KERNEL32"]
    fn CheckTokenMembershipEx(
        &self,
        token_handle: super::Foundation::HANDLE,
        sid_to_check: super::Foundation::PSID,
        flags: u32,
        is_member: MutPtr<super::Foundation::BOOL>,
    ) -> super::Foundation::BOOL {
        todo!("CheckTokenMembershipEx")
    }
    #[doc = "ConvertToAutoInheritPrivateObjectSecurity from ADVAPI32"]
    fn ConvertToAutoInheritPrivateObjectSecurity(
        &self,
        parent_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        current_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        new_security_descriptor: MutPtr<ConstPtr<SECURITY_DESCRIPTOR>>,
        object_type: ConstPtr<crate::core::GUID>,
        is_directory_object: super::Foundation::BOOLEAN,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
    ) -> super::Foundation::BOOL {
        todo!("ConvertToAutoInheritPrivateObjectSecurity")
    }
    #[doc = "CopySid from ADVAPI32"]
    fn CopySid(
        &self,
        n_destination_sid_length: u32,
        p_destination_sid: super::Foundation::PSID,
        p_source_sid: super::Foundation::PSID,
    ) -> super::Foundation::BOOL {
        todo!("CopySid")
    }
    #[doc = "CreatePrivateObjectSecurity from ADVAPI32"]
    fn CreatePrivateObjectSecurity(
        &self,
        parent_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        creator_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        new_descriptor: MutPtr<ConstPtr<SECURITY_DESCRIPTOR>>,
        is_directory_object: super::Foundation::BOOL,
        token: super::Foundation::HANDLE,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
    ) -> super::Foundation::BOOL {
        todo!("CreatePrivateObjectSecurity")
    }
    #[doc = "CreatePrivateObjectSecurityEx from ADVAPI32"]
    fn CreatePrivateObjectSecurityEx(
        &self,
        parent_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        creator_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        new_descriptor: MutPtr<ConstPtr<SECURITY_DESCRIPTOR>>,
        object_type: ConstPtr<crate::core::GUID>,
        is_container_object: super::Foundation::BOOL,
        auto_inherit_flags: SECURITY_AUTO_INHERIT_FLAGS,
        token: super::Foundation::HANDLE,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
    ) -> super::Foundation::BOOL {
        todo!("CreatePrivateObjectSecurityEx")
    }
    #[doc = "CreatePrivateObjectSecurityWithMultipleInheritance from ADVAPI32"]
    fn CreatePrivateObjectSecurityWithMultipleInheritance(
        &self,
        parent_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        creator_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        new_descriptor: MutPtr<ConstPtr<SECURITY_DESCRIPTOR>>,
        object_types: ConstPtr<ConstPtr<crate::core::GUID>>,
        guid_count: u32,
        is_container_object: super::Foundation::BOOL,
        auto_inherit_flags: SECURITY_AUTO_INHERIT_FLAGS,
        token: super::Foundation::HANDLE,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
    ) -> super::Foundation::BOOL {
        todo!("CreatePrivateObjectSecurityWithMultipleInheritance")
    }
    #[doc = "CreateRestrictedToken from ADVAPI32"]
    fn CreateRestrictedToken(
        &self,
        existing_token_handle: super::Foundation::HANDLE,
        flags: CREATE_RESTRICTED_TOKEN_FLAGS,
        disable_sid_count: u32,
        sids_to_disable: ConstPtr<SID_AND_ATTRIBUTES>,
        delete_privilege_count: u32,
        privileges_to_delete: ConstPtr<LUID_AND_ATTRIBUTES>,
        restricted_sid_count: u32,
        sids_to_restrict: ConstPtr<SID_AND_ATTRIBUTES>,
        new_token_handle: MutPtr<super::Foundation::HANDLE>,
    ) -> super::Foundation::BOOL {
        todo!("CreateRestrictedToken")
    }
    #[doc = "CreateWellKnownSid from ADVAPI32"]
    fn CreateWellKnownSid(
        &self,
        well_known_sid_type: WELL_KNOWN_SID_TYPE,
        domain_sid: super::Foundation::PSID,
        p_sid: super::Foundation::PSID,
        cb_sid: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("CreateWellKnownSid")
    }
    #[doc = "DeleteAce from ADVAPI32"]
    fn DeleteAce(&self, p_acl: MutPtr<ACL>, dw_ace_index: u32) -> super::Foundation::BOOL {
        todo!("DeleteAce")
    }
    #[doc = "DestroyPrivateObjectSecurity from ADVAPI32"]
    fn DestroyPrivateObjectSecurity(
        &self,
        object_descriptor: ConstPtr<ConstPtr<SECURITY_DESCRIPTOR>>,
    ) -> super::Foundation::BOOL {
        todo!("DestroyPrivateObjectSecurity")
    }
    #[doc = "DuplicateToken from ADVAPI32"]
    fn DuplicateToken(
        &self,
        existing_token_handle: super::Foundation::HANDLE,
        impersonation_level: SECURITY_IMPERSONATION_LEVEL,
        duplicate_token_handle: MutPtr<super::Foundation::HANDLE>,
    ) -> super::Foundation::BOOL {
        todo!("DuplicateToken")
    }
    #[doc = "DuplicateTokenEx from ADVAPI32"]
    fn DuplicateTokenEx(
        &self,
        h_existing_token: super::Foundation::HANDLE,
        dw_desired_access: TOKEN_ACCESS_MASK,
        lp_token_attributes: ConstPtr<SECURITY_ATTRIBUTES>,
        impersonation_level: SECURITY_IMPERSONATION_LEVEL,
        token_type: TOKEN_TYPE,
        ph_new_token: MutPtr<super::Foundation::HANDLE>,
    ) -> super::Foundation::BOOL {
        todo!("DuplicateTokenEx")
    }
    #[doc = "EqualDomainSid from ADVAPI32"]
    fn EqualDomainSid(
        &self,
        p_sid_1: super::Foundation::PSID,
        p_sid_2: super::Foundation::PSID,
        pf_equal: MutPtr<super::Foundation::BOOL>,
    ) -> super::Foundation::BOOL {
        todo!("EqualDomainSid")
    }
    #[doc = "EqualPrefixSid from ADVAPI32"]
    fn EqualPrefixSid(
        &self,
        p_sid_1: super::Foundation::PSID,
        p_sid_2: super::Foundation::PSID,
    ) -> super::Foundation::BOOL {
        todo!("EqualPrefixSid")
    }
    #[doc = "EqualSid from ADVAPI32"]
    fn EqualSid(
        &self,
        p_sid_1: super::Foundation::PSID,
        p_sid_2: super::Foundation::PSID,
    ) -> super::Foundation::BOOL {
        todo!("EqualSid")
    }
    #[doc = "FindFirstFreeAce from ADVAPI32"]
    fn FindFirstFreeAce(
        &self,
        p_acl: ConstPtr<ACL>,
        p_ace: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> super::Foundation::BOOL {
        todo!("FindFirstFreeAce")
    }
    #[doc = "FreeSid from ADVAPI32"]
    fn FreeSid(&self, p_sid: super::Foundation::PSID) -> MutPtr<::core::ffi::c_void> {
        todo!("FreeSid")
    }
    #[doc = "GetAce from ADVAPI32"]
    fn GetAce(
        &self,
        p_acl: ConstPtr<ACL>,
        dw_ace_index: u32,
        p_ace: MutPtr<ConstPtr<::core::ffi::c_void>>,
    ) -> super::Foundation::BOOL {
        todo!("GetAce")
    }
    #[doc = "GetAclInformation from ADVAPI32"]
    fn GetAclInformation(
        &self,
        p_acl: ConstPtr<ACL>,
        p_acl_information: MutPtr<::core::ffi::c_void>,
        n_acl_information_length: u32,
        dw_acl_information_class: ACL_INFORMATION_CLASS,
    ) -> super::Foundation::BOOL {
        todo!("GetAclInformation")
    }
    #[doc = "GetAppContainerAce from KERNEL32"]
    fn GetAppContainerAce(
        &self,
        acl: ConstPtr<ACL>,
        starting_ace_index: u32,
        app_container_ace: MutPtr<ConstPtr<::core::ffi::c_void>>,
        app_container_ace_index: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetAppContainerAce")
    }
    #[doc = "GetCachedSigningLevel from KERNEL32"]
    fn GetCachedSigningLevel(
        &self,
        file: super::Foundation::HANDLE,
        flags: MutPtr<u32>,
        signing_level: MutPtr<u32>,
        thumbprint: MutPtr<u8>,
        thumbprint_size: MutPtr<u32>,
        thumbprint_algorithm: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetCachedSigningLevel")
    }
    #[doc = "GetFileSecurityA from ADVAPI32"]
    fn GetFileSecurityA(
        &self,
        lp_file_name: PCSTR,
        requested_information: u32,
        p_security_descriptor: MutPtr<SECURITY_DESCRIPTOR>,
        n_length: u32,
        lpn_length_needed: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetFileSecurityA")
    }
    #[doc = "GetFileSecurityW from ADVAPI32"]
    fn GetFileSecurityW(
        &self,
        lp_file_name: PCWSTR,
        requested_information: u32,
        p_security_descriptor: MutPtr<SECURITY_DESCRIPTOR>,
        n_length: u32,
        lpn_length_needed: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetFileSecurityW")
    }
    #[doc = "GetKernelObjectSecurity from ADVAPI32"]
    fn GetKernelObjectSecurity(
        &self,
        handle: super::Foundation::HANDLE,
        requested_information: u32,
        p_security_descriptor: MutPtr<SECURITY_DESCRIPTOR>,
        n_length: u32,
        lpn_length_needed: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetKernelObjectSecurity")
    }
    #[doc = "GetLengthSid from ADVAPI32"]
    fn GetLengthSid(&self, p_sid: super::Foundation::PSID) -> u32 {
        todo!("GetLengthSid")
    }
    #[doc = "GetPrivateObjectSecurity from ADVAPI32"]
    fn GetPrivateObjectSecurity(
        &self,
        object_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        security_information: u32,
        resultant_descriptor: MutPtr<SECURITY_DESCRIPTOR>,
        descriptor_length: u32,
        return_length: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetPrivateObjectSecurity")
    }
    #[doc = "GetSecurityDescriptorControl from ADVAPI32"]
    fn GetSecurityDescriptorControl(
        &self,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        p_control: MutPtr<u16>,
        lpdw_revision: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetSecurityDescriptorControl")
    }
    #[doc = "GetSecurityDescriptorDacl from ADVAPI32"]
    fn GetSecurityDescriptorDacl(
        &self,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        lpb_dacl_present: MutPtr<i32>,
        p_dacl: MutPtr<ConstPtr<ACL>>,
        lpb_dacl_defaulted: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("GetSecurityDescriptorDacl")
    }
    #[doc = "GetSecurityDescriptorGroup from ADVAPI32"]
    fn GetSecurityDescriptorGroup(
        &self,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        p_group: MutPtr<super::Foundation::PSID>,
        lpb_group_defaulted: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("GetSecurityDescriptorGroup")
    }
    #[doc = "GetSecurityDescriptorLength from ADVAPI32"]
    fn GetSecurityDescriptorLength(
        &self,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
    ) -> u32 {
        todo!("GetSecurityDescriptorLength")
    }
    #[doc = "GetSecurityDescriptorOwner from ADVAPI32"]
    fn GetSecurityDescriptorOwner(
        &self,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        p_owner: MutPtr<super::Foundation::PSID>,
        lpb_owner_defaulted: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("GetSecurityDescriptorOwner")
    }
    #[doc = "GetSecurityDescriptorRMControl from ADVAPI32"]
    fn GetSecurityDescriptorRMControl(
        &self,
        security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        rm_control: MutPtr<u8>,
    ) -> u32 {
        todo!("GetSecurityDescriptorRMControl")
    }
    #[doc = "GetSecurityDescriptorSacl from ADVAPI32"]
    fn GetSecurityDescriptorSacl(
        &self,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        lpb_sacl_present: MutPtr<i32>,
        p_sacl: MutPtr<ConstPtr<ACL>>,
        lpb_sacl_defaulted: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("GetSecurityDescriptorSacl")
    }
    #[doc = "GetSidIdentifierAuthority from ADVAPI32"]
    fn GetSidIdentifierAuthority(
        &self,
        p_sid: super::Foundation::PSID,
    ) -> MutPtr<SID_IDENTIFIER_AUTHORITY> {
        todo!("GetSidIdentifierAuthority")
    }
    #[doc = "GetSidLengthRequired from ADVAPI32"]
    fn GetSidLengthRequired(&self, n_sub_authority_count: u8) -> u32 {
        todo!("GetSidLengthRequired")
    }
    #[doc = "GetSidSubAuthority from ADVAPI32"]
    fn GetSidSubAuthority(
        &self,
        p_sid: super::Foundation::PSID,
        n_sub_authority: u32,
    ) -> MutPtr<u32> {
        todo!("GetSidSubAuthority")
    }
    #[doc = "GetSidSubAuthorityCount from ADVAPI32"]
    fn GetSidSubAuthorityCount(&self, p_sid: super::Foundation::PSID) -> MutPtr<u8> {
        todo!("GetSidSubAuthorityCount")
    }
    #[doc = "GetTokenInformation from ADVAPI32"]
    fn GetTokenInformation(
        &self,
        token_handle: super::Foundation::HANDLE,
        token_information_class: TOKEN_INFORMATION_CLASS,
        token_information: MutPtr<::core::ffi::c_void>,
        token_information_length: u32,
        return_length: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetTokenInformation")
    }
    #[doc = "GetUserObjectSecurity from USER32"]
    fn GetUserObjectSecurity(
        &self,
        h_obj: super::Foundation::HANDLE,
        p_si_requested: ConstPtr<u32>,
        p_sid: MutPtr<SECURITY_DESCRIPTOR>,
        n_length: u32,
        lpn_length_needed: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetUserObjectSecurity")
    }
    #[doc = "GetWindowsAccountDomainSid from ADVAPI32"]
    fn GetWindowsAccountDomainSid(
        &self,
        p_sid: super::Foundation::PSID,
        p_domain_sid: super::Foundation::PSID,
        cb_domain_sid: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("GetWindowsAccountDomainSid")
    }
    #[doc = "ImpersonateAnonymousToken from ADVAPI32"]
    fn ImpersonateAnonymousToken(
        &self,
        thread_handle: super::Foundation::HANDLE,
    ) -> super::Foundation::BOOL {
        todo!("ImpersonateAnonymousToken")
    }
    #[doc = "ImpersonateLoggedOnUser from ADVAPI32"]
    fn ImpersonateLoggedOnUser(
        &self,
        h_token: super::Foundation::HANDLE,
    ) -> super::Foundation::BOOL {
        todo!("ImpersonateLoggedOnUser")
    }
    #[doc = "ImpersonateSelf from ADVAPI32"]
    fn ImpersonateSelf(
        &self,
        impersonation_level: SECURITY_IMPERSONATION_LEVEL,
    ) -> super::Foundation::BOOL {
        todo!("ImpersonateSelf")
    }
    #[doc = "InitializeAcl from ADVAPI32"]
    fn InitializeAcl(
        &self,
        p_acl: MutPtr<ACL>,
        n_acl_length: u32,
        dw_acl_revision: u32,
    ) -> super::Foundation::BOOL {
        todo!("InitializeAcl")
    }
    #[doc = "InitializeSecurityDescriptor from ADVAPI32"]
    fn InitializeSecurityDescriptor(
        &self,
        p_security_descriptor: MutPtr<SECURITY_DESCRIPTOR>,
        dw_revision: u32,
    ) -> super::Foundation::BOOL {
        todo!("InitializeSecurityDescriptor")
    }
    #[doc = "InitializeSid from ADVAPI32"]
    fn InitializeSid(
        &self,
        sid: super::Foundation::PSID,
        p_identifier_authority: ConstPtr<SID_IDENTIFIER_AUTHORITY>,
        n_sub_authority_count: u8,
    ) -> super::Foundation::BOOL {
        todo!("InitializeSid")
    }
    #[doc = "IsTokenRestricted from ADVAPI32"]
    fn IsTokenRestricted(
        &self,
        token_handle: super::Foundation::HANDLE,
    ) -> super::Foundation::BOOL {
        todo!("IsTokenRestricted")
    }
    #[doc = "IsValidAcl from ADVAPI32"]
    fn IsValidAcl(&self, p_acl: ConstPtr<ACL>) -> super::Foundation::BOOL {
        todo!("IsValidAcl")
    }
    #[doc = "IsValidSecurityDescriptor from ADVAPI32"]
    fn IsValidSecurityDescriptor(
        &self,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
    ) -> super::Foundation::BOOL {
        todo!("IsValidSecurityDescriptor")
    }
    #[doc = "IsValidSid from ADVAPI32"]
    fn IsValidSid(&self, p_sid: super::Foundation::PSID) -> super::Foundation::BOOL {
        todo!("IsValidSid")
    }
    #[doc = "IsWellKnownSid from ADVAPI32"]
    fn IsWellKnownSid(
        &self,
        p_sid: super::Foundation::PSID,
        well_known_sid_type: WELL_KNOWN_SID_TYPE,
    ) -> super::Foundation::BOOL {
        todo!("IsWellKnownSid")
    }
    #[doc = "LogonUserA from ADVAPI32"]
    fn LogonUserA(
        &self,
        lpsz_username: PCSTR,
        lpsz_domain: PCSTR,
        lpsz_password: PCSTR,
        dw_logon_type: LOGON32_LOGON,
        dw_logon_provider: LOGON32_PROVIDER,
        ph_token: MutPtr<super::Foundation::HANDLE>,
    ) -> super::Foundation::BOOL {
        todo!("LogonUserA")
    }
    #[doc = "LogonUserExA from ADVAPI32"]
    fn LogonUserExA(
        &self,
        lpsz_username: PCSTR,
        lpsz_domain: PCSTR,
        lpsz_password: PCSTR,
        dw_logon_type: LOGON32_LOGON,
        dw_logon_provider: LOGON32_PROVIDER,
        ph_token: MutPtr<super::Foundation::HANDLE>,
        pp_logon_sid: MutPtr<super::Foundation::PSID>,
        pp_profile_buffer: MutPtr<ConstPtr<::core::ffi::c_void>>,
        pdw_profile_length: MutPtr<u32>,
        p_quota_limits: MutPtr<QUOTA_LIMITS>,
    ) -> super::Foundation::BOOL {
        todo!("LogonUserExA")
    }
    #[doc = "LogonUserExW from ADVAPI32"]
    fn LogonUserExW(
        &self,
        lpsz_username: PCWSTR,
        lpsz_domain: PCWSTR,
        lpsz_password: PCWSTR,
        dw_logon_type: LOGON32_LOGON,
        dw_logon_provider: LOGON32_PROVIDER,
        ph_token: MutPtr<super::Foundation::HANDLE>,
        pp_logon_sid: MutPtr<super::Foundation::PSID>,
        pp_profile_buffer: MutPtr<ConstPtr<::core::ffi::c_void>>,
        pdw_profile_length: MutPtr<u32>,
        p_quota_limits: MutPtr<QUOTA_LIMITS>,
    ) -> super::Foundation::BOOL {
        todo!("LogonUserExW")
    }
    #[doc = "LogonUserW from ADVAPI32"]
    fn LogonUserW(
        &self,
        lpsz_username: PCWSTR,
        lpsz_domain: PCWSTR,
        lpsz_password: PCWSTR,
        dw_logon_type: LOGON32_LOGON,
        dw_logon_provider: LOGON32_PROVIDER,
        ph_token: MutPtr<super::Foundation::HANDLE>,
    ) -> super::Foundation::BOOL {
        todo!("LogonUserW")
    }
    #[doc = "LookupAccountNameA from ADVAPI32"]
    fn LookupAccountNameA(
        &self,
        lp_system_name: PCSTR,
        lp_account_name: PCSTR,
        sid: super::Foundation::PSID,
        cb_sid: MutPtr<u32>,
        referenced_domain_name: PSTR,
        cch_referenced_domain_name: MutPtr<u32>,
        pe_use: MutPtr<SID_NAME_USE>,
    ) -> super::Foundation::BOOL {
        todo!("LookupAccountNameA")
    }
    #[doc = "LookupAccountNameW from ADVAPI32"]
    fn LookupAccountNameW(
        &self,
        lp_system_name: PCWSTR,
        lp_account_name: PCWSTR,
        sid: super::Foundation::PSID,
        cb_sid: MutPtr<u32>,
        referenced_domain_name: PWSTR,
        cch_referenced_domain_name: MutPtr<u32>,
        pe_use: MutPtr<SID_NAME_USE>,
    ) -> super::Foundation::BOOL {
        todo!("LookupAccountNameW")
    }
    #[doc = "LookupAccountSidA from ADVAPI32"]
    fn LookupAccountSidA(
        &self,
        lp_system_name: PCSTR,
        sid: super::Foundation::PSID,
        name: PSTR,
        cch_name: MutPtr<u32>,
        referenced_domain_name: PSTR,
        cch_referenced_domain_name: MutPtr<u32>,
        pe_use: MutPtr<SID_NAME_USE>,
    ) -> super::Foundation::BOOL {
        todo!("LookupAccountSidA")
    }
    #[doc = "LookupAccountSidW from ADVAPI32"]
    fn LookupAccountSidW(
        &self,
        lp_system_name: PCWSTR,
        sid: super::Foundation::PSID,
        name: PWSTR,
        cch_name: MutPtr<u32>,
        referenced_domain_name: PWSTR,
        cch_referenced_domain_name: MutPtr<u32>,
        pe_use: MutPtr<SID_NAME_USE>,
    ) -> super::Foundation::BOOL {
        todo!("LookupAccountSidW")
    }
    #[doc = "LookupPrivilegeDisplayNameA from ADVAPI32"]
    fn LookupPrivilegeDisplayNameA(
        &self,
        lp_system_name: PCSTR,
        lp_name: PCSTR,
        lp_display_name: PSTR,
        cch_display_name: MutPtr<u32>,
        lp_language_id: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("LookupPrivilegeDisplayNameA")
    }
    #[doc = "LookupPrivilegeDisplayNameW from ADVAPI32"]
    fn LookupPrivilegeDisplayNameW(
        &self,
        lp_system_name: PCWSTR,
        lp_name: PCWSTR,
        lp_display_name: PWSTR,
        cch_display_name: MutPtr<u32>,
        lp_language_id: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("LookupPrivilegeDisplayNameW")
    }
    #[doc = "LookupPrivilegeNameA from ADVAPI32"]
    fn LookupPrivilegeNameA(
        &self,
        lp_system_name: PCSTR,
        lp_luid: ConstPtr<super::Foundation::LUID>,
        lp_name: PSTR,
        cch_name: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("LookupPrivilegeNameA")
    }
    #[doc = "LookupPrivilegeNameW from ADVAPI32"]
    fn LookupPrivilegeNameW(
        &self,
        lp_system_name: PCWSTR,
        lp_luid: ConstPtr<super::Foundation::LUID>,
        lp_name: PWSTR,
        cch_name: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("LookupPrivilegeNameW")
    }
    #[doc = "LookupPrivilegeValueA from ADVAPI32"]
    fn LookupPrivilegeValueA(
        &self,
        lp_system_name: PCSTR,
        lp_name: PCSTR,
        lp_luid: MutPtr<super::Foundation::LUID>,
    ) -> super::Foundation::BOOL {
        todo!("LookupPrivilegeValueA")
    }
    #[doc = "LookupPrivilegeValueW from ADVAPI32"]
    fn LookupPrivilegeValueW(
        &self,
        lp_system_name: PCWSTR,
        lp_name: PCWSTR,
        lp_luid: MutPtr<super::Foundation::LUID>,
    ) -> super::Foundation::BOOL {
        todo!("LookupPrivilegeValueW")
    }
    #[doc = "MakeAbsoluteSD from ADVAPI32"]
    fn MakeAbsoluteSD(
        &self,
        p_self_relative_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        p_absolute_security_descriptor: MutPtr<SECURITY_DESCRIPTOR>,
        lpdw_absolute_security_descriptor_size: MutPtr<u32>,
        p_dacl: MutPtr<ACL>,
        lpdw_dacl_size: MutPtr<u32>,
        p_sacl: MutPtr<ACL>,
        lpdw_sacl_size: MutPtr<u32>,
        p_owner: super::Foundation::PSID,
        lpdw_owner_size: MutPtr<u32>,
        p_primary_group: super::Foundation::PSID,
        lpdw_primary_group_size: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("MakeAbsoluteSD")
    }
    #[doc = "MakeSelfRelativeSD from ADVAPI32"]
    fn MakeSelfRelativeSD(
        &self,
        p_absolute_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        p_self_relative_security_descriptor: MutPtr<SECURITY_DESCRIPTOR>,
        lpdw_buffer_length: MutPtr<u32>,
    ) -> super::Foundation::BOOL {
        todo!("MakeSelfRelativeSD")
    }
    #[doc = "MapGenericMask from ADVAPI32"]
    fn MapGenericMask(&self, access_mask: MutPtr<u32>, generic_mapping: ConstPtr<GENERIC_MAPPING>) {
        todo!("MapGenericMask")
    }
    #[doc = "ObjectCloseAuditAlarmA from ADVAPI32"]
    fn ObjectCloseAuditAlarmA(
        &self,
        subsystem_name: PCSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        generate_on_close: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("ObjectCloseAuditAlarmA")
    }
    #[doc = "ObjectCloseAuditAlarmW from ADVAPI32"]
    fn ObjectCloseAuditAlarmW(
        &self,
        subsystem_name: PCWSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        generate_on_close: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("ObjectCloseAuditAlarmW")
    }
    #[doc = "ObjectDeleteAuditAlarmA from ADVAPI32"]
    fn ObjectDeleteAuditAlarmA(
        &self,
        subsystem_name: PCSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        generate_on_close: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("ObjectDeleteAuditAlarmA")
    }
    #[doc = "ObjectDeleteAuditAlarmW from ADVAPI32"]
    fn ObjectDeleteAuditAlarmW(
        &self,
        subsystem_name: PCWSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        generate_on_close: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("ObjectDeleteAuditAlarmW")
    }
    #[doc = "ObjectOpenAuditAlarmA from ADVAPI32"]
    fn ObjectOpenAuditAlarmA(
        &self,
        subsystem_name: PCSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        object_type_name: PCSTR,
        object_name: PCSTR,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        client_token: super::Foundation::HANDLE,
        desired_access: u32,
        granted_access: u32,
        privileges: ConstPtr<PRIVILEGE_SET>,
        object_creation: super::Foundation::BOOL,
        access_granted: super::Foundation::BOOL,
        generate_on_close: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("ObjectOpenAuditAlarmA")
    }
    #[doc = "ObjectOpenAuditAlarmW from ADVAPI32"]
    fn ObjectOpenAuditAlarmW(
        &self,
        subsystem_name: PCWSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        object_type_name: PCWSTR,
        object_name: PCWSTR,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        client_token: super::Foundation::HANDLE,
        desired_access: u32,
        granted_access: u32,
        privileges: ConstPtr<PRIVILEGE_SET>,
        object_creation: super::Foundation::BOOL,
        access_granted: super::Foundation::BOOL,
        generate_on_close: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("ObjectOpenAuditAlarmW")
    }
    #[doc = "ObjectPrivilegeAuditAlarmA from ADVAPI32"]
    fn ObjectPrivilegeAuditAlarmA(
        &self,
        subsystem_name: PCSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        client_token: super::Foundation::HANDLE,
        desired_access: u32,
        privileges: ConstPtr<PRIVILEGE_SET>,
        access_granted: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("ObjectPrivilegeAuditAlarmA")
    }
    #[doc = "ObjectPrivilegeAuditAlarmW from ADVAPI32"]
    fn ObjectPrivilegeAuditAlarmW(
        &self,
        subsystem_name: PCWSTR,
        handle_id: ConstPtr<::core::ffi::c_void>,
        client_token: super::Foundation::HANDLE,
        desired_access: u32,
        privileges: ConstPtr<PRIVILEGE_SET>,
        access_granted: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("ObjectPrivilegeAuditAlarmW")
    }
    #[doc = "PrivilegeCheck from ADVAPI32"]
    fn PrivilegeCheck(
        &self,
        client_token: super::Foundation::HANDLE,
        required_privileges: MutPtr<PRIVILEGE_SET>,
        pf_result: MutPtr<i32>,
    ) -> super::Foundation::BOOL {
        todo!("PrivilegeCheck")
    }
    #[doc = "PrivilegedServiceAuditAlarmA from ADVAPI32"]
    fn PrivilegedServiceAuditAlarmA(
        &self,
        subsystem_name: PCSTR,
        service_name: PCSTR,
        client_token: super::Foundation::HANDLE,
        privileges: ConstPtr<PRIVILEGE_SET>,
        access_granted: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("PrivilegedServiceAuditAlarmA")
    }
    #[doc = "PrivilegedServiceAuditAlarmW from ADVAPI32"]
    fn PrivilegedServiceAuditAlarmW(
        &self,
        subsystem_name: PCWSTR,
        service_name: PCWSTR,
        client_token: super::Foundation::HANDLE,
        privileges: ConstPtr<PRIVILEGE_SET>,
        access_granted: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("PrivilegedServiceAuditAlarmW")
    }
    #[doc = "QuerySecurityAccessMask from ADVAPI32"]
    fn QuerySecurityAccessMask(&self, security_information: u32, desired_access: MutPtr<u32>) {
        todo!("QuerySecurityAccessMask")
    }
    #[doc = "RevertToSelf from ADVAPI32"]
    fn RevertToSelf(&self) -> super::Foundation::BOOL {
        todo!("RevertToSelf")
    }
    #[doc = "SetAclInformation from ADVAPI32"]
    fn SetAclInformation(
        &self,
        p_acl: MutPtr<ACL>,
        p_acl_information: ConstPtr<::core::ffi::c_void>,
        n_acl_information_length: u32,
        dw_acl_information_class: ACL_INFORMATION_CLASS,
    ) -> super::Foundation::BOOL {
        todo!("SetAclInformation")
    }
    #[doc = "SetCachedSigningLevel from KERNEL32"]
    fn SetCachedSigningLevel(
        &self,
        source_files: ConstPtr<super::Foundation::HANDLE>,
        source_file_count: u32,
        flags: u32,
        target_file: super::Foundation::HANDLE,
    ) -> super::Foundation::BOOL {
        todo!("SetCachedSigningLevel")
    }
    #[doc = "SetFileSecurityA from ADVAPI32"]
    fn SetFileSecurityA(
        &self,
        lp_file_name: PCSTR,
        security_information: u32,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
    ) -> super::Foundation::BOOL {
        todo!("SetFileSecurityA")
    }
    #[doc = "SetFileSecurityW from ADVAPI32"]
    fn SetFileSecurityW(
        &self,
        lp_file_name: PCWSTR,
        security_information: u32,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
    ) -> super::Foundation::BOOL {
        todo!("SetFileSecurityW")
    }
    #[doc = "SetKernelObjectSecurity from ADVAPI32"]
    fn SetKernelObjectSecurity(
        &self,
        handle: super::Foundation::HANDLE,
        security_information: u32,
        security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
    ) -> super::Foundation::BOOL {
        todo!("SetKernelObjectSecurity")
    }
    #[doc = "SetPrivateObjectSecurity from ADVAPI32"]
    fn SetPrivateObjectSecurity(
        &self,
        security_information: u32,
        modification_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        objects_security_descriptor: MutPtr<ConstPtr<SECURITY_DESCRIPTOR>>,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
        token: super::Foundation::HANDLE,
    ) -> super::Foundation::BOOL {
        todo!("SetPrivateObjectSecurity")
    }
    #[doc = "SetPrivateObjectSecurityEx from ADVAPI32"]
    fn SetPrivateObjectSecurityEx(
        &self,
        security_information: u32,
        modification_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        objects_security_descriptor: MutPtr<ConstPtr<SECURITY_DESCRIPTOR>>,
        auto_inherit_flags: SECURITY_AUTO_INHERIT_FLAGS,
        generic_mapping: ConstPtr<GENERIC_MAPPING>,
        token: super::Foundation::HANDLE,
    ) -> super::Foundation::BOOL {
        todo!("SetPrivateObjectSecurityEx")
    }
    #[doc = "SetSecurityAccessMask from ADVAPI32"]
    fn SetSecurityAccessMask(&self, security_information: u32, desired_access: MutPtr<u32>) {
        todo!("SetSecurityAccessMask")
    }
    #[doc = "SetSecurityDescriptorControl from ADVAPI32"]
    fn SetSecurityDescriptorControl(
        &self,
        p_security_descriptor: ConstPtr<SECURITY_DESCRIPTOR>,
        control_bits_of_interest: u16,
        control_bits_to_set: u16,
    ) -> super::Foundation::BOOL {
        todo!("SetSecurityDescriptorControl")
    }
    #[doc = "SetSecurityDescriptorDacl from ADVAPI32"]
    fn SetSecurityDescriptorDacl(
        &self,
        p_security_descriptor: MutPtr<SECURITY_DESCRIPTOR>,
        b_dacl_present: super::Foundation::BOOL,
        p_dacl: ConstPtr<ACL>,
        b_dacl_defaulted: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("SetSecurityDescriptorDacl")
    }
    #[doc = "SetSecurityDescriptorGroup from ADVAPI32"]
    fn SetSecurityDescriptorGroup(
        &self,
        p_security_descriptor: MutPtr<SECURITY_DESCRIPTOR>,
        p_group: super::Foundation::PSID,
        b_group_defaulted: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("SetSecurityDescriptorGroup")
    }
    #[doc = "SetSecurityDescriptorOwner from ADVAPI32"]
    fn SetSecurityDescriptorOwner(
        &self,
        p_security_descriptor: MutPtr<SECURITY_DESCRIPTOR>,
        p_owner: super::Foundation::PSID,
        b_owner_defaulted: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("SetSecurityDescriptorOwner")
    }
    #[doc = "SetSecurityDescriptorRMControl from ADVAPI32"]
    fn SetSecurityDescriptorRMControl(
        &self,
        security_descriptor: MutPtr<SECURITY_DESCRIPTOR>,
        rm_control: ConstPtr<u8>,
    ) -> u32 {
        todo!("SetSecurityDescriptorRMControl")
    }
    #[doc = "SetSecurityDescriptorSacl from ADVAPI32"]
    fn SetSecurityDescriptorSacl(
        &self,
        p_security_descriptor: MutPtr<SECURITY_DESCRIPTOR>,
        b_sacl_present: super::Foundation::BOOL,
        p_sacl: ConstPtr<ACL>,
        b_sacl_defaulted: super::Foundation::BOOL,
    ) -> super::Foundation::BOOL {
        todo!("SetSecurityDescriptorSacl")
    }
    #[doc = "SetTokenInformation from ADVAPI32"]
    fn SetTokenInformation(
        &self,
        token_handle: super::Foundation::HANDLE,
        token_information_class: TOKEN_INFORMATION_CLASS,
        token_information: ConstPtr<::core::ffi::c_void>,
        token_information_length: u32,
    ) -> super::Foundation::BOOL {
        todo!("SetTokenInformation")
    }
    #[doc = "SetUserObjectSecurity from USER32"]
    fn SetUserObjectSecurity(
        &self,
        h_obj: super::Foundation::HANDLE,
        p_si_requested: ConstPtr<OBJECT_SECURITY_INFORMATION>,
        p_sid: ConstPtr<SECURITY_DESCRIPTOR>,
    ) -> super::Foundation::BOOL {
        todo!("SetUserObjectSecurity")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> std::sync::Arc<dyn Api> {
    ctx.get::<dyn Api>()
}
