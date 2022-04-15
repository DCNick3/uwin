#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[allow(unused)]
use win32::core::prelude::*;
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 1u32;
pub type ENUMRESLANGPROCA = ::core::option::Option<
    unsafe extern "system" fn(
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCSTR,
        lp_name: crate::core::PCSTR,
        w_language: u16,
        l_param: PtrDiffRepr,
    ) -> super::super::Foundation::BOOL,
>;
pub type ENUMRESLANGPROCW = ::core::option::Option<
    unsafe extern "system" fn(
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCWSTR,
        lp_name: crate::core::PCWSTR,
        w_language: u16,
        l_param: PtrDiffRepr,
    ) -> super::super::Foundation::BOOL,
>;
pub type ENUMRESNAMEPROCA = ::core::option::Option<
    unsafe extern "system" fn(
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCSTR,
        lp_name: crate::core::PCSTR,
        l_param: PtrDiffRepr,
    ) -> super::super::Foundation::BOOL,
>;
pub type ENUMRESNAMEPROCW = ::core::option::Option<
    unsafe extern "system" fn(
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCWSTR,
        lp_name: crate::core::PCWSTR,
        l_param: PtrDiffRepr,
    ) -> super::super::Foundation::BOOL,
>;
pub type ENUMRESTYPEPROCA = ::core::option::Option<
    unsafe extern "system" fn(
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCSTR,
        l_param: PtrDiffRepr,
    ) -> super::super::Foundation::BOOL,
>;
pub type ENUMRESTYPEPROCW = ::core::option::Option<
    unsafe extern "system" fn(
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCWSTR,
        l_param: PtrDiffRepr,
    ) -> super::super::Foundation::BOOL,
