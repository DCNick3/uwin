//
// Created by dcnick3 on 11/11/20.
//

#pragma once

#include <fmt/core.h>

#include <exception>
#include <string>

namespace uwin {
    namespace win32 {
        namespace dll {
            class stub_exception : std::exception {
                std::string _what;

                const char *what() const _GLIBCXX_TXN_SAFE_DYN _GLIBCXX_NOTHROW override;

            public:
                inline explicit stub_exception(const char* unimplemented_fun_name) :
                    _what(fmt::format("Called a DLL function {}, which is not implemented", unimplemented_fun_name)) {
                }
            };
        }
    }
}