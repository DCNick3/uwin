
#pragma once

#include "mem/tptr.h"
#include "win32/ldr/except.h"
#include "win32/types/types.h"

#include <string>

namespace uwin::win32::ldr {
    class module {
    public:
        [[nodiscard]] virtual mem::tcaddr try_resolve(std::string const& symbol_name) const = 0;
        [[nodiscard]] virtual types::hmodule handle() const = 0;
        [[nodiscard]] virtual std::string const& name() const = 0;
    };
}
