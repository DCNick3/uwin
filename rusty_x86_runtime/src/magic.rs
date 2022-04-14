#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all,
    unused_mut
)]
#[allow(unused)]
use crate::ExtendedContext;
#[allow(unused)]
use core_abi::stdcall::StdCallHelper;
#[allow(unused)]
use core_mem::ctx::FlatMemoryCtx;
#[no_mangle]
extern "C" fn magic_CloseHandle(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Foundation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hObject = call.get_arg();
        let res = api.CloseHandle(hObject);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CompareObjectHandles(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Foundation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hFirstObjectHandle = call.get_arg();
        let hSecondObjectHandle = call.get_arg();
        let res = api.CompareObjectHandles(hFirstObjectHandle, hSecondObjectHandle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DuplicateHandle(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Foundation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hSourceProcessHandle = call.get_arg();
        let hSourceHandle = call.get_arg();
        let hTargetProcessHandle = call.get_arg();
        let lpTargetHandle = call.get_arg();
        let dwDesiredAccess = call.get_arg();
        let bInheritHandle = call.get_arg();
        let dwOptions = call.get_arg();
        let res = api.DuplicateHandle(
            hSourceProcessHandle,
            hSourceHandle,
            hTargetProcessHandle,
            lpTargetHandle,
            dwDesiredAccess,
            bInheritHandle,
            dwOptions,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetHandleInformation(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Foundation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hObject = call.get_arg();
        let lpdwFlags = call.get_arg();
        let res = api.GetHandleInformation(hObject, lpdwFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLastError(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Foundation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetLastError();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetHandleInformation(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Foundation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hObject = call.get_arg();
        let dwMask = call.get_arg();
        let dwFlags = call.get_arg();
        let res = api.SetHandleInformation(hObject, dwMask, dwFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetLastError(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Foundation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dwErrCode = call.get_arg();
        let res = api.SetLastError(dwErrCode);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetComputerNameExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let NameType = call.get_arg();
        let lpBuffer = call.get_arg();
        let nSize = call.get_arg();
        let res = api.GetComputerNameExA(NameType, lpBuffer, nSize);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetComputerNameExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let NameType = call.get_arg();
        let lpBuffer = call.get_arg();
        let nSize = call.get_arg();
        let res = api.GetComputerNameExW(NameType, lpBuffer, nSize);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLocalTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpSystemTime = call.get_arg();
        let res = api.GetLocalTime(lpSystemTime);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLogicalProcessorInformation(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let Buffer = call.get_arg();
        let ReturnedLength = call.get_arg();
        let res = api.GetLogicalProcessorInformation(Buffer, ReturnedLength);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLogicalProcessorInformationEx(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let RelationshipType = call.get_arg();
        let Buffer = call.get_arg();
        let ReturnedLength = call.get_arg();
        let res = api.GetLogicalProcessorInformationEx(RelationshipType, Buffer, ReturnedLength);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetNativeSystemInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpSystemInfo = call.get_arg();
        let res = api.GetNativeSystemInfo(lpSystemInfo);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetOsManufacturingMode(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pbEnabled = call.get_arg();
        let res = api.GetOsManufacturingMode(pbEnabled);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetOsSafeBootMode(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let Flags = call.get_arg();
        let res = api.GetOsSafeBootMode(Flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetPhysicallyInstalledSystemMemory(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let TotalMemoryInKilobytes = call.get_arg();
        let res = api.GetPhysicallyInstalledSystemMemory(TotalMemoryInKilobytes);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetProcessorSystemCycleTime(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let Group = call.get_arg();
        let Buffer = call.get_arg();
        let ReturnedLength = call.get_arg();
        let res = api.GetProcessorSystemCycleTime(Group, Buffer, ReturnedLength);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetProductInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dwOSMajorVersion = call.get_arg();
        let dwOSMinorVersion = call.get_arg();
        let dwSpMajorVersion = call.get_arg();
        let dwSpMinorVersion = call.get_arg();
        let pdwReturnedProductType = call.get_arg();
        let res = api.GetProductInfo(
            dwOSMajorVersion,
            dwOSMinorVersion,
            dwSpMajorVersion,
            dwSpMinorVersion,
            pdwReturnedProductType,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemCpuSetInformation(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let Information = call.get_arg();
        let BufferLength = call.get_arg();
        let ReturnedLength = call.get_arg();
        let Process = call.get_arg();
        let Flags = call.get_arg();
        let res = api.GetSystemCpuSetInformation(
            Information,
            BufferLength,
            ReturnedLength,
            Process,
            Flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemDEPPolicy(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetSystemDEPPolicy();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemDirectoryA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpBuffer = call.get_arg();
        let uSize = call.get_arg();
        let res = api.GetSystemDirectoryA(lpBuffer, uSize);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemDirectoryW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpBuffer = call.get_arg();
        let uSize = call.get_arg();
        let res = api.GetSystemDirectoryW(lpBuffer, uSize);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemFirmwareTable(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let FirmwareTableProviderSignature = call.get_arg();
        let FirmwareTableID = call.get_arg();
        let pFirmwareTableBuffer = call.get_arg();
        let BufferSize = call.get_arg();
        let res = api.GetSystemFirmwareTable(
            FirmwareTableProviderSignature,
            FirmwareTableID,
            pFirmwareTableBuffer,
            BufferSize,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpSystemInfo = call.get_arg();
        let res = api.GetSystemInfo(lpSystemInfo);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemLeapSecondInformation(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let Enabled = call.get_arg();
        let Flags = call.get_arg();
        let res = api.GetSystemLeapSecondInformation(Enabled, Flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpSystemTime = call.get_arg();
        let res = api.GetSystemTime(lpSystemTime);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemTimeAdjustment(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpTimeAdjustment = call.get_arg();
        let lpTimeIncrement = call.get_arg();
        let lpTimeAdjustmentDisabled = call.get_arg();
        let res = api.GetSystemTimeAdjustment(
            lpTimeAdjustment,
            lpTimeIncrement,
            lpTimeAdjustmentDisabled,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemTimeAdjustmentPrecise(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpTimeAdjustment = call.get_arg();
        let lpTimeIncrement = call.get_arg();
        let lpTimeAdjustmentDisabled = call.get_arg();
        let res = api.GetSystemTimeAdjustmentPrecise(
            lpTimeAdjustment,
            lpTimeIncrement,
            lpTimeAdjustmentDisabled,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemTimeAsFileTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpSystemTimeAsFileTime = call.get_arg();
        let res = api.GetSystemTimeAsFileTime(lpSystemTimeAsFileTime);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemTimePreciseAsFileTime(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpSystemTimeAsFileTime = call.get_arg();
        let res = api.GetSystemTimePreciseAsFileTime(lpSystemTimeAsFileTime);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemWindowsDirectoryA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpBuffer = call.get_arg();
        let uSize = call.get_arg();
        let res = api.GetSystemWindowsDirectoryA(lpBuffer, uSize);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemWindowsDirectoryW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpBuffer = call.get_arg();
        let uSize = call.get_arg();
        let res = api.GetSystemWindowsDirectoryW(lpBuffer, uSize);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemWow64Directory2A(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpBuffer = call.get_arg();
        let uSize = call.get_arg();
        let ImageFileMachineType = call.get_arg();
        let res = api.GetSystemWow64Directory2A(lpBuffer, uSize, ImageFileMachineType);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemWow64Directory2W(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpBuffer = call.get_arg();
        let uSize = call.get_arg();
        let ImageFileMachineType = call.get_arg();
        let res = api.GetSystemWow64Directory2W(lpBuffer, uSize, ImageFileMachineType);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemWow64DirectoryA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpBuffer = call.get_arg();
        let uSize = call.get_arg();
        let res = api.GetSystemWow64DirectoryA(lpBuffer, uSize);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemWow64DirectoryW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpBuffer = call.get_arg();
        let uSize = call.get_arg();
        let res = api.GetSystemWow64DirectoryW(lpBuffer, uSize);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTickCount(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetTickCount();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTickCount64(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetTickCount64();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVersion(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetVersion();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVersionExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpVersionInformation = call.get_arg();
        let res = api.GetVersionExA(lpVersionInformation);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVersionExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpVersionInformation = call.get_arg();
        let res = api.GetVersionExW(lpVersionInformation);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowsDirectoryA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpBuffer = call.get_arg();
        let uSize = call.get_arg();
        let res = api.GetWindowsDirectoryA(lpBuffer, uSize);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowsDirectoryW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpBuffer = call.get_arg();
        let uSize = call.get_arg();
        let res = api.GetWindowsDirectoryW(lpBuffer, uSize);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalMemoryStatus(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpBuffer = call.get_arg();
        let res = api.GlobalMemoryStatus(lpBuffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalMemoryStatusEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpBuffer = call.get_arg();
        let res = api.GlobalMemoryStatusEx(lpBuffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlConvertDeviceFamilyInfoToString(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pulDeviceFamilyBufferSize = call.get_arg();
        let pulDeviceFormBufferSize = call.get_arg();
        let DeviceFamily = call.get_arg();
        let DeviceForm = call.get_arg();
        let res = api.RtlConvertDeviceFamilyInfoToString(
            pulDeviceFamilyBufferSize,
            pulDeviceFormBufferSize,
            DeviceFamily,
            DeviceForm,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlGetDeviceFamilyInfoEnum(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pullUAPInfo = call.get_arg();
        let pulDeviceFamily = call.get_arg();
        let pulDeviceForm = call.get_arg();
        let res = api.RtlGetDeviceFamilyInfoEnum(pullUAPInfo, pulDeviceFamily, pulDeviceForm);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlGetProductInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let OSMajorVersion = call.get_arg();
        let OSMinorVersion = call.get_arg();
        let SpMajorVersion = call.get_arg();
        let SpMinorVersion = call.get_arg();
        let ReturnedProductType = call.get_arg();
        let res = api.RtlGetProductInfo(
            OSMajorVersion,
            OSMinorVersion,
            SpMajorVersion,
            SpMinorVersion,
            ReturnedProductType,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlGetSystemGlobalData(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let DataId = call.get_arg();
        let Buffer = call.get_arg();
        let Size = call.get_arg();
        let res = api.RtlGetSystemGlobalData(DataId, Buffer, Size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlOsDeploymentState(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let Flags = call.get_arg();
        let res = api.RtlOsDeploymentState(Flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlSwitchedVVI(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let VersionInfo = call.get_arg();
        let TypeMask = call.get_arg();
        let ConditionMask = call.get_arg();
        let res = api.RtlSwitchedVVI(VersionInfo, TypeMask, ConditionMask);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetComputerNameA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpComputerName = call.get_arg();
        let res = api.SetComputerNameA(lpComputerName);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetComputerNameEx2W(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let NameType = call.get_arg();
        let Flags = call.get_arg();
        let lpBuffer = call.get_arg();
        let res = api.SetComputerNameEx2W(NameType, Flags, lpBuffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetComputerNameExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let NameType = call.get_arg();
        let lpBuffer = call.get_arg();
        let res = api.SetComputerNameExA(NameType, lpBuffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetComputerNameExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let NameType = call.get_arg();
        let lpBuffer = call.get_arg();
        let res = api.SetComputerNameExW(NameType, lpBuffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetComputerNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpComputerName = call.get_arg();
        let res = api.SetComputerNameW(lpComputerName);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetLocalTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpSystemTime = call.get_arg();
        let res = api.SetLocalTime(lpSystemTime);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetSystemTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpSystemTime = call.get_arg();
        let res = api.SetSystemTime(lpSystemTime);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetSystemTimeAdjustment(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dwTimeAdjustment = call.get_arg();
        let bTimeAdjustmentDisabled = call.get_arg();
        let res = api.SetSystemTimeAdjustment(dwTimeAdjustment, bTimeAdjustmentDisabled);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetSystemTimeAdjustmentPrecise(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dwTimeAdjustment = call.get_arg();
        let bTimeAdjustmentDisabled = call.get_arg();
        let res = api.SetSystemTimeAdjustmentPrecise(dwTimeAdjustment, bTimeAdjustmentDisabled);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VerSetConditionMask(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ConditionMask = call.get_arg();
        let TypeMask = call.get_arg();
        let Condition = call.get_arg();
        let res = api.VerSetConditionMask(ConditionMask, TypeMask, Condition);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VerifyVersionInfoA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpVersionInformation = call.get_arg();
        let dwTypeMask = call.get_arg();
        let dwlConditionMask = call.get_arg();
        let res = api.VerifyVersionInfoA(lpVersionInformation, dwTypeMask, dwlConditionMask);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VerifyVersionInfoW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::SystemInformation::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpVersionInformation = call.get_arg();
        let dwTypeMask = call.get_arg();
        let dwlConditionMask = call.get_arg();
        let res = api.VerifyVersionInfoW(lpVersionInformation, dwTypeMask, dwlConditionMask);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AdjustWindowRect(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpRect = call.get_arg();
        let dwStyle = call.get_arg();
        let bMenu = call.get_arg();
        let res = api.AdjustWindowRect(lpRect, dwStyle, bMenu);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AdjustWindowRectEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpRect = call.get_arg();
        let dwStyle = call.get_arg();
        let bMenu = call.get_arg();
        let dwExStyle = call.get_arg();
        let res = api.AdjustWindowRectEx(lpRect, dwStyle, bMenu, dwExStyle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AllowSetForegroundWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dwProcessId = call.get_arg();
        let res = api.AllowSetForegroundWindow(dwProcessId);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AnimateWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let dwTime = call.get_arg();
        let dwFlags = call.get_arg();
        let res = api.AnimateWindow(hWnd, dwTime, dwFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AnyPopup(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.AnyPopup();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AppendMenuA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let uFlags = call.get_arg();
        let uIDNewItem = call.get_arg();
        let lpNewItem = call.get_arg();
        let res = api.AppendMenuA(hMenu, uFlags, uIDNewItem, lpNewItem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AppendMenuW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let uFlags = call.get_arg();
        let uIDNewItem = call.get_arg();
        let lpNewItem = call.get_arg();
        let res = api.AppendMenuW(hMenu, uFlags, uIDNewItem, lpNewItem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ArrangeIconicWindows(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.ArrangeIconicWindows(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_BeginDeferWindowPos(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let nNumWindows = call.get_arg();
        let res = api.BeginDeferWindowPos(nNumWindows);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_BringWindowToTop(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.BringWindowToTop(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CalculatePopupWindowPosition(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let anchorPoint = call.get_arg();
        let windowSize = call.get_arg();
        let flags = call.get_arg();
        let excludeRect = call.get_arg();
        let popupWindowPosition = call.get_arg();
        let res = api.CalculatePopupWindowPosition(
            anchorPoint,
            windowSize,
            flags,
            excludeRect,
            popupWindowPosition,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CallMsgFilterA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpMsg = call.get_arg();
        let nCode = call.get_arg();
        let res = api.CallMsgFilterA(lpMsg, nCode);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CallMsgFilterW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpMsg = call.get_arg();
        let nCode = call.get_arg();
        let res = api.CallMsgFilterW(lpMsg, nCode);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CallNextHookEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hhk = call.get_arg();
        let nCode = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.CallNextHookEx(hhk, nCode, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CallWindowProcA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpPrevWndFunc = call.get_arg();
        let hWnd = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.CallWindowProcA(lpPrevWndFunc, hWnd, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CallWindowProcW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpPrevWndFunc = call.get_arg();
        let hWnd = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.CallWindowProcW(lpPrevWndFunc, hWnd, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CancelShutdown(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.CancelShutdown();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CascadeWindows(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwndParent = call.get_arg();
        let wHow = call.get_arg();
        let lpRect = call.get_arg();
        let cKids = call.get_arg();
        let lpKids = call.get_arg();
        let res = api.CascadeWindows(hwndParent, wHow, lpRect, cKids, lpKids);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ChangeMenuA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let cmd = call.get_arg();
        let lpszNewItem = call.get_arg();
        let cmdInsert = call.get_arg();
        let flags = call.get_arg();
        let res = api.ChangeMenuA(hMenu, cmd, lpszNewItem, cmdInsert, flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ChangeMenuW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let cmd = call.get_arg();
        let lpszNewItem = call.get_arg();
        let cmdInsert = call.get_arg();
        let flags = call.get_arg();
        let res = api.ChangeMenuW(hMenu, cmd, lpszNewItem, cmdInsert, flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ChangeWindowMessageFilter(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let message = call.get_arg();
        let dwFlag = call.get_arg();
        let res = api.ChangeWindowMessageFilter(message, dwFlag);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ChangeWindowMessageFilterEx(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let message = call.get_arg();
        let action = call.get_arg();
        let pChangeFilterStruct = call.get_arg();
        let res = api.ChangeWindowMessageFilterEx(hwnd, message, action, pChangeFilterStruct);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharLowerA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz = call.get_arg();
        let res = api.CharLowerA(lpsz);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharLowerBuffA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz = call.get_arg();
        let cchLength = call.get_arg();
        let res = api.CharLowerBuffA(lpsz, cchLength);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharLowerBuffW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz = call.get_arg();
        let cchLength = call.get_arg();
        let res = api.CharLowerBuffW(lpsz, cchLength);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharLowerW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz = call.get_arg();
        let res = api.CharLowerW(lpsz);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharNextA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz = call.get_arg();
        let res = api.CharNextA(lpsz);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharNextExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let CodePage = call.get_arg();
        let lpCurrentChar = call.get_arg();
        let dwFlags = call.get_arg();
        let res = api.CharNextExA(CodePage, lpCurrentChar, dwFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharNextW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz = call.get_arg();
        let res = api.CharNextW(lpsz);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharPrevA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpszStart = call.get_arg();
        let lpszCurrent = call.get_arg();
        let res = api.CharPrevA(lpszStart, lpszCurrent);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharPrevExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let CodePage = call.get_arg();
        let lpStart = call.get_arg();
        let lpCurrentChar = call.get_arg();
        let dwFlags = call.get_arg();
        let res = api.CharPrevExA(CodePage, lpStart, lpCurrentChar, dwFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharPrevW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpszStart = call.get_arg();
        let lpszCurrent = call.get_arg();
        let res = api.CharPrevW(lpszStart, lpszCurrent);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharToOemA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pSrc = call.get_arg();
        let pDst = call.get_arg();
        let res = api.CharToOemA(pSrc, pDst);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharToOemBuffA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpszSrc = call.get_arg();
        let lpszDst = call.get_arg();
        let cchDstLength = call.get_arg();
        let res = api.CharToOemBuffA(lpszSrc, lpszDst, cchDstLength);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharToOemBuffW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpszSrc = call.get_arg();
        let lpszDst = call.get_arg();
        let cchDstLength = call.get_arg();
        let res = api.CharToOemBuffW(lpszSrc, lpszDst, cchDstLength);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharToOemW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pSrc = call.get_arg();
        let pDst = call.get_arg();
        let res = api.CharToOemW(pSrc, pDst);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharUpperA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz = call.get_arg();
        let res = api.CharUpperA(lpsz);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharUpperBuffA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz = call.get_arg();
        let cchLength = call.get_arg();
        let res = api.CharUpperBuffA(lpsz, cchLength);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharUpperBuffW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz = call.get_arg();
        let cchLength = call.get_arg();
        let res = api.CharUpperBuffW(lpsz, cchLength);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CharUpperW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz = call.get_arg();
        let res = api.CharUpperW(lpsz);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CheckMenuItem(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let uIDCheckItem = call.get_arg();
        let uCheck = call.get_arg();
        let res = api.CheckMenuItem(hMenu, uIDCheckItem, uCheck);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CheckMenuRadioItem(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hmenu = call.get_arg();
        let first = call.get_arg();
        let last = call.get_arg();
        let check = call.get_arg();
        let flags = call.get_arg();
        let res = api.CheckMenuRadioItem(hmenu, first, last, check, flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ChildWindowFromPoint(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWndParent = call.get_arg();
        let Point = call.get_arg();
        let res = api.ChildWindowFromPoint(hWndParent, Point);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ChildWindowFromPointEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let pt = call.get_arg();
        let flags = call.get_arg();
        let res = api.ChildWindowFromPointEx(hwnd, pt, flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ClipCursor(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpRect = call.get_arg();
        let res = api.ClipCursor(lpRect);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CloseWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.CloseWindow(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CopyAcceleratorTableA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hAccelSrc = call.get_arg();
        let lpAccelDst = call.get_arg();
        let cAccelEntries = call.get_arg();
        let res = api.CopyAcceleratorTableA(hAccelSrc, lpAccelDst, cAccelEntries);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CopyAcceleratorTableW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hAccelSrc = call.get_arg();
        let lpAccelDst = call.get_arg();
        let cAccelEntries = call.get_arg();
        let res = api.CopyAcceleratorTableW(hAccelSrc, lpAccelDst, cAccelEntries);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CopyIcon(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hIcon = call.get_arg();
        let res = api.CopyIcon(hIcon);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CopyImage(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h = call.get_arg();
        let r#type = call.get_arg();
        let cx = call.get_arg();
        let cy = call.get_arg();
        let flags = call.get_arg();
        let res = api.CopyImage(h, r#type, cx, cy, flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateAcceleratorTableA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let paccel = call.get_arg();
        let cAccel = call.get_arg();
        let res = api.CreateAcceleratorTableA(paccel, cAccel);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateAcceleratorTableW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let paccel = call.get_arg();
        let cAccel = call.get_arg();
        let res = api.CreateAcceleratorTableW(paccel, cAccel);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateCursor(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInst = call.get_arg();
        let xHotSpot = call.get_arg();
        let yHotSpot = call.get_arg();
        let nWidth = call.get_arg();
        let nHeight = call.get_arg();
        let pvANDPlane = call.get_arg();
        let pvXORPlane = call.get_arg();
        let res = api.CreateCursor(
            hInst, xHotSpot, yHotSpot, nWidth, nHeight, pvANDPlane, pvXORPlane,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateDialogIndirectParamA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let lpTemplate = call.get_arg();
        let hWndParent = call.get_arg();
        let lpDialogFunc = call.get_arg();
        let dwInitParam = call.get_arg();
        let res = api.CreateDialogIndirectParamA(
            hInstance,
            lpTemplate,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateDialogIndirectParamW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let lpTemplate = call.get_arg();
        let hWndParent = call.get_arg();
        let lpDialogFunc = call.get_arg();
        let dwInitParam = call.get_arg();
        let res = api.CreateDialogIndirectParamW(
            hInstance,
            lpTemplate,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateDialogParamA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let lpTemplateName = call.get_arg();
        let hWndParent = call.get_arg();
        let lpDialogFunc = call.get_arg();
        let dwInitParam = call.get_arg();
        let res = api.CreateDialogParamA(
            hInstance,
            lpTemplateName,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateDialogParamW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let lpTemplateName = call.get_arg();
        let hWndParent = call.get_arg();
        let lpDialogFunc = call.get_arg();
        let dwInitParam = call.get_arg();
        let res = api.CreateDialogParamW(
            hInstance,
            lpTemplateName,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateIcon(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let nWidth = call.get_arg();
        let nHeight = call.get_arg();
        let cPlanes = call.get_arg();
        let cBitsPixel = call.get_arg();
        let lpbANDbits = call.get_arg();
        let lpbXORbits = call.get_arg();
        let res = api.CreateIcon(
            hInstance, nWidth, nHeight, cPlanes, cBitsPixel, lpbANDbits, lpbXORbits,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateIconFromResource(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let presbits = call.get_arg();
        let dwResSize = call.get_arg();
        let fIcon = call.get_arg();
        let dwVer = call.get_arg();
        let res = api.CreateIconFromResource(presbits, dwResSize, fIcon, dwVer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateIconFromResourceEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let presbits = call.get_arg();
        let dwResSize = call.get_arg();
        let fIcon = call.get_arg();
        let dwVer = call.get_arg();
        let cxDesired = call.get_arg();
        let cyDesired = call.get_arg();
        let Flags = call.get_arg();
        let res = api.CreateIconFromResourceEx(
            presbits, dwResSize, fIcon, dwVer, cxDesired, cyDesired, Flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateMDIWindowA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpClassName = call.get_arg();
        let lpWindowName = call.get_arg();
        let dwStyle = call.get_arg();
        let X = call.get_arg();
        let Y = call.get_arg();
        let nWidth = call.get_arg();
        let nHeight = call.get_arg();
        let hWndParent = call.get_arg();
        let hInstance = call.get_arg();
        let lParam = call.get_arg();
        let res = api.CreateMDIWindowA(
            lpClassName,
            lpWindowName,
            dwStyle,
            X,
            Y,
            nWidth,
            nHeight,
            hWndParent,
            hInstance,
            lParam,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateMDIWindowW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpClassName = call.get_arg();
        let lpWindowName = call.get_arg();
        let dwStyle = call.get_arg();
        let X = call.get_arg();
        let Y = call.get_arg();
        let nWidth = call.get_arg();
        let nHeight = call.get_arg();
        let hWndParent = call.get_arg();
        let hInstance = call.get_arg();
        let lParam = call.get_arg();
        let res = api.CreateMDIWindowW(
            lpClassName,
            lpWindowName,
            dwStyle,
            X,
            Y,
            nWidth,
            nHeight,
            hWndParent,
            hInstance,
            lParam,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateMenu(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.CreateMenu();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreatePopupMenu(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.CreatePopupMenu();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateResourceIndexer(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let projectRoot = call.get_arg();
        let extensionDllPath = call.get_arg();
        let ppResourceIndexer = call.get_arg();
        let res = api.CreateResourceIndexer(projectRoot, extensionDllPath, ppResourceIndexer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateWindowExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dwExStyle = call.get_arg();
        let lpClassName = call.get_arg();
        let lpWindowName = call.get_arg();
        let dwStyle = call.get_arg();
        let X = call.get_arg();
        let Y = call.get_arg();
        let nWidth = call.get_arg();
        let nHeight = call.get_arg();
        let hWndParent = call.get_arg();
        let hMenu = call.get_arg();
        let hInstance = call.get_arg();
        let lpParam = call.get_arg();
        let res = api.CreateWindowExA(
            dwExStyle,
            lpClassName,
            lpWindowName,
            dwStyle,
            X,
            Y,
            nWidth,
            nHeight,
            hWndParent,
            hMenu,
            hInstance,
            lpParam,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateWindowExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dwExStyle = call.get_arg();
        let lpClassName = call.get_arg();
        let lpWindowName = call.get_arg();
        let dwStyle = call.get_arg();
        let X = call.get_arg();
        let Y = call.get_arg();
        let nWidth = call.get_arg();
        let nHeight = call.get_arg();
        let hWndParent = call.get_arg();
        let hMenu = call.get_arg();
        let hInstance = call.get_arg();
        let lpParam = call.get_arg();
        let res = api.CreateWindowExW(
            dwExStyle,
            lpClassName,
            lpWindowName,
            dwStyle,
            X,
            Y,
            nWidth,
            nHeight,
            hWndParent,
            hMenu,
            hInstance,
            lpParam,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DefDlgProcA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.DefDlgProcA(hDlg, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DefDlgProcW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.DefDlgProcW(hDlg, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DefFrameProcA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let hWndMDIClient = call.get_arg();
        let uMsg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.DefFrameProcA(hWnd, hWndMDIClient, uMsg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DefFrameProcW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let hWndMDIClient = call.get_arg();
        let uMsg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.DefFrameProcW(hWnd, hWndMDIClient, uMsg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DefMDIChildProcA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let uMsg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.DefMDIChildProcA(hWnd, uMsg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DefMDIChildProcW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let uMsg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.DefMDIChildProcW(hWnd, uMsg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DefWindowProcA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.DefWindowProcA(hWnd, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DefWindowProcW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.DefWindowProcW(hWnd, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DeferWindowPos(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWinPosInfo = call.get_arg();
        let hWnd = call.get_arg();
        let hWndInsertAfter = call.get_arg();
        let x = call.get_arg();
        let y = call.get_arg();
        let cx = call.get_arg();
        let cy = call.get_arg();
        let uFlags = call.get_arg();
        let res = api.DeferWindowPos(hWinPosInfo, hWnd, hWndInsertAfter, x, y, cx, cy, uFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DeleteMenu(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let uPosition = call.get_arg();
        let uFlags = call.get_arg();
        let res = api.DeleteMenu(hMenu, uPosition, uFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DeregisterShellHookWindow(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let res = api.DeregisterShellHookWindow(hwnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DestroyAcceleratorTable(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hAccel = call.get_arg();
        let res = api.DestroyAcceleratorTable(hAccel);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DestroyCaret(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.DestroyCaret();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DestroyCursor(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hCursor = call.get_arg();
        let res = api.DestroyCursor(hCursor);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DestroyIcon(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hIcon = call.get_arg();
        let res = api.DestroyIcon(hIcon);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DestroyIndexedResults(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let resourceUri = call.get_arg();
        let qualifierCount = call.get_arg();
        let qualifiers = call.get_arg();
        let res = api.DestroyIndexedResults(resourceUri, qualifierCount, qualifiers);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DestroyMenu(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let res = api.DestroyMenu(hMenu);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DestroyResourceIndexer(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let resourceIndexer = call.get_arg();
        let res = api.DestroyResourceIndexer(resourceIndexer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DestroyWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.DestroyWindow(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DialogBoxIndirectParamA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let hDialogTemplate = call.get_arg();
        let hWndParent = call.get_arg();
        let lpDialogFunc = call.get_arg();
        let dwInitParam = call.get_arg();
        let res = api.DialogBoxIndirectParamA(
            hInstance,
            hDialogTemplate,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DialogBoxIndirectParamW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let hDialogTemplate = call.get_arg();
        let hWndParent = call.get_arg();
        let lpDialogFunc = call.get_arg();
        let dwInitParam = call.get_arg();
        let res = api.DialogBoxIndirectParamW(
            hInstance,
            hDialogTemplate,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DialogBoxParamA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let lpTemplateName = call.get_arg();
        let hWndParent = call.get_arg();
        let lpDialogFunc = call.get_arg();
        let dwInitParam = call.get_arg();
        let res = api.DialogBoxParamA(
            hInstance,
            lpTemplateName,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DialogBoxParamW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let lpTemplateName = call.get_arg();
        let hWndParent = call.get_arg();
        let lpDialogFunc = call.get_arg();
        let dwInitParam = call.get_arg();
        let res = api.DialogBoxParamW(
            hInstance,
            lpTemplateName,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DisableProcessWindowsGhosting(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.DisableProcessWindowsGhosting();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DispatchMessageA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpMsg = call.get_arg();
        let res = api.DispatchMessageA(lpMsg);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DispatchMessageW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpMsg = call.get_arg();
        let res = api.DispatchMessageW(lpMsg);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DragObject(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwndParent = call.get_arg();
        let hwndFrom = call.get_arg();
        let fmt = call.get_arg();
        let data = call.get_arg();
        let hcur = call.get_arg();
        let res = api.DragObject(hwndParent, hwndFrom, fmt, data, hcur);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DrawMenuBar(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.DrawMenuBar(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnableMenuItem(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let uIDEnableItem = call.get_arg();
        let uEnable = call.get_arg();
        let res = api.EnableMenuItem(hMenu, uIDEnableItem, uEnable);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EndDeferWindowPos(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWinPosInfo = call.get_arg();
        let res = api.EndDeferWindowPos(hWinPosInfo);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EndDialog(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let nResult = call.get_arg();
        let res = api.EndDialog(hDlg, nResult);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EndMenu(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.EndMenu();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumChildWindows(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWndParent = call.get_arg();
        let lpEnumFunc = call.get_arg();
        let lParam = call.get_arg();
        let res = api.EnumChildWindows(hWndParent, lpEnumFunc, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumPropsA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpEnumFunc = call.get_arg();
        let res = api.EnumPropsA(hWnd, lpEnumFunc);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumPropsExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpEnumFunc = call.get_arg();
        let lParam = call.get_arg();
        let res = api.EnumPropsExA(hWnd, lpEnumFunc, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumPropsExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpEnumFunc = call.get_arg();
        let lParam = call.get_arg();
        let res = api.EnumPropsExW(hWnd, lpEnumFunc, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumPropsW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpEnumFunc = call.get_arg();
        let res = api.EnumPropsW(hWnd, lpEnumFunc);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumThreadWindows(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dwThreadId = call.get_arg();
        let lpfn = call.get_arg();
        let lParam = call.get_arg();
        let res = api.EnumThreadWindows(dwThreadId, lpfn, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumWindows(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpEnumFunc = call.get_arg();
        let lParam = call.get_arg();
        let res = api.EnumWindows(lpEnumFunc, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindWindowA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpClassName = call.get_arg();
        let lpWindowName = call.get_arg();
        let res = api.FindWindowA(lpClassName, lpWindowName);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindWindowExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWndParent = call.get_arg();
        let hWndChildAfter = call.get_arg();
        let lpszClass = call.get_arg();
        let lpszWindow = call.get_arg();
        let res = api.FindWindowExA(hWndParent, hWndChildAfter, lpszClass, lpszWindow);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindWindowExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWndParent = call.get_arg();
        let hWndChildAfter = call.get_arg();
        let lpszClass = call.get_arg();
        let lpszWindow = call.get_arg();
        let res = api.FindWindowExW(hWndParent, hWndChildAfter, lpszClass, lpszWindow);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindWindowW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpClassName = call.get_arg();
        let lpWindowName = call.get_arg();
        let res = api.FindWindowW(lpClassName, lpWindowName);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FlashWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let bInvert = call.get_arg();
        let res = api.FlashWindow(hWnd, bInvert);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FlashWindowEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pfwi = call.get_arg();
        let res = api.FlashWindowEx(pfwi);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetAltTabInfoA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let iItem = call.get_arg();
        let pati = call.get_arg();
        let pszItemText = call.get_arg();
        let cchItemText = call.get_arg();
        let res = api.GetAltTabInfoA(hwnd, iItem, pati, pszItemText, cchItemText);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetAltTabInfoW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let iItem = call.get_arg();
        let pati = call.get_arg();
        let pszItemText = call.get_arg();
        let cchItemText = call.get_arg();
        let res = api.GetAltTabInfoW(hwnd, iItem, pati, pszItemText, cchItemText);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetAncestor(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let gaFlags = call.get_arg();
        let res = api.GetAncestor(hwnd, gaFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCaretBlinkTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetCaretBlinkTime();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCaretPos(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpPoint = call.get_arg();
        let res = api.GetCaretPos(lpPoint);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetClassLongA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nIndex = call.get_arg();
        let res = api.GetClassLongA(hWnd, nIndex);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetClassLongW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nIndex = call.get_arg();
        let res = api.GetClassLongW(hWnd, nIndex);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetClassNameA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpClassName = call.get_arg();
        let nMaxCount = call.get_arg();
        let res = api.GetClassNameA(hWnd, lpClassName, nMaxCount);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetClassNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpClassName = call.get_arg();
        let nMaxCount = call.get_arg();
        let res = api.GetClassNameW(hWnd, lpClassName, nMaxCount);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetClassWord(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nIndex = call.get_arg();
        let res = api.GetClassWord(hWnd, nIndex);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetClientRect(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpRect = call.get_arg();
        let res = api.GetClientRect(hWnd, lpRect);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetClipCursor(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpRect = call.get_arg();
        let res = api.GetClipCursor(lpRect);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCursor(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetCursor();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCursorInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pci = call.get_arg();
        let res = api.GetCursorInfo(pci);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCursorPos(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpPoint = call.get_arg();
        let res = api.GetCursorPos(lpPoint);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDesktopWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetDesktopWindow();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDialogBaseUnits(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetDialogBaseUnits();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDlgCtrlID(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.GetDlgCtrlID(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDlgItem(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let nIDDlgItem = call.get_arg();
        let res = api.GetDlgItem(hDlg, nIDDlgItem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDlgItemInt(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let nIDDlgItem = call.get_arg();
        let lpTranslated = call.get_arg();
        let bSigned = call.get_arg();
        let res = api.GetDlgItemInt(hDlg, nIDDlgItem, lpTranslated, bSigned);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDlgItemTextA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let nIDDlgItem = call.get_arg();
        let lpString = call.get_arg();
        let cchMax = call.get_arg();
        let res = api.GetDlgItemTextA(hDlg, nIDDlgItem, lpString, cchMax);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDlgItemTextW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let nIDDlgItem = call.get_arg();
        let lpString = call.get_arg();
        let cchMax = call.get_arg();
        let res = api.GetDlgItemTextW(hDlg, nIDDlgItem, lpString, cchMax);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetForegroundWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetForegroundWindow();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetGUIThreadInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let idThread = call.get_arg();
        let pgui = call.get_arg();
        let res = api.GetGUIThreadInfo(idThread, pgui);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetInputState(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetInputState();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLastActivePopup(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.GetLastActivePopup(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLayeredWindowAttributes(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let pcrKey = call.get_arg();
        let pbAlpha = call.get_arg();
        let pdwFlags = call.get_arg();
        let res = api.GetLayeredWindowAttributes(hwnd, pcrKey, pbAlpha, pdwFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMenu(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.GetMenu(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMenuBarInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let idObject = call.get_arg();
        let idItem = call.get_arg();
        let pmbi = call.get_arg();
        let res = api.GetMenuBarInfo(hwnd, idObject, idItem, pmbi);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMenuCheckMarkDimensions(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetMenuCheckMarkDimensions();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMenuDefaultItem(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let fByPos = call.get_arg();
        let gmdiFlags = call.get_arg();
        let res = api.GetMenuDefaultItem(hMenu, fByPos, gmdiFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMenuItemCount(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let res = api.GetMenuItemCount(hMenu);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMenuItemID(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let nPos = call.get_arg();
        let res = api.GetMenuItemID(hMenu, nPos);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMenuItemRect(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let hMenu = call.get_arg();
        let uItem = call.get_arg();
        let lprcItem = call.get_arg();
        let res = api.GetMenuItemRect(hWnd, hMenu, uItem, lprcItem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMenuState(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let uId = call.get_arg();
        let uFlags = call.get_arg();
        let res = api.GetMenuState(hMenu, uId, uFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMenuStringA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let uIDItem = call.get_arg();
        let lpString = call.get_arg();
        let cchMax = call.get_arg();
        let flags = call.get_arg();
        let res = api.GetMenuStringA(hMenu, uIDItem, lpString, cchMax, flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMenuStringW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let uIDItem = call.get_arg();
        let lpString = call.get_arg();
        let cchMax = call.get_arg();
        let flags = call.get_arg();
        let res = api.GetMenuStringW(hMenu, uIDItem, lpString, cchMax, flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMessageA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpMsg = call.get_arg();
        let hWnd = call.get_arg();
        let wMsgFilterMin = call.get_arg();
        let wMsgFilterMax = call.get_arg();
        let res = api.GetMessageA(lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMessageExtraInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetMessageExtraInfo();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMessagePos(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetMessagePos();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMessageTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetMessageTime();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMessageW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpMsg = call.get_arg();
        let hWnd = call.get_arg();
        let wMsgFilterMin = call.get_arg();
        let wMsgFilterMax = call.get_arg();
        let res = api.GetMessageW(lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetNextDlgGroupItem(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let hCtl = call.get_arg();
        let bPrevious = call.get_arg();
        let res = api.GetNextDlgGroupItem(hDlg, hCtl, bPrevious);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetNextDlgTabItem(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let hCtl = call.get_arg();
        let bPrevious = call.get_arg();
        let res = api.GetNextDlgTabItem(hDlg, hCtl, bPrevious);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetParent(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.GetParent(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetPhysicalCursorPos(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpPoint = call.get_arg();
        let res = api.GetPhysicalCursorPos(lpPoint);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetProcessDefaultLayout(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pdwDefaultLayout = call.get_arg();
        let res = api.GetProcessDefaultLayout(pdwDefaultLayout);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetPropA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpString = call.get_arg();
        let res = api.GetPropA(hWnd, lpString);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetPropW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpString = call.get_arg();
        let res = api.GetPropW(hWnd, lpString);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetQueueStatus(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let flags = call.get_arg();
        let res = api.GetQueueStatus(flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetScrollBarInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let idObject = call.get_arg();
        let psbi = call.get_arg();
        let res = api.GetScrollBarInfo(hwnd, idObject, psbi);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetScrollInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let nBar = call.get_arg();
        let lpsi = call.get_arg();
        let res = api.GetScrollInfo(hwnd, nBar, lpsi);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetScrollPos(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nBar = call.get_arg();
        let res = api.GetScrollPos(hWnd, nBar);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetScrollRange(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nBar = call.get_arg();
        let lpMinPos = call.get_arg();
        let lpMaxPos = call.get_arg();
        let res = api.GetScrollRange(hWnd, nBar, lpMinPos, lpMaxPos);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetShellWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetShellWindow();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSubMenu(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let nPos = call.get_arg();
        let res = api.GetSubMenu(hMenu, nPos);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSysColor(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let nIndex = call.get_arg();
        let res = api.GetSysColor(nIndex);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemMenu(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let bRevert = call.get_arg();
        let res = api.GetSystemMenu(hWnd, bRevert);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemMetrics(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let nIndex = call.get_arg();
        let res = api.GetSystemMetrics(nIndex);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTitleBarInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let pti = call.get_arg();
        let res = api.GetTitleBarInfo(hwnd, pti);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTopWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.GetTopWindow(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let uCmd = call.get_arg();
        let res = api.GetWindow(hWnd, uCmd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowDisplayAffinity(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let pdwAffinity = call.get_arg();
        let res = api.GetWindowDisplayAffinity(hWnd, pdwAffinity);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let pwi = call.get_arg();
        let res = api.GetWindowInfo(hwnd, pwi);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowLongA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nIndex = call.get_arg();
        let res = api.GetWindowLongA(hWnd, nIndex);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowLongW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nIndex = call.get_arg();
        let res = api.GetWindowLongW(hWnd, nIndex);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowModuleFileNameA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let pszFileName = call.get_arg();
        let cchFileNameMax = call.get_arg();
        let res = api.GetWindowModuleFileNameA(hwnd, pszFileName, cchFileNameMax);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowModuleFileNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let pszFileName = call.get_arg();
        let cchFileNameMax = call.get_arg();
        let res = api.GetWindowModuleFileNameW(hwnd, pszFileName, cchFileNameMax);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowPlacement(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpwndpl = call.get_arg();
        let res = api.GetWindowPlacement(hWnd, lpwndpl);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowRect(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpRect = call.get_arg();
        let res = api.GetWindowRect(hWnd, lpRect);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowTextA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpString = call.get_arg();
        let nMaxCount = call.get_arg();
        let res = api.GetWindowTextA(hWnd, lpString, nMaxCount);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowTextLengthA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.GetWindowTextLengthA(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowTextLengthW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.GetWindowTextLengthW(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowTextW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpString = call.get_arg();
        let nMaxCount = call.get_arg();
        let res = api.GetWindowTextW(hWnd, lpString, nMaxCount);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowThreadProcessId(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpdwProcessId = call.get_arg();
        let res = api.GetWindowThreadProcessId(hWnd, lpdwProcessId);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWindowWord(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nIndex = call.get_arg();
        let res = api.GetWindowWord(hWnd, nIndex);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HideCaret(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.HideCaret(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HiliteMenuItem(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let hMenu = call.get_arg();
        let uIDHiliteItem = call.get_arg();
        let uHilite = call.get_arg();
        let res = api.HiliteMenuItem(hWnd, hMenu, uIDHiliteItem, uHilite);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_InSendMessage(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.InSendMessage();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_InSendMessageEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpReserved = call.get_arg();
        let res = api.InSendMessageEx(lpReserved);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IndexFilePath(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let resourceIndexer = call.get_arg();
        let filePath = call.get_arg();
        let ppResourceUri = call.get_arg();
        let pQualifierCount = call.get_arg();
        let ppQualifiers = call.get_arg();
        let res = api.IndexFilePath(
            resourceIndexer,
            filePath,
            ppResourceUri,
            pQualifierCount,
            ppQualifiers,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_InheritWindowMonitor(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let hwndInherit = call.get_arg();
        let res = api.InheritWindowMonitor(hwnd, hwndInherit);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_InsertMenuA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let uPosition = call.get_arg();
        let uFlags = call.get_arg();
        let uIDNewItem = call.get_arg();
        let lpNewItem = call.get_arg();
        let res = api.InsertMenuA(hMenu, uPosition, uFlags, uIDNewItem, lpNewItem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_InsertMenuW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let uPosition = call.get_arg();
        let uFlags = call.get_arg();
        let uIDNewItem = call.get_arg();
        let lpNewItem = call.get_arg();
        let res = api.InsertMenuW(hMenu, uPosition, uFlags, uIDNewItem, lpNewItem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_InternalGetWindowText(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let pString = call.get_arg();
        let cchMaxCount = call.get_arg();
        let res = api.InternalGetWindowText(hWnd, pString, cchMaxCount);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsCharAlphaA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ch = call.get_arg();
        let res = api.IsCharAlphaA(ch);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsCharAlphaNumericA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ch = call.get_arg();
        let res = api.IsCharAlphaNumericA(ch);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsCharAlphaNumericW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ch = call.get_arg();
        let res = api.IsCharAlphaNumericW(ch);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsCharAlphaW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ch = call.get_arg();
        let res = api.IsCharAlphaW(ch);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsCharLowerA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ch = call.get_arg();
        let res = api.IsCharLowerA(ch);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsCharUpperA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ch = call.get_arg();
        let res = api.IsCharUpperA(ch);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsCharUpperW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ch = call.get_arg();
        let res = api.IsCharUpperW(ch);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsChild(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWndParent = call.get_arg();
        let hWnd = call.get_arg();
        let res = api.IsChild(hWndParent, hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsDialogMessageA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let lpMsg = call.get_arg();
        let res = api.IsDialogMessageA(hDlg, lpMsg);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsDialogMessageW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let lpMsg = call.get_arg();
        let res = api.IsDialogMessageW(hDlg, lpMsg);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsGUIThread(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let bConvert = call.get_arg();
        let res = api.IsGUIThread(bConvert);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsHungAppWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let res = api.IsHungAppWindow(hwnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsIconic(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.IsIconic(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsMenu(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let res = api.IsMenu(hMenu);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsProcessDPIAware(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.IsProcessDPIAware();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.IsWindow(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsWindowUnicode(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.IsWindowUnicode(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsWindowVisible(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.IsWindowVisible(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsWow64Message(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.IsWow64Message();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsZoomed(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.IsZoomed(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_KillTimer(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let uIDEvent = call.get_arg();
        let res = api.KillTimer(hWnd, uIDEvent);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadAcceleratorsA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let lpTableName = call.get_arg();
        let res = api.LoadAcceleratorsA(hInstance, lpTableName);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadAcceleratorsW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let lpTableName = call.get_arg();
        let res = api.LoadAcceleratorsW(hInstance, lpTableName);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadCursorA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let lpCursorName = call.get_arg();
        let res = api.LoadCursorA(hInstance, lpCursorName);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadCursorFromFileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpFileName = call.get_arg();
        let res = api.LoadCursorFromFileA(lpFileName);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadCursorFromFileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpFileName = call.get_arg();
        let res = api.LoadCursorFromFileW(lpFileName);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadCursorW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let lpCursorName = call.get_arg();
        let res = api.LoadCursorW(hInstance, lpCursorName);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadIconA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let lpIconName = call.get_arg();
        let res = api.LoadIconA(hInstance, lpIconName);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadIconW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let lpIconName = call.get_arg();
        let res = api.LoadIconW(hInstance, lpIconName);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadImageA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInst = call.get_arg();
        let name = call.get_arg();
        let r#type = call.get_arg();
        let cx = call.get_arg();
        let cy = call.get_arg();
        let fuLoad = call.get_arg();
        let res = api.LoadImageA(hInst, name, r#type, cx, cy, fuLoad);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadImageW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInst = call.get_arg();
        let name = call.get_arg();
        let r#type = call.get_arg();
        let cx = call.get_arg();
        let cy = call.get_arg();
        let fuLoad = call.get_arg();
        let res = api.LoadImageW(hInst, name, r#type, cx, cy, fuLoad);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadMenuA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let lpMenuName = call.get_arg();
        let res = api.LoadMenuA(hInstance, lpMenuName);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadMenuIndirectA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpMenuTemplate = call.get_arg();
        let res = api.LoadMenuIndirectA(lpMenuTemplate);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadMenuIndirectW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpMenuTemplate = call.get_arg();
        let res = api.LoadMenuIndirectW(lpMenuTemplate);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadMenuW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let lpMenuName = call.get_arg();
        let res = api.LoadMenuW(hInstance, lpMenuName);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadStringA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let uID = call.get_arg();
        let lpBuffer = call.get_arg();
        let cchBufferMax = call.get_arg();
        let res = api.LoadStringA(hInstance, uID, lpBuffer, cchBufferMax);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadStringW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hInstance = call.get_arg();
        let uID = call.get_arg();
        let lpBuffer = call.get_arg();
        let cchBufferMax = call.get_arg();
        let res = api.LoadStringW(hInstance, uID, lpBuffer, cchBufferMax);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LockSetForegroundWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let uLockCode = call.get_arg();
        let res = api.LockSetForegroundWindow(uLockCode);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LogicalToPhysicalPoint(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpPoint = call.get_arg();
        let res = api.LogicalToPhysicalPoint(hWnd, lpPoint);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LookupIconIdFromDirectory(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let presbits = call.get_arg();
        let fIcon = call.get_arg();
        let res = api.LookupIconIdFromDirectory(presbits, fIcon);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LookupIconIdFromDirectoryEx(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let presbits = call.get_arg();
        let fIcon = call.get_arg();
        let cxDesired = call.get_arg();
        let cyDesired = call.get_arg();
        let Flags = call.get_arg();
        let res = api.LookupIconIdFromDirectoryEx(presbits, fIcon, cxDesired, cyDesired, Flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MapDialogRect(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let lpRect = call.get_arg();
        let res = api.MapDialogRect(hDlg, lpRect);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MenuItemFromPoint(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let hMenu = call.get_arg();
        let ptScreen = call.get_arg();
        let res = api.MenuItemFromPoint(hWnd, hMenu, ptScreen);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MessageBoxA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpText = call.get_arg();
        let lpCaption = call.get_arg();
        let uType = call.get_arg();
        let res = api.MessageBoxA(hWnd, lpText, lpCaption, uType);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MessageBoxExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpText = call.get_arg();
        let lpCaption = call.get_arg();
        let uType = call.get_arg();
        let wLanguageId = call.get_arg();
        let res = api.MessageBoxExA(hWnd, lpText, lpCaption, uType, wLanguageId);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MessageBoxExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpText = call.get_arg();
        let lpCaption = call.get_arg();
        let uType = call.get_arg();
        let wLanguageId = call.get_arg();
        let res = api.MessageBoxExW(hWnd, lpText, lpCaption, uType, wLanguageId);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MessageBoxW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpText = call.get_arg();
        let lpCaption = call.get_arg();
        let uType = call.get_arg();
        let res = api.MessageBoxW(hWnd, lpText, lpCaption, uType);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ModifyMenuA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMnu = call.get_arg();
        let uPosition = call.get_arg();
        let uFlags = call.get_arg();
        let uIDNewItem = call.get_arg();
        let lpNewItem = call.get_arg();
        let res = api.ModifyMenuA(hMnu, uPosition, uFlags, uIDNewItem, lpNewItem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ModifyMenuW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMnu = call.get_arg();
        let uPosition = call.get_arg();
        let uFlags = call.get_arg();
        let uIDNewItem = call.get_arg();
        let lpNewItem = call.get_arg();
        let res = api.ModifyMenuW(hMnu, uPosition, uFlags, uIDNewItem, lpNewItem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MoveWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let X = call.get_arg();
        let Y = call.get_arg();
        let nWidth = call.get_arg();
        let nHeight = call.get_arg();
        let bRepaint = call.get_arg();
        let res = api.MoveWindow(hWnd, X, Y, nWidth, nHeight, bRepaint);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmCreateConfig(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let platformVersion = call.get_arg();
        let defaultQualifiers = call.get_arg();
        let outputXmlFile = call.get_arg();
        let res = api.MrmCreateConfig(platformVersion, defaultQualifiers, outputXmlFile);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmCreateConfigInMemory(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let platformVersion = call.get_arg();
        let defaultQualifiers = call.get_arg();
        let outputXmlData = call.get_arg();
        let outputXmlSize = call.get_arg();
        let res = api.MrmCreateConfigInMemory(
            platformVersion,
            defaultQualifiers,
            outputXmlData,
            outputXmlSize,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmCreateResourceFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let indexer = call.get_arg();
        let packagingMode = call.get_arg();
        let packagingOptions = call.get_arg();
        let outputDirectory = call.get_arg();
        let res =
            api.MrmCreateResourceFile(indexer, packagingMode, packagingOptions, outputDirectory);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmCreateResourceFileInMemory(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let indexer = call.get_arg();
        let packagingMode = call.get_arg();
        let packagingOptions = call.get_arg();
        let outputPriData = call.get_arg();
        let outputPriSize = call.get_arg();
        let res = api.MrmCreateResourceFileInMemory(
            indexer,
            packagingMode,
            packagingOptions,
            outputPriData,
            outputPriSize,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmCreateResourceFileWithChecksum(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let indexer = call.get_arg();
        let packagingMode = call.get_arg();
        let packagingOptions = call.get_arg();
        let checksum = call.get_arg();
        let outputDirectory = call.get_arg();
        let res = api.MrmCreateResourceFileWithChecksum(
            indexer,
            packagingMode,
            packagingOptions,
            checksum,
            outputDirectory,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmCreateResourceIndexer(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let packageFamilyName = call.get_arg();
        let projectRoot = call.get_arg();
        let platformVersion = call.get_arg();
        let defaultQualifiers = call.get_arg();
        let indexer = call.get_arg();
        let res = api.MrmCreateResourceIndexer(
            packageFamilyName,
            projectRoot,
            platformVersion,
            defaultQualifiers,
            indexer,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmCreateResourceIndexerFromPreviousPriData(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let projectRoot = call.get_arg();
        let platformVersion = call.get_arg();
        let defaultQualifiers = call.get_arg();
        let priData = call.get_arg();
        let priSize = call.get_arg();
        let indexer = call.get_arg();
        let res = api.MrmCreateResourceIndexerFromPreviousPriData(
            projectRoot,
            platformVersion,
            defaultQualifiers,
            priData,
            priSize,
            indexer,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmCreateResourceIndexerFromPreviousPriFile(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let projectRoot = call.get_arg();
        let platformVersion = call.get_arg();
        let defaultQualifiers = call.get_arg();
        let priFile = call.get_arg();
        let indexer = call.get_arg();
        let res = api.MrmCreateResourceIndexerFromPreviousPriFile(
            projectRoot,
            platformVersion,
            defaultQualifiers,
            priFile,
            indexer,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmCreateResourceIndexerFromPreviousSchemaData(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let projectRoot = call.get_arg();
        let platformVersion = call.get_arg();
        let defaultQualifiers = call.get_arg();
        let schemaXmlData = call.get_arg();
        let schemaXmlSize = call.get_arg();
        let indexer = call.get_arg();
        let res = api.MrmCreateResourceIndexerFromPreviousSchemaData(
            projectRoot,
            platformVersion,
            defaultQualifiers,
            schemaXmlData,
            schemaXmlSize,
            indexer,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmCreateResourceIndexerFromPreviousSchemaFile(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let projectRoot = call.get_arg();
        let platformVersion = call.get_arg();
        let defaultQualifiers = call.get_arg();
        let schemaFile = call.get_arg();
        let indexer = call.get_arg();
        let res = api.MrmCreateResourceIndexerFromPreviousSchemaFile(
            projectRoot,
            platformVersion,
            defaultQualifiers,
            schemaFile,
            indexer,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmCreateResourceIndexerWithFlags(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let packageFamilyName = call.get_arg();
        let projectRoot = call.get_arg();
        let platformVersion = call.get_arg();
        let defaultQualifiers = call.get_arg();
        let flags = call.get_arg();
        let indexer = call.get_arg();
        let res = api.MrmCreateResourceIndexerWithFlags(
            packageFamilyName,
            projectRoot,
            platformVersion,
            defaultQualifiers,
            flags,
            indexer,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmDestroyIndexerAndMessages(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let indexer = call.get_arg();
        let res = api.MrmDestroyIndexerAndMessages(indexer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmDumpPriDataInMemory(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let inputPriData = call.get_arg();
        let inputPriSize = call.get_arg();
        let schemaPriData = call.get_arg();
        let schemaPriSize = call.get_arg();
        let dumpType = call.get_arg();
        let outputXmlData = call.get_arg();
        let outputXmlSize = call.get_arg();
        let res = api.MrmDumpPriDataInMemory(
            inputPriData,
            inputPriSize,
            schemaPriData,
            schemaPriSize,
            dumpType,
            outputXmlData,
            outputXmlSize,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmDumpPriFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let indexFileName = call.get_arg();
        let schemaPriFile = call.get_arg();
        let dumpType = call.get_arg();
        let outputXmlFile = call.get_arg();
        let res = api.MrmDumpPriFile(indexFileName, schemaPriFile, dumpType, outputXmlFile);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmDumpPriFileInMemory(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let indexFileName = call.get_arg();
        let schemaPriFile = call.get_arg();
        let dumpType = call.get_arg();
        let outputXmlData = call.get_arg();
        let outputXmlSize = call.get_arg();
        let res = api.MrmDumpPriFileInMemory(
            indexFileName,
            schemaPriFile,
            dumpType,
            outputXmlData,
            outputXmlSize,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmFreeMemory(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let data = call.get_arg();
        let res = api.MrmFreeMemory(data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmGetPriFileContentChecksum(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let priFile = call.get_arg();
        let checksum = call.get_arg();
        let res = api.MrmGetPriFileContentChecksum(priFile, checksum);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmIndexEmbeddedData(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let indexer = call.get_arg();
        let resourceUri = call.get_arg();
        let embeddedData = call.get_arg();
        let embeddedDataSize = call.get_arg();
        let qualifiers = call.get_arg();
        let res = api.MrmIndexEmbeddedData(
            indexer,
            resourceUri,
            embeddedData,
            embeddedDataSize,
            qualifiers,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmIndexFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let indexer = call.get_arg();
        let resourceUri = call.get_arg();
        let filePath = call.get_arg();
        let qualifiers = call.get_arg();
        let res = api.MrmIndexFile(indexer, resourceUri, filePath, qualifiers);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmIndexFileAutoQualifiers(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let indexer = call.get_arg();
        let filePath = call.get_arg();
        let res = api.MrmIndexFileAutoQualifiers(indexer, filePath);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmIndexResourceContainerAutoQualifiers(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let indexer = call.get_arg();
        let containerPath = call.get_arg();
        let res = api.MrmIndexResourceContainerAutoQualifiers(indexer, containerPath);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmIndexString(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let indexer = call.get_arg();
        let resourceUri = call.get_arg();
        let resourceString = call.get_arg();
        let qualifiers = call.get_arg();
        let res = api.MrmIndexString(indexer, resourceUri, resourceString, qualifiers);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MrmPeekResourceIndexerMessages(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let handle = call.get_arg();
        let messages = call.get_arg();
        let numMsgs = call.get_arg();
        let res = api.MrmPeekResourceIndexerMessages(handle, messages, numMsgs);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MsgWaitForMultipleObjects(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let nCount = call.get_arg();
        let pHandles = call.get_arg();
        let fWaitAll = call.get_arg();
        let dwMilliseconds = call.get_arg();
        let dwWakeMask = call.get_arg();
        let res =
            api.MsgWaitForMultipleObjects(nCount, pHandles, fWaitAll, dwMilliseconds, dwWakeMask);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MsgWaitForMultipleObjectsEx(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let nCount = call.get_arg();
        let pHandles = call.get_arg();
        let dwMilliseconds = call.get_arg();
        let dwWakeMask = call.get_arg();
        let dwFlags = call.get_arg();
        let res =
            api.MsgWaitForMultipleObjectsEx(nCount, pHandles, dwMilliseconds, dwWakeMask, dwFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OemToCharA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pSrc = call.get_arg();
        let pDst = call.get_arg();
        let res = api.OemToCharA(pSrc, pDst);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OemToCharBuffA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpszSrc = call.get_arg();
        let lpszDst = call.get_arg();
        let cchDstLength = call.get_arg();
        let res = api.OemToCharBuffA(lpszSrc, lpszDst, cchDstLength);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OemToCharBuffW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpszSrc = call.get_arg();
        let lpszDst = call.get_arg();
        let cchDstLength = call.get_arg();
        let res = api.OemToCharBuffW(lpszSrc, lpszDst, cchDstLength);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OemToCharW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pSrc = call.get_arg();
        let pDst = call.get_arg();
        let res = api.OemToCharW(pSrc, pDst);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenIcon(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.OpenIcon(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PeekMessageA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpMsg = call.get_arg();
        let hWnd = call.get_arg();
        let wMsgFilterMin = call.get_arg();
        let wMsgFilterMax = call.get_arg();
        let wRemoveMsg = call.get_arg();
        let res = api.PeekMessageA(lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax, wRemoveMsg);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PeekMessageW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpMsg = call.get_arg();
        let hWnd = call.get_arg();
        let wMsgFilterMin = call.get_arg();
        let wMsgFilterMax = call.get_arg();
        let wRemoveMsg = call.get_arg();
        let res = api.PeekMessageW(lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax, wRemoveMsg);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PhysicalToLogicalPoint(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpPoint = call.get_arg();
        let res = api.PhysicalToLogicalPoint(hWnd, lpPoint);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PostMessageA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.PostMessageA(hWnd, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PostMessageW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.PostMessageW(hWnd, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PostQuitMessage(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let nExitCode = call.get_arg();
        let res = api.PostQuitMessage(nExitCode);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PostThreadMessageA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let idThread = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.PostThreadMessageA(idThread, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PostThreadMessageW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let idThread = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.PostThreadMessageW(idThread, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PrivateExtractIconsA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let szFileName = call.get_arg();
        let nIconIndex = call.get_arg();
        let cxIcon = call.get_arg();
        let cyIcon = call.get_arg();
        let phicon = call.get_arg();
        let piconid = call.get_arg();
        let nIcons = call.get_arg();
        let flags = call.get_arg();
        let res = api.PrivateExtractIconsA(
            szFileName, nIconIndex, cxIcon, cyIcon, phicon, piconid, nIcons, flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PrivateExtractIconsW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let szFileName = call.get_arg();
        let nIconIndex = call.get_arg();
        let cxIcon = call.get_arg();
        let cyIcon = call.get_arg();
        let phicon = call.get_arg();
        let piconid = call.get_arg();
        let nIcons = call.get_arg();
        let flags = call.get_arg();
        let res = api.PrivateExtractIconsW(
            szFileName, nIconIndex, cxIcon, cyIcon, phicon, piconid, nIcons, flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RealChildWindowFromPoint(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwndParent = call.get_arg();
        let ptParentClientCoords = call.get_arg();
        let res = api.RealChildWindowFromPoint(hwndParent, ptParentClientCoords);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RealGetWindowClassA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let ptszClassName = call.get_arg();
        let cchClassNameMax = call.get_arg();
        let res = api.RealGetWindowClassA(hwnd, ptszClassName, cchClassNameMax);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RealGetWindowClassW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let ptszClassName = call.get_arg();
        let cchClassNameMax = call.get_arg();
        let res = api.RealGetWindowClassW(hwnd, ptszClassName, cchClassNameMax);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RegisterShellHookWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let res = api.RegisterShellHookWindow(hwnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RegisterWindowMessageA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpString = call.get_arg();
        let res = api.RegisterWindowMessageA(lpString);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RegisterWindowMessageW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpString = call.get_arg();
        let res = api.RegisterWindowMessageW(lpString);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RemoveMenu(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let uPosition = call.get_arg();
        let uFlags = call.get_arg();
        let res = api.RemoveMenu(hMenu, uPosition, uFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RemovePropA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpString = call.get_arg();
        let res = api.RemovePropA(hWnd, lpString);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RemovePropW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpString = call.get_arg();
        let res = api.RemovePropW(hWnd, lpString);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReplyMessage(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lResult = call.get_arg();
        let res = api.ReplyMessage(lResult);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScrollWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let XAmount = call.get_arg();
        let YAmount = call.get_arg();
        let lpRect = call.get_arg();
        let lpClipRect = call.get_arg();
        let res = api.ScrollWindow(hWnd, XAmount, YAmount, lpRect, lpClipRect);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SendDlgItemMessageA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let nIDDlgItem = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.SendDlgItemMessageA(hDlg, nIDDlgItem, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SendDlgItemMessageW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let nIDDlgItem = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.SendDlgItemMessageW(hDlg, nIDDlgItem, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SendMessageA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.SendMessageA(hWnd, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SendMessageCallbackA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let lpResultCallBack = call.get_arg();
        let dwData = call.get_arg();
        let res = api.SendMessageCallbackA(hWnd, Msg, wParam, lParam, lpResultCallBack, dwData);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SendMessageCallbackW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let lpResultCallBack = call.get_arg();
        let dwData = call.get_arg();
        let res = api.SendMessageCallbackW(hWnd, Msg, wParam, lParam, lpResultCallBack, dwData);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SendMessageTimeoutA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let fuFlags = call.get_arg();
        let uTimeout = call.get_arg();
        let lpdwResult = call.get_arg();
        let res = api.SendMessageTimeoutA(hWnd, Msg, wParam, lParam, fuFlags, uTimeout, lpdwResult);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SendMessageTimeoutW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let fuFlags = call.get_arg();
        let uTimeout = call.get_arg();
        let lpdwResult = call.get_arg();
        let res = api.SendMessageTimeoutW(hWnd, Msg, wParam, lParam, fuFlags, uTimeout, lpdwResult);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SendMessageW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.SendMessageW(hWnd, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SendNotifyMessageA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.SendNotifyMessageA(hWnd, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SendNotifyMessageW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let Msg = call.get_arg();
        let wParam = call.get_arg();
        let lParam = call.get_arg();
        let res = api.SendNotifyMessageW(hWnd, Msg, wParam, lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetCaretBlinkTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let uMSeconds = call.get_arg();
        let res = api.SetCaretBlinkTime(uMSeconds);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetCaretPos(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let X = call.get_arg();
        let Y = call.get_arg();
        let res = api.SetCaretPos(X, Y);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetClassLongA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nIndex = call.get_arg();
        let dwNewLong = call.get_arg();
        let res = api.SetClassLongA(hWnd, nIndex, dwNewLong);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetClassLongW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nIndex = call.get_arg();
        let dwNewLong = call.get_arg();
        let res = api.SetClassLongW(hWnd, nIndex, dwNewLong);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetClassWord(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nIndex = call.get_arg();
        let wNewWord = call.get_arg();
        let res = api.SetClassWord(hWnd, nIndex, wNewWord);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetCoalescableTimer(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nIDEvent = call.get_arg();
        let uElapse = call.get_arg();
        let lpTimerFunc = call.get_arg();
        let uToleranceDelay = call.get_arg();
        let res = api.SetCoalescableTimer(hWnd, nIDEvent, uElapse, lpTimerFunc, uToleranceDelay);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetCursor(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hCursor = call.get_arg();
        let res = api.SetCursor(hCursor);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetCursorPos(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let X = call.get_arg();
        let Y = call.get_arg();
        let res = api.SetCursorPos(X, Y);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetDebugErrorLevel(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dwLevel = call.get_arg();
        let res = api.SetDebugErrorLevel(dwLevel);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetDlgItemInt(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let nIDDlgItem = call.get_arg();
        let uValue = call.get_arg();
        let bSigned = call.get_arg();
        let res = api.SetDlgItemInt(hDlg, nIDDlgItem, uValue, bSigned);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetDlgItemTextA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let nIDDlgItem = call.get_arg();
        let lpString = call.get_arg();
        let res = api.SetDlgItemTextA(hDlg, nIDDlgItem, lpString);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetDlgItemTextW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hDlg = call.get_arg();
        let nIDDlgItem = call.get_arg();
        let lpString = call.get_arg();
        let res = api.SetDlgItemTextW(hDlg, nIDDlgItem, lpString);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetForegroundWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.SetForegroundWindow(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetLayeredWindowAttributes(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let crKey = call.get_arg();
        let bAlpha = call.get_arg();
        let dwFlags = call.get_arg();
        let res = api.SetLayeredWindowAttributes(hwnd, crKey, bAlpha, dwFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetMenu(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let hMenu = call.get_arg();
        let res = api.SetMenu(hWnd, hMenu);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetMenuDefaultItem(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let uItem = call.get_arg();
        let fByPos = call.get_arg();
        let res = api.SetMenuDefaultItem(hMenu, uItem, fByPos);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetMessageExtraInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lParam = call.get_arg();
        let res = api.SetMessageExtraInfo(lParam);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetMessageQueue(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let cMessagesMax = call.get_arg();
        let res = api.SetMessageQueue(cMessagesMax);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetParent(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWndChild = call.get_arg();
        let hWndNewParent = call.get_arg();
        let res = api.SetParent(hWndChild, hWndNewParent);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetPhysicalCursorPos(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let X = call.get_arg();
        let Y = call.get_arg();
        let res = api.SetPhysicalCursorPos(X, Y);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetProcessDPIAware(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.SetProcessDPIAware();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetProcessDefaultLayout(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dwDefaultLayout = call.get_arg();
        let res = api.SetProcessDefaultLayout(dwDefaultLayout);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetPropA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpString = call.get_arg();
        let hData = call.get_arg();
        let res = api.SetPropA(hWnd, lpString, hData);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetPropW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpString = call.get_arg();
        let hData = call.get_arg();
        let res = api.SetPropW(hWnd, lpString, hData);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetSysColors(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let cElements = call.get_arg();
        let lpaElements = call.get_arg();
        let lpaRgbValues = call.get_arg();
        let res = api.SetSysColors(cElements, lpaElements, lpaRgbValues);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetSystemCursor(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hcur = call.get_arg();
        let id = call.get_arg();
        let res = api.SetSystemCursor(hcur, id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetTimer(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nIDEvent = call.get_arg();
        let uElapse = call.get_arg();
        let lpTimerFunc = call.get_arg();
        let res = api.SetTimer(hWnd, nIDEvent, uElapse, lpTimerFunc);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetWindowDisplayAffinity(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let dwAffinity = call.get_arg();
        let res = api.SetWindowDisplayAffinity(hWnd, dwAffinity);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetWindowLongA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nIndex = call.get_arg();
        let dwNewLong = call.get_arg();
        let res = api.SetWindowLongA(hWnd, nIndex, dwNewLong);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetWindowLongW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nIndex = call.get_arg();
        let dwNewLong = call.get_arg();
        let res = api.SetWindowLongW(hWnd, nIndex, dwNewLong);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetWindowPlacement(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpwndpl = call.get_arg();
        let res = api.SetWindowPlacement(hWnd, lpwndpl);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetWindowPos(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let hWndInsertAfter = call.get_arg();
        let X = call.get_arg();
        let Y = call.get_arg();
        let cx = call.get_arg();
        let cy = call.get_arg();
        let uFlags = call.get_arg();
        let res = api.SetWindowPos(hWnd, hWndInsertAfter, X, Y, cx, cy, uFlags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetWindowTextA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpString = call.get_arg();
        let res = api.SetWindowTextA(hWnd, lpString);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetWindowTextW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let lpString = call.get_arg();
        let res = api.SetWindowTextW(hWnd, lpString);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetWindowWord(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nIndex = call.get_arg();
        let wNewWord = call.get_arg();
        let res = api.SetWindowWord(hWnd, nIndex, wNewWord);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetWindowsHookA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let nFilterType = call.get_arg();
        let pfnFilterProc = call.get_arg();
        let res = api.SetWindowsHookA(nFilterType, pfnFilterProc);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetWindowsHookExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let idHook = call.get_arg();
        let lpfn = call.get_arg();
        let hmod = call.get_arg();
        let dwThreadId = call.get_arg();
        let res = api.SetWindowsHookExA(idHook, lpfn, hmod, dwThreadId);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetWindowsHookExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let idHook = call.get_arg();
        let lpfn = call.get_arg();
        let hmod = call.get_arg();
        let dwThreadId = call.get_arg();
        let res = api.SetWindowsHookExW(idHook, lpfn, hmod, dwThreadId);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetWindowsHookW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let nFilterType = call.get_arg();
        let pfnFilterProc = call.get_arg();
        let res = api.SetWindowsHookW(nFilterType, pfnFilterProc);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ShowCaret(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let res = api.ShowCaret(hWnd);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ShowCursor(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let bShow = call.get_arg();
        let res = api.ShowCursor(bShow);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ShowOwnedPopups(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let fShow = call.get_arg();
        let res = api.ShowOwnedPopups(hWnd, fShow);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ShowWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nCmdShow = call.get_arg();
        let res = api.ShowWindow(hWnd, nCmdShow);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ShowWindowAsync(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let nCmdShow = call.get_arg();
        let res = api.ShowWindowAsync(hWnd, nCmdShow);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SoundSentry(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.SoundSentry();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SwitchToThisWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let fUnknown = call.get_arg();
        let res = api.SwitchToThisWindow(hwnd, fUnknown);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SystemParametersInfoA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let uiAction = call.get_arg();
        let uiParam = call.get_arg();
        let pvParam = call.get_arg();
        let fWinIni = call.get_arg();
        let res = api.SystemParametersInfoA(uiAction, uiParam, pvParam, fWinIni);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SystemParametersInfoW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let uiAction = call.get_arg();
        let uiParam = call.get_arg();
        let pvParam = call.get_arg();
        let fWinIni = call.get_arg();
        let res = api.SystemParametersInfoW(uiAction, uiParam, pvParam, fWinIni);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TileWindows(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwndParent = call.get_arg();
        let wHow = call.get_arg();
        let lpRect = call.get_arg();
        let cKids = call.get_arg();
        let lpKids = call.get_arg();
        let res = api.TileWindows(hwndParent, wHow, lpRect, cKids, lpKids);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TrackPopupMenu(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let uFlags = call.get_arg();
        let x = call.get_arg();
        let y = call.get_arg();
        let nReserved = call.get_arg();
        let hWnd = call.get_arg();
        let prcRect = call.get_arg();
        let res = api.TrackPopupMenu(hMenu, uFlags, x, y, nReserved, hWnd, prcRect);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TrackPopupMenuEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hMenu = call.get_arg();
        let uFlags = call.get_arg();
        let x = call.get_arg();
        let y = call.get_arg();
        let hwnd = call.get_arg();
        let lptpm = call.get_arg();
        let res = api.TrackPopupMenuEx(hMenu, uFlags, x, y, hwnd, lptpm);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TranslateAcceleratorA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let hAccTable = call.get_arg();
        let lpMsg = call.get_arg();
        let res = api.TranslateAcceleratorA(hWnd, hAccTable, lpMsg);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TranslateAcceleratorW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWnd = call.get_arg();
        let hAccTable = call.get_arg();
        let lpMsg = call.get_arg();
        let res = api.TranslateAcceleratorW(hWnd, hAccTable, lpMsg);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TranslateMDISysAccel(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hWndClient = call.get_arg();
        let lpMsg = call.get_arg();
        let res = api.TranslateMDISysAccel(hWndClient, lpMsg);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TranslateMessage(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpMsg = call.get_arg();
        let res = api.TranslateMessage(lpMsg);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UnhookWindowsHook(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let nCode = call.get_arg();
        let pfnFilterProc = call.get_arg();
        let res = api.UnhookWindowsHook(nCode, pfnFilterProc);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UnhookWindowsHookEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hhk = call.get_arg();
        let res = api.UnhookWindowsHookEx(hhk);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UnregisterClassA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpClassName = call.get_arg();
        let hInstance = call.get_arg();
        let res = api.UnregisterClassA(lpClassName, hInstance);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UnregisterClassW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpClassName = call.get_arg();
        let hInstance = call.get_arg();
        let res = api.UnregisterClassW(lpClassName, hInstance);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WaitMessage(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.WaitMessage();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WindowFromPhysicalPoint(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let Point = call.get_arg();
        let res = api.WindowFromPhysicalPoint(Point);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WindowFromPoint(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let Point = call.get_arg();
        let res = api.WindowFromPoint(Point);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_wsprintfA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param0 = call.get_arg();
        let param1 = call.get_arg();
        let res = api.wsprintfA(param0, param1);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_wsprintfW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param0 = call.get_arg();
        let param1 = call.get_arg();
        let res = api.wsprintfW(param0, param1);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_wvsprintfA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param0 = call.get_arg();
        let param1 = call.get_arg();
        let arglist = call.get_arg();
        let res = api.wvsprintfA(param0, param1, arglist);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_wvsprintfW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param0 = call.get_arg();
        let param1 = call.get_arg();
        let arglist = call.get_arg();
        let res = api.wvsprintfW(param0, param1, arglist);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
