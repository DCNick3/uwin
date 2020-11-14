//
// Created by dcnick3 on 11/14/20.
//

#pragma once

#include <stdexcept>

namespace uwin {
    namespace win32 {
        namespace ldr {
            class pe_format_exception : public std::runtime_error {

            };
        }
    }
}
