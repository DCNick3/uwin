//
// Created by dcnick3 on 11/11/20.
//

#pragma once

#include <fmt/core.h>

#include <stdexcept>
#include <string>

namespace uwin::win32::dll {
    class stub_exception : std::runtime_error {
    public:
        inline explicit stub_exception(const char *unimplemented_fun_name) :
                std::runtime_error(fmt::format("Called a DLL function {}, which is not implemented",
                                               unimplemented_fun_name)) {
        }
    };
}