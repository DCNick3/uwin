//
// Created by dcnick3 on 4/26/21.
//

#include <fmt/core.h>

#include "util/except.h"
#include "str/uconv.h"
#include "str/uconv_internal.h"

namespace uwin::str {
    std::unique_ptr<converter<wide, native>> cvt_wide_to_native() {
        return std::make_unique<cvt_from_wide_impl<native>>(open_converter("UTF-8"));
    }
    std::unique_ptr<converter<wide, narrow>> cvt_wide_to_narrow(std::uint32_t codepage) {
        return std::make_unique<cvt_from_wide_impl<narrow>>(open_converter(codepage_to_ccsid(codepage)));
    }
    std::unique_ptr<converter<native, wide>> cvt_native_to_wide() {
        return std::make_unique<cvt_to_wide_impl<native>>(open_converter("UTF-8"));
    }
    std::unique_ptr<converter<narrow, wide>> cvt_narrow_to_wide(std::uint32_t codepage) {
        return std::make_unique<cvt_to_wide_impl<narrow>>(open_converter(codepage_to_ccsid(codepage)));
    }
    std::unique_ptr<converter<narrow, native>> cvt_narrow_to_native(std::uint32_t codepage) {
        return make_unique_cvt_composition(cvt_narrow_to_wide(codepage), cvt_wide_to_native());
    }
    std::unique_ptr<converter<native, narrow>> cvt_native_to_narrow(std::uint32_t codepage) {
        return make_unique_cvt_composition(cvt_native_to_wide(), cvt_wide_to_narrow(codepage));
    }

    native wide_to_native(wide_view src_view) {
        return string_uconv_with_cvt(*cvt_wide_to_native(), src_view);
    }

    wide native_to_wide(native_view src_view) {
        return string_uconv_with_cvt(*cvt_native_to_wide(), src_view);
    }

    wide narrow_to_wide(std::uint32_t codepage, narrow_view src_view) {
        return string_uconv_with_cvt(*cvt_narrow_to_wide(codepage), src_view);
    }

    native narrow_to_native(std::uint32_t codepage, narrow_view src_view) {
        return string_uconv_with_cvt(*cvt_narrow_to_native(codepage), src_view);
    }

    narrow wide_to_narrow(std::uint32_t codepage, wide_view src_view) {
        return string_uconv_with_cvt(*cvt_wide_to_narrow(codepage), src_view);
    }

    narrow native_to_narrow(std::uint32_t codepage, native_view src_view) {
        return string_uconv_with_cvt(*cvt_native_to_narrow(codepage), src_view);
    }
}