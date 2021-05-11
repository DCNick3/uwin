//
// Created by dcnick3 on 5/12/21.
//

#pragma once

#include <cstdint>

#include "ctx/env.h"
#include "str/uconv.h"

namespace uwin::win32 {
    class uconv {
        std::uint32_t _ansi_codepage;
        std::uint32_t _oem_codepage;
    public:
        inline explicit uconv(ctx::env_param const& env_param)
            : _ansi_codepage(env_param.ansi_codepage),
              _oem_codepage(env_param.oem_codepage) {
        }
        static inline str::native wide_to_native(str::wide_view src) {
            return str::wide_to_native(src);
        }
        static inline str::wide native_to_wide(str::native_view src) {
            return str::native_to_wide(src);
        }

        [[nodiscard]] inline str::wide ansi_to_wide(str::narrow_view src) const {
            return str::narrow_to_wide(_ansi_codepage, src);
        }
        [[nodiscard]] inline str::wide oem_to_wide(str::narrow_view src) const {
            return str::narrow_to_wide(_oem_codepage, src);
        }

        [[nodiscard]] inline str::native ansi_to_native(str::narrow_view src) const {
            return str::narrow_to_native(_ansi_codepage, src);
        }
        [[nodiscard]] inline str::native oem_to_native(str::narrow_view src) const {
            return str::narrow_to_native(_oem_codepage, src);
        }

        [[nodiscard]] inline str::narrow native_to_ansi(str::native_view src) const {
            return str::native_to_narrow(_ansi_codepage, src);
        }
        [[nodiscard]] inline str::narrow native_to_oem(str::native_view src) const {
            return str::native_to_narrow(_oem_codepage, src);
        }

        [[nodiscard]] inline str::narrow wide_to_ansi(str::wide_view src) const {
            return str::wide_to_narrow(_ansi_codepage, src);
        }
        [[nodiscard]] inline str::narrow wide_to_oem(str::wide_view src) const {
            return str::wide_to_narrow(_oem_codepage, src);
        }

        [[nodiscard]] inline std::uint32_t get_ansi_codepage() const {
            return _ansi_codepage;
        }

        [[nodiscard]] inline std::uint32_t get_oem_codepage() const {
            return _oem_codepage;
        }
    };
}