>;
pub struct ENUMUILANG {
    pub NumOfEnumUILang: u32,
    pub SizeOfEnumUIBuffer: u32,
    pub pEnumUIBuffer: MutPtr<u16>,
}
impl ::core::marker::Copy for ENUMUILANG {}
impl ::core::clone::Clone for ENUMUILANG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMUILANG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMUILANG")
            .field("NumOfEnumUILang", &self.NumOfEnumUILang)
            .field("SizeOfEnumUIBuffer", &self.SizeOfEnumUIBuffer)
            .field("pEnumUIBuffer", &self.pEnumUIBuffer)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ENUMUILANG {
    fn eq(&self, other: &Self) -> bool {
        self.NumOfEnumUILang == other.NumOfEnumUILang
            && self.SizeOfEnumUIBuffer == other.SizeOfEnumUIBuffer
            && self.pEnumUIBuffer == other.pEnumUIBuffer
    }
}
impl ::core::cmp::Eq for ENUMUILANG {}
impl FromIntoMemory for ENUMUILANG {
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
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 1024u32;
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 512u32;
pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 256u32;
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4u32;
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1u32;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LOAD_LIBRARY_FLAGS(pub u32);
pub const DONT_RESOLVE_DLL_REFERENCES: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(1u32);
pub const LOAD_LIBRARY_AS_DATAFILE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(2u32);
pub const LOAD_WITH_ALTERED_SEARCH_PATH: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(8u32);
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(16u32);
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(32u32);
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(64u32);
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(128u32);
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(256u32);
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(512u32);
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(1024u32);
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(2048u32);
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(4096u32);
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(8192u32);
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: LOAD_LIBRARY_FLAGS =
    LOAD_LIBRARY_FLAGS(16384u32);
impl ::core::marker::Copy for LOAD_LIBRARY_FLAGS {}
impl ::core::clone::Clone for LOAD_LIBRARY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOAD_LIBRARY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOAD_LIBRARY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOAD_LIBRARY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LOAD_LIBRARY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LOAD_LIBRARY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl FromIntoMemory for LOAD_LIBRARY_FLAGS {
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
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 32768u32;
pub type PGET_MODULE_HANDLE_EXA = ::core::option::Option<
    unsafe extern "system" fn(
        dw_flags: u32,
        lp_module_name: crate::core::PCSTR,
        ph_module: MutPtr<super::super::Foundation::HINSTANCE>,
    ) -> super::super::Foundation::BOOL,
>;
pub type PGET_MODULE_HANDLE_EXW = ::core::option::Option<
    unsafe extern "system" fn(
        dw_flags: u32,
        lp_module_name: crate::core::PCWSTR,
        ph_module: MutPtr<super::super::Foundation::HINSTANCE>,
    ) -> super::super::Foundation::BOOL,
>;
pub struct REDIRECTION_DESCRIPTOR {
    pub Version: u32,
    pub FunctionCount: u32,
    pub Redirections: MutPtr<REDIRECTION_FUNCTION_DESCRIPTOR>,
}
impl ::core::marker::Copy for REDIRECTION_DESCRIPTOR {}
impl ::core::clone::Clone for REDIRECTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REDIRECTION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REDIRECTION_DESCRIPTOR")
            .field("Version", &self.Version)
            .field("FunctionCount", &self.FunctionCount)
            .field("Redirections", &self.Redirections)
            .finish()
    }
}
impl ::core::cmp::PartialEq for REDIRECTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.FunctionCount == other.FunctionCount
            && self.Redirections == other.Redirections
    }
}
impl ::core::cmp::Eq for REDIRECTION_DESCRIPTOR {}
impl FromIntoMemory for REDIRECTION_DESCRIPTOR {
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
pub struct REDIRECTION_FUNCTION_DESCRIPTOR {
    pub DllName: crate::core::PCSTR,
    pub FunctionName: crate::core::PCSTR,
    pub RedirectionTarget: MutPtr<::core::ffi::c_void>,
}
impl ::core::marker::Copy for REDIRECTION_FUNCTION_DESCRIPTOR {}
impl ::core::clone::Clone for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REDIRECTION_FUNCTION_DESCRIPTOR")
            .field("DllName", &self.DllName)
            .field("FunctionName", &self.FunctionName)
            .field("RedirectionTarget", &self.RedirectionTarget)
            .finish()
    }
}
impl ::core::cmp::PartialEq for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.DllName == other.DllName
            && self.FunctionName == other.FunctionName
            && self.RedirectionTarget == other.RedirectionTarget
    }
}
impl ::core::cmp::Eq for REDIRECTION_FUNCTION_DESCRIPTOR {}
impl FromIntoMemory for REDIRECTION_FUNCTION_DESCRIPTOR {
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
pub const RESOURCE_ENUM_LN: u32 = 1u32;
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 16u32;
pub const RESOURCE_ENUM_MUI: u32 = 2u32;
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 4u32;
pub const RESOURCE_ENUM_VALIDATE: u32 = 8u32;
pub const SUPPORT_LANG_NUMBER: u32 = 32u32;
pub trait Api {
    fn AddDllDirectory(&self, new_directory: crate::core::PCWSTR) -> MutPtr<::core::ffi::c_void> {
        todo!("AddDllDirectory")
    }
    fn BeginUpdateResourceA(
        &self,
        p_file_name: crate::core::PCSTR,
        b_delete_existing_resources: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::HANDLE {
        todo!("BeginUpdateResourceA")
    }
    fn BeginUpdateResourceW(
        &self,
        p_file_name: crate::core::PCWSTR,
        b_delete_existing_resources: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::HANDLE {
        todo!("BeginUpdateResourceW")
    }
    fn DisableThreadLibraryCalls(
        &self,
        h_lib_module: super::super::Foundation::HINSTANCE,
    ) -> super::super::Foundation::BOOL {
        todo!("DisableThreadLibraryCalls")
    }
    fn EndUpdateResourceA(
        &self,
        h_update: super::super::Foundation::HANDLE,
        f_discard: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("EndUpdateResourceA")
    }
    fn EndUpdateResourceW(
        &self,
        h_update: super::super::Foundation::HANDLE,
        f_discard: super::super::Foundation::BOOL,
    ) -> super::super::Foundation::BOOL {
        todo!("EndUpdateResourceW")
    }
    fn EnumResourceLanguagesA(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCSTR,
        lp_name: crate::core::PCSTR,
        lp_enum_func: ENUMRESLANGPROCA,
        l_param: PtrDiffRepr,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumResourceLanguagesA")
    }
    fn EnumResourceLanguagesExA(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCSTR,
        lp_name: crate::core::PCSTR,
        lp_enum_func: ENUMRESLANGPROCA,
        l_param: PtrDiffRepr,
        dw_flags: u32,
        lang_id: u16,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumResourceLanguagesExA")
    }
    fn EnumResourceLanguagesExW(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCWSTR,
        lp_name: crate::core::PCWSTR,
        lp_enum_func: ENUMRESLANGPROCW,
        l_param: PtrDiffRepr,
        dw_flags: u32,
        lang_id: u16,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumResourceLanguagesExW")
    }
    fn EnumResourceLanguagesW(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCWSTR,
        lp_name: crate::core::PCWSTR,
        lp_enum_func: ENUMRESLANGPROCW,
        l_param: PtrDiffRepr,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumResourceLanguagesW")
    }
    fn EnumResourceNamesA(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCSTR,
        lp_enum_func: ENUMRESNAMEPROCA,
        l_param: PtrDiffRepr,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumResourceNamesA")
    }
    fn EnumResourceNamesExA(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCSTR,
        lp_enum_func: ENUMRESNAMEPROCA,
        l_param: PtrDiffRepr,
        dw_flags: u32,
        lang_id: u16,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumResourceNamesExA")
    }
    fn EnumResourceNamesExW(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCWSTR,
        lp_enum_func: ENUMRESNAMEPROCW,
        l_param: PtrDiffRepr,
        dw_flags: u32,
        lang_id: u16,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumResourceNamesExW")
    }
    fn EnumResourceNamesW(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCWSTR,
        lp_enum_func: ENUMRESNAMEPROCW,
        l_param: PtrDiffRepr,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumResourceNamesW")
    }
    fn EnumResourceTypesA(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_enum_func: ENUMRESTYPEPROCA,
        l_param: PtrDiffRepr,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumResourceTypesA")
    }
    fn EnumResourceTypesExA(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_enum_func: ENUMRESTYPEPROCA,
        l_param: PtrDiffRepr,
        dw_flags: u32,
        lang_id: u16,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumResourceTypesExA")
    }
    fn EnumResourceTypesExW(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_enum_func: ENUMRESTYPEPROCW,
        l_param: PtrDiffRepr,
        dw_flags: u32,
        lang_id: u16,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumResourceTypesExW")
    }
    fn EnumResourceTypesW(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_enum_func: ENUMRESTYPEPROCW,
        l_param: PtrDiffRepr,
    ) -> super::super::Foundation::BOOL {
        todo!("EnumResourceTypesW")
    }
    fn FindResourceA(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_name: crate::core::PCSTR,
        lp_type: crate::core::PCSTR,
    ) -> super::super::Foundation::HRSRC {
        todo!("FindResourceA")
    }
    fn FindResourceExA(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCSTR,
        lp_name: crate::core::PCSTR,
        w_language: u16,
    ) -> super::super::Foundation::HRSRC {
        todo!("FindResourceExA")
    }
    fn FindResourceExW(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_type: crate::core::PCWSTR,
        lp_name: crate::core::PCWSTR,
        w_language: u16,
    ) -> super::super::Foundation::HRSRC {
        todo!("FindResourceExW")
    }
    fn FindResourceW(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_name: crate::core::PCWSTR,
        lp_type: crate::core::PCWSTR,
    ) -> super::super::Foundation::HRSRC {
        todo!("FindResourceW")
    }
    fn FreeLibrary(
        &self,
        h_lib_module: super::super::Foundation::HINSTANCE,
    ) -> super::super::Foundation::BOOL {
        todo!("FreeLibrary")
    }
    fn FreeLibraryAndExitThread(
        &self,
        h_lib_module: super::super::Foundation::HINSTANCE,
        dw_exit_code: u32,
    ) {
        todo!("FreeLibraryAndExitThread")
    }
    fn FreeResource(&self, h_res_data: PtrDiffRepr) -> super::super::Foundation::BOOL {
        todo!("FreeResource")
    }
    fn GetDllDirectoryA(&self, n_buffer_length: u32, lp_buffer: crate::core::PSTR) -> u32 {
        todo!("GetDllDirectoryA")
    }
    fn GetDllDirectoryW(&self, n_buffer_length: u32, lp_buffer: crate::core::PWSTR) -> u32 {
        todo!("GetDllDirectoryW")
    }
    fn GetModuleFileNameA(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_filename: crate::core::PSTR,
        n_size: u32,
    ) -> u32 {
        todo!("GetModuleFileNameA")
    }
    fn GetModuleFileNameW(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_filename: crate::core::PWSTR,
        n_size: u32,
    ) -> u32 {
        todo!("GetModuleFileNameW")
    }
    fn GetModuleHandleA(
        &self,
        lp_module_name: crate::core::PCSTR,
    ) -> super::super::Foundation::HINSTANCE {
        todo!("GetModuleHandleA")
    }
    fn GetModuleHandleExA(
        &self,
        dw_flags: u32,
        lp_module_name: crate::core::PCSTR,
        ph_module: MutPtr<super::super::Foundation::HINSTANCE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetModuleHandleExA")
    }
    fn GetModuleHandleExW(
        &self,
        dw_flags: u32,
        lp_module_name: crate::core::PCWSTR,
        ph_module: MutPtr<super::super::Foundation::HINSTANCE>,
    ) -> super::super::Foundation::BOOL {
        todo!("GetModuleHandleExW")
    }
    fn GetModuleHandleW(
        &self,
        lp_module_name: crate::core::PCWSTR,
    ) -> super::super::Foundation::HINSTANCE {
        todo!("GetModuleHandleW")
    }
    fn GetProcAddress(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        lp_proc_name: crate::core::PCSTR,
    ) -> super::super::Foundation::FARPROC {
        todo!("GetProcAddress")
    }
    fn LoadLibraryA(
        &self,
        lp_lib_file_name: crate::core::PCSTR,
    ) -> super::super::Foundation::HINSTANCE {
        todo!("LoadLibraryA")
    }
    fn LoadLibraryExA(
        &self,
        lp_lib_file_name: crate::core::PCSTR,
        h_file: super::super::Foundation::HANDLE,
        dw_flags: LOAD_LIBRARY_FLAGS,
    ) -> super::super::Foundation::HINSTANCE {
        todo!("LoadLibraryExA")
    }
    fn LoadLibraryExW(
        &self,
        lp_lib_file_name: crate::core::PCWSTR,
        h_file: super::super::Foundation::HANDLE,
        dw_flags: LOAD_LIBRARY_FLAGS,
    ) -> super::super::Foundation::HINSTANCE {
        todo!("LoadLibraryExW")
    }
    fn LoadLibraryW(
        &self,
        lp_lib_file_name: crate::core::PCWSTR,
    ) -> super::super::Foundation::HINSTANCE {
        todo!("LoadLibraryW")
    }
    fn LoadModule(
        &self,
        lp_module_name: crate::core::PCSTR,
        lp_parameter_block: ConstPtr<::core::ffi::c_void>,
    ) -> u32 {
        todo!("LoadModule")
    }
    fn LoadPackagedLibrary(
        &self,
        lpw_lib_file_name: crate::core::PCWSTR,
        reserved: u32,
    ) -> super::super::Foundation::HINSTANCE {
        todo!("LoadPackagedLibrary")
    }
    fn LoadResource(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        h_res_info: super::super::Foundation::HRSRC,
    ) -> PtrDiffRepr {
        todo!("LoadResource")
    }
    fn LockResource(&self, h_res_data: PtrDiffRepr) -> MutPtr<::core::ffi::c_void> {
        todo!("LockResource")
    }
    fn RemoveDllDirectory(
        &self,
        cookie: ConstPtr<::core::ffi::c_void>,
    ) -> super::super::Foundation::BOOL {
        todo!("RemoveDllDirectory")
    }
    fn SetDefaultDllDirectories(
        &self,
        directory_flags: LOAD_LIBRARY_FLAGS,
    ) -> super::super::Foundation::BOOL {
        todo!("SetDefaultDllDirectories")
    }
    fn SetDllDirectoryA(&self, lp_path_name: crate::core::PCSTR) -> super::super::Foundation::BOOL {
        todo!("SetDllDirectoryA")
    }
    fn SetDllDirectoryW(
        &self,
        lp_path_name: crate::core::PCWSTR,
    ) -> super::super::Foundation::BOOL {
        todo!("SetDllDirectoryW")
    }
    fn SizeofResource(
        &self,
        h_module: super::super::Foundation::HINSTANCE,
        h_res_info: super::super::Foundation::HRSRC,
    ) -> u32 {
        todo!("SizeofResource")
    }
    fn UpdateResourceA(
        &self,
        h_update: super::super::Foundation::HANDLE,
        lp_type: crate::core::PCSTR,
        lp_name: crate::core::PCSTR,
        w_language: u16,
        lp_data: ConstPtr<::core::ffi::c_void>,
        cb: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("UpdateResourceA")
    }
    fn UpdateResourceW(
        &self,
        h_update: super::super::Foundation::HANDLE,
        lp_type: crate::core::PCWSTR,
        lp_name: crate::core::PCWSTR,
        w_language: u16,
        lp_data: ConstPtr<::core::ffi::c_void>,
        cb: u32,
    ) -> super::super::Foundation::BOOL {
        todo!("UpdateResourceW")
    }
}
pub fn get_api(ctx: &crate::core::Win32Context) -> &dyn Api {
    ctx.get::<dyn Api>()
}
