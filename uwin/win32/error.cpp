//
// Created by dcnick3 on 11/2/20.
//

#include "win32/error.h"

namespace uwin {
    namespace win32 {

        const char *error::what() const noexcept {
            return _what.c_str();
        }
    }
}
