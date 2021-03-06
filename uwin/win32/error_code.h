//
// Created by dcnick3 on 11/2/20.
//

#pragma once

#include <cstdint>

namespace uwin::win32 {
    enum class error_code : std::uint32_t;

    const char *error_code_to_slug(error_code code);

    const char *error_code_to_desc(error_code code);

    bool error_code_known(error_code code);
}

#include "win32/error_code_generated.h"