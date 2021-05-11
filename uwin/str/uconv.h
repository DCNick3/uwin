//
// Created by dcnick3 on 4/26/21.
//

#pragma once

#include <string>

#include "str/str.h"

namespace uwin::str {
    native wide_to_native(wide_view src);
    wide native_to_wide(native_view src);

    wide narrow_to_wide(std::uint32_t codepage, narrow_view src);
    native narrow_to_native(std::uint32_t codepage, narrow_view src);

    narrow native_to_narrow(std::uint32_t codepage, native_view src);
    narrow wide_to_narrow(std::uint32_t codepage, wide_view src);

    class icu_error : public std::runtime_error {
    public:
        inline explicit icu_error(std::string const& s) : std::runtime_error(s) {}
    };
}