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
        let h_object = call.get_arg();
        let res = api.CloseHandle(h_object);
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
        let h_first_object_handle = call.get_arg();
        let h_second_object_handle = call.get_arg();
        let res = api.CompareObjectHandles(h_first_object_handle, h_second_object_handle);
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
        let h_source_process_handle = call.get_arg();
        let h_source_handle = call.get_arg();
        let h_target_process_handle = call.get_arg();
        let lp_target_handle = call.get_arg();
        let dw_desired_access = call.get_arg();
        let b_inherit_handle = call.get_arg();
        let dw_options = call.get_arg();
        let res = api.DuplicateHandle(
            h_source_process_handle,
            h_source_handle,
            h_target_process_handle,
            lp_target_handle,
            dw_desired_access,
            b_inherit_handle,
            dw_options,
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
        let h_object = call.get_arg();
        let lpdw_flags = call.get_arg();
        let res = api.GetHandleInformation(h_object, lpdw_flags);
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
        let h_object = call.get_arg();
        let dw_mask = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.SetHandleInformation(h_object, dw_mask, dw_flags);
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
        let dw_err_code = call.get_arg();
        let res = api.SetLastError(dw_err_code);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CompareStringA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_cmp_flags = call.get_arg();
        let lp_string_1 = call.get_arg();
        let cch_count_1 = call.get_arg();
        let lp_string_2 = call.get_arg();
        let cch_count_2 = call.get_arg();
        let res = api.CompareStringA(
            locale,
            dw_cmp_flags,
            lp_string_1,
            cch_count_1,
            lp_string_2,
            cch_count_2,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CompareStringEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_name = call.get_arg();
        let dw_cmp_flags = call.get_arg();
        let lp_string_1 = call.get_arg();
        let cch_count_1 = call.get_arg();
        let lp_string_2 = call.get_arg();
        let cch_count_2 = call.get_arg();
        let lp_version_information = call.get_arg();
        let lp_reserved = call.get_arg();
        let l_param = call.get_arg();
        let res = api.CompareStringEx(
            lp_locale_name,
            dw_cmp_flags,
            lp_string_1,
            cch_count_1,
            lp_string_2,
            cch_count_2,
            lp_version_information,
            lp_reserved,
            l_param,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CompareStringOrdinal(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_string_1 = call.get_arg();
        let cch_count_1 = call.get_arg();
        let lp_string_2 = call.get_arg();
        let cch_count_2 = call.get_arg();
        let b_ignore_case = call.get_arg();
        let res = api.CompareStringOrdinal(
            lp_string_1,
            cch_count_1,
            lp_string_2,
            cch_count_2,
            b_ignore_case,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CompareStringW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_cmp_flags = call.get_arg();
        let lp_string_1 = call.get_arg();
        let cch_count_1 = call.get_arg();
        let lp_string_2 = call.get_arg();
        let cch_count_2 = call.get_arg();
        let res = api.CompareStringW(
            locale,
            dw_cmp_flags,
            lp_string_1,
            cch_count_1,
            lp_string_2,
            cch_count_2,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ConvertDefaultLocale(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let res = api.ConvertDefaultLocale(locale);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumCalendarInfoA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_cal_info_enum_proc = call.get_arg();
        let locale = call.get_arg();
        let calendar = call.get_arg();
        let cal_type = call.get_arg();
        let res = api.EnumCalendarInfoA(lp_cal_info_enum_proc, locale, calendar, cal_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumCalendarInfoExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_cal_info_enum_proc_ex = call.get_arg();
        let locale = call.get_arg();
        let calendar = call.get_arg();
        let cal_type = call.get_arg();
        let res = api.EnumCalendarInfoExA(lp_cal_info_enum_proc_ex, locale, calendar, cal_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumCalendarInfoExEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let p_cal_info_enum_proc_ex_ex = call.get_arg();
        let lp_locale_name = call.get_arg();
        let calendar = call.get_arg();
        let lp_reserved = call.get_arg();
        let cal_type = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumCalendarInfoExEx(
            p_cal_info_enum_proc_ex_ex,
            lp_locale_name,
            calendar,
            lp_reserved,
            cal_type,
            l_param,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumCalendarInfoExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_cal_info_enum_proc_ex = call.get_arg();
        let locale = call.get_arg();
        let calendar = call.get_arg();
        let cal_type = call.get_arg();
        let res = api.EnumCalendarInfoExW(lp_cal_info_enum_proc_ex, locale, calendar, cal_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumCalendarInfoW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_cal_info_enum_proc = call.get_arg();
        let locale = call.get_arg();
        let calendar = call.get_arg();
        let cal_type = call.get_arg();
        let res = api.EnumCalendarInfoW(lp_cal_info_enum_proc, locale, calendar, cal_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumDateFormatsA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_date_fmt_enum_proc = call.get_arg();
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.EnumDateFormatsA(lp_date_fmt_enum_proc, locale, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumDateFormatsExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_date_fmt_enum_proc_ex = call.get_arg();
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.EnumDateFormatsExA(lp_date_fmt_enum_proc_ex, locale, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumDateFormatsExEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_date_fmt_enum_proc_ex_ex = call.get_arg();
        let lp_locale_name = call.get_arg();
        let dw_flags = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumDateFormatsExEx(
            lp_date_fmt_enum_proc_ex_ex,
            lp_locale_name,
            dw_flags,
            l_param,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumDateFormatsExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_date_fmt_enum_proc_ex = call.get_arg();
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.EnumDateFormatsExW(lp_date_fmt_enum_proc_ex, locale, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumDateFormatsW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_date_fmt_enum_proc = call.get_arg();
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.EnumDateFormatsW(lp_date_fmt_enum_proc, locale, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumLanguageGroupLocalesA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_lang_group_locale_enum_proc = call.get_arg();
        let language_group = call.get_arg();
        let dw_flags = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumLanguageGroupLocalesA(
            lp_lang_group_locale_enum_proc,
            language_group,
            dw_flags,
            l_param,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumLanguageGroupLocalesW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_lang_group_locale_enum_proc = call.get_arg();
        let language_group = call.get_arg();
        let dw_flags = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumLanguageGroupLocalesW(
            lp_lang_group_locale_enum_proc,
            language_group,
            dw_flags,
            l_param,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumSystemCodePagesA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_code_page_enum_proc = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.EnumSystemCodePagesA(lp_code_page_enum_proc, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumSystemCodePagesW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_code_page_enum_proc = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.EnumSystemCodePagesW(lp_code_page_enum_proc, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumSystemGeoID(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let geo_class = call.get_arg();
        let parent_geo_id = call.get_arg();
        let lp_geo_enum_proc = call.get_arg();
        let res = api.EnumSystemGeoID(geo_class, parent_geo_id, lp_geo_enum_proc);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumSystemGeoNames(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let geo_class = call.get_arg();
        let geo_enum_proc = call.get_arg();
        let data = call.get_arg();
        let res = api.EnumSystemGeoNames(geo_class, geo_enum_proc, data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumSystemLanguageGroupsA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_language_group_enum_proc = call.get_arg();
        let dw_flags = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumSystemLanguageGroupsA(lp_language_group_enum_proc, dw_flags, l_param);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumSystemLanguageGroupsW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_language_group_enum_proc = call.get_arg();
        let dw_flags = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumSystemLanguageGroupsW(lp_language_group_enum_proc, dw_flags, l_param);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumSystemLocalesA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_enum_proc = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.EnumSystemLocalesA(lp_locale_enum_proc, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumSystemLocalesEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_enum_proc_ex = call.get_arg();
        let dw_flags = call.get_arg();
        let l_param = call.get_arg();
        let lp_reserved = call.get_arg();
        let res = api.EnumSystemLocalesEx(lp_locale_enum_proc_ex, dw_flags, l_param, lp_reserved);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumSystemLocalesW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_enum_proc = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.EnumSystemLocalesW(lp_locale_enum_proc, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumTimeFormatsA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_time_fmt_enum_proc = call.get_arg();
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.EnumTimeFormatsA(lp_time_fmt_enum_proc, locale, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumTimeFormatsEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_time_fmt_enum_proc_ex = call.get_arg();
        let lp_locale_name = call.get_arg();
        let dw_flags = call.get_arg();
        let l_param = call.get_arg();
        let res =
            api.EnumTimeFormatsEx(lp_time_fmt_enum_proc_ex, lp_locale_name, dw_flags, l_param);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumTimeFormatsW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_time_fmt_enum_proc = call.get_arg();
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.EnumTimeFormatsW(lp_time_fmt_enum_proc, locale, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumUILanguagesA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_ui_language_enum_proc = call.get_arg();
        let dw_flags = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumUILanguagesA(lp_ui_language_enum_proc, dw_flags, l_param);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumUILanguagesW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_ui_language_enum_proc = call.get_arg();
        let dw_flags = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumUILanguagesW(lp_ui_language_enum_proc, dw_flags, l_param);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindNLSString(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_find_nls_string_flags = call.get_arg();
        let lp_string_source = call.get_arg();
        let cch_source = call.get_arg();
        let lp_string_value = call.get_arg();
        let cch_value = call.get_arg();
        let pcch_found = call.get_arg();
        let res = api.FindNLSString(
            locale,
            dw_find_nls_string_flags,
            lp_string_source,
            cch_source,
            lp_string_value,
            cch_value,
            pcch_found,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindNLSStringEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_name = call.get_arg();
        let dw_find_nls_string_flags = call.get_arg();
        let lp_string_source = call.get_arg();
        let cch_source = call.get_arg();
        let lp_string_value = call.get_arg();
        let cch_value = call.get_arg();
        let pcch_found = call.get_arg();
        let lp_version_information = call.get_arg();
        let lp_reserved = call.get_arg();
        let sort_handle = call.get_arg();
        let res = api.FindNLSStringEx(
            lp_locale_name,
            dw_find_nls_string_flags,
            lp_string_source,
            cch_source,
            lp_string_value,
            cch_value,
            pcch_found,
            lp_version_information,
            lp_reserved,
            sort_handle,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindStringOrdinal(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_find_string_ordinal_flags = call.get_arg();
        let lp_string_source = call.get_arg();
        let cch_source = call.get_arg();
        let lp_string_value = call.get_arg();
        let cch_value = call.get_arg();
        let b_ignore_case = call.get_arg();
        let res = api.FindStringOrdinal(
            dw_find_string_ordinal_flags,
            lp_string_source,
            cch_source,
            lp_string_value,
            cch_value,
            b_ignore_case,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FoldStringA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_map_flags = call.get_arg();
        let lp_src_str = call.get_arg();
        let cch_src = call.get_arg();
        let lp_dest_str = call.get_arg();
        let cch_dest = call.get_arg();
        let res = api.FoldStringA(dw_map_flags, lp_src_str, cch_src, lp_dest_str, cch_dest);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FoldStringW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_map_flags = call.get_arg();
        let lp_src_str = call.get_arg();
        let cch_src = call.get_arg();
        let lp_dest_str = call.get_arg();
        let cch_dest = call.get_arg();
        let res = api.FoldStringW(dw_map_flags, lp_src_str, cch_src, lp_dest_str, cch_dest);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetACP(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetACP();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCPInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let code_page = call.get_arg();
        let lp_cp_info = call.get_arg();
        let res = api.GetCPInfo(code_page, lp_cp_info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCPInfoExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let code_page = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_cp_info_ex = call.get_arg();
        let res = api.GetCPInfoExA(code_page, dw_flags, lp_cp_info_ex);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCPInfoExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let code_page = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_cp_info_ex = call.get_arg();
        let res = api.GetCPInfoExW(code_page, dw_flags, lp_cp_info_ex);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCalendarInfoA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let calendar = call.get_arg();
        let cal_type = call.get_arg();
        let lp_cal_data = call.get_arg();
        let cch_data = call.get_arg();
        let lp_value = call.get_arg();
        let res = api.GetCalendarInfoA(locale, calendar, cal_type, lp_cal_data, cch_data, lp_value);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCalendarInfoEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_name = call.get_arg();
        let calendar = call.get_arg();
        let lp_reserved = call.get_arg();
        let cal_type = call.get_arg();
        let lp_cal_data = call.get_arg();
        let cch_data = call.get_arg();
        let lp_value = call.get_arg();
        let res = api.GetCalendarInfoEx(
            lp_locale_name,
            calendar,
            lp_reserved,
            cal_type,
            lp_cal_data,
            cch_data,
            lp_value,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCalendarInfoW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let calendar = call.get_arg();
        let cal_type = call.get_arg();
        let lp_cal_data = call.get_arg();
        let cch_data = call.get_arg();
        let lp_value = call.get_arg();
        let res = api.GetCalendarInfoW(locale, calendar, cal_type, lp_cal_data, cch_data, lp_value);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCurrencyFormatA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_value = call.get_arg();
        let lp_format = call.get_arg();
        let lp_currency_str = call.get_arg();
        let cch_currency = call.get_arg();
        let res = api.GetCurrencyFormatA(
            locale,
            dw_flags,
            lp_value,
            lp_format,
            lp_currency_str,
            cch_currency,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCurrencyFormatEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_name = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_value = call.get_arg();
        let lp_format = call.get_arg();
        let lp_currency_str = call.get_arg();
        let cch_currency = call.get_arg();
        let res = api.GetCurrencyFormatEx(
            lp_locale_name,
            dw_flags,
            lp_value,
            lp_format,
            lp_currency_str,
            cch_currency,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCurrencyFormatW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_value = call.get_arg();
        let lp_format = call.get_arg();
        let lp_currency_str = call.get_arg();
        let cch_currency = call.get_arg();
        let res = api.GetCurrencyFormatW(
            locale,
            dw_flags,
            lp_value,
            lp_format,
            lp_currency_str,
            cch_currency,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDateFormatA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_date = call.get_arg();
        let lp_format = call.get_arg();
        let lp_date_str = call.get_arg();
        let cch_date = call.get_arg();
        let res = api.GetDateFormatA(locale, dw_flags, lp_date, lp_format, lp_date_str, cch_date);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDateFormatEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_name = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_date = call.get_arg();
        let lp_format = call.get_arg();
        let lp_date_str = call.get_arg();
        let cch_date = call.get_arg();
        let lp_calendar = call.get_arg();
        let res = api.GetDateFormatEx(
            lp_locale_name,
            dw_flags,
            lp_date,
            lp_format,
            lp_date_str,
            cch_date,
            lp_calendar,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDateFormatW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_date = call.get_arg();
        let lp_format = call.get_arg();
        let lp_date_str = call.get_arg();
        let cch_date = call.get_arg();
        let res = api.GetDateFormatW(locale, dw_flags, lp_date, lp_format, lp_date_str, cch_date);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDistanceOfClosestLanguageInList(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let psz_language = call.get_arg();
        let psz_languages_list = call.get_arg();
        let wch_list_delimiter = call.get_arg();
        let p_closest_distance = call.get_arg();
        let res = api.GetDistanceOfClosestLanguageInList(
            psz_language,
            psz_languages_list,
            wch_list_delimiter,
            p_closest_distance,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDurationFormat(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_duration = call.get_arg();
        let ull_duration = call.get_arg();
        let lp_format = call.get_arg();
        let lp_duration_str = call.get_arg();
        let cch_duration = call.get_arg();
        let res = api.GetDurationFormat(
            locale,
            dw_flags,
            lp_duration,
            ull_duration,
            lp_format,
            lp_duration_str,
            cch_duration,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDurationFormatEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_name = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_duration = call.get_arg();
        let ull_duration = call.get_arg();
        let lp_format = call.get_arg();
        let lp_duration_str = call.get_arg();
        let cch_duration = call.get_arg();
        let res = api.GetDurationFormatEx(
            lp_locale_name,
            dw_flags,
            lp_duration,
            ull_duration,
            lp_format,
            lp_duration_str,
            cch_duration,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileMUIInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let pcwsz_file_path = call.get_arg();
        let p_file_mui_info = call.get_arg();
        let pcb_file_mui_info = call.get_arg();
        let res = api.GetFileMUIInfo(
            dw_flags,
            pcwsz_file_path,
            p_file_mui_info,
            pcb_file_mui_info,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileMUIPath(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let pcwsz_file_path = call.get_arg();
        let pwsz_language = call.get_arg();
        let pcch_language = call.get_arg();
        let pwsz_file_mui_path = call.get_arg();
        let pcch_file_mui_path = call.get_arg();
        let pulul_enumerator = call.get_arg();
        let res = api.GetFileMUIPath(
            dw_flags,
            pcwsz_file_path,
            pwsz_language,
            pcch_language,
            pwsz_file_mui_path,
            pcch_file_mui_path,
            pulul_enumerator,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetGeoInfoA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let location = call.get_arg();
        let geo_type = call.get_arg();
        let lp_geo_data = call.get_arg();
        let cch_data = call.get_arg();
        let lang_id = call.get_arg();
        let res = api.GetGeoInfoA(location, geo_type, lp_geo_data, cch_data, lang_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetGeoInfoEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let location = call.get_arg();
        let geo_type = call.get_arg();
        let geo_data = call.get_arg();
        let geo_data_count = call.get_arg();
        let res = api.GetGeoInfoEx(location, geo_type, geo_data, geo_data_count);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetGeoInfoW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let location = call.get_arg();
        let geo_type = call.get_arg();
        let lp_geo_data = call.get_arg();
        let cch_data = call.get_arg();
        let lang_id = call.get_arg();
        let res = api.GetGeoInfoW(location, geo_type, lp_geo_data, cch_data, lang_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLocaleInfoA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let lc_type = call.get_arg();
        let lp_lc_data = call.get_arg();
        let cch_data = call.get_arg();
        let res = api.GetLocaleInfoA(locale, lc_type, lp_lc_data, cch_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLocaleInfoEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_name = call.get_arg();
        let lc_type = call.get_arg();
        let lp_lc_data = call.get_arg();
        let cch_data = call.get_arg();
        let res = api.GetLocaleInfoEx(lp_locale_name, lc_type, lp_lc_data, cch_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLocaleInfoW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let lc_type = call.get_arg();
        let lp_lc_data = call.get_arg();
        let cch_data = call.get_arg();
        let res = api.GetLocaleInfoW(locale, lc_type, lp_lc_data, cch_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetNLSVersion(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let function = call.get_arg();
        let locale = call.get_arg();
        let lp_version_information = call.get_arg();
        let res = api.GetNLSVersion(function, locale, lp_version_information);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetNLSVersionEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let function = call.get_arg();
        let lp_locale_name = call.get_arg();
        let lp_version_information = call.get_arg();
        let res = api.GetNLSVersionEx(function, lp_locale_name, lp_version_information);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetNumberFormatA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_value = call.get_arg();
        let lp_format = call.get_arg();
        let lp_number_str = call.get_arg();
        let cch_number = call.get_arg();
        let res = api.GetNumberFormatA(
            locale,
            dw_flags,
            lp_value,
            lp_format,
            lp_number_str,
            cch_number,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetNumberFormatEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_name = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_value = call.get_arg();
        let lp_format = call.get_arg();
        let lp_number_str = call.get_arg();
        let cch_number = call.get_arg();
        let res = api.GetNumberFormatEx(
            lp_locale_name,
            dw_flags,
            lp_value,
            lp_format,
            lp_number_str,
            cch_number,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetNumberFormatW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_value = call.get_arg();
        let lp_format = call.get_arg();
        let lp_number_str = call.get_arg();
        let cch_number = call.get_arg();
        let res = api.GetNumberFormatW(
            locale,
            dw_flags,
            lp_value,
            lp_format,
            lp_number_str,
            cch_number,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetOEMCP(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetOEMCP();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetProcessPreferredUILanguages(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let pul_num_languages = call.get_arg();
        let pwsz_languages_buffer = call.get_arg();
        let pcch_languages_buffer = call.get_arg();
        let res = api.GetProcessPreferredUILanguages(
            dw_flags,
            pul_num_languages,
            pwsz_languages_buffer,
            pcch_languages_buffer,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetStringScripts(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let lp_string = call.get_arg();
        let cch_string = call.get_arg();
        let lp_scripts = call.get_arg();
        let cch_scripts = call.get_arg();
        let res = api.GetStringScripts(dw_flags, lp_string, cch_string, lp_scripts, cch_scripts);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetStringTypeA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_info_type = call.get_arg();
        let lp_src_str = call.get_arg();
        let cch_src = call.get_arg();
        let lp_char_type = call.get_arg();
        let res = api.GetStringTypeA(locale, dw_info_type, lp_src_str, cch_src, lp_char_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetStringTypeExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_info_type = call.get_arg();
        let lp_src_str = call.get_arg();
        let cch_src = call.get_arg();
        let lp_char_type = call.get_arg();
        let res = api.GetStringTypeExA(locale, dw_info_type, lp_src_str, cch_src, lp_char_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetStringTypeExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_info_type = call.get_arg();
        let lp_src_str = call.get_arg();
        let cch_src = call.get_arg();
        let lp_char_type = call.get_arg();
        let res = api.GetStringTypeExW(locale, dw_info_type, lp_src_str, cch_src, lp_char_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetStringTypeW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_info_type = call.get_arg();
        let lp_src_str = call.get_arg();
        let cch_src = call.get_arg();
        let lp_char_type = call.get_arg();
        let res = api.GetStringTypeW(dw_info_type, lp_src_str, cch_src, lp_char_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemDefaultLCID(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetSystemDefaultLCID();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemDefaultLangID(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetSystemDefaultLangID();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemDefaultLocaleName(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_name = call.get_arg();
        let cch_locale_name = call.get_arg();
        let res = api.GetSystemDefaultLocaleName(lp_locale_name, cch_locale_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemDefaultUILanguage(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetSystemDefaultUILanguage();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemPreferredUILanguages(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let pul_num_languages = call.get_arg();
        let pwsz_languages_buffer = call.get_arg();
        let pcch_languages_buffer = call.get_arg();
        let res = api.GetSystemPreferredUILanguages(
            dw_flags,
            pul_num_languages,
            pwsz_languages_buffer,
            pcch_languages_buffer,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetThreadLocale(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetThreadLocale();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetThreadPreferredUILanguages(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let pul_num_languages = call.get_arg();
        let pwsz_languages_buffer = call.get_arg();
        let pcch_languages_buffer = call.get_arg();
        let res = api.GetThreadPreferredUILanguages(
            dw_flags,
            pul_num_languages,
            pwsz_languages_buffer,
            pcch_languages_buffer,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetThreadUILanguage(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetThreadUILanguage();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTimeFormatA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_time = call.get_arg();
        let lp_format = call.get_arg();
        let lp_time_str = call.get_arg();
        let cch_time = call.get_arg();
        let res = api.GetTimeFormatA(locale, dw_flags, lp_time, lp_format, lp_time_str, cch_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTimeFormatEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_name = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_time = call.get_arg();
        let lp_format = call.get_arg();
        let lp_time_str = call.get_arg();
        let cch_time = call.get_arg();
        let res = api.GetTimeFormatEx(
            lp_locale_name,
            dw_flags,
            lp_time,
            lp_format,
            lp_time_str,
            cch_time,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTimeFormatW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_time = call.get_arg();
        let lp_format = call.get_arg();
        let lp_time_str = call.get_arg();
        let cch_time = call.get_arg();
        let res = api.GetTimeFormatW(locale, dw_flags, lp_time, lp_format, lp_time_str, cch_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetUILanguageInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let pwmsz_language = call.get_arg();
        let pwsz_fallback_languages = call.get_arg();
        let pcch_fallback_languages = call.get_arg();
        let p_attributes = call.get_arg();
        let res = api.GetUILanguageInfo(
            dw_flags,
            pwmsz_language,
            pwsz_fallback_languages,
            pcch_fallback_languages,
            p_attributes,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetUserDefaultGeoName(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let geo_name = call.get_arg();
        let geo_name_count = call.get_arg();
        let res = api.GetUserDefaultGeoName(geo_name, geo_name_count);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetUserDefaultLCID(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetUserDefaultLCID();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetUserDefaultLangID(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetUserDefaultLangID();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetUserDefaultLocaleName(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_name = call.get_arg();
        let cch_locale_name = call.get_arg();
        let res = api.GetUserDefaultLocaleName(lp_locale_name, cch_locale_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetUserDefaultUILanguage(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetUserDefaultUILanguage();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetUserGeoID(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let geo_class = call.get_arg();
        let res = api.GetUserGeoID(geo_class);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetUserPreferredUILanguages(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let pul_num_languages = call.get_arg();
        let pwsz_languages_buffer = call.get_arg();
        let pcch_languages_buffer = call.get_arg();
        let res = api.GetUserPreferredUILanguages(
            dw_flags,
            pul_num_languages,
            pwsz_languages_buffer,
            pcch_languages_buffer,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IdnToAscii(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let lp_unicode_char_str = call.get_arg();
        let cch_unicode_char = call.get_arg();
        let lp_ascii_char_str = call.get_arg();
        let cch_ascii_char = call.get_arg();
        let res = api.IdnToAscii(
            dw_flags,
            lp_unicode_char_str,
            cch_unicode_char,
            lp_ascii_char_str,
            cch_ascii_char,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IdnToNameprepUnicode(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let lp_unicode_char_str = call.get_arg();
        let cch_unicode_char = call.get_arg();
        let lp_nameprep_char_str = call.get_arg();
        let cch_nameprep_char = call.get_arg();
        let res = api.IdnToNameprepUnicode(
            dw_flags,
            lp_unicode_char_str,
            cch_unicode_char,
            lp_nameprep_char_str,
            cch_nameprep_char,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IdnToUnicode(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let lp_ascii_char_str = call.get_arg();
        let cch_ascii_char = call.get_arg();
        let lp_unicode_char_str = call.get_arg();
        let cch_unicode_char = call.get_arg();
        let res = api.IdnToUnicode(
            dw_flags,
            lp_ascii_char_str,
            cch_ascii_char,
            lp_unicode_char_str,
            cch_unicode_char,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsDBCSLeadByte(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let test_char = call.get_arg();
        let res = api.IsDBCSLeadByte(test_char);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsDBCSLeadByteEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let code_page = call.get_arg();
        let test_char = call.get_arg();
        let res = api.IsDBCSLeadByteEx(code_page, test_char);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsNLSDefinedString(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let function = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_version_information = call.get_arg();
        let lp_string = call.get_arg();
        let cch_str = call.get_arg();
        let res = api.IsNLSDefinedString(
            function,
            dw_flags,
            lp_version_information,
            lp_string,
            cch_str,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsNormalizedString(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let norm_form = call.get_arg();
        let lp_string = call.get_arg();
        let cw_length = call.get_arg();
        let res = api.IsNormalizedString(norm_form, lp_string, cw_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsTextUnicode(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpv = call.get_arg();
        let i_size = call.get_arg();
        let lpi_result = call.get_arg();
        let res = api.IsTextUnicode(lpv, i_size, lpi_result);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsValidCodePage(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let code_page = call.get_arg();
        let res = api.IsValidCodePage(code_page);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsValidLanguageGroup(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let language_group = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.IsValidLanguageGroup(language_group, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsValidLocale(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.IsValidLocale(locale, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsValidLocaleName(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_name = call.get_arg();
        let res = api.IsValidLocaleName(lp_locale_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsValidNLSVersion(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let function = call.get_arg();
        let lp_locale_name = call.get_arg();
        let lp_version_information = call.get_arg();
        let res = api.IsValidNLSVersion(function, lp_locale_name, lp_version_information);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsWellFormedTag(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let psz_tag = call.get_arg();
        let res = api.IsWellFormedTag(psz_tag);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LCIDToLocaleName(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let lp_name = call.get_arg();
        let cch_name = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.LCIDToLocaleName(locale, lp_name, cch_name, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LCMapStringA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_map_flags = call.get_arg();
        let lp_src_str = call.get_arg();
        let cch_src = call.get_arg();
        let lp_dest_str = call.get_arg();
        let cch_dest = call.get_arg();
        let res = api.LCMapStringA(
            locale,
            dw_map_flags,
            lp_src_str,
            cch_src,
            lp_dest_str,
            cch_dest,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LCMapStringEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_locale_name = call.get_arg();
        let dw_map_flags = call.get_arg();
        let lp_src_str = call.get_arg();
        let cch_src = call.get_arg();
        let lp_dest_str = call.get_arg();
        let cch_dest = call.get_arg();
        let lp_version_information = call.get_arg();
        let lp_reserved = call.get_arg();
        let sort_handle = call.get_arg();
        let res = api.LCMapStringEx(
            lp_locale_name,
            dw_map_flags,
            lp_src_str,
            cch_src,
            lp_dest_str,
            cch_dest,
            lp_version_information,
            lp_reserved,
            sort_handle,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LCMapStringW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let dw_map_flags = call.get_arg();
        let lp_src_str = call.get_arg();
        let cch_src = call.get_arg();
        let lp_dest_str = call.get_arg();
        let cch_dest = call.get_arg();
        let res = api.LCMapStringW(
            locale,
            dw_map_flags,
            lp_src_str,
            cch_src,
            lp_dest_str,
            cch_dest,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LocaleNameToLCID(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.LocaleNameToLCID(lp_name, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MappingDoAction(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let p_bag = call.get_arg();
        let dw_range_index = call.get_arg();
        let psz_action_id = call.get_arg();
        let res = api.MappingDoAction(p_bag, dw_range_index, psz_action_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MappingFreePropertyBag(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let p_bag = call.get_arg();
        let res = api.MappingFreePropertyBag(p_bag);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MappingFreeServices(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let p_service_info = call.get_arg();
        let res = api.MappingFreeServices(p_service_info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MappingGetServices(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let p_options = call.get_arg();
        let prg_services = call.get_arg();
        let pdw_services_count = call.get_arg();
        let res = api.MappingGetServices(p_options, prg_services, pdw_services_count);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MappingRecognizeText(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let p_service_info = call.get_arg();
        let psz_text = call.get_arg();
        let dw_length = call.get_arg();
        let dw_index = call.get_arg();
        let p_options = call.get_arg();
        let pbag = call.get_arg();
        let res = api.MappingRecognizeText(
            p_service_info,
            psz_text,
            dw_length,
            dw_index,
            p_options,
            pbag,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MultiByteToWideChar(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let code_page = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_multi_byte_str = call.get_arg();
        let cb_multi_byte = call.get_arg();
        let lp_wide_char_str = call.get_arg();
        let cch_wide_char = call.get_arg();
        let res = api.MultiByteToWideChar(
            code_page,
            dw_flags,
            lp_multi_byte_str,
            cb_multi_byte,
            lp_wide_char_str,
            cch_wide_char,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NormalizeString(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let norm_form = call.get_arg();
        let lp_src_string = call.get_arg();
        let cw_src_length = call.get_arg();
        let lp_dst_string = call.get_arg();
        let cw_dst_length = call.get_arg();
        let res = api.NormalizeString(
            norm_form,
            lp_src_string,
            cw_src_length,
            lp_dst_string,
            cw_dst_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NotifyUILanguageChange(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let pcwstr_new_language = call.get_arg();
        let pcwstr_previous_language = call.get_arg();
        let dw_reserved = call.get_arg();
        let pdw_status_rtrn = call.get_arg();
        let res = api.NotifyUILanguageChange(
            dw_flags,
            pcwstr_new_language,
            pcwstr_previous_language,
            dw_reserved,
            pdw_status_rtrn,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ResolveLocaleName(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name_to_resolve = call.get_arg();
        let lp_locale_name = call.get_arg();
        let cch_locale_name = call.get_arg();
        let res = api.ResolveLocaleName(lp_name_to_resolve, lp_locale_name, cch_locale_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RestoreThreadPreferredUILanguages(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let snapshot = call.get_arg();
        let res = api.RestoreThreadPreferredUILanguages(snapshot);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptApplyDigitSubstitution(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let psds = call.get_arg();
        let psc = call.get_arg();
        let pss = call.get_arg();
        let res = api.ScriptApplyDigitSubstitution(psds, psc, pss);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptBreak(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pwc_chars = call.get_arg();
        let c_chars = call.get_arg();
        let psa = call.get_arg();
        let psla = call.get_arg();
        let res = api.ScriptBreak(pwc_chars, c_chars, psa, psla);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptCPtoX(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let i_cp = call.get_arg();
        let f_trailing = call.get_arg();
        let c_chars = call.get_arg();
        let c_glyphs = call.get_arg();
        let pw_log_clust = call.get_arg();
        let psva = call.get_arg();
        let pi_advance = call.get_arg();
        let psa = call.get_arg();
        let pi_x = call.get_arg();
        let res = api.ScriptCPtoX(
            i_cp,
            f_trailing,
            c_chars,
            c_glyphs,
            pw_log_clust,
            psva,
            pi_advance,
            psa,
            pi_x,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptFreeCache(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let psc = call.get_arg();
        let res = api.ScriptFreeCache(psc);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptGetLogicalWidths(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let psa = call.get_arg();
        let c_chars = call.get_arg();
        let c_glyphs = call.get_arg();
        let pi_glyph_width = call.get_arg();
        let pw_log_clust = call.get_arg();
        let psva = call.get_arg();
        let pi_dx = call.get_arg();
        let res = api.ScriptGetLogicalWidths(
            psa,
            c_chars,
            c_glyphs,
            pi_glyph_width,
            pw_log_clust,
            psva,
            pi_dx,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptGetProperties(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pp_sp = call.get_arg();
        let pi_num_scripts = call.get_arg();
        let res = api.ScriptGetProperties(pp_sp, pi_num_scripts);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptIsComplex(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pwc_in_chars = call.get_arg();
        let c_in_chars = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.ScriptIsComplex(pwc_in_chars, c_in_chars, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptItemize(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pwc_in_chars = call.get_arg();
        let c_in_chars = call.get_arg();
        let c_max_items = call.get_arg();
        let ps_control = call.get_arg();
        let ps_state = call.get_arg();
        let p_items = call.get_arg();
        let pc_items = call.get_arg();
        let res = api.ScriptItemize(
            pwc_in_chars,
            c_in_chars,
            c_max_items,
            ps_control,
            ps_state,
            p_items,
            pc_items,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptItemizeOpenType(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pwc_in_chars = call.get_arg();
        let c_in_chars = call.get_arg();
        let c_max_items = call.get_arg();
        let ps_control = call.get_arg();
        let ps_state = call.get_arg();
        let p_items = call.get_arg();
        let p_script_tags = call.get_arg();
        let pc_items = call.get_arg();
        let res = api.ScriptItemizeOpenType(
            pwc_in_chars,
            c_in_chars,
            c_max_items,
            ps_control,
            ps_state,
            p_items,
            p_script_tags,
            pc_items,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptJustify(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let psva = call.get_arg();
        let pi_advance = call.get_arg();
        let c_glyphs = call.get_arg();
        let i_dx = call.get_arg();
        let i_min_kashida = call.get_arg();
        let pi_justify = call.get_arg();
        let res = api.ScriptJustify(psva, pi_advance, c_glyphs, i_dx, i_min_kashida, pi_justify);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptLayout(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let c_runs = call.get_arg();
        let pb_level = call.get_arg();
        let pi_visual_to_logical = call.get_arg();
        let pi_logical_to_visual = call.get_arg();
        let res = api.ScriptLayout(c_runs, pb_level, pi_visual_to_logical, pi_logical_to_visual);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptRecordDigitSubstitution(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let psds = call.get_arg();
        let res = api.ScriptRecordDigitSubstitution(locale, psds);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptStringCPtoX(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ssa = call.get_arg();
        let icp = call.get_arg();
        let f_trailing = call.get_arg();
        let p_x = call.get_arg();
        let res = api.ScriptStringCPtoX(ssa, icp, f_trailing, p_x);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptStringFree(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pssa = call.get_arg();
        let res = api.ScriptStringFree(pssa);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptStringGetLogicalWidths(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ssa = call.get_arg();
        let pi_dx = call.get_arg();
        let res = api.ScriptStringGetLogicalWidths(ssa, pi_dx);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptStringGetOrder(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ssa = call.get_arg();
        let pu_order = call.get_arg();
        let res = api.ScriptStringGetOrder(ssa, pu_order);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptStringValidate(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ssa = call.get_arg();
        let res = api.ScriptStringValidate(ssa);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptStringXtoCP(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ssa = call.get_arg();
        let i_x = call.get_arg();
        let pi_ch = call.get_arg();
        let pi_trailing = call.get_arg();
        let res = api.ScriptStringXtoCP(ssa, i_x, pi_ch, pi_trailing);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptString_pLogAttr(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ssa = call.get_arg();
        let res = api.ScriptString_pLogAttr(ssa);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptString_pSize(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ssa = call.get_arg();
        let res = api.ScriptString_pSize(ssa);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptString_pcOutChars(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ssa = call.get_arg();
        let res = api.ScriptString_pcOutChars(ssa);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScriptXtoCP(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let i_x = call.get_arg();
        let c_chars = call.get_arg();
        let c_glyphs = call.get_arg();
        let pw_log_clust = call.get_arg();
        let psva = call.get_arg();
        let pi_advance = call.get_arg();
        let psa = call.get_arg();
        let pi_cp = call.get_arg();
        let pi_trailing = call.get_arg();
        let res = api.ScriptXtoCP(
            i_x,
            c_chars,
            c_glyphs,
            pw_log_clust,
            psva,
            pi_advance,
            psa,
            pi_cp,
            pi_trailing,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetCalendarInfoA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let calendar = call.get_arg();
        let cal_type = call.get_arg();
        let lp_cal_data = call.get_arg();
        let res = api.SetCalendarInfoA(locale, calendar, cal_type, lp_cal_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetCalendarInfoW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let calendar = call.get_arg();
        let cal_type = call.get_arg();
        let lp_cal_data = call.get_arg();
        let res = api.SetCalendarInfoW(locale, calendar, cal_type, lp_cal_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetLocaleInfoA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let lc_type = call.get_arg();
        let lp_lc_data = call.get_arg();
        let res = api.SetLocaleInfoA(locale, lc_type, lp_lc_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetLocaleInfoW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let lc_type = call.get_arg();
        let lp_lc_data = call.get_arg();
        let res = api.SetLocaleInfoW(locale, lc_type, lp_lc_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetProcessPreferredUILanguages(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let pwsz_languages_buffer = call.get_arg();
        let pul_num_languages = call.get_arg();
        let res =
            api.SetProcessPreferredUILanguages(dw_flags, pwsz_languages_buffer, pul_num_languages);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetThreadLocale(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let locale = call.get_arg();
        let res = api.SetThreadLocale(locale);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetThreadPreferredUILanguages(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let pwsz_languages_buffer = call.get_arg();
        let pul_num_languages = call.get_arg();
        let res =
            api.SetThreadPreferredUILanguages(dw_flags, pwsz_languages_buffer, pul_num_languages);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetThreadPreferredUILanguages2(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let flags = call.get_arg();
        let languages = call.get_arg();
        let num_languages_set = call.get_arg();
        let snapshot = call.get_arg();
        let res = api.SetThreadPreferredUILanguages2(flags, languages, num_languages_set, snapshot);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetThreadUILanguage(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lang_id = call.get_arg();
        let res = api.SetThreadUILanguage(lang_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetUserGeoID(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let geo_id = call.get_arg();
        let res = api.SetUserGeoID(geo_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetUserGeoName(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let geo_name = call.get_arg();
        let res = api.SetUserGeoName(geo_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TranslateCharsetInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_src = call.get_arg();
        let lp_cs = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.TranslateCharsetInfo(lp_src, lp_cs, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VerifyScripts(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let lp_locale_scripts = call.get_arg();
        let cch_locale_scripts = call.get_arg();
        let lp_test_scripts = call.get_arg();
        let cch_test_scripts = call.get_arg();
        let res = api.VerifyScripts(
            dw_flags,
            lp_locale_scripts,
            cch_locale_scripts,
            lp_test_scripts,
            cch_test_scripts,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WideCharToMultiByte(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let code_page = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_wide_char_str = call.get_arg();
        let cch_wide_char = call.get_arg();
        let lp_multi_byte_str = call.get_arg();
        let cb_multi_byte = call.get_arg();
        let lp_default_char = call.get_arg();
        let lp_used_default_char = call.get_arg();
        let res = api.WideCharToMultiByte(
            code_page,
            dw_flags,
            lp_wide_char_str,
            cch_wide_char,
            lp_multi_byte_str,
            cb_multi_byte,
            lp_default_char,
            lp_used_default_char,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_lstrcatA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_string_1 = call.get_arg();
        let lp_string_2 = call.get_arg();
        let res = api.lstrcatA(lp_string_1, lp_string_2);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_lstrcatW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_string_1 = call.get_arg();
        let lp_string_2 = call.get_arg();
        let res = api.lstrcatW(lp_string_1, lp_string_2);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_lstrcmpA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_string_1 = call.get_arg();
        let lp_string_2 = call.get_arg();
        let res = api.lstrcmpA(lp_string_1, lp_string_2);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_lstrcmpW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_string_1 = call.get_arg();
        let lp_string_2 = call.get_arg();
        let res = api.lstrcmpW(lp_string_1, lp_string_2);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_lstrcmpiA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_string_1 = call.get_arg();
        let lp_string_2 = call.get_arg();
        let res = api.lstrcmpiA(lp_string_1, lp_string_2);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_lstrcmpiW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_string_1 = call.get_arg();
        let lp_string_2 = call.get_arg();
        let res = api.lstrcmpiW(lp_string_1, lp_string_2);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_lstrcpyA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_string_1 = call.get_arg();
        let lp_string_2 = call.get_arg();
        let res = api.lstrcpyA(lp_string_1, lp_string_2);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_lstrcpyW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_string_1 = call.get_arg();
        let lp_string_2 = call.get_arg();
        let res = api.lstrcpyW(lp_string_1, lp_string_2);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_lstrcpynA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_string_1 = call.get_arg();
        let lp_string_2 = call.get_arg();
        let i_max_length = call.get_arg();
        let res = api.lstrcpynA(lp_string_1, lp_string_2, i_max_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_lstrcpynW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_string_1 = call.get_arg();
        let lp_string_2 = call.get_arg();
        let i_max_length = call.get_arg();
        let res = api.lstrcpynW(lp_string_1, lp_string_2, i_max_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_lstrlenA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_string = call.get_arg();
        let res = api.lstrlenA(lp_string);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_lstrlenW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Globalization::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_string = call.get_arg();
        let res = api.lstrlenW(lp_string);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AreFileApisANSI(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.AreFileApisANSI();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AreShortNamesEnabled(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let handle = call.get_arg();
        let enabled = call.get_arg();
        let res = api.AreShortNamesEnabled(handle, enabled);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_BackupRead(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_number_of_bytes_to_read = call.get_arg();
        let lp_number_of_bytes_read = call.get_arg();
        let b_abort = call.get_arg();
        let b_process_security = call.get_arg();
        let lp_context = call.get_arg();
        let res = api.BackupRead(
            h_file,
            lp_buffer,
            n_number_of_bytes_to_read,
            lp_number_of_bytes_read,
            b_abort,
            b_process_security,
            lp_context,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_BackupSeek(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let dw_low_bytes_to_seek = call.get_arg();
        let dw_high_bytes_to_seek = call.get_arg();
        let lpdw_low_byte_seeked = call.get_arg();
        let lpdw_high_byte_seeked = call.get_arg();
        let lp_context = call.get_arg();
        let res = api.BackupSeek(
            h_file,
            dw_low_bytes_to_seek,
            dw_high_bytes_to_seek,
            lpdw_low_byte_seeked,
            lpdw_high_byte_seeked,
            lp_context,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_BackupWrite(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_number_of_bytes_to_write = call.get_arg();
        let lp_number_of_bytes_written = call.get_arg();
        let b_abort = call.get_arg();
        let b_process_security = call.get_arg();
        let lp_context = call.get_arg();
        let res = api.BackupWrite(
            h_file,
            lp_buffer,
            n_number_of_bytes_to_write,
            lp_number_of_bytes_written,
            b_abort,
            b_process_security,
            lp_context,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_BuildIoRingCancelRequest(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let io_ring = call.get_arg();
        let file = call.get_arg();
        let op_to_cancel = call.get_arg();
        let user_data = call.get_arg();
        let res = api.BuildIoRingCancelRequest(io_ring, file, op_to_cancel, user_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_BuildIoRingReadFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let io_ring = call.get_arg();
        let file_ref = call.get_arg();
        let data_ref = call.get_arg();
        let number_of_bytes_to_read = call.get_arg();
        let file_offset = call.get_arg();
        let user_data = call.get_arg();
        let flags = call.get_arg();
        let res = api.BuildIoRingReadFile(
            io_ring,
            file_ref,
            data_ref,
            number_of_bytes_to_read,
            file_offset,
            user_data,
            flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_BuildIoRingRegisterBuffers(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let io_ring = call.get_arg();
        let count = call.get_arg();
        let buffers = call.get_arg();
        let user_data = call.get_arg();
        let res = api.BuildIoRingRegisterBuffers(io_ring, count, buffers, user_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_BuildIoRingRegisterFileHandles(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let io_ring = call.get_arg();
        let count = call.get_arg();
        let handles = call.get_arg();
        let user_data = call.get_arg();
        let res = api.BuildIoRingRegisterFileHandles(io_ring, count, handles, user_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CheckNameLegalDOS8Dot3A(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let lp_oem_name = call.get_arg();
        let oem_name_size = call.get_arg();
        let pb_name_contains_spaces = call.get_arg();
        let pb_name_legal = call.get_arg();
        let res = api.CheckNameLegalDOS8Dot3A(
            lp_name,
            lp_oem_name,
            oem_name_size,
            pb_name_contains_spaces,
            pb_name_legal,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CheckNameLegalDOS8Dot3W(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let lp_oem_name = call.get_arg();
        let oem_name_size = call.get_arg();
        let pb_name_contains_spaces = call.get_arg();
        let pb_name_legal = call.get_arg();
        let res = api.CheckNameLegalDOS8Dot3W(
            lp_name,
            lp_oem_name,
            oem_name_size,
            pb_name_contains_spaces,
            pb_name_legal,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CloseEncryptedFileRaw(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pv_context = call.get_arg();
        let res = api.CloseEncryptedFileRaw(pv_context);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CloseIoRing(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let io_ring = call.get_arg();
        let res = api.CloseIoRing(io_ring);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CommitComplete(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enlistment_handle = call.get_arg();
        let tm_virtual_clock = call.get_arg();
        let res = api.CommitComplete(enlistment_handle, tm_virtual_clock);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CommitEnlistment(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enlistment_handle = call.get_arg();
        let tm_virtual_clock = call.get_arg();
        let res = api.CommitEnlistment(enlistment_handle, tm_virtual_clock);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CommitTransaction(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let transaction_handle = call.get_arg();
        let res = api.CommitTransaction(transaction_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CommitTransactionAsync(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let transaction_handle = call.get_arg();
        let res = api.CommitTransactionAsync(transaction_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CompareFileTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_time_1 = call.get_arg();
        let lp_file_time_2 = call.get_arg();
        let res = api.CompareFileTime(lp_file_time_1, lp_file_time_2);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CopyFile2(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pwsz_existing_file_name = call.get_arg();
        let pwsz_new_file_name = call.get_arg();
        let p_extended_parameters = call.get_arg();
        let res = api.CopyFile2(
            pwsz_existing_file_name,
            pwsz_new_file_name,
            p_extended_parameters,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CopyFileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let b_fail_if_exists = call.get_arg();
        let res = api.CopyFileA(lp_existing_file_name, lp_new_file_name, b_fail_if_exists);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CopyFileExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let lp_progress_routine = call.get_arg();
        let lp_data = call.get_arg();
        let pb_cancel = call.get_arg();
        let dw_copy_flags = call.get_arg();
        let res = api.CopyFileExA(
            lp_existing_file_name,
            lp_new_file_name,
            lp_progress_routine,
            lp_data,
            pb_cancel,
            dw_copy_flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CopyFileExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let lp_progress_routine = call.get_arg();
        let lp_data = call.get_arg();
        let pb_cancel = call.get_arg();
        let dw_copy_flags = call.get_arg();
        let res = api.CopyFileExW(
            lp_existing_file_name,
            lp_new_file_name,
            lp_progress_routine,
            lp_data,
            pb_cancel,
            dw_copy_flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CopyFileFromAppW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let b_fail_if_exists = call.get_arg();
        let res = api.CopyFileFromAppW(lp_existing_file_name, lp_new_file_name, b_fail_if_exists);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CopyFileTransactedA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let lp_progress_routine = call.get_arg();
        let lp_data = call.get_arg();
        let pb_cancel = call.get_arg();
        let dw_copy_flags = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.CopyFileTransactedA(
            lp_existing_file_name,
            lp_new_file_name,
            lp_progress_routine,
            lp_data,
            pb_cancel,
            dw_copy_flags,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CopyFileTransactedW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let lp_progress_routine = call.get_arg();
        let lp_data = call.get_arg();
        let pb_cancel = call.get_arg();
        let dw_copy_flags = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.CopyFileTransactedW(
            lp_existing_file_name,
            lp_new_file_name,
            lp_progress_routine,
            lp_data,
            pb_cancel,
            dw_copy_flags,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CopyFileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let b_fail_if_exists = call.get_arg();
        let res = api.CopyFileW(lp_existing_file_name, lp_new_file_name, b_fail_if_exists);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CopyLZFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hf_source = call.get_arg();
        let hf_dest = call.get_arg();
        let res = api.CopyLZFile(hf_source, hf_dest);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateIoRing(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ioring_version = call.get_arg();
        let flags = call.get_arg();
        let submission_queue_size = call.get_arg();
        let completion_queue_size = call.get_arg();
        let h = call.get_arg();
        let res = api.CreateIoRing(
            ioring_version,
            flags,
            submission_queue_size,
            completion_queue_size,
            h,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateSymbolicLinkA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_symlink_file_name = call.get_arg();
        let lp_target_file_name = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.CreateSymbolicLinkA(lp_symlink_file_name, lp_target_file_name, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateSymbolicLinkTransactedA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_symlink_file_name = call.get_arg();
        let lp_target_file_name = call.get_arg();
        let dw_flags = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.CreateSymbolicLinkTransactedA(
            lp_symlink_file_name,
            lp_target_file_name,
            dw_flags,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateSymbolicLinkTransactedW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_symlink_file_name = call.get_arg();
        let lp_target_file_name = call.get_arg();
        let dw_flags = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.CreateSymbolicLinkTransactedW(
            lp_symlink_file_name,
            lp_target_file_name,
            dw_flags,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateSymbolicLinkW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_symlink_file_name = call.get_arg();
        let lp_target_file_name = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.CreateSymbolicLinkW(lp_symlink_file_name, lp_target_file_name, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateTapePartition(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_device = call.get_arg();
        let dw_partition_method = call.get_arg();
        let dw_count = call.get_arg();
        let dw_size = call.get_arg();
        let res = api.CreateTapePartition(h_device, dw_partition_method, dw_count, dw_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DecryptFileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let dw_reserved = call.get_arg();
        let res = api.DecryptFileA(lp_file_name, dw_reserved);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DecryptFileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let dw_reserved = call.get_arg();
        let res = api.DecryptFileW(lp_file_name, dw_reserved);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DefineDosDeviceA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let lp_device_name = call.get_arg();
        let lp_target_path = call.get_arg();
        let res = api.DefineDosDeviceA(dw_flags, lp_device_name, lp_target_path);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DefineDosDeviceW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let lp_device_name = call.get_arg();
        let lp_target_path = call.get_arg();
        let res = api.DefineDosDeviceW(dw_flags, lp_device_name, lp_target_path);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DeleteFileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let res = api.DeleteFileA(lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DeleteFileFromAppW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let res = api.DeleteFileFromAppW(lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DeleteFileTransactedA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.DeleteFileTransactedA(lp_file_name, h_transaction);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DeleteFileTransactedW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.DeleteFileTransactedW(lp_file_name, h_transaction);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DeleteFileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let res = api.DeleteFileW(lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DeleteVolumeMountPointA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_volume_mount_point = call.get_arg();
        let res = api.DeleteVolumeMountPointA(lpsz_volume_mount_point);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DeleteVolumeMountPointW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_volume_mount_point = call.get_arg();
        let res = api.DeleteVolumeMountPointW(lpsz_volume_mount_point);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EncryptFileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let res = api.EncryptFileA(lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EncryptFileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let res = api.EncryptFileW(lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EncryptionDisable(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dir_path = call.get_arg();
        let disable = call.get_arg();
        let res = api.EncryptionDisable(dir_path, disable);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EraseTape(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_device = call.get_arg();
        let dw_erase_type = call.get_arg();
        let b_immediate = call.get_arg();
        let res = api.EraseTape(h_device, dw_erase_type, b_immediate);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FileEncryptionStatusA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let lp_status = call.get_arg();
        let res = api.FileEncryptionStatusA(lp_file_name, lp_status);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FileEncryptionStatusW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let lp_status = call.get_arg();
        let res = api.FileEncryptionStatusW(lp_file_name, lp_status);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FileTimeToLocalFileTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_time = call.get_arg();
        let lp_local_file_time = call.get_arg();
        let res = api.FileTimeToLocalFileTime(lp_file_time, lp_local_file_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindClose(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_find_file = call.get_arg();
        let res = api.FindClose(h_find_file);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindCloseChangeNotification(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_change_handle = call.get_arg();
        let res = api.FindCloseChangeNotification(h_change_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstChangeNotificationA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let b_watch_subtree = call.get_arg();
        let dw_notify_filter = call.get_arg();
        let res = api.FindFirstChangeNotificationA(lp_path_name, b_watch_subtree, dw_notify_filter);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstChangeNotificationW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let b_watch_subtree = call.get_arg();
        let dw_notify_filter = call.get_arg();
        let res = api.FindFirstChangeNotificationW(lp_path_name, b_watch_subtree, dw_notify_filter);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstFileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let lp_find_file_data = call.get_arg();
        let res = api.FindFirstFileA(lp_file_name, lp_find_file_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstFileExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let f_info_level_id = call.get_arg();
        let lp_find_file_data = call.get_arg();
        let f_search_op = call.get_arg();
        let lp_search_filter = call.get_arg();
        let dw_additional_flags = call.get_arg();
        let res = api.FindFirstFileExA(
            lp_file_name,
            f_info_level_id,
            lp_find_file_data,
            f_search_op,
            lp_search_filter,
            dw_additional_flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstFileExFromAppW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let f_info_level_id = call.get_arg();
        let lp_find_file_data = call.get_arg();
        let f_search_op = call.get_arg();
        let lp_search_filter = call.get_arg();
        let dw_additional_flags = call.get_arg();
        let res = api.FindFirstFileExFromAppW(
            lp_file_name,
            f_info_level_id,
            lp_find_file_data,
            f_search_op,
            lp_search_filter,
            dw_additional_flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstFileExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let f_info_level_id = call.get_arg();
        let lp_find_file_data = call.get_arg();
        let f_search_op = call.get_arg();
        let lp_search_filter = call.get_arg();
        let dw_additional_flags = call.get_arg();
        let res = api.FindFirstFileExW(
            lp_file_name,
            f_info_level_id,
            lp_find_file_data,
            f_search_op,
            lp_search_filter,
            dw_additional_flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstFileNameTransactedW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let dw_flags = call.get_arg();
        let string_length = call.get_arg();
        let link_name = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.FindFirstFileNameTransactedW(
            lp_file_name,
            dw_flags,
            string_length,
            link_name,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstFileNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let dw_flags = call.get_arg();
        let string_length = call.get_arg();
        let link_name = call.get_arg();
        let res = api.FindFirstFileNameW(lp_file_name, dw_flags, string_length, link_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstFileTransactedA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let f_info_level_id = call.get_arg();
        let lp_find_file_data = call.get_arg();
        let f_search_op = call.get_arg();
        let lp_search_filter = call.get_arg();
        let dw_additional_flags = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.FindFirstFileTransactedA(
            lp_file_name,
            f_info_level_id,
            lp_find_file_data,
            f_search_op,
            lp_search_filter,
            dw_additional_flags,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstFileTransactedW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let f_info_level_id = call.get_arg();
        let lp_find_file_data = call.get_arg();
        let f_search_op = call.get_arg();
        let lp_search_filter = call.get_arg();
        let dw_additional_flags = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.FindFirstFileTransactedW(
            lp_file_name,
            f_info_level_id,
            lp_find_file_data,
            f_search_op,
            lp_search_filter,
            dw_additional_flags,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstFileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let lp_find_file_data = call.get_arg();
        let res = api.FindFirstFileW(lp_file_name, lp_find_file_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstStreamTransactedW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let info_level = call.get_arg();
        let lp_find_stream_data = call.get_arg();
        let dw_flags = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.FindFirstStreamTransactedW(
            lp_file_name,
            info_level,
            lp_find_stream_data,
            dw_flags,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstStreamW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let info_level = call.get_arg();
        let lp_find_stream_data = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.FindFirstStreamW(lp_file_name, info_level, lp_find_stream_data, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstVolumeA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_volume_name = call.get_arg();
        let cch_buffer_length = call.get_arg();
        let res = api.FindFirstVolumeA(lpsz_volume_name, cch_buffer_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstVolumeMountPointA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_root_path_name = call.get_arg();
        let lpsz_volume_mount_point = call.get_arg();
        let cch_buffer_length = call.get_arg();
        let res = api.FindFirstVolumeMountPointA(
            lpsz_root_path_name,
            lpsz_volume_mount_point,
            cch_buffer_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstVolumeMountPointW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_root_path_name = call.get_arg();
        let lpsz_volume_mount_point = call.get_arg();
        let cch_buffer_length = call.get_arg();
        let res = api.FindFirstVolumeMountPointW(
            lpsz_root_path_name,
            lpsz_volume_mount_point,
            cch_buffer_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindFirstVolumeW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_volume_name = call.get_arg();
        let cch_buffer_length = call.get_arg();
        let res = api.FindFirstVolumeW(lpsz_volume_name, cch_buffer_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindNextChangeNotification(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_change_handle = call.get_arg();
        let res = api.FindNextChangeNotification(h_change_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindNextFileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_find_file = call.get_arg();
        let lp_find_file_data = call.get_arg();
        let res = api.FindNextFileA(h_find_file, lp_find_file_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindNextFileNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_find_stream = call.get_arg();
        let string_length = call.get_arg();
        let link_name = call.get_arg();
        let res = api.FindNextFileNameW(h_find_stream, string_length, link_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindNextFileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_find_file = call.get_arg();
        let lp_find_file_data = call.get_arg();
        let res = api.FindNextFileW(h_find_file, lp_find_file_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindNextStreamW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_find_stream = call.get_arg();
        let lp_find_stream_data = call.get_arg();
        let res = api.FindNextStreamW(h_find_stream, lp_find_stream_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindNextVolumeA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_find_volume = call.get_arg();
        let lpsz_volume_name = call.get_arg();
        let cch_buffer_length = call.get_arg();
        let res = api.FindNextVolumeA(h_find_volume, lpsz_volume_name, cch_buffer_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindNextVolumeMountPointA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_find_volume_mount_point = call.get_arg();
        let lpsz_volume_mount_point = call.get_arg();
        let cch_buffer_length = call.get_arg();
        let res = api.FindNextVolumeMountPointA(
            h_find_volume_mount_point,
            lpsz_volume_mount_point,
            cch_buffer_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindNextVolumeMountPointW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_find_volume_mount_point = call.get_arg();
        let lpsz_volume_mount_point = call.get_arg();
        let cch_buffer_length = call.get_arg();
        let res = api.FindNextVolumeMountPointW(
            h_find_volume_mount_point,
            lpsz_volume_mount_point,
            cch_buffer_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindNextVolumeW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_find_volume = call.get_arg();
        let lpsz_volume_name = call.get_arg();
        let cch_buffer_length = call.get_arg();
        let res = api.FindNextVolumeW(h_find_volume, lpsz_volume_name, cch_buffer_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindVolumeClose(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_find_volume = call.get_arg();
        let res = api.FindVolumeClose(h_find_volume);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindVolumeMountPointClose(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_find_volume_mount_point = call.get_arg();
        let res = api.FindVolumeMountPointClose(h_find_volume_mount_point);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FlushFileBuffers(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let res = api.FlushFileBuffers(h_file);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FreeEncryptedFileMetadata(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pb_metadata = call.get_arg();
        let res = api.FreeEncryptedFileMetadata(pb_metadata);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetBinaryTypeA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_application_name = call.get_arg();
        let lp_binary_type = call.get_arg();
        let res = api.GetBinaryTypeA(lp_application_name, lp_binary_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetBinaryTypeW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_application_name = call.get_arg();
        let lp_binary_type = call.get_arg();
        let res = api.GetBinaryTypeW(lp_application_name, lp_binary_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCompressedFileSizeA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let lp_file_size_high = call.get_arg();
        let res = api.GetCompressedFileSizeA(lp_file_name, lp_file_size_high);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCompressedFileSizeTransactedA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let lp_file_size_high = call.get_arg();
        let h_transaction = call.get_arg();
        let res =
            api.GetCompressedFileSizeTransactedA(lp_file_name, lp_file_size_high, h_transaction);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCompressedFileSizeTransactedW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let lp_file_size_high = call.get_arg();
        let h_transaction = call.get_arg();
        let res =
            api.GetCompressedFileSizeTransactedW(lp_file_name, lp_file_size_high, h_transaction);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCompressedFileSizeW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let lp_file_size_high = call.get_arg();
        let res = api.GetCompressedFileSizeW(lp_file_name, lp_file_size_high);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCurrentClockTransactionManager(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let transaction_manager_handle = call.get_arg();
        let tm_virtual_clock = call.get_arg();
        let res =
            api.GetCurrentClockTransactionManager(transaction_manager_handle, tm_virtual_clock);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDiskFreeSpaceA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_root_path_name = call.get_arg();
        let lp_sectors_per_cluster = call.get_arg();
        let lp_bytes_per_sector = call.get_arg();
        let lp_number_of_free_clusters = call.get_arg();
        let lp_total_number_of_clusters = call.get_arg();
        let res = api.GetDiskFreeSpaceA(
            lp_root_path_name,
            lp_sectors_per_cluster,
            lp_bytes_per_sector,
            lp_number_of_free_clusters,
            lp_total_number_of_clusters,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDiskFreeSpaceExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_directory_name = call.get_arg();
        let lp_free_bytes_available_to_caller = call.get_arg();
        let lp_total_number_of_bytes = call.get_arg();
        let lp_total_number_of_free_bytes = call.get_arg();
        let res = api.GetDiskFreeSpaceExA(
            lp_directory_name,
            lp_free_bytes_available_to_caller,
            lp_total_number_of_bytes,
            lp_total_number_of_free_bytes,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDiskFreeSpaceExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_directory_name = call.get_arg();
        let lp_free_bytes_available_to_caller = call.get_arg();
        let lp_total_number_of_bytes = call.get_arg();
        let lp_total_number_of_free_bytes = call.get_arg();
        let res = api.GetDiskFreeSpaceExW(
            lp_directory_name,
            lp_free_bytes_available_to_caller,
            lp_total_number_of_bytes,
            lp_total_number_of_free_bytes,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDiskFreeSpaceW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_root_path_name = call.get_arg();
        let lp_sectors_per_cluster = call.get_arg();
        let lp_bytes_per_sector = call.get_arg();
        let lp_number_of_free_clusters = call.get_arg();
        let lp_total_number_of_clusters = call.get_arg();
        let res = api.GetDiskFreeSpaceW(
            lp_root_path_name,
            lp_sectors_per_cluster,
            lp_bytes_per_sector,
            lp_number_of_free_clusters,
            lp_total_number_of_clusters,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDiskSpaceInformationA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let root_path = call.get_arg();
        let disk_space_info = call.get_arg();
        let res = api.GetDiskSpaceInformationA(root_path, disk_space_info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDiskSpaceInformationW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let root_path = call.get_arg();
        let disk_space_info = call.get_arg();
        let res = api.GetDiskSpaceInformationW(root_path, disk_space_info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDriveTypeA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_root_path_name = call.get_arg();
        let res = api.GetDriveTypeA(lp_root_path_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDriveTypeW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_root_path_name = call.get_arg();
        let res = api.GetDriveTypeW(lp_root_path_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetEncryptedFileMetadata(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let pcb_metadata = call.get_arg();
        let ppb_metadata = call.get_arg();
        let res = api.GetEncryptedFileMetadata(lp_file_name, pcb_metadata, ppb_metadata);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetEnlistmentId(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enlistment_handle = call.get_arg();
        let enlistment_id = call.get_arg();
        let res = api.GetEnlistmentId(enlistment_handle, enlistment_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetEnlistmentRecoveryInformation(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enlistment_handle = call.get_arg();
        let buffer_size = call.get_arg();
        let buffer = call.get_arg();
        let buffer_used = call.get_arg();
        let res = api.GetEnlistmentRecoveryInformation(
            enlistment_handle,
            buffer_size,
            buffer,
            buffer_used,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetExpandedNameA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_source = call.get_arg();
        let lpsz_buffer = call.get_arg();
        let res = api.GetExpandedNameA(lpsz_source, lpsz_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetExpandedNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_source = call.get_arg();
        let lpsz_buffer = call.get_arg();
        let res = api.GetExpandedNameW(lpsz_source, lpsz_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileAttributesA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let res = api.GetFileAttributesA(lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileAttributesExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let f_info_level_id = call.get_arg();
        let lp_file_information = call.get_arg();
        let res = api.GetFileAttributesExA(lp_file_name, f_info_level_id, lp_file_information);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileAttributesExFromAppW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let f_info_level_id = call.get_arg();
        let lp_file_information = call.get_arg();
        let res =
            api.GetFileAttributesExFromAppW(lp_file_name, f_info_level_id, lp_file_information);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileAttributesExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let f_info_level_id = call.get_arg();
        let lp_file_information = call.get_arg();
        let res = api.GetFileAttributesExW(lp_file_name, f_info_level_id, lp_file_information);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileAttributesTransactedA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let f_info_level_id = call.get_arg();
        let lp_file_information = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.GetFileAttributesTransactedA(
            lp_file_name,
            f_info_level_id,
            lp_file_information,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileAttributesTransactedW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let f_info_level_id = call.get_arg();
        let lp_file_information = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.GetFileAttributesTransactedW(
            lp_file_name,
            f_info_level_id,
            lp_file_information,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileAttributesW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let res = api.GetFileAttributesW(lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileBandwidthReservation(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_period_milliseconds = call.get_arg();
        let lp_bytes_per_period = call.get_arg();
        let p_discardable = call.get_arg();
        let lp_transfer_size = call.get_arg();
        let lp_num_outstanding_requests = call.get_arg();
        let res = api.GetFileBandwidthReservation(
            h_file,
            lp_period_milliseconds,
            lp_bytes_per_period,
            p_discardable,
            lp_transfer_size,
            lp_num_outstanding_requests,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileInformationByHandle(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_file_information = call.get_arg();
        let res = api.GetFileInformationByHandle(h_file, lp_file_information);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileInformationByHandleEx(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let file_information_class = call.get_arg();
        let lp_file_information = call.get_arg();
        let dw_buffer_size = call.get_arg();
        let res = api.GetFileInformationByHandleEx(
            h_file,
            file_information_class,
            lp_file_information,
            dw_buffer_size,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileSize(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_file_size_high = call.get_arg();
        let res = api.GetFileSize(h_file, lp_file_size_high);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileSizeEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_file_size = call.get_arg();
        let res = api.GetFileSizeEx(h_file, lp_file_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_creation_time = call.get_arg();
        let lp_last_access_time = call.get_arg();
        let lp_last_write_time = call.get_arg();
        let res = api.GetFileTime(
            h_file,
            lp_creation_time,
            lp_last_access_time,
            lp_last_write_time,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileType(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let res = api.GetFileType(h_file);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileVersionInfoA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lptstr_filename = call.get_arg();
        let dw_handle = call.get_arg();
        let dw_len = call.get_arg();
        let lp_data = call.get_arg();
        let res = api.GetFileVersionInfoA(lptstr_filename, dw_handle, dw_len, lp_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileVersionInfoExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let lpwstr_filename = call.get_arg();
        let dw_handle = call.get_arg();
        let dw_len = call.get_arg();
        let lp_data = call.get_arg();
        let res = api.GetFileVersionInfoExA(dw_flags, lpwstr_filename, dw_handle, dw_len, lp_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileVersionInfoExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let lpwstr_filename = call.get_arg();
        let dw_handle = call.get_arg();
        let dw_len = call.get_arg();
        let lp_data = call.get_arg();
        let res = api.GetFileVersionInfoExW(dw_flags, lpwstr_filename, dw_handle, dw_len, lp_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileVersionInfoSizeA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lptstr_filename = call.get_arg();
        let lpdw_handle = call.get_arg();
        let res = api.GetFileVersionInfoSizeA(lptstr_filename, lpdw_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileVersionInfoSizeExA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let lpwstr_filename = call.get_arg();
        let lpdw_handle = call.get_arg();
        let res = api.GetFileVersionInfoSizeExA(dw_flags, lpwstr_filename, lpdw_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileVersionInfoSizeExW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let lpwstr_filename = call.get_arg();
        let lpdw_handle = call.get_arg();
        let res = api.GetFileVersionInfoSizeExW(dw_flags, lpwstr_filename, lpdw_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileVersionInfoSizeW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lptstr_filename = call.get_arg();
        let lpdw_handle = call.get_arg();
        let res = api.GetFileVersionInfoSizeW(lptstr_filename, lpdw_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFileVersionInfoW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lptstr_filename = call.get_arg();
        let dw_handle = call.get_arg();
        let dw_len = call.get_arg();
        let lp_data = call.get_arg();
        let res = api.GetFileVersionInfoW(lptstr_filename, dw_handle, dw_len, lp_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFinalPathNameByHandleA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lpsz_file_path = call.get_arg();
        let cch_file_path = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.GetFinalPathNameByHandleA(h_file, lpsz_file_path, cch_file_path, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFinalPathNameByHandleW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lpsz_file_path = call.get_arg();
        let cch_file_path = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.GetFinalPathNameByHandleW(h_file, lpsz_file_path, cch_file_path, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFullPathNameA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let n_buffer_length = call.get_arg();
        let lp_buffer = call.get_arg();
        let lp_file_part = call.get_arg();
        let res = api.GetFullPathNameA(lp_file_name, n_buffer_length, lp_buffer, lp_file_part);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFullPathNameTransactedA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let n_buffer_length = call.get_arg();
        let lp_buffer = call.get_arg();
        let lp_file_part = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.GetFullPathNameTransactedA(
            lp_file_name,
            n_buffer_length,
            lp_buffer,
            lp_file_part,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFullPathNameTransactedW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let n_buffer_length = call.get_arg();
        let lp_buffer = call.get_arg();
        let lp_file_part = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.GetFullPathNameTransactedW(
            lp_file_name,
            n_buffer_length,
            lp_buffer,
            lp_file_part,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFullPathNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let n_buffer_length = call.get_arg();
        let lp_buffer = call.get_arg();
        let lp_file_part = call.get_arg();
        let res = api.GetFullPathNameW(lp_file_name, n_buffer_length, lp_buffer, lp_file_part);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetIoRingInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let io_ring = call.get_arg();
        let info = call.get_arg();
        let res = api.GetIoRingInfo(io_ring, info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLogicalDriveStringsA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let n_buffer_length = call.get_arg();
        let lp_buffer = call.get_arg();
        let res = api.GetLogicalDriveStringsA(n_buffer_length, lp_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLogicalDriveStringsW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let n_buffer_length = call.get_arg();
        let lp_buffer = call.get_arg();
        let res = api.GetLogicalDriveStringsW(n_buffer_length, lp_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLogicalDrives(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetLogicalDrives();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLongPathNameA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_short_path = call.get_arg();
        let lpsz_long_path = call.get_arg();
        let cch_buffer = call.get_arg();
        let res = api.GetLongPathNameA(lpsz_short_path, lpsz_long_path, cch_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLongPathNameTransactedA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_short_path = call.get_arg();
        let lpsz_long_path = call.get_arg();
        let cch_buffer = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.GetLongPathNameTransactedA(
            lpsz_short_path,
            lpsz_long_path,
            cch_buffer,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLongPathNameTransactedW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_short_path = call.get_arg();
        let lpsz_long_path = call.get_arg();
        let cch_buffer = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.GetLongPathNameTransactedW(
            lpsz_short_path,
            lpsz_long_path,
            cch_buffer,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLongPathNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_short_path = call.get_arg();
        let lpsz_long_path = call.get_arg();
        let cch_buffer = call.get_arg();
        let res = api.GetLongPathNameW(lpsz_short_path, lpsz_long_path, cch_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetNotificationResourceManager(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let resource_manager_handle = call.get_arg();
        let transaction_notification = call.get_arg();
        let notification_length = call.get_arg();
        let dw_milliseconds = call.get_arg();
        let return_length = call.get_arg();
        let res = api.GetNotificationResourceManager(
            resource_manager_handle,
            transaction_notification,
            notification_length,
            dw_milliseconds,
            return_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetShortPathNameA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_long_path = call.get_arg();
        let lpsz_short_path = call.get_arg();
        let cch_buffer = call.get_arg();
        let res = api.GetShortPathNameA(lpsz_long_path, lpsz_short_path, cch_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetShortPathNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_long_path = call.get_arg();
        let lpsz_short_path = call.get_arg();
        let cch_buffer = call.get_arg();
        let res = api.GetShortPathNameW(lpsz_long_path, lpsz_short_path, cch_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTapeParameters(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_device = call.get_arg();
        let dw_operation = call.get_arg();
        let lpdw_size = call.get_arg();
        let lp_tape_information = call.get_arg();
        let res = api.GetTapeParameters(h_device, dw_operation, lpdw_size, lp_tape_information);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTapePosition(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_device = call.get_arg();
        let dw_position_type = call.get_arg();
        let lpdw_partition = call.get_arg();
        let lpdw_offset_low = call.get_arg();
        let lpdw_offset_high = call.get_arg();
        let res = api.GetTapePosition(
            h_device,
            dw_position_type,
            lpdw_partition,
            lpdw_offset_low,
            lpdw_offset_high,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTapeStatus(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_device = call.get_arg();
        let res = api.GetTapeStatus(h_device);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTempFileNameA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let lp_prefix_string = call.get_arg();
        let u_unique = call.get_arg();
        let lp_temp_file_name = call.get_arg();
        let res = api.GetTempFileNameA(lp_path_name, lp_prefix_string, u_unique, lp_temp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTempFileNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let lp_prefix_string = call.get_arg();
        let u_unique = call.get_arg();
        let lp_temp_file_name = call.get_arg();
        let res = api.GetTempFileNameW(lp_path_name, lp_prefix_string, u_unique, lp_temp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTempPath2A(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let buffer_length = call.get_arg();
        let buffer = call.get_arg();
        let res = api.GetTempPath2A(buffer_length, buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTempPath2W(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let buffer_length = call.get_arg();
        let buffer = call.get_arg();
        let res = api.GetTempPath2W(buffer_length, buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTempPathA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let n_buffer_length = call.get_arg();
        let lp_buffer = call.get_arg();
        let res = api.GetTempPathA(n_buffer_length, lp_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTempPathW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let n_buffer_length = call.get_arg();
        let lp_buffer = call.get_arg();
        let res = api.GetTempPathW(n_buffer_length, lp_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTransactionId(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let transaction_handle = call.get_arg();
        let transaction_id = call.get_arg();
        let res = api.GetTransactionId(transaction_handle, transaction_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTransactionInformation(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let transaction_handle = call.get_arg();
        let outcome = call.get_arg();
        let isolation_level = call.get_arg();
        let isolation_flags = call.get_arg();
        let timeout = call.get_arg();
        let buffer_length = call.get_arg();
        let description = call.get_arg();
        let res = api.GetTransactionInformation(
            transaction_handle,
            outcome,
            isolation_level,
            isolation_flags,
            timeout,
            buffer_length,
            description,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetTransactionManagerId(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let transaction_manager_handle = call.get_arg();
        let transaction_manager_id = call.get_arg();
        let res = api.GetTransactionManagerId(transaction_manager_handle, transaction_manager_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVolumeInformationA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_root_path_name = call.get_arg();
        let lp_volume_name_buffer = call.get_arg();
        let n_volume_name_size = call.get_arg();
        let lp_volume_serial_number = call.get_arg();
        let lp_maximum_component_length = call.get_arg();
        let lp_file_system_flags = call.get_arg();
        let lp_file_system_name_buffer = call.get_arg();
        let n_file_system_name_size = call.get_arg();
        let res = api.GetVolumeInformationA(
            lp_root_path_name,
            lp_volume_name_buffer,
            n_volume_name_size,
            lp_volume_serial_number,
            lp_maximum_component_length,
            lp_file_system_flags,
            lp_file_system_name_buffer,
            n_file_system_name_size,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVolumeInformationByHandleW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_volume_name_buffer = call.get_arg();
        let n_volume_name_size = call.get_arg();
        let lp_volume_serial_number = call.get_arg();
        let lp_maximum_component_length = call.get_arg();
        let lp_file_system_flags = call.get_arg();
        let lp_file_system_name_buffer = call.get_arg();
        let n_file_system_name_size = call.get_arg();
        let res = api.GetVolumeInformationByHandleW(
            h_file,
            lp_volume_name_buffer,
            n_volume_name_size,
            lp_volume_serial_number,
            lp_maximum_component_length,
            lp_file_system_flags,
            lp_file_system_name_buffer,
            n_file_system_name_size,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVolumeInformationW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_root_path_name = call.get_arg();
        let lp_volume_name_buffer = call.get_arg();
        let n_volume_name_size = call.get_arg();
        let lp_volume_serial_number = call.get_arg();
        let lp_maximum_component_length = call.get_arg();
        let lp_file_system_flags = call.get_arg();
        let lp_file_system_name_buffer = call.get_arg();
        let n_file_system_name_size = call.get_arg();
        let res = api.GetVolumeInformationW(
            lp_root_path_name,
            lp_volume_name_buffer,
            n_volume_name_size,
            lp_volume_serial_number,
            lp_maximum_component_length,
            lp_file_system_flags,
            lp_file_system_name_buffer,
            n_file_system_name_size,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVolumeNameForVolumeMountPointA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_volume_mount_point = call.get_arg();
        let lpsz_volume_name = call.get_arg();
        let cch_buffer_length = call.get_arg();
        let res = api.GetVolumeNameForVolumeMountPointA(
            lpsz_volume_mount_point,
            lpsz_volume_name,
            cch_buffer_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVolumeNameForVolumeMountPointW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_volume_mount_point = call.get_arg();
        let lpsz_volume_name = call.get_arg();
        let cch_buffer_length = call.get_arg();
        let res = api.GetVolumeNameForVolumeMountPointW(
            lpsz_volume_mount_point,
            lpsz_volume_name,
            cch_buffer_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVolumePathNameA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_file_name = call.get_arg();
        let lpsz_volume_path_name = call.get_arg();
        let cch_buffer_length = call.get_arg();
        let res = api.GetVolumePathNameA(lpsz_file_name, lpsz_volume_path_name, cch_buffer_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVolumePathNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_file_name = call.get_arg();
        let lpsz_volume_path_name = call.get_arg();
        let cch_buffer_length = call.get_arg();
        let res = api.GetVolumePathNameW(lpsz_file_name, lpsz_volume_path_name, cch_buffer_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVolumePathNamesForVolumeNameA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_volume_name = call.get_arg();
        let lpsz_volume_path_names = call.get_arg();
        let cch_buffer_length = call.get_arg();
        let lpcch_return_length = call.get_arg();
        let res = api.GetVolumePathNamesForVolumeNameA(
            lpsz_volume_name,
            lpsz_volume_path_names,
            cch_buffer_length,
            lpcch_return_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVolumePathNamesForVolumeNameW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_volume_name = call.get_arg();
        let lpsz_volume_path_names = call.get_arg();
        let cch_buffer_length = call.get_arg();
        let lpcch_return_length = call.get_arg();
        let res = api.GetVolumePathNamesForVolumeNameW(
            lpsz_volume_name,
            lpsz_volume_path_names,
            cch_buffer_length,
            lpcch_return_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsIoRingOpSupported(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let io_ring = call.get_arg();
        let op = call.get_arg();
        let res = api.IsIoRingOpSupported(io_ring, op);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LZClose(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let res = api.LZClose(h_file);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LZCopy(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hf_source = call.get_arg();
        let hf_dest = call.get_arg();
        let res = api.LZCopy(hf_source, hf_dest);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LZDone(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.LZDone();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LZInit(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hf_source = call.get_arg();
        let res = api.LZInit(hf_source);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LZOpenFileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let lp_re_open_buf = call.get_arg();
        let w_style = call.get_arg();
        let res = api.LZOpenFileA(lp_file_name, lp_re_open_buf, w_style);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LZOpenFileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let lp_re_open_buf = call.get_arg();
        let w_style = call.get_arg();
        let res = api.LZOpenFileW(lp_file_name, lp_re_open_buf, w_style);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LZRead(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_buffer = call.get_arg();
        let cb_read = call.get_arg();
        let res = api.LZRead(h_file, lp_buffer, cb_read);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LZSeek(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let l_offset = call.get_arg();
        let i_origin = call.get_arg();
        let res = api.LZSeek(h_file, l_offset, i_origin);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LZStart(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.LZStart();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MoveFileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let res = api.MoveFileA(lp_existing_file_name, lp_new_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MoveFileExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.MoveFileExA(lp_existing_file_name, lp_new_file_name, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MoveFileExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.MoveFileExW(lp_existing_file_name, lp_new_file_name, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MoveFileFromAppW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let res = api.MoveFileFromAppW(lp_existing_file_name, lp_new_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MoveFileTransactedA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let lp_progress_routine = call.get_arg();
        let lp_data = call.get_arg();
        let dw_flags = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.MoveFileTransactedA(
            lp_existing_file_name,
            lp_new_file_name,
            lp_progress_routine,
            lp_data,
            dw_flags,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MoveFileTransactedW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let lp_progress_routine = call.get_arg();
        let lp_data = call.get_arg();
        let dw_flags = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.MoveFileTransactedW(
            lp_existing_file_name,
            lp_new_file_name,
            lp_progress_routine,
            lp_data,
            dw_flags,
            h_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MoveFileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let res = api.MoveFileW(lp_existing_file_name, lp_new_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MoveFileWithProgressA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let lp_progress_routine = call.get_arg();
        let lp_data = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.MoveFileWithProgressA(
            lp_existing_file_name,
            lp_new_file_name,
            lp_progress_routine,
            lp_data,
            dw_flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MoveFileWithProgressW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_existing_file_name = call.get_arg();
        let lp_new_file_name = call.get_arg();
        let lp_progress_routine = call.get_arg();
        let lp_data = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.MoveFileWithProgressW(
            lp_existing_file_name,
            lp_new_file_name,
            lp_progress_routine,
            lp_data,
            dw_flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetConnectionEnum(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let qualifier = call.get_arg();
        let level = call.get_arg();
        let bufptr = call.get_arg();
        let prefmaxlen = call.get_arg();
        let entriesread = call.get_arg();
        let totalentries = call.get_arg();
        let resume_handle = call.get_arg();
        let res = api.NetConnectionEnum(
            servername,
            qualifier,
            level,
            bufptr,
            prefmaxlen,
            entriesread,
            totalentries,
            resume_handle,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetFileClose(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let fileid = call.get_arg();
        let res = api.NetFileClose(servername, fileid);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetFileEnum(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let basepath = call.get_arg();
        let username = call.get_arg();
        let level = call.get_arg();
        let bufptr = call.get_arg();
        let prefmaxlen = call.get_arg();
        let entriesread = call.get_arg();
        let totalentries = call.get_arg();
        let resume_handle = call.get_arg();
        let res = api.NetFileEnum(
            servername,
            basepath,
            username,
            level,
            bufptr,
            prefmaxlen,
            entriesread,
            totalentries,
            resume_handle,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetFileGetInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let fileid = call.get_arg();
        let level = call.get_arg();
        let bufptr = call.get_arg();
        let res = api.NetFileGetInfo(servername, fileid, level, bufptr);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetServerAliasAdd(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let level = call.get_arg();
        let buf = call.get_arg();
        let res = api.NetServerAliasAdd(servername, level, buf);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetServerAliasDel(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let level = call.get_arg();
        let buf = call.get_arg();
        let res = api.NetServerAliasDel(servername, level, buf);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetServerAliasEnum(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let level = call.get_arg();
        let bufptr = call.get_arg();
        let prefmaxlen = call.get_arg();
        let entriesread = call.get_arg();
        let totalentries = call.get_arg();
        let resumehandle = call.get_arg();
        let res = api.NetServerAliasEnum(
            servername,
            level,
            bufptr,
            prefmaxlen,
            entriesread,
            totalentries,
            resumehandle,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetSessionDel(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let unc_client_name = call.get_arg();
        let username = call.get_arg();
        let res = api.NetSessionDel(servername, unc_client_name, username);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetSessionEnum(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let unc_client_name = call.get_arg();
        let username = call.get_arg();
        let level = call.get_arg();
        let bufptr = call.get_arg();
        let prefmaxlen = call.get_arg();
        let entriesread = call.get_arg();
        let totalentries = call.get_arg();
        let resume_handle = call.get_arg();
        let res = api.NetSessionEnum(
            servername,
            unc_client_name,
            username,
            level,
            bufptr,
            prefmaxlen,
            entriesread,
            totalentries,
            resume_handle,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetSessionGetInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let unc_client_name = call.get_arg();
        let username = call.get_arg();
        let level = call.get_arg();
        let bufptr = call.get_arg();
        let res = api.NetSessionGetInfo(servername, unc_client_name, username, level, bufptr);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetShareAdd(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let level = call.get_arg();
        let buf = call.get_arg();
        let parm_err = call.get_arg();
        let res = api.NetShareAdd(servername, level, buf, parm_err);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetShareCheck(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let device = call.get_arg();
        let r#type = call.get_arg();
        let res = api.NetShareCheck(servername, device, r#type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetShareDel(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let netname = call.get_arg();
        let reserved = call.get_arg();
        let res = api.NetShareDel(servername, netname, reserved);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetShareDelEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let level = call.get_arg();
        let buf = call.get_arg();
        let res = api.NetShareDelEx(servername, level, buf);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetShareDelSticky(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let netname = call.get_arg();
        let reserved = call.get_arg();
        let res = api.NetShareDelSticky(servername, netname, reserved);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetShareEnum(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let level = call.get_arg();
        let bufptr = call.get_arg();
        let prefmaxlen = call.get_arg();
        let entriesread = call.get_arg();
        let totalentries = call.get_arg();
        let resume_handle = call.get_arg();
        let res = api.NetShareEnum(
            servername,
            level,
            bufptr,
            prefmaxlen,
            entriesread,
            totalentries,
            resume_handle,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetShareEnumSticky(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let level = call.get_arg();
        let bufptr = call.get_arg();
        let prefmaxlen = call.get_arg();
        let entriesread = call.get_arg();
        let totalentries = call.get_arg();
        let resume_handle = call.get_arg();
        let res = api.NetShareEnumSticky(
            servername,
            level,
            bufptr,
            prefmaxlen,
            entriesread,
            totalentries,
            resume_handle,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetShareGetInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let netname = call.get_arg();
        let level = call.get_arg();
        let bufptr = call.get_arg();
        let res = api.NetShareGetInfo(servername, netname, level, bufptr);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetShareSetInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let servername = call.get_arg();
        let netname = call.get_arg();
        let level = call.get_arg();
        let buf = call.get_arg();
        let parm_err = call.get_arg();
        let res = api.NetShareSetInfo(servername, netname, level, buf, parm_err);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NetStatisticsGet(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let server_name = call.get_arg();
        let service = call.get_arg();
        let level = call.get_arg();
        let options = call.get_arg();
        let buffer = call.get_arg();
        let res = api.NetStatisticsGet(server_name, service, level, options, buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NtCreateFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let file_handle = call.get_arg();
        let desired_access = call.get_arg();
        let object_attributes = call.get_arg();
        let io_status_block = call.get_arg();
        let allocation_size = call.get_arg();
        let file_attributes = call.get_arg();
        let share_access = call.get_arg();
        let create_disposition = call.get_arg();
        let create_options = call.get_arg();
        let ea_buffer = call.get_arg();
        let ea_length = call.get_arg();
        let res = api.NtCreateFile(
            file_handle,
            desired_access,
            object_attributes,
            io_status_block,
            allocation_size,
            file_attributes,
            share_access,
            create_disposition,
            create_options,
            ea_buffer,
            ea_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenEncryptedFileRawA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let ul_flags = call.get_arg();
        let pv_context = call.get_arg();
        let res = api.OpenEncryptedFileRawA(lp_file_name, ul_flags, pv_context);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenEncryptedFileRawW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let ul_flags = call.get_arg();
        let pv_context = call.get_arg();
        let res = api.OpenEncryptedFileRawW(lp_file_name, ul_flags, pv_context);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenEnlistment(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_desired_access = call.get_arg();
        let resource_manager_handle = call.get_arg();
        let enlistment_id = call.get_arg();
        let res = api.OpenEnlistment(dw_desired_access, resource_manager_handle, enlistment_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let lp_re_open_buff = call.get_arg();
        let u_style = call.get_arg();
        let res = api.OpenFile(lp_file_name, lp_re_open_buff, u_style);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenResourceManager(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_desired_access = call.get_arg();
        let tm_handle = call.get_arg();
        let resource_manager_id = call.get_arg();
        let res = api.OpenResourceManager(dw_desired_access, tm_handle, resource_manager_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenTransaction(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_desired_access = call.get_arg();
        let transaction_id = call.get_arg();
        let res = api.OpenTransaction(dw_desired_access, transaction_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenTransactionManager(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let log_file_name = call.get_arg();
        let desired_access = call.get_arg();
        let open_options = call.get_arg();
        let res = api.OpenTransactionManager(log_file_name, desired_access, open_options);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenTransactionManagerById(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let transaction_manager_id = call.get_arg();
        let desired_access = call.get_arg();
        let open_options = call.get_arg();
        let res =
            api.OpenTransactionManagerById(transaction_manager_id, desired_access, open_options);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PopIoRingCompletion(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let io_ring = call.get_arg();
        let cqe = call.get_arg();
        let res = api.PopIoRingCompletion(io_ring, cqe);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PrePrepareComplete(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enlistment_handle = call.get_arg();
        let tm_virtual_clock = call.get_arg();
        let res = api.PrePrepareComplete(enlistment_handle, tm_virtual_clock);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PrePrepareEnlistment(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enlistment_handle = call.get_arg();
        let tm_virtual_clock = call.get_arg();
        let res = api.PrePrepareEnlistment(enlistment_handle, tm_virtual_clock);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PrepareComplete(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enlistment_handle = call.get_arg();
        let tm_virtual_clock = call.get_arg();
        let res = api.PrepareComplete(enlistment_handle, tm_virtual_clock);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PrepareEnlistment(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enlistment_handle = call.get_arg();
        let tm_virtual_clock = call.get_arg();
        let res = api.PrepareEnlistment(enlistment_handle, tm_virtual_clock);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PrepareTape(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_device = call.get_arg();
        let dw_operation = call.get_arg();
        let b_immediate = call.get_arg();
        let res = api.PrepareTape(h_device, dw_operation, b_immediate);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryDosDeviceA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_device_name = call.get_arg();
        let lp_target_path = call.get_arg();
        let ucch_max = call.get_arg();
        let res = api.QueryDosDeviceA(lp_device_name, lp_target_path, ucch_max);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryDosDeviceW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_device_name = call.get_arg();
        let lp_target_path = call.get_arg();
        let ucch_max = call.get_arg();
        let res = api.QueryDosDeviceW(lp_device_name, lp_target_path, ucch_max);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryIoRingCapabilities(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let capabilities = call.get_arg();
        let res = api.QueryIoRingCapabilities(capabilities);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReOpenFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_original_file = call.get_arg();
        let dw_desired_access = call.get_arg();
        let dw_share_mode = call.get_arg();
        let dw_flags_and_attributes = call.get_arg();
        let res = api.ReOpenFile(
            h_original_file,
            dw_desired_access,
            dw_share_mode,
            dw_flags_and_attributes,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReadEncryptedFileRaw(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pf_export_callback = call.get_arg();
        let pv_callback_context = call.get_arg();
        let pv_context = call.get_arg();
        let res = api.ReadEncryptedFileRaw(pf_export_callback, pv_callback_context, pv_context);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReadOnlyEnlistment(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enlistment_handle = call.get_arg();
        let tm_virtual_clock = call.get_arg();
        let res = api.ReadOnlyEnlistment(enlistment_handle, tm_virtual_clock);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RecoverEnlistment(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enlistment_handle = call.get_arg();
        let enlistment_key = call.get_arg();
        let res = api.RecoverEnlistment(enlistment_handle, enlistment_key);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RecoverResourceManager(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let resource_manager_handle = call.get_arg();
        let res = api.RecoverResourceManager(resource_manager_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RecoverTransactionManager(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let transaction_manager_handle = call.get_arg();
        let res = api.RecoverTransactionManager(transaction_manager_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RemoveDirectoryA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let res = api.RemoveDirectoryA(lp_path_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RemoveDirectoryFromAppW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let res = api.RemoveDirectoryFromAppW(lp_path_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RemoveDirectoryTransactedA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.RemoveDirectoryTransactedA(lp_path_name, h_transaction);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RemoveDirectoryTransactedW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.RemoveDirectoryTransactedW(lp_path_name, h_transaction);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RemoveDirectoryW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let res = api.RemoveDirectoryW(lp_path_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RenameTransactionManager(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let log_file_name = call.get_arg();
        let existing_transaction_manager_guid = call.get_arg();
        let res = api.RenameTransactionManager(log_file_name, existing_transaction_manager_guid);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReplaceFileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_replaced_file_name = call.get_arg();
        let lp_replacement_file_name = call.get_arg();
        let lp_backup_file_name = call.get_arg();
        let dw_replace_flags = call.get_arg();
        let lp_exclude = call.get_arg();
        let lp_reserved = call.get_arg();
        let res = api.ReplaceFileA(
            lp_replaced_file_name,
            lp_replacement_file_name,
            lp_backup_file_name,
            dw_replace_flags,
            lp_exclude,
            lp_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReplaceFileFromAppW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_replaced_file_name = call.get_arg();
        let lp_replacement_file_name = call.get_arg();
        let lp_backup_file_name = call.get_arg();
        let dw_replace_flags = call.get_arg();
        let lp_exclude = call.get_arg();
        let lp_reserved = call.get_arg();
        let res = api.ReplaceFileFromAppW(
            lp_replaced_file_name,
            lp_replacement_file_name,
            lp_backup_file_name,
            dw_replace_flags,
            lp_exclude,
            lp_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReplaceFileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_replaced_file_name = call.get_arg();
        let lp_replacement_file_name = call.get_arg();
        let lp_backup_file_name = call.get_arg();
        let dw_replace_flags = call.get_arg();
        let lp_exclude = call.get_arg();
        let lp_reserved = call.get_arg();
        let res = api.ReplaceFileW(
            lp_replaced_file_name,
            lp_replacement_file_name,
            lp_backup_file_name,
            dw_replace_flags,
            lp_exclude,
            lp_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RollbackComplete(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enlistment_handle = call.get_arg();
        let tm_virtual_clock = call.get_arg();
        let res = api.RollbackComplete(enlistment_handle, tm_virtual_clock);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RollbackEnlistment(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enlistment_handle = call.get_arg();
        let tm_virtual_clock = call.get_arg();
        let res = api.RollbackEnlistment(enlistment_handle, tm_virtual_clock);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RollbackTransaction(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let transaction_handle = call.get_arg();
        let res = api.RollbackTransaction(transaction_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RollbackTransactionAsync(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let transaction_handle = call.get_arg();
        let res = api.RollbackTransactionAsync(transaction_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RollforwardTransactionManager(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let transaction_manager_handle = call.get_arg();
        let tm_virtual_clock = call.get_arg();
        let res = api.RollforwardTransactionManager(transaction_manager_handle, tm_virtual_clock);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SearchPathA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path = call.get_arg();
        let lp_file_name = call.get_arg();
        let lp_extension = call.get_arg();
        let n_buffer_length = call.get_arg();
        let lp_buffer = call.get_arg();
        let lp_file_part = call.get_arg();
        let res = api.SearchPathA(
            lp_path,
            lp_file_name,
            lp_extension,
            n_buffer_length,
            lp_buffer,
            lp_file_part,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SearchPathW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path = call.get_arg();
        let lp_file_name = call.get_arg();
        let lp_extension = call.get_arg();
        let n_buffer_length = call.get_arg();
        let lp_buffer = call.get_arg();
        let lp_file_part = call.get_arg();
        let res = api.SearchPathW(
            lp_path,
            lp_file_name,
            lp_extension,
            n_buffer_length,
            lp_buffer,
            lp_file_part,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetEndOfFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let res = api.SetEndOfFile(h_file);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetEnlistmentRecoveryInformation(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enlistment_handle = call.get_arg();
        let buffer_size = call.get_arg();
        let buffer = call.get_arg();
        let res = api.SetEnlistmentRecoveryInformation(enlistment_handle, buffer_size, buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileApisToANSI(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.SetFileApisToANSI();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileApisToOEM(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.SetFileApisToOEM();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileAttributesA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let dw_file_attributes = call.get_arg();
        let res = api.SetFileAttributesA(lp_file_name, dw_file_attributes);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileAttributesFromAppW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let dw_file_attributes = call.get_arg();
        let res = api.SetFileAttributesFromAppW(lp_file_name, dw_file_attributes);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileAttributesTransactedA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let dw_file_attributes = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.SetFileAttributesTransactedA(lp_file_name, dw_file_attributes, h_transaction);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileAttributesTransactedW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let dw_file_attributes = call.get_arg();
        let h_transaction = call.get_arg();
        let res = api.SetFileAttributesTransactedW(lp_file_name, dw_file_attributes, h_transaction);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileAttributesW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_name = call.get_arg();
        let dw_file_attributes = call.get_arg();
        let res = api.SetFileAttributesW(lp_file_name, dw_file_attributes);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileBandwidthReservation(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let n_period_milliseconds = call.get_arg();
        let n_bytes_per_period = call.get_arg();
        let b_discardable = call.get_arg();
        let lp_transfer_size = call.get_arg();
        let lp_num_outstanding_requests = call.get_arg();
        let res = api.SetFileBandwidthReservation(
            h_file,
            n_period_milliseconds,
            n_bytes_per_period,
            b_discardable,
            lp_transfer_size,
            lp_num_outstanding_requests,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileCompletionNotificationModes(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let file_handle = call.get_arg();
        let flags = call.get_arg();
        let res = api.SetFileCompletionNotificationModes(file_handle, flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileInformationByHandle(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let file_information_class = call.get_arg();
        let lp_file_information = call.get_arg();
        let dw_buffer_size = call.get_arg();
        let res = api.SetFileInformationByHandle(
            h_file,
            file_information_class,
            lp_file_information,
            dw_buffer_size,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileIoOverlappedRange(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let file_handle = call.get_arg();
        let overlapped_range_start = call.get_arg();
        let length = call.get_arg();
        let res = api.SetFileIoOverlappedRange(file_handle, overlapped_range_start, length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFilePointer(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let l_distance_to_move = call.get_arg();
        let lp_distance_to_move_high = call.get_arg();
        let dw_move_method = call.get_arg();
        let res = api.SetFilePointer(
            h_file,
            l_distance_to_move,
            lp_distance_to_move_high,
            dw_move_method,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFilePointerEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let li_distance_to_move = call.get_arg();
        let lp_new_file_pointer = call.get_arg();
        let dw_move_method = call.get_arg();
        let res = api.SetFilePointerEx(
            h_file,
            li_distance_to_move,
            lp_new_file_pointer,
            dw_move_method,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileShortNameA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_short_name = call.get_arg();
        let res = api.SetFileShortNameA(h_file, lp_short_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileShortNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_short_name = call.get_arg();
        let res = api.SetFileShortNameW(h_file, lp_short_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_creation_time = call.get_arg();
        let lp_last_access_time = call.get_arg();
        let lp_last_write_time = call.get_arg();
        let res = api.SetFileTime(
            h_file,
            lp_creation_time,
            lp_last_access_time,
            lp_last_write_time,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFileValidData(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let valid_data_length = call.get_arg();
        let res = api.SetFileValidData(h_file, valid_data_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetIoRingCompletionEvent(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let io_ring = call.get_arg();
        let h_event = call.get_arg();
        let res = api.SetIoRingCompletionEvent(io_ring, h_event);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetResourceManagerCompletionPort(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let resource_manager_handle = call.get_arg();
        let io_completion_port_handle = call.get_arg();
        let completion_key = call.get_arg();
        let res = api.SetResourceManagerCompletionPort(
            resource_manager_handle,
            io_completion_port_handle,
            completion_key,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetSearchPathMode(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let flags = call.get_arg();
        let res = api.SetSearchPathMode(flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetTapeParameters(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_device = call.get_arg();
        let dw_operation = call.get_arg();
        let lp_tape_information = call.get_arg();
        let res = api.SetTapeParameters(h_device, dw_operation, lp_tape_information);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetTapePosition(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_device = call.get_arg();
        let dw_position_method = call.get_arg();
        let dw_partition = call.get_arg();
        let dw_offset_low = call.get_arg();
        let dw_offset_high = call.get_arg();
        let b_immediate = call.get_arg();
        let res = api.SetTapePosition(
            h_device,
            dw_position_method,
            dw_partition,
            dw_offset_low,
            dw_offset_high,
            b_immediate,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetTransactionInformation(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let transaction_handle = call.get_arg();
        let isolation_level = call.get_arg();
        let isolation_flags = call.get_arg();
        let timeout = call.get_arg();
        let description = call.get_arg();
        let res = api.SetTransactionInformation(
            transaction_handle,
            isolation_level,
            isolation_flags,
            timeout,
            description,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetVolumeLabelA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_root_path_name = call.get_arg();
        let lp_volume_name = call.get_arg();
        let res = api.SetVolumeLabelA(lp_root_path_name, lp_volume_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetVolumeLabelW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_root_path_name = call.get_arg();
        let lp_volume_name = call.get_arg();
        let res = api.SetVolumeLabelW(lp_root_path_name, lp_volume_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetVolumeMountPointA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_volume_mount_point = call.get_arg();
        let lpsz_volume_name = call.get_arg();
        let res = api.SetVolumeMountPointA(lpsz_volume_mount_point, lpsz_volume_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetVolumeMountPointW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_volume_mount_point = call.get_arg();
        let lpsz_volume_name = call.get_arg();
        let res = api.SetVolumeMountPointW(lpsz_volume_mount_point, lpsz_volume_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SinglePhaseReject(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enlistment_handle = call.get_arg();
        let tm_virtual_clock = call.get_arg();
        let res = api.SinglePhaseReject(enlistment_handle, tm_virtual_clock);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SubmitIoRing(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let io_ring = call.get_arg();
        let wait_operations = call.get_arg();
        let milliseconds = call.get_arg();
        let submitted_entries = call.get_arg();
        let res = api.SubmitIoRing(io_ring, wait_operations, milliseconds, submitted_entries);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TxfGetThreadMiniVersionForCreate(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let mini_version = call.get_arg();
        let res = api.TxfGetThreadMiniVersionForCreate(mini_version);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TxfLogCreateFileReadContext(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let log_path = call.get_arg();
        let beginning_lsn = call.get_arg();
        let ending_lsn = call.get_arg();
        let txf_file_id = call.get_arg();
        let txf_log_context = call.get_arg();
        let res = api.TxfLogCreateFileReadContext(
            log_path,
            beginning_lsn,
            ending_lsn,
            txf_file_id,
            txf_log_context,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TxfLogCreateRangeReadContext(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let log_path = call.get_arg();
        let beginning_lsn = call.get_arg();
        let ending_lsn = call.get_arg();
        let beginning_virtual_clock = call.get_arg();
        let ending_virtual_clock = call.get_arg();
        let record_type_mask = call.get_arg();
        let txf_log_context = call.get_arg();
        let res = api.TxfLogCreateRangeReadContext(
            log_path,
            beginning_lsn,
            ending_lsn,
            beginning_virtual_clock,
            ending_virtual_clock,
            record_type_mask,
            txf_log_context,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TxfLogDestroyReadContext(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let txf_log_context = call.get_arg();
        let res = api.TxfLogDestroyReadContext(txf_log_context);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TxfLogReadRecords(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let txf_log_context = call.get_arg();
        let buffer_length = call.get_arg();
        let buffer = call.get_arg();
        let bytes_used = call.get_arg();
        let record_count = call.get_arg();
        let res = api.TxfLogReadRecords(
            txf_log_context,
            buffer_length,
            buffer,
            bytes_used,
            record_count,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TxfLogRecordGetFileName(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let record_buffer = call.get_arg();
        let record_buffer_length_in_bytes = call.get_arg();
        let name_buffer = call.get_arg();
        let name_buffer_length_in_bytes = call.get_arg();
        let txf_id = call.get_arg();
        let res = api.TxfLogRecordGetFileName(
            record_buffer,
            record_buffer_length_in_bytes,
            name_buffer,
            name_buffer_length_in_bytes,
            txf_id,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TxfLogRecordGetGenericType(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let record_buffer = call.get_arg();
        let record_buffer_length_in_bytes = call.get_arg();
        let generic_type = call.get_arg();
        let virtual_clock = call.get_arg();
        let res = api.TxfLogRecordGetGenericType(
            record_buffer,
            record_buffer_length_in_bytes,
            generic_type,
            virtual_clock,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TxfReadMetadataInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let file_handle = call.get_arg();
        let txf_file_id = call.get_arg();
        let last_lsn = call.get_arg();
        let transaction_state = call.get_arg();
        let locking_transaction = call.get_arg();
        let res = api.TxfReadMetadataInfo(
            file_handle,
            txf_file_id,
            last_lsn,
            transaction_state,
            locking_transaction,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TxfSetThreadMiniVersionForCreate(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let mini_version = call.get_arg();
        let res = api.TxfSetThreadMiniVersionForCreate(mini_version);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UnlockFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let dw_file_offset_low = call.get_arg();
        let dw_file_offset_high = call.get_arg();
        let n_number_of_bytes_to_unlock_low = call.get_arg();
        let n_number_of_bytes_to_unlock_high = call.get_arg();
        let res = api.UnlockFile(
            h_file,
            dw_file_offset_low,
            dw_file_offset_high,
            n_number_of_bytes_to_unlock_low,
            n_number_of_bytes_to_unlock_high,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VerFindFileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let u_flags = call.get_arg();
        let sz_file_name = call.get_arg();
        let sz_win_dir = call.get_arg();
        let sz_app_dir = call.get_arg();
        let sz_cur_dir = call.get_arg();
        let pu_cur_dir_len = call.get_arg();
        let sz_dest_dir = call.get_arg();
        let pu_dest_dir_len = call.get_arg();
        let res = api.VerFindFileA(
            u_flags,
            sz_file_name,
            sz_win_dir,
            sz_app_dir,
            sz_cur_dir,
            pu_cur_dir_len,
            sz_dest_dir,
            pu_dest_dir_len,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VerFindFileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let u_flags = call.get_arg();
        let sz_file_name = call.get_arg();
        let sz_win_dir = call.get_arg();
        let sz_app_dir = call.get_arg();
        let sz_cur_dir = call.get_arg();
        let pu_cur_dir_len = call.get_arg();
        let sz_dest_dir = call.get_arg();
        let pu_dest_dir_len = call.get_arg();
        let res = api.VerFindFileW(
            u_flags,
            sz_file_name,
            sz_win_dir,
            sz_app_dir,
            sz_cur_dir,
            pu_cur_dir_len,
            sz_dest_dir,
            pu_dest_dir_len,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VerInstallFileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let u_flags = call.get_arg();
        let sz_src_file_name = call.get_arg();
        let sz_dest_file_name = call.get_arg();
        let sz_src_dir = call.get_arg();
        let sz_dest_dir = call.get_arg();
        let sz_cur_dir = call.get_arg();
        let sz_tmp_file = call.get_arg();
        let pu_tmp_file_len = call.get_arg();
        let res = api.VerInstallFileA(
            u_flags,
            sz_src_file_name,
            sz_dest_file_name,
            sz_src_dir,
            sz_dest_dir,
            sz_cur_dir,
            sz_tmp_file,
            pu_tmp_file_len,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VerInstallFileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let u_flags = call.get_arg();
        let sz_src_file_name = call.get_arg();
        let sz_dest_file_name = call.get_arg();
        let sz_src_dir = call.get_arg();
        let sz_dest_dir = call.get_arg();
        let sz_cur_dir = call.get_arg();
        let sz_tmp_file = call.get_arg();
        let pu_tmp_file_len = call.get_arg();
        let res = api.VerInstallFileW(
            u_flags,
            sz_src_file_name,
            sz_dest_file_name,
            sz_src_dir,
            sz_dest_dir,
            sz_cur_dir,
            sz_tmp_file,
            pu_tmp_file_len,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VerLanguageNameA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let w_lang = call.get_arg();
        let sz_lang = call.get_arg();
        let cch_lang = call.get_arg();
        let res = api.VerLanguageNameA(w_lang, sz_lang, cch_lang);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VerLanguageNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let w_lang = call.get_arg();
        let sz_lang = call.get_arg();
        let cch_lang = call.get_arg();
        let res = api.VerLanguageNameW(w_lang, sz_lang, cch_lang);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VerQueryValueA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let p_block = call.get_arg();
        let lp_sub_block = call.get_arg();
        let lplp_buffer = call.get_arg();
        let pu_len = call.get_arg();
        let res = api.VerQueryValueA(p_block, lp_sub_block, lplp_buffer, pu_len);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VerQueryValueW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let p_block = call.get_arg();
        let lp_sub_block = call.get_arg();
        let lplp_buffer = call.get_arg();
        let pu_len = call.get_arg();
        let res = api.VerQueryValueW(p_block, lp_sub_block, lplp_buffer, pu_len);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WofEnumEntries(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let volume_name = call.get_arg();
        let provider = call.get_arg();
        let enum_proc = call.get_arg();
        let user_data = call.get_arg();
        let res = api.WofEnumEntries(volume_name, provider, enum_proc, user_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WofFileEnumFiles(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let volume_name = call.get_arg();
        let algorithm = call.get_arg();
        let enum_proc = call.get_arg();
        let user_data = call.get_arg();
        let res = api.WofFileEnumFiles(volume_name, algorithm, enum_proc, user_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WofGetDriverVersion(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let file_or_volume_handle = call.get_arg();
        let provider = call.get_arg();
        let wof_version = call.get_arg();
        let res = api.WofGetDriverVersion(file_or_volume_handle, provider, wof_version);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WofIsExternalFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let file_path = call.get_arg();
        let is_external_file = call.get_arg();
        let provider = call.get_arg();
        let external_file_info = call.get_arg();
        let buffer_length = call.get_arg();
        let res = api.WofIsExternalFile(
            file_path,
            is_external_file,
            provider,
            external_file_info,
            buffer_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WofSetFileDataLocation(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let file_handle = call.get_arg();
        let provider = call.get_arg();
        let external_file_info = call.get_arg();
        let length = call.get_arg();
        let res = api.WofSetFileDataLocation(file_handle, provider, external_file_info, length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WofShouldCompressBinaries(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let volume = call.get_arg();
        let algorithm = call.get_arg();
        let res = api.WofShouldCompressBinaries(volume, algorithm);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WofWimAddEntry(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let volume_name = call.get_arg();
        let wim_path = call.get_arg();
        let wim_type = call.get_arg();
        let wim_index = call.get_arg();
        let data_source_id = call.get_arg();
        let res = api.WofWimAddEntry(volume_name, wim_path, wim_type, wim_index, data_source_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WofWimEnumFiles(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let volume_name = call.get_arg();
        let data_source_id = call.get_arg();
        let enum_proc = call.get_arg();
        let user_data = call.get_arg();
        let res = api.WofWimEnumFiles(volume_name, data_source_id, enum_proc, user_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WofWimRemoveEntry(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let volume_name = call.get_arg();
        let data_source_id = call.get_arg();
        let res = api.WofWimRemoveEntry(volume_name, data_source_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WofWimSuspendEntry(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let volume_name = call.get_arg();
        let data_source_id = call.get_arg();
        let res = api.WofWimSuspendEntry(volume_name, data_source_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WofWimUpdateEntry(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let volume_name = call.get_arg();
        let data_source_id = call.get_arg();
        let new_wim_path = call.get_arg();
        let res = api.WofWimUpdateEntry(volume_name, data_source_id, new_wim_path);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_Wow64DisableWow64FsRedirection(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let old_value = call.get_arg();
        let res = api.Wow64DisableWow64FsRedirection(old_value);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_Wow64EnableWow64FsRedirection(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let wow_64_fs_enable_redirection = call.get_arg();
        let res = api.Wow64EnableWow64FsRedirection(wow_64_fs_enable_redirection);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_Wow64RevertWow64FsRedirection(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ol_value = call.get_arg();
        let res = api.Wow64RevertWow64FsRedirection(ol_value);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteEncryptedFileRaw(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pf_import_callback = call.get_arg();
        let pv_callback_context = call.get_arg();
        let pv_context = call.get_arg();
        let res = api.WriteEncryptedFileRaw(pf_import_callback, pv_callback_context, pv_context);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteTapemark(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::Storage::FileSystem::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_device = call.get_arg();
        let dw_tapemark_type = call.get_arg();
        let dw_tapemark_count = call.get_arg();
        let b_immediate = call.get_arg();
        let res = api.WriteTapemark(h_device, dw_tapemark_type, dw_tapemark_count, b_immediate);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AddConsoleAliasA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let source = call.get_arg();
        let target = call.get_arg();
        let exe_name = call.get_arg();
        let res = api.AddConsoleAliasA(source, target, exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AddConsoleAliasW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let source = call.get_arg();
        let target = call.get_arg();
        let exe_name = call.get_arg();
        let res = api.AddConsoleAliasW(source, target, exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AllocConsole(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.AllocConsole();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AttachConsole(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_process_id = call.get_arg();
        let res = api.AttachConsole(dw_process_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ClosePseudoConsole(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_pc = call.get_arg();
        let res = api.ClosePseudoConsole(h_pc);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreatePseudoConsole(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let size = call.get_arg();
        let h_input = call.get_arg();
        let h_output = call.get_arg();
        let dw_flags = call.get_arg();
        let ph_pc = call.get_arg();
        let res = api.CreatePseudoConsole(size, h_input, h_output, dw_flags, ph_pc);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ExpungeConsoleCommandHistoryA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let exe_name = call.get_arg();
        let res = api.ExpungeConsoleCommandHistoryA(exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ExpungeConsoleCommandHistoryW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let exe_name = call.get_arg();
        let res = api.ExpungeConsoleCommandHistoryW(exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FillConsoleOutputAttribute(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let w_attribute = call.get_arg();
        let n_length = call.get_arg();
        let dw_write_coord = call.get_arg();
        let lp_number_of_attrs_written = call.get_arg();
        let res = api.FillConsoleOutputAttribute(
            h_console_output,
            w_attribute,
            n_length,
            dw_write_coord,
            lp_number_of_attrs_written,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FillConsoleOutputCharacterA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let c_character = call.get_arg();
        let n_length = call.get_arg();
        let dw_write_coord = call.get_arg();
        let lp_number_of_chars_written = call.get_arg();
        let res = api.FillConsoleOutputCharacterA(
            h_console_output,
            c_character,
            n_length,
            dw_write_coord,
            lp_number_of_chars_written,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FillConsoleOutputCharacterW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let c_character = call.get_arg();
        let n_length = call.get_arg();
        let dw_write_coord = call.get_arg();
        let lp_number_of_chars_written = call.get_arg();
        let res = api.FillConsoleOutputCharacterW(
            h_console_output,
            c_character,
            n_length,
            dw_write_coord,
            lp_number_of_chars_written,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FlushConsoleInputBuffer(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_input = call.get_arg();
        let res = api.FlushConsoleInputBuffer(h_console_input);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FreeConsole(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.FreeConsole();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GenerateConsoleCtrlEvent(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_ctrl_event = call.get_arg();
        let dw_process_group_id = call.get_arg();
        let res = api.GenerateConsoleCtrlEvent(dw_ctrl_event, dw_process_group_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleAliasA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let source = call.get_arg();
        let target_buffer = call.get_arg();
        let target_buffer_length = call.get_arg();
        let exe_name = call.get_arg();
        let res = api.GetConsoleAliasA(source, target_buffer, target_buffer_length, exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleAliasExesA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let exe_name_buffer = call.get_arg();
        let exe_name_buffer_length = call.get_arg();
        let res = api.GetConsoleAliasExesA(exe_name_buffer, exe_name_buffer_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleAliasExesLengthA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetConsoleAliasExesLengthA();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleAliasExesLengthW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetConsoleAliasExesLengthW();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleAliasExesW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let exe_name_buffer = call.get_arg();
        let exe_name_buffer_length = call.get_arg();
        let res = api.GetConsoleAliasExesW(exe_name_buffer, exe_name_buffer_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleAliasW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let source = call.get_arg();
        let target_buffer = call.get_arg();
        let target_buffer_length = call.get_arg();
        let exe_name = call.get_arg();
        let res = api.GetConsoleAliasW(source, target_buffer, target_buffer_length, exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleAliasesA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let alias_buffer = call.get_arg();
        let alias_buffer_length = call.get_arg();
        let exe_name = call.get_arg();
        let res = api.GetConsoleAliasesA(alias_buffer, alias_buffer_length, exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleAliasesLengthA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let exe_name = call.get_arg();
        let res = api.GetConsoleAliasesLengthA(exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleAliasesLengthW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let exe_name = call.get_arg();
        let res = api.GetConsoleAliasesLengthW(exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleAliasesW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let alias_buffer = call.get_arg();
        let alias_buffer_length = call.get_arg();
        let exe_name = call.get_arg();
        let res = api.GetConsoleAliasesW(alias_buffer, alias_buffer_length, exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleCP(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetConsoleCP();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleCommandHistoryA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let commands = call.get_arg();
        let command_buffer_length = call.get_arg();
        let exe_name = call.get_arg();
        let res = api.GetConsoleCommandHistoryA(commands, command_buffer_length, exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleCommandHistoryLengthA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let exe_name = call.get_arg();
        let res = api.GetConsoleCommandHistoryLengthA(exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleCommandHistoryLengthW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let exe_name = call.get_arg();
        let res = api.GetConsoleCommandHistoryLengthW(exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleCommandHistoryW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let commands = call.get_arg();
        let command_buffer_length = call.get_arg();
        let exe_name = call.get_arg();
        let res = api.GetConsoleCommandHistoryW(commands, command_buffer_length, exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleCursorInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_console_cursor_info = call.get_arg();
        let res = api.GetConsoleCursorInfo(h_console_output, lp_console_cursor_info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleDisplayMode(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_mode_flags = call.get_arg();
        let res = api.GetConsoleDisplayMode(lp_mode_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleHistoryInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_console_history_info = call.get_arg();
        let res = api.GetConsoleHistoryInfo(lp_console_history_info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleMode(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_handle = call.get_arg();
        let lp_mode = call.get_arg();
        let res = api.GetConsoleMode(h_console_handle, lp_mode);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleOriginalTitleA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_console_title = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetConsoleOriginalTitleA(lp_console_title, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleOriginalTitleW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_console_title = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetConsoleOriginalTitleW(lp_console_title, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleOutputCP(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetConsoleOutputCP();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleProcessList(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpdw_process_list = call.get_arg();
        let dw_process_count = call.get_arg();
        let res = api.GetConsoleProcessList(lpdw_process_list, dw_process_count);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleScreenBufferInfo(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_console_screen_buffer_info = call.get_arg();
        let res = api.GetConsoleScreenBufferInfo(h_console_output, lp_console_screen_buffer_info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleScreenBufferInfoEx(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_console_screen_buffer_info_ex = call.get_arg();
        let res =
            api.GetConsoleScreenBufferInfoEx(h_console_output, lp_console_screen_buffer_info_ex);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleSelectionInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_console_selection_info = call.get_arg();
        let res = api.GetConsoleSelectionInfo(lp_console_selection_info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleTitleA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_console_title = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetConsoleTitleA(lp_console_title, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleTitleW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_console_title = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetConsoleTitleW(lp_console_title, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetConsoleWindow(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetConsoleWindow();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCurrentConsoleFont(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let b_maximum_window = call.get_arg();
        let lp_console_current_font = call.get_arg();
        let res =
            api.GetCurrentConsoleFont(h_console_output, b_maximum_window, lp_console_current_font);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCurrentConsoleFontEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let b_maximum_window = call.get_arg();
        let lp_console_current_font_ex = call.get_arg();
        let res = api.GetCurrentConsoleFontEx(
            h_console_output,
            b_maximum_window,
            lp_console_current_font_ex,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetNumberOfConsoleInputEvents(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_input = call.get_arg();
        let lp_number_of_events = call.get_arg();
        let res = api.GetNumberOfConsoleInputEvents(h_console_input, lp_number_of_events);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetNumberOfConsoleMouseButtons(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_number_of_mouse_buttons = call.get_arg();
        let res = api.GetNumberOfConsoleMouseButtons(lp_number_of_mouse_buttons);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetStdHandle(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let n_std_handle = call.get_arg();
        let res = api.GetStdHandle(n_std_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PeekConsoleInputA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_input = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_length = call.get_arg();
        let lp_number_of_events_read = call.get_arg();
        let res = api.PeekConsoleInputA(
            h_console_input,
            lp_buffer,
            n_length,
            lp_number_of_events_read,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PeekConsoleInputW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_input = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_length = call.get_arg();
        let lp_number_of_events_read = call.get_arg();
        let res = api.PeekConsoleInputW(
            h_console_input,
            lp_buffer,
            n_length,
            lp_number_of_events_read,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReadConsoleA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_input = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_number_of_chars_to_read = call.get_arg();
        let lp_number_of_chars_read = call.get_arg();
        let p_input_control = call.get_arg();
        let res = api.ReadConsoleA(
            h_console_input,
            lp_buffer,
            n_number_of_chars_to_read,
            lp_number_of_chars_read,
            p_input_control,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReadConsoleInputA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_input = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_length = call.get_arg();
        let lp_number_of_events_read = call.get_arg();
        let res = api.ReadConsoleInputA(
            h_console_input,
            lp_buffer,
            n_length,
            lp_number_of_events_read,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReadConsoleInputW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_input = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_length = call.get_arg();
        let lp_number_of_events_read = call.get_arg();
        let res = api.ReadConsoleInputW(
            h_console_input,
            lp_buffer,
            n_length,
            lp_number_of_events_read,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReadConsoleOutputA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_buffer = call.get_arg();
        let dw_buffer_size = call.get_arg();
        let dw_buffer_coord = call.get_arg();
        let lp_read_region = call.get_arg();
        let res = api.ReadConsoleOutputA(
            h_console_output,
            lp_buffer,
            dw_buffer_size,
            dw_buffer_coord,
            lp_read_region,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReadConsoleOutputAttribute(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_attribute = call.get_arg();
        let n_length = call.get_arg();
        let dw_read_coord = call.get_arg();
        let lp_number_of_attrs_read = call.get_arg();
        let res = api.ReadConsoleOutputAttribute(
            h_console_output,
            lp_attribute,
            n_length,
            dw_read_coord,
            lp_number_of_attrs_read,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReadConsoleOutputCharacterA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_character = call.get_arg();
        let n_length = call.get_arg();
        let dw_read_coord = call.get_arg();
        let lp_number_of_chars_read = call.get_arg();
        let res = api.ReadConsoleOutputCharacterA(
            h_console_output,
            lp_character,
            n_length,
            dw_read_coord,
            lp_number_of_chars_read,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReadConsoleOutputCharacterW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_character = call.get_arg();
        let n_length = call.get_arg();
        let dw_read_coord = call.get_arg();
        let lp_number_of_chars_read = call.get_arg();
        let res = api.ReadConsoleOutputCharacterW(
            h_console_output,
            lp_character,
            n_length,
            dw_read_coord,
            lp_number_of_chars_read,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReadConsoleOutputW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_buffer = call.get_arg();
        let dw_buffer_size = call.get_arg();
        let dw_buffer_coord = call.get_arg();
        let lp_read_region = call.get_arg();
        let res = api.ReadConsoleOutputW(
            h_console_output,
            lp_buffer,
            dw_buffer_size,
            dw_buffer_coord,
            lp_read_region,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReadConsoleW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_input = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_number_of_chars_to_read = call.get_arg();
        let lp_number_of_chars_read = call.get_arg();
        let p_input_control = call.get_arg();
        let res = api.ReadConsoleW(
            h_console_input,
            lp_buffer,
            n_number_of_chars_to_read,
            lp_number_of_chars_read,
            p_input_control,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ResizePseudoConsole(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_pc = call.get_arg();
        let size = call.get_arg();
        let res = api.ResizePseudoConsole(h_pc, size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScrollConsoleScreenBufferA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_scroll_rectangle = call.get_arg();
        let lp_clip_rectangle = call.get_arg();
        let dw_destination_origin = call.get_arg();
        let lp_fill = call.get_arg();
        let res = api.ScrollConsoleScreenBufferA(
            h_console_output,
            lp_scroll_rectangle,
            lp_clip_rectangle,
            dw_destination_origin,
            lp_fill,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ScrollConsoleScreenBufferW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_scroll_rectangle = call.get_arg();
        let lp_clip_rectangle = call.get_arg();
        let dw_destination_origin = call.get_arg();
        let lp_fill = call.get_arg();
        let res = api.ScrollConsoleScreenBufferW(
            h_console_output,
            lp_scroll_rectangle,
            lp_clip_rectangle,
            dw_destination_origin,
            lp_fill,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleActiveScreenBuffer(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let res = api.SetConsoleActiveScreenBuffer(h_console_output);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleCP(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let w_code_page_id = call.get_arg();
        let res = api.SetConsoleCP(w_code_page_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleCtrlHandler(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let handler_routine = call.get_arg();
        let add = call.get_arg();
        let res = api.SetConsoleCtrlHandler(handler_routine, add);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleCursorInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_console_cursor_info = call.get_arg();
        let res = api.SetConsoleCursorInfo(h_console_output, lp_console_cursor_info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleCursorPosition(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let dw_cursor_position = call.get_arg();
        let res = api.SetConsoleCursorPosition(h_console_output, dw_cursor_position);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleDisplayMode(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_new_screen_buffer_dimensions = call.get_arg();
        let res =
            api.SetConsoleDisplayMode(h_console_output, dw_flags, lp_new_screen_buffer_dimensions);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleHistoryInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_console_history_info = call.get_arg();
        let res = api.SetConsoleHistoryInfo(lp_console_history_info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleMode(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_handle = call.get_arg();
        let dw_mode = call.get_arg();
        let res = api.SetConsoleMode(h_console_handle, dw_mode);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleNumberOfCommandsA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let number = call.get_arg();
        let exe_name = call.get_arg();
        let res = api.SetConsoleNumberOfCommandsA(number, exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleNumberOfCommandsW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let number = call.get_arg();
        let exe_name = call.get_arg();
        let res = api.SetConsoleNumberOfCommandsW(number, exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleOutputCP(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let w_code_page_id = call.get_arg();
        let res = api.SetConsoleOutputCP(w_code_page_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleScreenBufferInfoEx(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_console_screen_buffer_info_ex = call.get_arg();
        let res =
            api.SetConsoleScreenBufferInfoEx(h_console_output, lp_console_screen_buffer_info_ex);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleScreenBufferSize(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let dw_size = call.get_arg();
        let res = api.SetConsoleScreenBufferSize(h_console_output, dw_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleTextAttribute(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let w_attributes = call.get_arg();
        let res = api.SetConsoleTextAttribute(h_console_output, w_attributes);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleTitleA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_console_title = call.get_arg();
        let res = api.SetConsoleTitleA(lp_console_title);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleTitleW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_console_title = call.get_arg();
        let res = api.SetConsoleTitleW(lp_console_title);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetConsoleWindowInfo(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let b_absolute = call.get_arg();
        let lp_console_window = call.get_arg();
        let res = api.SetConsoleWindowInfo(h_console_output, b_absolute, lp_console_window);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetCurrentConsoleFontEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let b_maximum_window = call.get_arg();
        let lp_console_current_font_ex = call.get_arg();
        let res = api.SetCurrentConsoleFontEx(
            h_console_output,
            b_maximum_window,
            lp_console_current_font_ex,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetStdHandle(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let n_std_handle = call.get_arg();
        let h_handle = call.get_arg();
        let res = api.SetStdHandle(n_std_handle, h_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetStdHandleEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let n_std_handle = call.get_arg();
        let h_handle = call.get_arg();
        let ph_prev_value = call.get_arg();
        let res = api.SetStdHandleEx(n_std_handle, h_handle, ph_prev_value);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteConsoleA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_number_of_chars_to_write = call.get_arg();
        let lp_number_of_chars_written = call.get_arg();
        let lp_reserved = call.get_arg();
        let res = api.WriteConsoleA(
            h_console_output,
            lp_buffer,
            n_number_of_chars_to_write,
            lp_number_of_chars_written,
            lp_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteConsoleInputA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_input = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_length = call.get_arg();
        let lp_number_of_events_written = call.get_arg();
        let res = api.WriteConsoleInputA(
            h_console_input,
            lp_buffer,
            n_length,
            lp_number_of_events_written,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteConsoleInputW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_input = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_length = call.get_arg();
        let lp_number_of_events_written = call.get_arg();
        let res = api.WriteConsoleInputW(
            h_console_input,
            lp_buffer,
            n_length,
            lp_number_of_events_written,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteConsoleOutputA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_buffer = call.get_arg();
        let dw_buffer_size = call.get_arg();
        let dw_buffer_coord = call.get_arg();
        let lp_write_region = call.get_arg();
        let res = api.WriteConsoleOutputA(
            h_console_output,
            lp_buffer,
            dw_buffer_size,
            dw_buffer_coord,
            lp_write_region,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteConsoleOutputAttribute(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_attribute = call.get_arg();
        let n_length = call.get_arg();
        let dw_write_coord = call.get_arg();
        let lp_number_of_attrs_written = call.get_arg();
        let res = api.WriteConsoleOutputAttribute(
            h_console_output,
            lp_attribute,
            n_length,
            dw_write_coord,
            lp_number_of_attrs_written,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteConsoleOutputCharacterA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_character = call.get_arg();
        let n_length = call.get_arg();
        let dw_write_coord = call.get_arg();
        let lp_number_of_chars_written = call.get_arg();
        let res = api.WriteConsoleOutputCharacterA(
            h_console_output,
            lp_character,
            n_length,
            dw_write_coord,
            lp_number_of_chars_written,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteConsoleOutputCharacterW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_character = call.get_arg();
        let n_length = call.get_arg();
        let dw_write_coord = call.get_arg();
        let lp_number_of_chars_written = call.get_arg();
        let res = api.WriteConsoleOutputCharacterW(
            h_console_output,
            lp_character,
            n_length,
            dw_write_coord,
            lp_number_of_chars_written,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteConsoleOutputW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_buffer = call.get_arg();
        let dw_buffer_size = call.get_arg();
        let dw_buffer_coord = call.get_arg();
        let lp_write_region = call.get_arg();
        let res = api.WriteConsoleOutputW(
            h_console_output,
            lp_buffer,
            dw_buffer_size,
            dw_buffer_coord,
            lp_write_region,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteConsoleW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Console::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_console_output = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_number_of_chars_to_write = call.get_arg();
        let lp_number_of_chars_written = call.get_arg();
        let lp_reserved = call.get_arg();
        let res = api.WriteConsoleW(
            h_console_output,
            lp_buffer,
            n_number_of_chars_to_write,
            lp_number_of_chars_written,
            lp_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlUnwind(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Diagnostics::Debug::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let target_frame = call.get_arg();
        let target_ip = call.get_arg();
        let exception_record = call.get_arg();
        let return_value = call.get_arg();
        let res = api.RtlUnwind(target_frame, target_ip, exception_record, return_value);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CallEnclave(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_routine = call.get_arg();
        let lp_parameter = call.get_arg();
        let f_wait_for_thread = call.get_arg();
        let lp_return_value = call.get_arg();
        let res = api.CallEnclave(lp_routine, lp_parameter, f_wait_for_thread, lp_return_value);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateEnclave(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let lp_address = call.get_arg();
        let dw_size = call.get_arg();
        let dw_initial_commitment = call.get_arg();
        let fl_enclave_type = call.get_arg();
        let lp_enclave_information = call.get_arg();
        let dw_info_length = call.get_arg();
        let lp_enclave_error = call.get_arg();
        let res = api.CreateEnclave(
            h_process,
            lp_address,
            dw_size,
            dw_initial_commitment,
            fl_enclave_type,
            lp_enclave_information,
            dw_info_length,
            lp_enclave_error,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateEnvironmentBlock(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_environment = call.get_arg();
        let h_token = call.get_arg();
        let b_inherit = call.get_arg();
        let res = api.CreateEnvironmentBlock(lp_environment, h_token, b_inherit);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DeleteEnclave(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_address = call.get_arg();
        let res = api.DeleteEnclave(lp_address);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DestroyEnvironmentBlock(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_environment = call.get_arg();
        let res = api.DestroyEnvironmentBlock(lp_environment);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnclaveGetAttestationReport(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enclave_data = call.get_arg();
        let report = call.get_arg();
        let buffer_size = call.get_arg();
        let output_size = call.get_arg();
        let res = api.EnclaveGetAttestationReport(enclave_data, report, buffer_size, output_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnclaveGetEnclaveInformation(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let information_size = call.get_arg();
        let enclave_information = call.get_arg();
        let res = api.EnclaveGetEnclaveInformation(information_size, enclave_information);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnclaveSealData(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let data_to_encrypt = call.get_arg();
        let data_to_encrypt_size = call.get_arg();
        let identity_policy = call.get_arg();
        let runtime_policy = call.get_arg();
        let protected_blob = call.get_arg();
        let buffer_size = call.get_arg();
        let protected_blob_size = call.get_arg();
        let res = api.EnclaveSealData(
            data_to_encrypt,
            data_to_encrypt_size,
            identity_policy,
            runtime_policy,
            protected_blob,
            buffer_size,
            protected_blob_size,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnclaveUnsealData(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let protected_blob = call.get_arg();
        let protected_blob_size = call.get_arg();
        let decrypted_data = call.get_arg();
        let buffer_size = call.get_arg();
        let decrypted_data_size = call.get_arg();
        let sealing_identity = call.get_arg();
        let unsealing_flags = call.get_arg();
        let res = api.EnclaveUnsealData(
            protected_blob,
            protected_blob_size,
            decrypted_data,
            buffer_size,
            decrypted_data_size,
            sealing_identity,
            unsealing_flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnclaveVerifyAttestationReport(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let enclave_type = call.get_arg();
        let report = call.get_arg();
        let report_size = call.get_arg();
        let res = api.EnclaveVerifyAttestationReport(enclave_type, report, report_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ExpandEnvironmentStringsA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_src = call.get_arg();
        let lp_dst = call.get_arg();
        let n_size = call.get_arg();
        let res = api.ExpandEnvironmentStringsA(lp_src, lp_dst, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ExpandEnvironmentStringsForUserA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_token = call.get_arg();
        let lp_src = call.get_arg();
        let lp_dest = call.get_arg();
        let dw_size = call.get_arg();
        let res = api.ExpandEnvironmentStringsForUserA(h_token, lp_src, lp_dest, dw_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ExpandEnvironmentStringsForUserW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_token = call.get_arg();
        let lp_src = call.get_arg();
        let lp_dest = call.get_arg();
        let dw_size = call.get_arg();
        let res = api.ExpandEnvironmentStringsForUserW(h_token, lp_src, lp_dest, dw_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ExpandEnvironmentStringsW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_src = call.get_arg();
        let lp_dst = call.get_arg();
        let n_size = call.get_arg();
        let res = api.ExpandEnvironmentStringsW(lp_src, lp_dst, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FreeEnvironmentStringsA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let penv = call.get_arg();
        let res = api.FreeEnvironmentStringsA(penv);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FreeEnvironmentStringsW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let penv = call.get_arg();
        let res = api.FreeEnvironmentStringsW(penv);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCommandLineA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetCommandLineA();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCommandLineW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetCommandLineW();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCurrentDirectoryA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let n_buffer_length = call.get_arg();
        let lp_buffer = call.get_arg();
        let res = api.GetCurrentDirectoryA(n_buffer_length, lp_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCurrentDirectoryW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let n_buffer_length = call.get_arg();
        let lp_buffer = call.get_arg();
        let res = api.GetCurrentDirectoryW(n_buffer_length, lp_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetEnvironmentStrings(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetEnvironmentStrings();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetEnvironmentStringsW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetEnvironmentStringsW();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetEnvironmentVariableA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetEnvironmentVariableA(lp_name, lp_buffer, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetEnvironmentVariableW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetEnvironmentVariableW(lp_name, lp_buffer, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_InitializeEnclave(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let lp_address = call.get_arg();
        let lp_enclave_information = call.get_arg();
        let dw_info_length = call.get_arg();
        let lp_enclave_error = call.get_arg();
        let res = api.InitializeEnclave(
            h_process,
            lp_address,
            lp_enclave_information,
            dw_info_length,
            lp_enclave_error,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsEnclaveTypeSupported(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let fl_enclave_type = call.get_arg();
        let res = api.IsEnclaveTypeSupported(fl_enclave_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadEnclaveData(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let lp_address = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_size = call.get_arg();
        let fl_protect = call.get_arg();
        let lp_page_information = call.get_arg();
        let dw_info_length = call.get_arg();
        let lp_number_of_bytes_written = call.get_arg();
        let lp_enclave_error = call.get_arg();
        let res = api.LoadEnclaveData(
            h_process,
            lp_address,
            lp_buffer,
            n_size,
            fl_protect,
            lp_page_information,
            dw_info_length,
            lp_number_of_bytes_written,
            lp_enclave_error,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadEnclaveImageA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_enclave_address = call.get_arg();
        let lp_image_name = call.get_arg();
        let res = api.LoadEnclaveImageA(lp_enclave_address, lp_image_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadEnclaveImageW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_enclave_address = call.get_arg();
        let lp_image_name = call.get_arg();
        let res = api.LoadEnclaveImageW(lp_enclave_address, lp_image_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NeedCurrentDirectoryForExePathA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let exe_name = call.get_arg();
        let res = api.NeedCurrentDirectoryForExePathA(exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NeedCurrentDirectoryForExePathW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let exe_name = call.get_arg();
        let res = api.NeedCurrentDirectoryForExePathW(exe_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetCurrentDirectoryA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let res = api.SetCurrentDirectoryA(lp_path_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetCurrentDirectoryW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let res = api.SetCurrentDirectoryW(lp_path_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetEnvironmentStringsW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let new_environment = call.get_arg();
        let res = api.SetEnvironmentStringsW(new_environment);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetEnvironmentVariableA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let lp_value = call.get_arg();
        let res = api.SetEnvironmentVariableA(lp_name, lp_value);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetEnvironmentVariableW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let lp_value = call.get_arg();
        let res = api.SetEnvironmentVariableW(lp_name, lp_value);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TerminateEnclave(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Environment::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_address = call.get_arg();
        let f_wait = call.get_arg();
        let res = api.TerminateEnclave(lp_address, f_wait);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AddDllDirectory(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let new_directory = call.get_arg();
        let res = api.AddDllDirectory(new_directory);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_BeginUpdateResourceA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let p_file_name = call.get_arg();
        let b_delete_existing_resources = call.get_arg();
        let res = api.BeginUpdateResourceA(p_file_name, b_delete_existing_resources);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_BeginUpdateResourceW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let p_file_name = call.get_arg();
        let b_delete_existing_resources = call.get_arg();
        let res = api.BeginUpdateResourceW(p_file_name, b_delete_existing_resources);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DisableThreadLibraryCalls(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_lib_module = call.get_arg();
        let res = api.DisableThreadLibraryCalls(h_lib_module);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EndUpdateResourceA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_update = call.get_arg();
        let f_discard = call.get_arg();
        let res = api.EndUpdateResourceA(h_update, f_discard);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EndUpdateResourceW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_update = call.get_arg();
        let f_discard = call.get_arg();
        let res = api.EndUpdateResourceW(h_update, f_discard);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumResourceLanguagesA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_type = call.get_arg();
        let lp_name = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumResourceLanguagesA(h_module, lp_type, lp_name, lp_enum_func, l_param);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumResourceLanguagesExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_type = call.get_arg();
        let lp_name = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let dw_flags = call.get_arg();
        let lang_id = call.get_arg();
        let res = api.EnumResourceLanguagesExA(
            h_module,
            lp_type,
            lp_name,
            lp_enum_func,
            l_param,
            dw_flags,
            lang_id,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumResourceLanguagesExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_type = call.get_arg();
        let lp_name = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let dw_flags = call.get_arg();
        let lang_id = call.get_arg();
        let res = api.EnumResourceLanguagesExW(
            h_module,
            lp_type,
            lp_name,
            lp_enum_func,
            l_param,
            dw_flags,
            lang_id,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumResourceLanguagesW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_type = call.get_arg();
        let lp_name = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumResourceLanguagesW(h_module, lp_type, lp_name, lp_enum_func, l_param);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumResourceNamesA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_type = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumResourceNamesA(h_module, lp_type, lp_enum_func, l_param);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumResourceNamesExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_type = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let dw_flags = call.get_arg();
        let lang_id = call.get_arg();
        let res =
            api.EnumResourceNamesExA(h_module, lp_type, lp_enum_func, l_param, dw_flags, lang_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumResourceNamesExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_type = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let dw_flags = call.get_arg();
        let lang_id = call.get_arg();
        let res =
            api.EnumResourceNamesExW(h_module, lp_type, lp_enum_func, l_param, dw_flags, lang_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumResourceNamesW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_type = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumResourceNamesW(h_module, lp_type, lp_enum_func, l_param);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumResourceTypesA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumResourceTypesA(h_module, lp_enum_func, l_param);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumResourceTypesExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let dw_flags = call.get_arg();
        let lang_id = call.get_arg();
        let res = api.EnumResourceTypesExA(h_module, lp_enum_func, l_param, dw_flags, lang_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumResourceTypesExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let dw_flags = call.get_arg();
        let lang_id = call.get_arg();
        let res = api.EnumResourceTypesExW(h_module, lp_enum_func, l_param, dw_flags, lang_id);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnumResourceTypesW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumResourceTypesW(h_module, lp_enum_func, l_param);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindResourceA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_name = call.get_arg();
        let lp_type = call.get_arg();
        let res = api.FindResourceA(h_module, lp_name, lp_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindResourceExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_type = call.get_arg();
        let lp_name = call.get_arg();
        let w_language = call.get_arg();
        let res = api.FindResourceExA(h_module, lp_type, lp_name, w_language);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindResourceExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_type = call.get_arg();
        let lp_name = call.get_arg();
        let w_language = call.get_arg();
        let res = api.FindResourceExW(h_module, lp_type, lp_name, w_language);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FindResourceW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_name = call.get_arg();
        let lp_type = call.get_arg();
        let res = api.FindResourceW(h_module, lp_name, lp_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FreeLibrary(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_lib_module = call.get_arg();
        let res = api.FreeLibrary(h_lib_module);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FreeLibraryAndExitThread(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_lib_module = call.get_arg();
        let dw_exit_code = call.get_arg();
        let res = api.FreeLibraryAndExitThread(h_lib_module, dw_exit_code);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FreeResource(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_res_data = call.get_arg();
        let res = api.FreeResource(h_res_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDllDirectoryA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let n_buffer_length = call.get_arg();
        let lp_buffer = call.get_arg();
        let res = api.GetDllDirectoryA(n_buffer_length, lp_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetDllDirectoryW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let n_buffer_length = call.get_arg();
        let lp_buffer = call.get_arg();
        let res = api.GetDllDirectoryW(n_buffer_length, lp_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetModuleFileNameA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_filename = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetModuleFileNameA(h_module, lp_filename, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetModuleFileNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_filename = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetModuleFileNameW(h_module, lp_filename, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetModuleHandleA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_module_name = call.get_arg();
        let res = api.GetModuleHandleA(lp_module_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetModuleHandleExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let lp_module_name = call.get_arg();
        let ph_module = call.get_arg();
        let res = api.GetModuleHandleExA(dw_flags, lp_module_name, ph_module);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetModuleHandleExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let lp_module_name = call.get_arg();
        let ph_module = call.get_arg();
        let res = api.GetModuleHandleExW(dw_flags, lp_module_name, ph_module);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetModuleHandleW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_module_name = call.get_arg();
        let res = api.GetModuleHandleW(lp_module_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetProcAddress(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let lp_proc_name = call.get_arg();
        let res = api.GetProcAddress(h_module, lp_proc_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadLibraryA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_lib_file_name = call.get_arg();
        let res = api.LoadLibraryA(lp_lib_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadLibraryExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_lib_file_name = call.get_arg();
        let h_file = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.LoadLibraryExA(lp_lib_file_name, h_file, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadLibraryExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_lib_file_name = call.get_arg();
        let h_file = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.LoadLibraryExW(lp_lib_file_name, h_file, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadLibraryW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_lib_file_name = call.get_arg();
        let res = api.LoadLibraryW(lp_lib_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadModule(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_module_name = call.get_arg();
        let lp_parameter_block = call.get_arg();
        let res = api.LoadModule(lp_module_name, lp_parameter_block);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadPackagedLibrary(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpw_lib_file_name = call.get_arg();
        let reserved = call.get_arg();
        let res = api.LoadPackagedLibrary(lpw_lib_file_name, reserved);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LoadResource(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let h_res_info = call.get_arg();
        let res = api.LoadResource(h_module, h_res_info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LockResource(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_res_data = call.get_arg();
        let res = api.LockResource(h_res_data);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RemoveDllDirectory(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let cookie = call.get_arg();
        let res = api.RemoveDllDirectory(cookie);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetDefaultDllDirectories(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let directory_flags = call.get_arg();
        let res = api.SetDefaultDllDirectories(directory_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetDllDirectoryA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let res = api.SetDllDirectoryA(lp_path_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetDllDirectoryW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let res = api.SetDllDirectoryW(lp_path_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SizeofResource(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_module = call.get_arg();
        let h_res_info = call.get_arg();
        let res = api.SizeofResource(h_module, h_res_info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UpdateResourceA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_update = call.get_arg();
        let lp_type = call.get_arg();
        let lp_name = call.get_arg();
        let w_language = call.get_arg();
        let lp_data = call.get_arg();
        let cb = call.get_arg();
        let res = api.UpdateResourceA(h_update, lp_type, lp_name, w_language, lp_data, cb);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UpdateResourceW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::LibraryLoader::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_update = call.get_arg();
        let lp_type = call.get_arg();
        let lp_name = call.get_arg();
        let w_language = call.get_arg();
        let lp_data = call.get_arg();
        let cb = call.get_arg();
        let res = api.UpdateResourceW(h_update, lp_type, lp_name, w_language, lp_data, cb);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AddSecureMemoryCacheCallback(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pfn_call_back = call.get_arg();
        let res = api.AddSecureMemoryCacheCallback(pfn_call_back);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AllocateUserPhysicalPages(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let number_of_pages = call.get_arg();
        let page_array = call.get_arg();
        let res = api.AllocateUserPhysicalPages(h_process, number_of_pages, page_array);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AllocateUserPhysicalPages2(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let object_handle = call.get_arg();
        let number_of_pages = call.get_arg();
        let page_array = call.get_arg();
        let extended_parameters = call.get_arg();
        let extended_parameter_count = call.get_arg();
        let res = api.AllocateUserPhysicalPages2(
            object_handle,
            number_of_pages,
            page_array,
            extended_parameters,
            extended_parameter_count,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AllocateUserPhysicalPagesNuma(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let number_of_pages = call.get_arg();
        let page_array = call.get_arg();
        let nnd_preferred = call.get_arg();
        let res = api.AllocateUserPhysicalPagesNuma(
            h_process,
            number_of_pages,
            page_array,
            nnd_preferred,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CreateMemoryResourceNotification(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let notification_type = call.get_arg();
        let res = api.CreateMemoryResourceNotification(notification_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DiscardVirtualMemory(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let virtual_address = call.get_arg();
        let size = call.get_arg();
        let res = api.DiscardVirtualMemory(virtual_address, size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FlushViewOfFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_base_address = call.get_arg();
        let dw_number_of_bytes_to_flush = call.get_arg();
        let res = api.FlushViewOfFile(lp_base_address, dw_number_of_bytes_to_flush);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FreeUserPhysicalPages(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let number_of_pages = call.get_arg();
        let page_array = call.get_arg();
        let res = api.FreeUserPhysicalPages(h_process, number_of_pages, page_array);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetLargePageMinimum(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetLargePageMinimum();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetMemoryErrorHandlingCapabilities(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let capabilities = call.get_arg();
        let res = api.GetMemoryErrorHandlingCapabilities(capabilities);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetProcessHeap(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetProcessHeap();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetProcessHeaps(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let number_of_heaps = call.get_arg();
        let process_heaps = call.get_arg();
        let res = api.GetProcessHeaps(number_of_heaps, process_heaps);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetProcessWorkingSetSizeEx(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let lp_minimum_working_set_size = call.get_arg();
        let lp_maximum_working_set_size = call.get_arg();
        let flags = call.get_arg();
        let res = api.GetProcessWorkingSetSizeEx(
            h_process,
            lp_minimum_working_set_size,
            lp_maximum_working_set_size,
            flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemFileCacheSize(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_minimum_file_cache_size = call.get_arg();
        let lp_maximum_file_cache_size = call.get_arg();
        let lp_flags = call.get_arg();
        let res = api.GetSystemFileCacheSize(
            lp_minimum_file_cache_size,
            lp_maximum_file_cache_size,
            lp_flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetWriteWatch(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_flags = call.get_arg();
        let lp_base_address = call.get_arg();
        let dw_region_size = call.get_arg();
        let lp_addresses = call.get_arg();
        let lpdw_count = call.get_arg();
        let lpdw_granularity = call.get_arg();
        let res = api.GetWriteWatch(
            dw_flags,
            lp_base_address,
            dw_region_size,
            lp_addresses,
            lpdw_count,
            lpdw_granularity,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalAlloc(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let u_flags = call.get_arg();
        let dw_bytes = call.get_arg();
        let res = api.GlobalAlloc(u_flags, dw_bytes);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalFlags(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let res = api.GlobalFlags(h_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalFree(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let res = api.GlobalFree(h_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalHandle(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let p_mem = call.get_arg();
        let res = api.GlobalHandle(p_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalLock(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let res = api.GlobalLock(h_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalReAlloc(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let dw_bytes = call.get_arg();
        let u_flags = call.get_arg();
        let res = api.GlobalReAlloc(h_mem, dw_bytes, u_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalSize(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let res = api.GlobalSize(h_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalUnlock(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let res = api.GlobalUnlock(h_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HeapAlloc(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_heap = call.get_arg();
        let dw_flags = call.get_arg();
        let dw_bytes = call.get_arg();
        let res = api.HeapAlloc(h_heap, dw_flags, dw_bytes);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HeapCompact(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_heap = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.HeapCompact(h_heap, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HeapCreate(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let fl_options = call.get_arg();
        let dw_initial_size = call.get_arg();
        let dw_maximum_size = call.get_arg();
        let res = api.HeapCreate(fl_options, dw_initial_size, dw_maximum_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HeapDestroy(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_heap = call.get_arg();
        let res = api.HeapDestroy(h_heap);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HeapFree(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_heap = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_mem = call.get_arg();
        let res = api.HeapFree(h_heap, dw_flags, lp_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HeapLock(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_heap = call.get_arg();
        let res = api.HeapLock(h_heap);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HeapQueryInformation(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let heap_handle = call.get_arg();
        let heap_information_class = call.get_arg();
        let heap_information = call.get_arg();
        let heap_information_length = call.get_arg();
        let return_length = call.get_arg();
        let res = api.HeapQueryInformation(
            heap_handle,
            heap_information_class,
            heap_information,
            heap_information_length,
            return_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HeapReAlloc(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_heap = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_mem = call.get_arg();
        let dw_bytes = call.get_arg();
        let res = api.HeapReAlloc(h_heap, dw_flags, lp_mem, dw_bytes);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HeapSetInformation(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let heap_handle = call.get_arg();
        let heap_information_class = call.get_arg();
        let heap_information = call.get_arg();
        let heap_information_length = call.get_arg();
        let res = api.HeapSetInformation(
            heap_handle,
            heap_information_class,
            heap_information,
            heap_information_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HeapSize(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_heap = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_mem = call.get_arg();
        let res = api.HeapSize(h_heap, dw_flags, lp_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HeapSummary(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_heap = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_summary = call.get_arg();
        let res = api.HeapSummary(h_heap, dw_flags, lp_summary);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HeapUnlock(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_heap = call.get_arg();
        let res = api.HeapUnlock(h_heap);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HeapValidate(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_heap = call.get_arg();
        let dw_flags = call.get_arg();
        let lp_mem = call.get_arg();
        let res = api.HeapValidate(h_heap, dw_flags, lp_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_HeapWalk(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_heap = call.get_arg();
        let lp_entry = call.get_arg();
        let res = api.HeapWalk(h_heap, lp_entry);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsBadCodePtr(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpfn = call.get_arg();
        let res = api.IsBadCodePtr(lpfn);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsBadReadPtr(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp = call.get_arg();
        let ucb = call.get_arg();
        let res = api.IsBadReadPtr(lp, ucb);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsBadStringPtrA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz = call.get_arg();
        let ucch_max = call.get_arg();
        let res = api.IsBadStringPtrA(lpsz, ucch_max);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsBadStringPtrW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz = call.get_arg();
        let ucch_max = call.get_arg();
        let res = api.IsBadStringPtrW(lpsz, ucch_max);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsBadWritePtr(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp = call.get_arg();
        let ucb = call.get_arg();
        let res = api.IsBadWritePtr(lp, ucb);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LocalAlloc(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let u_flags = call.get_arg();
        let u_bytes = call.get_arg();
        let res = api.LocalAlloc(u_flags, u_bytes);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LocalFlags(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let res = api.LocalFlags(h_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LocalFree(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let res = api.LocalFree(h_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LocalHandle(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let p_mem = call.get_arg();
        let res = api.LocalHandle(p_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LocalLock(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let res = api.LocalLock(h_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LocalReAlloc(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let u_bytes = call.get_arg();
        let u_flags = call.get_arg();
        let res = api.LocalReAlloc(h_mem, u_bytes, u_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LocalSize(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let res = api.LocalSize(h_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LocalUnlock(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let res = api.LocalUnlock(h_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MapUserPhysicalPages(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let virtual_address = call.get_arg();
        let number_of_pages = call.get_arg();
        let page_array = call.get_arg();
        let res = api.MapUserPhysicalPages(virtual_address, number_of_pages, page_array);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MapUserPhysicalPagesScatter(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let virtual_addresses = call.get_arg();
        let number_of_pages = call.get_arg();
        let page_array = call.get_arg();
        let res = api.MapUserPhysicalPagesScatter(virtual_addresses, number_of_pages, page_array);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MapViewOfFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file_mapping_object = call.get_arg();
        let dw_desired_access = call.get_arg();
        let dw_file_offset_high = call.get_arg();
        let dw_file_offset_low = call.get_arg();
        let dw_number_of_bytes_to_map = call.get_arg();
        let res = api.MapViewOfFile(
            h_file_mapping_object,
            dw_desired_access,
            dw_file_offset_high,
            dw_file_offset_low,
            dw_number_of_bytes_to_map,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MapViewOfFile3(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let file_mapping = call.get_arg();
        let process = call.get_arg();
        let base_address = call.get_arg();
        let offset = call.get_arg();
        let view_size = call.get_arg();
        let allocation_type = call.get_arg();
        let page_protection = call.get_arg();
        let extended_parameters = call.get_arg();
        let parameter_count = call.get_arg();
        let res = api.MapViewOfFile3(
            file_mapping,
            process,
            base_address,
            offset,
            view_size,
            allocation_type,
            page_protection,
            extended_parameters,
            parameter_count,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MapViewOfFile3FromApp(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let file_mapping = call.get_arg();
        let process = call.get_arg();
        let base_address = call.get_arg();
        let offset = call.get_arg();
        let view_size = call.get_arg();
        let allocation_type = call.get_arg();
        let page_protection = call.get_arg();
        let extended_parameters = call.get_arg();
        let parameter_count = call.get_arg();
        let res = api.MapViewOfFile3FromApp(
            file_mapping,
            process,
            base_address,
            offset,
            view_size,
            allocation_type,
            page_protection,
            extended_parameters,
            parameter_count,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MapViewOfFileEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file_mapping_object = call.get_arg();
        let dw_desired_access = call.get_arg();
        let dw_file_offset_high = call.get_arg();
        let dw_file_offset_low = call.get_arg();
        let dw_number_of_bytes_to_map = call.get_arg();
        let lp_base_address = call.get_arg();
        let res = api.MapViewOfFileEx(
            h_file_mapping_object,
            dw_desired_access,
            dw_file_offset_high,
            dw_file_offset_low,
            dw_number_of_bytes_to_map,
            lp_base_address,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MapViewOfFileExNuma(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file_mapping_object = call.get_arg();
        let dw_desired_access = call.get_arg();
        let dw_file_offset_high = call.get_arg();
        let dw_file_offset_low = call.get_arg();
        let dw_number_of_bytes_to_map = call.get_arg();
        let lp_base_address = call.get_arg();
        let nnd_preferred = call.get_arg();
        let res = api.MapViewOfFileExNuma(
            h_file_mapping_object,
            dw_desired_access,
            dw_file_offset_high,
            dw_file_offset_low,
            dw_number_of_bytes_to_map,
            lp_base_address,
            nnd_preferred,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MapViewOfFileFromApp(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file_mapping_object = call.get_arg();
        let desired_access = call.get_arg();
        let file_offset = call.get_arg();
        let number_of_bytes_to_map = call.get_arg();
        let res = api.MapViewOfFileFromApp(
            h_file_mapping_object,
            desired_access,
            file_offset,
            number_of_bytes_to_map,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MapViewOfFileNuma2(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let file_mapping_handle = call.get_arg();
        let process_handle = call.get_arg();
        let offset = call.get_arg();
        let base_address = call.get_arg();
        let view_size = call.get_arg();
        let allocation_type = call.get_arg();
        let page_protection = call.get_arg();
        let preferred_node = call.get_arg();
        let res = api.MapViewOfFileNuma2(
            file_mapping_handle,
            process_handle,
            offset,
            base_address,
            view_size,
            allocation_type,
            page_protection,
            preferred_node,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OfferVirtualMemory(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let virtual_address = call.get_arg();
        let size = call.get_arg();
        let priority = call.get_arg();
        let res = api.OfferVirtualMemory(virtual_address, size, priority);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenDedicatedMemoryPartition(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let partition = call.get_arg();
        let dedicated_memory_type_id = call.get_arg();
        let desired_access = call.get_arg();
        let inherit_handle = call.get_arg();
        let res = api.OpenDedicatedMemoryPartition(
            partition,
            dedicated_memory_type_id,
            desired_access,
            inherit_handle,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenFileMappingA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_desired_access = call.get_arg();
        let b_inherit_handle = call.get_arg();
        let lp_name = call.get_arg();
        let res = api.OpenFileMappingA(dw_desired_access, b_inherit_handle, lp_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenFileMappingFromApp(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let desired_access = call.get_arg();
        let inherit_handle = call.get_arg();
        let name = call.get_arg();
        let res = api.OpenFileMappingFromApp(desired_access, inherit_handle, name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenFileMappingW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_desired_access = call.get_arg();
        let b_inherit_handle = call.get_arg();
        let lp_name = call.get_arg();
        let res = api.OpenFileMappingW(dw_desired_access, b_inherit_handle, lp_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_PrefetchVirtualMemory(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let number_of_entries = call.get_arg();
        let virtual_addresses = call.get_arg();
        let flags = call.get_arg();
        let res = api.PrefetchVirtualMemory(h_process, number_of_entries, virtual_addresses, flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryMemoryResourceNotification(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let resource_notification_handle = call.get_arg();
        let resource_state = call.get_arg();
        let res = api.QueryMemoryResourceNotification(resource_notification_handle, resource_state);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryPartitionInformation(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let partition = call.get_arg();
        let partition_information_class = call.get_arg();
        let partition_information = call.get_arg();
        let partition_information_length = call.get_arg();
        let res = api.QueryPartitionInformation(
            partition,
            partition_information_class,
            partition_information,
            partition_information_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryVirtualMemoryInformation(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let process = call.get_arg();
        let virtual_address = call.get_arg();
        let memory_information_class = call.get_arg();
        let memory_information = call.get_arg();
        let memory_information_size = call.get_arg();
        let return_size = call.get_arg();
        let res = api.QueryVirtualMemoryInformation(
            process,
            virtual_address,
            memory_information_class,
            memory_information,
            memory_information_size,
            return_size,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReclaimVirtualMemory(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let virtual_address = call.get_arg();
        let size = call.get_arg();
        let res = api.ReclaimVirtualMemory(virtual_address, size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RegisterBadMemoryNotification(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let callback = call.get_arg();
        let res = api.RegisterBadMemoryNotification(callback);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RemoveSecureMemoryCacheCallback(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pfn_call_back = call.get_arg();
        let res = api.RemoveSecureMemoryCacheCallback(pfn_call_back);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ResetWriteWatch(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_base_address = call.get_arg();
        let dw_region_size = call.get_arg();
        let res = api.ResetWriteWatch(lp_base_address, dw_region_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlCompareMemory(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let source_1 = call.get_arg();
        let source_2 = call.get_arg();
        let length = call.get_arg();
        let res = api.RtlCompareMemory(source_1, source_2, length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlCrc32(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let buffer = call.get_arg();
        let size = call.get_arg();
        let initial_crc = call.get_arg();
        let res = api.RtlCrc32(buffer, size, initial_crc);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlCrc64(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let buffer = call.get_arg();
        let size = call.get_arg();
        let initial_crc = call.get_arg();
        let res = api.RtlCrc64(buffer, size, initial_crc);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlIsZeroMemory(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let buffer = call.get_arg();
        let length = call.get_arg();
        let res = api.RtlIsZeroMemory(buffer, length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetProcessValidCallTargets(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let virtual_address = call.get_arg();
        let region_size = call.get_arg();
        let number_of_offsets = call.get_arg();
        let offset_information = call.get_arg();
        let res = api.SetProcessValidCallTargets(
            h_process,
            virtual_address,
            region_size,
            number_of_offsets,
            offset_information,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetProcessValidCallTargetsForMappedView(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let process = call.get_arg();
        let virtual_address = call.get_arg();
        let region_size = call.get_arg();
        let number_of_offsets = call.get_arg();
        let offset_information = call.get_arg();
        let section = call.get_arg();
        let expected_file_offset = call.get_arg();
        let res = api.SetProcessValidCallTargetsForMappedView(
            process,
            virtual_address,
            region_size,
            number_of_offsets,
            offset_information,
            section,
            expected_file_offset,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetProcessWorkingSetSizeEx(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let dw_minimum_working_set_size = call.get_arg();
        let dw_maximum_working_set_size = call.get_arg();
        let flags = call.get_arg();
        let res = api.SetProcessWorkingSetSizeEx(
            h_process,
            dw_minimum_working_set_size,
            dw_maximum_working_set_size,
            flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetSystemFileCacheSize(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let minimum_file_cache_size = call.get_arg();
        let maximum_file_cache_size = call.get_arg();
        let flags = call.get_arg();
        let res =
            api.SetSystemFileCacheSize(minimum_file_cache_size, maximum_file_cache_size, flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UnmapViewOfFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_base_address = call.get_arg();
        let res = api.UnmapViewOfFile(lp_base_address);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UnmapViewOfFile2(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let process = call.get_arg();
        let base_address = call.get_arg();
        let unmap_flags = call.get_arg();
        let res = api.UnmapViewOfFile2(process, base_address, unmap_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UnmapViewOfFileEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let base_address = call.get_arg();
        let unmap_flags = call.get_arg();
        let res = api.UnmapViewOfFileEx(base_address, unmap_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UnregisterBadMemoryNotification(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let registration_handle = call.get_arg();
        let res = api.UnregisterBadMemoryNotification(registration_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualAlloc(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_address = call.get_arg();
        let dw_size = call.get_arg();
        let fl_allocation_type = call.get_arg();
        let fl_protect = call.get_arg();
        let res = api.VirtualAlloc(lp_address, dw_size, fl_allocation_type, fl_protect);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualAlloc2(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let process = call.get_arg();
        let base_address = call.get_arg();
        let size = call.get_arg();
        let allocation_type = call.get_arg();
        let page_protection = call.get_arg();
        let extended_parameters = call.get_arg();
        let parameter_count = call.get_arg();
        let res = api.VirtualAlloc2(
            process,
            base_address,
            size,
            allocation_type,
            page_protection,
            extended_parameters,
            parameter_count,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualAlloc2FromApp(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let process = call.get_arg();
        let base_address = call.get_arg();
        let size = call.get_arg();
        let allocation_type = call.get_arg();
        let page_protection = call.get_arg();
        let extended_parameters = call.get_arg();
        let parameter_count = call.get_arg();
        let res = api.VirtualAlloc2FromApp(
            process,
            base_address,
            size,
            allocation_type,
            page_protection,
            extended_parameters,
            parameter_count,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualAllocEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let lp_address = call.get_arg();
        let dw_size = call.get_arg();
        let fl_allocation_type = call.get_arg();
        let fl_protect = call.get_arg();
        let res = api.VirtualAllocEx(
            h_process,
            lp_address,
            dw_size,
            fl_allocation_type,
            fl_protect,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualAllocExNuma(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let lp_address = call.get_arg();
        let dw_size = call.get_arg();
        let fl_allocation_type = call.get_arg();
        let fl_protect = call.get_arg();
        let nnd_preferred = call.get_arg();
        let res = api.VirtualAllocExNuma(
            h_process,
            lp_address,
            dw_size,
            fl_allocation_type,
            fl_protect,
            nnd_preferred,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualAllocFromApp(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let base_address = call.get_arg();
        let size = call.get_arg();
        let allocation_type = call.get_arg();
        let protection = call.get_arg();
        let res = api.VirtualAllocFromApp(base_address, size, allocation_type, protection);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualFree(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_address = call.get_arg();
        let dw_size = call.get_arg();
        let dw_free_type = call.get_arg();
        let res = api.VirtualFree(lp_address, dw_size, dw_free_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualFreeEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let lp_address = call.get_arg();
        let dw_size = call.get_arg();
        let dw_free_type = call.get_arg();
        let res = api.VirtualFreeEx(h_process, lp_address, dw_size, dw_free_type);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualLock(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_address = call.get_arg();
        let dw_size = call.get_arg();
        let res = api.VirtualLock(lp_address, dw_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualProtect(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_address = call.get_arg();
        let dw_size = call.get_arg();
        let fl_new_protect = call.get_arg();
        let lpfl_old_protect = call.get_arg();
        let res = api.VirtualProtect(lp_address, dw_size, fl_new_protect, lpfl_old_protect);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualProtectEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let lp_address = call.get_arg();
        let dw_size = call.get_arg();
        let fl_new_protect = call.get_arg();
        let lpfl_old_protect = call.get_arg();
        let res = api.VirtualProtectEx(
            h_process,
            lp_address,
            dw_size,
            fl_new_protect,
            lpfl_old_protect,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualProtectFromApp(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let address = call.get_arg();
        let size = call.get_arg();
        let new_protection = call.get_arg();
        let old_protection = call.get_arg();
        let res = api.VirtualProtectFromApp(address, size, new_protection, old_protection);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualQuery(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_address = call.get_arg();
        let lp_buffer = call.get_arg();
        let dw_length = call.get_arg();
        let res = api.VirtualQuery(lp_address, lp_buffer, dw_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualQueryEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_process = call.get_arg();
        let lp_address = call.get_arg();
        let lp_buffer = call.get_arg();
        let dw_length = call.get_arg();
        let res = api.VirtualQueryEx(h_process, lp_address, lp_buffer, dw_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualUnlock(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_address = call.get_arg();
        let dw_size = call.get_arg();
        let res = api.VirtualUnlock(lp_address, dw_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_VirtualUnlockEx(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::Memory::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let process = call.get_arg();
        let address = call.get_arg();
        let size = call.get_arg();
        let res = api.VirtualUnlockEx(process, address, size);
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
        let name_type = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetComputerNameExA(name_type, lp_buffer, n_size);
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
        let name_type = call.get_arg();
        let lp_buffer = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetComputerNameExW(name_type, lp_buffer, n_size);
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
        let lp_system_time = call.get_arg();
        let res = api.GetLocalTime(lp_system_time);
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
        let buffer = call.get_arg();
        let returned_length = call.get_arg();
        let res = api.GetLogicalProcessorInformation(buffer, returned_length);
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
        let relationship_type = call.get_arg();
        let buffer = call.get_arg();
        let returned_length = call.get_arg();
        let res = api.GetLogicalProcessorInformationEx(relationship_type, buffer, returned_length);
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
        let lp_system_info = call.get_arg();
        let res = api.GetNativeSystemInfo(lp_system_info);
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
        let pb_enabled = call.get_arg();
        let res = api.GetOsManufacturingMode(pb_enabled);
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
        let flags = call.get_arg();
        let res = api.GetOsSafeBootMode(flags);
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
        let total_memory_in_kilobytes = call.get_arg();
        let res = api.GetPhysicallyInstalledSystemMemory(total_memory_in_kilobytes);
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
        let group = call.get_arg();
        let buffer = call.get_arg();
        let returned_length = call.get_arg();
        let res = api.GetProcessorSystemCycleTime(group, buffer, returned_length);
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
        let dw_os_major_version = call.get_arg();
        let dw_os_minor_version = call.get_arg();
        let dw_sp_major_version = call.get_arg();
        let dw_sp_minor_version = call.get_arg();
        let pdw_returned_product_type = call.get_arg();
        let res = api.GetProductInfo(
            dw_os_major_version,
            dw_os_minor_version,
            dw_sp_major_version,
            dw_sp_minor_version,
            pdw_returned_product_type,
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
        let information = call.get_arg();
        let buffer_length = call.get_arg();
        let returned_length = call.get_arg();
        let process = call.get_arg();
        let flags = call.get_arg();
        let res = api.GetSystemCpuSetInformation(
            information,
            buffer_length,
            returned_length,
            process,
            flags,
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
        let lp_buffer = call.get_arg();
        let u_size = call.get_arg();
        let res = api.GetSystemDirectoryA(lp_buffer, u_size);
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
        let lp_buffer = call.get_arg();
        let u_size = call.get_arg();
        let res = api.GetSystemDirectoryW(lp_buffer, u_size);
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
        let firmware_table_provider_signature = call.get_arg();
        let firmware_table_id = call.get_arg();
        let p_firmware_table_buffer = call.get_arg();
        let buffer_size = call.get_arg();
        let res = api.GetSystemFirmwareTable(
            firmware_table_provider_signature,
            firmware_table_id,
            p_firmware_table_buffer,
            buffer_size,
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
        let lp_system_info = call.get_arg();
        let res = api.GetSystemInfo(lp_system_info);
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
        let enabled = call.get_arg();
        let flags = call.get_arg();
        let res = api.GetSystemLeapSecondInformation(enabled, flags);
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
        let lp_system_time = call.get_arg();
        let res = api.GetSystemTime(lp_system_time);
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
        let lp_time_adjustment = call.get_arg();
        let lp_time_increment = call.get_arg();
        let lp_time_adjustment_disabled = call.get_arg();
        let res = api.GetSystemTimeAdjustment(
            lp_time_adjustment,
            lp_time_increment,
            lp_time_adjustment_disabled,
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
        let lp_time_adjustment = call.get_arg();
        let lp_time_increment = call.get_arg();
        let lp_time_adjustment_disabled = call.get_arg();
        let res = api.GetSystemTimeAdjustmentPrecise(
            lp_time_adjustment,
            lp_time_increment,
            lp_time_adjustment_disabled,
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
        let lp_system_time_as_file_time = call.get_arg();
        let res = api.GetSystemTimeAsFileTime(lp_system_time_as_file_time);
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
        let lp_system_time_as_file_time = call.get_arg();
        let res = api.GetSystemTimePreciseAsFileTime(lp_system_time_as_file_time);
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
        let lp_buffer = call.get_arg();
        let u_size = call.get_arg();
        let res = api.GetSystemWindowsDirectoryA(lp_buffer, u_size);
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
        let lp_buffer = call.get_arg();
        let u_size = call.get_arg();
        let res = api.GetSystemWindowsDirectoryW(lp_buffer, u_size);
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
        let lp_buffer = call.get_arg();
        let u_size = call.get_arg();
        let image_file_machine_type = call.get_arg();
        let res = api.GetSystemWow64Directory2A(lp_buffer, u_size, image_file_machine_type);
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
        let lp_buffer = call.get_arg();
        let u_size = call.get_arg();
        let image_file_machine_type = call.get_arg();
        let res = api.GetSystemWow64Directory2W(lp_buffer, u_size, image_file_machine_type);
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
        let lp_buffer = call.get_arg();
        let u_size = call.get_arg();
        let res = api.GetSystemWow64DirectoryA(lp_buffer, u_size);
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
        let lp_buffer = call.get_arg();
        let u_size = call.get_arg();
        let res = api.GetSystemWow64DirectoryW(lp_buffer, u_size);
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
        let lp_version_information = call.get_arg();
        let res = api.GetVersionExA(lp_version_information);
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
        let lp_version_information = call.get_arg();
        let res = api.GetVersionExW(lp_version_information);
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
        let lp_buffer = call.get_arg();
        let u_size = call.get_arg();
        let res = api.GetWindowsDirectoryA(lp_buffer, u_size);
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
        let lp_buffer = call.get_arg();
        let u_size = call.get_arg();
        let res = api.GetWindowsDirectoryW(lp_buffer, u_size);
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
        let lp_buffer = call.get_arg();
        let res = api.GlobalMemoryStatus(lp_buffer);
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
        let lp_buffer = call.get_arg();
        let res = api.GlobalMemoryStatusEx(lp_buffer);
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
        let pul_device_family_buffer_size = call.get_arg();
        let pul_device_form_buffer_size = call.get_arg();
        let device_family = call.get_arg();
        let device_form = call.get_arg();
        let res = api.RtlConvertDeviceFamilyInfoToString(
            pul_device_family_buffer_size,
            pul_device_form_buffer_size,
            device_family,
            device_form,
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
        let pull_uap_info = call.get_arg();
        let pul_device_family = call.get_arg();
        let pul_device_form = call.get_arg();
        let res = api.RtlGetDeviceFamilyInfoEnum(pull_uap_info, pul_device_family, pul_device_form);
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
        let os_major_version = call.get_arg();
        let os_minor_version = call.get_arg();
        let sp_major_version = call.get_arg();
        let sp_minor_version = call.get_arg();
        let returned_product_type = call.get_arg();
        let res = api.RtlGetProductInfo(
            os_major_version,
            os_minor_version,
            sp_major_version,
            sp_minor_version,
            returned_product_type,
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
        let data_id = call.get_arg();
        let buffer = call.get_arg();
        let size = call.get_arg();
        let res = api.RtlGetSystemGlobalData(data_id, buffer, size);
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
        let flags = call.get_arg();
        let res = api.RtlOsDeploymentState(flags);
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
        let version_info = call.get_arg();
        let type_mask = call.get_arg();
        let condition_mask = call.get_arg();
        let res = api.RtlSwitchedVVI(version_info, type_mask, condition_mask);
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
        let lp_computer_name = call.get_arg();
        let res = api.SetComputerNameA(lp_computer_name);
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
        let name_type = call.get_arg();
        let flags = call.get_arg();
        let lp_buffer = call.get_arg();
        let res = api.SetComputerNameEx2W(name_type, flags, lp_buffer);
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
        let name_type = call.get_arg();
        let lp_buffer = call.get_arg();
        let res = api.SetComputerNameExA(name_type, lp_buffer);
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
        let name_type = call.get_arg();
        let lp_buffer = call.get_arg();
        let res = api.SetComputerNameExW(name_type, lp_buffer);
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
        let lp_computer_name = call.get_arg();
        let res = api.SetComputerNameW(lp_computer_name);
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
        let lp_system_time = call.get_arg();
        let res = api.SetLocalTime(lp_system_time);
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
        let lp_system_time = call.get_arg();
        let res = api.SetSystemTime(lp_system_time);
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
        let dw_time_adjustment = call.get_arg();
        let b_time_adjustment_disabled = call.get_arg();
        let res = api.SetSystemTimeAdjustment(dw_time_adjustment, b_time_adjustment_disabled);
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
        let dw_time_adjustment = call.get_arg();
        let b_time_adjustment_disabled = call.get_arg();
        let res =
            api.SetSystemTimeAdjustmentPrecise(dw_time_adjustment, b_time_adjustment_disabled);
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
        let condition_mask = call.get_arg();
        let type_mask = call.get_arg();
        let condition = call.get_arg();
        let res = api.VerSetConditionMask(condition_mask, type_mask, condition);
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
        let lp_version_information = call.get_arg();
        let dw_type_mask = call.get_arg();
        let dwl_condition_mask = call.get_arg();
        let res = api.VerifyVersionInfoA(lp_version_information, dw_type_mask, dwl_condition_mask);
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
        let lp_version_information = call.get_arg();
        let dw_type_mask = call.get_arg();
        let dwl_condition_mask = call.get_arg();
        let res = api.VerifyVersionInfoW(lp_version_information, dw_type_mask, dwl_condition_mask);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AddDelBackupEntryA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpcsz_file_list = call.get_arg();
        let lpcsz_backup_dir = call.get_arg();
        let lpcsz_base_name = call.get_arg();
        let dw_flags = call.get_arg();
        let res =
            api.AddDelBackupEntryA(lpcsz_file_list, lpcsz_backup_dir, lpcsz_base_name, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AddDelBackupEntryW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpcsz_file_list = call.get_arg();
        let lpcsz_backup_dir = call.get_arg();
        let lpcsz_base_name = call.get_arg();
        let dw_flags = call.get_arg();
        let res =
            api.AddDelBackupEntryW(lpcsz_file_list, lpcsz_backup_dir, lpcsz_base_name, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AdvInstallFileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let lpsz_source_dir = call.get_arg();
        let lpsz_source_file = call.get_arg();
        let lpsz_dest_dir = call.get_arg();
        let lpsz_dest_file = call.get_arg();
        let dw_flags = call.get_arg();
        let dw_reserved = call.get_arg();
        let res = api.AdvInstallFileA(
            hwnd,
            lpsz_source_dir,
            lpsz_source_file,
            lpsz_dest_dir,
            lpsz_dest_file,
            dw_flags,
            dw_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_AdvInstallFileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let lpsz_source_dir = call.get_arg();
        let lpsz_source_file = call.get_arg();
        let lpsz_dest_dir = call.get_arg();
        let lpsz_dest_file = call.get_arg();
        let dw_flags = call.get_arg();
        let dw_reserved = call.get_arg();
        let res = api.AdvInstallFileW(
            hwnd,
            lpsz_source_dir,
            lpsz_source_file,
            lpsz_dest_dir,
            lpsz_dest_file,
            dw_flags,
            dw_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ApphelpCheckShellObject(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let object_clsid = call.get_arg();
        let b_shim_if_necessary = call.get_arg();
        let pull_flags = call.get_arg();
        let res = api.ApphelpCheckShellObject(object_clsid, b_shim_if_necessary, pull_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CancelDeviceWakeupRequest(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_device = call.get_arg();
        let res = api.CancelDeviceWakeupRequest(h_device);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CancelTimerQueueTimer(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let timer_queue = call.get_arg();
        let timer = call.get_arg();
        let res = api.CancelTimerQueueTimer(timer_queue, timer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_CloseINFEngine(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_inf = call.get_arg();
        let res = api.CloseINFEngine(h_inf);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ConvertAuxiliaryCounterToPerformanceCounter(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ull_auxiliary_counter_value = call.get_arg();
        let lp_performance_counter_value = call.get_arg();
        let lp_conversion_error = call.get_arg();
        let res = api.ConvertAuxiliaryCounterToPerformanceCounter(
            ull_auxiliary_counter_value,
            lp_performance_counter_value,
            lp_conversion_error,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ConvertPerformanceCounterToAuxiliaryCounter(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let ull_performance_counter_value = call.get_arg();
        let lp_auxiliary_counter_value = call.get_arg();
        let lp_conversion_error = call.get_arg();
        let res = api.ConvertPerformanceCounterToAuxiliaryCounter(
            ull_performance_counter_value,
            lp_auxiliary_counter_value,
            lp_conversion_error,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DelNodeA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let psz_file_or_dir_name = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.DelNodeA(psz_file_or_dir_name, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DelNodeRunDLL32W(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let h_instance = call.get_arg();
        let psz_parms = call.get_arg();
        let n_show = call.get_arg();
        let res = api.DelNodeRunDLL32W(hwnd, h_instance, psz_parms, n_show);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DelNodeW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let psz_file_or_dir_name = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.DelNodeW(psz_file_or_dir_name, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DnsHostnameToComputerNameA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hostname = call.get_arg();
        let computer_name = call.get_arg();
        let n_size = call.get_arg();
        let res = api.DnsHostnameToComputerNameA(hostname, computer_name, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DnsHostnameToComputerNameW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hostname = call.get_arg();
        let computer_name = call.get_arg();
        let n_size = call.get_arg();
        let res = api.DnsHostnameToComputerNameW(hostname, computer_name, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_DosDateTimeToFileTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let w_fat_date = call.get_arg();
        let w_fat_time = call.get_arg();
        let lp_file_time = call.get_arg();
        let res = api.DosDateTimeToFileTime(w_fat_date, w_fat_time, lp_file_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_EnableProcessOptionalXStateFeatures(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let features = call.get_arg();
        let res = api.EnableProcessOptionalXStateFeatures(features);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ExecuteCabA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let p_cab = call.get_arg();
        let p_reserved = call.get_arg();
        let res = api.ExecuteCabA(hwnd, p_cab, p_reserved);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ExecuteCabW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let p_cab = call.get_arg();
        let p_reserved = call.get_arg();
        let res = api.ExecuteCabW(hwnd, p_cab, p_reserved);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ExtractFilesA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let psz_cab_name = call.get_arg();
        let psz_expand_dir = call.get_arg();
        let dw_flags = call.get_arg();
        let psz_file_list = call.get_arg();
        let lp_reserved = call.get_arg();
        let dw_reserved = call.get_arg();
        let res = api.ExtractFilesA(
            psz_cab_name,
            psz_expand_dir,
            dw_flags,
            psz_file_list,
            lp_reserved,
            dw_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ExtractFilesW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let psz_cab_name = call.get_arg();
        let psz_expand_dir = call.get_arg();
        let dw_flags = call.get_arg();
        let psz_file_list = call.get_arg();
        let lp_reserved = call.get_arg();
        let dw_reserved = call.get_arg();
        let res = api.ExtractFilesW(
            psz_cab_name,
            psz_expand_dir,
            dw_flags,
            psz_file_list,
            lp_reserved,
            dw_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FileSaveMarkNotExistA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_list = call.get_arg();
        let lp_dir = call.get_arg();
        let lp_base_name = call.get_arg();
        let res = api.FileSaveMarkNotExistA(lp_file_list, lp_dir, lp_base_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FileSaveMarkNotExistW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_list = call.get_arg();
        let lp_dir = call.get_arg();
        let lp_base_name = call.get_arg();
        let res = api.FileSaveMarkNotExistW(lp_file_list, lp_dir, lp_base_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FileSaveRestoreOnINFA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_wnd = call.get_arg();
        let psz_title = call.get_arg();
        let psz_inf = call.get_arg();
        let psz_section = call.get_arg();
        let psz_backup_dir = call.get_arg();
        let psz_base_backup_file = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.FileSaveRestoreOnINFA(
            h_wnd,
            psz_title,
            psz_inf,
            psz_section,
            psz_backup_dir,
            psz_base_backup_file,
            dw_flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FileSaveRestoreOnINFW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_wnd = call.get_arg();
        let psz_title = call.get_arg();
        let psz_inf = call.get_arg();
        let psz_section = call.get_arg();
        let psz_backup_dir = call.get_arg();
        let psz_base_backup_file = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.FileSaveRestoreOnINFW(
            h_wnd,
            psz_title,
            psz_inf,
            psz_section,
            psz_backup_dir,
            psz_base_backup_file,
            dw_flags,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FileSaveRestoreW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_dlg = call.get_arg();
        let lp_file_list = call.get_arg();
        let lp_dir = call.get_arg();
        let lp_base_name = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.FileSaveRestoreW(h_dlg, lp_file_list, lp_dir, lp_base_name, dw_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_FileTimeToDosDateTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_file_time = call.get_arg();
        let lp_fat_date = call.get_arg();
        let lp_fat_time = call.get_arg();
        let res = api.FileTimeToDosDateTime(lp_file_time, lp_fat_date, lp_fat_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GdiEntry13(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GdiEntry13();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetComputerNameA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_buffer = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetComputerNameA(lp_buffer, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetComputerNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_buffer = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetComputerNameW(lp_buffer, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCurrentHwProfileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_hw_profile_info = call.get_arg();
        let res = api.GetCurrentHwProfileA(lp_hw_profile_info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetCurrentHwProfileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_hw_profile_info = call.get_arg();
        let res = api.GetCurrentHwProfileW(lp_hw_profile_info);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFeatureEnabledState(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let feature_id = call.get_arg();
        let change_time = call.get_arg();
        let res = api.GetFeatureEnabledState(feature_id, change_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFeatureVariant(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let feature_id = call.get_arg();
        let change_time = call.get_arg();
        let payload_id = call.get_arg();
        let has_notification = call.get_arg();
        let res = api.GetFeatureVariant(feature_id, change_time, payload_id, has_notification);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFirmwareEnvironmentVariableA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let lp_guid = call.get_arg();
        let p_buffer = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetFirmwareEnvironmentVariableA(lp_name, lp_guid, p_buffer, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFirmwareEnvironmentVariableExA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let lp_guid = call.get_arg();
        let p_buffer = call.get_arg();
        let n_size = call.get_arg();
        let pdw_attribubutes = call.get_arg();
        let res = api.GetFirmwareEnvironmentVariableExA(
            lp_name,
            lp_guid,
            p_buffer,
            n_size,
            pdw_attribubutes,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFirmwareEnvironmentVariableExW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let lp_guid = call.get_arg();
        let p_buffer = call.get_arg();
        let n_size = call.get_arg();
        let pdw_attribubutes = call.get_arg();
        let res = api.GetFirmwareEnvironmentVariableExW(
            lp_name,
            lp_guid,
            p_buffer,
            n_size,
            pdw_attribubutes,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetFirmwareEnvironmentVariableW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let lp_guid = call.get_arg();
        let p_buffer = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetFirmwareEnvironmentVariableW(lp_name, lp_guid, p_buffer, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetPrivateProfileIntA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_key_name = call.get_arg();
        let n_default = call.get_arg();
        let lp_file_name = call.get_arg();
        let res = api.GetPrivateProfileIntA(lp_app_name, lp_key_name, n_default, lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetPrivateProfileIntW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_key_name = call.get_arg();
        let n_default = call.get_arg();
        let lp_file_name = call.get_arg();
        let res = api.GetPrivateProfileIntW(lp_app_name, lp_key_name, n_default, lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetPrivateProfileSectionA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_returned_string = call.get_arg();
        let n_size = call.get_arg();
        let lp_file_name = call.get_arg();
        let res =
            api.GetPrivateProfileSectionA(lp_app_name, lp_returned_string, n_size, lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetPrivateProfileSectionNamesA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_return_buffer = call.get_arg();
        let n_size = call.get_arg();
        let lp_file_name = call.get_arg();
        let res = api.GetPrivateProfileSectionNamesA(lpsz_return_buffer, n_size, lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetPrivateProfileSectionNamesW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_return_buffer = call.get_arg();
        let n_size = call.get_arg();
        let lp_file_name = call.get_arg();
        let res = api.GetPrivateProfileSectionNamesW(lpsz_return_buffer, n_size, lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetPrivateProfileSectionW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_returned_string = call.get_arg();
        let n_size = call.get_arg();
        let lp_file_name = call.get_arg();
        let res =
            api.GetPrivateProfileSectionW(lp_app_name, lp_returned_string, n_size, lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetPrivateProfileStringA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_key_name = call.get_arg();
        let lp_default = call.get_arg();
        let lp_returned_string = call.get_arg();
        let n_size = call.get_arg();
        let lp_file_name = call.get_arg();
        let res = api.GetPrivateProfileStringA(
            lp_app_name,
            lp_key_name,
            lp_default,
            lp_returned_string,
            n_size,
            lp_file_name,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetPrivateProfileStringW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_key_name = call.get_arg();
        let lp_default = call.get_arg();
        let lp_returned_string = call.get_arg();
        let n_size = call.get_arg();
        let lp_file_name = call.get_arg();
        let res = api.GetPrivateProfileStringW(
            lp_app_name,
            lp_key_name,
            lp_default,
            lp_returned_string,
            n_size,
            lp_file_name,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetPrivateProfileStructA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_section = call.get_arg();
        let lpsz_key = call.get_arg();
        let lp_struct = call.get_arg();
        let u_size_struct = call.get_arg();
        let sz_file = call.get_arg();
        let res =
            api.GetPrivateProfileStructA(lpsz_section, lpsz_key, lp_struct, u_size_struct, sz_file);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetPrivateProfileStructW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_section = call.get_arg();
        let lpsz_key = call.get_arg();
        let lp_struct = call.get_arg();
        let u_size_struct = call.get_arg();
        let sz_file = call.get_arg();
        let res =
            api.GetPrivateProfileStructW(lpsz_section, lpsz_key, lp_struct, u_size_struct, sz_file);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetProfileIntA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_key_name = call.get_arg();
        let n_default = call.get_arg();
        let res = api.GetProfileIntA(lp_app_name, lp_key_name, n_default);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetProfileIntW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_key_name = call.get_arg();
        let n_default = call.get_arg();
        let res = api.GetProfileIntW(lp_app_name, lp_key_name, n_default);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetProfileSectionA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_returned_string = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetProfileSectionA(lp_app_name, lp_returned_string, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetProfileSectionW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_returned_string = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetProfileSectionW(lp_app_name, lp_returned_string, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetProfileStringA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_key_name = call.get_arg();
        let lp_default = call.get_arg();
        let lp_returned_string = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetProfileStringA(
            lp_app_name,
            lp_key_name,
            lp_default,
            lp_returned_string,
            n_size,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetProfileStringW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_key_name = call.get_arg();
        let lp_default = call.get_arg();
        let lp_returned_string = call.get_arg();
        let n_size = call.get_arg();
        let res = api.GetProfileStringW(
            lp_app_name,
            lp_key_name,
            lp_default,
            lp_returned_string,
            n_size,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetSystemRegistryQuota(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let pdw_quota_allowed = call.get_arg();
        let pdw_quota_used = call.get_arg();
        let res = api.GetSystemRegistryQuota(pdw_quota_allowed, pdw_quota_used);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetThreadEnabledXStateFeatures(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.GetThreadEnabledXStateFeatures();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetUserNameA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_buffer = call.get_arg();
        let pcb_buffer = call.get_arg();
        let res = api.GetUserNameA(lp_buffer, pcb_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetUserNameW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_buffer = call.get_arg();
        let pcb_buffer = call.get_arg();
        let res = api.GetUserNameW(lp_buffer, pcb_buffer);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVersionFromFileA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_filename = call.get_arg();
        let pdw_ms_ver = call.get_arg();
        let pdw_ls_ver = call.get_arg();
        let b_version = call.get_arg();
        let res = api.GetVersionFromFileA(lpsz_filename, pdw_ms_ver, pdw_ls_ver, b_version);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVersionFromFileExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_filename = call.get_arg();
        let pdw_ms_ver = call.get_arg();
        let pdw_ls_ver = call.get_arg();
        let b_version = call.get_arg();
        let res = api.GetVersionFromFileExA(lpsz_filename, pdw_ms_ver, pdw_ls_ver, b_version);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVersionFromFileExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_filename = call.get_arg();
        let pdw_ms_ver = call.get_arg();
        let pdw_ls_ver = call.get_arg();
        let b_version = call.get_arg();
        let res = api.GetVersionFromFileExW(lpsz_filename, pdw_ms_ver, pdw_ls_ver, b_version);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GetVersionFromFileW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_filename = call.get_arg();
        let pdw_ms_ver = call.get_arg();
        let pdw_ls_ver = call.get_arg();
        let b_version = call.get_arg();
        let res = api.GetVersionFromFileW(lpsz_filename, pdw_ms_ver, pdw_ls_ver, b_version);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalCompact(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_min_free = call.get_arg();
        let res = api.GlobalCompact(dw_min_free);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalFix(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let res = api.GlobalFix(h_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalUnWire(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let res = api.GlobalUnWire(h_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalUnfix(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let res = api.GlobalUnfix(h_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_GlobalWire(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let res = api.GlobalWire(h_mem);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IMPGetIMEA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param_0 = call.get_arg();
        let param_1 = call.get_arg();
        let res = api.IMPGetIMEA(param_0, param_1);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IMPGetIMEW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param_0 = call.get_arg();
        let param_1 = call.get_arg();
        let res = api.IMPGetIMEW(param_0, param_1);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IMPQueryIMEA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param_0 = call.get_arg();
        let res = api.IMPQueryIMEA(param_0);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IMPQueryIMEW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param_0 = call.get_arg();
        let res = api.IMPQueryIMEW(param_0);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IMPSetIMEA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param_0 = call.get_arg();
        let param_1 = call.get_arg();
        let res = api.IMPSetIMEA(param_0, param_1);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IMPSetIMEW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param_0 = call.get_arg();
        let param_1 = call.get_arg();
        let res = api.IMPSetIMEW(param_0, param_1);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsApiSetImplemented(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let contract = call.get_arg();
        let res = api.IsApiSetImplemented(contract);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsBadHugeReadPtr(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp = call.get_arg();
        let ucb = call.get_arg();
        let res = api.IsBadHugeReadPtr(lp, ucb);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsBadHugeWritePtr(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp = call.get_arg();
        let ucb = call.get_arg();
        let res = api.IsBadHugeWritePtr(lp, ucb);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsNTAdmin(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_reserved = call.get_arg();
        let lpdw_reserved = call.get_arg();
        let res = api.IsNTAdmin(dw_reserved, lpdw_reserved);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsNativeVhdBoot(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let native_vhd_boot = call.get_arg();
        let res = api.IsNativeVhdBoot(native_vhd_boot);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_IsTokenUntrusted(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let token_handle = call.get_arg();
        let res = api.IsTokenUntrusted(token_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LaunchINFSectionExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let h_instance = call.get_arg();
        let psz_parms = call.get_arg();
        let n_show = call.get_arg();
        let res = api.LaunchINFSectionExW(hwnd, h_instance, psz_parms, n_show);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LaunchINFSectionW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd_owner = call.get_arg();
        let h_instance = call.get_arg();
        let psz_params = call.get_arg();
        let n_show = call.get_arg();
        let res = api.LaunchINFSectionW(hwnd_owner, h_instance, psz_params, n_show);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LocalCompact(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let u_min_free = call.get_arg();
        let res = api.LocalCompact(u_min_free);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_LocalShrink(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_mem = call.get_arg();
        let cb_new_size = call.get_arg();
        let res = api.LocalShrink(h_mem, cb_new_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_MulDiv(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let n_number = call.get_arg();
        let n_numerator = call.get_arg();
        let n_denominator = call.get_arg();
        let res = api.MulDiv(n_number, n_numerator, n_denominator);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NeedReboot(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_reboot_check = call.get_arg();
        let res = api.NeedReboot(dw_reboot_check);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NeedRebootInit(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.NeedRebootInit();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NtClose(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let handle = call.get_arg();
        let res = api.NtClose(handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NtDeviceIoControlFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let file_handle = call.get_arg();
        let event = call.get_arg();
        let apc_routine = call.get_arg();
        let apc_context = call.get_arg();
        let io_status_block = call.get_arg();
        let io_control_code = call.get_arg();
        let input_buffer = call.get_arg();
        let input_buffer_length = call.get_arg();
        let output_buffer = call.get_arg();
        let output_buffer_length = call.get_arg();
        let res = api.NtDeviceIoControlFile(
            file_handle,
            event,
            apc_routine,
            apc_context,
            io_status_block,
            io_control_code,
            input_buffer,
            input_buffer_length,
            output_buffer,
            output_buffer_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NtNotifyChangeMultipleKeys(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let master_key_handle = call.get_arg();
        let count = call.get_arg();
        let subordinate_objects = call.get_arg();
        let event = call.get_arg();
        let apc_routine = call.get_arg();
        let apc_context = call.get_arg();
        let io_status_block = call.get_arg();
        let completion_filter = call.get_arg();
        let watch_tree = call.get_arg();
        let buffer = call.get_arg();
        let buffer_size = call.get_arg();
        let asynchronous = call.get_arg();
        let res = api.NtNotifyChangeMultipleKeys(
            master_key_handle,
            count,
            subordinate_objects,
            event,
            apc_routine,
            apc_context,
            io_status_block,
            completion_filter,
            watch_tree,
            buffer,
            buffer_size,
            asynchronous,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NtOpenFile(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let file_handle = call.get_arg();
        let desired_access = call.get_arg();
        let object_attributes = call.get_arg();
        let io_status_block = call.get_arg();
        let share_access = call.get_arg();
        let open_options = call.get_arg();
        let res = api.NtOpenFile(
            file_handle,
            desired_access,
            object_attributes,
            io_status_block,
            share_access,
            open_options,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NtQueryMultipleValueKey(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let key_handle = call.get_arg();
        let value_entries = call.get_arg();
        let entry_count = call.get_arg();
        let value_buffer = call.get_arg();
        let buffer_length = call.get_arg();
        let required_buffer_length = call.get_arg();
        let res = api.NtQueryMultipleValueKey(
            key_handle,
            value_entries,
            entry_count,
            value_buffer,
            buffer_length,
            required_buffer_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NtQueryObject(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let handle = call.get_arg();
        let object_information_class = call.get_arg();
        let object_information = call.get_arg();
        let object_information_length = call.get_arg();
        let return_length = call.get_arg();
        let res = api.NtQueryObject(
            handle,
            object_information_class,
            object_information,
            object_information_length,
            return_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NtQuerySystemInformation(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let system_information_class = call.get_arg();
        let system_information = call.get_arg();
        let system_information_length = call.get_arg();
        let return_length = call.get_arg();
        let res = api.NtQuerySystemInformation(
            system_information_class,
            system_information,
            system_information_length,
            return_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NtQuerySystemTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let system_time = call.get_arg();
        let res = api.NtQuerySystemTime(system_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NtQueryTimerResolution(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let maximum_time = call.get_arg();
        let minimum_time = call.get_arg();
        let current_time = call.get_arg();
        let res = api.NtQueryTimerResolution(maximum_time, minimum_time, current_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NtRenameKey(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let key_handle = call.get_arg();
        let new_name = call.get_arg();
        let res = api.NtRenameKey(key_handle, new_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NtSetInformationKey(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let key_handle = call.get_arg();
        let key_set_information_class = call.get_arg();
        let key_set_information = call.get_arg();
        let key_set_information_length = call.get_arg();
        let res = api.NtSetInformationKey(
            key_handle,
            key_set_information_class,
            key_set_information,
            key_set_information_length,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_NtWaitForSingleObject(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let handle = call.get_arg();
        let alertable = call.get_arg();
        let timeout = call.get_arg();
        let res = api.NtWaitForSingleObject(handle, alertable, timeout);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenINFEngineA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let psz_inf_filename = call.get_arg();
        let psz_install_section = call.get_arg();
        let dw_flags = call.get_arg();
        let ph_inf = call.get_arg();
        let pv_reserved = call.get_arg();
        let res = api.OpenINFEngineA(
            psz_inf_filename,
            psz_install_section,
            dw_flags,
            ph_inf,
            pv_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenINFEngineW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let psz_inf_filename = call.get_arg();
        let psz_install_section = call.get_arg();
        let dw_flags = call.get_arg();
        let ph_inf = call.get_arg();
        let pv_reserved = call.get_arg();
        let res = api.OpenINFEngineW(
            psz_inf_filename,
            psz_install_section,
            dw_flags,
            ph_inf,
            pv_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenMutexA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_desired_access = call.get_arg();
        let b_inherit_handle = call.get_arg();
        let lp_name = call.get_arg();
        let res = api.OpenMutexA(dw_desired_access, b_inherit_handle, lp_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenSemaphoreA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_desired_access = call.get_arg();
        let b_inherit_handle = call.get_arg();
        let lp_name = call.get_arg();
        let res = api.OpenSemaphoreA(dw_desired_access, b_inherit_handle, lp_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_OpenWaitableTimerA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_desired_access = call.get_arg();
        let b_inherit_handle = call.get_arg();
        let lp_timer_name = call.get_arg();
        let res = api.OpenWaitableTimerA(dw_desired_access, b_inherit_handle, lp_timer_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryAuxiliaryCounterFrequency(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_auxiliary_counter_frequency = call.get_arg();
        let res = api.QueryAuxiliaryCounterFrequency(lp_auxiliary_counter_frequency);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryIdleProcessorCycleTime(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let buffer_length = call.get_arg();
        let processor_idle_cycle_time = call.get_arg();
        let res = api.QueryIdleProcessorCycleTime(buffer_length, processor_idle_cycle_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryIdleProcessorCycleTimeEx(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let group = call.get_arg();
        let buffer_length = call.get_arg();
        let processor_idle_cycle_time = call.get_arg();
        let res =
            api.QueryIdleProcessorCycleTimeEx(group, buffer_length, processor_idle_cycle_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryInterruptTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_interrupt_time = call.get_arg();
        let res = api.QueryInterruptTime(lp_interrupt_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryInterruptTimePrecise(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_interrupt_time_precise = call.get_arg();
        let res = api.QueryInterruptTimePrecise(lp_interrupt_time_precise);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryProcessCycleTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let process_handle = call.get_arg();
        let cycle_time = call.get_arg();
        let res = api.QueryProcessCycleTime(process_handle, cycle_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryThreadCycleTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let thread_handle = call.get_arg();
        let cycle_time = call.get_arg();
        let res = api.QueryThreadCycleTime(thread_handle, cycle_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryUnbiasedInterruptTime(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let unbiased_time = call.get_arg();
        let res = api.QueryUnbiasedInterruptTime(unbiased_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_QueryUnbiasedInterruptTimePrecise(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_unbiased_interrupt_time_precise = call.get_arg();
        let res = api.QueryUnbiasedInterruptTimePrecise(lp_unbiased_interrupt_time_precise);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RaiseCustomSystemEventTrigger(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let custom_system_event_trigger_config = call.get_arg();
        let res = api.RaiseCustomSystemEventTrigger(custom_system_event_trigger_config);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RebootCheckOnInstallA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let psz_inf = call.get_arg();
        let psz_sec = call.get_arg();
        let dw_reserved = call.get_arg();
        let res = api.RebootCheckOnInstallA(hwnd, psz_inf, psz_sec, dw_reserved);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RebootCheckOnInstallW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let psz_inf = call.get_arg();
        let psz_sec = call.get_arg();
        let dw_reserved = call.get_arg();
        let res = api.RebootCheckOnInstallW(hwnd, psz_inf, psz_sec, dw_reserved);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RecordFeatureError(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let feature_id = call.get_arg();
        let error = call.get_arg();
        let res = api.RecordFeatureError(feature_id, error);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RecordFeatureUsage(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let feature_id = call.get_arg();
        let kind = call.get_arg();
        let addend = call.get_arg();
        let origin_name = call.get_arg();
        let res = api.RecordFeatureUsage(feature_id, kind, addend, origin_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RegInstallA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hmod = call.get_arg();
        let psz_section = call.get_arg();
        let pst_table = call.get_arg();
        let res = api.RegInstallA(hmod, psz_section, pst_table);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RegInstallW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hmod = call.get_arg();
        let psz_section = call.get_arg();
        let pst_table = call.get_arg();
        let res = api.RegInstallW(hmod, psz_section, pst_table);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_ReplacePartitionUnit(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let target_partition = call.get_arg();
        let spare_partition = call.get_arg();
        let flags = call.get_arg();
        let res = api.ReplacePartitionUnit(target_partition, spare_partition, flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RequestDeviceWakeup(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_device = call.get_arg();
        let res = api.RequestDeviceWakeup(h_device);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlCharToInteger(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let string = call.get_arg();
        let base = call.get_arg();
        let value = call.get_arg();
        let res = api.RtlCharToInteger(string, base, value);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlFreeUnicodeString(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let unicode_string = call.get_arg();
        let res = api.RtlFreeUnicodeString(unicode_string);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlGetReturnAddressHijackTarget(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let res = api.RtlGetReturnAddressHijackTarget();
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlInitUnicodeString(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let destination_string = call.get_arg();
        let source_string = call.get_arg();
        let res = api.RtlInitUnicodeString(destination_string, source_string);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlLocalTimeToSystemTime(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let local_time = call.get_arg();
        let system_time = call.get_arg();
        let res = api.RtlLocalTimeToSystemTime(local_time, system_time);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlRaiseCustomSystemEventTrigger(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let trigger_config = call.get_arg();
        let res = api.RtlRaiseCustomSystemEventTrigger(trigger_config);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlTimeToSecondsSince1970(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let time = call.get_arg();
        let elapsed_seconds = call.get_arg();
        let res = api.RtlTimeToSecondsSince1970(time, elapsed_seconds);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlUnicodeToMultiByteSize(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let bytes_in_multi_byte_string = call.get_arg();
        let unicode_string = call.get_arg();
        let bytes_in_unicode_string = call.get_arg();
        let res = api.RtlUnicodeToMultiByteSize(
            bytes_in_multi_byte_string,
            unicode_string,
            bytes_in_unicode_string,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RtlUniform(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let seed = call.get_arg();
        let res = api.RtlUniform(seed);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RunSetupCommandA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_wnd = call.get_arg();
        let sz_cmd_name = call.get_arg();
        let sz_inf_section = call.get_arg();
        let sz_dir = call.get_arg();
        let lpsz_title = call.get_arg();
        let ph_exe = call.get_arg();
        let dw_flags = call.get_arg();
        let pv_reserved = call.get_arg();
        let res = api.RunSetupCommandA(
            h_wnd,
            sz_cmd_name,
            sz_inf_section,
            sz_dir,
            lpsz_title,
            ph_exe,
            dw_flags,
            pv_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_RunSetupCommandW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_wnd = call.get_arg();
        let sz_cmd_name = call.get_arg();
        let sz_inf_section = call.get_arg();
        let sz_dir = call.get_arg();
        let lpsz_title = call.get_arg();
        let ph_exe = call.get_arg();
        let dw_flags = call.get_arg();
        let pv_reserved = call.get_arg();
        let res = api.RunSetupCommandW(
            h_wnd,
            sz_cmd_name,
            sz_inf_section,
            sz_dir,
            lpsz_title,
            ph_exe,
            dw_flags,
            pv_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SendIMEMessageExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param_0 = call.get_arg();
        let param_1 = call.get_arg();
        let res = api.SendIMEMessageExA(param_0, param_1);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SendIMEMessageExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param_0 = call.get_arg();
        let param_1 = call.get_arg();
        let res = api.SendIMEMessageExW(param_0, param_1);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetEnvironmentStringsA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let new_environment = call.get_arg();
        let res = api.SetEnvironmentStringsA(new_environment);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFirmwareEnvironmentVariableA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let lp_guid = call.get_arg();
        let p_value = call.get_arg();
        let n_size = call.get_arg();
        let res = api.SetFirmwareEnvironmentVariableA(lp_name, lp_guid, p_value, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFirmwareEnvironmentVariableExA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let lp_guid = call.get_arg();
        let p_value = call.get_arg();
        let n_size = call.get_arg();
        let dw_attributes = call.get_arg();
        let res =
            api.SetFirmwareEnvironmentVariableExA(lp_name, lp_guid, p_value, n_size, dw_attributes);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFirmwareEnvironmentVariableExW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let lp_guid = call.get_arg();
        let p_value = call.get_arg();
        let n_size = call.get_arg();
        let dw_attributes = call.get_arg();
        let res =
            api.SetFirmwareEnvironmentVariableExW(lp_name, lp_guid, p_value, n_size, dw_attributes);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetFirmwareEnvironmentVariableW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_name = call.get_arg();
        let lp_guid = call.get_arg();
        let p_value = call.get_arg();
        let n_size = call.get_arg();
        let res = api.SetFirmwareEnvironmentVariableW(lp_name, lp_guid, p_value, n_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetHandleCount(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let u_number = call.get_arg();
        let res = api.SetHandleCount(u_number);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetMessageWaitingIndicator(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_msg_indicator = call.get_arg();
        let ul_msg_count = call.get_arg();
        let res = api.SetMessageWaitingIndicator(h_msg_indicator, ul_msg_count);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetPerUserSecValuesA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let p_per_user = call.get_arg();
        let res = api.SetPerUserSecValuesA(p_per_user);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SetPerUserSecValuesW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let p_per_user = call.get_arg();
        let res = api.SetPerUserSecValuesW(p_per_user);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SignalObjectAndWait(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_object_to_signal = call.get_arg();
        let h_object_to_wait_on = call.get_arg();
        let dw_milliseconds = call.get_arg();
        let b_alertable = call.get_arg();
        let res = api.SignalObjectAndWait(
            h_object_to_signal,
            h_object_to_wait_on,
            dw_milliseconds,
            b_alertable,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_SubscribeFeatureStateChangeNotification(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let subscription = call.get_arg();
        let callback = call.get_arg();
        let context = call.get_arg();
        let res = api.SubscribeFeatureStateChangeNotification(subscription, callback, context);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TranslateInfStringA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let psz_inf_filename = call.get_arg();
        let psz_install_section = call.get_arg();
        let psz_translate_section = call.get_arg();
        let psz_translate_key = call.get_arg();
        let psz_buffer = call.get_arg();
        let cch_buffer = call.get_arg();
        let pdw_required_size = call.get_arg();
        let pv_reserved = call.get_arg();
        let res = api.TranslateInfStringA(
            psz_inf_filename,
            psz_install_section,
            psz_translate_section,
            psz_translate_key,
            psz_buffer,
            cch_buffer,
            pdw_required_size,
            pv_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TranslateInfStringExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_inf = call.get_arg();
        let psz_inf_filename = call.get_arg();
        let psz_translate_section = call.get_arg();
        let psz_translate_key = call.get_arg();
        let psz_buffer = call.get_arg();
        let dw_buffer_size = call.get_arg();
        let pdw_required_size = call.get_arg();
        let pv_reserved = call.get_arg();
        let res = api.TranslateInfStringExA(
            h_inf,
            psz_inf_filename,
            psz_translate_section,
            psz_translate_key,
            psz_buffer,
            dw_buffer_size,
            pdw_required_size,
            pv_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TranslateInfStringExW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_inf = call.get_arg();
        let psz_inf_filename = call.get_arg();
        let psz_translate_section = call.get_arg();
        let psz_translate_key = call.get_arg();
        let psz_buffer = call.get_arg();
        let dw_buffer_size = call.get_arg();
        let pdw_required_size = call.get_arg();
        let pv_reserved = call.get_arg();
        let res = api.TranslateInfStringExW(
            h_inf,
            psz_inf_filename,
            psz_translate_section,
            psz_translate_key,
            psz_buffer,
            dw_buffer_size,
            pdw_required_size,
            pv_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_TranslateInfStringW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let psz_inf_filename = call.get_arg();
        let psz_install_section = call.get_arg();
        let psz_translate_section = call.get_arg();
        let psz_translate_key = call.get_arg();
        let psz_buffer = call.get_arg();
        let cch_buffer = call.get_arg();
        let pdw_required_size = call.get_arg();
        let pv_reserved = call.get_arg();
        let res = api.TranslateInfStringW(
            psz_inf_filename,
            psz_install_section,
            psz_translate_section,
            psz_translate_key,
            psz_buffer,
            cch_buffer,
            pdw_required_size,
            pv_reserved,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UnsubscribeFeatureStateChangeNotification(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let subscription = call.get_arg();
        let res = api.UnsubscribeFeatureStateChangeNotification(subscription);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UserInstStubWrapperA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let h_instance = call.get_arg();
        let psz_parms = call.get_arg();
        let n_show = call.get_arg();
        let res = api.UserInstStubWrapperA(hwnd, h_instance, psz_parms, n_show);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UserInstStubWrapperW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let h_instance = call.get_arg();
        let psz_parms = call.get_arg();
        let n_show = call.get_arg();
        let res = api.UserInstStubWrapperW(hwnd, h_instance, psz_parms, n_show);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UserUnInstStubWrapperA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let h_instance = call.get_arg();
        let psz_parms = call.get_arg();
        let n_show = call.get_arg();
        let res = api.UserUnInstStubWrapperA(hwnd, h_instance, psz_parms, n_show);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_UserUnInstStubWrapperW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let hwnd = call.get_arg();
        let h_instance = call.get_arg();
        let psz_parms = call.get_arg();
        let n_show = call.get_arg();
        let res = api.UserUnInstStubWrapperW(hwnd, h_instance, psz_parms, n_show);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WINNLSEnableIME(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param_0 = call.get_arg();
        let param_1 = call.get_arg();
        let res = api.WINNLSEnableIME(param_0, param_1);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WINNLSGetEnableStatus(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param_0 = call.get_arg();
        let res = api.WINNLSGetEnableStatus(param_0);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WINNLSGetIMEHotkey(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let param_0 = call.get_arg();
        let res = api.WINNLSGetIMEHotkey(param_0);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WldpGetLockdownPolicy(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let host_information = call.get_arg();
        let lockdown_state = call.get_arg();
        let lockdown_flags = call.get_arg();
        let res = api.WldpGetLockdownPolicy(host_information, lockdown_state, lockdown_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WldpIsClassInApprovedList(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let class_id = call.get_arg();
        let host_information = call.get_arg();
        let is_approved = call.get_arg();
        let optional_flags = call.get_arg();
        let res =
            api.WldpIsClassInApprovedList(class_id, host_information, is_approved, optional_flags);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WldpIsDynamicCodePolicyEnabled(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let is_enabled = call.get_arg();
        let res = api.WldpIsDynamicCodePolicyEnabled(is_enabled);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WldpQueryDeviceSecurityInformation(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let information = call.get_arg();
        let information_length = call.get_arg();
        let return_length = call.get_arg();
        let res =
            api.WldpQueryDeviceSecurityInformation(information, information_length, return_length);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WldpQueryDynamicCodeTrust(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let file_handle = call.get_arg();
        let base_image = call.get_arg();
        let image_size = call.get_arg();
        let res = api.WldpQueryDynamicCodeTrust(file_handle, base_image, image_size);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WldpSetDynamicCodeTrust(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let file_handle = call.get_arg();
        let res = api.WldpSetDynamicCodeTrust(file_handle);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WritePrivateProfileSectionA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_string = call.get_arg();
        let lp_file_name = call.get_arg();
        let res = api.WritePrivateProfileSectionA(lp_app_name, lp_string, lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WritePrivateProfileSectionW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_string = call.get_arg();
        let lp_file_name = call.get_arg();
        let res = api.WritePrivateProfileSectionW(lp_app_name, lp_string, lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WritePrivateProfileStringA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_key_name = call.get_arg();
        let lp_string = call.get_arg();
        let lp_file_name = call.get_arg();
        let res = api.WritePrivateProfileStringA(lp_app_name, lp_key_name, lp_string, lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WritePrivateProfileStringW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_key_name = call.get_arg();
        let lp_string = call.get_arg();
        let lp_file_name = call.get_arg();
        let res = api.WritePrivateProfileStringW(lp_app_name, lp_key_name, lp_string, lp_file_name);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WritePrivateProfileStructA(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_section = call.get_arg();
        let lpsz_key = call.get_arg();
        let lp_struct = call.get_arg();
        let u_size_struct = call.get_arg();
        let sz_file = call.get_arg();
        let res = api.WritePrivateProfileStructA(
            lpsz_section,
            lpsz_key,
            lp_struct,
            u_size_struct,
            sz_file,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WritePrivateProfileStructW(
    context: &mut ExtendedContext,
    memory: FlatMemoryCtx,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lpsz_section = call.get_arg();
        let lpsz_key = call.get_arg();
        let lp_struct = call.get_arg();
        let u_size_struct = call.get_arg();
        let sz_file = call.get_arg();
        let res = api.WritePrivateProfileStructW(
            lpsz_section,
            lpsz_key,
            lp_struct,
            u_size_struct,
            sz_file,
        );
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteProfileSectionA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_string = call.get_arg();
        let res = api.WriteProfileSectionA(lp_app_name, lp_string);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteProfileSectionW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_string = call.get_arg();
        let res = api.WriteProfileSectionW(lp_app_name, lp_string);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteProfileStringA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_key_name = call.get_arg();
        let lp_string = call.get_arg();
        let res = api.WriteProfileStringA(lp_app_name, lp_key_name, lp_string);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic_WriteProfileStringW(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_app_name = call.get_arg();
        let lp_key_name = call.get_arg();
        let lp_string = call.get_arg();
        let res = api.WriteProfileStringW(lp_app_name, lp_key_name, lp_string);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic__hread(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_buffer = call.get_arg();
        let l_bytes = call.get_arg();
        let res = api._hread(h_file, lp_buffer, l_bytes);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic__hwrite(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_buffer = call.get_arg();
        let l_bytes = call.get_arg();
        let res = api._hwrite(h_file, lp_buffer, l_bytes);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic__lclose(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let res = api._lclose(h_file);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic__lcreat(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let i_attribute = call.get_arg();
        let res = api._lcreat(lp_path_name, i_attribute);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic__llseek(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let l_offset = call.get_arg();
        let i_origin = call.get_arg();
        let res = api._llseek(h_file, l_offset, i_origin);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic__lopen(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let lp_path_name = call.get_arg();
        let i_read_write = call.get_arg();
        let res = api._lopen(lp_path_name, i_read_write);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic__lread(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_buffer = call.get_arg();
        let u_bytes = call.get_arg();
        let res = api._lread(h_file, lp_buffer, u_bytes);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
#[no_mangle]
extern "C" fn magic__lwrite(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::System::WindowsProgramming::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let h_file = call.get_arg();
        let lp_buffer = call.get_arg();
        let u_bytes = call.get_arg();
        let res = api._lwrite(h_file, lp_buffer, u_bytes);
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
        let lp_rect = call.get_arg();
        let dw_style = call.get_arg();
        let b_menu = call.get_arg();
        let res = api.AdjustWindowRect(lp_rect, dw_style, b_menu);
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
        let lp_rect = call.get_arg();
        let dw_style = call.get_arg();
        let b_menu = call.get_arg();
        let dw_ex_style = call.get_arg();
        let res = api.AdjustWindowRectEx(lp_rect, dw_style, b_menu, dw_ex_style);
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
        let dw_process_id = call.get_arg();
        let res = api.AllowSetForegroundWindow(dw_process_id);
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
        let h_wnd = call.get_arg();
        let dw_time = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.AnimateWindow(h_wnd, dw_time, dw_flags);
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
        let h_menu = call.get_arg();
        let u_flags = call.get_arg();
        let u_id_new_item = call.get_arg();
        let lp_new_item = call.get_arg();
        let res = api.AppendMenuA(h_menu, u_flags, u_id_new_item, lp_new_item);
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
        let h_menu = call.get_arg();
        let u_flags = call.get_arg();
        let u_id_new_item = call.get_arg();
        let lp_new_item = call.get_arg();
        let res = api.AppendMenuW(h_menu, u_flags, u_id_new_item, lp_new_item);
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
        let h_wnd = call.get_arg();
        let res = api.ArrangeIconicWindows(h_wnd);
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
        let n_num_windows = call.get_arg();
        let res = api.BeginDeferWindowPos(n_num_windows);
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
        let h_wnd = call.get_arg();
        let res = api.BringWindowToTop(h_wnd);
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
        let anchor_point = call.get_arg();
        let window_size = call.get_arg();
        let flags = call.get_arg();
        let exclude_rect = call.get_arg();
        let popup_window_position = call.get_arg();
        let res = api.CalculatePopupWindowPosition(
            anchor_point,
            window_size,
            flags,
            exclude_rect,
            popup_window_position,
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
        let lp_msg = call.get_arg();
        let n_code = call.get_arg();
        let res = api.CallMsgFilterA(lp_msg, n_code);
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
        let lp_msg = call.get_arg();
        let n_code = call.get_arg();
        let res = api.CallMsgFilterW(lp_msg, n_code);
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
        let n_code = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.CallNextHookEx(hhk, n_code, w_param, l_param);
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
        let lp_prev_wnd_func = call.get_arg();
        let h_wnd = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.CallWindowProcA(lp_prev_wnd_func, h_wnd, msg, w_param, l_param);
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
        let lp_prev_wnd_func = call.get_arg();
        let h_wnd = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.CallWindowProcW(lp_prev_wnd_func, h_wnd, msg, w_param, l_param);
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
        let hwnd_parent = call.get_arg();
        let w_how = call.get_arg();
        let lp_rect = call.get_arg();
        let c_kids = call.get_arg();
        let lp_kids = call.get_arg();
        let res = api.CascadeWindows(hwnd_parent, w_how, lp_rect, c_kids, lp_kids);
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
        let h_menu = call.get_arg();
        let cmd = call.get_arg();
        let lpsz_new_item = call.get_arg();
        let cmd_insert = call.get_arg();
        let flags = call.get_arg();
        let res = api.ChangeMenuA(h_menu, cmd, lpsz_new_item, cmd_insert, flags);
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
        let h_menu = call.get_arg();
        let cmd = call.get_arg();
        let lpsz_new_item = call.get_arg();
        let cmd_insert = call.get_arg();
        let flags = call.get_arg();
        let res = api.ChangeMenuW(h_menu, cmd, lpsz_new_item, cmd_insert, flags);
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
        let dw_flag = call.get_arg();
        let res = api.ChangeWindowMessageFilter(message, dw_flag);
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
        let p_change_filter_struct = call.get_arg();
        let res = api.ChangeWindowMessageFilterEx(hwnd, message, action, p_change_filter_struct);
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
        let cch_length = call.get_arg();
        let res = api.CharLowerBuffA(lpsz, cch_length);
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
        let cch_length = call.get_arg();
        let res = api.CharLowerBuffW(lpsz, cch_length);
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
        let code_page = call.get_arg();
        let lp_current_char = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.CharNextExA(code_page, lp_current_char, dw_flags);
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
        let lpsz_start = call.get_arg();
        let lpsz_current = call.get_arg();
        let res = api.CharPrevA(lpsz_start, lpsz_current);
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
        let code_page = call.get_arg();
        let lp_start = call.get_arg();
        let lp_current_char = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.CharPrevExA(code_page, lp_start, lp_current_char, dw_flags);
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
        let lpsz_start = call.get_arg();
        let lpsz_current = call.get_arg();
        let res = api.CharPrevW(lpsz_start, lpsz_current);
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
        let p_src = call.get_arg();
        let p_dst = call.get_arg();
        let res = api.CharToOemA(p_src, p_dst);
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
        let lpsz_src = call.get_arg();
        let lpsz_dst = call.get_arg();
        let cch_dst_length = call.get_arg();
        let res = api.CharToOemBuffA(lpsz_src, lpsz_dst, cch_dst_length);
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
        let lpsz_src = call.get_arg();
        let lpsz_dst = call.get_arg();
        let cch_dst_length = call.get_arg();
        let res = api.CharToOemBuffW(lpsz_src, lpsz_dst, cch_dst_length);
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
        let p_src = call.get_arg();
        let p_dst = call.get_arg();
        let res = api.CharToOemW(p_src, p_dst);
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
        let cch_length = call.get_arg();
        let res = api.CharUpperBuffA(lpsz, cch_length);
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
        let cch_length = call.get_arg();
        let res = api.CharUpperBuffW(lpsz, cch_length);
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
        let h_menu = call.get_arg();
        let u_id_check_item = call.get_arg();
        let u_check = call.get_arg();
        let res = api.CheckMenuItem(h_menu, u_id_check_item, u_check);
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
        let h_wnd_parent = call.get_arg();
        let point = call.get_arg();
        let res = api.ChildWindowFromPoint(h_wnd_parent, point);
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
        let lp_rect = call.get_arg();
        let res = api.ClipCursor(lp_rect);
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
        let h_wnd = call.get_arg();
        let res = api.CloseWindow(h_wnd);
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
        let h_accel_src = call.get_arg();
        let lp_accel_dst = call.get_arg();
        let c_accel_entries = call.get_arg();
        let res = api.CopyAcceleratorTableA(h_accel_src, lp_accel_dst, c_accel_entries);
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
        let h_accel_src = call.get_arg();
        let lp_accel_dst = call.get_arg();
        let c_accel_entries = call.get_arg();
        let res = api.CopyAcceleratorTableW(h_accel_src, lp_accel_dst, c_accel_entries);
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
        let h_icon = call.get_arg();
        let res = api.CopyIcon(h_icon);
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
        let c_accel = call.get_arg();
        let res = api.CreateAcceleratorTableA(paccel, c_accel);
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
        let c_accel = call.get_arg();
        let res = api.CreateAcceleratorTableW(paccel, c_accel);
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
        let h_inst = call.get_arg();
        let x_hot_spot = call.get_arg();
        let y_hot_spot = call.get_arg();
        let n_width = call.get_arg();
        let n_height = call.get_arg();
        let pv_and_plane = call.get_arg();
        let pv_xor_plane = call.get_arg();
        let res = api.CreateCursor(
            h_inst,
            x_hot_spot,
            y_hot_spot,
            n_width,
            n_height,
            pv_and_plane,
            pv_xor_plane,
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
        let h_instance = call.get_arg();
        let lp_template = call.get_arg();
        let h_wnd_parent = call.get_arg();
        let lp_dialog_func = call.get_arg();
        let dw_init_param = call.get_arg();
        let res = api.CreateDialogIndirectParamA(
            h_instance,
            lp_template,
            h_wnd_parent,
            lp_dialog_func,
            dw_init_param,
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
        let h_instance = call.get_arg();
        let lp_template = call.get_arg();
        let h_wnd_parent = call.get_arg();
        let lp_dialog_func = call.get_arg();
        let dw_init_param = call.get_arg();
        let res = api.CreateDialogIndirectParamW(
            h_instance,
            lp_template,
            h_wnd_parent,
            lp_dialog_func,
            dw_init_param,
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
        let h_instance = call.get_arg();
        let lp_template_name = call.get_arg();
        let h_wnd_parent = call.get_arg();
        let lp_dialog_func = call.get_arg();
        let dw_init_param = call.get_arg();
        let res = api.CreateDialogParamA(
            h_instance,
            lp_template_name,
            h_wnd_parent,
            lp_dialog_func,
            dw_init_param,
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
        let h_instance = call.get_arg();
        let lp_template_name = call.get_arg();
        let h_wnd_parent = call.get_arg();
        let lp_dialog_func = call.get_arg();
        let dw_init_param = call.get_arg();
        let res = api.CreateDialogParamW(
            h_instance,
            lp_template_name,
            h_wnd_parent,
            lp_dialog_func,
            dw_init_param,
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
        let h_instance = call.get_arg();
        let n_width = call.get_arg();
        let n_height = call.get_arg();
        let c_planes = call.get_arg();
        let c_bits_pixel = call.get_arg();
        let lpb_an_dbits = call.get_arg();
        let lpb_xo_rbits = call.get_arg();
        let res = api.CreateIcon(
            h_instance,
            n_width,
            n_height,
            c_planes,
            c_bits_pixel,
            lpb_an_dbits,
            lpb_xo_rbits,
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
        let dw_res_size = call.get_arg();
        let f_icon = call.get_arg();
        let dw_ver = call.get_arg();
        let res = api.CreateIconFromResource(presbits, dw_res_size, f_icon, dw_ver);
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
        let dw_res_size = call.get_arg();
        let f_icon = call.get_arg();
        let dw_ver = call.get_arg();
        let cx_desired = call.get_arg();
        let cy_desired = call.get_arg();
        let flags = call.get_arg();
        let res = api.CreateIconFromResourceEx(
            presbits,
            dw_res_size,
            f_icon,
            dw_ver,
            cx_desired,
            cy_desired,
            flags,
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
        let lp_class_name = call.get_arg();
        let lp_window_name = call.get_arg();
        let dw_style = call.get_arg();
        let x = call.get_arg();
        let y = call.get_arg();
        let n_width = call.get_arg();
        let n_height = call.get_arg();
        let h_wnd_parent = call.get_arg();
        let h_instance = call.get_arg();
        let l_param = call.get_arg();
        let res = api.CreateMDIWindowA(
            lp_class_name,
            lp_window_name,
            dw_style,
            x,
            y,
            n_width,
            n_height,
            h_wnd_parent,
            h_instance,
            l_param,
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
        let lp_class_name = call.get_arg();
        let lp_window_name = call.get_arg();
        let dw_style = call.get_arg();
        let x = call.get_arg();
        let y = call.get_arg();
        let n_width = call.get_arg();
        let n_height = call.get_arg();
        let h_wnd_parent = call.get_arg();
        let h_instance = call.get_arg();
        let l_param = call.get_arg();
        let res = api.CreateMDIWindowW(
            lp_class_name,
            lp_window_name,
            dw_style,
            x,
            y,
            n_width,
            n_height,
            h_wnd_parent,
            h_instance,
            l_param,
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
extern "C" fn magic_CreateWindowExA(context: &mut ExtendedContext, memory: FlatMemoryCtx) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let api = win32::Win32::UI::WindowsAndMessaging::get_api(&context.win32);
        let mut call = StdCallHelper::new(memory, &mut context.cpu);
        let dw_ex_style = call.get_arg();
        let lp_class_name = call.get_arg();
        let lp_window_name = call.get_arg();
        let dw_style = call.get_arg();
        let x = call.get_arg();
        let y = call.get_arg();
        let n_width = call.get_arg();
        let n_height = call.get_arg();
        let h_wnd_parent = call.get_arg();
        let h_menu = call.get_arg();
        let h_instance = call.get_arg();
        let lp_param = call.get_arg();
        let res = api.CreateWindowExA(
            dw_ex_style,
            lp_class_name,
            lp_window_name,
            dw_style,
            x,
            y,
            n_width,
            n_height,
            h_wnd_parent,
            h_menu,
            h_instance,
            lp_param,
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
        let dw_ex_style = call.get_arg();
        let lp_class_name = call.get_arg();
        let lp_window_name = call.get_arg();
        let dw_style = call.get_arg();
        let x = call.get_arg();
        let y = call.get_arg();
        let n_width = call.get_arg();
        let n_height = call.get_arg();
        let h_wnd_parent = call.get_arg();
        let h_menu = call.get_arg();
        let h_instance = call.get_arg();
        let lp_param = call.get_arg();
        let res = api.CreateWindowExW(
            dw_ex_style,
            lp_class_name,
            lp_window_name,
            dw_style,
            x,
            y,
            n_width,
            n_height,
            h_wnd_parent,
            h_menu,
            h_instance,
            lp_param,
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
        let h_dlg = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.DefDlgProcA(h_dlg, msg, w_param, l_param);
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
        let h_dlg = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.DefDlgProcW(h_dlg, msg, w_param, l_param);
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
        let h_wnd = call.get_arg();
        let h_wnd_mdi_client = call.get_arg();
        let u_msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.DefFrameProcA(h_wnd, h_wnd_mdi_client, u_msg, w_param, l_param);
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
        let h_wnd = call.get_arg();
        let h_wnd_mdi_client = call.get_arg();
        let u_msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.DefFrameProcW(h_wnd, h_wnd_mdi_client, u_msg, w_param, l_param);
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
        let h_wnd = call.get_arg();
        let u_msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.DefMDIChildProcA(h_wnd, u_msg, w_param, l_param);
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
        let h_wnd = call.get_arg();
        let u_msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.DefMDIChildProcW(h_wnd, u_msg, w_param, l_param);
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
        let h_wnd = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.DefWindowProcA(h_wnd, msg, w_param, l_param);
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
        let h_wnd = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.DefWindowProcW(h_wnd, msg, w_param, l_param);
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
        let h_win_pos_info = call.get_arg();
        let h_wnd = call.get_arg();
        let h_wnd_insert_after = call.get_arg();
        let x = call.get_arg();
        let y = call.get_arg();
        let cx = call.get_arg();
        let cy = call.get_arg();
        let u_flags = call.get_arg();
        let res = api.DeferWindowPos(
            h_win_pos_info,
            h_wnd,
            h_wnd_insert_after,
            x,
            y,
            cx,
            cy,
            u_flags,
        );
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
        let h_menu = call.get_arg();
        let u_position = call.get_arg();
        let u_flags = call.get_arg();
        let res = api.DeleteMenu(h_menu, u_position, u_flags);
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
        let h_accel = call.get_arg();
        let res = api.DestroyAcceleratorTable(h_accel);
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
        let h_cursor = call.get_arg();
        let res = api.DestroyCursor(h_cursor);
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
        let h_icon = call.get_arg();
        let res = api.DestroyIcon(h_icon);
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
        let h_menu = call.get_arg();
        let res = api.DestroyMenu(h_menu);
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
        let h_wnd = call.get_arg();
        let res = api.DestroyWindow(h_wnd);
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
        let h_instance = call.get_arg();
        let h_dialog_template = call.get_arg();
        let h_wnd_parent = call.get_arg();
        let lp_dialog_func = call.get_arg();
        let dw_init_param = call.get_arg();
        let res = api.DialogBoxIndirectParamA(
            h_instance,
            h_dialog_template,
            h_wnd_parent,
            lp_dialog_func,
            dw_init_param,
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
        let h_instance = call.get_arg();
        let h_dialog_template = call.get_arg();
        let h_wnd_parent = call.get_arg();
        let lp_dialog_func = call.get_arg();
        let dw_init_param = call.get_arg();
        let res = api.DialogBoxIndirectParamW(
            h_instance,
            h_dialog_template,
            h_wnd_parent,
            lp_dialog_func,
            dw_init_param,
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
        let h_instance = call.get_arg();
        let lp_template_name = call.get_arg();
        let h_wnd_parent = call.get_arg();
        let lp_dialog_func = call.get_arg();
        let dw_init_param = call.get_arg();
        let res = api.DialogBoxParamA(
            h_instance,
            lp_template_name,
            h_wnd_parent,
            lp_dialog_func,
            dw_init_param,
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
        let h_instance = call.get_arg();
        let lp_template_name = call.get_arg();
        let h_wnd_parent = call.get_arg();
        let lp_dialog_func = call.get_arg();
        let dw_init_param = call.get_arg();
        let res = api.DialogBoxParamW(
            h_instance,
            lp_template_name,
            h_wnd_parent,
            lp_dialog_func,
            dw_init_param,
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
        let lp_msg = call.get_arg();
        let res = api.DispatchMessageA(lp_msg);
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
        let lp_msg = call.get_arg();
        let res = api.DispatchMessageW(lp_msg);
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
        let hwnd_parent = call.get_arg();
        let hwnd_from = call.get_arg();
        let fmt = call.get_arg();
        let data = call.get_arg();
        let hcur = call.get_arg();
        let res = api.DragObject(hwnd_parent, hwnd_from, fmt, data, hcur);
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
        let h_wnd = call.get_arg();
        let res = api.DrawMenuBar(h_wnd);
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
        let h_menu = call.get_arg();
        let u_id_enable_item = call.get_arg();
        let u_enable = call.get_arg();
        let res = api.EnableMenuItem(h_menu, u_id_enable_item, u_enable);
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
        let h_win_pos_info = call.get_arg();
        let res = api.EndDeferWindowPos(h_win_pos_info);
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
        let h_dlg = call.get_arg();
        let n_result = call.get_arg();
        let res = api.EndDialog(h_dlg, n_result);
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
        let h_wnd_parent = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumChildWindows(h_wnd_parent, lp_enum_func, l_param);
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
        let h_wnd = call.get_arg();
        let lp_enum_func = call.get_arg();
        let res = api.EnumPropsA(h_wnd, lp_enum_func);
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
        let h_wnd = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumPropsExA(h_wnd, lp_enum_func, l_param);
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
        let h_wnd = call.get_arg();
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumPropsExW(h_wnd, lp_enum_func, l_param);
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
        let h_wnd = call.get_arg();
        let lp_enum_func = call.get_arg();
        let res = api.EnumPropsW(h_wnd, lp_enum_func);
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
        let dw_thread_id = call.get_arg();
        let lpfn = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumThreadWindows(dw_thread_id, lpfn, l_param);
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
        let lp_enum_func = call.get_arg();
        let l_param = call.get_arg();
        let res = api.EnumWindows(lp_enum_func, l_param);
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
        let lp_class_name = call.get_arg();
        let lp_window_name = call.get_arg();
        let res = api.FindWindowA(lp_class_name, lp_window_name);
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
        let h_wnd_parent = call.get_arg();
        let h_wnd_child_after = call.get_arg();
        let lpsz_class = call.get_arg();
        let lpsz_window = call.get_arg();
        let res = api.FindWindowExA(h_wnd_parent, h_wnd_child_after, lpsz_class, lpsz_window);
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
        let h_wnd_parent = call.get_arg();
        let h_wnd_child_after = call.get_arg();
        let lpsz_class = call.get_arg();
        let lpsz_window = call.get_arg();
        let res = api.FindWindowExW(h_wnd_parent, h_wnd_child_after, lpsz_class, lpsz_window);
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
        let lp_class_name = call.get_arg();
        let lp_window_name = call.get_arg();
        let res = api.FindWindowW(lp_class_name, lp_window_name);
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
        let h_wnd = call.get_arg();
        let b_invert = call.get_arg();
        let res = api.FlashWindow(h_wnd, b_invert);
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
        let i_item = call.get_arg();
        let pati = call.get_arg();
        let psz_item_text = call.get_arg();
        let cch_item_text = call.get_arg();
        let res = api.GetAltTabInfoA(hwnd, i_item, pati, psz_item_text, cch_item_text);
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
        let i_item = call.get_arg();
        let pati = call.get_arg();
        let psz_item_text = call.get_arg();
        let cch_item_text = call.get_arg();
        let res = api.GetAltTabInfoW(hwnd, i_item, pati, psz_item_text, cch_item_text);
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
        let ga_flags = call.get_arg();
        let res = api.GetAncestor(hwnd, ga_flags);
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
        let lp_point = call.get_arg();
        let res = api.GetCaretPos(lp_point);
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
        let h_wnd = call.get_arg();
        let n_index = call.get_arg();
        let res = api.GetClassLongA(h_wnd, n_index);
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
        let h_wnd = call.get_arg();
        let n_index = call.get_arg();
        let res = api.GetClassLongW(h_wnd, n_index);
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
        let h_wnd = call.get_arg();
        let lp_class_name = call.get_arg();
        let n_max_count = call.get_arg();
        let res = api.GetClassNameA(h_wnd, lp_class_name, n_max_count);
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
        let h_wnd = call.get_arg();
        let lp_class_name = call.get_arg();
        let n_max_count = call.get_arg();
        let res = api.GetClassNameW(h_wnd, lp_class_name, n_max_count);
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
        let h_wnd = call.get_arg();
        let n_index = call.get_arg();
        let res = api.GetClassWord(h_wnd, n_index);
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
        let h_wnd = call.get_arg();
        let lp_rect = call.get_arg();
        let res = api.GetClientRect(h_wnd, lp_rect);
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
        let lp_rect = call.get_arg();
        let res = api.GetClipCursor(lp_rect);
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
        let lp_point = call.get_arg();
        let res = api.GetCursorPos(lp_point);
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
        let h_wnd = call.get_arg();
        let res = api.GetDlgCtrlID(h_wnd);
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
        let h_dlg = call.get_arg();
        let n_id_dlg_item = call.get_arg();
        let res = api.GetDlgItem(h_dlg, n_id_dlg_item);
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
        let h_dlg = call.get_arg();
        let n_id_dlg_item = call.get_arg();
        let lp_translated = call.get_arg();
        let b_signed = call.get_arg();
        let res = api.GetDlgItemInt(h_dlg, n_id_dlg_item, lp_translated, b_signed);
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
        let h_dlg = call.get_arg();
        let n_id_dlg_item = call.get_arg();
        let lp_string = call.get_arg();
        let cch_max = call.get_arg();
        let res = api.GetDlgItemTextA(h_dlg, n_id_dlg_item, lp_string, cch_max);
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
        let h_dlg = call.get_arg();
        let n_id_dlg_item = call.get_arg();
        let lp_string = call.get_arg();
        let cch_max = call.get_arg();
        let res = api.GetDlgItemTextW(h_dlg, n_id_dlg_item, lp_string, cch_max);
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
        let id_thread = call.get_arg();
        let pgui = call.get_arg();
        let res = api.GetGUIThreadInfo(id_thread, pgui);
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
        let h_wnd = call.get_arg();
        let res = api.GetLastActivePopup(h_wnd);
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
        let pcr_key = call.get_arg();
        let pb_alpha = call.get_arg();
        let pdw_flags = call.get_arg();
        let res = api.GetLayeredWindowAttributes(hwnd, pcr_key, pb_alpha, pdw_flags);
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
        let h_wnd = call.get_arg();
        let res = api.GetMenu(h_wnd);
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
        let id_object = call.get_arg();
        let id_item = call.get_arg();
        let pmbi = call.get_arg();
        let res = api.GetMenuBarInfo(hwnd, id_object, id_item, pmbi);
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
        let h_menu = call.get_arg();
        let f_by_pos = call.get_arg();
        let gmdi_flags = call.get_arg();
        let res = api.GetMenuDefaultItem(h_menu, f_by_pos, gmdi_flags);
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
        let h_menu = call.get_arg();
        let res = api.GetMenuItemCount(h_menu);
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
        let h_menu = call.get_arg();
        let n_pos = call.get_arg();
        let res = api.GetMenuItemID(h_menu, n_pos);
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
        let h_wnd = call.get_arg();
        let h_menu = call.get_arg();
        let u_item = call.get_arg();
        let lprc_item = call.get_arg();
        let res = api.GetMenuItemRect(h_wnd, h_menu, u_item, lprc_item);
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
        let h_menu = call.get_arg();
        let u_id = call.get_arg();
        let u_flags = call.get_arg();
        let res = api.GetMenuState(h_menu, u_id, u_flags);
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
        let h_menu = call.get_arg();
        let u_id_item = call.get_arg();
        let lp_string = call.get_arg();
        let cch_max = call.get_arg();
        let flags = call.get_arg();
        let res = api.GetMenuStringA(h_menu, u_id_item, lp_string, cch_max, flags);
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
        let h_menu = call.get_arg();
        let u_id_item = call.get_arg();
        let lp_string = call.get_arg();
        let cch_max = call.get_arg();
        let flags = call.get_arg();
        let res = api.GetMenuStringW(h_menu, u_id_item, lp_string, cch_max, flags);
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
        let lp_msg = call.get_arg();
        let h_wnd = call.get_arg();
        let w_msg_filter_min = call.get_arg();
        let w_msg_filter_max = call.get_arg();
        let res = api.GetMessageA(lp_msg, h_wnd, w_msg_filter_min, w_msg_filter_max);
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
        let lp_msg = call.get_arg();
        let h_wnd = call.get_arg();
        let w_msg_filter_min = call.get_arg();
        let w_msg_filter_max = call.get_arg();
        let res = api.GetMessageW(lp_msg, h_wnd, w_msg_filter_min, w_msg_filter_max);
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
        let h_dlg = call.get_arg();
        let h_ctl = call.get_arg();
        let b_previous = call.get_arg();
        let res = api.GetNextDlgGroupItem(h_dlg, h_ctl, b_previous);
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
        let h_dlg = call.get_arg();
        let h_ctl = call.get_arg();
        let b_previous = call.get_arg();
        let res = api.GetNextDlgTabItem(h_dlg, h_ctl, b_previous);
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
        let h_wnd = call.get_arg();
        let res = api.GetParent(h_wnd);
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
        let lp_point = call.get_arg();
        let res = api.GetPhysicalCursorPos(lp_point);
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
        let pdw_default_layout = call.get_arg();
        let res = api.GetProcessDefaultLayout(pdw_default_layout);
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
        let h_wnd = call.get_arg();
        let lp_string = call.get_arg();
        let res = api.GetPropA(h_wnd, lp_string);
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
        let h_wnd = call.get_arg();
        let lp_string = call.get_arg();
        let res = api.GetPropW(h_wnd, lp_string);
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
        let id_object = call.get_arg();
        let psbi = call.get_arg();
        let res = api.GetScrollBarInfo(hwnd, id_object, psbi);
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
        let n_bar = call.get_arg();
        let lpsi = call.get_arg();
        let res = api.GetScrollInfo(hwnd, n_bar, lpsi);
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
        let h_wnd = call.get_arg();
        let n_bar = call.get_arg();
        let res = api.GetScrollPos(h_wnd, n_bar);
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
        let h_wnd = call.get_arg();
        let n_bar = call.get_arg();
        let lp_min_pos = call.get_arg();
        let lp_max_pos = call.get_arg();
        let res = api.GetScrollRange(h_wnd, n_bar, lp_min_pos, lp_max_pos);
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
        let h_menu = call.get_arg();
        let n_pos = call.get_arg();
        let res = api.GetSubMenu(h_menu, n_pos);
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
        let n_index = call.get_arg();
        let res = api.GetSysColor(n_index);
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
        let h_wnd = call.get_arg();
        let b_revert = call.get_arg();
        let res = api.GetSystemMenu(h_wnd, b_revert);
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
        let n_index = call.get_arg();
        let res = api.GetSystemMetrics(n_index);
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
        let h_wnd = call.get_arg();
        let res = api.GetTopWindow(h_wnd);
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
        let h_wnd = call.get_arg();
        let u_cmd = call.get_arg();
        let res = api.GetWindow(h_wnd, u_cmd);
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
        let h_wnd = call.get_arg();
        let pdw_affinity = call.get_arg();
        let res = api.GetWindowDisplayAffinity(h_wnd, pdw_affinity);
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
        let h_wnd = call.get_arg();
        let n_index = call.get_arg();
        let res = api.GetWindowLongA(h_wnd, n_index);
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
        let h_wnd = call.get_arg();
        let n_index = call.get_arg();
        let res = api.GetWindowLongW(h_wnd, n_index);
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
        let psz_file_name = call.get_arg();
        let cch_file_name_max = call.get_arg();
        let res = api.GetWindowModuleFileNameA(hwnd, psz_file_name, cch_file_name_max);
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
        let psz_file_name = call.get_arg();
        let cch_file_name_max = call.get_arg();
        let res = api.GetWindowModuleFileNameW(hwnd, psz_file_name, cch_file_name_max);
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
        let h_wnd = call.get_arg();
        let lpwndpl = call.get_arg();
        let res = api.GetWindowPlacement(h_wnd, lpwndpl);
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
        let h_wnd = call.get_arg();
        let lp_rect = call.get_arg();
        let res = api.GetWindowRect(h_wnd, lp_rect);
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
        let h_wnd = call.get_arg();
        let lp_string = call.get_arg();
        let n_max_count = call.get_arg();
        let res = api.GetWindowTextA(h_wnd, lp_string, n_max_count);
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
        let h_wnd = call.get_arg();
        let res = api.GetWindowTextLengthA(h_wnd);
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
        let h_wnd = call.get_arg();
        let res = api.GetWindowTextLengthW(h_wnd);
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
        let h_wnd = call.get_arg();
        let lp_string = call.get_arg();
        let n_max_count = call.get_arg();
        let res = api.GetWindowTextW(h_wnd, lp_string, n_max_count);
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
        let h_wnd = call.get_arg();
        let lpdw_process_id = call.get_arg();
        let res = api.GetWindowThreadProcessId(h_wnd, lpdw_process_id);
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
        let h_wnd = call.get_arg();
        let n_index = call.get_arg();
        let res = api.GetWindowWord(h_wnd, n_index);
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
        let h_wnd = call.get_arg();
        let res = api.HideCaret(h_wnd);
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
        let h_wnd = call.get_arg();
        let h_menu = call.get_arg();
        let u_id_hilite_item = call.get_arg();
        let u_hilite = call.get_arg();
        let res = api.HiliteMenuItem(h_wnd, h_menu, u_id_hilite_item, u_hilite);
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
        let lp_reserved = call.get_arg();
        let res = api.InSendMessageEx(lp_reserved);
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
        let hwnd_inherit = call.get_arg();
        let res = api.InheritWindowMonitor(hwnd, hwnd_inherit);
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
        let h_menu = call.get_arg();
        let u_position = call.get_arg();
        let u_flags = call.get_arg();
        let u_id_new_item = call.get_arg();
        let lp_new_item = call.get_arg();
        let res = api.InsertMenuA(h_menu, u_position, u_flags, u_id_new_item, lp_new_item);
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
        let h_menu = call.get_arg();
        let u_position = call.get_arg();
        let u_flags = call.get_arg();
        let u_id_new_item = call.get_arg();
        let lp_new_item = call.get_arg();
        let res = api.InsertMenuW(h_menu, u_position, u_flags, u_id_new_item, lp_new_item);
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
        let h_wnd = call.get_arg();
        let p_string = call.get_arg();
        let cch_max_count = call.get_arg();
        let res = api.InternalGetWindowText(h_wnd, p_string, cch_max_count);
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
        let h_wnd_parent = call.get_arg();
        let h_wnd = call.get_arg();
        let res = api.IsChild(h_wnd_parent, h_wnd);
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
        let h_dlg = call.get_arg();
        let lp_msg = call.get_arg();
        let res = api.IsDialogMessageA(h_dlg, lp_msg);
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
        let h_dlg = call.get_arg();
        let lp_msg = call.get_arg();
        let res = api.IsDialogMessageW(h_dlg, lp_msg);
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
        let b_convert = call.get_arg();
        let res = api.IsGUIThread(b_convert);
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
        let h_wnd = call.get_arg();
        let res = api.IsIconic(h_wnd);
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
        let h_menu = call.get_arg();
        let res = api.IsMenu(h_menu);
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
        let h_wnd = call.get_arg();
        let res = api.IsWindow(h_wnd);
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
        let h_wnd = call.get_arg();
        let res = api.IsWindowUnicode(h_wnd);
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
        let h_wnd = call.get_arg();
        let res = api.IsWindowVisible(h_wnd);
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
        let h_wnd = call.get_arg();
        let res = api.IsZoomed(h_wnd);
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
        let h_wnd = call.get_arg();
        let u_id_event = call.get_arg();
        let res = api.KillTimer(h_wnd, u_id_event);
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
        let h_instance = call.get_arg();
        let lp_table_name = call.get_arg();
        let res = api.LoadAcceleratorsA(h_instance, lp_table_name);
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
        let h_instance = call.get_arg();
        let lp_table_name = call.get_arg();
        let res = api.LoadAcceleratorsW(h_instance, lp_table_name);
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
        let h_instance = call.get_arg();
        let lp_cursor_name = call.get_arg();
        let res = api.LoadCursorA(h_instance, lp_cursor_name);
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
        let lp_file_name = call.get_arg();
        let res = api.LoadCursorFromFileA(lp_file_name);
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
        let lp_file_name = call.get_arg();
        let res = api.LoadCursorFromFileW(lp_file_name);
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
        let h_instance = call.get_arg();
        let lp_cursor_name = call.get_arg();
        let res = api.LoadCursorW(h_instance, lp_cursor_name);
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
        let h_instance = call.get_arg();
        let lp_icon_name = call.get_arg();
        let res = api.LoadIconA(h_instance, lp_icon_name);
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
        let h_instance = call.get_arg();
        let lp_icon_name = call.get_arg();
        let res = api.LoadIconW(h_instance, lp_icon_name);
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
        let h_inst = call.get_arg();
        let name = call.get_arg();
        let r#type = call.get_arg();
        let cx = call.get_arg();
        let cy = call.get_arg();
        let fu_load = call.get_arg();
        let res = api.LoadImageA(h_inst, name, r#type, cx, cy, fu_load);
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
        let h_inst = call.get_arg();
        let name = call.get_arg();
        let r#type = call.get_arg();
        let cx = call.get_arg();
        let cy = call.get_arg();
        let fu_load = call.get_arg();
        let res = api.LoadImageW(h_inst, name, r#type, cx, cy, fu_load);
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
        let h_instance = call.get_arg();
        let lp_menu_name = call.get_arg();
        let res = api.LoadMenuA(h_instance, lp_menu_name);
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
        let lp_menu_template = call.get_arg();
        let res = api.LoadMenuIndirectA(lp_menu_template);
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
        let lp_menu_template = call.get_arg();
        let res = api.LoadMenuIndirectW(lp_menu_template);
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
        let h_instance = call.get_arg();
        let lp_menu_name = call.get_arg();
        let res = api.LoadMenuW(h_instance, lp_menu_name);
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
        let h_instance = call.get_arg();
        let u_id = call.get_arg();
        let lp_buffer = call.get_arg();
        let cch_buffer_max = call.get_arg();
        let res = api.LoadStringA(h_instance, u_id, lp_buffer, cch_buffer_max);
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
        let h_instance = call.get_arg();
        let u_id = call.get_arg();
        let lp_buffer = call.get_arg();
        let cch_buffer_max = call.get_arg();
        let res = api.LoadStringW(h_instance, u_id, lp_buffer, cch_buffer_max);
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
        let u_lock_code = call.get_arg();
        let res = api.LockSetForegroundWindow(u_lock_code);
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
        let h_wnd = call.get_arg();
        let lp_point = call.get_arg();
        let res = api.LogicalToPhysicalPoint(h_wnd, lp_point);
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
        let f_icon = call.get_arg();
        let res = api.LookupIconIdFromDirectory(presbits, f_icon);
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
        let f_icon = call.get_arg();
        let cx_desired = call.get_arg();
        let cy_desired = call.get_arg();
        let flags = call.get_arg();
        let res = api.LookupIconIdFromDirectoryEx(presbits, f_icon, cx_desired, cy_desired, flags);
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
        let h_dlg = call.get_arg();
        let lp_rect = call.get_arg();
        let res = api.MapDialogRect(h_dlg, lp_rect);
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
        let h_wnd = call.get_arg();
        let h_menu = call.get_arg();
        let pt_screen = call.get_arg();
        let res = api.MenuItemFromPoint(h_wnd, h_menu, pt_screen);
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
        let h_wnd = call.get_arg();
        let lp_text = call.get_arg();
        let lp_caption = call.get_arg();
        let u_type = call.get_arg();
        let res = api.MessageBoxA(h_wnd, lp_text, lp_caption, u_type);
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
        let h_wnd = call.get_arg();
        let lp_text = call.get_arg();
        let lp_caption = call.get_arg();
        let u_type = call.get_arg();
        let w_language_id = call.get_arg();
        let res = api.MessageBoxExA(h_wnd, lp_text, lp_caption, u_type, w_language_id);
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
        let h_wnd = call.get_arg();
        let lp_text = call.get_arg();
        let lp_caption = call.get_arg();
        let u_type = call.get_arg();
        let w_language_id = call.get_arg();
        let res = api.MessageBoxExW(h_wnd, lp_text, lp_caption, u_type, w_language_id);
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
        let h_wnd = call.get_arg();
        let lp_text = call.get_arg();
        let lp_caption = call.get_arg();
        let u_type = call.get_arg();
        let res = api.MessageBoxW(h_wnd, lp_text, lp_caption, u_type);
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
        let h_mnu = call.get_arg();
        let u_position = call.get_arg();
        let u_flags = call.get_arg();
        let u_id_new_item = call.get_arg();
        let lp_new_item = call.get_arg();
        let res = api.ModifyMenuA(h_mnu, u_position, u_flags, u_id_new_item, lp_new_item);
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
        let h_mnu = call.get_arg();
        let u_position = call.get_arg();
        let u_flags = call.get_arg();
        let u_id_new_item = call.get_arg();
        let lp_new_item = call.get_arg();
        let res = api.ModifyMenuW(h_mnu, u_position, u_flags, u_id_new_item, lp_new_item);
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
        let h_wnd = call.get_arg();
        let x = call.get_arg();
        let y = call.get_arg();
        let n_width = call.get_arg();
        let n_height = call.get_arg();
        let b_repaint = call.get_arg();
        let res = api.MoveWindow(h_wnd, x, y, n_width, n_height, b_repaint);
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
        let n_count = call.get_arg();
        let p_handles = call.get_arg();
        let f_wait_all = call.get_arg();
        let dw_milliseconds = call.get_arg();
        let dw_wake_mask = call.get_arg();
        let res = api.MsgWaitForMultipleObjects(
            n_count,
            p_handles,
            f_wait_all,
            dw_milliseconds,
            dw_wake_mask,
        );
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
        let n_count = call.get_arg();
        let p_handles = call.get_arg();
        let dw_milliseconds = call.get_arg();
        let dw_wake_mask = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.MsgWaitForMultipleObjectsEx(
            n_count,
            p_handles,
            dw_milliseconds,
            dw_wake_mask,
            dw_flags,
        );
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
        let p_src = call.get_arg();
        let p_dst = call.get_arg();
        let res = api.OemToCharA(p_src, p_dst);
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
        let lpsz_src = call.get_arg();
        let lpsz_dst = call.get_arg();
        let cch_dst_length = call.get_arg();
        let res = api.OemToCharBuffA(lpsz_src, lpsz_dst, cch_dst_length);
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
        let lpsz_src = call.get_arg();
        let lpsz_dst = call.get_arg();
        let cch_dst_length = call.get_arg();
        let res = api.OemToCharBuffW(lpsz_src, lpsz_dst, cch_dst_length);
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
        let p_src = call.get_arg();
        let p_dst = call.get_arg();
        let res = api.OemToCharW(p_src, p_dst);
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
        let h_wnd = call.get_arg();
        let res = api.OpenIcon(h_wnd);
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
        let lp_msg = call.get_arg();
        let h_wnd = call.get_arg();
        let w_msg_filter_min = call.get_arg();
        let w_msg_filter_max = call.get_arg();
        let w_remove_msg = call.get_arg();
        let res = api.PeekMessageA(
            lp_msg,
            h_wnd,
            w_msg_filter_min,
            w_msg_filter_max,
            w_remove_msg,
        );
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
        let lp_msg = call.get_arg();
        let h_wnd = call.get_arg();
        let w_msg_filter_min = call.get_arg();
        let w_msg_filter_max = call.get_arg();
        let w_remove_msg = call.get_arg();
        let res = api.PeekMessageW(
            lp_msg,
            h_wnd,
            w_msg_filter_min,
            w_msg_filter_max,
            w_remove_msg,
        );
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
        let h_wnd = call.get_arg();
        let lp_point = call.get_arg();
        let res = api.PhysicalToLogicalPoint(h_wnd, lp_point);
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
        let h_wnd = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.PostMessageA(h_wnd, msg, w_param, l_param);
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
        let h_wnd = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.PostMessageW(h_wnd, msg, w_param, l_param);
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
        let n_exit_code = call.get_arg();
        let res = api.PostQuitMessage(n_exit_code);
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
        let id_thread = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.PostThreadMessageA(id_thread, msg, w_param, l_param);
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
        let id_thread = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.PostThreadMessageW(id_thread, msg, w_param, l_param);
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
        let sz_file_name = call.get_arg();
        let n_icon_index = call.get_arg();
        let cx_icon = call.get_arg();
        let cy_icon = call.get_arg();
        let phicon = call.get_arg();
        let piconid = call.get_arg();
        let n_icons = call.get_arg();
        let flags = call.get_arg();
        let res = api.PrivateExtractIconsA(
            sz_file_name,
            n_icon_index,
            cx_icon,
            cy_icon,
            phicon,
            piconid,
            n_icons,
            flags,
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
        let sz_file_name = call.get_arg();
        let n_icon_index = call.get_arg();
        let cx_icon = call.get_arg();
        let cy_icon = call.get_arg();
        let phicon = call.get_arg();
        let piconid = call.get_arg();
        let n_icons = call.get_arg();
        let flags = call.get_arg();
        let res = api.PrivateExtractIconsW(
            sz_file_name,
            n_icon_index,
            cx_icon,
            cy_icon,
            phicon,
            piconid,
            n_icons,
            flags,
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
        let hwnd_parent = call.get_arg();
        let pt_parent_client_coords = call.get_arg();
        let res = api.RealChildWindowFromPoint(hwnd_parent, pt_parent_client_coords);
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
        let ptsz_class_name = call.get_arg();
        let cch_class_name_max = call.get_arg();
        let res = api.RealGetWindowClassA(hwnd, ptsz_class_name, cch_class_name_max);
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
        let ptsz_class_name = call.get_arg();
        let cch_class_name_max = call.get_arg();
        let res = api.RealGetWindowClassW(hwnd, ptsz_class_name, cch_class_name_max);
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
        let lp_string = call.get_arg();
        let res = api.RegisterWindowMessageA(lp_string);
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
        let lp_string = call.get_arg();
        let res = api.RegisterWindowMessageW(lp_string);
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
        let h_menu = call.get_arg();
        let u_position = call.get_arg();
        let u_flags = call.get_arg();
        let res = api.RemoveMenu(h_menu, u_position, u_flags);
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
        let h_wnd = call.get_arg();
        let lp_string = call.get_arg();
        let res = api.RemovePropA(h_wnd, lp_string);
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
        let h_wnd = call.get_arg();
        let lp_string = call.get_arg();
        let res = api.RemovePropW(h_wnd, lp_string);
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
        let l_result = call.get_arg();
        let res = api.ReplyMessage(l_result);
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
        let h_wnd = call.get_arg();
        let x_amount = call.get_arg();
        let y_amount = call.get_arg();
        let lp_rect = call.get_arg();
        let lp_clip_rect = call.get_arg();
        let res = api.ScrollWindow(h_wnd, x_amount, y_amount, lp_rect, lp_clip_rect);
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
        let h_dlg = call.get_arg();
        let n_id_dlg_item = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.SendDlgItemMessageA(h_dlg, n_id_dlg_item, msg, w_param, l_param);
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
        let h_dlg = call.get_arg();
        let n_id_dlg_item = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.SendDlgItemMessageW(h_dlg, n_id_dlg_item, msg, w_param, l_param);
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
        let h_wnd = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.SendMessageA(h_wnd, msg, w_param, l_param);
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
        let h_wnd = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let lp_result_call_back = call.get_arg();
        let dw_data = call.get_arg();
        let res =
            api.SendMessageCallbackA(h_wnd, msg, w_param, l_param, lp_result_call_back, dw_data);
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
        let h_wnd = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let lp_result_call_back = call.get_arg();
        let dw_data = call.get_arg();
        let res =
            api.SendMessageCallbackW(h_wnd, msg, w_param, l_param, lp_result_call_back, dw_data);
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
        let h_wnd = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let fu_flags = call.get_arg();
        let u_timeout = call.get_arg();
        let lpdw_result = call.get_arg();
        let res = api.SendMessageTimeoutA(
            h_wnd,
            msg,
            w_param,
            l_param,
            fu_flags,
            u_timeout,
            lpdw_result,
        );
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
        let h_wnd = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let fu_flags = call.get_arg();
        let u_timeout = call.get_arg();
        let lpdw_result = call.get_arg();
        let res = api.SendMessageTimeoutW(
            h_wnd,
            msg,
            w_param,
            l_param,
            fu_flags,
            u_timeout,
            lpdw_result,
        );
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
        let h_wnd = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.SendMessageW(h_wnd, msg, w_param, l_param);
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
        let h_wnd = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.SendNotifyMessageA(h_wnd, msg, w_param, l_param);
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
        let h_wnd = call.get_arg();
        let msg = call.get_arg();
        let w_param = call.get_arg();
        let l_param = call.get_arg();
        let res = api.SendNotifyMessageW(h_wnd, msg, w_param, l_param);
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
        let u_m_seconds = call.get_arg();
        let res = api.SetCaretBlinkTime(u_m_seconds);
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
        let x = call.get_arg();
        let y = call.get_arg();
        let res = api.SetCaretPos(x, y);
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
        let h_wnd = call.get_arg();
        let n_index = call.get_arg();
        let dw_new_long = call.get_arg();
        let res = api.SetClassLongA(h_wnd, n_index, dw_new_long);
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
        let h_wnd = call.get_arg();
        let n_index = call.get_arg();
        let dw_new_long = call.get_arg();
        let res = api.SetClassLongW(h_wnd, n_index, dw_new_long);
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
        let h_wnd = call.get_arg();
        let n_index = call.get_arg();
        let w_new_word = call.get_arg();
        let res = api.SetClassWord(h_wnd, n_index, w_new_word);
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
        let h_wnd = call.get_arg();
        let n_id_event = call.get_arg();
        let u_elapse = call.get_arg();
        let lp_timer_func = call.get_arg();
        let u_tolerance_delay = call.get_arg();
        let res = api.SetCoalescableTimer(
            h_wnd,
            n_id_event,
            u_elapse,
            lp_timer_func,
            u_tolerance_delay,
        );
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
        let h_cursor = call.get_arg();
        let res = api.SetCursor(h_cursor);
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
        let x = call.get_arg();
        let y = call.get_arg();
        let res = api.SetCursorPos(x, y);
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
        let dw_level = call.get_arg();
        let res = api.SetDebugErrorLevel(dw_level);
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
        let h_dlg = call.get_arg();
        let n_id_dlg_item = call.get_arg();
        let u_value = call.get_arg();
        let b_signed = call.get_arg();
        let res = api.SetDlgItemInt(h_dlg, n_id_dlg_item, u_value, b_signed);
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
        let h_dlg = call.get_arg();
        let n_id_dlg_item = call.get_arg();
        let lp_string = call.get_arg();
        let res = api.SetDlgItemTextA(h_dlg, n_id_dlg_item, lp_string);
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
        let h_dlg = call.get_arg();
        let n_id_dlg_item = call.get_arg();
        let lp_string = call.get_arg();
        let res = api.SetDlgItemTextW(h_dlg, n_id_dlg_item, lp_string);
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
        let h_wnd = call.get_arg();
        let res = api.SetForegroundWindow(h_wnd);
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
        let cr_key = call.get_arg();
        let b_alpha = call.get_arg();
        let dw_flags = call.get_arg();
        let res = api.SetLayeredWindowAttributes(hwnd, cr_key, b_alpha, dw_flags);
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
        let h_wnd = call.get_arg();
        let h_menu = call.get_arg();
        let res = api.SetMenu(h_wnd, h_menu);
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
        let h_menu = call.get_arg();
        let u_item = call.get_arg();
        let f_by_pos = call.get_arg();
        let res = api.SetMenuDefaultItem(h_menu, u_item, f_by_pos);
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
        let l_param = call.get_arg();
        let res = api.SetMessageExtraInfo(l_param);
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
        let c_messages_max = call.get_arg();
        let res = api.SetMessageQueue(c_messages_max);
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
        let h_wnd_child = call.get_arg();
        let h_wnd_new_parent = call.get_arg();
        let res = api.SetParent(h_wnd_child, h_wnd_new_parent);
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
        let x = call.get_arg();
        let y = call.get_arg();
        let res = api.SetPhysicalCursorPos(x, y);
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
        let dw_default_layout = call.get_arg();
        let res = api.SetProcessDefaultLayout(dw_default_layout);
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
        let h_wnd = call.get_arg();
        let lp_string = call.get_arg();
        let h_data = call.get_arg();
        let res = api.SetPropA(h_wnd, lp_string, h_data);
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
        let h_wnd = call.get_arg();
        let lp_string = call.get_arg();
        let h_data = call.get_arg();
        let res = api.SetPropW(h_wnd, lp_string, h_data);
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
        let c_elements = call.get_arg();
        let lpa_elements = call.get_arg();
        let lpa_rgb_values = call.get_arg();
        let res = api.SetSysColors(c_elements, lpa_elements, lpa_rgb_values);
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
        let h_wnd = call.get_arg();
        let n_id_event = call.get_arg();
        let u_elapse = call.get_arg();
        let lp_timer_func = call.get_arg();
        let res = api.SetTimer(h_wnd, n_id_event, u_elapse, lp_timer_func);
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
        let h_wnd = call.get_arg();
        let dw_affinity = call.get_arg();
        let res = api.SetWindowDisplayAffinity(h_wnd, dw_affinity);
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
        let h_wnd = call.get_arg();
        let n_index = call.get_arg();
        let dw_new_long = call.get_arg();
        let res = api.SetWindowLongA(h_wnd, n_index, dw_new_long);
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
        let h_wnd = call.get_arg();
        let n_index = call.get_arg();
        let dw_new_long = call.get_arg();
        let res = api.SetWindowLongW(h_wnd, n_index, dw_new_long);
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
        let h_wnd = call.get_arg();
        let lpwndpl = call.get_arg();
        let res = api.SetWindowPlacement(h_wnd, lpwndpl);
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
        let h_wnd = call.get_arg();
        let h_wnd_insert_after = call.get_arg();
        let x = call.get_arg();
        let y = call.get_arg();
        let cx = call.get_arg();
        let cy = call.get_arg();
        let u_flags = call.get_arg();
        let res = api.SetWindowPos(h_wnd, h_wnd_insert_after, x, y, cx, cy, u_flags);
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
        let h_wnd = call.get_arg();
        let lp_string = call.get_arg();
        let res = api.SetWindowTextA(h_wnd, lp_string);
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
        let h_wnd = call.get_arg();
        let lp_string = call.get_arg();
        let res = api.SetWindowTextW(h_wnd, lp_string);
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
        let h_wnd = call.get_arg();
        let n_index = call.get_arg();
        let w_new_word = call.get_arg();
        let res = api.SetWindowWord(h_wnd, n_index, w_new_word);
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
        let n_filter_type = call.get_arg();
        let pfn_filter_proc = call.get_arg();
        let res = api.SetWindowsHookA(n_filter_type, pfn_filter_proc);
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
        let id_hook = call.get_arg();
        let lpfn = call.get_arg();
        let hmod = call.get_arg();
        let dw_thread_id = call.get_arg();
        let res = api.SetWindowsHookExA(id_hook, lpfn, hmod, dw_thread_id);
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
        let id_hook = call.get_arg();
        let lpfn = call.get_arg();
        let hmod = call.get_arg();
        let dw_thread_id = call.get_arg();
        let res = api.SetWindowsHookExW(id_hook, lpfn, hmod, dw_thread_id);
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
        let n_filter_type = call.get_arg();
        let pfn_filter_proc = call.get_arg();
        let res = api.SetWindowsHookW(n_filter_type, pfn_filter_proc);
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
        let h_wnd = call.get_arg();
        let res = api.ShowCaret(h_wnd);
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
        let b_show = call.get_arg();
        let res = api.ShowCursor(b_show);
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
        let h_wnd = call.get_arg();
        let f_show = call.get_arg();
        let res = api.ShowOwnedPopups(h_wnd, f_show);
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
        let h_wnd = call.get_arg();
        let n_cmd_show = call.get_arg();
        let res = api.ShowWindow(h_wnd, n_cmd_show);
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
        let h_wnd = call.get_arg();
        let n_cmd_show = call.get_arg();
        let res = api.ShowWindowAsync(h_wnd, n_cmd_show);
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
        let f_unknown = call.get_arg();
        let res = api.SwitchToThisWindow(hwnd, f_unknown);
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
        let ui_action = call.get_arg();
        let ui_param = call.get_arg();
        let pv_param = call.get_arg();
        let f_win_ini = call.get_arg();
        let res = api.SystemParametersInfoA(ui_action, ui_param, pv_param, f_win_ini);
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
        let ui_action = call.get_arg();
        let ui_param = call.get_arg();
        let pv_param = call.get_arg();
        let f_win_ini = call.get_arg();
        let res = api.SystemParametersInfoW(ui_action, ui_param, pv_param, f_win_ini);
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
        let hwnd_parent = call.get_arg();
        let w_how = call.get_arg();
        let lp_rect = call.get_arg();
        let c_kids = call.get_arg();
        let lp_kids = call.get_arg();
        let res = api.TileWindows(hwnd_parent, w_how, lp_rect, c_kids, lp_kids);
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
        let h_menu = call.get_arg();
        let u_flags = call.get_arg();
        let x = call.get_arg();
        let y = call.get_arg();
        let n_reserved = call.get_arg();
        let h_wnd = call.get_arg();
        let prc_rect = call.get_arg();
        let res = api.TrackPopupMenu(h_menu, u_flags, x, y, n_reserved, h_wnd, prc_rect);
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
        let h_menu = call.get_arg();
        let u_flags = call.get_arg();
        let x = call.get_arg();
        let y = call.get_arg();
        let hwnd = call.get_arg();
        let lptpm = call.get_arg();
        let res = api.TrackPopupMenuEx(h_menu, u_flags, x, y, hwnd, lptpm);
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
        let h_wnd = call.get_arg();
        let h_acc_table = call.get_arg();
        let lp_msg = call.get_arg();
        let res = api.TranslateAcceleratorA(h_wnd, h_acc_table, lp_msg);
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
        let h_wnd = call.get_arg();
        let h_acc_table = call.get_arg();
        let lp_msg = call.get_arg();
        let res = api.TranslateAcceleratorW(h_wnd, h_acc_table, lp_msg);
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
        let h_wnd_client = call.get_arg();
        let lp_msg = call.get_arg();
        let res = api.TranslateMDISysAccel(h_wnd_client, lp_msg);
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
        let lp_msg = call.get_arg();
        let res = api.TranslateMessage(lp_msg);
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
        let n_code = call.get_arg();
        let pfn_filter_proc = call.get_arg();
        let res = api.UnhookWindowsHook(n_code, pfn_filter_proc);
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
        let lp_class_name = call.get_arg();
        let h_instance = call.get_arg();
        let res = api.UnregisterClassA(lp_class_name, h_instance);
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
        let lp_class_name = call.get_arg();
        let h_instance = call.get_arg();
        let res = api.UnregisterClassW(lp_class_name, h_instance);
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
        let point = call.get_arg();
        let res = api.WindowFromPhysicalPoint(point);
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
        let point = call.get_arg();
        let res = api.WindowFromPoint(point);
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
        let param_0 = call.get_arg();
        let param_1 = call.get_arg();
        let res = api.wsprintfA(param_0, param_1);
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
        let param_0 = call.get_arg();
        let param_1 = call.get_arg();
        let res = api.wsprintfW(param_0, param_1);
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
        let param_0 = call.get_arg();
        let param_1 = call.get_arg();
        let arglist = call.get_arg();
        let res = api.wvsprintfA(param_0, param_1, arglist);
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
        let param_0 = call.get_arg();
        let param_1 = call.get_arg();
        let arglist = call.get_arg();
        let res = api.wvsprintfW(param_0, param_1, arglist);
        call.finish(res);
    }));
    if result.is_err() {
        eprintln!("Caught a panic in native code. Whoops, aborting..");
        std::process::abort();
    }
}
