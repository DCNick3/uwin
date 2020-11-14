//
// Created by dcnick3 on 11/14/20.
//

#pragma once

#include <stdexcept>

namespace uwin::win32::ldr {
    class pe_format_exception : public std::runtime_error {
    public:
        pe_format_exception() : runtime_error("Attempt to load PE file with invalid format was made") {}
    };
}
