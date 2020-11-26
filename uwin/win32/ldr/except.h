//
// Created by dcnick3 on 11/15/20.
//

#pragma once

#include <stdexcept>

namespace uwin::win32::ldr {
    class loader_exception : public std::runtime_error {
    public:
        explicit inline loader_exception(const std::string &arg) : runtime_error(arg) {}
    };
}
