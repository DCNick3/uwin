//
// Created by dcnick3 on 11/15/20.
//

#pragma once

#include "mem/tptr.h"
#include "win32/ldr/except.h"

#include <string>

namespace uwin::win32::ldr {
    class linkable {
    public:
        [[nodiscard]] virtual mem::tcaddr try_resolve(std::string const& symbol_name) const = 0;
    };
}
