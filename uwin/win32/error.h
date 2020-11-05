//
// Created by dcnick3 on 11/2/20.
//

#pragma once

#include "win32/error_code.h"

#include <cstdint>

#include <string>
#include <exception>

#include <fmt/core.h>

namespace uwin {
    namespace win32 {

        class error : public std::exception {
            error_code _code;
            const char* _slug;
            const char* _desc;
            std::string _what;

        public:
            inline error(error_code code, std::string const& additional_desc)
                : error(code) {
                _what = fmt::format("win32 error ({}): {}\nAdditional description: {}", _slug, _desc, additional_desc);
            }

            inline explicit error(error_code code)
                    : _code(code), _slug(error_code_to_slug(code)), _desc(error_code_to_desc(code)) {
                _what = fmt::format("win32 error ({}): {}", _slug, _desc);
            }

            inline error_code code() const noexcept { return _code; }
            inline const char* slug() const noexcept { return _slug; }
            inline const char* desc() const noexcept { return _desc; }

            const char* what() const noexcept override;
        };
    }
}
